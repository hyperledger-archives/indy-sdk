use std::collections::HashMap;
use std::convert::Into;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{Error, Fail};
use futures::*;
use serde_json;

use crate::actors::{HandleA2AMsg, HandleAdminMessage};
use crate::actors::admin::Admin;
use crate::actors::agent_connection::agent_connection::AgentConnection;
use crate::actors::router::Router;
use crate::domain::admin_message::{ResAdminQuery, ResQueryAgent};
use crate::domain::config::WalletStorageConfig;
use crate::domain::invite::ForwardAgentDetail;
use crate::domain::key_derivation::KeyDerivationDirective;
use crate::indy::{did, ErrorCode, IndyError, wallet, WalletHandle};
use crate::utils::config_env::*;
use crate::utils::futures::*;
use crate::utils::rand;
use crate::utils::wallet::build_wallet_credentials;

/// Storage
/// - Agent has his own wallet, this contains
/// agent_did + metadata = agent configs (webhookurl, logourl, name)

/// by agent_connections in wallet:
/// their_did: [  {did: user_pairwise_did, verkey: user_pairwise_verkey}, ... ]
/// my_did: [ {did: <generated>, vkey: <generated>} ]
/// pairwise [ {their_did, my_did, metadata: <serialized AgentConnectionState>} ]

/// Represents cloud agent owned and controlled by Agency's client. Agent can receive messages
/// on behalf of its owner. The owner can communicate with the agent to pick up received messages
/// or forward his messages to other destinations.
/// Messages received by this Agent should be encrypted by the receiving edge client's verkey and
/// hence unreadable by Agency.
#[allow(unused)] //FIXME:
pub struct Agent {
    /// Reference to Agent's wallet. Apart from others, the wallet contains pairwise DIDs of this
    /// agent's relationships with other parties
    pub(super) wallet_handle: WalletHandle,
    /// Agent's owner's pairwise DID with its Agent (Owner.DID@Client:Agent)
    pub(super) owner_did: String,
    /// Agent's owner's pairwise verkey for its Agent (Owner.VK@Client:Agent)
    pub(super) owner_verkey: String,
    /// DID of this agent (Agent.DID@Agent:Owner), addressable via router
    pub(super) agent_did: String,
    /// Verkey of this agent (Agent.VK@Agent:Owner), addressable via router
    pub(super) agent_verkey: String,
    /// Information about Agency's forward agent
    pub(super) forward_agent_detail: ForwardAgentDetail,
    /// reference to message router
    pub(super) router: Rc<RwLock<Router>>,
    /// reference to Admin actor
    pub(super) admin: Option<Arc<RwLock<Admin>>>
}

impl Agent {
    /// Returns information about the created agent as
    /// tuple (wallet_id, agent_did, agent_verkey, key_derivation_directive) whereas:
    /// - wallet_id is ID of wallet used by this agent to store data
    /// - agent_did is the DID of the created agent (Agent.DID@Agent:Owner)
    /// - agent_verkey is public key of the created created agent (Agent.VK@Agent:Owner)
    /// - key_derivation_directive describes how to open agent's wallet
    ///
    /// # Arguments
    ///
    /// * `owner_did` - Agent's owner's DID at ClientToAgency relationship (Owner.DID@Client:Agent)
    /// * `owner_verkey` - Agent's owner's verkey at ClientToAgency relationship (Owner.VK@Client:Agent)
    /// * `router` - Reference to Router actor
    /// * `forward_agent_detail` - Information about this agency's Forward Agent
    /// * `wallet_storage_config` - Configuration data to access wallet storage used across Agency
    /// * `admin` - Reference to Admin actor
    pub fn create_record_load_actor(agent_owner_did: String,
                                    agent_owner_verkey: String,
                                    router: Rc<RwLock<Router>>,
                                    forward_agent_detail: ForwardAgentDetail,
                                    wallet_storage_config: WalletStorageConfig,
                                    admin: Option<Arc<RwLock<Admin>>>) -> BoxedFuture<(String, String, String, KeyDerivationDirective), Error> {
        Self::create_record(&agent_owner_did.clone(), wallet_storage_config.clone())
            .and_then(move |(wallet_id, agent_did, agent_verkey, kdf_directives)| {
                Self::load_actor(&wallet_id,
                                 &kdf_directives,
                                 &agent_did.clone(),
                                 &agent_owner_did.clone(),
                                 &agent_owner_verkey.clone(),
                                 router,
                                 forward_agent_detail,
                                 wallet_storage_config,
                                 admin)
                    .map(move |_| (wallet_id, agent_did, agent_verkey, kdf_directives))
            })
            .into_box()
    }

