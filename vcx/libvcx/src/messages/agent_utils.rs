use settings;
use utils::constants::*;
use messages::{A2AMessage, A2AMessageKinds, prepare_message_for_agency, parse_response_from_agency};
use messages::message_type::MessageTypes;
use utils::{error, httpclient};
use utils::libindy::{wallet, anoncreds};
use utils::libindy::signus::create_and_store_my_did;

#[derive(Serialize, Deserialize, Debug)]
pub struct Connect {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
    #[serde(rename = "fromDID")]
    pub from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    pub from_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectResponse {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
    #[serde(rename = "withPairwiseDID")]
    pub from_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub from_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUp {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpResponse {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAgent {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAgentResponse {
    #[serde(rename = "@type")]
    pub msg_type: MessageTypes,
    #[serde(rename = "withPairwiseDID")]
    pub from_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub from_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateConnectionMethod {
    #[serde(rename = "@type")]
    pub  msg_type: MessageTypes,
    #[serde(rename = "comMethod")]
    pub com_method: ComMethod,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComMethod {
    id: String,
    #[serde(rename = "type")]
    e_type: i32,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default)]
    protocol_type: settings::ProtocolTypes,
    agency_url: String,
    agency_did: String,
    agency_verkey: String,
    wallet_name: Option<String>,
    wallet_key: String,
    wallet_type: Option<String>,
    agent_seed: Option<String>,
    enterprise_seed: Option<String>,
    wallet_key_derivation: Option<String>,
    name: Option<String>,
    logo: Option<String>,
    path: Option<String>,
}

pub fn connect_register_provision(config: &str) -> Result<String, u32> {
    trace!("connect_register_provision >>> config: {:?}", config);

    trace!("***Registering with agency");
    let my_config: Config = serde_json::from_str(&config).or(Err(error::INVALID_CONFIGURATION.code_num))?;

    let wallet_name = my_config.wallet_name.unwrap_or(settings::DEFAULT_WALLET_NAME.to_string());

    settings::set_config_value(settings::CONFIG_PROTOCOL_TYPE, &my_config.protocol_type.to_string());
    settings::set_config_value(settings::CONFIG_AGENCY_ENDPOINT, &my_config.agency_url);
    settings::set_config_value(settings::CONFIG_WALLET_NAME, &wallet_name);
    settings::set_config_value(settings::CONFIG_AGENCY_DID, &my_config.agency_did);
    settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &my_config.agency_verkey);
    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &my_config.agency_verkey);
    settings::set_config_value(settings::CONFIG_WALLET_KEY, &my_config.wallet_key);

    if let Some(key_derivation) = &my_config.wallet_key_derivation {
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION, key_derivation);
    }
    if let Some(wallet_type) = &my_config.wallet_type {
        settings::set_config_value(settings::CONFIG_WALLET_TYPE, wallet_type);
    }

    wallet::init_wallet(&wallet_name, my_config.wallet_type.as_ref().map(String::as_str))?;
    trace!("initialized wallet");

    anoncreds::libindy_prover_create_master_secret(::settings::DEFAULT_LINK_SECRET_ALIAS).ok(); // If MS is already in wallet then just continue

    let name = my_config.name.unwrap_or(String::from("<CHANGE_ME>"));
    let logo = my_config.logo.unwrap_or(String::from("<CHANGE_ME>"));
    let path = my_config.path.unwrap_or(String::from("<CHANGE_ME>"));

    let (my_did, my_vk) = create_and_store_my_did(my_config.agent_seed.as_ref().map(String::as_str))?;

    let (issuer_did, issuer_vk) = if my_config.enterprise_seed != my_config.agent_seed {
        create_and_store_my_did(my_config.enterprise_seed.as_ref().map(String::as_str))?
    } else {
        (my_did.clone(), my_vk.clone())
    };

    settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
    settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

    /* STEP 1 - CONNECT */
    if settings::test_agency_mode_enabled() {
        httpclient::set_next_u8_response(CONNECTED_RESPONSE.to_vec());
    }

    trace!("Connecting to Agency");
    let message = A2AMessage::Connect(Connect {
        msg_type: MessageTypes::build(A2AMessageKinds::Connect),
        from_did: my_did.to_string(),
        from_vk: my_vk.to_string(),
    });

    let mut response = send_message_to_agency(&message, &my_config.agency_did)?;
    let response: ConnectResponse = ConnectResponse::from_a2a_message(response.remove(0))?;

    let agency_pw_vk = response.from_vk;
    let agency_pw_did = response.from_did;

    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agency_pw_vk);

    /* STEP 2 - REGISTER */
    if settings::test_agency_mode_enabled() {
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
    }

    let message = A2AMessage::SignUp(SignUp {
        msg_type: MessageTypes::build(A2AMessageKinds::SignUp)
    });

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;
    let response: SignUpResponse = SignUpResponse::from_a2a_message(response.remove(0))?;

    /* STEP 3 - CREATE AGENT */
    if settings::test_agency_mode_enabled() {
        httpclient::set_next_u8_response(AGENT_CREATED.to_vec());
    }

    let message = A2AMessage::CreateAgent(CreateAgent {
        msg_type: MessageTypes::build(A2AMessageKinds::CreateAgent)
    });

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;
    let response: CreateAgentResponse = CreateAgentResponse::from_a2a_message(response.remove(0))?;

    let agent_did = response.from_did;
    let agent_vk = response.from_vk;

    let mut final_config = json!({
        "wallet_key": &my_config.wallet_key,
        "wallet_name": wallet_name,
        "agency_endpoint": &my_config.agency_url,
        "agency_did": &my_config.agency_did,
        "agency_verkey": &my_config.agency_verkey,
        "sdk_to_remote_did": my_did,
        "sdk_to_remote_verkey": my_vk,
        "institution_did": issuer_did,
        "institution_verkey": issuer_vk,
        "remote_to_sdk_did": agent_did,
        "remote_to_sdk_verkey": agent_vk,
        "institution_name": name,
        "institution_logo_url": logo,
        "genesis_path": path,
    });
    if let Some(key_derivation) = &my_config.wallet_key_derivation {
        final_config["wallet_key_derivation"] = json!(key_derivation);
    }

    wallet::close_wallet()?;

    Ok(final_config.to_string())
}

pub fn update_agent_info(id: &str, value: &str) -> Result<(), u32> {
    trace!("update_agent_info >>> id: {}, value: {}", id, value);

    let message = A2AMessage::UpdateConnectionMethod(UpdateConnectionMethod {
        msg_type: MessageTypes::build(A2AMessageKinds::UpdateConMethod),
        com_method: ComMethod {
            id: id.to_string(),
            e_type: 1,
            value: value.to_string()
        },
    });

    if settings::test_agency_mode_enabled() {
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
    }

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

    send_message_to_agency(&message, &to_did)?;

    Ok(())
}

pub fn send_message_to_agency(message: &A2AMessage, did: &str) -> Result<Vec<A2AMessage>, u32> {
    let data = prepare_message_for_agency(message, did)?;
    let response = httpclient::post_u8(&data).or(Err(error::INVALID_HTTP_RESPONSE.code_num))?;
    parse_response_from_agency(&response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_register_provision() {
        init!("true");

        let agency_did = "Ab8TvZa3Q19VNkQVzAWVL7";
        let agency_vk = "5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf";
        let host = "http://www.whocares.org";
        let wallet_key = "test_key";
        let config = json!({
            "agency_url": host.to_string(),
            "agency_did": agency_did.to_string(),
            "agency_verkey": agency_vk.to_string(),
            "wallet_key": wallet_key.to_string(),
        });

        let result = connect_register_provision(&config.to_string()).unwrap();
        assert!(result.len() > 0);
    }

    #[ignore]
    #[test]
    fn test_real_connect_register_provision() {
        settings::set_defaults();

        let agency_did = "VsKV7grR1BUE29mG2Fm2kX";
        let agency_vk = "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR";
        let host = "http://localhost:8080";
        let wallet_key = "test_key";
        let config = json!({
            "agency_url": host.to_string(),
            "agency_did": agency_did.to_string(),
            "agency_verkey": agency_vk.to_string(),
            "wallet_key": wallet_key.to_string(),
        });

        let result = connect_register_provision(&config.to_string()).unwrap();
        assert!(result.len() > 0);
    }

    #[test]
    fn test_update_agent_info() {
        init!("true");
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        match update_agent_info("123", "value") {
            Ok(_) => assert_eq!(0, 0),
            Err(x) => assert_eq!(x, 0), // should fail here
        };
    }
}
