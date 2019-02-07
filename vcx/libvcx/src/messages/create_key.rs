use settings;
use messages::*;
use utils::{httpclient, error};
use utils::constants::CREATE_KEYS_RESPONSE;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateKey {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "forDID")]
    for_did: String,
    #[serde(rename = "forDIDVerKey")]
    for_verkey: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKeyResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "withPairwiseDID")]
    pub for_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub for_verkey: String,
}

#[derive(Debug)]
pub struct CreateKeyBuilder {
    payload: CreateKey,
}

impl CreateKeyBuilder {
    pub fn create() -> CreateKeyBuilder {
        trace!("CreateKeyBuilder::create_message >>>");

        CreateKeyBuilder {
            payload: CreateKey {
                msg_type: MessageTypes::build(A2AMessageKinds::CreateKey),
                for_did: String::new(),
                for_verkey: String::new(),
            },
        }
    }

    pub fn for_did(&mut self, did: &str) -> Result<&mut Self, u32> {
        validation::validate_did(did)?;
        self.payload.for_did = did.to_string();
        Ok(self)
    }

    pub fn for_verkey(&mut self, verkey: &str) -> Result<&mut Self, u32> {
        validation::validate_verkey(verkey)?;
        self.payload.for_verkey = verkey.to_string();
        Ok(self)
    }

    pub fn send_secure(&mut self) -> Result<CreateKeyResponse, u32> {
        trace!("CreateKeyMsg::send >>>");

        if settings::test_agency_mode_enabled() {
            return CreateKeyBuilder::parse_response(&CREATE_KEYS_RESPONSE.to_vec());
        }

        let data = self.prepare()?;

        let response = httpclient::post_u8(&data).or(Err(error::POST_MSG_FAILURE.code_num))?;

        CreateKeyBuilder::parse_response(&response)
    }

    fn prepare(&mut self) -> Result<Vec<u8>, u32> {
        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
        prepare_message_for_agency(&A2AMessage::CreateKey(self.payload.clone()), &agency_did)
    }

    fn parse_response(response: &Vec<u8>) -> Result<CreateKeyResponse, u32> {
        trace!("parse_create_keys_response >>>");
        let mut response = parse_response_from_agency(response)?;
        let response = CreateKeyResponse::from_a2a_message(response.remove(0))?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::constants::{MY1_SEED, MY2_SEED, MY3_SEED};
    use utils::libindy::signus::create_and_store_my_did;
    use messages::create_keys;
    use messages::message_type::{MessageTypes, MessageTypeV1};

    #[test]
    fn test_create_key_returns_message_with_create_key_as_payload() {
        let msg = create_keys();
        let msg_payload = CreateKey {
            for_did: String::new(),
            for_verkey: String::new(),
            msg_type: MessageTypes::MessageTypeV0(MessageTypeV1 { name: "CREATE_KEY".to_string(), ver: "1.0".to_string() }),
        };
        assert_eq!(msg.payload, msg_payload);
    }

    #[test]
    fn test_create_key_set_values() {
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let for_did = "11235yBzrpJQmNyZzgoTqB";
        let for_verkey = "EkVTa7SCJ5SntpYyX7CSb2pcBhiVGT9kWSagA8a9T69A";
        let msg_payload = CreateKey {
            for_did: for_did.to_string(),
            for_verkey: for_verkey.to_string(),
            msg_type: MessageTypes::MessageTypeV0(MessageTypeV1 { name: "CREATE_KEY".to_string(), ver: "1.0".to_string() }),
        };
        let msg = create_keys()
            .for_did(for_did).unwrap()
            .for_verkey(for_verkey).unwrap().payload.clone();
        assert_eq!(msg, msg_payload);
    }

    #[test]
    fn test_create_key_set_values_and_serialize() {
        init!("false");

        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let bytes = create_keys()
            .for_did(&my_did).unwrap()
            .for_verkey(&my_vk).unwrap()
            .prepare().unwrap();
        assert!(bytes.len() > 0);
    }

    #[test]
    fn test_parse_create_keys_response() {
        init!("true");

        let result = CreateKeyBuilder::parse_response(&CREATE_KEYS_RESPONSE.to_vec()).unwrap();

        assert_eq!(result.for_did, "U5LXs4U7P9msh647kToezy");
        assert_eq!(result.for_verkey, "FktSZg8idAVzyQZrdUppK6FTrfAzW3wWVzAjJAfdUvJq");
    }

    #[test]
    fn test_create_key_set_invalid_did_errors() {
        let for_did = "11235yBzrpJQmNyZzgoT";
        let res = create_keys().for_did(for_did).unwrap_err();
        assert_eq!(res, error::INVALID_DID.code_num);
    }
}

