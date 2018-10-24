use actix::prelude::*;
use actors::{AddA2ARoute, HandleA2AMsg};
use actors::router::Router;
use domain::a2a::*;
use domain::constants::AGENCY_DOMAIN_URL_PREFIX;
use domain::status::{ConnectionStatus, MessageStatusCode};
use domain::invite::{ForwardAgentDetail, InviteDetail, SenderDetail};
use domain::internal_message::InternalMessage;
use domain::key_deligation_proof::KeyDlgProof;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{did, crypto};
use std::convert::Into;
use utils::futures::*;

use base64;

use std::collections::HashMap;

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

#[allow(unused)] // FIXME: Use!
pub struct AgentConnection {
    // Agent wallet handle
    wallet_handle: i32,
    // Agent Owner DID
    pub owner_did: String,
    // Agent Owner Verkey
    pub owner_verkey: String,
    // Agent DID
    agent_did: String,
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
    // Remote User Forward Agent info
    remote_forward_agent_detail: Option<ForwardAgentDetail>,
    // Remote Agent Key Delegation Proof
    remote_agent_key_dlg_proof: Option<KeyDlgProof>,
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
                    user_pairwise_did: config.user_pairwise_did,
                    user_pairwise_verkey: config.user_pairwise_verkey,
                    agent_pairwise_did: config.agent_pairwise_did.clone(),
                    agent_pairwise_verkey: config.agent_pairwise_verkey,
                    forward_agent_detail: ForwardAgentDetail {
                        did: String::new(),
                        verkey: String::new(),
                        endpoint: String::new(),
                    },
                    agent_key_dlg_proof: None,
                    remote_agent_key_dlg_proof: None,
                    remote_forward_agent_detail: None,
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
            .and_then(move |(sender_vk, mut msgs), slf, _| {
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
            .and_then(move |msgs, slf, _| {
                A2AMessage::bundle_authcrypted(slf.wallet_handle, &slf.agent_pairwise_verkey, &slf.owner_verkey, &msgs)
                    .map_err(|err| err.context("Can't bundle and authcrypt message.").into())
                    .into_actor(slf)
            })
            .into_box()
    }

    fn handle_create_message(&mut self,
                             msg: CreateMessage,
                             tail: Vec<A2AMessage>,
                             sender_verkey: String) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_create_message >> {:?}, {:?}, {:?}", msg, tail, sender_verkey);

        let CreateMessage { mtype, send_msg, reply_to_msg_id } = msg;

        match mtype {
            MessageType::ConnReq => {
                self.handle_create_connection_request(send_msg, tail, sender_verkey)
            }
            MessageType::ConnReqAnswer => {
                self.handle_create_connection_request_answer(send_msg, tail, reply_to_msg_id)
            }
            type_ @ _ =>
                self.handle_create_general_message(type_, send_msg, tail, reply_to_msg_id)
        }
    }

