use settings;
use messages::*;
use messages::message_type::MessageTypes;
use utils::{httpclient, error};
use utils::libindy::crypto;

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

#[derive(Debug)]
pub struct GetMessagesBuilder {
    to_did: String,
    payload: GetMessages,
    to_vk: String,
    agent_did: String,
    agent_vk: String,
}

impl GetMessagesBuilder {
    pub fn create() -> GetMessagesBuilder {
        trace!("GetMessages::create_message >>>");

        GetMessagesBuilder {
            to_did: String::new(),
            to_vk: String::new(),
            payload: GetMessages {
                msg_type: MessageTypes::build(A2AMessageKinds::GetMessages),
                uids: None,
                exclude_payload: None,
                status_codes: None,
                pairwise_dids: None,
            },
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }

    pub fn uid(&mut self, uids: Option<Vec<String>>) -> Result<&mut Self, u32> {
        //Todo: validate msg_uid??
        self.payload.uids = uids;
        Ok(self)
    }

    pub fn status_codes(&mut self, status_codes: Option<Vec<String>>) -> Result<&mut Self, u32> {
        let status_codes =
            match status_codes {
                Some(codes) => {
                    let codes = codes
                        .iter()
                        .map(|code|
                            serde_json::from_str::<MessageStatusCode>(&format!("\"{}\"", code)).or(Err(error::INVALID_JSON.code_num))
                        ).collect::<Result<Vec<MessageStatusCode>, u32>>()?;
                    Some(codes)
                }
                None => None
            };

        self.payload.status_codes = status_codes;
        Ok(self)
    }

    pub fn pairwise_dids(&mut self, pairwise_dids: Option<Vec<String>>) -> Result<&mut Self, u32> {
        //Todo: validate msg_uid??
        self.payload.pairwise_dids = pairwise_dids;
        Ok(self)
    }

    pub fn include_edge_payload(&mut self, payload: &str) -> Result<&mut Self, u32> {
        //todo: is this a json value, String??
        self.payload.exclude_payload = Some(payload.to_string());
        Ok(self)
    }

    pub fn send_secure(&mut self) -> Result<Vec<Message>, u32> {
        trace!("GetMessages::send >>>");

        let data = self.prepare()?;

        let response = httpclient::post_u8(&data).or(Err(error::POST_MSG_FAILURE.code_num))?;

        if settings::test_agency_mode_enabled() && response.len() == 0 {
            return Ok(Vec::new());
        }

        let response = GetMessagesBuilder::parse_response(response)?;

        Ok(response)
    }

    fn parse_response(response: Vec<u8>) -> Result<Vec<Message>, u32> {
        trace!("parse_get_messages_response >>>");
        let mut response = parse_response_from_agency(&response)?;
        let response: GetMessagesResponse = GetMessagesResponse::from_a2a_message(response.remove(0))?;
        Ok(response.msgs)
    }

    pub fn download_messages(&mut self) -> Result<Vec<MessageByConnection>, u32> {
        trace!("GetMessages::download >>>");

        self.payload.msg_type = MessageTypes::build(A2AMessageKinds::GetMessagesByConnections);

        let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        let data = prepare_message_for_agency(&A2AMessage::GetMessagesByConnections(self.payload.clone()), &to_did)?;

        let response = httpclient::post_u8(&data).or(Err(error::POST_MSG_FAILURE.code_num))?;

        if settings::test_agency_mode_enabled() && response.len() == 0 {
            return Ok(Vec::new());
        }

        let response = GetMessagesBuilder::parse_get_connection_messages_response(response)?;

        Ok(response)
    }

    fn parse_get_connection_messages_response(response: Vec<u8>) -> Result<Vec<MessageByConnection>, u32> {
        trace!("parse_get_connection_messages_response >>>");
        let mut response = parse_response_from_agency(&response)?;
        let response: MessagesByConnections = MessagesByConnections::from_a2a_message(response.remove(0))?;

        response.msgs
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

    fn prepare(&mut self) -> Result<Vec<u8>, u32> {
        let messages = vec![A2AMessage::GetMessages(self.payload.clone())];
        prepare_message_for_agent(messages, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    #[serde(rename = "@type")]
    msg_type: PayloadTypes,
    #[serde(rename = "@msg")]
    pub msg: Vec<i8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryDetails {
    to: String,
    status_code: String,
    last_updated_date_time: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(rename = "statusCode")]
    pub status_code: MessageStatusCode,
    pub payload: Option<Vec<i8>>,
    #[serde(rename = "senderDID")]
    pub sender_did: String,
    pub uid: String,
    #[serde(rename = "type")]
    pub msg_type: String,
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
            let payload = ::messages::to_u8(payload);
            let payload = ::utils::libindy::crypto::parse_msg(&vk, &payload).unwrap_or((String::new(), Vec::new()));
            new_message.decrypted_payload = rmp_serde::from_slice(&payload.1[..]).ok();
        }
        new_message.payload = None;
        new_message
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
    pairwise_did: String,
    msgs: Vec<Message>,
}

pub fn get_connection_messages(pw_did: &str, pw_vk: &str, agent_did: &str, agent_vk: &str, msg_uid: Option<Vec<String>>) -> Result<Vec<Message>, u32> {
    trace!("get_connection_messages >>> pw_did: {}, pw_vk: {}, agent_vk: {}, msg_uid: {:?}",
           pw_did, pw_vk, agent_vk, msg_uid);

    let response = get_messages()
        .to(&pw_did)?
        .to_vk(&pw_vk)?
        .agent_did(&agent_did)?
        .agent_vk(&agent_vk)?
        .uid(msg_uid)?
        .send_secure()
        .map_err(|err| {
            error!("could not post get_messages: {}", err);
            error::POST_MSG_FAILURE.code_num
        })?;

    trace!("message returned: {:?}", response);
    Ok(response)
}

pub fn get_ref_msg(msg_id: &str, pw_did: &str, pw_vk: &str, agent_did: &str, agent_vk: &str) -> Result<(String, Vec<u8>), u32> {
    trace!("get_ref_msg >>> msg_id: {}, pw_did: {}, pw_vk: {}, agent_did: {}, agent_vk: {}",
           msg_id, pw_did, pw_vk, agent_did, agent_vk);

    let message: Vec<Message> = get_connection_messages(pw_did, pw_vk, agent_did, agent_vk, Some(vec![msg_id.to_string()]))?;
    trace!("checking for ref_msg: {:?}", message);

    let msg_id = match message.get(0).as_ref().and_then(|message| message.ref_msg_id.as_ref()) {
        Some(ref ref_msg_id) if message[0].status_code == MessageStatusCode::Accepted => ref_msg_id.to_string(),
        _ => return Err(error::NOT_READY.code_num),
    };

    let message = get_connection_messages(pw_did, pw_vk, agent_did, agent_vk, Some(vec![msg_id]))?;

    trace!("checking for pending message: {:?}", message);

    // this will work for both credReq and proof types
    match message.get(0).as_ref().and_then(|message| message.payload.as_ref()) {
        Some(ref payload) if message[0].status_code == MessageStatusCode::Pending => {
            // TODO: check returned verkey
            let (_, msg) = crypto::parse_msg(&pw_vk, &to_u8(payload))?;
            Ok((message[0].uid.clone(), msg))
        }
        _ => Err(error::INVALID_HTTP_RESPONSE.code_num)
    }
}

pub fn download_messages(pairwise_dids: Option<Vec<String>>, status_codes: Option<Vec<String>>, uids: Option<Vec<String>>) -> Result<Vec<MessageByConnection>, u32> {
    trace!("download_messages >>> pairwise_dids: {:?}, status_codes: {:?}, uids: {:?}",
           pairwise_dids, status_codes, uids);

    if settings::test_agency_mode_enabled() {
        ::utils::httpclient::set_next_u8_response(::utils::constants::GET_ALL_MESSAGES_RESPONSE.to_vec());
    }

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

        let result = GetMessagesBuilder::parse_response(GET_MESSAGES_RESPONSE.to_vec()).unwrap();
        assert_eq!(result.len(), 3)
    }

    #[test]
    fn test_parse_get_connection_messages_response() {
        init!("true");

        let json: serde_json::Value = rmp_serde::from_slice(GET_ALL_MESSAGES_RESPONSE).unwrap();
        let result = GetMessagesBuilder::parse_get_connection_messages_response(GET_ALL_MESSAGES_RESPONSE.to_vec()).unwrap();
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
            payload: Some(vec![-9, 108, 97, 105, 109, 45, 100, 97, 116, 97]),
            sender_did: "WVsWVh8nL96BE3T3qwaCd5".to_string(),
            uid: "mmi3yze".to_string(),
            msg_type: "connReq".to_string(),
            ref_msg_id: None,
            delivery_details: vec![delivery_details1],
            decrypted_payload: None,
        };
        let msg2 = Message {
            status_code: MessageStatusCode::Created,
            payload: None,
            sender_did: "WVsWVh8nL96BE3T3qwaCd5".to_string(),
            uid: "zjcynmq".to_string(),
            msg_type: "credOffer".to_string(),
            ref_msg_id: None,
            delivery_details: Vec::new(),
            decrypted_payload: None,
        };

        let response = GetMessagesResponse {
            msg_type: MessageTypes::MessageTypeV0(MessageTypeV1 { name: "MSGS".to_string(), ver: "1.0".to_string() }),
            msgs: vec![msg1, msg2],
        };

        let my_vk = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY).unwrap();
        let verkey = settings::get_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY).unwrap();

