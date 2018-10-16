use actix::prelude::{Actor, Addr, Context, Handler, Message as ActorMessage, ResponseFuture};
use domain::config::ForwardAgentConfig;
use domain::messages::*;
use domain::pairwise::*;
use failure::*;
use futures::*;
use indy::{did, IndyError, pairwise, wallet};
use serde_json;
use utils::futures::*;
use utils::messages::*;

pub struct ForwardAgent {
    wallet_handle: i32,
    verkey: String,
    config: ForwardAgentConfig,
}

impl ForwardAgent {
    pub fn start(config: ForwardAgentConfig) -> ResponseFuture<Addr<ForwardAgent>, Error> {
        Self::new(config)
            .map(|forward_agent| forward_agent.start())
            .into_box()
    }

    pub fn new(config: ForwardAgentConfig) -> ResponseFuture<ForwardAgent, Error> {
        let wallet_config = json!({
            "id": config.wallet_id,
            "storage_type": config.wallet_storage_type,
            "storage_config": config.wallet_storage_config,
         })
            .to_string();

        let wallet_credentials = json!({
            "key": config.wallet_passphrase,
            "storage_credentials": config.wallet_storage_credentials,
        })
            .to_string();

        let did_info = json!({
            "did": config.did,
            "seed": config.did_seed,
        })
            .to_string();

        future::ok(())
            .and_then(move |_| {
                wallet::create_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .then(|res| {
                        match res {
                            Err(IndyError::WalletAlreadyExistsError) => Ok(()),
                            r => r
                        }
                    })
                    .map(|_| (wallet_config, wallet_credentials))
                    .map_err(|err| err.context("Can't ensure Forward Agent wallet created.").into())
            })
            .and_then(|(wallet_config, wallet_credentials)| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map_err(|err| err.context("Can't open Forward Agent wallet.`").into())
            })
            .and_then(move |wallet_handle| {
                did::create_and_store_my_did(wallet_handle, did_info.as_ref())
                    .then(|res| match res {
                        Ok(_) => Ok(()),
                        Err(IndyError::DidAlreadyExistsError) => Ok(()), // Already exists
                        Err(err) => Err(err),
                    })
                    .map(move |_| wallet_handle)
                    .map_err(|err| err.context("Can't create Forward Agent did.").into())
            })
            .and_then(move |wallet_handle| {
                did::key_for_local_did(wallet_handle,
                                       config.did.as_ref())
                    .map(move |verkey| (wallet_handle, verkey, config))
                    .map_err(|err| err.context("Can't get Forward Agent did key").into())
            })
            .map(move |(wallet_handle, verkey, config)| {
                ForwardAgent {
                    wallet_handle,
                    verkey,
                    config,
                }
            })
            .into_box()
    }

    pub fn get_endpoint_details(&self) -> (String, String) {
        trace!("ForwardAgent::get_endpoint_details >>");
        let res = (self.config.did.clone(), self.verkey.clone());
        res
    }

    pub fn forward_message(&mut self, msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("ForwardAgent::forward_message >> {:?}", msg);

        let forward_agent_did = self.config.did.clone();
        let forward_agent_vk = self.verkey.clone();
        let wallet_handle = self.wallet_handle;

        unbundle_anoncrypted(self.wallet_handle, &forward_agent_vk, &msg)
            .and_then(move |msg| {
                match msg {
                    Message::Forward(msg) => {
                        if msg.fwd == forward_agent_did {
                            Self::_handle_forward_agent_message(wallet_handle, forward_agent_vk, msg.msg)
                        } else {
                            Self::_handle_forward_agent_pairwise_message(msg.msg)
                        }
                    }
                    _ => future::err(err_msg("Unsupported message")).into_box()
                }
            })
            .into_box()
    }

    fn _handle_forward_agent_message(wallet_handle: i32,
                                     forward_agent_vk: String,
                                     msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("ForwardAgent::_handle_forward_agent_message >> {:?}, {:?}, {:?}",
               wallet_handle, forward_agent_vk, msg);

        unbundle_authcrypted(wallet_handle, &forward_agent_vk, &msg)
            .and_then(move |(sender_vk, msg)| {
                match msg {
                    Message::Connect(msg) => {
                        Self::_handle_connect(wallet_handle, forward_agent_vk, sender_vk, msg)
                    }
                    _ => future::err(err_msg("Unsupported message")).into_box()
                }
            })
            .into_box()
    }

    fn _handle_forward_agent_pairwise_message(_msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("ForwardAgent::_handle_forward_agent_pairwise_message >> {:?}", _msg);
        unimplemented!()
    }

    fn _handle_connect(wallet_handle: i32,
                       forward_agent_vk: String,
                       sender_vk: String,
                       msg: Connect) -> ResponseFuture<Vec<u8>, Error> {
        trace!("ForwardAgent::_handle_connect >> {:?}, {:?}, {:?}, {:?}",
               wallet_handle, forward_agent_vk, sender_vk, msg);

        let Connect { from_did: their_did, from_did_verkey: their_verkey } = msg;

        if their_verkey != sender_vk {
            return err!(err_msg("Inconsistent sender and connection verkeys"));
        };

        let their_did_info = json!({
            "did": their_did,
            "verkey": their_verkey,
        })
            .to_string();

        let pairwise_metadata = ftry!(
            serde_json::to_string(&PairwiseMetadata {})
                .map_err(|err| err.context("Can't serialize connection pairwise_metadata."))
        );

        future::ok(())
            .and_then(move |_| {
                // FIXME: Return specific error for already exists case
                did::store_their_did(wallet_handle, &their_did_info)
                    .map_err(|err| err.context("Can't store their DID for connection pairwise.").into())
            })
            .and_then(move |_| {
                did::create_and_store_my_did(wallet_handle, "{}")
                    .map_err(|err| err.context("Can't create my DID for connection pairwise.").into())
            })
            .and_then(move |(my_did, my_verkey)| {
                pairwise::create_pairwise(wallet_handle, &their_did, &my_did, &pairwise_metadata)
                    .map(|_| (my_did, my_verkey))
                    .map_err(|err| err.context("Can't store connection pairwise.").into())
            })
            .and_then(move |(my_did, my_verkey)| {
                let msg = Message::Connected(Connected {
                    with_pairwise_did: my_did,
                    with_pairwise_did_verkey: my_verkey,
                });

                bundle_authcrypted(wallet_handle, &forward_agent_vk, &their_verkey, &msg)
                    .map_err(|err| err.context("Can't bundle and authcrypt connected message.").into())
            })
            .into_box()
    }
}

