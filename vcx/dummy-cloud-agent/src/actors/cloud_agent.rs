use actix::prelude::*;
use actors::{AddA2ARoute, ForwardA2AMsg, HandleA2AMsg, RouteA2AMsg};
use actors::cloud_agent_connection::CloudAgentConnection;
use actors::router::Router;
use domain::a2a::*;
use domain::config::{CloudAgentConfig, WalletStorageConfig, CloudAgentConnectionConfig};
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, IndyError, wallet, pairwise};
use std::convert::Into;
use utils::futures::*;

pub struct CloudAgent {
    wallet_handle: i32,
    owner_did: String,
    did: String,
    verkey: String,
    router: Addr<Router>,
}

impl CloudAgent {
    #[allow(unused)] // FIXME: Use!
    pub fn new(config: CloudAgentConfig,
               wallet_storage_config: WalletStorageConfig,
               router: Addr<Router>) -> ResponseFuture<CloudAgent, Error> {
        let wallet_config = json!({
                    "id": config.wallet_id,
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

        let wallet_credentials = json!({
                    "key": config.wallet_passphrase,
                    "storage_credentials": wallet_storage_config.credentials,
                }).to_string();

        future::ok(())
            .and_then(move |_| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map_err(|err| err.context("Can't open Cloud Agent wallet.`").into())
            })
            .and_then(move |wallet_handle| {
                did::key_for_local_did(wallet_handle,
                                       config.did.as_ref())
                    .map(move |verkey| (wallet_handle, verkey, config))
                    .map_err(|err| err.context("Can't get Cloud Agent did key").into())
            })
            .map(move |(wallet_handle, verkey, config)| {
                CloudAgent {
                    wallet_handle,
                    verkey,
                    did: config.did,
                    owner_did: config.owner_did,
                    router
                }
            })
            .into_box()
    }

    fn forward_a2a_msg(&mut self,
                       msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("CloudAgent::forward_a2a_msg >> {:?}", msg);

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
        trace!("CloudAgent::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::unbundle_authcrypted(slf.wallet_handle, &slf.verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::CreateKey(msg)) => {
                        slf.handle_create_key(sender_vk, msg)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }


    fn handle_create_key(&mut self,
                         sender_vk: String,
                         msg: CreateKey) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("CloudAgent::_handle_create_key >> {:?}, {:?}",
               sender_vk, msg);

        let CreateKey { for_did, for_did_verkey } = msg;

        let their_did_info = json!({
            "did": for_did,
            "verkey": for_did_verkey,
        }).to_string();

        future::ok(for_did.clone())
            .into_actor(self)
            .and_then(move |did, slf, _|
                slf.check_no_pairwise_exists(&did)
                    .into_actor(slf)
            )
            .and_then(move |_, slf, _|
                slf.store_their_key(&their_did_info)
                    .into_actor(slf)
            )
            .and_then(move |_, slf, _| {
                did::create_and_store_my_did(slf.wallet_handle, "{}")
                    .map_err(|err| err.context("Can't create DID for agent pairwise connection.").into())
                    .into_actor(slf)
            })
            .and_then(move |(pairwise_did, pairwise_did_verkey), slf, _| {
                pairwise::create_pairwise(slf.wallet_handle, &for_did, &pairwise_did, "{}")
                    .map(|_| (for_did, pairwise_did, pairwise_did_verkey))
                    .map_err(|err| err.context("Can't store agent pairwise connection.").into())
                    .into_actor(slf)
            })
            .and_then(move |(for_did, pairwise_did, pairwise_did_verkey), slf, _|
                slf.init_pairwise_connection(&for_did, &for_did_verkey, &pairwise_did, &pairwise_did_verkey)
                    .map(|_, _, _| (pairwise_did, pairwise_did_verkey))
            )
            .and_then(move |(pairwise_did, pairwise_did_verkey), slf, _| {
                let msgs = vec![A2AMessage::KeyCreated(KeyCreated {
                    with_pairwise_did: pairwise_did,
                    with_pairwise_did_verkey: pairwise_did_verkey,
                })];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.verkey, &sender_vk, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn check_no_pairwise_exists(&mut self,
                                did: &str) -> ResponseFuture<(), Error> {
        pairwise::is_pairwise_exists(self.wallet_handle, did)
            .map_err(|err| err.context("Can't check if agent pairwise connection exists.").into())
            .and_then(|is_exist|
                if is_exist {
                    err!(err_msg("Agent pairwise connection already exists.")).into()
                } else {
                    future::ok(()).into_box()
                }
            )
            .into_box()
    }

    fn store_their_key(&mut self,
                       did_info: &str) -> ResponseFuture<(), Error> {
        did::store_their_did(self.wallet_handle, &did_info)
            .then(|res| match res {
                Ok(_) => Ok(()),
                Err(IndyError::DidAlreadyExistsError) => Ok(()), // Already exists
                Err(err) => Err(err),
            })
            .map_err(|err| err.context("Can't store their DID for pairwise.").into())
            .into_box()
    }

    fn init_pairwise_connection(&mut self,
                                for_did: &str,
                                for_did_verkey: &str,
                                pairwise_did: &str,
                                pairwise_did_verkey: &str) -> ResponseActFuture<Self, (), Error> {
        let config = CloudAgentConnectionConfig {
            wallet_handle: self.wallet_handle,
            owner_did: self.owner_did.to_string(),
            agent_did: self.did.to_string(),
            from_did: for_did.to_string(),
            from_did_verkey: for_did_verkey.to_string(),
            pairwise_did: pairwise_did.to_string(),
            pairwise_did_verkey: pairwise_did_verkey.to_string(),
        };

        let for_did = for_did.to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                CloudAgentConnection::new(config)
                    .map_err(|err| err.context("Can't establish agent pairwise connection.").into())
                    .into_actor(slf)
            })
            .and_then(move |agent_pairwise_connection, slf, _| {
                slf.router
                    .send(AddA2ARoute(for_did, agent_pairwise_connection.start().recipient()))
                    .from_err()
                    .map_err(|err: Error| err.context("Can't add route for agent pairwise connection.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}

impl Actor for CloudAgent {
    type Context = Context<Self>;
}

impl Handler<ForwardA2AMsg> for CloudAgent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: ForwardA2AMsg, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Handler<ForwardA2AMsg>::handle >> {:?}", msg);
        self.forward_a2a_msg(msg.0)
    }
}

impl Handler<HandleA2AMsg> for CloudAgent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AgentMsgsBundle>::handle >> {:?}", msg);
        self.handle_a2a_msg(msg.0)
    }
}