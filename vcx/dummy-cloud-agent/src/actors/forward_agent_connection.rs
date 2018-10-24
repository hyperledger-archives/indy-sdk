use actix::prelude::*;
use actors::{AddA2ARoute, HandleA2AMsg};
use actors::router::Router;
use domain::a2a::*;
use domain::indy::*;
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
    my_verkey: String,
    is_signed_up: bool,
    registrations: Vec<(String, String, String)>, // cloud agent did, wallet_id, wallet_key
}

impl ForwardAgentConnection {
    pub fn create(wallet_handle: i32,
                  their_did: String,
                  their_verkey: String,
                  router: Addr<Router>) -> BoxedFuture<(String, String), Error> {
        trace!("ForwardAgentConnection::create >> {:?}, {:?}, {:?}",
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
                    .map_err(|err| err.context("Can't store their DID for Forward Agent Connection pairwise.").into())
            })
            .and_then(move |(their_did, their_verkey)| {
                did::create_and_store_my_did(wallet_handle, "{}")
                    .map(|(my_did, my_verkey)| (my_did, my_verkey, their_did, their_verkey))
                    .map_err(|err| err.context("Can't create my DID for Forward Agent Connection pairwise.").into())
            })
            .and_then(move |(my_did, my_verkey, their_did, their_verkey)| {
                let state = ForwardAgentConnectionState {
                    is_signed_up: false,
                    registrations: Vec::new(),
                };

                let metadata = ftry!(
                    serde_json::to_string(&state)
                        .map_err(|err| err.context("Can't serialize Forward Agent Connection state."))
                ).to_string();

                pairwise::create_pairwise(wallet_handle, &their_did, &my_did, &metadata)
                    .map(|_| (my_did, my_verkey, their_did, their_verkey))
                    .map_err(|err| err.context("Can't store Forward Agent Connection pairwise.").into())
                    .into_box()
            })
            .and_then(move |(my_did, my_verkey, their_did, their_verkey)| {
                let forward_agent_connection = ForwardAgentConnection {
                    wallet_handle,
                    their_did,
                    their_verkey,
                    my_verkey: my_verkey.clone(),
                    is_signed_up: false,
                    registrations: Vec::new(),
                };

                let forward_agent_connection = forward_agent_connection.start();

                router
                    .send(AddA2ARoute(my_did.clone(), forward_agent_connection.clone().recipient()))
                    .from_err()
                    .map(move |_| (my_did, my_verkey))
                    .map_err(|err: Error| err.context("Can't add route for Forward Agent Connection").into())
            })
            .into_box()
    }

    pub fn restore(wallet_handle: i32,
                   their_did: String,
                   router: Addr<Router>) -> BoxedFuture<(), Error> {
        trace!("ForwardAgentConnection::restore >> {:?}, {:?}",
               wallet_handle, their_did);

        future::ok(())
            .and_then(move |_| {
                pairwise::get_pairwise(wallet_handle, &their_did)
                    .map(|pairwise_info| (pairwise_info, their_did))
                    .map_err(|err| err.context("Can't get Forward Agent Connection pairwise.").into())
            })
            .and_then(move |(pairwise_info, their_did)| {
                serde_json::from_str::<PairwiseInfo>(&pairwise_info)
                    .map(|pairwise_info| (pairwise_info, their_did))
                    .map_err(|err| err.context("Can't parse Forward Agent Connection pairwise info.").into())
            })
            .and_then(move |(pairwise_info, their_did)| {
                let PairwiseInfo { my_did, metadata: pairwise_metadata } = pairwise_info;

                serde_json::from_str::<ForwardAgentConnectionState>(&pairwise_metadata)
                    .map(|state| (my_did, their_did, state))
                    .map_err(|err| err.context("Can't parse Forward Agent Connection pairwise info.").into())
            })
            .and_then(move |(my_did, their_did, state)| {
                let my_verkey_fut = did::key_for_local_did(wallet_handle, &my_did)
                    .map_err(|err| err.context("Can't get Forward Agent Connection my did key").into());

                let their_verkey_fut = did::key_for_local_did(wallet_handle, &their_did)
                    .map_err(|err| err.context("Can't get Forward Agent Connection their did key").into());

                my_verkey_fut
                    .join(their_verkey_fut)
                    .map(|(my_verkey, their_verkey)| (my_did, my_verkey, their_did, their_verkey, state))
            })
            .and_then(move |(my_did, my_verkey, their_did, their_verkey, state)| {
                let ForwardAgentConnectionState { is_signed_up, registrations } = state;

                let forward_agent_connection = ForwardAgentConnection {
                    wallet_handle,
                    their_did,
                    their_verkey,
                    my_verkey,
                    is_signed_up,
                    registrations,
                };

                let forward_agent_connection = forward_agent_connection.start();

                router
                    .send(AddA2ARoute(my_did.clone(), forward_agent_connection.clone().recipient()))
                    .from_err()
                    .map_err(|err: Error| err.context("Can't add route for Forward Agent Connection").into())
            })
            .into_box()
    }

    fn _handle_a2a_msg(&mut self,
                       msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::handle_message >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::unbundle_authcrypted(slf.wallet_handle, &slf.my_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle a2a message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, _| {
                if slf.their_verkey != sender_vk {
                    return err_act!(slf, err_msg("Inconsistent sender and connection pairwise verkeys"));
                };

                match msgs.pop() {
                    Some(A2AMessage::SignUp(msg)) => {
                        slf._sign_up(msg)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    fn _sign_up(&mut self, msg: SignUp) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::handle_message >> {:?}", msg);

        if self.is_signed_up {
            return err_act!(self, err_msg("Already signed up"));
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
        self._handle_a2a_msg(msg.0)
    }
}