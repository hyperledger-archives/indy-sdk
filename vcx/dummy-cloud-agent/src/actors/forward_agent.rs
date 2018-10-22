use actix::prelude::*;
use actors::{AddA2ARoute, Endpoint, ForwardA2AMsg, GetEndpoint, HandleA2AMsg, RouteA2AMsg};
use actors::forward_agent_connection::ForwardAgentConnection;
use actors::router::Router;
use domain::a2a::*;
use domain::config::{ForwardAgentConfig, WalletStorageConfig};
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, IndyError, wallet};
use std::convert::Into;
use utils::futures::*;

pub struct ForwardAgent {
    wallet_handle: i32,
    did: String,
    verkey: String,
    router: Addr<Router>,
    #[allow(unused)] // FIXME: Use!
    wallet_storage_config: WalletStorageConfig,
}

impl ForwardAgent {
    pub fn new(config: ForwardAgentConfig,
               wallet_storage_config: WalletStorageConfig,
               router: Addr<Router>) -> ResponseFuture<ForwardAgent, Error> {
        future::ok(())
            .and_then(move |_| {
                let wallet_config = json!({
                    "id": config.wallet_id,
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

                let wallet_credentials = json!({
                    "key": config.wallet_passphrase,
                    "storage_credentials": wallet_storage_config.credentials,
                }).to_string();

                wallet::create_wallet(&wallet_config, &wallet_credentials)
                    .then(|res| {
                        match res {
                            Err(IndyError::WalletAlreadyExistsError) => Ok(()),
                            r => r
                        }
                    })
                    .map(|_| (config, wallet_storage_config, wallet_config, wallet_credentials))
                    .map_err(|err| err.context("Can't ensure Forward Agent wallet created.").into())
            })
            .and_then(|(config, wallet_storage_config, wallet_config, wallet_credentials)| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map(|wallet_handle| (wallet_handle, config, wallet_storage_config))
                    .map_err(|err| err.context("Can't open Forward Agent wallet.`").into())
            })
            .and_then(move |(wallet_handle, config, wallet_storage_config)| {
                let did_info = json!({
                    "did": config.did,
                    "seed": config.did_seed,
                }).to_string();

                did::create_and_store_my_did(wallet_handle, &did_info)
                    .then(|res| match res {
                        Ok(_) => Ok(()),
                        Err(IndyError::DidAlreadyExistsError) => Ok(()), // Already exists
                        Err(err) => Err(err),
                    })
                    .map(move |_| (wallet_handle, config, wallet_storage_config))
                    .map_err(|err| err.context("Can't create Forward Agent did.").into())
            })
            .and_then(move |(wallet_handle, config, wallet_storage_config)| {
                did::key_for_local_did(wallet_handle,
                                       config.did.as_ref())
                    .map(move |verkey| (wallet_handle, verkey, config, wallet_storage_config))
                    .map_err(|err| err.context("Can't get Forward Agent did key").into())
            })
            .map(move |(wallet_handle, verkey, config, wallet_storage_config)| {
                ForwardAgent {
                    wallet_handle,
                    did: config.did,
                    verkey,
                    router,
                    wallet_storage_config,
                }
            })
            .into_box()
    }

    fn get_endpoint(&self) -> (String, String) {
        trace!("ForwardAgent::get_endpoint >>");
        (self.did.clone(), self.verkey.clone())
    }

    fn forward_a2a_msg(&mut self,
                       msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::forward_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::unbundle_anoncrypted(slf.wallet_handle, &slf.verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(move |mut msgs, slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::Forward(msg)) => {
                        slf.router
                            .send(RouteA2AMsg(msg.fwd, msg.msg))
                            .from_err()
                            .and_then(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    fn handle_a2a_msg(&mut self,
                      msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::unbundle_authcrypted(slf.wallet_handle, &slf.verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::Connect(msg)) => {
                        slf.connect(sender_vk, msg)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    fn connect(&mut self,
               sender_vk: String,
               msg: Connect) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgent::connect >> {:?}, {:?}", sender_vk, msg);

        let Connect { from_did: their_did, from_did_verkey: their_verkey } = msg;

        if their_verkey != sender_vk {
            return err_act!(self, err_msg("Inconsistent sender and connection verkeys"));
        };

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                ForwardAgentConnection::establish(slf.wallet_handle, their_did.clone(), their_verkey.clone())
                    .map(|connection| (connection, their_did, their_verkey))
                    .map_err(|err| err.context("Can't establish connection.").into())
                    .into_actor(slf)
            })
            .and_then(move |(connection, their_did, their_verkey), slf, _| {
                let (my_did, my_verkey) = connection.get_endpoint();

                slf.router
                    .send(AddA2ARoute(their_did, connection.start().recipient()))
                    .from_err()
                    .map(move |_| (my_did, my_verkey, their_verkey))
                    .map_err(|err: Error| err.context("Can't add route for connection.").into())
                    .into_actor(slf)
            })
            .and_then(move |(my_did, my_verkey, their_verkey), slf, _| {
                let msgs = vec![A2AMessage::Connected(Connected {
                    with_pairwise_did: my_did,
                    with_pairwise_did_verkey: my_verkey,
                })];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.verkey, &their_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt connected message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}

impl Actor for ForwardAgent {
    type Context = Context<Self>;
}

impl Handler<ForwardA2AMsg> for ForwardAgent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: ForwardA2AMsg, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Handler<ForwardA2AMsg>::handle >> {:?}", msg);
        self.forward_a2a_msg(msg.0)
    }
}

impl Handler<GetEndpoint> for ForwardAgent {
    type Result = Result<Endpoint, Error>;

    fn handle(&mut self, _msg: GetEndpoint, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Handler<GetEndpoint>::handle >> {:?}", _msg);
        let (did, verkey) = self.get_endpoint();
        Ok(Endpoint { did, verkey })
    }
}

impl Handler<HandleA2AMsg> for ForwardAgent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AgentMsgsBundle>::handle >> {:?}", msg);
        self.handle_a2a_msg(msg.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_core::reactor::Core;
    use utils::tests::*;

    #[test]
    fn forward_agent_new_works() {
        let mut core = Core::new().unwrap();

        let config = ForwardAgentConfig {
            wallet_id: FORWARD_AGENT_WALLET_ID.into(),
            wallet_passphrase: FORWARD_AGENT_WALLET_PASSPHRASE.into(),
            did: FORWARD_AGENT_DID.into(),
            did_seed: Some(FORWARD_AGENT_DID_SEED.into()),
        };

        let wallet_storage_config = WalletStorageConfig {
            xtype: None,
            config: None,
            credentials: None,
        };

        let res = core.run({
            ForwardAgent::start(config, wallet_storage_config, Router::new().start())
        });

        res.unwrap();
    }
}