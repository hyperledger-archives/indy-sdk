use actix::prelude::*;
use actors::{AddA2ARoute, HandleA2AMsg};
use actors::router::Router;
use domain::a2a::*;
use domain::status::{ConnectionStatus, MessageStatusCode};
use domain::invite::{ForwardAgentDetail, InviteDetail, SenderDetail, AgentDetail};
use domain::internal_message::InternalMessage;
use domain::key_deligation_proof::KeyDlgProof;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, crypto};
use std::convert::Into;
use utils::futures::*;
use utils::request;

use base64;
use rmp_serde;

use std::collections::HashMap;


#[derive(Clone, Debug, Deserialize)]
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
    // Agent DID
    pub agent_did: String,
    // Agent Verkey
    pub agent_verkey: String,
    // User pairwise DID
    pub user_pairwise_did: String,
    // User pairwise DID Verkey
    pub user_pairwise_verkey: String,
    // Agent pairwise DID
    pub agent_pairwise_did: String,
    // Agent pairwise DID Verkey
    pub agent_pairwise_verkey: String,
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
    // Agent DID
    agent_did: String,
    // Agent Verkey
    agent_verkey: String,
    // User pairwise DID
    user_pairwise_did: String,
    // User pairwise Verkey
    user_pairwise_verkey: String,
    // User pairwise DID
    agent_pairwise_did: String,
    // User pairwise Verkey
    agent_pairwise_verkey: String,
    // User Forward Agent info
    forward_agent_detail: ForwardAgentDetail,
    // Agent Key Delegation Proof
    agent_key_dlg_proof: Option<KeyDlgProof>,
    // Remote Agent Key Delegation Proof
    remote_connection_detail: Option<RemoteConnectionDetail>,
    // Agent Connection Status
    connection_status: ConnectionStatus,
    // Agent Connection internal messages
    messages: HashMap<String, InternalMessage>
}

impl AgentConnection {
    pub fn create(config: AgentConnectionConfig,
                  router: Addr<Router>) -> ResponseFuture<(), Error> {
        future::ok(())
            .and_then(move |_| {
                let agent_connection = AgentConnection {
                    wallet_handle: config.wallet_handle,
                    owner_did: config.owner_did,
                    owner_verkey: config.owner_verkey,
                    agent_did: config.agent_did,
                    agent_verkey: config.agent_verkey,
                    user_pairwise_did: config.user_pairwise_did,
                    user_pairwise_verkey: config.user_pairwise_verkey,
                    agent_pairwise_did: config.agent_pairwise_did.clone(),
                    agent_pairwise_verkey: config.agent_pairwise_verkey,
                    forward_agent_detail: config.forward_agent_detail,
                    agent_key_dlg_proof: None,
                    remote_connection_detail: None,
                    connection_status: ConnectionStatus::NotConnected,
                    messages: HashMap::new(),
                };

                let agent_connection = agent_connection.start();

                router
                    .send(AddA2ARoute(config.agent_pairwise_did.clone(), agent_connection.clone().recipient()))
                    .from_err()
                    .map_err(|err: Error| err.context("Can't add route for Agent Connection").into())
            })
            .into_box()
    }


    #[allow(unused)] // FIXME: Use!
    pub fn restore(config: &AgentConnectionConfig,
                   router: Addr<Router>) -> BoxedFuture<(), Error> {
        unimplemented!()
    }

    fn store_message(&mut self, msg: &InternalMessage) {
        self.messages.insert(msg.uid.to_string(), msg.clone());
    }

