use actix::prelude::{Actor, Addr, Context, Handler, Message as ActorMessage, ResponseFuture};
use domain::messages::*;
use failure::*;
use futures::*;
use indy::{did, wallet};
use utils::futures::*;
use utils::messages::*;

pub struct CloudAgent {
    wallet_config: String,
    wallet_credentials: String,
    owner_did: String,
    did: String,
    verkey: String,
}

impl CloudAgent {
    pub fn start() -> ResponseFuture<Addr<CloudAgent>, Error> {
        Self::new()
            .map(|cloud_agent| cloud_agent.start())
            .into_box()
    }

    pub fn new() -> ResponseFuture<CloudAgent, Error> {
        unimplemented!()
    }

    pub fn handle_message(&mut self, msg: Vec<u8>) -> ResponseFuture<Vec<u8>, Error> {
        trace!("CloudAgent::handle_message >> msg: {:?}", msg);

        let wallet_config = self.wallet_config.clone();
        let wallet_credentials = self.wallet_credentials.clone();
        let owner_did = self.owner_did.clone();
        let cloud_agent_did = self.did.clone();
        let cloud_agent_vk = self.verkey.clone();

        future::ok(())
            .and_then(move |_| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map_err(|err| err.context("Can't open Cloud Agent wallet.`").into())
            })
            .and_then(move |wallet_handle|
                unbundle_authcrypted(wallet_handle, &cloud_agent_vk, &msg)
                    .map(move |(sender_vk, msg)| (wallet_handle, sender_vk, msg, cloud_agent_vk))
            )
            .and_then(|(wallet_handle, sender_vk, msg, cloud_agent_vk)| {
                match msg {
                    Message::CreateKey(msg) => {
                        Self::_handle_create_key(wallet_handle, owner_did, cloud_agent_did, cloud_agent_vk, sender_vk, msg)
                    }
                    _ => err!(err_msg("Unsupported message"))
                }
                    .map(move |response| (response, wallet_handle))
            })
            .and_then(|(response, wallet_handle)|
                wallet::close_wallet(wallet_handle)
                    .map(move |_| response)
                    .map_err(|err| err.context("Can't close Cloud Agent wallet.").into())
            )
            .into_box()
    }

    fn _handle_create_key(wallet_handle: i32,
                          owner_did: String,
                          cloud_agent_did: String,
                          cloud_agent_vk: String,
                          sender_vk: String,
                          msg: CreateKey) -> ResponseFuture<Vec<u8>, Error> {
        trace!("CloudAgent::_handle_create_key >> {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
               wallet_handle, owner_did, cloud_agent_did, cloud_agent_vk, sender_vk, msg);

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
