use actix::prelude::*;
use actors::{AddA2ARoute, HandleA2AMsg};
use actors::agent::Agent;
use actors::router::Router;
use domain::a2a::*;
use domain::config::WalletStorageConfig;
use domain::invite::ForwardAgentDetail;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, pairwise, pairwise::PairwiseInfo};
use serde_json;
use std::convert::Into;
use utils::futures::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct ForwardAgentConnectionState {
    pub is_signed_up: bool,
    pub agent: Option<(String, String, String)>,
    //  agent's (wallet_id, wallet_key, did)
}

pub struct ForwardAgentConnection {
    wallet_handle: i32,
    their_did: String,
    their_verkey: String,
    my_verkey: String,
    state: ForwardAgentConnectionState,
    router: Addr<Router>,
    forward_agent_detail: ForwardAgentDetail,
    wallet_storage_config: WalletStorageConfig,
}

impl ForwardAgentConnection {
    pub fn create(wallet_handle: i32,
                  their_did: String,
                  their_verkey: String,
                  router: Addr<Router>,
                  forward_agent_detail: ForwardAgentDetail,
                  wallet_storage_config: WalletStorageConfig) -> BoxedFuture<(String, String), Error> {
        trace!("ForwardAgentConnection::create >> {:?}, {:?}, {:?}, {:?}, {:?}",
               wallet_handle, their_did, their_verkey, forward_agent_detail, wallet_storage_config);

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
                    agent: None,
                };

                let metadata = ftry!(
                    serde_json::to_string(&state)
                        .map_err(|err| err.context("Can't serialize Forward Agent Connection state."))
                ).to_string();

