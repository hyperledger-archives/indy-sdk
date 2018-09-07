extern crate serde;
extern crate rmp_serde;
extern crate libc;

use self::rmp_serde::encode;
use self::rmp_serde::Deserializer;
use serde::Deserialize;
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

pub fn connect_register_provision(endpoint: &str,
                                  agency_did: &str,
                                  agency_vk: &str,
                                  wallet_name: Option<String>,
                                  seed: Option<String>,
                                  issuer_seed: Option<String>,
                                  wallet_key: &str,
                                  name: Option<String>,
                                  logo: Option<String>,
                                  path: Option<String>) -> Result<String,u32> {

    trace!("***Registering with agency");
    let (wallet_name_string, wallet_name) = match wallet_name {
        Some(x) => (format!("\"wallet_name\":\"{}\",", x), x),
        None => ("".to_string(), settings::DEFAULT_WALLET_NAME.to_string()),
    };

    settings::set_config_value(settings::CONFIG_AGENCY_ENDPOINT, endpoint);
    settings::set_config_value(settings::CONFIG_WALLET_NAME, &wallet_name);
    settings::set_config_value(settings::CONFIG_AGENCY_DID, agency_did);
    settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, agency_vk);
    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, agency_vk);
    settings::set_config_value(settings::CONFIG_WALLET_KEY, &wallet_key);

    wallet::init_wallet(&wallet_name)?;
    trace!("initialized wallet");

    match ::utils::libindy::anoncreds::libindy_prover_create_master_secret(::settings::DEFAULT_LINK_SECRET_ALIAS) {
        Ok(_) => (),
        Err(_) => (),  // If MS is already in wallet then just continue
    };

    let seed = match seed {
        Some(x) => x,
        None => "".to_string(),
    };

    let name = match name {
        Some(x) => x,
        None => "<CHANGE_ME>".to_string(),
    };

    let logo = match logo {
        Some(x) => x,
        None => "<CHANGE_ME>".to_string(),
    };

    let path = match path {
        Some(x) => x,
        None => "<CHANGE_ME>".to_string(),
    };

    let seed_opt = if seed.len() > 0 {Some(seed.as_ref())} else {None};
    let (my_did, my_vk) = create_and_store_my_did(seed_opt)?;

    let issuer_seed = match issuer_seed {
        Some(x) => x,
        None => "".to_string(),
    };

    let issuer_seed_opt = if issuer_seed.len() > 0 {Some(issuer_seed.as_ref())} else {None};
    let (issuer_did, issuer_vk) = create_and_store_my_did(issuer_seed_opt)?;

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
    let data = Bundled::create(encode::to_vec_named(&payload).unwrap()).encode()?;
    let data = bundle_for_agency(data, &agency_did)?;
    let data = unbundle_from_agency(httpclient::post_u8(&data).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num).unwrap())?;

    trace!("deserializing connect response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: ConnectResponseMsg = Deserialize::deserialize(&mut de).map_err(|ec| {error::INVALID_OPTION.code_num}).unwrap();
    //self.my_vk = Some(connection::get_pw_verkey(connection_handle).map_err(|ec| CredentialError::CommonError(ec.to_error_code()))?);
    let agency_pw_vk = response.from_vk.to_owned();
    let agency_pw_did = response.from_did.to_owned();

    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY,&agency_pw_vk);

    /* STEP 2 - REGISTER */

    let payload = GenericMsg {
        msg_type: MsgType { name: "SIGNUP".to_string(), ver: "1.0".to_string(), },
    };

    let data = encode::to_vec_named(&payload).unwrap();
    let data = Bundled::create(data).encode().unwrap();
    let data = bundle_for_agency(data, &agency_pw_did)?;
    let data = unbundle_from_agency(httpclient::post_u8(&data).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num).unwrap())?;

    trace!("deserializing register response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: RegisterResponse = Deserialize::deserialize(&mut de).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num).unwrap();

    /* STEP 3 - CREATE AGENT */
    let payload = GenericMsg {
        msg_type: MsgType { name: "CREATE_AGENT".to_string(), ver: "1.0".to_string(), },
    };

    let data = encode::to_vec_named(&payload).unwrap();
    let data = Bundled::create(data).encode().unwrap();
    let data = bundle_for_agency(data, &agency_pw_did)?;
    let data = unbundle_from_agency(httpclient::post_u8(&data).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num).unwrap())?;

    trace!("deserializing provision response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: ConnectResponseMsg = Deserialize::deserialize(&mut de).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num).unwrap();
    let agent_did = response.from_did;
    let agent_vk = response.from_vk;

    let final_config = format!("{{\
    \"wallet_key\":\"{}\",\
    {}\
    \"agency_endpoint\":\"{}\",\
    \"agency_did\":\"{}\",\
    \"agency_verkey\":\"{}\",\
    \"sdk_to_remote_did\":\"{}\",\
    \"sdk_to_remote_verkey\":\"{}\",\
    \"institution_did\":\"{}\",\
    \"institution_verkey\":\"{}\",\
    \"remote_to_sdk_did\":\"{}\",\
    \"remote_to_sdk_verkey\":\"{}\",\
    \"institution_name\":\"{}\",\
    \"institution_logo_url\":\"{}\",\
    \"genesis_path\":\"{}\"\
    }}",
        wallet_key,
        wallet_name_string,
        endpoint,
        agency_did,
        agency_vk,
        my_did,
        my_vk,
        issuer_did,
        issuer_vk,
        agent_did,
        agent_vk,
        name,
        logo,
        path);

    wallet::close_wallet()?;

    Ok(final_config.to_owned())
}

