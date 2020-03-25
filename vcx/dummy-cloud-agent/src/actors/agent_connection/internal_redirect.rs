use actix::prelude::*;
use failure::{err_msg, Error};
use futures::*;

use crate::actors::agent_connection::agent_connection::{AgentConnection, MessageHandlerRole, RemoteConnectionDetail};
use crate::domain::a2a::*;
use crate::domain::internal_message::InternalMessage;
use crate::domain::invite::{AgentDetail, SenderDetail};
use crate::domain::payload::Thread;
use crate::domain::protocol_type::{ProtocolType, ProtocolTypes};
use crate::domain::status::MessageStatusCode;
use crate::utils::futures::*;

/// Implementation of methods related to connection redirect feature
impl AgentConnection {
    pub(super) fn handle_create_connection_request_redirect(&mut self,
                                                            msg_detail: ConnectionRequestRedirectMessageDetail,
                                                            reply_to_msg_id: Option<String>,
                                                            msg_uid: Option<String>,
                                                            sender_verkey: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::handle_create_connection_request_redirect >> {:?}, {:?}, {:?}, {:?}",
               msg_detail, reply_to_msg_id, msg_uid, sender_verkey);

        let reply_to_msg_id = ftry_act!(self, {
            reply_to_msg_id.clone()
                .ok_or(err_msg("Required field `reply_to_msg_id` is missed."))
        });

        ftry_act!(self, self.validate_connection_request_redirect(&msg_detail, &reply_to_msg_id));

