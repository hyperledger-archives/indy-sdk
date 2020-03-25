use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{Error, Fail};
use futures::*;
use serde_json;

use crate::actors::{HandleA2AMsg, HandleA2ConnMsg, HandleAdminMessage};
use crate::actors::admin::Admin;
use crate::actors::router::Router;
use crate::domain::a2connection::*;
use crate::domain::admin_message::{ResAdminQuery, ResQueryAgentConn};
use crate::domain::internal_message::InternalMessage;
use crate::domain::invite::{AgentDetail, ForwardAgentDetail};
use crate::domain::key_deligation_proof::KeyDlgProof;
use crate::domain::status::ConnectionStatus;
use crate::indy::{did, WalletHandle};
use crate::utils::futures::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct RemoteConnectionDetail {
    // Remote User Forward Agent info
    pub(super) forward_agent_detail: ForwardAgentDetail,
    // Remote Agent Connection Info
    pub(super) agent_detail: AgentDetail,
    // Remote Agent Key Delegation Proof
    pub(super) agent_key_dlg_proof: KeyDlgProof,
}

/// Configuration object used when creating new agent connection
#[derive(Clone, Debug)]
pub struct AgentConnectionConfig {
    // Agent wallet handle
    pub wallet_handle: WalletHandle,
    // Agent Owner DID (Owner.DID@Client:AgentConnection)
    pub owner_did: String,
    // Agent Owner Verkey (Owner.VK@Client:AgentConnection)
    pub owner_verkey: String,
    // User pairwise DID with a 3rd party (Owner.DID@Client:3rdParty)
    pub user_pairwise_did: String,
    // User pairwise Verkey with a 3rd party (Owner.VK@Client:3rdParty)
    pub user_pairwise_verkey: String,
    // Agent Connection's DID (AgentConnection.DID@AgentConnection:3rdParty)
    pub agent_connection_did: String,
    // Agent Connection's Verkey (AgentConnection.VK@AgentConnection:3rdParty)
    pub agent_connection_verkey: String,
    // Agent configs
    pub agent_configs: HashMap<String, String>,
    // Forward Agent info
    pub forward_agent_detail: ForwardAgentDetail,
}