    fn handle_a2a_msg(&mut self,
                      msg: Vec<u8>) -> ResponseActFuture<Self, Vec<u8>, Error> {
        trace!("AgentConnection::handle_a2a_msg >> {:?}", msg);

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                A2AMessage::unbundle_authcrypted(slf.wallet_handle, &slf.agent_pairwise_verkey, &msg)
                    .map_err(|err| err.context("Can't unbundle message.").into())
                    .into_actor(slf)
            })
            .and_then(|(sender_vk, mut msgs), slf, _| {
                match msgs.pop() {
                    Some(A2AMessage::CreateMessage(msg)) => {
                        slf.handle_create_message(msg, msgs, sender_vk)
                    }
                    Some(A2AMessage::SendMessages(msg)) => {
                        slf.handle_send_messages(msg)
                    }
                    Some(A2AMessage::GetMessages(msg)) => {
                        slf.handle_get_messages(msg)
                    }
                    Some(A2AMessage::UpdateConnectionStatus(msg)) => {
                        slf.handle_update_connection_status(msg)
                    }
                    Some(A2AMessage::UpdateMessageStatus(msg)) => {
                        slf.handle_update_message_status(msg)
                    }
                    _ => err_act!(slf, err_msg("Unsupported message"))
                }
            })
            .and_then(|msgs, slf, _| {
                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.agent_pairwise_verkey, &slf.owner_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_create_message(&mut self,
                             msg: CreateMessage,
                             mut tail: Vec<A2AMessage>,
                             sender_verkey: String) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_create_message >> {:?}, {:?}, {:?}",
               msg, tail, sender_verkey);

        let CreateMessage { mtype, send_msg, reply_to_msg_id, uid } = msg;

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                match (mtype, tail.pop()) {
                    (MessageType::ConnReq, Some(A2AMessage::MessageDetail(MessageDetail::ConnectionRequest(detail)))) => {
                        slf.handle_create_connection_request(detail, sender_verkey)
                    }
                    (MessageType::ConnReqAnswer, Some(A2AMessage::MessageDetail(MessageDetail::ConnectionRequestAnswer(detail)))) => {
                        slf.handle_create_connection_request_answer(detail,
                                                                    reply_to_msg_id.clone(),
                                                                    uid,
                                                                    sender_verkey)
                    }
                    (type_ @ _, Some(A2AMessage::MessageDetail(MessageDetail::General(detail)))) =>
                        slf.handle_create_general_message(type_,
                                                          detail,
                                                          reply_to_msg_id.clone(),
                                                          uid,
                                                          sender_verkey),
                    _ => err_act!(slf, err_msg("Unsupported message type."))
                }
                    .map(|(msg_uid, a2a_msgs), _, _| (msg_uid, a2a_msgs, reply_to_msg_id))
            })
            .and_then(move |(msg_uid, mut a2a_msgs, reply_to_msg_id), slf, _| {
                slf.send_message_if_needed(send_msg,
                                           &msg_uid,
                                           reply_to_msg_id.as_ref().map(String::as_str))
                    .map(|mut sent_message| {
                        a2a_msgs.append(&mut sent_message);
                        a2a_msgs
                    })
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_create_connection_request(&mut self,
                                        msg_detail: ConnectionRequestMessageDetail,
                                        sender_verkey: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::handle_create_connection_request >> {:?}, {:?}", msg_detail, sender_verkey);

        if sender_verkey != self.user_pairwise_verkey {
            return err_act!(self, err_msg("Unknown message sender."));
        }

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _|
                slf.validate_connection_request(&msg_detail)
                    .into_future()
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .and_then(|msg_detail, slf, _|
                slf.verify_agent_key_dlg_proof(&slf.agent_pairwise_verkey, &msg_detail.key_dlg_proof)
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .map(|msg_detail, slf, _| {
                slf.agent_key_dlg_proof = Some(msg_detail.key_dlg_proof.clone());

                let sender_did = slf.user_pairwise_did.clone();
                let msg = slf.create_and_store_internal_message(None,
                                                                MessageType::ConnReq,
                                                                MessageStatusCode::Created,
                                                                &sender_did,
                                                                None,
                                                                None,
                                                                Some(map! { "phone_no" => Some(msg_detail.phone_no.clone()) }));

                (msg, msg_detail)
            })
            .map(move |(msg, msg_detail), slf, _| {
                let msg_uid = msg.uid.clone();
                let messages = slf.build_invite_message(msg, msg_detail);
                (msg_uid, messages)
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

        let reply_to_msg_id = match reply_to_msg_id {
            Some(msg_id) => msg_id,
            None => return err_act!(self, err_msg("Required field `reply_to_msg_id` is missed"))
        };

        match self.get_message_handler_role(&sender_verkey) {
            MessageHandlerRole::Sender =>
                self.sender_handle_create_connection_request_answer(msg_detail, reply_to_msg_id),
            MessageHandlerRole::Recipient =>
                self.receipt_handle_create_connection_request_answer(msg_detail, reply_to_msg_id, msg_uid)
        }
    }

    fn handle_create_general_message(&mut self,
                                     mtype: MessageType,
                                     msg_detail: GeneralMessageDetail,
                                     reply_to_msg_id: Option<String>,
                                     uid: Option<String>,
                                     sender_verkey: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::handle_create_general_message >> {:?}, {:?}, {:?}, {:?}, {:?}",
               mtype, msg_detail, reply_to_msg_id, uid, sender_verkey);

        if let Err(err) = self.validate_general_message(reply_to_msg_id.as_ref().map(String::as_str)) {
            return err_act!(self, err);
        }

        let (status_code, sender_did) =
            match self.get_message_handler_role(&sender_verkey) {
                MessageHandlerRole::Sender =>
                    (MessageStatusCode::Created, self.user_pairwise_did.clone()),
                MessageHandlerRole::Recipient =>
                    (MessageStatusCode::Received,
                     self.remote_connection_detail.as_ref()
                         .map(|detail| detail.agent_detail.did.clone())
                         .unwrap_or(self.user_pairwise_did.clone())) // TODO: FIXME
            };

        let msg = self.create_and_store_internal_message(uid.as_ref().map(String::as_str),
                                                         mtype,
                                                         status_code,
                                                         &sender_did,
                                                         None,
                                                         Some(msg_detail.msg),
                                                         Some(map! {"detail" => msg_detail.detail, "title"=> msg_detail.title}));


        if let Some(msg_id) = reply_to_msg_id.as_ref() {
            self.answer_message(msg_id, &msg.uid, &MessageStatusCode::Accepted).unwrap();
        }

        let messages = vec![A2AMessage::MessageCreated(MessageCreated { uid: msg.uid.clone() })];

        future::ok((msg.uid, messages))
            .into_actor(self)
            .into_box()
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
        trace!("AgentConnection::handle_get_messages >> {:?}",
               msg);

        let GetMessages { exclude_payload, uids, status_codes } = msg;

        let msgs =
            self.messages
                .values()
                .filter(|msg|
                    (uids.is_empty() || uids.contains(&msg.uid)) &&
                        (status_codes.is_empty() || status_codes.contains(&msg.status_code)))
                .map(|message| {
                    GetMessagesDetailResponse {
                        uid: message.uid.clone(),
                        status_codes: message.status_code.clone(),
                        sender_did: message.sender_did.clone(),
                        type_: message._type.clone(),
                        payload: if !exclude_payload { message.payload.clone() } else { None },
                        ref_msg_id: message.ref_msg_id.clone(),
                    }
                })
                .collect::<Vec<GetMessagesDetailResponse>>();

        future::ok(vec![A2AMessage::Messages(Messages { msgs })])
            .into_actor(self)
            .into_box()
    }

    fn handle_update_connection_status(&mut self, msg: UpdateConnectionStatus) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_update_connection_status >> {:?}",
               msg);

        let UpdateConnectionStatus { status_code } = msg;

        if status_code != ConnectionStatus::Deleted {
            return err_act!(self, err_msg("Invalid status code received."));
        }

        self.connection_status = status_code.clone();

        future::ok(vec![A2AMessage::ConnectionStatusUpdated(ConnectionStatusUpdated { status_code })])
            .into_actor(self)
            .into_box()
    }

    fn handle_update_message_status(&mut self, msg: UpdateMessageStatus) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_update_message_status >> {:?}",
               msg);

        let UpdateMessageStatus { uids, status_code } = msg;

        let messages_can_be_updated = uids
            .iter()
            .all(|uid| self.check_if_message_status_can_be_updated(uid, &status_code).is_ok());

        if messages_can_be_updated {
            return err_act!(self, err_msg("Message can't be updated."));
        }

        uids.iter()
            .map(|uid| self.update_message_status(uid, &status_code))
            .collect::<Result<Vec<_>, _>>()
            .map(|_| vec![A2AMessage::MessageStatusUpdated(MessageStatusUpdated { uids, status_code })])
            .into_future()
            .into_actor(self)
            .into_box()
    }

    fn sender_handle_create_connection_request_answer(&mut self,
                                                      msg_detail: ConnectionRequestAnswerMessageDetail,
                                                      reply_to_msg_id: String) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::initiator_handle_create_connection_request_answer >> {:?}, {:?}", msg_detail, reply_to_msg_id);

        let key_dlg_proof = match msg_detail.key_dlg_proof.clone() {
            Some(key_dlg_proof) => key_dlg_proof,
            None => return err_act!(self, err_msg("Required field `key_dlg_proof` is missed"))
        };

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _|
                slf.validate_connection_request_answer(&msg_detail, &reply_to_msg_id)
                    .into_future()
                    .map(|_| (msg_detail, reply_to_msg_id))
                    .into_actor(slf)
            )
            .and_then(|(msg_detail, reply_to_msg_id), slf, _|
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
                                                                         MessageType::ConnReq,
                                                                         MessageStatusCode::Received,
                                                                         &msg_detail.sender_detail.did,
                                                                         None,
                                                                         None,
                                                                         None);

                let sender_did = slf.user_pairwise_did.clone();
                let answer_msg = slf.create_and_store_internal_message(None,
                                                                       MessageType::ConnReqAnswer,
                                                                       msg_detail.answer_status_code.clone(),
                                                                       &sender_did,
                                                                       Some(conn_req_msg.uid.as_str()),
                                                                       None,
                                                                       None);
                slf.agent_key_dlg_proof = Some(key_dlg_proof);

                slf.remote_connection_detail = Some(RemoteConnectionDetail {
                    forward_agent_detail: msg_detail.sender_agency_detail.clone(),
                    agent_detail: AgentDetail {
                        did: msg_detail.sender_detail.did.clone(),
                        verkey: msg_detail.sender_detail.verkey.clone()
                    },
                    agent_key_dlg_proof: msg_detail.sender_detail.agent_key_dlg_proof.clone(),
                });

                (msg_detail, reply_to_msg_id, answer_msg.uid)
            })
            .and_then(|(msg_detail, reply_to_msg_id, uid), slf, _| {
                slf.answer_message(&reply_to_msg_id, &uid, &msg_detail.answer_status_code)
                    .into_future()
                    .map(|_| (msg_detail, uid))
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
                let messages = vec![A2AMessage::MessageCreated(MessageCreated { uid: uid.clone() })];
                (uid, messages)
            })
            .into_box()
    }

    fn receipt_handle_create_connection_request_answer(&mut self,
                                                       msg_detail: ConnectionRequestAnswerMessageDetail,
                                                       reply_to_msg_id: String,
                                                       msg_uid: Option<String>) -> ResponseActFuture<Self, (String, Vec<A2AMessage>), Error> {
        trace!("AgentConnection::receipt_handle_create_connection_request_answer >> {:?}, {:?}, {:?}", msg_detail, reply_to_msg_id, msg_uid);

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _|
                slf.validate_connection_request_answer(&msg_detail, &reply_to_msg_id)
                    .into_future()
                    .map(|_| (msg_detail, reply_to_msg_id))
                    .into_actor(slf)
            )
            .and_then(|(msg_detail, reply_to_msg_id), slf, _|
                slf.verify_agent_key_dlg_proof(&msg_detail.sender_detail.verkey, &msg_detail.sender_detail.agent_key_dlg_proof)
                    .map(|_| (msg_detail, reply_to_msg_id))
                    .into_actor(slf)
            )
            .map(move |(msg_detail, reply_to_msg_id), slf, _| {
                let sender_did = msg_detail.sender_detail.did.clone();
                let answer_msg = slf.create_and_store_internal_message(msg_uid.as_ref().map(String::as_str),
                                                                       MessageType::ConnReqAnswer,
                                                                       msg_detail.answer_status_code.clone(),
                                                                       &sender_did,
                                                                       None,
                                                                       None,
                                                                       None);

                slf.remote_connection_detail = Some(RemoteConnectionDetail {
                    forward_agent_detail: msg_detail.sender_agency_detail.clone(),
                    agent_detail: AgentDetail {
                        did: msg_detail.sender_detail.did.clone(),
                        verkey: msg_detail.sender_detail.verkey.clone()
                    },
                    agent_key_dlg_proof: msg_detail.sender_detail.agent_key_dlg_proof.clone(),
                });

                (msg_detail, reply_to_msg_id, answer_msg.uid)
            })
            .and_then(|(msg_detail, reply_to_msg_id, uid), slf, _| {
                slf.answer_message(&reply_to_msg_id, &uid, &msg_detail.answer_status_code).into_future()
                    .map(|_| (msg_detail, uid))
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
                let messages = vec![A2AMessage::MessageCreated(MessageCreated { uid: uid.clone() })];
                (uid, messages)
            })
            .into_box()
    }

    fn create_and_store_internal_message(&mut self,
                                         uid: Option<&str>,
                                         mtype: MessageType,
                                         status_code: MessageStatusCode,
                                         sender_did: &str,
                                         ref_msg_id: Option<&str>,
                                         payload: Option<Vec<u8>>,
                                         sending_data: Option<HashMap<&'static str, Option<String>>>) -> InternalMessage {
        trace!("AgentConnection::create_and_store_internal_message >> {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
               uid, mtype, status_code, sender_did, ref_msg_id, payload, sending_data);

        let msg = InternalMessage::new(uid,
                                       mtype,
                                       status_code,
                                       sender_did,
                                       ref_msg_id,
                                       payload,
                                       sending_data);
        self.store_message(&msg);
        msg
    }

    fn store_payload_for_connection_request_answer(&mut self,
                                                   msg_uid: &str,
                                                   msg_detail: ConnectionRequestAnswerMessageDetail) -> ResponseActFuture<Self, (), Error> {
        trace!("AgentConnection::store_payload_for_connection_request_answer >> {:?}, {:?}",
               msg_uid, msg_detail);

        if !self.messages.contains_key(msg_uid) {
            return err_act!(self, err_msg("Message not found."));
        }

        let msg_uid = msg_uid.to_string();

        future::ok(())
            .and_then(move |_|
                rmp_serde::to_vec(&msg_detail.sender_detail)
                    .map_err(|err| err.into())
                    .into_future()
            )
            .and_then(move |msg| {
                let payload_msg = PayloadMessage { type_: "connReqAnswer".to_string(), msg };
                rmp_serde::to_vec(&payload_msg)
                    .map_err(|err| err.into())
                    .into_future()
            })
            .into_actor(self)
            .and_then(move |msg, slf, _|
                crypto::auth_crypt(slf.wallet_handle, &slf.user_pairwise_verkey, &slf.agent_verkey, &msg)
                    .map_err(|err| err.context("Can't create my DID for pairwise.").into())
                    .into_actor(slf)
            )
            .map(move |payload, slf, _|
                slf.messages.get_mut(&msg_uid)
                    .map(|message| message.payload = Some(payload))
                    .unwrap()
            )
            .into_box()
    }

    fn get_message_handler_role(&self, sender_verkey: &str) -> MessageHandlerRole {
        if sender_verkey == self.user_pairwise_verkey { MessageHandlerRole::Sender } else { MessageHandlerRole::Recipient }
    }

    fn validate_connection_request(&self, msg_detail: &ConnectionRequestMessageDetail) -> Result<(), Error> {
        trace!("AgentConnection::validate_connection_request >> {:?}", msg_detail);

        self.check_no_connection_established()?;
        self.check_no_accepted_invitation_exists()?;
        self.check_valid_phone_no(&msg_detail.phone_no)?;
        Ok(())
    }

    fn validate_connection_request_answer(&self,
                                          msg_detail: &ConnectionRequestAnswerMessageDetail,
                                          reply_to_msg_id: &str) -> Result<(), Error> {
        trace!("AgentConnection::validate_connection_request_answer >> {:?}", msg_detail);

        self.check_no_accepted_invitation_exists()?;
        self.check_valid_status_code(&msg_detail.answer_status_code)?;

        if let Some(msg) = self.messages.get(reply_to_msg_id) {
            self.check_if_message_not_already_answered(&msg.status_code)?;
        }
        // TODO: check same endpoints?

        Ok(())
    }

    fn validate_general_message(&self, reply_to_msg_id: Option<&str>) -> Result<(), Error> {
        if let Some(msg_id) = reply_to_msg_id {
            let message = self.messages.get(msg_id)
                .ok_or(err_msg("Message not found."))?;

            self.check_if_message_not_already_answered(&message.status_code)?;
        }
        Ok(())
    }

    fn update_message_status(&mut self, uid: &str, status: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::update_message_status >> {:?}, {:?}", uid, status);

        self.messages.get_mut(uid)
            .map(|message| message.status_code = status.clone())
            .ok_or(err_msg("Message not found."))
    }

    fn answer_message(&mut self, uid: &str, ref_msg_id: &str, status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::answer_message >> {:?}, {:?}, {:?}", uid, ref_msg_id, status_code);

        self.messages.get_mut(uid)
            .map(|message| {
                message.status_code = status_code.clone();
                message.ref_msg_id = Some(ref_msg_id.to_string());
            })
            .ok_or(err_msg("Message mot found"))?;

        Ok(())
    }

    fn store_their_did(&self, did: &str, verkey: &str) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::store_their_did >> {:?}, {:?}", did, verkey);

        let their_did_info = json!({
            "did": did,
            "verkey": verkey,
        }).to_string();

        did::store_their_did(self.wallet_handle, &their_did_info)
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
        if self.remote_connection_detail.is_some() {
            return Err(err_msg("Accepted connection already exists.")); //
        }
        Ok(())
    }

    fn check_no_accepted_invitation_exists(&self) -> Result<(), Error> {
        trace!("AgentConnection::check_no_accepted_invitation_exists >>");

        let is_exists = self.messages.values()
            .any(|msg|
                msg._type == MessageType::ConnReq && msg.status_code == MessageStatusCode::Accepted
            );
        if is_exists {
            return Err(err_msg("Accepted connection already exists."));
        }
        Ok(())
    }

    fn check_valid_phone_no(&self, phone_no: &str) -> Result<(), Error> {
        trace!("AgentConnection::check_valid_phone_no >> {:?}", phone_no);

        let phone_no = phone_no.replace("+", "").replace("-", "").replace(" ", "").replace("(", "").replace(")", "");
        if !phone_no.chars().all(|c| c.is_numeric()) {
            return Err(err_msg("Invalid phone number."));
        }
        Ok(())
    }

    fn verify_agent_key_dlg_proof(&self,
                                  sender_verkey: &str,
                                  key_dlg_proof: &KeyDlgProof) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::verify_agent_key_dlg_proof >> {:?}, {:?}",
               sender_verkey, key_dlg_proof);

        let signature = base64::decode(&key_dlg_proof.signature).unwrap();

        crypto::indy_verify(sender_verkey, &key_dlg_proof.challenge().as_bytes(), &signature)
            .map_err(|err| err.context("Agent key delegation proof verification failed.").into())
            .and_then(|valid|
                if !valid {
                    err!(err_msg("Agent key delegation proof verification failed.")).into()
                } else {
                    future::ok(()).into_box()
                }
            )
            .into_box()
    }

    fn check_if_message_status_can_be_updated(&self,
                                              uid: &str,
                                              status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::check_if_message_status_can_be_updated >> {:?}, {:?}",
               uid, status_code);

        let message = self.messages.get(uid)
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
                              reply_to_msg_id: Option<&str>) -> ResponseFuture<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::send_message_if_needed >> {:?}, {:?}, {:?}", send_msg, uid, reply_to_msg_id);

        if !send_msg {
            return future::ok(Vec::new()).into_box();
        }

        self.send_messages(vec![(uid.to_string(), reply_to_msg_id.map(String::from))])
    }

    fn send_messages(&mut self,
                     msgs: Vec<(String, Option<String>)>) -> ResponseFuture<Vec<A2AMessage>, Error> {
        trace!("AgentConnection::send_messages >> {:?}", msgs);

        msgs
            .into_iter()
            .map(|(msg_uid, reply_to)| {
                let message = self.messages.get(&msg_uid).cloned().unwrap();
                match message._type {
                    MessageType::ConnReq => self.send_invite_message(&msg_uid),
                    MessageType::ConnReqAnswer => self.send_invite_answer_message(&message, reply_to),
                    _ => self.send_general_message(&message, &msg_uid, reply_to.as_ref().map(String::as_str)),
                }
                    .map(|_| msg_uid.to_string())
            })
            .collect::<Result<Vec<_>, Error>>() // TODO: change on ResponseFuture
            .map(|uids| vec![A2AMessage::MessageSent(MessageSent { uids })])
            .into_future()
            .into_box()
    }

    fn send_invite_message(&mut self,
                           _uid: &str) -> Result<(), Error> {
        unimplemented!() // TODO: send invite sms?
    }

    fn send_invite_answer_message(&mut self,
                                  message: &InternalMessage,
                                  reply_to: Option<String>) -> Result<(), Error> {
        let reply_to = reply_to.ok_or(err_msg("Missed required field."))?;

        if message.status_code != MessageStatusCode::Accepted {
            return Err(err_msg("Message status isn't accepted."));
        }

        let invite_answer = self.build_invite_answer_message(message, &reply_to)?;

        let message = self.prepare_remote_message(invite_answer)?;

        let endpoint = self.remote_connection_detail.as_ref().map(|detail| detail.forward_agent_detail.endpoint.to_string())
            .ok_or(err_msg("Missed remote Forward Agent Endpoit."))?;

        request::send_message_to_remote_endpoint(message, &endpoint)
    }

    fn send_general_message(&self, message: &InternalMessage, _uid: &str, reply_to: Option<&str>) -> Result<(), Error> {
        if message.sender_did == self.user_pairwise_did {
            // TODO: support message delivery statuses?
            let message = self.build_general_message(message, reply_to)?;
            let message = self.prepare_remote_message(message)?;

            let endpoint = self.remote_connection_detail.as_ref().map(|detail| detail.forward_agent_detail.endpoint.to_string())
                .ok_or(err_msg("Missed remote Forward Agent Endpoit."))?;

            request::send_message_to_remote_endpoint(message, &endpoint)
        } else {
            // TODO: Notify user
            Ok(())
        }
    }

    fn prepare_remote_message(&self, message: Vec<A2AMessage>) -> Result<Vec<u8>, Error> {
        let remote_connection_detail = self.remote_connection_detail.as_ref()
            .ok_or(err_msg("Missed Remote Connection Details."))?;

        let remote_forward_agent_detail = &remote_connection_detail.forward_agent_detail.verkey;
        let remote_agent_verkey = &remote_connection_detail.agent_detail.verkey;
        let remote_agent_pairwise_verkey = &remote_connection_detail.agent_key_dlg_proof.agent_delegated_key;

        let message = A2AMessage::bundle_authcrypted(self.wallet_handle,
                                                     &self.user_pairwise_verkey,
                                                     &remote_agent_pairwise_verkey,
                                                     &message).wait()?;

        let fwd_message = self.build_forward_message(&remote_agent_pairwise_verkey, message)?;

        let message = A2AMessage::bundle_authcrypted(self.wallet_handle,
                                                     &self.owner_verkey,
                                                     &remote_agent_verkey,
                                                     fwd_message.as_slice()).wait()?;

        let fwd_message = self.build_forward_message(&remote_agent_verkey, message)?;

        let message = A2AMessage::bundle_anoncrypted(&remote_forward_agent_detail, fwd_message.as_slice()).wait()?;

        Ok(message)
    }

    fn build_forward_message(&self, fwd: &str, message: Vec<u8>) -> Result<Vec<A2AMessage>, Error> {
        Ok(vec![A2AMessage::Forward(Forward { fwd: fwd.to_string(), msg: message })])
    }

    fn build_invite_message(&self,
                            msg: InternalMessage,
                            msg_detail: ConnectionRequestMessageDetail) -> Vec<A2AMessage> {
        let msg_created = MessageCreated { uid: msg.uid.clone() };

        let status_msg = msg.status_code.message().to_string();
        let msg_detail = ConnectionRequestMessageDetailResp {
            invite_detail: InviteDetail {
                conn_req_id: msg.uid.clone(),
                target_name: msg_detail.target_name,
                sender_agency_detail: self.forward_agent_detail.clone(),
                sender_detail: SenderDetail {
                    did: self.agent_did.clone(),
                    verkey: self.agent_verkey.clone(),
                    agent_key_dlg_proof: msg_detail.key_dlg_proof,
                    name: None,
                    logo_url: None,
                },
                status_code: msg.status_code,
                status_msg,
            },
            url_to_invite_detail: "".to_string() // format!("{}/agency/invite/{}?msg_uid{}", AGENCY_DOMAIN_URL_PREFIX, self.agent_pairwise_did, msg_uid)
        };

        vec![A2AMessage::MessageCreated(msg_created),
             A2AMessage::MessageDetail(MessageDetail::ConnectionRequestResp(msg_detail))]
    }

    fn build_invite_answer_message(&self, message: &InternalMessage, reply_to: &str) -> Result<Vec<A2AMessage>, Error> {
        let msg_create = CreateMessage {
            mtype: message._type.clone(),
            send_msg: false,
            uid: Some(message.uid.clone()),
            reply_to_msg_id: Some(reply_to.to_string()),
        };

        let msg_detail = ConnectionRequestAnswerMessageDetail {
            key_dlg_proof: self.agent_key_dlg_proof.clone(),
            sender_detail: SenderDetail {
                did: self.user_pairwise_did.clone(),
                verkey: self.user_pairwise_verkey.clone(),
                agent_key_dlg_proof: self.agent_key_dlg_proof.clone().unwrap(),
                name: None,
                logo_url: None,
            },
            sender_agency_detail: self.forward_agent_detail.clone(),
            answer_status_code: MessageStatusCode::Accepted,
        };

        let messages =
            vec![A2AMessage::CreateMessage(msg_create),
                 A2AMessage::MessageDetail(MessageDetail::ConnectionRequestAnswer(msg_detail))];

        Ok(messages)
    }

    fn build_general_message(&self, message: &InternalMessage, reply_to: Option<&str>) -> Result<Vec<A2AMessage>, Error> {
        let msg_create = CreateMessage {
            mtype: message._type.clone(),
            send_msg: false,
            uid: Some(message.uid.clone()),
            reply_to_msg_id: reply_to.map(String::from),
        };

        let title = message.sending_data.get("title").and_then(|val| val.clone());
        let detail = message.sending_data.get("detail").and_then(|val| val.clone());

        let msg = message.payload.clone()
            .ok_or(err_msg("Missed Payload."))?;

        let msg_detail = GeneralMessageDetail { msg, title, detail };

        let messages =
            vec![A2AMessage::CreateMessage(msg_create),
                 A2AMessage::MessageDetail(MessageDetail::General(msg_detail))];

        Ok(messages)
    }
}

impl Actor for AgentConnection {
    type Context = Context<Self>;
}

impl Handler<HandleA2AMsg> for AgentConnection {
    type Result = ResponseActFuture<Self, Vec<u8>, Error>;

    fn handle(&mut self, msg: HandleA2AMsg, _: &mut Self::Context) -> Self::Result {
        trace!("Handler<AgentMsgsBundle>::handle >> {:?}", msg);
        self.handle_a2a_msg(msg.0)
    }
}

enum MessageHandlerRole {
    Sender,
    Recipient
}