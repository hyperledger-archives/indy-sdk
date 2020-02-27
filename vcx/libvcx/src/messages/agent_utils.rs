use settings;
use messages::{A2AMessage, A2AMessageV1, A2AMessageV2, A2AMessageKinds, prepare_message_for_agency, parse_response_from_agency};
use messages::message_type::MessageTypes;
use utils::{error, httpclient, constants};
use utils::libindy::{wallet, anoncreds};
use utils::libindy::signus::create_and_store_my_did;
use error::prelude::*;
use utils::httpclient::AgencyMock;

#[derive(Serialize, Deserialize, Debug)]
pub struct Connect {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "fromDID")]
    from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    from_vk: String,
}

impl Connect {
    fn build(from_did: &str, from_vk: &str) -> Connect {
        Connect {
            msg_type: MessageTypes::build(A2AMessageKinds::Connect),
            from_did: from_did.to_string(),
            from_vk: from_vk.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "withPairwiseDID")]
    from_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    from_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUp {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

impl SignUp {
    fn build() -> SignUp {
        SignUp {
            msg_type: MessageTypes::build(A2AMessageKinds::SignUp),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAgent {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
}

impl CreateAgent {
    fn build() -> CreateAgent {
        CreateAgent {
            msg_type: MessageTypes::build(A2AMessageKinds::CreateAgent),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAgentResponse {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "withPairwiseDID")]
    from_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    from_vk: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComMethodUpdated {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateComMethod {
    #[serde(rename = "@type")]
    msg_type: MessageTypes,
    #[serde(rename = "comMethod")]
    com_method: ComMethod,
}

impl UpdateComMethod {
    fn build(com_method: ComMethod) -> UpdateComMethod {
        UpdateComMethod {
            msg_type: MessageTypes::build(A2AMessageKinds::UpdateComMethod),
            com_method,
        }
    }
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
    storage_config: Option<String>,
    storage_credentials: Option<String>,
    pool_config: Option<String>,
    did_method: Option<String>,
    communication_method: Option<String>,
    webhook_url: Option<String>,
    use_latest_protocols: Option<String>,
}


pub fn connect_register_provision(config: &str) -> VcxResult<String> {
    trace!("connect_register_provision >>> config: {:?}", config);

    trace!("***Registering with agency");
    let my_config: Config = ::serde_json::from_str(&config)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidConfiguration, format!("Cannot parse config: {}", err)))?;

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
    if let Some(_storage_config) = &my_config.storage_config {
        settings::set_config_value(settings::CONFIG_WALLET_STORAGE_CONFIG, _storage_config);
    }
    if let Some(_storage_credentials) = &my_config.storage_credentials {
        settings::set_config_value(settings::CONFIG_WALLET_STORAGE_CREDS, _storage_credentials);
    }
    if let Some(pool_config) = &my_config.pool_config {
        settings::set_config_value(settings::CONFIG_POOL_CONFIG, pool_config);
    }
    if let Some(did_method) = &my_config.did_method {
        settings::set_config_value(settings::CONFIG_DID_METHOD, did_method);
    }
    if let Some(communication_method) = &my_config.communication_method {
        settings::set_config_value(settings::COMMUNICATION_METHOD, communication_method);
    }
    if let Some(webhook_url) = &my_config.webhook_url {
        settings::set_config_value(settings::CONFIG_WEBHOOK_URL, webhook_url);
    }

    wallet::init_wallet(&wallet_name, my_config.wallet_type.as_ref().map(String::as_str),
                        my_config.storage_config.as_ref().map(String::as_str),
                        my_config.storage_credentials.as_ref().map(String::as_str))?;
    trace!("initialized wallet");

    anoncreds::libindy_prover_create_master_secret(::settings::DEFAULT_LINK_SECRET_ALIAS).ok(); // If MS is already in wallet then just continue

    let name = my_config.name.unwrap_or(String::from("<CHANGE_ME>"));
    let logo = my_config.logo.unwrap_or(String::from("<CHANGE_ME>"));
    let path = my_config.path.unwrap_or(String::from("<CHANGE_ME>"));

    let method_name = my_config.did_method.as_ref().map(String::as_str);

    let (my_did, my_vk) = create_and_store_my_did(my_config.agent_seed.as_ref().map(String::as_str), method_name)?;

    let (issuer_did, issuer_vk) = if my_config.enterprise_seed != my_config.agent_seed {
        create_and_store_my_did(my_config.enterprise_seed.as_ref().map(String::as_str), method_name)?
    } else {
        (my_did.clone(), my_vk.clone())
    };

    settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
    settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY, &my_vk);

    trace!("Connecting to Agency");

    let (agent_did, agent_vk) = match my_config.protocol_type {
        settings::ProtocolTypes::V1 => onboarding_v1(&my_did, &my_vk, &my_config.agency_did)?,
        settings::ProtocolTypes::V2 => onboarding_v2(&my_did, &my_vk, &my_config.agency_did)?,
    };

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
        "protocol_type": &my_config.protocol_type,
    });
    if let Some(key_derivation) = &my_config.wallet_key_derivation {
        final_config["wallet_key_derivation"] = json!(key_derivation);
    }
    if let Some(wallet_type) = &my_config.wallet_type {
        final_config["wallet_type"] = json!(wallet_type);
    }
    if let Some(_storage_config) = &my_config.storage_config {
        final_config["storage_config"] = json!(_storage_config);
    }
    if let Some(_storage_credentials) = &my_config.storage_credentials {
        final_config["storage_credentials"] = json!(_storage_credentials);
    }
    if let Some(_pool_config) = &my_config.pool_config {
        final_config["pool_config"] = json!(_pool_config);
    }
    if let Some(_communication_method) = &my_config.communication_method {
        final_config["communication_method"] = json!(_communication_method);
    }
    if let Some(_webhook_url) = &my_config.webhook_url {
        final_config["webhook_url"] = json!(_webhook_url);
    }
    if let Some(_use_latest_protocols) = &my_config.use_latest_protocols {
        final_config["use_latest_protocols"] = json!(_use_latest_protocols);
    }

    wallet::close_wallet()?;

    Ok(final_config.to_string())
}

fn onboarding_v1(my_did: &str, my_vk: &str, agency_did: &str) -> VcxResult<(String, String)> {
    /* STEP 1 - CONNECT */
    AgencyMock::set_next_response(constants::CONNECTED_RESPONSE.to_vec());

    let message = A2AMessage::Version1(
        A2AMessageV1::Connect(Connect::build(my_did, my_vk))
    );

    let mut response = send_message_to_agency(&message, agency_did)?;

    let ConnectResponse { from_vk: agency_pw_vk, from_did: agency_pw_did, .. } =
        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::ConnectResponse(resp)) => resp,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of ConnectResponse"))
        };

    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agency_pw_vk);

