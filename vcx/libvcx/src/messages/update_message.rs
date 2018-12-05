extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct UIDsByConn{
    #[serde(rename = "pairwiseDID")]
    pairwise_did: String,
    uids: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct UpdateMessages {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    status_code: Option<String>,
    uids_by_conns: Vec<UIDsByConn>
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct UpdateMessagesResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    status_code: Option<String>,
    updated_uids_by_conns: Vec<UIDsByConn>
}

impl UpdateMessages{

    pub fn send_secure(&mut self) -> Result<(), u32> {
        trace!("UpdateMessages::send >>>");

        let data = encode::to_vec_named(&self).or(Err(error::UNKNOWN_ERROR.code_num))?;
        trace!("update_message content: {:?}", data);

        let msg = Bundled::create(data).encode()?;

        let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
        let data = bundle_for_agency(msg, &to_did)?;

        if settings::test_agency_mode_enabled() {
            ::utils::httpclient::set_next_u8_response(::utils::constants::UPDATE_MESSAGES_RESPONSE.to_vec());
        }

        match httpclient::post_u8(&data) {
            Err(_) => return Err(error::POST_MSG_FAILURE.code_num),
            Ok(response) => if settings::test_agency_mode_enabled() && response.len() == 0 {
                return Ok(());
            } else {
                parse_update_messages_response(response)
            },
        }
    }
}

fn parse_update_messages_response(response: Vec<u8>) -> Result<(), u32> {
    let data = unbundle_from_agency(response)?;

    debug!("parse_update_messages_response: {:?}", data[0]);
    let mut de = Deserializer::new(&data[0][..]);
    let response: UpdateMessagesResponse = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);

            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    Ok(())
}

pub fn update_agency_messages(status_code: &str, msg_json: &str) -> Result<(), u32> {
    trace!("update_agency_messages >>> status_code: {:?}, msg_json: {:?}", status_code, msg_json);

    debug!("updating agency messages {} to status code: {}", msg_json, status_code);
    let uids_by_conns: Vec<UIDsByConn> = serde_json::from_str(msg_json)
        .map_err(|ec| {error::INVALID_JSON.code_num})?;
    let mut messages = UpdateMessages {
        msg_type: MsgType { name: "UPDATE_MSG_STATUS_BY_CONNS".to_string(), ver: "1.0".to_string(), },
        uids_by_conns,
        status_code: Some(status_code.to_string()),
    };

    match messages.send_secure() {
        Ok(x) => Ok(x),
        Err(x) => Err(error::POST_MSG_FAILURE.code_num)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_parse_update_messages_response() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let result = parse_update_messages_response(::utils::constants::UPDATE_MESSAGES_RESPONSE.to_vec()).unwrap();
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

        let (schema_id, _, cred_def_id, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def(::utils::constants::DEFAULT_SCHEMA_ATTRS);
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let credential_offer = ::issuer_credential::issuer_credential_create(cred_def_id.clone(),
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
        let message = serde_json::to_string(&vec![UIDsByConn{ pairwise_did: did, uids: vec![uid]}]).unwrap();
        update_agency_messages("MS-106",&message).unwrap();
        let updated = ::messages::get_message::download_messages(None, Some(vec!["MS-106".to_string()]), None).unwrap();
        assert_eq!(pending[0].msgs[0].uid, updated[0].msgs[0].uid);

        teardown!("agency");
    }
}
