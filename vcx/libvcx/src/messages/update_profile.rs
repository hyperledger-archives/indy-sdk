use settings;
use messages::*;
use messages::message_type::MessageTypes;
use utils::httpclient;
use utils::constants::*;
use error::prelude::*;

#[derive(Debug)]
pub struct UpdateProfileDataBuilder {
    to_did: String,
    agent_payload: String,
    configs: Vec<ConfigOption>
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct ConfigOption {
    name: String,
    value: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct UpdateConfigs {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    configs: Vec<ConfigOption>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UpdateConfigsResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

impl UpdateProfileDataBuilder {
    pub fn create() -> UpdateProfileDataBuilder {
        trace!("UpdateProfileData::create_message >>>");

        UpdateProfileDataBuilder {
            to_did: String::new(),
            configs: Vec::new(),
            agent_payload: String::new(),
        }
    }

    pub fn to(&mut self, did: &str) -> VcxResult<&mut Self> {
        validation::validate_did(did)?;
        self.to_did = did.to_string();
        Ok(self)
    }

    pub fn name(&mut self, name: &str) -> VcxResult<&mut Self> {
        let config = ConfigOption { name: "name".to_string(), value: name.to_string() };
        self.configs.push(config);
        Ok(self)
    }

    pub fn logo_url(&mut self, url: &str) -> VcxResult<&mut Self> {
        validation::validate_url(url)?;
        let config = ConfigOption { name: "logoUrl".to_string(), value: url.to_string() };
        self.configs.push(config);
        Ok(self)
    }

    pub fn use_public_did(&mut self, did: &Option<String>) -> VcxResult<&mut Self> {
        if let Some(x) = did {
            let config = ConfigOption { name: "publicDid".to_string(), value: x.to_string() };
            self.configs.push(config);
        };
        Ok(self)
    }

    pub fn send_secure(&mut self) -> VcxResult<()> {
        trace!("UpdateProfileData::send_secure >>>");

        if settings::test_agency_mode_enabled() {
            return self.parse_response(UPDATE_PROFILE_RESPONSE.to_vec());
        }

        let data = self.prepare_request()?;

        let response = httpclient::post_u8(&data)?;

        self.parse_response(response)
    }

    fn prepare_request(&self) -> VcxResult<Vec<u8>> {
        let message = match settings::get_protocol_type() {
            settings::ProtocolTypes::V1 =>
                A2AMessage::Version1(
                    A2AMessageV1::UpdateConfigs(
                        UpdateConfigs {
                            msg_type: MessageTypes::build(A2AMessageKinds::UpdateConfigs),
                            configs: self.configs.clone()
                        }
                    )
                ),
            settings::ProtocolTypes::V2 =>
                A2AMessage::Version2(
                    A2AMessageV2::UpdateConfigs(
                        UpdateConfigs {
                            msg_type: MessageTypes::build(A2AMessageKinds::UpdateConfigs),
                            configs: self.configs.clone(),
                        }
                    )
                )
        };

        let agency_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

        prepare_message_for_agency(&message, &agency_did)
    }

    fn parse_response(&self, response: Vec<u8>) -> VcxResult<()> {
        let mut response = parse_response_from_agency(&response)?;

        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::UpdateConfigsResponse(res)) => Ok(()),
            A2AMessage::Version2(A2AMessageV2::UpdateConfigsResponse(res)) => Ok(()),
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of UpdateConfigsResponse"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use messages::update_data;
    use utils::libindy::signus::create_and_store_my_did;

    #[test]
    fn test_update_data_post() {
        init!("true");
        let to_did = "8XFh8yBzrpJQmNyZzgoTqB";
        let name = "name";
        let url = "https://random.com";
        let msg = update_data()
            .to(to_did).unwrap()
            .name(&name).unwrap()
            .logo_url(&url).unwrap()
            .prepare_request().unwrap();
    }

    #[test]
    fn test_update_data_set_values_and_post() {
        init!("false");
        let (agent_did, agent_vk) = create_and_store_my_did(Some(MY2_SEED)).unwrap();
        let (my_did, my_vk) = create_and_store_my_did(Some(MY1_SEED)).unwrap();
        let (agency_did, agency_vk) = create_and_store_my_did(Some(MY3_SEED)).unwrap();

        settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &agency_vk);
        settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agent_vk);
        settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

        let msg = update_data()
            .to(agent_did.as_ref()).unwrap()
            .name("name").unwrap()
            .logo_url("https://random.com").unwrap()
            .prepare_request().unwrap();
        assert!(msg.len() > 0);
    }

    #[test]
    fn test_parse_update_profile_response() {
        init!("indy");
        UpdateProfileDataBuilder::create().parse_response(UPDATE_PROFILE_RESPONSE.to_vec()).unwrap();
    }
}