/// Represents routing agent between its owner and a 3rd party. If Alice is owner of this agent
/// and wants to establish pairwise connection with Bob, Alice will provide information about
/// her routing agent. In Aries, that's in form of including verkey of the routing agent into
/// connection invitation. Any messages Bob would like to send to Alice he should deliver into
/// routing agent Alice has prepared for communication with Bob. Once Bob's message arrives here,
/// Alice, as agent's owner, can fetch her received unread messages. The messages should be
/// encrypted E2E with Bob's and Alice's pairwise verkeys, so after Alice downloads message,
/// she'll still have to decrypt it using Alice.PK@Alice:Bob.
///
/// The routing agent also support notifications. If agent's config "notificationWebhookUrl" is
/// set, whenever a new message arrives, the agent's owner will be notified by sending HTTP(S)
/// message containing metadata about received message. The agent's owner can therefore instead
/// of constant polling for new messages rather download and process message right after its arrival.
#[allow(unused)] //FIXME:
pub struct AgentConnection {
    // Agent wallet handle
    pub(super) wallet_handle: WalletHandle,
    // Agent Owner DID (Owner.DID@Client:AgentConnection)
    pub(super) owner_did: String,
    // Agent Owner Verkey (Owner.VK@Client:AgentConnection)
    pub(super) owner_verkey: String,
    // User pairwise DID with a 3rd party (Owner.DID@Client:3rdParty)
    pub(super) user_pairwise_did: String,
    // User pairwise Verkey with a 3rd party (Owner.VK@Client:3rdParty)
    pub(super) user_pairwise_verkey: String,
    // Agent-Connection's DID (AgentConnection.DID@AgentConnection:3rdParty), addressable via router
    pub(super) agent_connection_did: String,
    // Agent-Connection's Verkey (AgentConnection.VK@AgentConnection:3rdParty), addressable via router
    pub(super) agent_connection_verkey: String,
    // agent config
    pub(super) agent_configs: HashMap<String, String>,
    // User Forward Agent info
    pub(super) forward_agent_detail: ForwardAgentDetail,
    // Connection State
    pub(super) state: AgentConnectionState,
    // Address of router agent
    pub(super) router: Rc<RwLock<Router>>,
    // Address of admin agent
    admin: Option<Arc<RwLock<Admin>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct AgentConnectionState {
    // Agent Key Delegation Proof
    pub(super) agent_key_dlg_proof: Option<KeyDlgProof>,
    // Remote Agent Key Delegation Proof
    pub(super) remote_connection_detail: Option<RemoteConnectionDetail>,
    // Agent Connection Status
    #[serde(default)]
    pub(super) connection_status: ConnectionStatus,
    // Agent Connection internal messages
    #[serde(default)]
    pub(super) messages: HashMap<String, InternalMessage>,
}

impl AgentConnection {
    /// Creates new pairwise agent. This is triggered by Agent's owner sending "CreateKeys" message.
    /// The Agent Connection owner is expected to generate connection invitation accordingly to
    /// details of this Agent Connection - for example, use this Agent Connection's 3rd party verkey
    /// to be part of invitation's routing keys.
    pub fn create(config: AgentConnectionConfig,
                  router: Rc<RwLock<Router>>,
                  admin: Option<Arc<RwLock<Admin>>>) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::create >> {:?}", config);
        future::ok(())
            .and_then(move |_| {
                let agent_connection = AgentConnection {
                    wallet_handle: config.wallet_handle,
                    owner_did: config.owner_did,
                    owner_verkey: config.owner_verkey,
                    user_pairwise_did: config.user_pairwise_did,
                    user_pairwise_verkey: config.user_pairwise_verkey,
                    agent_connection_did: config.agent_connection_did.clone(),
                    agent_connection_verkey: config.agent_connection_verkey.clone(),
                    agent_configs: config.agent_configs,
                    forward_agent_detail: config.forward_agent_detail,
                    state: AgentConnectionState {
                        agent_key_dlg_proof: None,
                        remote_connection_detail: None,
                        connection_status: ConnectionStatus::NotConnected,
                        messages: HashMap::new(),
                    },
                    router: router.clone(),
                    admin: admin.clone(),
                };

                let agent_connection = agent_connection.start();
                {
                    let mut router = router.write().unwrap();
                    router.add_a2a_route(config.agent_connection_did.clone(), config.agent_connection_verkey.clone(), agent_connection.clone().recipient());
                    router.add_a2conn_route(config.agent_connection_did.clone(), config.agent_connection_verkey.clone(), agent_connection.clone().recipient());
                }
                let agent_pairwise_did = config.agent_connection_did.clone();
                if let Some(admin) = admin {
                    admin.write().unwrap()
                        .register_agent_connection(agent_pairwise_did, agent_connection.clone())
                };
                future::ok(())
            })
            .into_box()
    }

