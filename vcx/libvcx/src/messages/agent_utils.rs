extern crate serde;
extern crate rmp_serde;
extern crate libc;

use self::rmp_serde::encode;
use self::rmp_serde::Deserializer;
use serde::Deserialize;
use serde_json;
use settings;
use utils::constants::*;
use utils::error;
use utils::libindy::wallet;
use utils::libindy::signus::create_and_store_my_did;
use utils::httpclient;
use messages::{Bundled, MsgType, bundle_for_agency, unbundle_from_agency};


#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ConnectMsg {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "fromDID")]
    from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    from_vk: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ConnectResponseMsg {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "withPairwiseDID")]
    from_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    from_vk: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct GenericMsg {
    #[serde(rename = "@type")]
    msg_type: MsgType,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct RegisterResponse {
    #[serde(rename = "@type")]
    msg_type: MsgType,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct UpdateAgentMsg {
    #[serde(rename = "@type")]
    msg_type: MsgType,
    #[serde(rename = "comMethod")]
    com_method: ComMethod,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
struct ComMethod {
    id: String,
    #[serde(rename = "type")]
    e_type: i32,
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
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
}

pub fn connect_register_provision(config: &str) -> Result<String,u32> {
    trace!("connect_register_provision >>> config: {:?}", config);

    trace!("***Registering with agency");
    let my_config: Config = serde_json::from_str(&config).or(Err(error::INVALID_CONFIGURATION.code_num))?;
    let (wallet_name_string, wallet_name) = match my_config.wallet_name {
        Some(x) => (format!("\"wallet_name\":\"{}\",", x), x),
        None => ("".to_string(), settings::DEFAULT_WALLET_NAME.to_string()),
    };

    settings::set_config_value(settings::CONFIG_AGENCY_ENDPOINT, &my_config.agency_url);
    settings::set_config_value(settings::CONFIG_WALLET_NAME, &wallet_name);
    settings::set_config_value(settings::CONFIG_AGENCY_DID, &my_config.agency_did);
    settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, &my_config.agency_verkey);
    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, &my_config.agency_verkey);
    settings::set_config_value(settings::CONFIG_WALLET_KEY, &my_config.wallet_key);
    if let Some(_key_derivation) = &my_config.wallet_key_derivation {
        settings::set_config_value(settings::CONFIG_WALLET_KEY_DERIVATION, _key_derivation);
    }
    if let Some(_wallet_type) = &my_config.wallet_type {
        settings::set_config_value(settings::CONFIG_WALLET_TYPE, _wallet_type);
    }
    if let Some(_storage_config) = &my_config.storage_config {
        settings::set_config_value(settings::CONFIG_WALLET_STORAGE_CONFIG, _storage_config);
    }
    if let Some(_storage_credentials) = &my_config.storage_credentials {
        settings::set_config_value(settings::CONFIG_WALLET_STORAGE_CREDS, _storage_credentials);
    }

    wallet::init_wallet(&wallet_name, my_config.wallet_type.as_ref().map(String::as_str), 
                        my_config.storage_config.as_ref().map(String::as_str), my_config.storage_credentials.as_ref().map(String::as_str))?;
    trace!("initialized wallet");

    match ::utils::libindy::anoncreds::libindy_prover_create_master_secret(::settings::DEFAULT_LINK_SECRET_ALIAS) {
        Ok(_) => (),
        Err(_) => (),  // If MS is already in wallet then just continue
    };

    let seed = my_config.agent_seed.as_ref().unwrap_or(&String::new()).to_string();
    let name = my_config.name.as_ref().unwrap_or(&String::from("<CHANGE_ME>")).to_string();
    let logo = my_config.logo.as_ref().unwrap_or(&String::from("<CHANGE_ME>")).to_string();
    let path = my_config.path.as_ref().unwrap_or(&String::from("<CHANGE_ME>")).to_string();

    let seed_opt = if seed.len() > 0 {Some(seed.as_ref())} else {None};
    let (my_did, my_vk) = create_and_store_my_did(seed_opt)?;

    let issuer_did;
    let issuer_vk;
    let issuer_seed = my_config.enterprise_seed.as_ref().unwrap_or(&String::new()).to_string();
    if issuer_seed != seed {
        let issuer_seed_opt = if issuer_seed.len() > 0 { Some(issuer_seed.as_ref()) } else { None };
        let (did1, vk1) = create_and_store_my_did(issuer_seed_opt)?;
        issuer_did = did1;
        issuer_vk = vk1;
    } else {
        issuer_did = my_did.clone();
        issuer_vk = my_vk.clone();
    }

    settings::set_config_value(settings::CONFIG_INSTITUTION_DID,&my_did);
    settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY,&my_vk);

    if settings::test_agency_mode_enabled() {
        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());
    }

    /* STEP 1 - CONNECT */

    trace!("Connecting to Agency");
    let payload = ConnectMsg {
        msg_type: MsgType { name: "CONNECT".to_string(), ver: "1.0".to_string(), },
        from_did: my_did.to_string(),
        from_vk: my_vk.to_string(),
    };
    let data = Bundled::create(
        encode::to_vec_named(&payload).or(Err(error::UNKNOWN_ERROR.code_num))?
    ).encode()?;
    let data = bundle_for_agency(data, &my_config.agency_did)?;
    let data = unbundle_from_agency(
        httpclient::post_u8(&data).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num)?
    )?;

    trace!("deserializing connect response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: ConnectResponseMsg = Deserialize::deserialize(&mut de).map_err(|ec| {error::INVALID_OPTION.code_num})?;
    //self.my_vk = Some(connection::get_pw_verkey(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);
    let agency_pw_vk = response.from_vk.to_owned();
    let agency_pw_did = response.from_did.to_owned();

    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY,&agency_pw_vk);

    /* STEP 2 - REGISTER */

    let payload = GenericMsg {
        msg_type: MsgType { name: "SIGNUP".to_string(), ver: "1.0".to_string(), },
    };

    let data = encode::to_vec_named(&payload)
        .or(Err(error::UNKNOWN_ERROR.code_num))?;
    let data = Bundled::create(data)
        .encode()
        .or(Err(error::UNKNOWN_ERROR.code_num))?;
    let data = bundle_for_agency(data, &agency_pw_did)?;
    let data = unbundle_from_agency(
        httpclient::post_u8(&data).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num)?
    )?;

    trace!("deserializing register response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: RegisterResponse = Deserialize::deserialize(&mut de)
        .map_err(|e|error::INVALID_HTTP_RESPONSE.code_num)?;

    /* STEP 3 - CREATE AGENT */
    let payload = GenericMsg {
        msg_type: MsgType { name: "CREATE_AGENT".to_string(), ver: "1.0".to_string(), },
    };

    let data = encode::to_vec_named(&payload)
        .or(Err(error::UNKNOWN_ERROR.code_num))?;
    let data = Bundled::create(data).encode()
        .or(Err(error::UNKNOWN_ERROR.code_num))?;
    let data = bundle_for_agency(data, &agency_pw_did)?;
    let data = unbundle_from_agency(
        httpclient::post_u8(&data).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num)?
    )?;

    trace!("deserializing provision response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: ConnectResponseMsg = Deserialize::deserialize(&mut de)
        .map_err(|e|error::INVALID_HTTP_RESPONSE.code_num)?;
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
    if let Some(_key_derivation) = &my_config.wallet_key_derivation {
        final_config["wallet_key_derivation"] = json!(_key_derivation);
    }
    if let Some(_wallet_type) = &my_config.wallet_type {
        final_config["wallet_type"] = json!(_wallet_type);
    }
    if let Some(_storage_config) = &my_config.storage_config {
        final_config["storage_config"] = json!(_storage_config);
    }
    if let Some(_storage_credentials) = &my_config.storage_credentials {
        final_config["storage_credentials"] = json!(_storage_credentials);
    }

    wallet::close_wallet()?;

    Ok(final_config.to_string())
}

