use settings;
use messages::*;
use messages::message_type::{MessageTypes, MessageTypeV1, MessageTypeV2};
use utils::httpclient;
use utils::constants::*;
use utils::uuid::uuid;
use error::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SendInviteMessageDetails {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV1,
    #[serde(rename = "keyDlgProof")]
    key_dlg_proof: KeyDlgProof,
    #[serde(rename = "targetName")]
    target_name: Option<String>,
    #[serde(rename = "phoneNo")]
    phone_no: Option<String>,
    #[serde(rename = "usePublicDID")]
    include_public_did: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ConnectionRequest {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV2,
    #[serde(rename = "sendMsg")]
    send_msg: bool,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "replyToMsgId")]
    reply_to_msg_id: Option<String>,
    #[serde(rename = "keyDlgProof")]
    key_dlg_proof: KeyDlgProof,
    #[serde(rename = "targetName")]
    target_name: Option<String>,
    #[serde(rename = "phoneNo")]
    phone_no: Option<String>,
    #[serde(rename = "usePublicDID")]
    include_public_did: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ConnectionRequestResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV2,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "inviteDetail")]
    invite_detail: InviteDetail,
    #[serde(rename = "urlToInviteDetail")]
    url_to_invite_detail: String,
    sent: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AcceptInviteMessageDetails {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV1,
    #[serde(rename = "keyDlgProof")]
    key_dlg_proof: KeyDlgProof,
    #[serde(rename = "senderDetail")]
    sender_detail: Option<SenderDetail>,
    #[serde(rename = "senderAgencyDetail")]
    sender_agency_detail: Option<SenderAgencyDetail>,
    #[serde(rename = "answerStatusCode")]
    answer_status_code: Option<MessageStatusCode>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ConnectionRequestAnswer {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV2,
    #[serde(rename = "sendMsg")]
    send_msg: bool,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "replyToMsgId")]
    reply_to_msg_id: Option<String>,
    #[serde(rename = "keyDlgProof")]
    key_dlg_proof: KeyDlgProof,
    #[serde(rename = "senderDetail")]
    sender_detail: Option<SenderDetail>,
    #[serde(rename = "senderAgencyDetail")]
    sender_agency_detail: Option<SenderAgencyDetail>,
    #[serde(rename = "answerStatusCode")]
    answer_status_code: Option<MessageStatusCode>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KeyDlgProof {
    #[serde(rename = "agentDID")]
    agent_did: String,
    #[serde(rename = "agentDelegatedKey")]
    agent_delegated_key: String,
    #[serde(rename = "signature")]
    signature: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SenderDetail {
    name: Option<String>,
    agent_key_dlg_proof: KeyDlgProof,
    #[serde(rename = "DID")]
    pub did: String,
    logo_url: Option<String>,
    #[serde(rename = "verKey")]
    pub verkey: String,
    #[serde(rename = "publicDID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_did: Option<String>,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SenderAgencyDetail {
    #[serde(rename = "DID")]
    did: String,
    #[serde(rename = "verKey")]
    verkey: String,
    endpoint: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InviteDetail {
    status_code: String,
    pub conn_req_id: String,
    pub sender_detail: SenderDetail,
    pub sender_agency_detail: SenderAgencyDetail,
    target_name: String,
    status_msg: String,
    pub thread_id: Option<String>
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SendInviteMessageDetailsResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV1,
    #[serde(rename = "inviteDetail")]
    invite_detail: InviteDetail,
    #[serde(rename = "urlToInviteDetail")]
    url_to_invite_detail: String,
}

#[derive(Debug)]
pub struct SendInviteBuilder {
    to_did: String,
    to_vk: String,
    payload: SendInviteMessageDetails,
    agent_did: String,
    agent_vk: String,
    public_did: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ConnectionRequestAnswerResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypeV2,
    #[serde(rename = "@id")]
    id: String,
    sent: bool,
}

impl InviteDetail {
    pub fn new() -> InviteDetail {
        InviteDetail {
            status_code: String::new(),
            conn_req_id: String::new(),
            sender_detail: SenderDetail {
                name: Some(String::new()),
                agent_key_dlg_proof: KeyDlgProof {
                    agent_did: String::new(),
                    agent_delegated_key: String::new(),
                    signature: String::new(),
                },
                did: String::new(),
                logo_url: Some(String::new()),
                verkey: String::new(),
                public_did: None,
            },
            sender_agency_detail: SenderAgencyDetail {
                did: String::new(),
                verkey: String::new(),
                endpoint: String::new(),
            },
            target_name: String::new(),
            status_msg: String::new(),
            thread_id: None,
        }
    }
}

impl SendInviteBuilder {
    pub fn create() -> SendInviteBuilder {
        trace!("SendInvite::create_message >>>");

        SendInviteBuilder {
            to_did: String::new(),
            to_vk: String::new(),
            payload: SendInviteMessageDetails {
                msg_type: MessageTypes::build_v1(A2AMessageKinds::MessageDetail),
                key_dlg_proof: KeyDlgProof { agent_did: String::new(), agent_delegated_key: String::new(), signature: String::new() },
                target_name: None,
                phone_no: None,
                include_public_did: false,
            },
            agent_did: String::new(),
            agent_vk: String::new(),
            public_did: None,
        }
    }

    pub fn key_delegate(&mut self, key: &str) -> VcxResult<&mut Self> {
        validation::validate_key_delegate(key)?;
        self.payload.key_dlg_proof.agent_delegated_key = key.to_string();
        Ok(self)
    }

    pub fn public_did(&mut self, did: Option<&str>) -> VcxResult<&mut Self> {
        if did.is_some() {
            self.payload.include_public_did = true;
        }
        self.public_did = did.map(String::from);
        Ok(self)
    }

    pub fn phone_number(&mut self, phone_number: Option<&str>) -> VcxResult<&mut Self> {
        if let Some(ref p_num) = phone_number {
            validation::validate_phone_number(p_num)?;
            self.payload.phone_no = phone_number.map(String::from);
        }
        Ok(self)
    }

    pub fn generate_signature(&mut self) -> VcxResult<()> {
        let signature = format!("{}{}", self.payload.key_dlg_proof.agent_did, self.payload.key_dlg_proof.agent_delegated_key);
        let signature = ::utils::libindy::crypto::sign(&self.to_vk, signature.as_bytes())?;
        let signature = base64::encode(&signature);
        self.payload.key_dlg_proof.signature = signature;
        Ok(())
    }

    pub fn send_secure(&mut self) -> VcxResult<(InviteDetail, String)> {
        trace!("SendInvite::send >>>");

        if settings::test_agency_mode_enabled() {
            return self.parse_response(SEND_INVITE_RESPONSE.to_vec());
        }

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        let (invite, url) = self.parse_response(response)?;

        Ok((invite, url))
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<(InviteDetail, String)> {
        let mut response = parse_response_from_agency(&response)?;

        let index = match settings::get_protocol_type() {
            // TODO: THINK better
            settings::ProtocolTypes::V1 => 1,
            settings::ProtocolTypes::V2 => 0
        };

        match response.remove(index) {
            A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequestResp(res))) =>
                Ok((res.invite_detail, res.url_to_invite_detail)),
            A2AMessage::Version2(A2AMessageV2::ConnectionRequestResponse(res)) =>
                Ok((res.invite_detail, res.url_to_invite_detail)),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of ConnectionRequestResponse"))
        }
    }
}

#[derive(Debug)]
pub struct AcceptInviteBuilder {
    to_did: String,
    to_vk: String,
    payload: AcceptInviteMessageDetails,
    agent_did: String,
    agent_vk: String,
    reply_to_msg_id: Option<String>,
}

impl AcceptInviteBuilder {
    pub fn create() -> AcceptInviteBuilder {
        trace!("AcceptInvite::create_message >>>");


        AcceptInviteBuilder {
            to_did: String::new(),
            to_vk: String::new(),
            payload: AcceptInviteMessageDetails {
                msg_type: MessageTypes::build_v1(A2AMessageKinds::MessageDetail),
                key_dlg_proof: KeyDlgProof { agent_did: String::new(), agent_delegated_key: String::new(), signature: String::new() },
                sender_detail: None,
                sender_agency_detail: None,
                answer_status_code: None,
            },
            agent_did: String::new(),
            agent_vk: String::new(),
            reply_to_msg_id: None,
        }
    }

    pub fn key_delegate(&mut self, key: &str) -> VcxResult<&mut Self> {
        validation::validate_key_delegate(key)?;
        self.payload.key_dlg_proof.agent_delegated_key = key.to_string();
        Ok(self)
    }

    pub fn sender_details(&mut self, details: &SenderDetail) -> VcxResult<&mut Self> {
        self.payload.sender_detail = Some(details.clone());
        Ok(self)
    }

    pub fn sender_agency_details(&mut self, details: &SenderAgencyDetail) -> VcxResult<&mut Self> {
        self.payload.sender_agency_detail = Some(details.clone());
        Ok(self)
    }

    pub fn answer_status_code(&mut self, code: &MessageStatusCode) -> VcxResult<&mut Self> {
        self.payload.answer_status_code = Some(code.clone());
        Ok(self)
    }

    pub fn reply_to(&mut self, id: &str) -> VcxResult<&mut Self> {
        self.reply_to_msg_id = Some(id.to_string());
        Ok(self)
    }

    pub fn generate_signature(&mut self) -> VcxResult<()> {
        let signature = format!("{}{}", self.payload.key_dlg_proof.agent_did, self.payload.key_dlg_proof.agent_delegated_key);
        let signature = crypto::sign(&self.to_vk, signature.as_bytes())?;
        let signature = base64::encode(&signature);
        self.payload.key_dlg_proof.signature = signature;
        Ok(())
    }

    pub fn send_secure(&mut self) -> VcxResult<String> {
        trace!("AcceptInvite::send >>>");

        if settings::test_agency_mode_enabled() {
            return self.parse_response(ACCEPT_INVITE_RESPONSE.to_vec());
        }

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<String> {
        let mut response = parse_response_from_agency(&response)?;

        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::MessageCreated(res)) => Ok(res.uid),
            A2AMessage::Version2(A2AMessageV2::ConnectionRequestAnswerResponse(res)) => Ok(res.id),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of ConnectionAnswerResponse"))
        }
    }
}


