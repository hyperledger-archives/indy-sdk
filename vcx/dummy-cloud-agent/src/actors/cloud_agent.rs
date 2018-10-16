use actix::prelude::{Actor, Addr, Context, Handler, Message as ActorMessage, ResponseFuture};
use domain::messages::*;
use domain::config::CloudAgentConfig;
use failure::*;
use futures::*;
use indy::{did, wallet};
use utils::futures::*;
use utils::messages::*;

pub struct CloudAgent {
    wallet_handle: i32,
    owner_did: String,
    did: String,
    verkey: String,
}

impl CloudAgent {
    pub fn start(config: CloudAgentConfig) -> ResponseFuture<Addr<CloudAgent>, Error> {
        Self::new(config)
            .map(|cloud_agent| cloud_agent.start())
            .into_box()
    }

    pub fn new(config: CloudAgentConfig) -> ResponseFuture<CloudAgent, Error> {
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
                }
            })
            .into_box()
    }

    pub fn handle_message(&mut self, msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("CloudAgent::handle_message >> msg: {:?}", msg);

        let wallet_handle = self.wallet_handle.clone();
        let owner_did = self.owner_did.clone();
        let cloud_agent_vk = self.verkey.clone();

        unbundle_authcrypted(wallet_handle, &cloud_agent_vk, &msg)
            .and_then(move |(sender_vk, msg)| {
                match msg {
                    Message::CreateKey(msg) => {
                        Self::_handle_create_key(wallet_handle, owner_did, cloud_agent_vk, sender_vk, msg)
                    }
                    _ => future::err(err_msg("Unsupported message")).into_box()
                }
            })
            .into_box()
    }

    fn _handle_create_key(wallet_handle: i32,
                          owner_did: String,
                          cloud_agent_vk: String,
                          sender_vk: String,
                          msg: CreateKey) -> ResponseFuture<Vec<u8>, Error> {
        trace!("CloudAgent::_handle_create_key >> {:?}, {:?}, {:?}, {:?}, {:?}",
               wallet_handle, owner_did, cloud_agent_vk, sender_vk, msg);

        if msg.from_did != owner_did {
            return err!(err_msg("Inconsistent sender did"));
        }

        if msg.from_did_verkey != sender_vk {
            return err!(err_msg("Inconsistent sender verkey"));
        }

        did::create_and_store_my_did(wallet_handle, "{}")
            .from_err()
            .map(|(pairwise_did, pairwise_vk)| {
                Message::KeyCreated(KeyCreated {
                    with_pairwise_did: pairwise_did,
                    with_pairwise_did_verkey: pairwise_vk,
                })
            })
            .and_then(move |msg| bundle_authcrypted(wallet_handle, &cloud_agent_vk, &sender_vk, &msg))
            .into_box()
    }
}

impl Actor for CloudAgent {
    type Context = Context<Self>;
}

#[derive(Debug)]
pub struct CloudMessage(pub Vec<u8>);

#[derive(Debug, Serialize)]
pub struct CloudMessageResponse(pub Vec<u8>);

impl ActorMessage for CloudMessage {
    type Result = Result<CloudMessageResponse, Error>;
}

impl Handler<CloudMessage> for CloudAgent {
    type Result = ResponseFuture<CloudMessageResponse, Error>;

    fn handle(&mut self, msg: CloudMessage, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<CloudMessage>::handle >> msg: {:?}", msg);

        self.handle_message(msg.0)
            .from_err()
            .map(|msg| CloudMessageResponse(msg))
            .into_box()
    }
}