    /* STEP 2 - REGISTER */
    AgencyMock::set_next_response(constants::REGISTER_RESPONSE.to_vec());

    let message = A2AMessage::Version1(
        A2AMessageV1::SignUp(SignUp::build())
    );

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;

    let _response: SignUpResponse =
        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::SignUpResponse(resp)) => resp,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of SignUpResponse"))
        };

    /* STEP 3 - CREATE AGENT */
    AgencyMock::set_next_response(constants::AGENT_CREATED.to_vec());

    let message = A2AMessage::Version1(
        A2AMessageV1::CreateAgent(CreateAgent::build())
    );

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;

    let response: CreateAgentResponse =
        match response.remove(0) {
            A2AMessage::Version1(A2AMessageV1::CreateAgentResponse(resp)) => resp,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of CreateAgentResponse"))
        };

    Ok((response.from_did, response.from_vk))
}

// it will be changed next
fn onboarding_v2(my_did: &str, my_vk: &str, agency_did: &str) -> VcxResult<(String, String)> {
    /* STEP 1 - CONNECT */
    let message = A2AMessage::Version2(
        A2AMessageV2::Connect(Connect::build(my_did, my_vk))
    );

    let mut response = send_message_to_agency(&message, agency_did)?;

    let ConnectResponse { from_vk: agency_pw_vk, from_did: agency_pw_did, .. } =
        match response.remove(0) {
            A2AMessage::Version2(A2AMessageV2::ConnectResponse(resp)) => resp,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of ConnectResponse"))
        };

    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &agency_pw_vk);

    /* STEP 2 - REGISTER */
    let message = A2AMessage::Version2(
        A2AMessageV2::SignUp(SignUp::build())
    );

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;

    let _response: SignUpResponse =
        match response.remove(0) {
            A2AMessage::Version2(A2AMessageV2::SignUpResponse(resp)) => resp,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of SignUpResponse"))
        };

    /* STEP 3 - CREATE AGENT */
    let message = A2AMessage::Version2(
        A2AMessageV2::CreateAgent(CreateAgent::build())
    );

    let mut response = send_message_to_agency(&message, &agency_pw_did)?;

    let response: CreateAgentResponse =
        match response.remove(0) {
            A2AMessage::Version2(A2AMessageV2::CreateAgentResponse(resp)) => resp,
            _ => return Err(VcxError::from_msg(VcxErrorKind::InvalidHttpResponse, "Message does not match any variant of CreateAgentResponse"))
        };

    Ok((response.from_did, response.from_vk))
}