pub fn update_agent_info(id: &str, value: &str) -> Result<(), u32> {
    trace!("update_agent_info >>> id: {}, value: {}", id, value);

    let new_config = UpdateAgentMsg {
        msg_type: MsgType { name: "UPDATE_COM_METHOD".to_string(), ver: "1.0".to_string(), },
        com_method: ComMethod { id: id.to_string(), e_type: 1, value: value.to_string(), },
    };

    if settings::test_agency_mode_enabled() {
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
    }

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
    let endpoint = settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)?;

    let data = encode::to_vec_named(&new_config)
        .or(Err(error::UNKNOWN_ERROR.code_num))?;

    let data = Bundled::create(data).encode()
        .or(Err(error::UNKNOWN_ERROR.code_num))?;

    let data = bundle_for_agency(data, &to_did)?;

    let data = unbundle_from_agency(
        httpclient::post_u8(&data) .map_err(|e|error::INVALID_HTTP_RESPONSE.code_num)?
    )?;

    Ok(())
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
        println!("result: {}", result);
    }

    #[test]
    fn test_update_agent_info() {
        init!("true");
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        match update_agent_info("123", "value") {
            Ok(_) => assert_eq!(0,0),
            Err(x) => assert_eq!(x, 0), // should fail here
        };

    }
}
