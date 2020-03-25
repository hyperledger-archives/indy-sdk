use std::collections::HashMap;

use actix::prelude::*;
use base64;
use failure::{err_msg, Error, Fail};
use futures::*;
use futures::future::ok;
use rmp_serde;
use serde_json;
use uuid::Uuid;

use crate::actors::{RemoteMsg, requester};
use crate::actors::agent_connection::agent_connection::{AgentConnection, MessageHandlerRole, RemoteConnectionDetail};
use crate::domain::a2a::*;
use crate::domain::a2connection::*;
use crate::domain::internal_message::InternalMessage;
use crate::domain::invite::{AgentDetail, InviteDetail, RedirectDetail, SenderDetail};
use crate::domain::key_deligation_proof::KeyDlgProof;
use crate::domain::payload::{PayloadKinds, PayloadTypes, PayloadV1, PayloadV2, Thread};
use crate::domain::protocol_type::{ProtocolType, ProtocolTypes};
use crate::domain::status::{ConnectionStatus, MessageStatusCode};
use crate::indy::{crypto, did, ErrorCode, IndyError, pairwise};
use crate::utils::futures::*;
use crate::utils::to_i8;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MessageNotification {
    msg_uid: String,
    msg_type: RemoteMessageType,
    their_pw_did: String,
    msg_status_code: MessageStatusCode,
    notification_id: String,
    pw_did: String,
}

impl AgentConnection {
    pub(super) fn handle_create_message(&mut self,
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
                    (RemoteMessageType::ConnReqRedirect, Some(A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequestRedirect(detail))))) => {
                        slf.handle_create_connection_request_redirect(detail,
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

    pub(super) fn handle_send_remote_message(&mut self,
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

    pub(super) fn handle_connection_request_message(&mut self,
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


    pub(super) fn handle_connection_request_answer_message(&mut self,
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

    pub(super) fn handle_create_connection_request(&mut self,
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
                                                                None,
                                                                None);

                (msg, msg_detail)
            })
            .map(move |(msg, msg_detail), slf, _| {
                let messages = slf.build_invite_message(&msg, &msg_detail);
                (msg.uid, messages)
            })
            .into_box()
    }

    pub(super) fn handle_create_connection_request_answer(&mut self,
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
                let conn_req_msg = slf.create_and_store_internal_message(
                    Some(reply_to_msg_id.as_str()),
                    RemoteMessageType::ConnReq,
                    MessageStatusCode::Received,
                    &msg_detail.sender_detail.did,
                    None,
                    None,
                    None,
                    None,
                    None
                );

                let sender_did = slf.user_pairwise_did.clone();
                let answer_msg = slf.create_and_store_internal_message(
                    None,
                    RemoteMessageType::ConnReqAnswer,
                    msg_detail.answer_status_code.clone(),
                    &sender_did,
                    Some(conn_req_msg.uid.as_str()),
                    None,
                    None,
                    msg_detail.thread.clone(),
                    None
                );
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
                let answer_msg = slf.create_and_store_internal_message(
                    msg_uid.as_ref().map(String::as_str),
                    RemoteMessageType::ConnReqAnswer,
                    msg_detail.answer_status_code.clone(),
                    &msg_detail.sender_detail.did,
                    None,
                    None,
                    None,
                    msg_detail.thread.clone(),
                    None,
                );

                slf.state.remote_connection_detail = Some(RemoteConnectionDetail {
                    forward_agent_detail: msg_detail.sender_agency_detail.clone(),
                    agent_detail: AgentDetail {
                        did: msg_detail.sender_detail.did.clone(),
                        verkey: msg_detail.sender_detail.verkey.clone(),
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
}