impl Actor for ForwardAgent {
    type Context = Context<Self>;
}

#[derive(Debug)]
pub struct GetEndpointDetails {}

#[derive(Debug, Serialize)]
pub struct EndpointDetails {
    #[serde(rename = "DID")]
    pub did: String,
    #[serde(rename = "verKey")]
    pub verkey: String,
}

impl ActorMessage for GetEndpointDetails {
    type Result = Result<EndpointDetails, Error>;
}

impl Handler<GetEndpointDetails> for ForwardAgent {
    type Result = Result<EndpointDetails, Error>;

    fn handle(&mut self, msg: GetEndpointDetails, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<GetEndpointDetails>::handle >> msg: {:?}", msg);
        let (did, verkey) = self.get_endpoint_details();
        let res = Ok(EndpointDetails { did, verkey });
        trace!("Handler<GetEndpointDetails>::handle <<< {:?}", res);
        res
    }
}

#[derive(Debug)]
pub struct ForwardMessage(pub Vec<u8>);

#[derive(Debug, Serialize)]
pub struct ForwardMessageResponse(pub Vec<u8>);

impl ActorMessage for ForwardMessage {
    type Result = Result<ForwardMessageResponse, Error>;
}

impl Handler<ForwardMessage> for ForwardAgent {
    type Result = ResponseFuture<ForwardMessageResponse, Error>;

    fn handle(&mut self, msg: ForwardMessage, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<ForwardMessage>::handle >> msg: {:?}", msg);

        self.forward_message(msg.0)
            .from_err()
            .map(|msg| ForwardMessageResponse(msg))
            .into_box()
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

        let res = core.run(
            ForwardAgent::start(ForwardAgentConfig {
                wallet_id: FORWARD_AGENT_WALLET_ID.into(),
                wallet_passphrase: FORWARD_AGENT_WALLET_PASSPHRASE.into(),
                did: FORWARD_AGENT_DID.into(),
                did_seed: Some(FORWARD_AGENT_DID_SEED.into()),
                wallet_storage_type: None,
                wallet_storage_config: None,
                wallet_storage_credentials: None,
            }));

        res.unwrap();
    }
}