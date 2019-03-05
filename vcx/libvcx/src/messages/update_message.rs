use settings;
use messages::*;
use messages::message_type::MessageTypes;
use utils::{httpclient, error};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessageStatusByConnections {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    status_code: Option<MessageStatusCode>,
    uids_by_conns: Vec<UIDsByConn>
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessageStatusByConnectionsResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    status_code: Option<String>,
    updated_uids_by_conns: Vec<UIDsByConn>
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct UIDsByConn {
    #[serde(rename = "pairwiseDID")]
    pairwise_did: String,
    uids: Vec<String>,
}

struct UpdateMessageStatusByConnectionsBuilder {
    status_code: Option<MessageStatusCode>,
    uids_by_conns: Vec<UIDsByConn>
}

impl UpdateMessageStatusByConnectionsBuilder {
    pub fn create() -> UpdateMessageStatusByConnectionsBuilder {
        trace!("UpdateMessageStatusByConnectionsBuilder::create >>>");

        UpdateMessageStatusByConnectionsBuilder {
            status_code: None,
            uids_by_conns: Vec::new(),
        }
    }

    pub fn uids_by_conns(&mut self, uids_by_conns: Vec<UIDsByConn>) -> Result<&mut Self, u32> {
        //Todo: validate msg_uid??
        self.uids_by_conns = uids_by_conns;
        Ok(self)
    }

    pub fn status_code(&mut self, code: MessageStatusCode) -> Result<&mut Self, u32> {
        //Todo: validate that it can be parsed to number??
        self.status_code = Some(code.clone());
        Ok(self)
    }

    pub fn send_secure(&mut self) -> Result<(), u32> {
        trace!("UpdateMessages::send >>>");

        if settings::test_agency_mode_enabled() {
            ::utils::httpclient::set_next_u8_response(::utils::constants::UPDATE_MESSAGES_RESPONSE.to_vec());
        }

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data).or(Err(error::POST_MSG_FAILURE.code_num))?;

        self.parse_response(&response)
    }

    fn prepare_request(&mut self) -> Result<Vec<u8>, u32> {
        let message = match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 =>
                A2AMessage::Version1(
                    A2AMessageV1::UpdateMessageStatusByConnections(
                        UpdateMessageStatusByConnections {
                            msg_type: MessageTypes::build(A2AMessageKinds::UpdateMessageStatusByConnections),
                            uids_by_conns: self.uids_by_conns.clone(),
                            status_code: self.status_code.clone(),
                        }
                    )
                ),
            settings::ProtocolTypes::V2 =>
                A2AMessage::Version2(
                    A2AMessageV2::UpdateMessageStatusByConnections(
                        UpdateMessageStatusByConnections {
                            msg_type: MessageTypes::build(A2AMessageKinds::UpdateMessageStatusByConnections),
                            uids_by_conns: self.uids_by_conns.clone(),
                            status_code: self.status_code.clone(),
                        }
                    )
                ),
        };

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
        prepare_message_for_agency(&message, &agency_did)
    }

    fn parse_response(&self, response: &Vec<u8>) -> Result<(), u32> {
        trace!("parse_create_keys_response >>>");

        let mut response = parse_response_from_agency(response)?;

        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::UpdateMessageStatusByConnectionsResponse(res)) => Ok(()),
            A2AMessage::Version2(A2AMessageV2::UpdateMessageStatusByConnectionsResponse(res)) => Ok(()),
            _ => Err(error::INVALID_HTTP_RESPONSE.code_num)
        }
    }
}

pub fn update_agency_messages(status_code: &str, msg_json: &str) -> Result<(), u32> {
    trace!("update_agency_messages >>> status_code: {:?}, msg_json: {:?}", status_code, msg_json);

    let status_code: MessageStatusCode = serde_json::from_str(&format!("\"{}\"", status_code)).or(Err(error::INVALID_JSON.code_num))?;

    debug!("updating agency messages {} to status code: {:?}", msg_json, status_code);

    let uids_by_conns: Vec<UIDsByConn> = serde_json::from_str(msg_json).or(Err(error::INVALID_JSON.code_num))?;

    UpdateMessageStatusByConnectionsBuilder::create()
        .uids_by_conns(uids_by_conns)?
        .status_code(status_code)?
        .send_secure()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_parse_update_messages_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        UpdateMessageStatusByConnectionsBuilder::create().parse_response(&::utils::constants::UPDATE_MESSAGES_RESPONSE.to_vec()).unwrap();
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_update_agency_messages() {
        use super::*;
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
        // AS CONSUMER GET MESSAGES
        ::utils::devsetup::tests::set_consumer();
        let pending = ::messages::get_message::download_messages(None, Some(vec!["MS-103".to_string()]), None).unwrap();
        assert!(pending.len() > 0);
        let did = pending[0].pairwise_did.clone();
        let uid = pending[0].msgs[0].uid.clone();
        let message = serde_json::to_string(&vec![UIDsByConn { pairwise_did: did, uids: vec![uid] }]).unwrap();
        update_agency_messages("MS-106", &message).unwrap();
        let updated = ::messages::get_message::download_messages(None, Some(vec!["MS-106".to_string()]), None).unwrap();
        assert_eq!(pending[0].msgs[0].uid, updated[0].msgs[0].uid);

        teardown!("agency");
    }
}
