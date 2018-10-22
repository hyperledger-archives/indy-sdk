use actix::prelude::*;
use actors::HandleA2AMsg;
use domain::a2a::*;
use domain::pairwise::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, pairwise};
use serde_json;
use std::convert::Into;
use utils::futures::*;

pub struct ForwardAgentConnection {
    wallet_handle: i32,
    their_did: String,
    their_verkey: String,
    my_did: String,
    my_verkey: String,
    is_signed_up: bool,
    registrations: Vec<(String, String, String)>, // cloud agent did, wallet_id, wallet_key
}

impl ForwardAgentConnection {
    pub fn establish(wallet_handle: i32,
                     their_did: String,
                     their_verkey: String) -> BoxedFuture<ForwardAgentConnection, Error> {
        trace!("ForwardAgentConnection::establish >> {:?}, {:?}, {:?}",
               wallet_handle, their_did, their_verkey);

        future::ok(())
            .and_then(move |_| {
                let their_did_info = json!({
                    "did": their_did,
                    "verkey": their_verkey,
                }).to_string();

                // FIXME: Return specific error for already exists case
                did::store_their_did(wallet_handle, &their_did_info)
                    .map(|_| (their_did, their_verkey))
                    .map_err(|err| err.context("Can't store their DID for connection pairwise.").into())
            })
            .and_then(move |(their_did, their_verkey)| {
                did::create_and_store_my_did(wallet_handle, "{}")
                    .map(|(my_did, my_verkey)| (my_did, my_verkey, their_did, their_verkey))
                    .map_err(|err| err.context("Can't create my DID for connection pairwise.").into())
            })
            .and_then(move |(my_did, my_verkey, their_did, their_verkey)| {
                let state = ForwardAgentConnectionState {
                    is_signed_up: false,
                    registrations: Vec::new(),
                };

                let metadata = ftry!(
                    serde_json::to_string(&state)
                        .map_err(|err| err.context("Can't serialize connection state."))
                ).to_string();

                pairwise::create_pairwise(wallet_handle, &their_did, &my_did, &metadata)
                    .map(|_| (my_did, my_verkey, their_did, their_verkey))
                    .map_err(|err| err.context("Can't store connection pairwise.").into())
                    .into_box()
            })
            .map(move |(my_did, my_verkey, their_did, their_verkey)| {
                ForwardAgentConnection {
                    wallet_handle,
                    their_did,
                    their_verkey,
                    my_did,
                    my_verkey,
                    is_signed_up: false,
                    registrations: Vec::new(),
                }
            })
            .into_box()
    }

    pub fn get_endpoint(&self) -> (String, String) {
        trace!("ForwardAgent::get_endpoint >>");
        (self.my_did.clone(), self.my_verkey.clone())
    }

    fn handle_a2a_msg(&mut self,
                      msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::handle_message >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::unbundle_authcrypted(slf.wallet_handle, &slf.my_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle a2a message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, ctx| {
                if slf.their_verkey != sender_vk {
                    return err_act!(slf, err_msg("Inconsistent sender and connection pairwise verkeys"))
                };

                match msgs.pop() {
                    Some(A2AMessage::SignUp(msg)) => {
                        slf.sign_up(msg)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    fn sign_up(&mut self, msg: SignUp) -> ResponseActFuture<Self, Vec<u8>, Error> {
        if self.is_signed_up {
            return err_act!(self, err_msg("Already signed up"))
        };

        self.is_signed_up = true;

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _| {
                let state = ForwardAgentConnectionState {
                    is_signed_up: slf.is_signed_up,
                    registrations: slf.registrations.clone(),
                };

                let metadata = ftry_act!(slf, {
                    serde_json::to_string(&state)
                        .map_err(|err| err.context("Can't serialize connection state."))
                });

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.their_did, &metadata)
                    .map_err(|err| err.context("Can't store connection pairwise.").into())
                    .into_actor(slf)
                    .into_box()
            })
            .and_then(|_, slf, _| {
                let msgs = vec![A2AMessage::SignedUp(SignedUp {})];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.my_verkey, &slf.their_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt connected message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }
}

impl Actor for ForwardAgentConnection {
    type Context = Context<Self>;
}


impl Handler<HandleA2AMsg> for ForwardAgentConnection {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<ForwardMessage>::handle >> {:?}", msg);
        self.handle_a2a_msg(msg.0)
    }
}