use settings;
use messages::*;
use messages::message_type::MessageTypes;
use messages::payload::Payloads;
use utils::httpclient;
use error::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetMessages {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "excludePayload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uids: Option<Vec<String>>,
    #[serde(rename = "statusCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    status_codes: Option<Vec<MessageStatusCode>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pairwiseDIDs")]
    pairwise_dids: Option<Vec<String>>,
}

impl GetMessages {
    fn build(kind: A2AMessageKinds, exclude_payload: Option<String>, uids: Option<Vec<String>>,
             status_codes: Option<Vec<MessageStatusCode>>, pairwise_dids: Option<Vec<String>>) -> GetMessages {
        GetMessages {
            msg_type: MessageTypes::build(kind),
            exclude_payload,
            uids,
            status_codes,
            pairwise_dids,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMessagesResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    msgs: Vec<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagesByConnections {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "msgsByConns")]
    #[serde(default)]
    msgs: Vec<MessageByConnection>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MessageByConnection {
    #[serde(rename = "pairwiseDID")]
    pub pairwise_did: String,
    pub msgs: Vec<Message>,
}

#[derive(Debug)]
pub struct GetMessagesBuilder {
    to_did: String,
    to_vk: String,
    agent_did: String,
    agent_vk: String,
    exclude_payload: Option<String>,
    uids: Option<Vec<String>>,
    status_codes: Option<Vec<MessageStatusCode>>,
    pairwise_dids: Option<Vec<String>>,
}

impl GetMessagesBuilder {
    pub fn create() -> GetMessagesBuilder {
        trace!("GetMessages::create_message >>>");

        GetMessagesBuilder {
            to_did: String::new(),
            to_vk: String::new(),
            agent_did: String::new(),
            agent_vk: String::new(),
            uids: None,
            exclude_payload: None,
            status_codes: None,
            pairwise_dids: None,
        }
    }

    pub fn uid(&mut self, uids: Option<Vec<String>>) -> VcxResult<&mut Self> {
        //Todo: validate msg_uid??
        self.uids = uids;
        Ok(self)
    }

    pub fn status_codes(&mut self, status_codes: Option<Vec<MessageStatusCode>>) -> VcxResult<&mut Self> {
        self.status_codes = status_codes;
        Ok(self)
    }

    pub fn pairwise_dids(&mut self, pairwise_dids: Option<Vec<String>>) -> VcxResult<&mut Self> {
        //Todo: validate msg_uid??
        self.pairwise_dids = pairwise_dids;
        Ok(self)
    }

    pub fn include_edge_payload(&mut self, payload: &str) -> VcxResult<&mut Self> {
        //todo: is this a json value, String??
        self.exclude_payload = Some(payload.to_string());
        Ok(self)
    }

    pub fn send_secure(&mut self) -> VcxResult<Vec<Message>> {
        trace!("GetMessages::send >>>");

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        if settings::test_agency_mode_enabled() && response.len() == 0 {
            return Ok(Vec::new());
        }

        self.parse_response(response)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<Vec<Message>> {
        trace!("parse_get_messages_response >>>");

        let mut response = parse_response_from_agency(&response)?;

        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::GetMessagesResponse(res)) => Ok(res.msgs),
            A2AMessage::Version2(A2AMessageV2::GetMessagesResponse(res)) => Ok(res.msgs),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of GetMessagesResponse"))
        }
    }

    pub fn download_messages(&mut self) -> VcxResult<Vec<MessageByConnection>> {
        trace!("GetMessages::download >>>");

        let data = self.prepare_download_request()?;

        let response = httpclient::post_u8(&data)?;

        if settings::test_agency_mode_enabled() && response.len() == 0 {
            return Ok(Vec::new());
        }

        let response = GetMessagesBuilder::parse_download_messages_response(response)?;

        Ok(response)
    }

    fn prepare_download_request(&self) -> VcxResult<Vec<u8>> {
        let message = match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 =>
                A2AMessage::Version1(
                    A2AMessageV1::GetMessages(
                        GetMessages::build(A2AMessageKinds::GetMessagesByConnections,
                                           self.exclude_payload.clone(),
                                           self.uids.clone(),
                                           self.status_codes.clone(),
                                           self.pairwise_dids.clone()))
                ),
            settings::ProtocolTypes::V2 =>
                A2AMessage::Version2(
                    A2AMessageV2::GetMessages(
                        GetMessages::build(A2AMessageKinds::GetMessagesByConnections,
                                           self.exclude_payload.clone(),
                                           self.uids.clone(),
                                           self.status_codes.clone(),
                                           self.pairwise_dids.clone()))
                )
        };

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        prepare_message_for_agency(&message, &agency_did)
    }