//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for SendInviteBuilder {
    type Msg = SendInviteBuilder;

    fn set_agent_did(&mut self, did: String) {
        self.agent_did = did;
        self.payload.key_dlg_proof.agent_did = self.agent_did.clone();
    }

    fn set_agent_vk(&mut self, vk: String) {
        self.agent_vk = vk;
        self.payload.key_dlg_proof.agent_delegated_key = self.agent_vk.clone();
    }

    fn set_to_did(&mut self, to_did: String) { self.to_did = to_did; }

    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }

    fn prepare_request(&mut self) -> VcxResult<Vec<u8>> {
        self.generate_signature()?;

        let messages =
            match settings::get_protocol_type() {
                settings::ProtocolTypes::V1 => {
                    let create_msg = CreateMessage {
                        msg_type: MessageTypes::build_v1(A2AMessageKinds::CreateMessage),
                        mtype: RemoteMessageType::ConnReq,
                        reply_to_msg_id: None,
                        send_msg: true,
                        uid: None,
                    };

                    let details = self.payload.clone();

                    vec![A2AMessage::Version1(A2AMessageV1::CreateMessage(create_msg)),
                         A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequest(details)))]
                }
                settings::ProtocolTypes::V2 => {
                    let msg = ConnectionRequest {
                        msg_type: MessageTypes::build_v2(A2AMessageKinds::ConnectionRequest),
                        send_msg: true,
                        id: uuid(),
                        reply_to_msg_id: None,
                        key_dlg_proof: self.payload.key_dlg_proof.clone(),
                        target_name: self.payload.target_name.clone(),
                        phone_no: self.payload.phone_no.clone(),
                        include_public_did: self.payload.include_public_did,
                    };

                    vec![A2AMessage::Version2(A2AMessageV2::ConnectionRequest(msg))]
                }
            };

        prepare_message_for_agent(messages, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

impl GeneralMessage for AcceptInviteBuilder {
    type Msg = AcceptInviteBuilder;

    fn set_agent_did(&mut self, did: String) {
        self.agent_did = did;
        self.payload.key_dlg_proof.agent_did = self.agent_did.to_string();
    }

    fn set_agent_vk(&mut self, vk: String) {
        self.agent_vk = vk;
        self.payload.key_dlg_proof.agent_delegated_key = self.agent_vk.to_string();
    }

    fn set_to_did(&mut self, to_did: String) { self.to_did = to_did; }
    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }

    fn prepare_request(&mut self) -> VcxResult<Vec<u8>> {
        self.generate_signature()?;

        let messages =
            match settings::get_protocol_type() {
                settings::ProtocolTypes::V1 => {
                    let msg_created = CreateMessage {
                        msg_type: MessageTypes::build_v1(A2AMessageKinds::CreateMessage),
                        mtype: RemoteMessageType::ConnReqAnswer,
                        reply_to_msg_id: self.reply_to_msg_id.clone(),
                        send_msg: true,
                        uid: None,
                    };

                    vec![A2AMessage::Version1(A2AMessageV1::CreateMessage(msg_created)),
                         A2AMessage::Version1(A2AMessageV1::MessageDetail(MessageDetail::ConnectionRequestAnswer(self.payload.clone())))]
                }
                settings::ProtocolTypes::V2 => {
                    let msg = ConnectionRequestAnswer {
                        msg_type: MessageTypes::build_v2(A2AMessageKinds::ConnectionRequestAnswer),
                        send_msg: true,
                        id: uuid(),
                        reply_to_msg_id: self.reply_to_msg_id.clone(),
                        key_dlg_proof: self.payload.key_dlg_proof.clone(),
                        sender_detail: self.payload.sender_detail.clone(),
                        sender_agency_detail: self.payload.sender_agency_detail.clone(),
                        answer_status_code: self.payload.answer_status_code.clone(),
                    };

                    vec![A2AMessage::Version2(A2AMessageV2::ConnectionRequestAnswer(msg))]
                }
            };

        prepare_message_for_agent(messages, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    #[serde(rename = "@type")]
    msg_type: ::messages::payload::PayloadTypes,
    #[serde(rename = "@msg")]
    pub msg: Vec<i8>,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AcceptanceDetails {
    pub sender_detail: SenderDetail,
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::send_invite;
    use utils::libindy::signus::create_and_store_my_did;

    #[test]
    fn test_send_invite_set_values_and_post() {
        init!("false");
        let (user_did, user_vk) = create_and_store_my_did(None).unwrap();
        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = send_invite()
            .to(&user_did).unwrap()
            .to_vk(&user_vk).unwrap()
            .agent_did(&agent_did).unwrap()
            .agent_vk(&agent_vk).unwrap()
            .phone_number(Some("phone")).unwrap()
            .key_delegate("key").unwrap()
            .prepare_request().unwrap();

        assert!(msg.len() > 0);
    }

    #[test]
    fn test_parse_send_invite_response() {
        init!("indy");
        let (result, url) = SendInviteBuilder::create().parse_response(SEND_INVITE_RESPONSE.to_vec()).unwrap();
        let invite = serde_json::from_str(INVITE_DETAIL_STRING).unwrap();

        assert_eq!(result, invite);
        assert_eq!(url, "http://localhost:9001/agency/invite/WRUzXXuFVTYkT8CjSZpFvT?uid=NjcwOWU");
    }
}