        let data = rmp_serde::to_vec_named(&response).unwrap();
        let bundle = Bundled::create(data).encode().unwrap();
        let message = crypto::prep_msg(&my_vk, &verkey, &bundle[..]).unwrap();

        let result = GetMessagesBuilder::parse_response(message).unwrap();
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_download_messages() {
        use std::thread;
        use std::time::Duration;
        ::utils::logger::LibvcxDefaultLogger::init_testing_logger();

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
        // AS CONSUMER GET MESSAGES
        ::utils::devsetup::tests::set_consumer();
        let all_messages = download_messages(None, None, None).unwrap();
        println!("{}", serde_json::to_string(&all_messages).unwrap());
        let pending = download_messages(None, Some(vec!["MS-103".to_string()]), None).unwrap();
        assert_eq!(pending.len(), 1);
        let accepted = download_messages(None, Some(vec!["MS-104".to_string()]), None).unwrap();
        assert_eq!(accepted[0].msgs.len(), 2);
        let specific = download_messages(None, None, Some(vec![accepted[0].msgs[0].uid.clone()])).unwrap();
        assert_eq!(specific.len(), 1);
        // No pending will return empty list
        let empty = download_messages(None, Some(vec!["MS-103".to_string()]), Some(vec![accepted[0].msgs[0].uid.clone()])).unwrap();
        assert_eq!(empty.len(), 1);
        // Agency returns a bad request response for invalid dids
        let invalid_did = "abc".to_string();
        let bad_req = download_messages(Some(vec![invalid_did]), None, None);
        assert_eq!(bad_req, Err(error::POST_MSG_FAILURE.code_num));
        teardown!("agency");
    }
}