    /// Recreates Agent Connection actor from information provided in arguments.
    ///
    /// * `owner_did` - Agent's owner's DID at ClientToAgency relationship (Owner.DID@Client:AgentConnection)
    /// * `owner_verkey` - Agent's owner's verkey at ClientToAgency relationship (Owner.VK@Client:AgentConnection)
    /// * `agent_pairwise_did` - Agent pairwise DID (AgentConnection.DID@AgentConnection:3rdParty)
    /// * `user_pairwise_did` - Agent's owner generated DID to identify the relationship with 3rd party (Owner.DID@Owner:3rdParty)
    pub fn restore(wallet_handle: WalletHandle,
                   owner_did: &str,
                   owner_verkey: &str,
                   agent_pairwise_did: &str,
                   user_pairwise_did: &str,
                   state: &str,
                   forward_agent_detail: &ForwardAgentDetail,
                   router: Rc<RwLock<Router>>,
                   admin: Option<Arc<RwLock<Admin>>>,
                   agent_configs: HashMap<String, String>) -> BoxedFuture<(), Error> {
        trace!("AgentConnection::restore >> {:?}", wallet_handle);

        let owner_did = owner_did.to_string();
        let owner_verkey = owner_verkey.to_string();
        let agent_pairwise_did = agent_pairwise_did.to_string();
        let user_pairwise_did = user_pairwise_did.to_string();
        let state = state.to_string();
        let forward_agent_detail = forward_agent_detail.clone();

        future::ok(())
            .and_then(move |_| {
                let state = serde_json::from_str::<AgentConnectionState>(&state).unwrap();

                let agent_pairwise_did_fut = did::key_for_local_did(wallet_handle, &agent_pairwise_did)
                    .map_err(|err| err.context("Can't get Agent Connection verkey").into());

                let user_pairwise_fut = did::key_for_local_did(wallet_handle, &user_pairwise_did)
                    .map_err(|err| err.context("Can't get Agent Connection User verkey").into());

                agent_pairwise_did_fut
                    .join(user_pairwise_fut)
                    .map(|(agent_pairwise_verkey, user_pairwise_verkey)| (agent_pairwise_did, agent_pairwise_verkey, user_pairwise_did, user_pairwise_verkey, state))
            })
            .and_then(move |(agent_pairwise_did, agent_pairwise_verkey, user_pairwise_did, user_pairwise_verkey, state)| {
                let agent_connection = AgentConnection {
                    wallet_handle,
                    owner_did,
                    owner_verkey,
                    user_pairwise_did,
                    user_pairwise_verkey,
                    agent_connection_did: agent_pairwise_did.clone(),
                    agent_connection_verkey: agent_pairwise_verkey.clone(),
                    agent_configs,
                    forward_agent_detail,
                    state: AgentConnectionState {
                        agent_key_dlg_proof: state.agent_key_dlg_proof,
                        remote_connection_detail: state.remote_connection_detail,
                        connection_status: state.connection_status,
                        messages: state.messages,
                    },
                    router: router.clone(),
                    admin: admin.clone(),
                };

                let agent_connection = agent_connection.start();
                {
                    let mut router = router.write().unwrap();
                    router.add_a2a_route(agent_pairwise_did.clone(), agent_pairwise_verkey.clone(), agent_connection.clone().recipient());
                    router.add_a2conn_route(agent_pairwise_did.clone(), agent_pairwise_verkey.clone(), agent_connection.clone().recipient());
                }
                if let Some(admin) = admin {
                    admin.write().unwrap()
                        .register_agent_connection(agent_pairwise_did, agent_connection.clone())
                };
                future::ok(())
            })
            .into_box()
    }
}

impl Actor for AgentConnection {
    type Context = Context<Self>;
}

impl Handler<HandleA2AMsg> for AgentConnection {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<HandleA2AMsg>::handle >> {:?}", msg);
        self.handle_a2a_msg(msg.0)
    }
}

impl Handler<HandleA2ConnMsg> for AgentConnection {
    type Result = ResponseFuture<A2ConnMessage, Error>;

    fn handle(&mut self, msg: HandleA2ConnMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<HandleA2ConnectionMsg>::handle >> {:?}", msg);
        self.handle_agent2conn_message(msg.0)
    }
}

impl Handler<HandleAdminMessage> for AgentConnection {
    type Result = Result<ResAdminQuery, Error>;

