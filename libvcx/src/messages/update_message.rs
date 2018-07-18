extern crate rust_base58;
extern crate serde_json;
extern crate serde;
extern crate rmp_serde;

use settings;
use utils::httpclient;
use utils::error;
use messages::*;
use error::ToErrorCode;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
#[serde(rename_all = "camelCase")]
struct UpdateMessagesPayload{
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(skip_serializing_if = "Option::is_none")]
    uids: Option<String>,
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessages {
    #[serde(rename = "to")]
    to_did: String,
    agent_payload: String,
    #[serde(skip_serializing, default)]
    payload: UpdateMessagesPayload,
    #[serde(skip_serializing, default)]
    to_vk: String,
    #[serde(skip_serializing, default)]
    validate_rc: u32,
    #[serde(skip_serializing, default)]
    agent_did: String,
    #[serde(skip_serializing, default)]
    agent_vk: String,
}

impl UpdateMessages{

    pub fn create() -> UpdateMessages {
        UpdateMessages {
            to_did: String::new(),
            to_vk: String::new(),
            payload: UpdateMessagesPayload{
                msg_type: MsgType { name: "UPDATE_MSG_STATUS".to_string(), ver: "1.0".to_string(), },
                uids: None,
                status_code: None,
            },
            agent_payload: String::new(),
            validate_rc: error::SUCCESS.code_num,
            agent_did: String::new(),
            agent_vk: String::new(),
        }
    }

    pub fn uid(&mut self, uid: &str) -> &mut Self{
        //Todo: validate msg_uid??
        self.payload.uids = Some(uid.to_string());
        self
    }

    pub fn send_secure(&mut self) -> Result<(), u32> {
        let data = match self.msgpack() {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

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

//Todo: Every GeneralMessage extension, duplicates code
impl GeneralMessage for UpdateMessages{
    type Msg = UpdateMessages;

    fn set_agent_did(&mut self, did: String) { self.agent_did = did; }
    fn set_agent_vk(&mut self, vk: String) { self.agent_vk = vk; }
    fn set_to_did(&mut self, to_did: String){ self.to_did = to_did; }
    fn set_validate_rc(&mut self, rc: u32){ self.validate_rc = rc; }
    fn set_to_vk(&mut self, to_vk: String){ self.to_vk = to_vk; }

    fn msgpack(&mut self) -> Result<Vec<u8>,u32> {
        if self.validate_rc != error::SUCCESS.code_num {
            return Err(self.validate_rc)
        }

        let data = encode::to_vec_named(&self.payload).unwrap();
        trace!("update_message content: {:?}", data);

        let msg = Bundled::create(data).encode()?;

        bundle_for_agent(msg, &self.to_vk, &self.agent_did, &self.agent_vk)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMsgType {
    pub name: String,
    pub ver: String,
    pub fmt: String,
}

fn parse_update_messages_response(response: Vec<u8>) -> Result<(), u32> {
    let data = unbundle_from_agency(response)?;

    trace!("get_message response: {:?}", data[0]);
    let mut de = Deserializer::new(&data[0][..]);
    let response: UpdateMessagesPayload = match Deserialize::deserialize(&mut de) {
        Ok(x) => x,
        Err(x) => {
            error!("Could not parse messagepack: {}", x);

            return Err(error::INVALID_MSGPACK.code_num)
        },
    };

    Ok(())
}

pub fn update_agency_messages(connection_handle: u32, status_code: &str, uids: &str) -> Result<(), u32> {
    let pw_did = ::connection::get_pw_did(connection_handle).map_err(|ec|ec.to_error_code())?;
    let pw_vk = ::connection::get_pw_verkey(connection_handle).map_err(|ec|ec.to_error_code())?;
    let agent_did = ::connection::get_agent_did(connection_handle).map_err(|ec|ec.to_error_code())?;
    let agent_vk = ::connection::get_agent_verkey(connection_handle).map_err(|ec|ec.to_error_code())?;

    let mut messages = UpdateMessages {
        to_did: pw_did,
        to_vk: pw_vk,
        payload: UpdateMessagesPayload{
            msg_type: MsgType { name: "UPDATE_MSG_STATUS".to_string(), ver: "1.1".to_string(), },
            uids: Some(uids.to_string()),
            status_code: Some(status_code.to_string()),
        },
        agent_payload: String::new(),
        validate_rc: error::SUCCESS.code_num,
        agent_did: agent_did,
        agent_vk: agent_vk,
    };

    debug!("updating messages");
    match messages.send_secure() {
        Ok(x) => Ok(x),
        Err(x) => Err(error::POST_MSG_FAILURE.code_num)
    }
}

#[cfg(test)]
mod tests {

    #[ignore]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_update_agency_messages() {
        use super::*;
        use std::thread;
        use std::time::Duration;
        settings::set_defaults();
        ::utils::devsetup::tests::setup_local_env("test_update_agency_messages");
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (faber, alice) = ::connection::tests::create_connected_connections();

        println!("creating schema/credential_def and paying fees");
        let (schema_id, _, cred_def_id, _) = ::utils::libindy::anoncreds::tests::create_and_store_credential_def();
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let credential_offer = ::issuer_credential::issuer_credential_create(cred_def_id.clone(),
                                                                             "1".to_string(),
                                                                             institution_did.clone(),
                                                                             "credential_name".to_string(),
                                                                             credential_data.to_owned(),
                                                                             1).unwrap();
        println!("sending credential offer");
        ::issuer_credential::send_credential_offer(credential_offer, alice).unwrap();
        thread::sleep(Duration::from_millis(2000));
        // AS CONSUMER GET MESSAGES
        ::utils::devsetup::tests::set_consumer();
        let pending = ::messages::get_message::download_messages(Some(vec!["MS-103".to_string()]), None).unwrap();
        assert!(pending.len() > 0);
        update_agency_messages(faber, "MS-106", &pending[0].msgs[0].uid).unwrap();
        let updated = ::messages::get_message::download_messages(Some(vec!["MS-106".to_string()]), None).unwrap();
        assert_eq!(pending[0].msgs[0].uid, updated[0].msgs[0].uid);

        ::utils::devsetup::tests::cleanup_dev_env("test_update_agency_messages");
    }
}
