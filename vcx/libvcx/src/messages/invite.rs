use settings;
use messages::*;
use messages::message_type::{MessageTypes, MessageTypeV1, MessageTypeV2};
use messages::payload::Thread;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    target_name: Option<String>,
    #[serde(rename = "phoneNo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_no: Option<String>,
    #[serde(rename = "includePublicDID")]
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
    #[serde(rename = "~thread")]
    pub thread: Thread,
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
    #[serde(rename = "~thread")]
    pub thread: Thread,
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
    thread: Thread
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
            thread: Thread::new(),
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

    pub fn thread(&mut self, thread: &Thread) -> VcxResult<&mut Self> {
        self.thread = thread.clone();
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
    thread: Thread
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
            thread: Thread::new(),
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

    pub fn thread(&mut self, thread: &Thread) -> VcxResult<&mut Self> {
        self.thread = thread.clone();
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
                        thread: self.thread.clone(),
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
                        thread: self.thread.clone(),
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

pub fn parse_invitation_acceptance_details(payload: Vec<u8>) -> VcxResult<SenderDetail> {
    debug!("parsing invitation acceptance details: {:?}", payload);
    let response: AcceptanceDetails = rmp_serde::from_slice(&payload[..])
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidMessagePack, format!("Cannot decode acceptance details: {:?}", err)))?;
    Ok(response.sender_detail)
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

    #[test]
    fn test_parse_invitation_acceptance_details() {
        let payload = vec![129, 172, 115, 101, 110, 100, 101, 114, 68, 101, 116, 97, 105, 108, 131, 163, 68, 73, 68, 182, 67, 113, 85, 88, 113, 53, 114, 76, 105, 117, 82, 111, 100, 55, 68, 67, 52, 97, 86, 84, 97, 115, 166, 118, 101, 114, 75, 101, 121, 217, 44, 67, 70, 86, 87, 122, 118, 97, 103, 113, 65, 99, 117, 50, 115, 114, 68, 106, 117, 106, 85, 113, 74, 102, 111, 72, 65, 80, 74, 66, 111, 65, 99, 70, 78, 117, 49, 55, 113, 117, 67, 66, 57, 118, 71, 176, 97, 103, 101, 110, 116, 75, 101, 121, 68, 108, 103, 80, 114, 111, 111, 102, 131, 168, 97, 103, 101, 110, 116, 68, 73, 68, 182, 57, 54, 106, 111, 119, 113, 111, 84, 68, 68, 104, 87, 102, 81, 100, 105, 72, 49, 117, 83, 109, 77, 177, 97, 103, 101, 110, 116, 68, 101, 108, 101, 103, 97, 116, 101, 100, 75, 101, 121, 217, 44, 66, 105, 118, 78, 52, 116, 114, 53, 78, 88, 107, 69, 103, 119, 66, 56, 81, 115, 66, 51, 109, 109, 109, 122, 118, 53, 102, 119, 122, 54, 85, 121, 53, 121, 112, 122, 90, 77, 102, 115, 74, 56, 68, 122, 169, 115, 105, 103, 110, 97, 116, 117, 114, 101, 217, 88, 77, 100, 115, 99, 66, 85, 47, 99, 89, 75, 72, 49, 113, 69, 82, 66, 56, 80, 74, 65, 43, 48, 51, 112, 121, 65, 80, 65, 102, 84, 113, 73, 80, 74, 102, 52, 84, 120, 102, 83, 98, 115, 110, 81, 86, 66, 68, 84, 115, 67, 100, 119, 122, 75, 114, 52, 54, 120, 87, 116, 80, 43, 78, 65, 68, 73, 57, 88, 68, 71, 55, 50, 50, 103, 113, 86, 80, 77, 104, 117, 76, 90, 103, 89, 67, 103, 61, 61];
        println!("payload: {:?}", payload);
        let response = parse_invitation_acceptance_details(payload).unwrap();
        println!("response: {:?}", response);
    }

    #[test]
    fn test_send_invite_null_parameters() {
        let details = SendInviteMessageDetails {
            msg_type: MessageTypeV1 {
                name: "Name".to_string(),
                ver: "1.0".to_string()
            },
            key_dlg_proof: KeyDlgProof {
                agent_did: "did".to_string(),
                agent_delegated_key: "key".to_string(),
                signature: "sig".to_string(),
            },
            target_name: None,
            phone_no: None,
            include_public_did: true
        };

        let string: String = serde_json::to_string(&details).unwrap();
        assert!(!string.contains("phoneNo"));
        assert!(!string.contains("targetName"));
        assert!(string.contains("includePublicDID"));
    }
}