pub fn update_agent_info(id: &str, value: &str) -> VcxResult<()> {
    trace!("update_agent_info >>> id: {}, value: {}", id, value);

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

    let com_method = ComMethod {
        id: id.to_string(),
        e_type: 1,
        value: value.to_string(),
    };

    match settings::get_protocol_type() {
        settings::ProtocolTypes::V1 => {
            update_agent_info_v1(&to_did, com_method)
        }
        settings::ProtocolTypes::V2 => {
            update_agent_info_v2(&to_did, com_method)
        }
    }
}

fn update_agent_info_v1(to_did: &str, com_method: ComMethod) -> VcxResult<()> {
    AgencyMock::set_next_response(constants::REGISTER_RESPONSE.to_vec());

    let message = A2AMessage::Version1(
        A2AMessageV1::UpdateComMethod(UpdateComMethod::build(com_method))
    );
    send_message_to_agency(&message, to_did)?;
    Ok(())
}

fn update_agent_info_v2(to_did: &str, com_method: ComMethod) -> VcxResult<()> {
    let message = A2AMessage::Version2(
        A2AMessageV2::UpdateComMethod(UpdateComMethod::build(com_method))
    );
    send_message_to_agency(&message, to_did)?;
    Ok(())
}

pub fn update_agent_webhook(webhook_url: &str) -> VcxResult<()> {
    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;

    ::messages::update_data()
        .to(&to_did[..])?
        .webhook_url(&Some(String::from(webhook_url)))?
        .send_secure()
        .map_err(|err| err.extend("Cannot update webhook"))?;

    Ok(())
}

fn send_message_to_agency(message: &A2AMessage, did: &str) -> VcxResult<Vec<A2AMessage>> {
    let data = prepare_message_for_agency(message, &did, &settings::get_protocol_type())?;

    let response = httpclient::post_u8(&data)
        .map_err(|err| err.map(VcxErrorKind::InvalidHttpResponse, error::INVALID_HTTP_RESPONSE.message))?;

    parse_response_from_agency(&response, &settings::get_protocol_type())
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::devsetup::*;

    #[test]
    fn test_connect_register_provision() {
        let _setup = SetupMocks::init();

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

        let expected = json!({
            "agency_did":"Ab8TvZa3Q19VNkQVzAWVL7",
            "agency_endpoint":"http://www.whocares.org",
            "agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf",
            "genesis_path":"<CHANGE_ME>",
            "institution_did":"FhrSrYtQcw3p9xwf7NYemf",
            "institution_logo_url":"<CHANGE_ME>",
            "institution_name":"<CHANGE_ME>",
            "institution_verkey":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "protocol_type":"1.0",
            "remote_to_sdk_did":"A4a69qafqZHPLPPu5JFQrc",
            "remote_to_sdk_verkey":"5wTKXrdfUiTQ7f3sZJzvHpcS7XHHxiBkFtPCsynZtv4k",
            "sdk_to_remote_did":"FhrSrYtQcw3p9xwf7NYemf",
            "sdk_to_remote_verkey":"91qMFrZjXDoi2Vc8Mm14Ys112tEZdDegBZZoembFEATE",
            "wallet_key":"test_key",
            "wallet_name":"LIBVCX_SDK_WALLET"
        });

        assert_eq!(expected, ::serde_json::from_str::<serde_json::Value>(&result).unwrap());
    }

    #[ignore]
    #[test]
    fn test_real_connect_register_provision() {
        let _setup = SetupDefaults::init();

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
        let _setup = SetupMocks::init();

        update_agent_info("123", "value").unwrap();
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_update_agent_info_real() {
        let _setup = SetupLibraryAgencyV1::init();

        ::utils::devsetup::set_consumer();
        update_agent_info("7b7f97f2", "FCM:Value").unwrap();
    }
}