    fn parse_download_messages_response(response: Vec<u8>) -> VcxResult<Vec<MessageByConnection>> {
        trace!("parse_get_connection_messages_response >>>");
        let mut response = parse_response_from_agency(&response)?;

        let msgs = match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::GetMessagesByConnectionsResponse(res)) => res.msgs,
            A2AMessage::Version2(A2AMessageV2::GetMessagesByConnectionsResponse(res)) => res.msgs,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of GetMessagesByConnectionsResponse"))
        };

        msgs
            .iter()
            .map(|connection| {
                ::utils::libindy::signus::get_local_verkey(&connection.pairwise_did)
                    .map(|vk| MessageByConnection {
                        pairwise_did: connection.pairwise_did.clone(),
                        msgs: connection.msgs.iter().map(|message| message.decrypt(&vk)).collect(),
                    })
            })
            .collect()
    }
}

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for GetMessagesBuilder {
    type Msg = GetMessagesBuilder;

    fn set_agent_did(&mut self, did: String) { self.agent_did = did; }
    fn set_agent_vk(&mut self, vk: String) { self.agent_vk = vk; }
    fn set_to_did(&mut self, to_did: String) { self.to_did = to_did; }
    fn set_to_vk(&mut self, to_vk: String) { self.to_vk = to_vk; }

    fn prepare_request(&mut self) -> VcxResult<Vec<u8>> {
        let message = match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 =>
                A2AMessage::Version1(
                    A2AMessageV1::GetMessages(
                        GetMessages::build(A2AMessageKinds::GetMessages,
                                           self.exclude_payload.clone(),
                                           self.uids.clone(),
                                           self.status_codes.clone(),
                                           self.pairwise_dids.clone()))
                ),
            settings::ProtocolTypes::V2 =>
                A2AMessage::Version2(
                    A2AMessageV2::GetMessages(
                        GetMessages::build(A2AMessageKinds::GetMessages,
                                           self.exclude_payload.clone(),
                                           self.uids.clone(),
                                           self.status_codes.clone(),
                                           self.pairwise_dids.clone()))
                )
        };

        prepare_message_for_agent(vec![message], &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryDetails {
    to: String,
    status_code: String,
    last_updated_date_time: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum MessagePayload {
    V1(Vec<i8>),
    V2(::serde_json::Value),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
    pub payload: Option<MessagePayload>,
    #[serde(rename = "senderDID")]
    pub sender_did: String,
    pub uid: String,
    #[serde(rename = "type")]
    pub msg_type: RemoteMessageType,
    pub ref_msg_id: Option<String>,
    #[serde(skip_deserializing)]
    pub delivery_details: Vec<DeliveryDetails>,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrypted_payload: Option<String>,
}

impl Message {
    pub fn decrypt(&self, vk: &str) -> Message {
        // TODO: must be Result
        let mut new_message = self.clone();
        if let Some(ref payload) = self.payload {
            let payload = match payload {
                MessagePayload::V1(payload) => Payloads::decrypt_payload_v1(&vk, &payload)
                    .map(|payload| Payloads::PayloadV1(payload)),
                MessagePayload::V2(payload) => Payloads::decrypt_payload_v2(&vk, &payload)
                    .map(|payload| Payloads::PayloadV2(payload))
            };

            new_message.decrypted_payload = match payload {
                Ok(payload) => ::serde_json::to_string(&payload).ok(),
                _ => ::serde_json::to_string(&json!(null)).ok()
            }
        }
        new_message.payload = None;
        new_message
    }
}

pub fn get_connection_messages(pw_did: &str, pw_vk: &str, agent_did: &str, agent_vk: &str, msg_uid: Option<Vec<String>>) -> VcxResult<Vec<Message>> {
    trace!("get_connection_messages >>> pw_did: {}, pw_vk: {}, agent_vk: {}, msg_uid: {:?}",
           pw_did, pw_vk, agent_vk, msg_uid);

    let response = get_messages()
        .to(&pw_did)?
        .to_vk(&pw_vk)?
        .agent_did(&agent_did)?
        .agent_vk(&agent_vk)?
        .uid(msg_uid)?
        .send_secure()
        .map_err(|err| err.map(VcxErrorKind::PostMessageFailed, "Cannot get messages"))?;

    trace!("message returned: {:?}", response);
    Ok(response)
}

pub fn get_ref_msg(msg_id: &str, pw_did: &str, pw_vk: &str, agent_did: &str, agent_vk: &str) -> VcxResult<(String, MessagePayload)> {
    trace!("get_ref_msg >>> msg_id: {}, pw_did: {}, pw_vk: {}, agent_did: {}, agent_vk: {}",
           msg_id, pw_did, pw_vk, agent_did, agent_vk);

    let message: Vec<Message> = get_connection_messages(pw_did, pw_vk, agent_did, agent_vk, Some(vec![msg_id.to_string()]))?;
    trace!("checking for ref_msg: {:?}", message);

    let msg_id = match message.get(0).as_ref().and_then(|message| message.ref_msg_id.as_ref()) {
        Some(ref ref_msg_id) if message[0].status_code == MessageStatusCode::Accepted => ref_msg_id.to_string(),
        _ => return Err(VcxError::from_msg(VcxErrorKind::NotReady, "Cannot find referent message")),
    };

    let message: Vec<Message> = get_connection_messages(pw_did, pw_vk, agent_did, agent_vk, Some(vec![msg_id]))?;

    trace!("checking for pending message: {:?}", message);

    // this will work for both credReq and proof types
    match message.get(0).as_ref().and_then(|message| message.payload.as_ref()) {
        Some(payload) if message[0].status_code == MessageStatusCode::Pending => {
            // TODO: check returned verkey
            Ok((message[0].uid.clone(), payload.to_owned()))
        }
        _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Cannot find referent message")),
    }
}

pub fn download_messages(pairwise_dids: Option<Vec<String>>, status_codes: Option<Vec<String>>, uids: Option<Vec<String>>) -> VcxResult<Vec<MessageByConnection>> {
    trace!("download_messages >>> pairwise_dids: {:?}, status_codes: {:?}, uids: {:?}",
           pairwise_dids, status_codes, uids);

    if settings::test_agency_mode_enabled() {
        ::utils::httpclient::set_next_u8_response(::utils::constants::GET_ALL_MESSAGES_RESPONSE.to_vec());
    }

    let status_codes =
        match status_codes {
            Some(codes) => {
                let codes = codes
                    .iter()
                    .map(|code|
                        ::serde_json::from_str::<MessageStatusCode>(&format!("\"{}\"", code))
                            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot parse message status code: {}", err)))
                    ).collect::<VcxResult<Vec<MessageStatusCode>>>()?;
                Some(codes)
            }
            None => None
        };

    let response =
        get_messages()
            .uid(uids)?
            .status_codes(status_codes)?
            .pairwise_dids(pairwise_dids)?
            .download_messages()?;

    trace!("message returned: {:?}", response);
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::{GET_MESSAGES_RESPONSE, GET_ALL_MESSAGES_RESPONSE};
    use messages::message_type::MessageTypeV1;


    #[test]
    fn test_parse_get_messages_response() {
        init!("true");

        let result = GetMessagesBuilder::create().parse_response(GET_MESSAGES_RESPONSE.to_vec()).unwrap();
        assert_eq!(result.len(), 3)
    }

    #[test]
    fn test_parse_get_connection_messages_response() {
        init!("true");

        let json: serde_json::Value = rmp_serde::from_slice(GET_ALL_MESSAGES_RESPONSE).unwrap();
        let result = GetMessagesBuilder::parse_download_messages_response(GET_ALL_MESSAGES_RESPONSE.to_vec()).unwrap();
        assert_eq!(result.len(), 1)
    }

    #[test]
    fn test_build_response() {
        init!("true");
        let delivery_details1 = DeliveryDetails {
            to: "3Xk9vxK9jeiqVaCPrEQ8bg".to_string(),
            status_code: "MDS-101".to_string(),
            last_updated_date_time: "2017-12-14T03:35:20.444Z[UTC]".to_string(),
        };

        let delivery_details2 = DeliveryDetails {
            to: "3Xk9vxK9jeiqVaCPrEQ8bg".to_string(),
            status_code: "MDS-101".to_string(),
            last_updated_date_time: "2017-12-14T03:35:20.500Z[UTC]".to_string(),
        };

        let msg1 = Message {
            status_code: MessageStatusCode::Accepted,
            payload: Some(MessagePayload::V1(vec![-9, 108, 97, 105, 109, 45, 100, 97, 116, 97])),
            sender_did: "WVsWVh8nL96BE3T3qwaCd5".to_string(),
            uid: "mmi3yze".to_string(),
            msg_type: RemoteMessageType::ConnReq,
            ref_msg_id: None,
            delivery_details: vec![delivery_details1],
            decrypted_payload: None,
        };
        let msg2 = Message {
            status_code: MessageStatusCode::Created,
            payload: None,
            sender_did: "WVsWVh8nL96BE3T3qwaCd5".to_string(),
            uid: "zjcynmq".to_string(),
            msg_type: RemoteMessageType::CredOffer,
            ref_msg_id: None,
            delivery_details: Vec::new(),
            decrypted_payload: None,
        };

        let response = GetMessagesResponse {
            msg_type: MessageTypes::MessageTypeV1(MessageTypeV1 { name: "MSGS".to_string(), ver: "1.0".to_string() }),
            msgs: vec![msg1, msg2],
        };

        let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY).unwrap();
        let verkey = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY).unwrap();

        let data = rmp_serde::to_vec_named(&response).unwrap();
        let bundle = Bundled::create(data).encode().unwrap();
        let message = crypto::prep_msg(&my_vk, &verkey, &bundle[..]).unwrap();

        let result = GetMessagesBuilder::create().parse_response(message).unwrap();
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_download_messages() {
        use std::thread;
        use std::time::Duration;

        init!("agency");
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (faber, alice) = ::connection::tests::create_connected_connections();

        let (_, cred_def_handle) = ::credential_def::tests::create_cred_def_real(false);
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let credential_offer = ::issuer_credential::issuer_credential_create(cred_def_handle,
                                                                             "1".to_string(),
                                                                             institution_did.clone(),
                                                                             "credential_name".to_string(),
                                                                             credential_data.to_owned(),
                                                                             1).unwrap();
        ::issuer_credential::send_credential_offer(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        let hello_uid = ::messages::send_message::send_generic_message(alice, "hello", &json!({"msg_type":"hello", "msg_title": "hello", "ref_msg_id": null}).to_string()).unwrap();
        // AS CONSUMER GET MESSAGES
        ::utils::devsetup::tests::set_consumer();
        let all_messages = download_messages(None, None, None).unwrap();
        let pending = download_messages(None, Some(vec!["MS-103".to_string()]), None).unwrap();
        assert_eq!(pending.len(), 1);
        assert!(pending[0].msgs[0].decrypted_payload.is_some());
        let accepted = download_messages(None, Some(vec!["MS-104".to_string()]), None).unwrap();
        assert_eq!(accepted[0].msgs.len(), 2);
        let specific = download_messages(None, None, Some(vec![accepted[0].msgs[0].uid.clone()])).unwrap();
        assert_eq!(specific.len(), 1);
        // No pending will return empty list
        let empty = download_messages(None, Some(vec!["MS-103".to_string()]), Some(vec![accepted[0].msgs[0].uid.clone()])).unwrap();
        assert_eq!(empty.len(), 1);
        let hello_msg = download_messages(None, None, Some(vec![hello_uid])).unwrap();
        assert_eq!(hello_msg[0].msgs[0].decrypted_payload, Some("{\"@type\":{\"name\":\"hello\",\"ver\":\"1.0\",\"fmt\":\"json\"},\"@msg\":\"hello\"}".to_string()));
        // Agency returns a bad request response for invalid dids
        let invalid_did = "abc".to_string();
        let bad_req = download_messages(Some(vec![invalid_did]), None, None);
        assert_eq!(bad_req.unwrap_err().kind(), VcxErrorKind::PostMessageFailed);
        teardown!("agency");
    }
}
