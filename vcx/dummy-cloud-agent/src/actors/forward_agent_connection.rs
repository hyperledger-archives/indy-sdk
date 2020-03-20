use std::convert::Into;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use futures::future::Either;
use serde_json;

use crate::actors::{HandleA2AMsg, HandleAdminMessage};
use crate::actors::admin::Admin;
use crate::actors::agent::Agent;
use crate::actors::router::Router;
use crate::domain::a2a::*;
use crate::domain::admin_message::ResAdminQuery;
use crate::domain::config::WalletStorageConfig;
use crate::domain::invite::ForwardAgentDetail;
use crate::domain::key_derivation::{KeyDerivationDirective, KeyDerivationFunction};
use crate::indy::{did, pairwise, pairwise::PairwiseInfo, WalletHandle};
use crate::utils::futures::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AgentWalletInfo {
    /// ID of agent's wallet
    pub wallet_id: String,
    /// DID of this agent (Agent.DID@Agent:Owner), addressable via router
    pub agent_did: String,
    /// Information about how to generate key to open agent's wallet
    pub kdf_directive: KeyDerivationDirective,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ForwardAgentConnectionState {
    pub is_signed_up: bool,
    pub agent: Option<(String, String, String)>,
    //  agent's (wallet_id, wallet_key, agent_did)
    pub agent_v2: Option<AgentWalletInfo>,
}

/// Converts the legacy agent state tuple (wallet_id, wallet_key, agent_did) into new data structure
/// AgentWalletInfo for keeping record about a previously created agent
fn convert_from_legacy_agent_to_agent_wallet(agent: (String, String, String)) -> AgentWalletInfo {
    AgentWalletInfo {
        wallet_id: agent.0,
        agent_did: agent.2,
        kdf_directive: KeyDerivationDirective {
            kdf: KeyDerivationFunction::Argon2iMod, // old agents were using Argon2iMod by default
            key: agent.1,
        },
    }
}

/// Represents pairwise connection between the agency and its client
pub struct ForwardAgentConnection {
    wallet_handle: WalletHandle,
    /// The DID of the owner of this connection (Owner.DID@Client:FWAC)
    owner_did: String,
    /// The verkey of the owner of this connection (Owner.VK@Client:FWAC)
    owner_verkey: String,
    /// Forward Agent Connection VKey (FWAC.VK@FWAC:Owner) (FWAC stands for "Forward Agent Connection"), addressable via router
    fwac_verkey: String,
    /// Metadata about the connection and possibly Agent bootstrapped off this connection
    state: ForwardAgentConnectionState,
    router: Rc<RwLock<Router>>,
    admin: Option<Arc<RwLock<Admin>>>,
    forward_agent_detail: ForwardAgentDetail,
    wallet_storage_config: WalletStorageConfig,
}

impl ForwardAgentConnection {
    /// Returns information about created Forward Agent Connection (FWAC)
    /// (FWAC.DID@FWAC:Client, FWAC.VK@FWAC:Client)
    /// representing pairwise relationship with a VCX client of Agency
    ///
    /// # Arguments
    ///
    /// * `wallet_handle` - Agency forward agent wallet handle. This actor has responsibility to
    /// update records about this connection this agency wallet
    /// * `owner_did` - The DID of the owner of this connection ( Owner.DID@Client:FWAC )
    /// * `owner_verkey` - The verkey of the owner of this connection ( Owner.VK@Client:FWAC )
    /// * `router` - Reference to Router actor
    /// * `forward_agent_detail` - Information about Agency's forward agent
    /// * `wallet_storage_config` - Configuration data to access wallet storage used across Agency
    /// * `admin` - Reference to Admin actor
    ///
    pub fn create(agency_wallet_handle: WalletHandle,
                  owner_did: String,
                  owner_verkey: String,
                  router: Rc<RwLock<Router>>,
                  forward_agent_detail: ForwardAgentDetail,
                  wallet_storage_config: WalletStorageConfig,
                  admin: Option<Arc<RwLock<Admin>>>) -> BoxedFuture<(String, String), Error> {
        debug!("ForwardAgentConnection::create >> {:?}, {:?}, {:?}, {:?}, {:?}",
               agency_wallet_handle, owner_did, owner_verkey, forward_agent_detail, wallet_storage_config);

        future::ok(())
            .and_then(move |_| {
                let their_did_info = json!({
                    "did": owner_did,
                    "verkey": owner_verkey,
                }).to_string();

                // FIXME: Return specific error for already exists case
                did::store_their_did(agency_wallet_handle, &their_did_info)
                    .map(|_| (owner_did, owner_verkey))
                    .map_err(|err| err.context("Can't store their DID for Forward Agent Connection pairwise.").into())
            })
            .and_then(move |(owner_did, owner_verkey)| {
                did::create_and_store_my_did(agency_wallet_handle, "{}")
                    .map(|(fwac_did, fwac_verkey)| (fwac_did, fwac_verkey, owner_did, owner_verkey))
                    .map_err(|err| err.context("Can't create my DID for Forward Agent Connection pairwise.").into())
            })
            .and_then(move |(fwac_did, fwac_verkey, owner_did, owner_verkey)| {
                let state = ForwardAgentConnectionState {
                    is_signed_up: false,
                    agent: None,
                    agent_v2: None,
                };

                let metadata = ftry!(
                    serde_json::to_string(&state)
                        .map_err(|err| err.context("Can't serialize Forward Agent Connection state."))
                ).to_string();

                pairwise::create_pairwise(agency_wallet_handle, &owner_did, &fwac_did, Some(&metadata))
                    .map(|_| (fwac_did, fwac_verkey, owner_did, owner_verkey, state))
                    .map_err(|err| err.context("Can't store Forward Agent Connection pairwise.").into())
                    .into_box()
            })
            .and_then(move |(fwac_did, fwac_verkey, owner_did, owner_verkey, state)| {
                let forward_agent_connection = ForwardAgentConnection {
                    wallet_handle: agency_wallet_handle,
                    owner_did,
                    owner_verkey,
                    fwac_verkey: fwac_verkey.clone(),
                    state,
                    router: router.clone(),
                    admin: admin.clone(),
                    forward_agent_detail,
                    wallet_storage_config,
                };

                let forward_agent_connection = forward_agent_connection.start();

                router.write().unwrap()
                    .add_a2a_route(fwac_did.clone(), fwac_verkey.clone(), forward_agent_connection.clone().recipient());
                if let Some(admin) = admin {
                    admin.write().unwrap()
                        .register_forward_agent_connection(fwac_did.clone(), forward_agent_connection.clone())
                };
                future::ok((fwac_did, fwac_verkey))
            })
            .into_box()
    }

    /// Restores a previously created and persisted Forward Agent Connection. If the connection
    /// peer has previously managed to create an agent in Agency, the Agent actor is restored
    /// as well.
    ///
    /// # Arguments
    ///
    /// * `wallet_handle` - Handle to the wallet assigned to this agent
    /// * `owner_did` - The DID of the owner of this connection (Owner.DID@Client:FWAC)
    /// * `forward_agent_detail` - Information about Agency's forward agent
    /// * `wallet_storage_config` - Configuration data to access wallet storage used across Agency
    /// * `router` - Reference to Router actor
    /// * `admin` - Reference to Admin actor
    ///
    pub fn restore(wallet_handle: WalletHandle,
                   owner_did: String,
                   forward_agent_detail: ForwardAgentDetail,
                   wallet_storage_config: WalletStorageConfig,
                   router: Rc<RwLock<Router>>,
                   admin: Option<Arc<RwLock<Admin>>>) -> BoxedFuture<(), Error> {
        debug!("ForwardAgentConnection::restore >> {:?}, {:?}, {:?}, {:?}",
               wallet_handle, owner_did, forward_agent_detail, wallet_storage_config);

        future::ok(())
            .and_then(move |_| {
                pairwise::get_pairwise(wallet_handle, &owner_did)
                    .map(|pairwise_info| (pairwise_info, owner_did))
                    .map_err(|err| err.context("Can't get Forward Agent Connection pairwise.").into())
            })
            .and_then(move |(pairwise_info, owner_did)| {
                serde_json::from_str::<PairwiseInfo>(&pairwise_info)
                    .map(|pairwise_info| (pairwise_info, owner_did))
                    .map_err(|err| err.context("Can't parse PairwiseInfo while restoring Forward Agent Connection.").into())
            })
            .and_then(move |(pairwise_info, owner_did)| {
                let PairwiseInfo { my_did: fwac_did, metadata: pairwise_metadata } = pairwise_info;

                serde_json::from_str::<ForwardAgentConnectionState>(&pairwise_metadata)
                    .map(|state| (fwac_did, owner_did, state))
                    .map_err(|err| err.context("Can't parse ForwardAgentConnectionState while restoring Forward Agent Connection.").into())
            })
            .and_then(move |(fwac_did, owner_did, state)| {
                let fwac_verkey_fut = did::key_for_local_did(wallet_handle, &fwac_did)
                    .map_err(|err| err.context("Can't get Forward Agent Connection my did key").into());

                let owner_verkey_fut = did::key_for_local_did(wallet_handle, &owner_did)
                    .map_err(|err| err.context("Can't get Forward Agent Connection their did key").into());

                fwac_verkey_fut
                    .join(owner_verkey_fut)
                    .map(|(fwac_verkey, owner_verkey)| (fwac_did, fwac_verkey, owner_did, owner_verkey, state))
            })
            .and_then(move |(fwac_did, fwac_verkey, owner_did, owner_verkey, state)| {
                debug!("Restoring agent from state: {:?}", state);
                {
                    let agent_v2: Option<AgentWalletInfo> = match state.agent_v2.clone() {
                        Some(agent_v2) => Some(agent_v2),
                        None => {
                            match state.agent.clone() {
                                Some(legacy_format) => Some(convert_from_legacy_agent_to_agent_wallet(legacy_format)),
                                None => None
                            }
                        }
                    };
                    {
                        if let Some(agent_v2) = agent_v2 {
                            Agent::restore(&agent_v2.wallet_id,
                                           &agent_v2.kdf_directive,
                                           &agent_v2.agent_did,
                                           &owner_did,
                                           &owner_verkey,
                                           router.clone(),
                                           forward_agent_detail.clone(),
                                           wallet_storage_config.clone(),
                                           admin.clone())
                                .into_box()
                        } else {
                            ok!(())
                        }
                    }
                }
                    .map(|_| (fwac_did, fwac_verkey, owner_did, owner_verkey, state,
                              router, admin, forward_agent_detail, wallet_storage_config))
                    .map_err(|err| err.context("Can't start Agent for Forward Agent Connection.").into())
            })
            .and_then(move |(fwac_did, fwac_verkey, owner_did, owner_verkey, state,
                                router, admin, forward_agent_detail, wallet_storage_config)| {
                let forward_agent_connection = ForwardAgentConnection {
                    wallet_handle,
                    owner_did,
                    owner_verkey,
                    fwac_verkey: fwac_verkey.clone(),
                    state,
                    router: router.clone(),
                    admin: admin.clone(),
                    forward_agent_detail,
                    wallet_storage_config,
                };

                let forward_agent_connection = forward_agent_connection.start();
                router.write().unwrap()
                    .add_a2a_route(fwac_did.clone(), fwac_verkey.clone(), forward_agent_connection.clone().recipient());
                if let Some(admin) = admin {
                    admin.write().unwrap()
                        .register_forward_agent_connection(fwac_did.clone(), forward_agent_connection.clone())
                };
                future::ok(())
            })
            .into_box()
    }

    /// Handles encrypted message which has been addressed for this actor, presumably coming from
    /// the peer of this pairwise relationship represented by this forward agency connection.
    ///
    /// # Arguments
    ///
    /// * `msg` - Authccrypted data addressed for this actor, forwarded by Router actor
    ///
    fn _handle_a2a_msg(&mut self,
                       msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.fwac_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle a2a message.").into())
                    .into_actor(slf)
            })
            .and_then(move |(sender_vk, mut msgs), slf, _| {
                if slf.owner_verkey != sender_vk {
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

    /// Handles messages types used for creating agent in Agency.
    /// See method onboarding_v1 in VCX library.
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

    /// Handles messages types used for creating agent in Agency.
    /// See method onboarding_v2 in VCX library.
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

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.fwac_verkey, &slf.owner_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt signed up message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _sign_up_v2(&mut self, msg: SignUp) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_sign_up_v2 >> {:?}", msg);

        self._sign_up()
            .and_then(|_, slf, _| {
                let msg = A2AMessageV2::SignedUp(SignedUp {});

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.fwac_verkey), &slf.owner_verkey, &msg)
                    .map_err(|err| err.context("Can't pack signed up message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    /// Agency client needs send "SignUp" to sing himself up before he can create his Agent
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

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.owner_did, &metadata)
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

                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.fwac_verkey, &slf.owner_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt agent created message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn _create_agent_v2(&mut self, msg: CreateAgent) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("ForwardAgentConnection::_create_agent_v2 >> {:?}", msg);

        self._create_agent()
            .and_then(|(agent_did, agent_verkey), slf, _| {
                let msg = A2AMessageV2::AgentCreated(AgentCreated {
                    with_pairwise_did: agent_did,
                    with_pairwise_did_verkey: agent_verkey,
                });

                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.fwac_verkey), &slf.owner_verkey, &msg)
                    .map_err(|err| err.context("Can't pack agent created message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    /// Creates agent for peer of pairwise relationship represented by this Forward Agent Connection
    fn _create_agent(&mut self) -> ResponseActFuture<Self, (String, String), Error> {
        trace!("ForwardAgentConnection::_create_agent >> ");

        if !self.state.is_signed_up {
            return err_act!(self, err_msg("Sign up is required."));
        };

        if self.state.agent.is_some() || self.state.agent_v2.is_some() {
            return err_act!(self, err_msg("Agent already created."));
        };

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _| {
                Agent::create(&slf.owner_did,
                              &slf.owner_verkey,
                              slf.router.clone(),
                              slf.forward_agent_detail.clone(),
                              slf.wallet_storage_config.clone(),
                              slf.admin.clone(),
                )
                    .into_actor(slf)
                    .into_box()
            })
            .and_then(|(wallet_id, agent_did, agent_verkey, kdf_directive), slf, _| {
                slf.state.agent_v2 = Some(AgentWalletInfo {
                    wallet_id,
                    agent_did: agent_did.clone(),
                    kdf_directive,
                });

                let metadata = ftry_act!(slf, {
                    serde_json::to_string(&slf.state)
                        .map_err(|err| err.context("Can't serialize agent reference."))
                });

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.owner_did, &metadata)
                    .map(move |_| (agent_did, agent_verkey))
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
    use crate::actors::ForwardA2AMsg;
    use crate::utils::tests::*;

    use super::*;

    #[test]
    fn should_convert_legacy_agent_state() {
        let wallet_id = "foo";
        let wallet_key = "bar";
        let did = "ReaAUqa9EajLJajMS3nsxr";
        let agent_info = convert_from_legacy_agent_to_agent_wallet((wallet_id.into(), wallet_key.into(), did.into()));
        assert_eq!(agent_info.kdf_directive.kdf, KeyDerivationFunction::Argon2iMod);
        assert_eq!(agent_info.kdf_directive.key, wallet_key);
        assert_eq!(agent_info.agent_did, did);
    }

    #[test]
    fn forward_agent_connection_signup_works() {
        run_test(|forward_agent, _| {
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
                .map(|e_wallet_handle|
                    crate::indy::wallet::close_wallet(e_wallet_handle).wait().unwrap())
        });
    }

    #[test]
    fn forward_agent_connection_create_agent_works() {
        run_test(|forward_agent, _| {
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
                .map(|e_wallet_handle| crate::indy::wallet::close_wallet(e_wallet_handle).wait().unwrap())
        });
    }
}