    fn handle(&mut self, _msg: HandleAdminMessage, _cnxt: &mut Self::Context) -> Self::Result {
        trace!("Agent Connection Handler<HandleAdminMessage>::handle >>");
        let res = ResQueryAgentConn {
            owner_did: self.owner_did.clone(),
            owner_verkey: self.owner_verkey.clone(),
            user_pairwise_did: self.user_pairwise_did.clone(),
            user_pairwise_verkey: self.user_pairwise_verkey.clone(),
            agent_pairwise_did: self.agent_connection_did.clone(),
            agent_pairwise_verkey: self.agent_connection_verkey.clone(),

            logo: self.agent_configs.get("logoUrl").map_or_else(|| String::from("unknown"), |v| v.clone()),
            name: self.agent_configs.get("name").map_or_else(|| String::from("unknown"), |v| v.clone()),
            agent_configs: self.agent_configs.iter().map(|(key, value)| (key.clone(), value.clone())).collect(),

            remote_agent_detail_did: self.state.remote_connection_detail
                .as_ref().map_or_else(|| "unknown".into(), |r| r.agent_detail.did.clone()),
            remote_agent_detail_verkey: self.state.remote_connection_detail
                .as_ref().map_or_else(|| "unknown".into(), |r| r.agent_detail.verkey.clone()),
            remote_forward_agent_detail_did: self.state.remote_connection_detail
                .as_ref().map_or_else(|| "unknown".into(), |r| r.forward_agent_detail.did.clone()),
            remote_forward_agent_detail_verkey: self.state.remote_connection_detail
                .as_ref().map_or_else(|| "unknown".into(), |r| r.forward_agent_detail.verkey.clone()),
            remote_forward_agent_detail_endpoint: self.state.remote_connection_detail
                .as_ref().map_or_else(|| "unknown".into(), |r| r.forward_agent_detail.endpoint.clone()),
        };
        Ok(ResAdminQuery::AgentConn(res))
    }
}

pub(super) enum MessageHandlerRole {
    Owner,
    Remote,
}

#[cfg(test)]
mod tests {
    use crate::actors::ForwardA2AMsg;
    use crate::utils::tests::*;
    use crate::utils::tests::compose_create_general_message;

    use super::*;
    use crate::domain::a2a::{GetMessagesDetailResponse, RemoteMessageType, MessageDetailPayload};
    use crate::domain::status::MessageStatusCode;
    use crate::utils::to_i8;

