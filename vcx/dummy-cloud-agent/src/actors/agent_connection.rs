use actix::prelude::*;
use actors::{AddA2ARoute, AddA2ConnRoute, HandleA2AMsg, HandleA2ConnMsg, RemoteMsg};
use actors::router::Router;
use domain::a2a::*;
use domain::a2connection::*;
use domain::status::{ConnectionStatus, MessageStatusCode};
use domain::invite::{ForwardAgentDetail, InviteDetail, SenderDetail, AgentDetail};
use domain::internal_message::InternalMessage;
use domain::key_deligation_proof::KeyDlgProof;
use domain::payload::{PayloadV1, PayloadV2, PayloadTypes, PayloadKinds, Thread};
use domain::protocol_type::{ProtocolType, ProtocolTypes};
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, crypto, pairwise, ErrorCode, IndyError};
use std::convert::Into;
use std::collections::HashMap;
use utils::futures::*;
use utils::to_i8;

use base64;
use rmp_serde;
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RemoteConnectionDetail {
    // Remote User Forward Agent info
    forward_agent_detail: ForwardAgentDetail,
    // Remote Agent Connection Info
    agent_detail: AgentDetail,
    // Remote Agent Key Delegation Proof
    agent_key_dlg_proof: KeyDlgProof,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AgentConnectionConfig {
    // Agent wallet handle
    pub wallet_handle: i32,
    // Agent Owner DID
    pub owner_did: String,
    // Agent Owner Verkey
    pub owner_verkey: String,
    // User pairwise DID
    pub user_pairwise_did: String,
    // User pairwise DID Verkey
    pub user_pairwise_verkey: String,
    // Agent pairwise DID
    pub agent_pairwise_did: String,
    // Agent pairwise DID Verkey
    pub agent_pairwise_verkey: String,
    // Agent configs
    pub agent_configs: HashMap<String, String>,
    // Forward Agent info
    pub forward_agent_detail: ForwardAgentDetail,
}

#[allow(unused)] //FIXME:
pub struct AgentConnection {
    // Agent wallet handle
    wallet_handle: i32,
    // Agent Owner DID
    owner_did: String,
    // Agent Owner Verkey
    owner_verkey: String,
    // User pairwise DID
    user_pairwise_did: String,
    // User pairwise Verkey
    user_pairwise_verkey: String,
    // User pairwise DID
    agent_pairwise_did: String,
    // User pairwise Verkey
    agent_pairwise_verkey: String,
    // agent config
    agent_configs: HashMap<String, String>,
    // User Forward Agent info
    forward_agent_detail: ForwardAgentDetail,
    // Connection State
    state: AgentConnectionState,
    // Address of router agent
    router: Addr<Router>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AgentConnectionState {
    // Agent Key Delegation Proof
    agent_key_dlg_proof: Option<KeyDlgProof>,
    // Remote Agent Key Delegation Proof
    remote_connection_detail: Option<RemoteConnectionDetail>,
    // Agent Connection Status
    #[serde(default)]
    connection_status: ConnectionStatus,
    // Agent Connection internal messages
    #[serde(default)]
    messages: HashMap<String, InternalMessage>,
}

impl AgentConnection {
    pub fn create(config: AgentConnectionConfig,
                  router: Addr<Router>) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::create >> {:?}", config);

        future::ok(())
            .and_then(move |_| {
                let agent_connection = AgentConnection {
                    wallet_handle: config.wallet_handle,
                    owner_did: config.owner_did,
                    owner_verkey: config.owner_verkey,
                    user_pairwise_did: config.user_pairwise_did,
                    user_pairwise_verkey: config.user_pairwise_verkey,
                    agent_pairwise_did: config.agent_pairwise_did.clone(),
                    agent_pairwise_verkey: config.agent_pairwise_verkey,
                    agent_configs: config.agent_configs,
                    forward_agent_detail: config.forward_agent_detail,
                    state: AgentConnectionState {
                        agent_key_dlg_proof: None,
                        remote_connection_detail: None,
                        connection_status: ConnectionStatus::NotConnected,
                        messages: HashMap::new(),
                    },
                    router: router.clone(),
                };

                let agent_connection = agent_connection.start();

                let add_route_f = router
                    .send(AddA2ARoute(config.agent_pairwise_did.clone(), agent_connection.clone().recipient()))
                    .from_err();

                let add_conn_route_f = router
                    .send(AddA2ConnRoute(config.agent_pairwise_did.clone(), agent_connection.clone().recipient()))
                    .from_err();

                add_route_f
                    .join(add_conn_route_f)
                    .map(|_| ())
                    .map_err(|err: Error| err.context("Can't add route for Agent Connection.").into())
            })
            .into_box()
    }

    pub fn restore(wallet_handle: i32,
                   owner_did: &str,
                   owner_verkey: &str,
                   agent_pairwise_did: &str,
                   user_pairwise_did: &str,
                   state: &str,
                   forward_agent_detail: &ForwardAgentDetail,
                   router: Addr<Router>,
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
                    agent_pairwise_did: agent_pairwise_did.clone(),
                    agent_pairwise_verkey,
                    agent_configs,
                    forward_agent_detail,
                    state: AgentConnectionState {
                        agent_key_dlg_proof: state.agent_key_dlg_proof,
                        remote_connection_detail: state.remote_connection_detail,
                        connection_status: state.connection_status,
                        messages: state.messages,
                    },
                    router: router.clone(),
                };

                let agent_connection = agent_connection.start();

                router
                    .send(AddA2ARoute(agent_pairwise_did.clone(), agent_connection.clone().recipient()))
                    .from_err()
                    .map_err(|err: Error| err.context("Can't add route for Agent Connection.").into())
            })
            .into_box()
    }

    fn handle_a2a_msg(&mut self,
                      msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("AgentConnection::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::parse_authcrypted(slf.wallet_handle, &slf.agent_pairwise_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(|(sender_vk, mut msgs), slf, _| {
                msgs.reverse();
                let msg = msgs.pop();
                slf.check_message_sender(msg.as_ref(), &sender_vk)
                    .map(|_| (sender_vk, msg, msgs))
                    .into_future()
                    .into_actor(slf)
            })
            .and_then(|(sender_vk, msg, msgs), slf, _| {
                match msg {
                    Some(A2AMessage::Version1(msg)) => {
                        match msg {
                            A2AMessageV1::CreateMessage(msg) => slf.handle_create_message(msg, msgs, &sender_vk),
                            A2AMessageV1::SendMessages(msg) => slf.handle_send_messages(msg),
                            A2AMessageV1::GetMessages(msg) => slf.handle_get_messages(msg),
                            A2AMessageV1::UpdateConnectionStatus(msg) => slf.handle_update_connection_status(msg),
                            A2AMessageV1::UpdateMessageStatus(msg) => slf.handle_update_message_status(msg),
                            _ => err_act!(slf, err_msg("Unsupported message"))
                        }
                    }
                    Some(A2AMessage::Version2(msg)) => {
                        match msg {
                            A2AMessageV2::ConnectionRequest(msg) => slf.handle_connection_request_message(msg, &sender_vk),
                            A2AMessageV2::ConnectionRequestAnswer(msg) => slf.handle_connection_request_answer_message(msg, &sender_vk),
                            A2AMessageV2::SendRemoteMessage(msg) => slf.handle_send_remote_message(msg, &sender_vk),
                            A2AMessageV2::SendMessages(msg) => slf.handle_send_messages(msg),
                            A2AMessageV2::GetMessages(msg) => slf.handle_get_messages(msg),
                            A2AMessageV2::UpdateConnectionStatus(msg) => slf.handle_update_connection_status(msg),
                            A2AMessageV2::UpdateMessageStatus(msg) => slf.handle_update_message_status(msg),
                            _ => err_act!(slf, err_msg("Unsupported message"))
                        }
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }.map(|msgs, _, _| (msgs, sender_vk))
            })
            .and_then(|(msgs, sender_vk), slf, _|
                slf.persist_connection_state()
                    .map(|_, _, _| (msgs, sender_vk))
            )
            .and_then(|(msgs, sender_vk), slf, _|
                slf.encrypt_response(&sender_vk, &msgs)
                    .into_actor(slf)
            )
            .into_box()
    }

    fn handle_create_message(&mut self,
                             msg: CreateMessage,
                             mut tail: Vec<A2AMessage>,
                             sender_verkey: &str) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_create_message >> {:?}, {:?}, {:?}",
               msg, tail, sender_verkey);

        let CreateMessage { mtype, send_msg, reply_to_msg_id, uid } = msg;

        let sender_verkey = sender_verkey.to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                match (mtype, tail.pop()) {
                    (RemoteMessageType::ConnReq, Some(A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequest(detail))))) => {
                        slf.handle_create_connection_request(detail, sender_verkey)
                    }
                    (RemoteMessageType::ConnReqAnswer, Some(A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequestAnswer(detail))))) => {
                        slf.handle_create_connection_request_answer(detail,
                                                                    reply_to_msg_id.clone(),
                                                                    uid,
                                                                    sender_verkey)
                    }
                    (mtype @ _, Some(A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::General(detail))))) => {
                        slf.handle_create_general_message(mtype,
                                                          detail,
                                                          reply_to_msg_id.clone(),
                                                          uid,
                                                          sender_verkey)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message."))
                }
                    .map(|(msg_uid, a2a_msgs), _, _| (msg_uid, a2a_msgs, reply_to_msg_id))
            })
            .and_then(move |(msg_uid, mut a2a_msgs, reply_to_msg_id), slf, _| {
                slf.send_message_if_needed(send_msg, &msg_uid, reply_to_msg_id)
                    .map(|mut sent_message| {
                        a2a_msgs.append(&mut sent_message);
                        a2a_msgs
                    })
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_connection_request_message(&mut self,
                                         msg: ConnectionRequest,
                                         sender_verkey: &str) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_connection_request_message >> {:?}, {:?}", msg, sender_verkey);

        let send_msg = msg.send_msg;
        let reply_to_msg_id = msg.reply_to_msg_id.clone();
        let msg_detail = msg.into();
        let sender_verkey = sender_verkey.to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                slf.handle_create_connection_request(msg_detail, sender_verkey)
            })
            .and_then(move |(msg_uid, a2a_msgs), slf, _| {
                slf.send_message_if_needed(send_msg, &msg_uid, reply_to_msg_id)
                    .map(|_| a2a_msgs)
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_create_connection_request(&mut self,
                                        msg_detail: ConnectionRequestMessageDetail,
                                        sender_verkey: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::handle_create_connection_request >> {:?}, {:?}", msg_detail, sender_verkey);

        ftry_act!(self, self.validate_connection_request(&msg_detail, &sender_verkey));

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _|
                slf.verify_agent_key_dlg_proof(&slf.user_pairwise_verkey, &msg_detail.key_dlg_proof)
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .map(|msg_detail, slf, _| {
                slf.state.agent_key_dlg_proof = Some(msg_detail.key_dlg_proof.clone());

                let sender_did = slf.user_pairwise_did.clone();
                let msg = slf.create_and_store_internal_message(None,
                                                                RemoteMessageType::ConnReq,
                                                                MessageStatusCode::Created,
                                                                &sender_did,
                                                                None,
                                                                None,
                                                                Some(map! { "phone_no".to_string() => msg_detail.phone_no.clone() }),
                                                                None);

                (msg, msg_detail)
            })
            .map(move |(msg, msg_detail), slf, _| {
                let messages = slf.build_invite_message(&msg, &msg_detail);
                (msg.uid, messages)
            })
            .into_box()
    }

    fn handle_connection_request_answer_message(&mut self,
                                                msg: ConnectionRequestAnswer,
                                                sender_verkey: &str) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_connection_request_answer_message >> {:?}, {:?}", msg, sender_verkey);

        let send_msg = msg.send_msg;
        let reply_to_msg_id = msg.reply_to_msg_id.clone();
        let msg_uid = msg.id.clone();
        let sender_verkey = sender_verkey.to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                slf.handle_create_connection_request_answer(msg.into(), reply_to_msg_id.clone(), Some(msg_uid), sender_verkey)
                    .map(|(msg_uid, a2a_msgs), _, _| (msg_uid, a2a_msgs, reply_to_msg_id))
            })
            .and_then(move |(msg_uid, a2a_msgs, reply_to_msg_id), slf, _| {
                slf.send_message_if_needed(send_msg, &msg_uid, reply_to_msg_id)
                    .map(|_| a2a_msgs)
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_create_connection_request_answer(&mut self,
                                               msg_detail: ConnectionRequestAnswerMessageDetail,
                                               reply_to_msg_id: Option<String>,
                                               msg_uid: Option<String>,
                                               sender_verkey: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::handle_create_connection_request_answer >> {:?}, {:?}, {:?}, {:?}",
               msg_detail, reply_to_msg_id, msg_uid, sender_verkey);

        let reply_to_msg_id = ftry_act!(self, {
            reply_to_msg_id.clone()
                .ok_or(err_msg("Required field `reply_to_msg_id` is missed."))
        });

        ftry_act!(self, self.validate_connection_request_answer(&msg_detail, &reply_to_msg_id));

        match self.get_message_handler_role(&sender_verkey) {
            MessageHandlerRole::Owner =>
                self.sender_handle_create_connection_request_answer(msg_detail, reply_to_msg_id),
            MessageHandlerRole::Remote =>
                self.receipt_handle_create_connection_request_answer(msg_detail, reply_to_msg_id, msg_uid)
        }
    }

    fn handle_send_remote_message(&mut self,
                                  msg: SendRemoteMessage,
                                  sender_verkey: &str) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_send_remote_message >> {:?}, {:?}", msg, sender_verkey);

        let send_msg = msg.send_msg;
        let mtype = msg.mtype.clone();
        let uid = msg.id.clone();
        let reply_to_msg_id = msg.reply_to_msg_id.clone();
        let sender_verkey = sender_verkey.to_string();

        let msg_ = ftry_act!(self, {serde_json::to_vec(&msg.msg)});

        let msg_detail = GeneralMessageDetail {
            msg: msg_,
            title: msg.title,
            detail: msg.detail,
        };

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                slf.handle_create_general_message(mtype, msg_detail, reply_to_msg_id.clone(), Some(uid), sender_verkey)
                    .map(|(msg_uid, a2a_msgs), _, _| (msg_uid, a2a_msgs, reply_to_msg_id))
            })
            .and_then(move |(msg_uid, a2a_msgs, reply_to_msg_id), slf, _| {
                slf.send_message_if_needed(send_msg, &msg_uid, reply_to_msg_id)
                    .map(|_| a2a_msgs)
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_create_general_message(&mut self,
                                     mtype: RemoteMessageType,
                                     msg_detail: GeneralMessageDetail,
                                     reply_to_msg_id: Option<String>,
                                     uid: Option<String>,
                                     sender_verkey: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::handle_create_general_message >> {:?}, {:?}, {:?}, {:?}, {:?}",
               mtype, msg_detail, reply_to_msg_id, uid, sender_verkey);

        ftry_act!(self, self.validate_general_message(reply_to_msg_id.as_ref().map(String::as_str)));

        let (status_code, sender_did) =
            match self.get_message_handler_role(&sender_verkey) {
                MessageHandlerRole::Owner =>
                    (MessageStatusCode::Created, self.user_pairwise_did.clone()),
                MessageHandlerRole::Remote =>
                    (MessageStatusCode::Received,
                     self.state.remote_connection_detail.as_ref()
                         .map(|detail| detail.agent_detail.did.clone())
                         .unwrap_or(self.user_pairwise_did.clone())) // TODO: FIXME use proper did
            };

        let msg = self.create_and_store_internal_message(uid.as_ref().map(String::as_str),
                                                         mtype,
                                                         status_code,
                                                         &sender_did,
                                                         None,
                                                         Some(msg_detail.msg),
                                                         Some(map! {"detail".to_string() => msg_detail.detail, "title".to_string()=> msg_detail.title}),
                                                         None);

        if let Some(msg_id) = reply_to_msg_id.as_ref() {
            self.answer_message(msg_id, &msg.uid, &MessageStatusCode::Accepted).unwrap();
        }

        let message = match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::MessageCreated(MessageCreated { uid: msg.uid.clone() })),
            ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::SendRemoteMessageResponse(SendRemoteMessageResponse { id: msg.uid.clone(), sent: true })),
        };

        ok_act!(self, (msg.uid, vec![message]))
    }

    fn handle_send_messages(&mut self, msg: SendMessages) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_send_messages >> {:?}",
               msg);

        let SendMessages { uids } = msg;

        let uids: Vec<(String, Option<String>)> = uids.into_iter().map(|uid| (uid, None)).collect();

        self.send_messages(uids)
            .into_actor(self)
            .into_box()
    }

    fn handle_get_messages(&mut self, msg: GetMessages) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_get_messages >> {:?}", msg);

        let messages = self.get_messages(msg);

        let message = match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::Messages(Messages { msgs: messages })),
            ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::Messages(Messages { msgs: messages })),
        };

        ok_act!(self, vec![message])
    }

    fn get_messages(&self, msg: GetMessages) -> Vec<GetMessagesDetailResponse> {
        let GetMessages { exclude_payload, uids, status_codes } = msg;

        self.state.messages
            .values()
            .filter(|msg|
                (uids.is_empty() || uids.contains(&msg.uid)) &&
                    (status_codes.is_empty() || status_codes.contains(&msg.status_code)))
            .map(|message| {
                GetMessagesDetailResponse {
                    uid: message.uid.clone(),
                    status_code: message.status_code.clone(),
                    sender_did: message.sender_did.clone(),
                    type_: message._type.clone(),
                    payload: match exclude_payload.as_ref().map(String::as_str) {
                        Some("Y") => None,
                        _ => message.payload.as_ref().map(|payload| {
                            match ProtocolType::get() {
                                ProtocolTypes::V1 => MessageDetailPayload::V1(to_i8(payload)),
                                ProtocolTypes::V2 => MessageDetailPayload::V2(serde_json::from_slice(&payload).unwrap()), // TODO: FIXME
                            }
                        })
                    },
                    ref_msg_id: message.ref_msg_id.clone(),
                }
            })
            .collect::<Vec<GetMessagesDetailResponse>>()
    }

    fn handle_update_connection_status(&mut self, msg: UpdateConnectionStatus) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_update_connection_status >> {:?}",
               msg);

        let UpdateConnectionStatus { status_code } = msg;

        if status_code != ConnectionStatus::Deleted {
            return err_act!(self, err_msg("Invalid status code received."));
        }

        self.state.connection_status = status_code.clone();

        let message = match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::ConnectionStatusUpdated(ConnectionStatusUpdated { status_code })),
            ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::ConnectionStatusUpdated(ConnectionStatusUpdated { status_code })),
        };

        ok_act!(self, vec![message])
    }

    fn handle_update_message_status(&mut self, msg: UpdateMessageStatus) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_update_message_status >> {:?}",
               msg);

        self.update_messages_status(msg)
            .map(|(uids, status_code)| {
                let msg = match ProtocolType::get() {
                    ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::MessageStatusUpdated(MessageStatusUpdated { uids, status_code })),
                    ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::MessageStatusUpdated(MessageStatusUpdated { uids, status_code })),
                };
                vec![msg]
            })
            .into_future()
            .into_actor(self)
            .into_box()
    }

    fn handle_agent2conn_message(&mut self,
                                 msg: A2ConnMessage) -> ResponseFuture<A2ConnMessage, Error> {
        trace!("AgentConnection::handle_agent_to_connection_message >> {:?}", msg);

        match msg {
            A2ConnMessage::GetMessages(msg) => {
                let msg = A2ConnMessage::MessagesByConnection(
                    MessagesByConnection {
                        did: self.user_pairwise_did.clone(),
                        msgs: self.get_messages(msg)
                    });
                ok!(msg)
            }
            A2ConnMessage::UpdateMessages(msg) => {
                let (uids, _) = ftry!(self.update_messages_status(msg));
                let msg = A2ConnMessage::MessageStatusUpdatedByConnection(
                    UidByConnection {
                        uids,
                        pairwise_did: self.user_pairwise_did.clone()
                    });
                ok!(msg)
            }
            _ => err!(err_msg("Unsupported message."))
        }
    }

    fn update_messages_status(&mut self, msg: UpdateMessageStatus) -> Result<(Vec<String>, MessageStatusCode), Error> {
        trace!("AgentConnection::update_messages_status >> {:?}",
               msg);

        let UpdateMessageStatus { uids, status_code } = msg;

        let messages_can_be_updated = uids
            .iter()
            .all(|uid| self.check_if_message_status_can_be_updated(uid, &status_code).is_ok());

        if !messages_can_be_updated {
            return Err(err_msg("Messages can't be updated."));
        }

        uids
            .into_iter()
            .map(|uid| self.update_message_status(&uid, &status_code)
                .map(|_| uid))
            .collect::<Result<Vec<_>, _>>()
            .map(|uids| (uids, status_code))
    }

    fn sender_handle_create_connection_request_answer(&mut self,
                                                      msg_detail: ConnectionRequestAnswerMessageDetail,
                                                      reply_to_msg_id: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::initiator_handle_create_connection_request_answer >> {:?}, {:?}",
               msg_detail, reply_to_msg_id);

        let key_dlg_proof = ftry_act!(self, {
            msg_detail.key_dlg_proof.clone()
                .ok_or(err_msg("Required field `key_dlg_proof` is missed."))
        });

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _|
                slf.verify_agent_key_dlg_proof(&slf.user_pairwise_verkey, &key_dlg_proof)
                    .map(|_| (msg_detail, reply_to_msg_id, key_dlg_proof))
                    .into_actor(slf)
            )
            .and_then(|(msg_detail, reply_to_msg_id, key_dlg_proof), slf, _|
                slf.verify_agent_key_dlg_proof(&msg_detail.sender_detail.verkey, &msg_detail.sender_detail.agent_key_dlg_proof)
                    .map(|_| (msg_detail, reply_to_msg_id, key_dlg_proof))
                    .into_actor(slf)
            )
            .map(|(msg_detail, reply_to_msg_id, key_dlg_proof), slf, _| {
                let conn_req_msg = slf.create_and_store_internal_message(Some(reply_to_msg_id.as_str()),
                                                                         RemoteMessageType::ConnReq,
                                                                         MessageStatusCode::Received,
                                                                         &msg_detail.sender_detail.did,
                                                                         None,
                                                                         None,
                                                                         None,
                                                                         None);

                let sender_did = slf.user_pairwise_did.clone();
                let answer_msg = slf.create_and_store_internal_message(None,
                                                                       RemoteMessageType::ConnReqAnswer,
                                                                       msg_detail.answer_status_code.clone(),
                                                                       &sender_did,
                                                                       Some(conn_req_msg.uid.as_str()),
                                                                       None,
                                                                       None,
                                                                       msg_detail.thread.clone());
                slf.state.agent_key_dlg_proof = Some(key_dlg_proof);

                slf.state.remote_connection_detail = Some(RemoteConnectionDetail {
                    forward_agent_detail: msg_detail.sender_agency_detail.clone(),
                    agent_detail: AgentDetail {
                        did: msg_detail.sender_detail.did.clone(),
                        verkey: msg_detail.sender_detail.verkey.clone()
                    },
                    agent_key_dlg_proof: msg_detail.sender_detail.agent_key_dlg_proof.clone(),
                });

                (msg_detail, reply_to_msg_id, answer_msg.uid)
            })
            .and_then(|(msg_detail, reply_to_msg_id, answer_msg_uid), slf, _| {
                slf.answer_message(&reply_to_msg_id, &answer_msg_uid, &msg_detail.answer_status_code)
                    .into_future()
                    .map(|_| (msg_detail, answer_msg_uid))
                    .into_actor(slf)
            })
            .and_then(|(msg_detail, uid), slf, _| {
                slf.store_their_did(&msg_detail.sender_detail.did, &msg_detail.sender_detail.verkey)
                    .map(|_| (msg_detail, uid))
                    .into_actor(slf)
            })
            .and_then(|(msg_detail, uid), slf, _| {
                slf.store_their_did(&msg_detail.sender_agency_detail.did, &msg_detail.sender_agency_detail.verkey)
                    .map(|_| uid)
                    .into_actor(slf)
            })
            .map(|uid, _, _| {
                let message = match ProtocolType::get() {
                    ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::MessageCreated(MessageCreated { uid: uid.clone() })),
                    ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::ConnectionRequestAnswerResponse(ConnectionRequestAnswerResponse { id: uid.clone(), sent: true }))
                };
                (uid, vec![message])
            })
            .into_box()
    }

    fn receipt_handle_create_connection_request_answer(&mut self,
                                                       msg_detail: ConnectionRequestAnswerMessageDetail,
                                                       reply_to_msg_id: String,
                                                       msg_uid: Option<String>) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::receipt_handle_create_connection_request_answer >> {:?}, {:?}, {:?}",
               msg_detail, reply_to_msg_id, msg_uid);

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _|
                slf.verify_agent_key_dlg_proof(&msg_detail.sender_detail.verkey, &msg_detail.sender_detail.agent_key_dlg_proof)
                    .map(|_| (msg_detail, reply_to_msg_id))
                    .into_actor(slf)
            )
            .map(move |(msg_detail, reply_to_msg_id), slf, _| {
                let answer_msg = slf.create_and_store_internal_message(msg_uid.as_ref().map(String::as_str),
                                                                       RemoteMessageType::ConnReqAnswer,
                                                                       msg_detail.answer_status_code.clone(),
                                                                       &msg_detail.sender_detail.did,
                                                                       None,
                                                                       None,
                                                                       None,
                                                                       msg_detail.thread.clone());

                slf.state.remote_connection_detail = Some(RemoteConnectionDetail {
                    forward_agent_detail: msg_detail.sender_agency_detail.clone(),
                    agent_detail: AgentDetail {
                        did: msg_detail.sender_detail.did.clone(),
                        verkey: msg_detail.sender_detail.verkey.clone()
                    },
                    agent_key_dlg_proof: msg_detail.sender_detail.agent_key_dlg_proof.clone(),
                });

                (msg_detail, reply_to_msg_id, answer_msg.uid)
            })
            .and_then(|(msg_detail, reply_to_msg_id, answer_msg_uid), slf, _| {
                slf.answer_message(&reply_to_msg_id, &answer_msg_uid, &msg_detail.answer_status_code).into_future()
                    .map(|_| (msg_detail, answer_msg_uid))
                    .into_actor(slf)
            })
            .and_then(|(msg_detail, uid), slf, _| {
                slf.store_their_did(&msg_detail.sender_detail.did, &msg_detail.sender_detail.verkey)
                    .map(|_| (msg_detail, uid))
                    .into_actor(slf)
            })
            .and_then(|(msg_detail, uid), slf, _| {
                slf.store_their_did(&msg_detail.sender_agency_detail.did, &msg_detail.sender_agency_detail.verkey)
                    .map(|_| (msg_detail, uid))
                    .into_actor(slf)
            })
            .and_then(|(msg_detail, uid), slf, _| {
                slf.store_payload_for_connection_request_answer(&uid, msg_detail)
                    .map(|_, _, _| uid)
            })
            .map(|uid, _, _| {
                let message = match ProtocolType::get() {
                    ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::MessageCreated(MessageCreated { uid: uid.clone() })),
                    ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::ConnectionRequestAnswerResponse(ConnectionRequestAnswerResponse { id: uid.clone(), sent: true }))
                };
                (uid, vec![message])
            })
            .into_box()
    }

    fn create_and_store_internal_message(&mut self,
                                         uid: Option<&str>,
                                         mtype: RemoteMessageType,
                                         status_code: MessageStatusCode,
                                         sender_did: &str,
                                         ref_msg_id: Option<&str>,
                                         payload: Option<Vec<u8>>,
                                         sending_data: Option<HashMap<String, Option<String>>>,
                                         thread: Option<Thread>) -> InternalMessage {
        trace!("AgentConnection::create_and_store_internal_message >> {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
               uid, mtype, status_code, sender_did, ref_msg_id, payload, sending_data);

        let msg = InternalMessage::new(uid,
                                       mtype,
                                       status_code,
                                       sender_did,
                                       ref_msg_id,
                                       payload,
                                       sending_data,
                                       thread);
        self.state.messages.insert(msg.uid.to_string(), msg.clone());
        msg
    }

    fn persist_connection_state(&self) -> ResponseActFuture<Self, (), Error> {
        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                let metadata = ftry_act!(slf, {
                    serde_json::to_string(&slf.state)
                        .map_err(|err| err.context("Can't serialize connection state."))
                });

                pairwise::set_pairwise_metadata(slf.wallet_handle, &slf.user_pairwise_did, &metadata)
                    .map_err(|err| err.context("Can't store connection state.").into())
                    .into_actor(slf)
                    .into_box()
            })
            .into_box()
    }


    fn store_payload_for_connection_request_answer(&mut self,
                                                   msg_uid: &str,
                                                   msg_detail: ConnectionRequestAnswerMessageDetail) -> ResponseActFuture<Self, (), Error> {
        trace!("AgentConnection::store_payload_for_connection_request_answer >> {:?}, {:?}",
               msg_uid, msg_detail);

        if !self.state.messages.contains_key(msg_uid) {
            return err_act!(self, err_msg("Message not found."));
        }

        let msg_uid = msg_uid.to_string();

        self.build_payload_message(RemoteMessageType::ConnReqAnswer, &json!({"senderDetail": msg_detail.sender_detail}))
            .into_actor(self)
            .map(move |payload, slf, _|
                slf.state.messages.get_mut(&msg_uid)
                    .map(|message| message.payload = Some(payload))
                    .unwrap()
            )
            .into_box()
    }

    fn is_sent_by_owner(&self, sender_verkey: &str) -> bool {
        sender_verkey == self.user_pairwise_verkey
    }

    fn is_sent_by_remote(&self, sender_verkey: &str) -> bool {
        match self.state.remote_connection_detail {
            Some(ref remote_connection_detail) => sender_verkey == remote_connection_detail.agent_key_dlg_proof.agent_delegated_key,
            None => true
        }
    }

    fn get_message_handler_role(&self, sender_verkey: &str) -> MessageHandlerRole {
        trace!("AgentConnection::get_message_handler_role >> {:?}",
               sender_verkey);

        if self.is_sent_by_owner(sender_verkey) { MessageHandlerRole::Owner } else { MessageHandlerRole::Remote }
    }

    fn validate_connection_request(&self,
                                   msg_detail: &ConnectionRequestMessageDetail,
                                   sender_verkey: &str) -> Result<(), Error> {
        trace!("AgentConnection::validate_connection_request >> {:?}, {:?}",
               msg_detail, sender_verkey);

        if !self.is_sent_by_owner(sender_verkey) {
            return Err(err_msg("Unknown message sender."));
        }

        self.check_no_connection_established()?;
        self.check_no_accepted_invitation_exists()?;

        Ok(())
    }

    fn check_message_sender(&self, msg: Option<&A2AMessage>, sender_verkey: &str) -> Result<(), Error> {
        trace!("AgentConnection::check_message_sender >> {:?}, {:?}",
               msg, sender_verkey);

        match msg {
            Some(A2AMessage::Version1(A2AMessageV1::CreateMessage(_))) |
            Some(A2AMessage::Version2(A2AMessageV2::ConnectionRequest(_))) |
            Some(A2AMessage::Version2(A2AMessageV2::ConnectionRequestAnswer(_))) |
            Some(A2AMessage::Version2(A2AMessageV2::SendRemoteMessage(_))) => {
                if self.is_sent_by_owner(sender_verkey) || self.is_sent_by_remote(sender_verkey) {
                    return Ok(());
                }
            }
            Some(A2AMessage::Version1(A2AMessageV1::SendMessages(_))) |
            Some(A2AMessage::Version2(A2AMessageV2::SendMessages(_))) |
            Some(A2AMessage::Version1(A2AMessageV1::GetMessages(_))) |
            Some(A2AMessage::Version2(A2AMessageV2::GetMessages(_))) |
            Some(A2AMessage::Version1(A2AMessageV1::UpdateConnectionStatus(_))) |
            Some(A2AMessage::Version2(A2AMessageV2::UpdateConnectionStatus(_))) |
            Some(A2AMessage::Version1(A2AMessageV1::UpdateMessageStatus(_))) |
            Some(A2AMessage::Version2(A2AMessageV2::UpdateMessageStatus(_))) => {
                if self.is_sent_by_owner(sender_verkey) {
                    return Ok(());
                }
            }
            _ => return Err(err_msg("Unsupported message."))
        }
        Err(err_msg("Invalid message sender."))
    }

    fn validate_connection_request_answer(&self,
                                          msg_detail: &ConnectionRequestAnswerMessageDetail,
                                          reply_to_msg_id: &str) -> Result<(), Error> {
        trace!("AgentConnection::validate_connection_request_answer >> {:?}, {:?}",
               msg_detail, reply_to_msg_id);

        self.check_no_accepted_invitation_exists()?;
        self.check_valid_status_code(&msg_detail.answer_status_code)?;

        if let Some(msg) = self.state.messages.get(reply_to_msg_id) {
            self.check_if_message_not_already_answered(&msg.status_code)?;
        }
        Ok(())
    }

    fn validate_general_message(&self, reply_to_msg_id: Option<&str>) -> Result<(), Error> {
        trace!("AgentConnection::validate_general_message >> {:?}", reply_to_msg_id);

        if let Some(msg_id) = reply_to_msg_id {
            let message = self.state.messages.get(msg_id)
                .ok_or(err_msg("Message not found."))?;

            self.check_if_message_not_already_answered(&message.status_code)?;
        }
        Ok(())
    }

    fn update_message_status(&mut self, uid: &str, status: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::update_message_status >> {:?}, {:?}", uid, status);

        self.state.messages.get_mut(uid)
            .ok_or(err_msg("Message not found."))
            .map(|message| message.status_code = status.clone())
    }

    fn answer_message(&mut self, uid: &str, ref_msg_id: &str, status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::answer_message >> {:?}, {:?}, {:?}", uid, ref_msg_id, status_code);

        self.state.messages.get_mut(uid)
            .ok_or(err_msg("Message mot found."))
            .map(|message| {
                message.status_code = status_code.clone();
                message.ref_msg_id = Some(ref_msg_id.to_string());
            })
    }

    fn store_their_did(&self, did: &str, verkey: &str) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::store_their_did >> {:?}, {:?}", did, verkey);

        let their_did_info = json!({
            "did": did,
            "verkey": verkey,
        }).to_string();

        did::store_their_did(self.wallet_handle, &their_did_info)
            .then(|res| match res {
                Err(IndyError { error_code: ErrorCode::WalletItemAlreadyExists, .. }) => Ok(()),
                r => r
            })
            .map_err(|err| err.context("Can't create my DID for pairwise.").into())
            .into_box()
    }

    fn check_if_message_not_already_answered(&self, status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::check_if_message_not_already_answered >> {:?}", status_code);

        if MessageStatusCode::valid_status_codes().contains(status_code) {
            return Err(err_msg("Message is already answered."));
        }
        Ok(())
    }

    fn check_valid_status_code(&self, status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::check_valid_status_code >> {:?}", status_code);

        if !MessageStatusCode::valid_status_codes().contains(status_code) {
            return Err(err_msg("Invalid answer status code."));
        }
        Ok(())
    }

    fn check_no_connection_established(&self) -> Result<(), Error> {
        trace!("AgentConnection::check_no_connection_established >>");

        if self.state.remote_connection_detail.is_some() {
            return Err(err_msg("Accepted connection already exists."));
        }
        Ok(())
    }

    fn check_no_accepted_invitation_exists(&self) -> Result<(), Error> {
        trace!("AgentConnection::check_no_accepted_invitation_exists >>");

        let is_exists = self.state.messages.values()
            .any(|msg|
                msg._type == RemoteMessageType::ConnReq && msg.status_code == MessageStatusCode::Accepted
            );
        if is_exists {
            return Err(err_msg("Accepted connection already exists."));
        }
        Ok(())
    }

    fn verify_agent_key_dlg_proof(&self, sender_verkey: &str, key_dlg_proof: &KeyDlgProof) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::verify_agent_key_dlg_proof >> {:?}, {:?}",
               sender_verkey, key_dlg_proof);

        let signature = base64::decode(&key_dlg_proof.signature).unwrap();

        crypto::verify(sender_verkey, &key_dlg_proof.challenge().as_bytes(), &signature)
            .then(|res| match res {
                Ok(true) => ok!(()),
                _ => err!(err_msg("Agent key delegation proof verification failed.")).into()
            })
            .into_box()
    }

    fn check_if_message_status_can_be_updated(&self, uid: &str, status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::check_if_message_status_can_be_updated >> {:?}, {:?}",
               uid, status_code);

        let message = self.state.messages.get(uid)
            .ok_or(err_msg("Message not found."))?;

        self.check_if_message_not_already_answered(&message.status_code)?;

        if !MessageStatusCode::valid_new_message_status_codes_allowed_update_to().contains(status_code) {
            return Err(err_msg("Invalid update status code."));
        }

        if !MessageStatusCode::valid_existing_message_statuses_to_update().contains(&message.status_code) {
            return Err(err_msg("Message is not in a state where it can be updated with the given status code."));
        }
        Ok(())
    }

    fn send_message_if_needed(&mut self,
                              send_msg: bool,
                              uid: &str,
                              reply_to_msg_id: Option<String>) -> ResponseFuture<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::send_message_if_needed >> {:?}, {:?}, {:?}", send_msg, uid, reply_to_msg_id);

        if !send_msg {
            return future::ok(Vec::new()).into_box();
        }
        self.send_messages(vec![(uid.to_string(), reply_to_msg_id)])
    }

    fn send_messages(&mut self, msgs: Vec<(String, Option<String>)>) -> ResponseFuture<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::send_messages >> {:?}", msgs);
        let futures: Vec<_> = msgs
            .into_iter()
            .map(|(msg_uid, reply_to)| {
                let message = self.state.messages.get(&msg_uid).cloned().unwrap();
                match message._type {
                    RemoteMessageType::ConnReq => self.send_invite_message(message),
                    RemoteMessageType::ConnReqAnswer => self.send_invite_answer_message(message, reply_to),
                    _ => self.send_general_message(message, reply_to.as_ref().map(String::as_str)),
                }
                    .map(move |_| msg_uid.to_string())
            })
            .collect();

        future::join_all(futures)
            .map(|uids| {
                let message = match ProtocolType::get() {
                    ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::MessageSent(MessageSent { uids })),
                    ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::MessageSent(MessageSent { uids }))
                };
                vec![message]
            })
            .into_future()
            .into_box()
    }

    fn send_invite_message(&mut self, _message: InternalMessage) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::send_invite_message >> {:?}", _message);
        ok!(()) // TODO: send invite sms?
    }

    fn send_invite_answer_message(&mut self, message: InternalMessage, reply_to: Option<String>) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::send_invite_answer_message >> {:?}, {:?}",
               message, reply_to);

        let reply_to = ftry!(reply_to.ok_or(err_msg("Missed required field `reply_to_msg_id`.")));

        if message.status_code != MessageStatusCode::Accepted {
            return err!(err_msg("Message status isn't accepted."));
        }

        let invite_answer = ftry!(self.build_invite_answer_message(&message, &reply_to));
        let message = ftry!(self.prepare_remote_message(invite_answer));
        let endpoint = ftry!(self.get_remote_endpoint());
        self.send_remote_message(message, endpoint)
    }

    fn send_general_message(&self, message: InternalMessage, reply_to: Option<&str>) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::send_general_message >> {:?}, {:?}",
               message, reply_to);

        if message.sender_did == self.user_pairwise_did {
            // TODO: support message delivery statuses?
            let general_message = ftry!(self.build_general_message(message, reply_to));
            let message = ftry!(self.prepare_remote_message(general_message));
            let endpoint = ftry!(self.get_remote_endpoint());
            self.send_remote_message(message, endpoint)
        } else {
            // TODO: Notify user
            ok!(())
        }
    }

    fn send_remote_message(&self, message: Vec<u8>, endpoint: String) -> ResponseFuture<(), Error> {
        let router = self.router.clone();
        future::ok(())
            .and_then(move |_| {
                router
                    .send(RemoteMsg { endpoint, body: message })
                    .from_err()
                    .and_then(|res| res)
                    .map_err(|err: Error| err.context("Can't send message to Remote Endpoint.").into())
            })
            .into_box()
    }

    fn get_remote_endpoint(&self) -> Result<String, Error> {
        let endpoint = self.state.remote_connection_detail.as_ref().map(|detail| detail.forward_agent_detail.endpoint.to_string())
            .ok_or(err_msg("Missed Remote Connection Details."))?;
        Ok(format!("{}/agency/msg", endpoint))
    }

    fn prepare_remote_message(&self, message: Vec<A2AMessage>) -> Result<Vec<u8>, Error> {
        trace!("AgentConnection::prepare_remote_message >> {:?}", message);

        let remote_connection_detail = self.state.remote_connection_detail.as_ref()
            .ok_or(err_msg("Missed Remote Connection Details."))?;

        let remote_forward_agent_detail = &remote_connection_detail.forward_agent_detail;
        let remote_agent_pairwise_detail = &remote_connection_detail.agent_key_dlg_proof;

        let message = A2AMessage::prepare_authcrypted(self.wallet_handle,
                                                      &self.agent_pairwise_verkey,
                                                      &remote_agent_pairwise_detail.agent_delegated_key,
                                                      &message).wait()?;

        let fwd_message = self.build_forward_message(&remote_agent_pairwise_detail.agent_did, message)?;

        let message = A2AMessage::prepare_anoncrypted(self.wallet_handle, &remote_forward_agent_detail.verkey, fwd_message.as_slice()).wait()?;

        Ok(message)
    }

    fn build_payload_message<T>(&self, type_: RemoteMessageType, msg: &T) -> ResponseFuture<Vec<u8>, Error> where T: ::serde::Serialize + ::std::fmt::Debug {
        trace!("AgentConnection::build_payload_message >> {:?}, {:?}",
               type_, msg);

        match ProtocolType::get() {
            ProtocolTypes::V1 => {
                let msg = ftry!(rmp_serde::to_vec_named(&msg));
                ;

                let payload_msg = PayloadV1 {
                    type_: PayloadTypes::build_v1(PayloadKinds::from(type_), "json"),
                    msg: to_i8(&msg)
                };

                let message = ftry!(rmp_serde::to_vec_named(&payload_msg));

                crypto::auth_crypt(self.wallet_handle, &self.agent_pairwise_verkey, &self.owner_verkey, &message)
                    .map_err(|err| err.context("Can't encode Answer Payload.").into())
                    .into_box()
            }
            ProtocolTypes::V2 => {
                let msg = ftry!(serde_json::to_string(&msg));
                ;

                let payload_msg = PayloadV2 {
                    type_: PayloadTypes::build_v2(PayloadKinds::from(type_)),
                    id: String::new(),
                    msg,
                    thread: Thread::new(),
                };

                let message = ftry!(serde_json::to_string(&payload_msg));
                let receiver_keys = ftry!(serde_json::to_string(&vec![&self.owner_verkey]));

                crypto::pack_message(self.wallet_handle, Some(&self.agent_pairwise_verkey), &receiver_keys, &message.as_bytes())
                    .map_err(|err| err.context("Can't encode Answer Payload.").into())
                    .into_box()
            }
        }
    }

    fn build_forward_message(&self, fwd: &str, message: Vec<u8>) -> Result<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::build_forward_message >> {:?}, {:?}",
               fwd, message);

        let message = match ProtocolType::get() {
            ProtocolTypes::V1 => A2AMessage::Version1(A2AMessageV1::Forward(ForwardV1 { fwd: fwd.to_string(), msg: message })),
            ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::Forward(ForwardV2 {
                fwd: fwd.to_string(),
                msg: serde_json::from_slice(message.as_slice())?
            }))
        };

        Ok(vec![message])
    }

    fn build_invite_message(&self, msg: &InternalMessage, msg_detail: &ConnectionRequestMessageDetail) -> Vec<A2AMessage> {
        trace!("AgentConnection::build_invite_message >> {:?}, {:?}",
               msg, msg_detail);

        let invite_detail = InviteDetail {
            conn_req_id: msg.uid.clone(),
            target_name: Some(String::new()),
            sender_agency_detail: self.forward_agent_detail.clone(),
            sender_detail: SenderDetail {
                did: self.user_pairwise_did.clone(),
                verkey: self.user_pairwise_verkey.clone(),
                agent_key_dlg_proof: msg_detail.key_dlg_proof.clone(),
                name: self.agent_configs.get("name").cloned(),
                logo_url: self.agent_configs.get("logo_url").cloned(),
                public_did: Some(self.owner_did.clone()),
            },
            status_code: msg.status_code.clone(),
            status_msg: msg.status_code.message().to_string(),
            thread_id: msg_detail.thread_id.clone(),
        };

        match ProtocolType::get() {
            ProtocolTypes::V1 => {
                let msg_created = MessageCreated { uid: msg.uid.clone() };
                let msg_detail = ConnectionRequestMessageDetailResp {
                    invite_detail,
                    url_to_invite_detail: "".to_string() // format!("{}/agency/invite/{}?msg_uid{}", AGENCY_DOMAIN_URL_PREFIX, self.agent_pairwise_did, msg_uid)
                };

                vec![A2AMessage::Version1(A2AMessageV1::MessageCreated(msg_created)),
                     A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequestResp(msg_detail)))]
            }
            ProtocolTypes::V2 => {
                let message = ConnectionRequestResponse {
                    id: msg.uid.clone(),
                    invite_detail,
                    url_to_invite_detail: "".to_string(),
                    // format!("{}/agency/invite/{}?msg_uid{}", AGENCY_DOMAIN_URL_PREFIX, self.agent_pairwise_did, msg_uid)
                    sent: true,
                    // TODO: FIXME set positive after sending
                };

                vec![A2AMessage::Version2(A2AMessageV2::ConnectionRequestResponse(message))]
            }
        }
    }

    fn build_invite_answer_message(&self, message: &InternalMessage, reply_to: &str) -> Result<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::build_invite_answer_message >> {:?}, {:?}",
               message, reply_to);

        let agent_key_dlg_proof = self.state.agent_key_dlg_proof.clone()
            .ok_or(err_msg("Missed Key Delegation Proof."))?;

        let sender_detail = SenderDetail {
            did: self.user_pairwise_did.clone(),
            verkey: self.user_pairwise_verkey.clone(),
            agent_key_dlg_proof,
            name: None,
            logo_url: None,
            public_did: None,
        };

        let messages =
            match ProtocolType::get() {
                ProtocolTypes::V1 => {
                    let msg_create = CreateMessage {
                        mtype: message._type.clone(),
                        send_msg: false,
                        uid: Some(message.uid.clone()),
                        reply_to_msg_id: Some(reply_to.to_string()),
                    };

                    let msg_detail = ConnectionRequestAnswerMessageDetail {
                        key_dlg_proof: None,
                        sender_detail,
                        sender_agency_detail: self.forward_agent_detail.clone(),
                        answer_status_code: MessageStatusCode::Accepted,
                        thread: None,
                    };

                    vec![A2AMessage::Version1(A2AMessageV1::CreateMessage(msg_create)),
                         A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequestAnswer(msg_detail)))]
                }
                ProtocolTypes::V2 => {
                    let msg = ConnectionRequestAnswer {
                        send_msg: false,
                        id: message.uid.clone(),
                        reply_to_msg_id: Some(reply_to.to_string()),
                        key_dlg_proof: None,
                        sender_detail,
                        sender_agency_detail: self.forward_agent_detail.clone(),
                        answer_status_code: MessageStatusCode::Accepted,
                        thread: message.thread.clone().unwrap_or(Thread::new()),
                    };
                    vec![A2AMessage::Version2(A2AMessageV2::ConnectionRequestAnswer(msg))]
                }
            };

        Ok(messages)
    }

    fn build_general_message(&self, message: InternalMessage, reply_to: Option<&str>) -> Result<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::build_general_message >> {:?}, {:?}",
               message, reply_to);

        let title = message.sending_data.get("title").cloned().and_then(|val| val);
        let detail = message.sending_data.get("detail").cloned().and_then(|val| val);

        let msg = message.payload
            .ok_or(err_msg("Missed Payload."))?;

        let messages =
            match ProtocolType::get() {
                ProtocolTypes::V1 => {
                    let msg_create = CreateMessage {
                        mtype: message._type,
                        send_msg: false,
                        uid: Some(message.uid),
                        reply_to_msg_id: reply_to.map(String::from),
                    };

                    let msg_detail = GeneralMessageDetail { msg, title, detail };

                    vec![A2AMessage::Version1(A2AMessageV1::CreateMessage(msg_create)),
                         A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::General(msg_detail)))]
                }
                ProtocolTypes::V2 => {
                    let msg = SendRemoteMessage {
                        mtype: message._type,
                        id: message.uid,
                        send_msg: false,
                        reply_to_msg_id: reply_to.map(String::from),
                        msg: serde_json::from_slice(&msg)?,
                        title,
                        detail
                    };

                    vec![A2AMessage::Version2(A2AMessageV2::SendRemoteMessage(msg))]
                }
            };

        Ok(messages)
    }

    fn get_recipient_key(&self, sender_vk: &str) -> Result<String, Error> {
        if self.is_sent_by_owner(sender_vk) {
            Ok(self.owner_verkey.clone())
        } else if self.is_sent_by_remote(sender_vk) {
            self.state.remote_connection_detail.as_ref().map(|detail| detail.agent_key_dlg_proof.agent_delegated_key.to_string())
                .ok_or(err_msg("Remote connection is not set."))
        } else { Err(err_msg("Unknown message sender.")) }
    }

    fn encrypt_response(&self, sender_vk: &str, msgs: &Vec<A2AMessage>) -> ResponseFuture<Vec<u8>, Error> {
        let recipient_vk = ftry!(self.get_recipient_key(&sender_vk));

        A2AMessage::prepare_authcrypted(self.wallet_handle, &self.agent_pairwise_verkey, &recipient_vk, &msgs)
            .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
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

enum MessageHandlerRole {
    Owner,
    Remote
}

#[cfg(test)]
mod tests {
    use actors::ForwardA2AMsg;
    use super::*;
    use utils::tests::*;

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