pub fn update_agent_info(id: &str, value: &str) -> Result<(), u32> {
    let new_config = UpdateAgentMsg {
        msg_type: MsgType { name: "UPDATE_COM_METHOD".to_string(), ver: "1.0".to_string(), },
        com_method: ComMethod { id: id.to_string(), e_type: 1, value: value.to_string(), },
    };

    if settings::test_agency_mode_enabled() {
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
    }

    let to_did = settings::get_config_value(settings::CONFIG_REMOTE_TO_SDK_DID)?;
    let endpoint = settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)?;

    let data = encode::to_vec_named(&new_config).unwrap();
    let data = Bundled::create(data).encode().unwrap();
    let data = bundle_for_agency(data, &to_did)?;
    let data = unbundle_from_agency(httpclient::post_u8(&data).map_err(|e|error::INVALID_HTTP_RESPONSE.code_num).unwrap())?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use utils::constants::{DEMO_ISSUER_PW_SEED};

    #[test]
    fn test_connect_register_provision() {
        init!("true");

        let agency_did = "Ab8TvZa3Q19VNkQVzAWVL7";
        let agency_vk = "5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf";
        let host = "http://www.whocares.org";
        let wallet_key = "test_key";

        let result = connect_register_provision(&host,
                                                &agency_did,
                                                &agency_vk,
                                                None,
                                                None,
                                                None,
                                                wallet_key,
                                                None,
                                                None,
                                                None).unwrap();
        assert!(result.len() > 0);
    }

    #[ignore]
    #[test]
    fn test_real_connect_register_provision() {
        settings::set_defaults();

        let config_path = "/tmp/test_real_agency_connect.json";
        let agency_did = "YRuVCckY6vfZfX9kcQZe3u";
        let agency_vk = "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v";
        let host = "https://enym-eagency.pdev.evernym.com";

        let result = connect_register_provision(&host,
                                                &agency_did,
                                                &agency_vk,
                                                None,
                                                None,
                                                Some(DEMO_ISSUER_PW_SEED.to_string()),
                                                settings::DEFAULT_WALLET_KEY,
                                                None,
                                                None,
                                                None).unwrap();
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