                pairwise::create_pairwise(wallet_handle, &their_did, &my_did, Some(&metadata))
                    .map(|_| (my_did, my_verkey, their_did, their_verkey, state))
                    .map_err(|err| err.context("Can't store Forward Agent Connection pairwise.").into())
                    .into_box()
            })
            .and_then(move |(my_did, my_verkey, their_did, their_verkey, state)| {
                let forward_agent_connection = ForwardAgentConnection {
                    wallet_handle,
                    their_did,
                    their_verkey,
                    my_verkey: my_verkey.clone(),
                    state,
                    router: router.clone(),
                    forward_agent_detail,
                    wallet_storage_config,
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
                   forward_agent_detail: ForwardAgentDetail,
                   wallet_storage_config: WalletStorageConfig,
                   router: Addr<Router>) -> BoxedFuture<(), Error> {
        trace!("ForwardAgentConnection::restore >> {:?}, {:?}, {:?}, {:?}",
               wallet_handle, their_did, forward_agent_detail, wallet_storage_config);

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
                if let Some((agent_wallet_id, agent_wallet_key, agent_did)) = state.agent.clone() {
                    Agent::restore(&agent_wallet_id,
                                   &agent_wallet_key,
                                   &agent_did,
                                   &their_did,
                                   &their_verkey,
                                   router.clone(),
                                   forward_agent_detail.clone(),
                                   wallet_storage_config.clone())
                        .into_box()
                } else {
                    ok!(())
                }
                    .map(|_| (my_did, my_verkey, their_did, their_verkey, state,
                              router, forward_agent_detail, wallet_storage_config))
                    .map_err(|err| err.context("Can't start Agent for Forward Agent Connection.").into())
            })
            .and_then(move |(my_did, my_verkey, their_did, their_verkey, state,
                                router, forward_agent_detail, wallet_storage_config)| {
                let forward_agent_connection = ForwardAgentConnection {
                    wallet_handle,
                    their_did,
                    their_verkey,
                    my_verkey,
                    state,
                    router: router.clone(),
                    forward_agent_detail,
                    wallet_storage_config,
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
        trace!("ForwardAgentConnection::_handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.my_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle a2a message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, _| {
                if slf.their_verkey != sender_vk {
                    return err_act!(slf, err_msg("Inconsistent sender and connection pairwise verkeys"));
                };

                match msgs.pop() {
                    Some(A2AMessage::Version1(msg)) => slf._handle_a2a_msg_v1(msg),
                    Some(A2AMessage::Version2(msg)) => slf._handle_a2a_msg_v2(msg),
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    fn _handle_a2a_msg_v1(&mut self,
                          msg: A2AMessageV1) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_handle_a2a_msg_v1 >> {:?}", msg);

        match msg {
            A2AMessageV1::SignUp(msg) => {
                self._sign_up_v1(msg)
            }
            A2AMessageV1::CreateAgent(msg) => {
                self._create_agent_v1(msg)
            }
            _ => err_act!(self, err_msg("Unsupported message"))
        }
    }

    fn _handle_a2a_msg_v2(&mut self,
                          msg: A2AMessageV2) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_handle_a2a_msg_v2 >> {:?}", msg);

        match msg {
            A2AMessageV2::SignUp(msg) => {
                self._sign_up_v2(msg)
            }
            A2AMessageV2::CreateAgent(msg) => {
                self._create_agent_v2(msg)
            }
            _ => err_act!(self, err_msg("Unsupported message"))
        }
    }

    fn _sign_up_v1(&mut self, msg: SignUp) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_sign_up_v1 >> {:?}", msg);

        self._sign_up()
            .and_then(|_, slf, _| {
                let msgs = vec![A2AMessage::Version1(A2AMessageV1::SignedUp(SignedUp {}))];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.my_verkey, &slf.their_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt signed up message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _sign_up_v2(&mut self, msg: SignUp) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_sign_up_v2 >> {:?}", msg);

        self._sign_up()
            .and_then(|_, slf, _| {
                let msg = A2AMessageV2::SignedUp(SignedUp { });

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.my_verkey), &slf.their_verkey, &msg)
                    .map_err(|err| err.context("Can't pack signed up message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _sign_up(&mut self) -> ResponseActFuture<Self, (), Error> {
        trace!("ForwardAgentConnection::_sign_up >>");

        if self.state.is_signed_up {
            return err_act!(self, err_msg("Already signed up"));
        };

        self.state.is_signed_up = true;

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _| {
                let metadata = ftry_act!(slf, {
                    serde_json::to_string(&slf.state)
                        .map_err(|err| err.context("Can't serialize connection state."))
                });

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.their_did, &metadata)
                    .map_err(|err| err.context("Can't store connection pairwise.").into())
                    .into_actor(slf)
                    .into_box()
            })
            .into_box()
    }

    fn _create_agent_v1(&mut self, msg: CreateAgent) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_create_agent_v1 >> {:?}", msg);

        self._create_agent()
            .and_then(|(did, verkey), slf, _| {
                let msgs = vec![A2AMessage::Version1(A2AMessageV1::AgentCreated(AgentCreated {
                    with_pairwise_did: did,
                    with_pairwise_did_verkey: verkey,
                }))];

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.my_verkey, &slf.their_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt agent created message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _create_agent_v2(&mut self, msg: CreateAgent) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_create_agent_v2 >> {:?}", msg);

        self._create_agent()
            .and_then(|(did, verkey), slf, _| {
                let msg = A2AMessageV2::AgentCreated(AgentCreated {
                    with_pairwise_did: did,
                    with_pairwise_did_verkey: verkey,
                });

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.my_verkey), &slf.their_verkey, &msg)
                    .map_err(|err| err.context("Can't pack agent created message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _create_agent(&mut self) -> ResponseActFuture<Self, (String, String), Error> {
        trace!("ForwardAgentConnection::_create_agent >> ");

        if !self.state.is_signed_up {
            return err_act!(self, err_msg("Sign up is required."));
        };

        if self.state.agent.is_some() {
            return err_act!(self, err_msg("Agent already created."));
        };

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _| {
                Agent::create(&slf.their_did,
                              &slf.their_verkey,
                              slf.router.clone(),
                              slf.forward_agent_detail.clone(),
                              slf.wallet_storage_config.clone())
                    .into_actor(slf)
                    .into_box()
            })
            .and_then(|(wallet_id, wallet_key, did, verkey), slf, _| {
                slf.state.agent = Some((wallet_id, wallet_key, did.clone()));

                let metadata = ftry_act!(slf, {
                    serde_json::to_string(&slf.state)
                        .map_err(|err| err.context("Can't serialize connection state."))
                });

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.their_did, &metadata)
                    .map(move |_| (did, verkey))
                    .map_err(|err| err.context("Can't store connection pairwise.").into())
                    .into_actor(slf)
                    .into_box()
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
        trace!("Handler<HandleA2AMsg>::handle >> {:?}", msg);
        self._handle_a2a_msg(msg.0)
    }
}

#[cfg(test)]
mod tests {
    use actors::ForwardA2AMsg;
    use super::*;
    use utils::tests::*;

    #[test]
    fn forward_agent_connection_signup_works() {
        run_test(|forward_agent| {
            future::ok(())
                .and_then(|()| {
                    let e_wallet_handle = edge_wallet_setup().wait().unwrap();
                    let connect_msg = compose_connect(e_wallet_handle).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(connect_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |connected_msg| (forward_agent, e_wallet_handle, connected_msg))
                })
                .and_then(|(forward_agent, e_wallet_handle, connected_msg)| {
                    let (sender_verkey, pairwise_did, pairwise_verkey) = decompose_connected(e_wallet_handle, &connected_msg).wait().unwrap();
                    assert_eq!(sender_verkey, FORWARD_AGENT_DID_VERKEY);
                    assert!(!pairwise_did.is_empty());
                    assert!(!pairwise_verkey.is_empty());
                    let signup_msg = compose_signup(e_wallet_handle, &pairwise_did, &pairwise_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(signup_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |signedup_msg| (e_wallet_handle, signedup_msg, pairwise_verkey))
                })
                .map(|(e_wallet_handle, signedup_msg, pairwise_verkey)| {
                    let sender_verkey = decompose_signedup(e_wallet_handle, &signedup_msg).wait().unwrap();
                    assert_eq!(sender_verkey, pairwise_verkey);
                    e_wallet_handle
                })
                .map(|e_wallet_handle| ::indy::wallet::close_wallet(e_wallet_handle).wait().unwrap())
        });
    }

    #[test]
    fn forward_agent_connection_create_agent_works() {
        run_test(|forward_agent| {
            future::ok(())
                .and_then(|()| {
                    let e_wallet_handle = edge_wallet_setup().wait().unwrap();
                    let connect_msg = compose_connect(e_wallet_handle).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(connect_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |connected_msg| (forward_agent, e_wallet_handle, connected_msg))
                })
                .and_then(|(forward_agent, e_wallet_handle, connected_msg)| {
                    let (sender_verkey, pairwise_did, pairwise_verkey) = decompose_connected(e_wallet_handle, &connected_msg).wait().unwrap();
                    assert_eq!(sender_verkey, FORWARD_AGENT_DID_VERKEY);
                    assert!(!pairwise_did.is_empty());
                    assert!(!pairwise_verkey.is_empty());
                    let signup_msg = compose_signup(e_wallet_handle, &pairwise_did, &pairwise_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(signup_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |signedup_msg| (forward_agent, e_wallet_handle, signedup_msg, pairwise_did, pairwise_verkey))
                })
                .and_then(move |(forward_agent, e_wallet_handle, signedup_msg, pairwise_did, pairwise_verkey)| {
                    let sender_verkey = decompose_signedup(e_wallet_handle, &signedup_msg).wait().unwrap();
                    assert_eq!(sender_verkey, pairwise_verkey);
                    let create_agent_msg = compose_create_agent(e_wallet_handle, &pairwise_did, &pairwise_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(create_agent_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |agent_created_msg| (e_wallet_handle, agent_created_msg, pairwise_verkey))
                })
                .and_then(|(e_wallet_handle, agent_created_msg, pairwise_verkey)| {
                    decompose_agent_created(e_wallet_handle, &agent_created_msg)
                        .map(move |(sender_vk, pw_did, pw_vk)| {
                            assert_eq!(sender_vk, pairwise_verkey);
                            assert!(!pw_did.is_empty());
                            assert!(!pw_vk.is_empty());
                            e_wallet_handle
                        })
                })
                .map(|e_wallet_handle| ::indy::wallet::close_wallet(e_wallet_handle).wait().unwrap())
        });
    }
}