    #[test]
    fn agent_create_connection_request_works() {
        run_agent_test(|(e_wallet_handle, agent_did, agent_verkey, agent_pw_did, agent_pw_vk, forward_agent)| {
            future::ok(())
                .and_then(move |_| {
                    let msg = compose_create_connection_request(e_wallet_handle,
                                                                &agent_did,
                                                                &agent_verkey,
                                                                &agent_pw_did,
                                                                &agent_pw_vk).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_pw_did, agent_pw_vk))
                })
                .map(|(e_wallet_handle, resp, agent_pw_did, agent_pw_vk)| {
                    let (sender_vk, msg_uid, invite_detail) = decompose_connection_request_created(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_pw_vk);
                    assert!(!msg_uid.is_empty());
                    assert_eq!(FORWARD_AGENT_DID, invite_detail.sender_agency_detail.did);
                    assert_eq!(FORWARD_AGENT_DID_VERKEY, invite_detail.sender_agency_detail.verkey);
                    assert_eq!(EDGE_PAIRWISE_DID, invite_detail.sender_detail.did);
                    assert_eq!(EDGE_PAIRWISE_DID_VERKEY, invite_detail.sender_detail.verkey);
                    assert_eq!(agent_pw_did, invite_detail.sender_detail.agent_key_dlg_proof.agent_did);
                    assert_eq!(agent_pw_vk, invite_detail.sender_detail.agent_key_dlg_proof.agent_delegated_key);
                    e_wallet_handle
                })
        });
    }

    #[test]
    fn agent_create_connection_request_answer_works() {
        run_agent_test(|(e_wallet_handle, agent_did, agent_verkey, agent_pw_did, agent_pw_vk, forward_agent)| {
            future::ok(())
                .and_then(move |_| {
                    let reply_to_msg_id = "123456789";
                    let msg = compose_create_connection_request_answer(e_wallet_handle,
                                                                       &agent_did,
                                                                       &agent_verkey,
                                                                       &agent_pw_did,
                                                                       &agent_pw_vk,
                                                                       reply_to_msg_id).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_pw_vk, reply_to_msg_id))
                })
                .map(|(e_wallet_handle, resp, agent_pw_vk, reply_to_msg_id)| {
                    let (sender_vk, msg_uid) = decompose_connection_request_answer_created(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_pw_vk);
                    assert_ne!(reply_to_msg_id, msg_uid);
                    e_wallet_handle
                })
        });
    }

    #[test]
    fn agent_create_general_message_works() {
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
                        .map(move |resp| (e_wallet_handle, resp, agent_pw_vk))
                })
                .map(|(e_wallet_handle, resp, agent_pw_vk)| {
                    let (sender_vk, msg_uid) = decompose_general_message_created(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_pw_vk);
                    assert!(!msg_uid.is_empty());
                    e_wallet_handle
                })
        });
    }

    #[test]
    fn agent_get_messages_works() {
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

                    let msg = compose_get_messages(e_wallet_handle,
                                                   &agent_did,
                                                   &agent_verkey,
                                                   &agent_pw_did,
                                                   &agent_pw_vk).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_pw_vk, msg_uid))
                })
                .map(|(e_wallet_handle, resp, agent_pw_vk, msg_uid)| {
                    let (sender_vk, messages) = decompose_get_messages(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_pw_vk);
                    assert_eq!(1, messages.len());

                    let expected_message = GetMessagesDetailResponse {
                        uid: msg_uid,
                        status_code: MessageStatusCode::Created,
                        sender_did: EDGE_PAIRWISE_DID.to_string(),
                        type_: RemoteMessageType::CredOffer,
                        payload: Some(MessageDetailPayload::V1(to_i8(&PAYLOAD.to_vec()))),
                        ref_msg_id: None,
                    };
                    assert_eq!(expected_message, messages[0]);
                    e_wallet_handle
                })
        });
    }

    #[test]
    #[ignore] // TODO: FIXME prepare proper message
    fn agent_update_message_status_works() {
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

                    let get_messages = compose_get_messages(e_wallet_handle,
                                                            &agent_did,
                                                            &agent_verkey,
                                                            &agent_pw_did,
                                                            &agent_pw_vk).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(get_messages))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_did, agent_verkey, forward_agent, agent_pw_did, agent_pw_vk, msg_uid))
                })
                .and_then(|(e_wallet_handle, get_messages, agent_did, agent_verkey, forward_agent, agent_pw_did, agent_pw_vk, msg_uid)| {
                    let (_, messages) = decompose_get_messages(e_wallet_handle, &get_messages).wait().unwrap();
                    assert_eq!(1, messages.len());
                    assert_eq!(msg_uid, messages[0].uid);
                    assert_eq!(MessageStatusCode::Created, messages[0].status_code);

                    let msg = compose_update_message_status_message(e_wallet_handle,
                                                                    &agent_did,
                                                                    &agent_verkey,
                                                                    &agent_pw_did,
                                                                    &agent_pw_vk,
                                                                    &msg_uid,
                                                                    MessageStatusCode::Accepted).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_pw_vk, msg_uid))
                })
                .map(|(e_wallet_handle, resp, agent_pw_vk, msg_uid)| {
                    let (sender_vk, resp) = decompose_message_status_updated(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_pw_vk);
                    assert_eq!(1, resp.uids.len());
                    assert_eq!(msg_uid, resp.uids[0].clone());
                    assert_eq!(MessageStatusCode::Accepted, resp.status_code);
                    e_wallet_handle
                })
        });
    }

    #[test]
    fn agent_update_connection_status_works() {
        run_agent_test(|(e_wallet_handle, agent_did, agent_verkey, agent_pw_did, agent_pw_vk, forward_agent)| {
            future::ok(())
                .and_then(move |_| {
                    let msg = compose_update_connection_status_message(e_wallet_handle,
                                                                       &agent_did,
                                                                       &agent_verkey,
                                                                       &agent_pw_did,
                                                                       &agent_pw_vk).wait().unwrap();

                    forward_agent
                        .send(ForwardA2AMsg(msg))
                        .from_err()
                        .and_then(|res| res)
                        .map(move |resp| (e_wallet_handle, resp, agent_pw_vk))
                })
                .map(move |(e_wallet_handle, resp, agent_pw_vk)| {
                    let (sender_vk, msg) = decompose_connection_status_updated(e_wallet_handle, &resp).wait().unwrap();
                    assert_eq!(sender_vk, agent_pw_vk);
                    assert_eq!(ConnectionStatus::Deleted, msg.status_code);
                    e_wallet_handle
                })
        });
    }
}