    fn handle_send_messages(&mut self,
                            msg: SendMessages) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_send_messages >> {:?}",
               msg);

        let SendMessages { uids } = msg;

        let uids: Vec<(String, Option<String>)> = uids.into_iter().map(|uid| (uid, None)).collect();

        self.send_msg(true,
                      uids,
                      HashMap::new())
            .map(|resp| vec![resp.unwrap()])
            .into_actor(self)
            .into_box()
    }

    fn handle_get_messages(&mut self,
                           msg: GetMessages) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
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

    fn handle_update_connection_status(&mut self,
                                       msg: UpdateConnectionStatus) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
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

    fn handle_update_message_status(&mut self,
                                    msg: UpdateMessageStatus) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
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

    fn handle_create_connection_request(&mut self,
                                        send_msg: bool,
                                        mut tail: Vec<A2AMessage>,
                                        sender_verkey: String) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_create_connection_request >> {:?}, {:?}, {:?}", send_msg, tail, sender_verkey);

        if sender_verkey != self.user_pairwise_verkey {
            return err_act!(self, err_msg("Unknown message sender"));
        }

        let msg_detail = match tail.pop() {
            Some(A2AMessage::MessageDetail(MessageDetail::ConnectionRequestMessageDetail(msg))) => msg,
            _ => return err_act!(self, err_msg("Inconsistent message"))
        };

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _|
                slf.validate_connection_request(&msg_detail)
                    .into_future()
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .and_then(move |msg_detail, slf, _|
                slf.verify_agent_key_dlg_proof(&slf.user_pairwise_verkey, &msg_detail.key_dlg_proof)
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .map(move |msg_detail, slf, _| {
                let msg = InternalMessage::new(None,
                                               &MessageType::ConnReq,
                                               MessageStatusCode::Created,
                                               &slf.user_pairwise_did,
                                               None);
                slf.store_message(&msg);
                slf.agent_key_dlg_proof = Some(msg_detail.key_dlg_proof.clone());

                (msg, msg_detail)
            })
            .and_then(move |(msg, msg_detail), slf, _| {
                let sending_details = map! { "phone_no" => msg_detail.phone_no.clone() };

                slf.send_msg(send_msg, vec![(msg.uid.clone(), None)], sending_details)
                    .map(|sent_msg| (msg, msg_detail, sent_msg))
                    .into_actor(slf)
            })
            .map(move |(msg, msg_detail, sent_msg), slf, _| {
                let msg_created_resp = MessageCreated { uid: msg.uid.clone() };

                let msg_detail_resp = ConnectionRequestMessageDetailResp {
                    invite_detail: InviteDetail {
                        conn_req_id: msg.uid.clone(),
                        target_name: msg_detail.target_name,
                        sender_agency_detail: slf.forward_agent_detail.clone(),
                        sender_detail: SenderDetail {
                            did: slf.user_pairwise_did.clone(),
                            verkey: slf.user_pairwise_verkey.clone(),
                            agent_key_dlg_proof: msg_detail.key_dlg_proof,
                            name: None,
                            logo_url: None,
                        },
                        status_code: msg.status_code.clone(),
                        status_msg: msg.status_code.message().to_string(),
                    },
                    url_to_invite_detail: slf.get_invite_url(&msg.uid)
                };

                let mut messages =
                    vec![A2AMessage::MessageCreated(msg_created_resp),
                         A2AMessage::MessageDetail(MessageDetail::ConnectionRequestMessageDetailResp(msg_detail_resp))];

                sent_msg.map(|msg| messages.push(msg));

                messages
            })
            .into_box()
    }

    fn handle_create_connection_request_answer(&mut self,
                                               send_msg: bool,
                                               mut tail: Vec<A2AMessage>,
                                               reply_to_msg_id: Option<String>) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_create_connection_request_answer >> {:?}, {:?}, {:?}", send_msg, tail, reply_to_msg_id);

        let reply_to_msg_id = match reply_to_msg_id {
            Some(msg_id) => msg_id,
            None => return err_act!(self, err_msg("Required fied `reply_to_msg_id` is missed"))
        };

        let msg_detail = match tail.pop() {
            Some(A2AMessage::MessageDetail(MessageDetail::ConnectionRequestAnswerMessageDetail(msg))) => msg,
            _ => return err_act!(self, err_msg("Inconsistent message"))
        };

        future::ok(())
            .into_actor(self)
            .and_then(|_, slf, _|
                slf.validate_connection_request_answer(&msg_detail).into_future()
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .and_then(move |msg_detail, slf, _|
                slf.verify_agent_key_dlg_proof(&slf.user_pairwise_verkey, &msg_detail.key_dlg_proof)
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .and_then(move |msg_detail, slf, _|
                slf.verify_agent_key_dlg_proof(&msg_detail.sender_detail.verkey, &msg_detail.sender_detail.agent_key_dlg_proof)
                    .map(|_| msg_detail)
                    .into_actor(slf)
            )
            .map(move |msg_detail, slf, _| {
                if slf.messages.get(&reply_to_msg_id).is_none() {
                    let msg_created = InternalMessage::new(Some(&reply_to_msg_id),
                                                           &MessageType::ConnReq,
                                                           MessageStatusCode::Created,
                                                           &msg_detail.sender_detail.did,
                                                           None);
                    slf.store_message(&msg_created);
                }
                (msg_detail, reply_to_msg_id)
            })
            .map(move |(msg_detail, reply_to_msg_id), slf, _| {
                let answer_msg = InternalMessage::new(None,
                                                      &MessageType::ConnReqAnswer,
                                                      msg_detail.answer_status_code.clone(),
                                                      &slf.user_pairwise_did,
                                                      None);

                slf.store_message(&answer_msg);
                slf.agent_key_dlg_proof = Some(msg_detail.key_dlg_proof.clone());
                slf.remote_agent_key_dlg_proof = Some(msg_detail.sender_detail.agent_key_dlg_proof.clone());
                slf.remote_forward_agent_detail = Some(msg_detail.sender_agency_detail.clone());

                (msg_detail, reply_to_msg_id, answer_msg.uid)
            })
            .and_then(move |(msg_detail, reply_to_msg_id, uid), slf, _| {
                slf.answer_message(&reply_to_msg_id, &uid, &msg_detail.answer_status_code).into_future()
                    .map(|_| (msg_detail, uid))
                    .into_actor(slf)
            })
            .and_then(move |(msg_detail, uid), slf, _| {
                slf.store_their_did(&msg_detail.sender_detail.did, &msg_detail.sender_detail.verkey)
                    .map(|_| (msg_detail, uid))
                    .into_actor(slf)
            })
            .and_then(move |(msg_detail, uid), slf, _| {
                slf.store_their_did(&msg_detail.sender_agency_detail.did, &msg_detail.sender_agency_detail.verkey)
                    .map(|_| uid)
                    .into_actor(slf)
            })
            .and_then(move |uid, slf, _| {
                slf.send_msg(send_msg, vec![(uid.clone(), None)], HashMap::new())
                    .map(|sent_message| (uid, sent_message))
                    .into_actor(slf)
            })
            .map(move |(uid, sent_message), _, _| {
                let mut messages = vec![A2AMessage::MessageCreated(MessageCreated { uid: uid.clone() })];

                sent_message.map(|msg| messages.push(msg));

                messages
            }).into_box()
    }


    fn handle_create_general_message(&mut self,
                                     mtype: MessageType,
                                     send_msg: bool,
                                     mut tail: Vec<A2AMessage>,
                                     reply_to_msg_id: Option<String>) -> ResponseActFuture<Self, Vec<A2AMessage>, Error> {
        trace!("AgentConnection::handle_create_general_message >> {:?}, {:?}, {:?}", send_msg, tail, reply_to_msg_id);

        let msg_detail = match tail.pop() {
            Some(A2AMessage::MessageDetail(MessageDetail::GeneralMessageDetail(msg))) => msg,
            _ => return err_act!(self, err_msg("Inconsistent message"))
        };

        if let Err(err) = self.validate_general_message(reply_to_msg_id.as_ref().map(String::as_str)) {
            return err_act!(self, err);
        }

        let msg = InternalMessage::new(None, &mtype, MessageStatusCode::Created, &self.user_pairwise_did, Some(msg_detail.msg.clone()));
        self.store_message(&msg);

        if let Some(msg_id) = reply_to_msg_id.as_ref() {
            self.answer_message(msg_id, &msg.uid, &MessageStatusCode::Accepted).unwrap();
        }

        future::ok(())
            .into_actor(self)
            .and_then(move |_, slf, _| {
                slf.send_msg(send_msg, vec![(msg.uid.clone(), reply_to_msg_id.clone())], HashMap::new())
                    .map(|sent_message| (msg.uid, sent_message))
                    .into_actor(slf)
            })
            .map(move |(uid, sent_message), _, _| {
                let mut messages = vec![A2AMessage::MessageCreated(MessageCreated { uid: uid.clone() })];

                sent_message.map(|msg| messages.push(msg));

                messages
            })
            .into_box()
    }


    fn validate_general_message(&self, reply_to_msg_id: Option<&str>) -> Result<(), Error> {
        if let Some(msg_id) = reply_to_msg_id {
            let message = self.messages.get(msg_id)
                .ok_or(err_msg("Message not found."))?;

            self.check_if_message_not_already_answered(&message.status_code)?;
        }
        Ok(())
    }

    fn validate_connection_request(&self,
                                   msg_detail: &ConnectionRequestMessageDetail) -> Result<(), Error> {
        trace!("AgentConnection::validate_connection_request >> {:?}", msg_detail);

        self.check_no_connection_established()?;
        self.check_no_accepted_invitation_exists()?;
        self.check_valid_phone_no(&msg_detail.phone_no)?;
        Ok(())
    }

    fn validate_connection_request_answer(&self,
                                          msg_detail: &ConnectionRequestAnswerMessageDetail) -> Result<(), Error> {
        trace!("AgentConnection::validate_connection_request_answer >> {:?}", msg_detail);

        self.check_no_accepted_invitation_exists()?;
        self.check_valid_status_code(&msg_detail.answer_status_code)?;
        self.check_if_message_not_already_answered(&msg_detail.answer_status_code)?;
        // TODO: check same endpoints?

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

    fn store_their_did(&self,
                       did: &str,
                       verkey: &str) -> ResponseFuture<(), Error> {
        trace!("AgentConnection::store_their_did >> {:?}, {:?}", did, verkey);

        let their_did_info = json!({
            "did": did,
            "verkey": verkey,
        }).to_string();

        did::store_their_did(self.wallet_handle, &their_did_info)
            .map_err(|err| err.context("Can't create my DID for pairwise.").into())
            .into_box()
    }

    fn get_invite_url(&self, msg_uid: &str) -> String {
        trace!("AgentConnection::get_invite_url >> {:?}", msg_uid);

        format!("{}/agency/invite/{}?msg_uid{}", AGENCY_DOMAIN_URL_PREFIX, self.agent_pairwise_did, msg_uid)
    }

    fn check_if_message_not_already_answered(&self,
                                             status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::check_if_message_not_already_answered >> {:?}", status_code);

        if MessageStatusCode::valid_status_codes().contains(status_code) {
            return Err(err_msg("Message is already answered."));
        }
        Ok(())
    }


    fn check_valid_status_code(&self,
                               status_code: &MessageStatusCode) -> Result<(), Error> {
        trace!("AgentConnection::check_valid_status_code >> {:?}", status_code);

        if !MessageStatusCode::valid_status_codes().contains(status_code) {
            return Err(err_msg("Invalid answer status code."));
        }
        Ok(())
    }

    fn check_no_connection_established(&self) -> Result<(), Error> {
        if self.remote_forward_agent_detail.is_some() {
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
        trace!("AgentConnection::verify_agent_key_dlg_proof >> {:?}, {:?}", sender_verkey, key_dlg_proof);

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

    fn check_if_message_status_can_be_updated(&self, uid: &str, status_code: &MessageStatusCode) -> Result<(), Error> {
        let message = self.messages.get(uid)
            .ok_or(err_msg("Message not found."))?;

        if self.check_if_message_not_already_answered(status_code).is_err() {
            return Err(err_msg("Message is already answered."));
        }

        if !MessageStatusCode::valid_new_message_status_codes_allowed_update_to().contains(status_code) {
            return Err(err_msg("Invalid update status code."));
        }

        if !MessageStatusCode::valid_existing_message_statuses_to_update().contains(&message.status_code) {
            return Err(err_msg("Message is not in a state where it can be updated with the given status code."));
        }
        Ok(())
    }

    fn send_msg(&mut self,
                send_msg: bool,
                msgs: Vec<(String, Option<String>)>,
                sending_details: HashMap<&str, String>) -> ResponseFuture<Option<A2AMessage>, Error> {
        if !send_msg {
            return future::ok(None).into_box();
        }

        msgs
            .into_iter()
            .map(|(msg_uid, reply_to)| {
                let message = self.messages.get(&msg_uid).cloned();
                match message.map(|m| m._type) {
                    Some(MessageType::ConnReq) => self.send_invite_message(&msg_uid, &sending_details),
                    Some(MessageType::ConnReqAnswer) => self.send_invite_answer_message(&msg_uid),
                    Some(_) => self.send_general_message(&msg_uid, reply_to.as_ref().map(String::as_str)),
                    None => Err(err_msg("Message not found."))
                }
                    .map(|_| msg_uid.to_string())
            })
            .collect::<Result<Vec<_>, Error>>() // TODO: change on ResponseFuture
            .map(|uids| Some(A2AMessage::MessageSent(MessageSent { uids })))
            .into_future()
            .into_box()
    }

    fn send_invite_message(&mut self,
                           _uid: &str,
                           _sending_details: &HashMap<&str, String>) -> Result<(), Error> {
        unimplemented!() // TODO: send invite sms?
    }

    fn send_invite_answer_message(&self, _uid: &str) -> Result<(), Error> {
        self.send_to_remote_connection();
        unimplemented!()
    }

    fn send_general_message(&self, _uid: &str, _reply_to: Option<&str>) -> Result<(), Error> {
        self.send_to_remote_connection();
        unimplemented!()
    }

    fn send_to_remote_connection(&self) {
        unimplemented!()
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