        match self.get_message_handler_role(&sender_verkey) {
            MessageHandlerRole::Owner =>
                self.sender_handle_create_connection_request_redirect(msg_detail, reply_to_msg_id),
            MessageHandlerRole::Remote =>
                self.receipt_handle_create_connection_request_redirect(msg_detail, reply_to_msg_id, msg_uid)
        }
    }


    fn build_invite_redirect_message(&self, message: InternalMessage, reply_to: &str) -> Result<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::build_invite_redirect_message >> {:?}, {:?}",
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

        let redirect_detail = message.redirect_detail.ok_or(err_msg("Missed Redirect Detail."))?;

        let messages =
            match ProtocolType::get() {
                ProtocolTypes::V1 => {
                    let msg_create = CreateMessage {
                        mtype: message._type.clone(),
                        send_msg: false,
                        uid: Some(message.uid.clone()),
                        reply_to_msg_id: Some(reply_to.to_string()),
                    };

                    let msg_detail = ConnectionRequestRedirectMessageDetail {
                        key_dlg_proof: None,
                        sender_detail,
                        redirect_detail,
                        sender_agency_detail: self.forward_agent_detail.clone(),
                        answer_status_code: MessageStatusCode::Redirected,
                        thread: None,
                    };

                    vec![A2AMessage::Version1(A2AMessageV1::CreateMessage(msg_create)),
                         A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequestRedirect(msg_detail)))]
                }
                ProtocolTypes::V2 => {
                    let msg = ConnectionRequestRedirect {
                        send_msg: false,
                        id: message.uid.clone(),
                        reply_to_msg_id: Some(reply_to.to_string()),
                        key_dlg_proof: None,
                        sender_detail,
                        redirect_detail,
                        sender_agency_detail: self.forward_agent_detail.clone(),
                        answer_status_code: MessageStatusCode::Redirected,
                        thread: message.thread.clone().unwrap_or(Thread::new()),
                    };
                    vec![A2AMessage::Version2(A2AMessageV2::ConnectionRequestRedirect(msg))]
                }
            };

        Ok(messages)
    }


    pub(super) fn handle_connection_request_redirect_message(&mut self,
                                                             msg: ConnectionRequestRedirect,
                                                             sender_verkey: &str) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_connection_request_redirect_message >> {:?}, {:?}", msg, sender_verkey);

        let send_msg = msg.send_msg;
        let reply_to_msg_id = msg.reply_to_msg_id.clone();
        let msg_uid = msg.id.clone();
        let sender_verkey = sender_verkey.to_string();

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                slf.handle_create_connection_request_redirect(msg.into(), reply_to_msg_id.clone(), Some(msg_uid), sender_verkey)
                    .map(|(msg_uid, a2a_msgs), _, _| (msg_uid, a2a_msgs, reply_to_msg_id))
            })
            .and_then(move |(msg_uid, a2a_msgs, reply_to_msg_id), slf, _| {
                slf.send_message_if_needed(send_msg, &msg_uid, reply_to_msg_id)
                    .map(|_| a2a_msgs)
                    .into_actor(slf)
            })
            .into_box()
    }

    fn sender_handle_create_connection_request_redirect(&mut self,
                                                        msg_detail: ConnectionRequestRedirectMessageDetail,
                                                        reply_to_msg_id: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::initiator_handle_create_connection_request_redirect >> {:?}, {:?}",
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
                    None,
                );

                let sender_did = slf.user_pairwise_did.clone();
                let answer_msg = slf.create_and_store_internal_message(
                    None,
                    RemoteMessageType::ConnReqRedirect,
                    msg_detail.answer_status_code.clone(),
                    &sender_did,
                    Some(conn_req_msg.uid.as_str()),
                    None,
                    None,
                    msg_detail.thread.clone(),
                    Some(msg_detail.redirect_detail.clone()),
                );
                slf.state.agent_key_dlg_proof = Some(key_dlg_proof);

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
                    ProtocolTypes::V2 => A2AMessage::Version2(A2AMessageV2::ConnectionRequestRedirectResponse(ConnectionRequestRedirectResponse { id: uid.clone(), sent: true }))
                };
                (uid, vec![message])
            })
            .into_box()
    }

    fn receipt_handle_create_connection_request_redirect(&mut self,
                                                         msg_detail: ConnectionRequestRedirectMessageDetail,
                                                         reply_to_msg_id: String,
                                                         msg_uid: Option<String>) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::receipt_handle_create_connection_request_redirect >> {:?}, {:?}, {:?}",
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
                    RemoteMessageType::ConnReqRedirect,
                    msg_detail.answer_status_code.clone(),
                    &msg_detail.sender_detail.did,
                    None,
                    None,
                    None,
                    msg_detail.thread.clone(),
                    Some(msg_detail.redirect_detail.clone()),
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
                slf.store_payload_for_connection_request_redirect(&uid, msg_detail)
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


    fn store_payload_for_connection_request_redirect(&mut self,
                                                     msg_uid: &str,
                                                     msg_detail: ConnectionRequestRedirectMessageDetail) -> ResponseActFuture<Self, (), Error> {
        trace!("AgentConnection::store_payload_for_connection_request_redirect >> {:?}, {:?}",
               msg_uid, msg_detail);

        if !self.state.messages.contains_key(msg_uid) {
            return err_act!(self, err_msg("Message not found."));
        }

        let msg_uid = msg_uid.to_string();

        // TODO: Darko: Fix
        self.build_payload_message(RemoteMessageType::ConnReqRedirect, &json!({"redirectDetail": msg_detail.redirect_detail}))
            .into_actor(self)
            .map(move |payload, slf, _|
                slf.state.messages.get_mut(&msg_uid)
                    .map(|message| message.payload = Some(payload))
                    .unwrap()
            )
            .into_box()
    }


    fn validate_connection_request_redirect(&self,
                                            msg_detail: &ConnectionRequestRedirectMessageDetail,
                                            reply_to_msg_id: &str) -> Result<(), Error> {
        trace!("AgentConnection::validate_connection_request_redirect >> {:?}, {:?}",
               msg_detail, reply_to_msg_id);

        self.check_no_accepted_invitation_exists()?;
        self.check_valid_status_code(&msg_detail.answer_status_code)?;

        if let Some(msg) = self.state.messages.get(reply_to_msg_id) {
            self.check_if_message_not_already_answered(&msg.status_code)?;
        }
        Ok(())
    }


    pub(super) fn send_invite_redirect_message(&mut self, message: InternalMessage, reply_to: Option<String>) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::send_invite_redirect_message >> {:?}, {:?}",
               message, reply_to);

        let reply_to = ftry!(reply_to.ok_or(err_msg("Missed required field `reply_to_msg_id`.")));

        if message.status_code != MessageStatusCode::Redirected {
            return err!(err_msg("Message status is not redirected."));
        }

        let invite_redirect = ftry!(self.build_invite_redirect_message(message, &reply_to));
        let message = ftry!(self.prepare_remote_message(invite_redirect));
        let endpoint = ftry!(self.get_remote_endpoint());
        self.send_remote_message(message, endpoint)
    }
}