    pub fn create_record(agent_owner_did: &str,
                         wallet_storage_config: WalletStorageConfig) -> BoxedFuture<(String, String, String, KeyDerivationDirective), Error> {
        debug!("Agent::create >> {:?}, {:?}",
               agent_owner_did, wallet_storage_config);

        let wallet_id = format!("dummy_{}_{}", agent_owner_did, rand::rand_string(10));
        let wallet_config = json!({
                    "id": wallet_id.clone(),
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

        future::ok(())
            .and_then(|_| {
                KeyDerivationDirective::new(get_app_env_config().new_agent_kdf.clone())
                    .map_err(|err| err.context("Can't open Agent wallet.`").into())
                    .map(|kdf_directives| {
                        let wallet_credentials = build_wallet_credentials(
                            &kdf_directives,
                            wallet_storage_config.credentials,
                        ).to_string();
                        (wallet_credentials, kdf_directives)
                    })
            })
            .and_then(move |(wallet_credentials, kdf_directives)| {
                wallet::create_wallet(&wallet_config, &wallet_credentials)
                    .map(|_| (wallet_config, wallet_credentials, kdf_directives))
                    .map_err(|err| err.context("Can't create Agent wallet.").into())
            })
            .and_then(move |(wallet_config, wallet_credentials, kdf_directives)| {
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map(move |wallet_handle| (wallet_handle, kdf_directives))
                    .map_err(|err| err.context("Can't open Agent wallet.`").into())
            })
            .and_then(|(wallet_handle, kdf_directives)| {
                did::create_and_store_my_did(wallet_handle, "{}")
                    .map(move |(agent_did, agent_verkey)| (wallet_handle, agent_did, agent_verkey, kdf_directives))
                    .map_err(|err| err.context("Can't get Agent did key").into())
            })
            .and_then(move |(wallet_handle, agent_did, agent_verkey, kdf_directives)| {
                wallet::close_wallet(wallet_handle)
                    .map(move |_| (wallet_id, agent_did, agent_verkey, kdf_directives))
                    .map_err(|err| err.context("Can't close Agent wallet.`").into())
            })
            .into_box()
    }

    /// Loads agent record into memory as actor
    ///
    /// # Arguments
    ///
    /// * `wallet_id` - id of this agent's wallet
    /// * `kdf_directives` - instructions for opening agent's wallet
    /// * `agent_did` - vhe DID of this agent (Agent.DID@Agent:Owner)
    /// * `owner_did` - The DID of the owner of this agent (Owner.DID@Client:Agent)
    /// * `owner_verkey` - The verkey of the owner of this connection (Owner.VK@Client:Agent)
    /// * `router` - Reference to Router actor
    /// * `forward_agent_detail` - Information about Agency's forward agent
    /// * `wallet_storage_c onfig` - Configuration data to access wallet storage used across Agency
    /// * `admin` - Reference to Admin actor
    pub fn load_actor(wallet_id: &str,
                      kdf_directives: &KeyDerivationDirective,
                      agent_did: &str,
                      owner_did: &str,
                      owner_verkey: &str,
                      router: Rc<RwLock<Router>>,
                      forward_agent_detail: ForwardAgentDetail,
                      wallet_storage_config: WalletStorageConfig,
                      admin: Option<Arc<RwLock<Admin>>>) -> BoxedFuture<(), Error> {
        debug!("Agent::restore >> {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
               wallet_id, agent_did, owner_did, owner_verkey, forward_agent_detail, wallet_storage_config);

        let wallet_config = json!({
                    "id": wallet_id.clone(),
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

        let wallet_credentials = build_wallet_credentials(
            &kdf_directives,
            wallet_storage_config.credentials,
        ).to_string();


        let agent_did = agent_did.to_string();
        let owner_did = owner_did.to_string();
        let owner_verkey = owner_verkey.to_string();

        future::ok(())
            .and_then(move |_| {
                debug!("Opening agent wallet {:?}", &wallet_config);
                wallet::open_wallet(wallet_config.as_ref(), wallet_credentials.as_ref())
                    .map_err(move |err| err.context(format!("Can't open Agent wallet using config {:?}.", wallet_config.clone())).into())
            })
            .and_then(move |wallet_handle| {
                did::key_for_local_did(wallet_handle, &agent_did)
                    .map(move |agent_verkey| (wallet_handle, agent_did, agent_verkey))
                    .map_err(|err| err.context("Can't get Agent did verkey.").into())
            })
            .and_then(move |(wallet_handle, agent_did, agent_verkey)| {
                Self::load_configs(wallet_handle, agent_did.clone())
                    .map(move |configs| (wallet_handle, agent_did, agent_verkey, configs))
            })
            .and_then(move |(wallet_handle, agent_did, agent_verkey, configs)| {
                // Resolve information about existing connections from the wallet
                // and start Agent Connection actor for each exists connection

                Agent::_load_connections(wallet_handle,
                                         &agent_did,
                                         &owner_did,
                                         &owner_verkey,
                                         &forward_agent_detail,
                                         router.clone(),
                                         admin.clone(),
                                         configs.clone())
                    .map(move |_| (wallet_handle, agent_did, agent_verkey, owner_did, owner_verkey, forward_agent_detail, router, admin, configs))
            })
            .and_then(move |(wallet_handle, agent_did, agent_verkey, owner_did, owner_verkey, forward_agent_detail, router, admin, configs)| {
                let agent = Agent {
                    wallet_handle,
                    agent_verkey: agent_verkey.clone(),
                    agent_did: agent_did.clone(),
                    owner_did,
                    owner_verkey,
                    router: router.clone(),
                    admin: admin.clone(),
                    forward_agent_detail
                };

                let agent = agent.start();

                router.write().unwrap()
                    .add_a2a_route(agent_did.clone(), agent_verkey.clone(), agent.clone().recipient());
                if let Some(admin) = admin {
                    admin.write().unwrap()
                        .register_agent(agent_did.clone(), agent.clone())
                };
                future::ok(())
            })
            .into_box()
    }

    /// Restores pairwise connections (as Actix Actors) owned by this agent
    ///
    /// # Arguments
    /// * `wallet_handle` - reference to this agent's wallet
    /// * `owner_did` - The DID of the owner of this agent (Owner.DID@Client:Agent)
    /// * `owner_verkey` - The verkey of the owner of this connection (Owner.VK@Client:Agent)
    /// * `forward_agent_detail` - Information about Agency's forward agent
    /// * `router` - Reference to Router actor
    /// * `admin` - Reference to Admin actor
    /// * `wallet_storage_config` - Configuration data to access wallet storage used across Agency
    fn _load_connections(wallet_handle: WalletHandle,
                         agent_did: &str,
                         owner_did: &str,
                         owner_verkey: &str,
                         forward_agent_detail: &ForwardAgentDetail,
                         router: Rc<RwLock<Router>>,
                         admin: Option<Arc<RwLock<Admin>>>,
                         agent_configs: HashMap<String, String>) -> ResponseFuture<(), Error> {
        trace!("Agent::_restore_connections >> {:?}, {:?}, {:?}, {:?}",
               wallet_handle, owner_did, owner_verkey, forward_agent_detail);

        let agent_did = agent_did.to_string();
        let owner_did = owner_did.to_string();
        let owner_verkey = owner_verkey.to_string();
        let forward_agent_detail = forward_agent_detail.clone();

        future::ok(())
            .and_then(move |_| Self::get_pairwise_list(wallet_handle).into_box())
            .and_then(move |pairwise_list| {
                let futures: Vec<_> = pairwise_list
                    .iter()
                    .map(move |pairwise| {
                        AgentConnection::load_actor(wallet_handle,
                                                    &agent_did,
                                                    &owner_did,
                                                    &owner_verkey,
                                                    &pairwise.my_did,
                                                    &pairwise.their_did,
                                                    &pairwise.metadata,
                                                    &forward_agent_detail,
                                                    router.clone(),
                                                    admin.clone())
                    })
                    .collect();

                future::join_all(futures)
                    .map(|_| ())
                    .map_err(|err| err.context("Can't restore Agent connections.").into())
            })
            .into_box()
    }
}

impl Actor for Agent {
    type Context = Context<Self>;
}

impl Handler<HandleA2AMsg> for Agent {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AgentMsgsBundle>::handle >> {:?}", msg);
        self.handle_a2a_msg(msg.0)
    }
}

impl Handler<HandleAdminMessage> for Agent {
    type Result = ResponseActFuture<Self, ResAdminQuery, Error>;

    fn handle(&mut self, _msg: HandleAdminMessage, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Agent Handler<HandleAdminMessage>::handle >>");
        future::ok(())
            .into_actor(self)
            .and_then( |_, slf, _| {
                let owner_did = slf.owner_did.clone();
                let owner_verkey = slf.owner_verkey.clone();
                let did = slf.agent_did.clone();
                let verkey = slf.agent_verkey.clone();
                Self::load_configs(slf.wallet_handle.clone(), slf.agent_did.clone())
                    .map( |configs| {
                        ResAdminQuery::Agent(ResQueryAgent {
                            owner_did,
                            owner_verkey,
                            did,
                            verkey,
                            configs: configs.iter().map(|(key, value)| (key.clone(), value.clone())).collect()
                        })
                    }).into_actor(slf)
            })
            .into_box()
    }
}

#[cfg(test)]
mod tests {
    use crate::actors::ForwardA2AMsg;
    use crate::domain::a2a::{ConfigOption, GetMessagesDetailResponse, MessageDetailPayload, RemoteMessageType};
    use crate::domain::a2connection::MessagesByConnection;
    use crate::domain::status::MessageStatusCode;
    use crate::utils::tests::*;
    use crate::utils::to_i8;

    use super::*;

    #[test]
    fn agent_create_key_works() {
        run_test(|forward_agent, _| {
            future::ok(())
                .and_then(|()| {
                    setup_agent(forward_agent)
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, _, _, forward_agent)| {
                    let (user_pw_did, user_pw_verkey) = did::create_and_store_my_did(e_wallet_handle, "{}").wait().unwrap();

                    let create_key_msg = compose_create_key(e_wallet_handle, &agent_did, &agent_verkey, &user_pw_did, &user_pw_verkey).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(create_key_msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |key_created_msg| (e_wallet_handle, key_created_msg, agent_verkey))
                })
                .map(|(e_wallet_handle, key_created_msg, agent_verkey)| {
                    let (sender_vk, key) = decompose_key_created(e_wallet_handle, &key_created_msg).wait().unwrap();
                    assert_eq!(sender_vk, agent_verkey);
                    assert!(!key.with_pairwise_did.is_empty());
                    assert!(!key.with_pairwise_did_verkey.is_empty());

                    wallet::close_wallet(e_wallet_handle).wait().unwrap();
                })
        });
    }

    #[test]
    fn agent_get_messages_by_connection_works() {
        run_agent_test(|(e_wallet_handle, agent_did, agent_verkey, agent_pw_did, agent_pw_vk, forward_agent)| {
            future::ok(())
                .and_then(move |_| {
                    let msg = compose_create_general_message(e_wallet_handle,
                                                             &agent_did,
                                                             &agent_verkey,
                                                             &agent_pw_did,
                                                             &agent_pw_vk,
                                                             RemoteMessageType::CredOffer).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_did, agent_verkey, forward_agent, agent_pw_did, agent_pw_vk))
                })
                .and_then(move |(e_wallet_handle, resp, agent_did, agent_verkey, forward_agent, agent_pw_did, agent_pw_vk)| {
                    let (_, msg_uid) = decompose_general_message_created(e_wallet_handle, &resp).wait().unwrap();

                    let msg = compose_get_messages_by_connection(e_wallet_handle,
                                                                 &agent_did,
                                                                 &agent_verkey,
                                                                 &agent_pw_did,
                                                                 &agent_pw_vk).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_verkey, msg_uid))
                })
                .map(|(e_wallet_handle, resp, agent_verkey, msg_uid)| {
                    let (sender_vk, messages) = decompose_get_messages_by_connection(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_verkey);
                    assert_eq!(1, messages.len());

                    let expected_message = MessagesByConnection {
                        did: EDGE_PAIRWISE_DID.to_string(),
                        msgs: vec![GetMessagesDetailResponse {
                            uid: msg_uid,
                            status_code: MessageStatusCode::Created,
                            sender_did: EDGE_PAIRWISE_DID.to_string(),
                            type_: RemoteMessageType::CredOffer,
                            payload: Some(MessageDetailPayload::V1(to_i8(&PAYLOAD.to_vec()))),
                            ref_msg_id: None,
                        }],
                    };
                    assert_eq!(expected_message, messages[0]);
                    e_wallet_handle
                })
        });
    }

    #[test]
    fn agent_configs_happy_path() {
        run_test(|forward_agent, _| {
            future::ok(())
                .and_then(|()| {
                    setup_agent(forward_agent)
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, _, _, forward_agent)| {
                    let msg = compose_update_configs(e_wallet_handle,
                                                     &agent_did,
                                                     &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |_| (e_wallet_handle, agent_did, agent_verkey, forward_agent))
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, forward_agent)| {
                    let msg = compose_get_configs(e_wallet_handle,
                                                  &agent_did,
                                                  &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_did, agent_verkey, forward_agent))
                })
                .and_then(move |(e_wallet_handle, resp, agent_did, agent_verkey, forward_agent)| {
                    let configs = decompose_configs(e_wallet_handle, &resp).wait().unwrap();

                    assert_eq!(configs.len(), 2);
                    assert!(configs.contains(&ConfigOption { name: "name".to_string(), value: "super agent".to_string() }));
                    assert!(configs.contains(&ConfigOption { name: "logoUrl".to_string(), value: "http://logo.url".to_string() }));

                    let msg = compose_remove_configs(e_wallet_handle,
                                                     &agent_did,
                                                     &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |_| (e_wallet_handle, agent_did, agent_verkey, forward_agent))
                })
                .and_then(move |(e_wallet_handle, agent_did, agent_verkey, forward_agent)| {
                    let msg = compose_get_configs(e_wallet_handle,
                                                  &agent_did,
                                                  &agent_verkey).wait().unwrap();
                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp))
                })
                .map(|(e_wallet_handle, resp)| {
                    let configs = decompose_configs(e_wallet_handle, &resp).wait().unwrap();

                    assert_eq!(configs.len(), 1);
                    assert!(!configs.contains(&ConfigOption { name: "name".to_string(), value: "super agent".to_string() }));
                    assert!(configs.contains(&ConfigOption { name: "logoUrl".to_string(), value: "http://logo.url".to_string() }));

                    wallet::close_wallet(e_wallet_handle).wait().unwrap();
                })
        });
    }
}
