use settings;
use messages::*;
use utils::{httpclient, error};
use utils::constants::*;

#[derive(Debug)]
pub struct UpdateProfileDataBuilder {
    to_did: String,
    agent_payload: String,
    payload: UpdateConfigs,
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
            payload: UpdateConfigs {
                msg_type: MessageTypes::build(A2AMessageKinds::UpdateConfigs),
                configs: Vec::new(),
            },
            agent_payload: String::new(),
        }
    }

    pub fn to(&mut self, did: &str) -> Result<&mut Self, u32> {
        validation::validate_did(did)?;
        self.to_did = did.to_string();
        Ok(self)
    }

    pub fn name(&mut self, name: &str) -> Result<&mut Self, u32> {
        let config = ConfigOption { name: "name".to_string(), value: name.to_string() };
        self.payload.configs.push(config);
        Ok(self)
    }

    pub fn logo_url(&mut self, url: &str) -> Result<&mut Self, u32> {
        validation::validate_url(url)?;
        let config = ConfigOption { name: "logoUrl".to_string(), value: url.to_string() };
        self.payload.configs.push(config);
        Ok(self)
    }

    pub fn use_public_did(&mut self, did: &Option<String>) -> Result<&mut Self, u32> {
        if let Some(x) = did {
            let config = ConfigOption { name: "publicDid".to_string(), value: x.to_string() };
            self.payload.configs.push(config);
        };
        Ok(self)
    }

    pub fn send_secure(&mut self) -> Result<UpdateConfigsResponse, u32> {
        trace!("UpdateProfileData::send_secure >>>");

        if settings::test_agency_mode_enabled() {
            return UpdateProfileDataBuilder::parse_response(UPDATE_PROFILE_RESPONSE.to_vec());
        }

        let data = self.prepare()?;

        let response = httpclient::post_u8(&data).or(Err(error::POST_MSG_FAILURE.code_num))?;

        let response = UpdateProfileDataBuilder::parse_response(response)?;

        Ok(response)
    }

    fn prepare(&mut self) -> Result<Vec<u8>, u32> {
        let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
        prepare_message_for_agency(&A2AMessage::UpdateConfigs(self.payload.clone()), &to_did)
    }

    fn parse_response(response: Vec<u8>) -> Result<UpdateConfigsResponse, u32> {
        let mut messages = parse_response_from_agency(&response)?;
        UpdateConfigsResponse::from_a2a_message(messages.remove(0))
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
            .prepare().unwrap();
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
            .prepare().unwrap();
        assert!(msg.len() > 0);
    }

    #[test]
    fn test_parse_update_profile_response() {
        init!("indy");

        let result = UpdateProfileDataBuilder::parse_response(UPDATE_PROFILE_RESPONSE.to_vec()).unwrap();
        let expected = UpdateConfigsResponse { msg_type: MessageTypes::build(A2AMessageKinds::ConfigsUpdated, ) };

        assert_eq!(expected, result);
    }
}
