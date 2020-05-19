use std::collections::HashMap;
use std::convert::Into;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{err_msg, Error, Fail};
use futures::*;
use futures::future::Either;
use serde_json;

use crate::actors::{HandleA2AMsg, HandleAdminMessage, RouteA2ConnMsg};
use crate::actors::admin::Admin;
use crate::actors::agent_connection::{AgentConnection, AgentConnectionConfig};
use crate::actors::router::Router;
use crate::domain::a2a::*;
use crate::domain::a2connection::*;
use crate::domain::admin_message::{ResAdminQuery, ResQueryAgent};
use crate::domain::config::WalletStorageConfig;
use crate::domain::invite::ForwardAgentDetail;
use crate::domain::key_derivation::KeyDerivationDirective;
use crate::indy::{did, ErrorCode, IndyError, pairwise, pairwise::Pairwise, wallet, WalletHandle};
use crate::utils::config_env::*;
use crate::utils::futures::*;
use crate::utils::rand;
use crate::utils::wallet::build_wallet_credentials;

/// Represents cloud agent owned and controlled by Agency's client. Agent can receive messages
/// on behalf of its owner. The owner can communicate with the agent to pick up received messages
/// or forward his messages to other destinations.
/// Messages received by this Agent should be encrypted by the receiving edge client's verkey and
/// hence unreadable by Agency.
#[allow(unused)] //FIXME:
pub struct Agent {
    /// Reference to Agent's wallet. Apart from others, the wallet contains pairwise DIDs of this
    /// agent's relationships with other parties
    wallet_handle: WalletHandle,
    /// Agent's owner's pairwise DID with its Agent (Owner.DID@Client:Agent)
    owner_did: String,
    /// Agent's owner's pairwise verkey for its Agent (Owner.VK@Client:Agent)
    owner_verkey: String,
    /// DID of this agent (Agent.DID@Agent:Owner), addressable via router
    agent_did: String,
    /// Verkey of this agent (Agent.VK@Agent:Owner), addressable via router
    agent_verkey: String,
    /// Information about Agency's forward agent
    forward_agent_detail: ForwardAgentDetail,
    /// reference to message router
    router: Rc<RwLock<Router>>,
    /// reference to Admin actor
    admin: Option<Arc<RwLock<Admin>>>,
    /// Map of keys and values representing arbitrary configuration of the agent that might be
    /// set by the agent's owner
    configs: HashMap<String, String>,
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
    pub fn create(agent_owner_did: &str,
                  agent_owner_verkey: &str,
                  router: Rc<RwLock<Router>>,
                  forward_agent_detail: ForwardAgentDetail,
                  wallet_storage_config: WalletStorageConfig,
                  admin: Option<Arc<RwLock<Admin>>>) -> BoxedFuture<(String, String, String, KeyDerivationDirective), Error> {
        debug!("Agent::create >> {:?}, {:?}, {:?}, {:?}",
               agent_owner_did, agent_owner_verkey, forward_agent_detail, wallet_storage_config);

        let wallet_id = format!("dummy_{}_{}", agent_owner_did, rand::rand_string(10));
        let wallet_config = json!({
                    "id": wallet_id.clone(),
                    "storage_type": wallet_storage_config.xtype,
                    "storage_config": wallet_storage_config.config,
                 }).to_string();

        let agent_owner_did = agent_owner_did.to_string();
        let agent_owner_verkey = agent_owner_verkey.to_string();

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
                let agent = Agent {
                    wallet_handle,
                    agent_verkey: agent_verkey.clone(),
                    agent_did: agent_did.clone(),
                    owner_did: agent_owner_did,
                    owner_verkey: agent_owner_verkey,
                    router: router.clone(),
                    admin: admin.clone(),
                    forward_agent_detail,
                    configs: HashMap::new(),
                };

                let agent = agent.start();

                router.write().unwrap()
                    .add_a2a_route(agent_did.clone(), agent_verkey.clone(), agent.clone().recipient());
                if let Some(admin) = admin {
                    admin.write().unwrap()
                        .register_agent(agent_did.clone(), agent.clone())
                };
                future::ok((wallet_id, agent_did, agent_verkey, kdf_directives))
            })
            .into_box()
    }

    /// Restores previously created agent (create its actor representation)
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
    pub fn restore(wallet_id: &str,
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
                did::get_did_metadata(wallet_handle, &agent_did)
                    .then(|res| match res {
                        Err(IndyError { error_code: ErrorCode::WalletItemNotFound, .. }) => Ok("{}".to_string()),
                        r => r
                    })
                    .map(move |metadata| (wallet_handle, agent_did, agent_verkey, metadata))
                    .map_err(|err| err.context("Can't get Agent DID Metadata.").into())
            })
            .and_then(move |(wallet_handle, agent_did, agent_verkey, metadata)| {
                // Resolve information about existing connections from the wallet
                // and start Agent Connection actor for each exists connection

                debug!("Agent restore. Agent configs to be loaded: {:?}", metadata);
                let configs: HashMap<String, String> = serde_json::from_str(&metadata).expect("Can't restore Agent config.");

                Agent::_restore_connections(wallet_handle,
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
                    forward_agent_detail,
                    configs,
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
    fn _restore_connections(wallet_handle: WalletHandle,
                            owner_did: &str,
                            owner_verkey: &str,
                            forward_agent_detail: &ForwardAgentDetail,
                            router: Rc<RwLock<Router>>,
                            admin: Option<Arc<RwLock<Admin>>>,
                            agent_configs: HashMap<String, String>) -> ResponseFuture<(), Error> {
        trace!("Agent::_restore_connections >> {:?}, {:?}, {:?}, {:?}",
               wallet_handle, owner_did, owner_verkey, forward_agent_detail);

        let owner_did = owner_did.to_string();
        let owner_verkey = owner_verkey.to_string();
        let forward_agent_detail = forward_agent_detail.clone();

        future::ok(())
            .and_then(move |_| Self::get_pairwise_list(wallet_handle).into_box())
            .and_then(move |pairwise_list| {
                let futures: Vec<_> = pairwise_list
                    .iter()
                    .map(move |pairwise| {
                        AgentConnection::restore(wallet_handle,
                                                 &owner_did,
                                                 &owner_verkey,
                                                 &pairwise.my_did,
                                                 &pairwise.their_did,
                                                 &pairwise.metadata,
                                                 &forward_agent_detail,
                                                 router.clone(),
                                                 admin.clone(),
                                                 agent_configs.clone())
                    })
                    .collect();

                future::join_all(futures)
                    .map(|_| ())
                    .map_err(|err| err.context("Can't restore Agent connections.").into())
            })
            .into_box()
    }

    /// Handles message which has been addressed for this agent. There's generally 2 classes of
    /// messages this agent can handle:
    /// 1. Forward messages - is data structure with encrypted data and recipient address of some sort.
    /// When agent is receives such message, all it can do is pass it to Router actor to take care
    /// delivery of such message.
    /// See details about forwarding in Aries at
    /// https://github.com/hyperledger/aries-rfcs/blob/master/concepts/0094-cross-domain-messaging/README.md
    ///
    /// 2. VCX Agent messages - messages which agent's owner sends to the agent to make it do something.
    /// For example message types such us GetMessagesByConnections, UpdateMessageStatusByConnections
    /// can download messages which the agent has received on its owner behalf.
    /// Another example are message types such as  UpdateConfigs, GetConfigs, RemoveConfigs
    /// for CRUD operations for agent's configuration.
    fn handle_a2a_msg(&mut self,
                      msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.agent_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(|(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::Version1(A2AMessageV1::Forward(msg))) => {
                        slf.router.read().unwrap()
                            .route_a2a_msg(msg.fwd, msg.msg)
                            .from_err()
                            .map(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::Forward(msg))) => {
                        let msg_ = ftry_act!(slf, serde_json::to_vec(&msg.msg));
                        slf.router.read().unwrap()
                            .route_a2a_msg(msg.fwd, msg_)
                            .from_err()
                            .map(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(A2AMessage::Version2(A2AMessageV2::ForwardV3(msg))) => {
                        let msg_ = ftry_act!(slf, serde_json::to_vec(&msg.msg));
                        slf.router.read().unwrap()
                            .route_a2a_msg(msg.to, msg_)
                            .from_err()
                            .map(|res| res)
                            .into_actor(slf)
                            .into_box()
                    }
                    Some(msg) => slf.handle_agent_msg(sender_vk, msg),
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .into_box()
    }

    /// Handles VCX agent messages the agent's owner might send to control this agent.
    fn handle_agent_msg(&mut self,
                        sender_vk: String,
                        msg: A2AMessage) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("Agent::handle_agent_msg >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessage::Version1(msg) => self.handle_agent_msg_v1(sender_vk, msg),
            A2AMessage::Version2(msg) => self.handle_agent_msg_v2(sender_vk, msg)
        }
    }

    fn handle_agent_msg_v1(&mut self,
                           sender_vk: String,
                           msg: A2AMessageV1) -> ResponseActFuture<Self, Vec<u8>, Error> {
        debug!("Agent::handle_agent_msg_v1 >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessageV1::CreateKey(msg) => self.handle_create_key_v1(msg),
            A2AMessageV1::GetMessagesByConnections(msg) => self.handle_get_messages_by_connections_v1(msg),
            A2AMessageV1::UpdateMessageStatusByConnections(msg) => self.handle_update_messages_by_connections_v1(msg),
            A2AMessageV1::UpdateConfigs(msg) => self.handle_update_configs_v1(msg),
            A2AMessageV1::GetConfigs(msg) => self.handle_get_configs_v1(msg),
            A2AMessageV1::RemoveConfigs(msg) => self.handle_remove_configs_v1(msg),
            A2AMessageV1::UpdateComMethod(msg) => self.handle_update_com_method_v1(msg),
            _ => err_act!(self, err_msg("Unsupported message"))
        }
            .and_then(move |msgs, slf, _|
                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.agent_verkey, &sender_vk, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
                    .into_actor(slf)
                    .into_box()
            )
            .into_box()
    }

    fn handle_agent_msg_v2(&mut self,
                           sender_vk: String,
                           msg: A2AMessageV2) -> ResponseActFuture<Self, Vec<u8>, Error> {
        debug!("Agent::handle_agent_msg_v2 >> {:?}, {:?}", sender_vk, msg);

        match msg {
            A2AMessageV2::CreateKey(msg) => self.handle_create_key_v2(msg),
            A2AMessageV2::GetMessagesByConnections(msg) => self.handle_get_messages_by_connections_v2(msg),
            A2AMessageV2::UpdateMessageStatusByConnections(msg) => self.handle_update_messages_by_connections_v2(msg),
            A2AMessageV2::UpdateConfigs(msg) => self.handle_update_configs_v2(msg),
            A2AMessageV2::GetConfigs(msg) => self.handle_get_configs_v2(msg),
            A2AMessageV2::RemoveConfigs(msg) => self.handle_remove_configs_v2(msg),
            A2AMessageV2::UpdateComMethod(msg) => self.handle_update_com_method_v2(msg),
            _ => err_act!(self, err_msg("Unsupported message"))
        }
            .and_then(move |msg, slf, _|
                A2AMessage::pack_v2(slf.wallet_handle, Some(&slf.agent_verkey), &sender_vk, &msg)
                    .map_err(|err| err.context("Can't pack message.").into())
                    .into_actor(slf)
                    .into_box()
            )
            .into_box()
    }

    fn handle_get_messages_by_connections_v1(&mut self,
                                             msg: GetMessagesByConnections) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("Agent::handle_get_messages_by_connections_v1 >> {:?}", msg);

        self.handle_get_messages_by_connections(msg)
            .map(|msgs: Vec<A2ConnMessage>| {
                vec![A2AMessage::Version1(A2AMessageV1::MessagesByConnections(
                    MessagesByConnections {
                        msgs: msgs.into_iter().map(|msg| msg.into()).collect()
                    }))]
            })
            .into_actor(self)
            .into_box()
    }

    fn handle_get_messages_by_connections_v2(&mut self,
                                             msg: GetMessagesByConnections) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        trace!("Agent::handle_get_messages_by_connections_v2 >> {:?}", msg);

        self.handle_get_messages_by_connections(msg)
            .map(|msgs: Vec<A2ConnMessage>| {
                A2AMessageV2::MessagesByConnections(
                    MessagesByConnections {
                        msgs: msgs.into_iter().map(|msg| msg.into()).collect()
                    })
            })
            .into_actor(self)
            .into_box()
    }

    /// Retrieves messages according to requested filters. Messages are however not managed by
    /// the agent actor directly - agent actor manages "agent connection" actors. Each
    /// "agent connection" actor represents one pairwise relationship of this agent's owner.
    /// Hence the agent requests every "agent connection" actor to return messages according to
    /// provided filters in "msg" argument.
    ///
    /// # Arguments
    /// * `msg` - represents filters to determine which messages shall be retrieved.
    ///
    fn handle_get_messages_by_connections(&mut self,
                                          msg: GetMessagesByConnections) -> ResponseFuture<Vec<A2ConnMessage>, Error> {
        trace!("Agent::handle_get_messages_by_connections >> {:?}", msg);

        let GetMessagesByConnections { exclude_payload, uids, status_codes, pairwise_dids } = msg;

        let router = self.router.clone();
        let wallet_handle = self.wallet_handle.clone();

        let msg = GetMessages { exclude_payload, uids, status_codes };

        future::ok(())
            .and_then(move |_| Self::get_pairwise_list(wallet_handle).into_box())
            .and_then(move |pairwise_list| {
                let pairwises: Vec<_> = pairwise_list
                    .into_iter()
                    .filter(|pairwise| pairwise_dids.is_empty() || pairwise_dids.contains(&pairwise.their_did))
                    .collect();

                if !pairwise_dids.is_empty() && pairwises.is_empty() {
                    return err!(err_msg("Pairwise DID not found.")).into_box();
                }

                let futures: Vec<_> = pairwises
                    .iter()
                    .map(move |pairwise| {
                        router.read().unwrap()
                            .route_a2conn_msg(pairwise.my_did.clone(), A2ConnMessage::GetMessages(msg.clone()))
                            .from_err()
                            .into_box()
                    })
                    .collect();

                future::join_all(futures)
                    // .map_err(|err| err.context("Can't get Agent Connection messages").into())
                    .into_box()
            })
            .into_box()
    }

    fn handle_update_messages_by_connections_v1(&mut self,
                                                msg: UpdateMessageStatusByConnections) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("Agent::handle_update_messages_by_connections_v1 >> {:?}", msg);

        self.handle_update_messages_by_connections(msg)
            .map(|uids_by_conn: Vec<A2ConnMessage>| {
                vec![A2AMessage::Version1(A2AMessageV1::MessageStatusUpdatedByConnections(
                    MessageStatusUpdatedByConnections {
                        updated_uids_by_conn: uids_by_conn.into_iter().map(|msg| msg.into()).collect(),
                        failed: Vec::new(),
                    }))]
            })
            .into_actor(self)
            .into_box()
    }

    fn handle_update_messages_by_connections_v2(&mut self,
                                                msg: UpdateMessageStatusByConnections) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        trace!("Agent::handle_update_messages_by_connections_v2 >> {:?}", msg);

        self.handle_update_messages_by_connections(msg)
            .map(|uids_by_conn: Vec<A2ConnMessage>| {
                A2AMessageV2::MessageStatusUpdatedByConnections(
                    MessageStatusUpdatedByConnections {
                        updated_uids_by_conn: uids_by_conn.into_iter().map(|msg| msg.into()).collect(),
                        failed: Vec::new(),
                    })
            })
            .into_actor(self)
            .into_box()
    }

    /// Updates state of messages. The set of messages which shall have update their status to
    /// a new value are determined by filter provided in "msg" argument.
    ///
    /// # Arguments
    /// * `msg` - represents filters to determine statuses of which messages shall be updated
    ///
    fn handle_update_messages_by_connections(&mut self,
                                             msg: UpdateMessageStatusByConnections) -> ResponseFuture<Vec<A2ConnMessage>, Error> {
        trace!("Agent::handle_update_messages_by_connections >> {:?}", msg);

        let UpdateMessageStatusByConnections { uids_by_conn, status_code } = msg;

        let router = self.router.clone();
        let wallet_handle = self.wallet_handle.clone();

        future::ok(())
            .and_then(move |_| Self::get_pairwise_list(wallet_handle).into_box())
            .and_then(move |pairwise_list| {
                let futures: Vec<_> = pairwise_list
                    .iter()
                    .filter_map(|pairwise| uids_by_conn
                        .iter()
                        .find(|uid_by_conn| uid_by_conn.pairwise_did == pairwise.their_did)
                        .map(|uid_by_conn| (uid_by_conn, pairwise))
                    )
                    .map(move |(uid_by_conn, pairwise)|

                        router.read().unwrap()
                            .route_a2conn_msg(pairwise.my_did.clone(),
                                              A2ConnMessage::UpdateMessages(
                                                  UpdateMessageStatus { uids: uid_by_conn.uids.clone(), status_code: status_code.clone() }
                                              ))
                            .from_err()
                            .into_box()
                    )
                    .collect();

                future::join_all(futures)
                // .map_err(|err| err.context("Can't get Agent Connection messages").into())
            })
            .into_box()
    }

    /// Returns list of all pairwise relationships managed by this agent
    fn get_pairwise_list(wallet_handle: WalletHandle) -> ResponseFuture<Vec<Pairwise>, Error> {
        future::ok(())
            .and_then(move |_| {
                pairwise::list_pairwise(wallet_handle)
                    .map_err(|err| err.context("Can't get Agent pairwise list").into())
                    .into_box()
            })
            .and_then(move |pairwise_list| {
                let pairwise_list = ftry!(serde_json::from_str::<Vec<String>>(&pairwise_list));

                pairwise_list
                    .iter()
                    .map(|pairwise| serde_json::from_str::<Pairwise>(&pairwise))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|err| err.context("Can't deserialize Agent pairwise").into())
                    .into_future()
                    .into_box()
            })
            .into_box()
    }

    fn handle_create_key_v1(&mut self,
                            msg: CreateKey) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("Agent::handle_create_key_v1 >> {:?}", msg);

        let CreateKey { for_did, for_did_verkey } = msg;

        self.handle_create_key(&for_did, &for_did_verkey)
            .map(|(pairwise_did, pairwise_did_verkey), _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::KeyCreated(KeyCreated {
                    with_pairwise_did: pairwise_did,
                    with_pairwise_did_verkey: pairwise_did_verkey,
                }))]
            })
            .into_box()
    }

    fn handle_create_key_v2(&mut self,
                            msg: CreateKey) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        trace!("Agent::handle_create_key_v2 >> {:?}", msg);

        let CreateKey { for_did, for_did_verkey, .. } = msg;

        self.handle_create_key(&for_did, &for_did_verkey)
            .map(|(pairwise_did, pairwise_did_verkey), _, _| {
                A2AMessageV2::KeyCreated(KeyCreated {
                    with_pairwise_did: pairwise_did,
                    with_pairwise_did_verkey: pairwise_did_verkey,
                })
            })
            .into_box()
    }

    /// Creates new "Agent Connection" actor managed by this agent. The "Agent Connection"
    /// represents a pairwise relationship between this agent's owner and another party.
    ///
    /// Returns information about created "Agent Connection" as
    /// tuple `(PairwiseAgent.DID@PairwiseAgent:Client, PairwiseAgent.VKey@PairwiseAgent:Client)`
    /// The DID represents an address where the counterparty of client's pairwise relationship
    /// is expected forward messages to. It's also expected that messages arriving to this
    /// Pairwise Agent will be authcrypted using its verkey.

    /// # Arguments
    /// * `for_did` - The DID which Agent's owner generated to identify pairwise relationship with the 3rd party
    /// * `for_did_verkey` - The verkey which Agent's owner generated for E2E encryption within pairwise relationship with the 3rd party
    fn handle_create_key(&mut self,
                         for_did: &str,
                         for_did_verkey: &str) -> ResponseActFuture<Self, (String, String), Error> {
        trace!("Agent::handle_create_key >> {:?}, {:?}", for_did, for_did_verkey);

        let user_pairwise_did = for_did.to_string();
        let user_pairwise_verkey = for_did_verkey.to_string();

        let their_did_info = json!({
            "did": for_did,
            "verkey": for_did_verkey,
        }).to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _|
                slf.check_no_pairwise_exists(&user_pairwise_did)
                    .map(|_| user_pairwise_did)
                    .into_actor(slf)
            )
            .and_then(move |user_pairwise_did, slf, _|
                did::store_their_did(slf.wallet_handle, &their_did_info)
                    .map_err(|err| err.context("Can't store their DID for Forward Agent Connection pairwise.").into())
                    .map(|_| user_pairwise_did)
                    .into_actor(slf)
            )
            .and_then(|user_pairwise_did, slf, _| {
                did::create_and_store_my_did(slf.wallet_handle, "{}")
                    .map_err(|err| err.context("Can't create DID for agent pairwise connection.").into())
                    .map(|(agent_connection_did, agent_connection_verkey)| (user_pairwise_did, agent_connection_did, agent_connection_verkey))
                    .into_actor(slf)
            })
            .and_then(|(user_pairwise_did, agent_connection_did, agent_connection_verkey), slf, _| {
                pairwise::create_pairwise(slf.wallet_handle, &user_pairwise_did, &agent_connection_did, Some("{}"))
                    .map_err(|err| err.context("Can't store agent pairwise connection.").into())
                    .map(|_| (user_pairwise_did, agent_connection_did, agent_connection_verkey))
                    .into_actor(slf)
            })
            .and_then(move |(user_pairwise_did, agent_connection_did, agent_connection_verkey), slf, _| {
                let config = AgentConnectionConfig {
                    wallet_handle: slf.wallet_handle,
                    owner_did: slf.owner_did.to_string(),
                    owner_verkey: slf.owner_verkey.to_string(),
                    user_pairwise_did,
                    user_pairwise_verkey,
                    agent_connection_did: agent_connection_did.clone(),
                    agent_connection_verkey: agent_connection_verkey.clone(),
                    agent_configs: slf.configs.clone(),
                    forward_agent_detail: slf.forward_agent_detail.clone(),
                };

                AgentConnection::create(config, slf.router.clone(), slf.admin.clone())
                    .map(|_| (agent_connection_did, agent_connection_verkey))
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_update_com_method_v1(&mut self, _msg: UpdateComMethod) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("UpdateComMethod: {:?}", _msg);
        let messages = vec![A2AMessage::Version1(A2AMessageV1::ComMethodUpdated(ComMethodUpdated { id: "123".to_string() }))];
        ok_act!(self,  messages)
    }

    fn handle_update_com_method_v2(&mut self, msg: UpdateComMethod) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        trace!("UpdateComMethodV2 (msg={:?})", msg);
        match msg.com_method.e_type {
            ComMethodType::Webhook => {
                self.configs.insert(String::from("notificationWebhookUrl"), String::from(msg.com_method.value));
            }
            _ => warn!("Agent was trying to handle unsupported communication type {:?}", msg.com_method.e_type)
        };
        let message = A2AMessageV2::ComMethodUpdated(ComMethodUpdated { id: msg.com_method.id });
        ok_act!(self, message)
    }

    fn handle_update_configs_v1(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        self.handle_update_configs(msg)
            .map(|_, _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::ConfigsUpdated(ConfigsUpdated {}))]
            })
            .into_box()
    }

    fn handle_update_configs_v2(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        self.handle_update_configs(msg)
            .map(|_, _, _| {
                A2AMessageV2::ConfigsUpdated(ConfigsUpdated {})
            })
            .into_box()
    }

    fn handle_update_configs(&mut self, msg: UpdateConfigs) -> ResponseActFuture<Self, (), Error> {
        for config_option in msg.configs {
            match config_option.name.as_str() {
                "name" | "logoUrl" | "notificationWebhookUrl" => self.configs.insert(config_option.name, config_option.value),
                _ => {
                    warn!("Agent was trying to set up unsupported agent configuration option {}", config_option.name.as_str());
                    continue;
                }
            };
        }

        let config_metadata = ftry_act!(self, serde_json::to_string(&self.configs));

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                did::set_did_metadata(slf.wallet_handle, &slf.agent_did, config_metadata.to_string().as_str())
                    .map_err(|err| err.context("Can't store config data as DID metadata.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_get_configs_v1(&mut self, msg: GetConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        let configs: Vec<ConfigOption> = self.handle_get_configs(msg);

        let messages = vec![A2AMessage::Version1(A2AMessageV1::Configs(Configs { configs }))];
        ok_act!(self,  messages)
    }

    fn handle_get_configs_v2(&mut self, msg: GetConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        let configs: Vec<ConfigOption> = self.handle_get_configs(msg);

        let messages = A2AMessageV2::Configs(Configs { configs });
        ok_act!(self,  messages)
    }

    fn handle_get_configs(&mut self, msg: GetConfigs) -> Vec<ConfigOption> {
        self.configs.iter()
            .filter(|(k, _)| msg.configs.contains(k))
            .map(|(k, v)| ConfigOption { name: k.clone(), value: v.clone() })
            .collect()
    }

    fn handle_remove_configs_v1(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        self.handle_remove_configs(msg)
            .map(|_, _, _| {
                vec![A2AMessage::Version1(A2AMessageV1::ConfigsRemoved(ConfigsRemoved {}))]
            })
            .into_box()
    }

    fn handle_remove_configs_v2(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, A2AMessageV2, Error> {
        self.handle_remove_configs(msg)
            .map(|_, _, _| {
                A2AMessageV2::ConfigsRemoved(ConfigsRemoved {})
            })
            .into_box()
    }

    fn handle_remove_configs(&mut self, msg: RemoveConfigs) -> ResponseActFuture<Self, (), Error> {
        self.configs.retain(|k, _| !msg.configs.contains(k));
        let config_metadata = ftry_act!(self, serde_json::to_string(&self.configs));

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                did::set_did_metadata(slf.wallet_handle, &slf.agent_did, config_metadata.to_string().as_str())
                    .map_err(|err| err.context("Can't store config data as DID metadata.").into())
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
    type Result = Result<ResAdminQuery, Error>;

    fn handle(&mut self, _msg: HandleAdminMessage, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Agent Handler<HandleAdminMessage>::handle >>");
        Ok(ResAdminQuery::Agent(ResQueryAgent {
            owner_did: self.owner_did.clone(),
            owner_verkey: self.owner_verkey.clone(),
            did: self.agent_did.clone(),
            verkey: self.agent_verkey.clone(),
            configs: self.configs.iter().map(|(key, value)| (key.clone(), value.clone())).collect(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::actors::ForwardA2AMsg;
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
