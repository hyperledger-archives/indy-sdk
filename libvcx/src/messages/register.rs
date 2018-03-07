extern crate serde;
extern crate rmp_serde;
extern crate libc;

use self::rmp_serde::encode;
use self::rmp_serde::Deserializer;
use serde::Deserialize;
use settings;
use utils::libindy::wallet;
use utils::libindy::signus::SignusUtils;
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

pub fn connect_register_provision(endpoint: &str,
                                  agency_did: &str,
                                  agency_vk: &str,
                                  wallet_name: Option<String>,
                                  seed: Option<String>,
                                  issuer_seed: Option<String>,
                                  wallet_key: Option<String>) -> Result<String,u32> {

    ::utils::logger::LoggerUtils::init();
    settings::set_defaults();

    let (wallet_name_string, wallet_name) = match wallet_name {
        Some(x) => (format!("\"wallet_name\":\"{}\",", x), x),
        None => ("".to_string(), settings::DEFAULT_WALLET_NAME.to_string()),
    };

    settings::set_config_value(settings::CONFIG_AGENCY_ENDPOINT, endpoint);
    settings::set_config_value(settings::CONFIG_WALLET_NAME, &wallet_name);
    settings::set_config_value(settings::CONFIG_AGENCY_DID, agency_did);
    settings::set_config_value(settings::CONFIG_AGENCY_VERKEY, agency_vk);
    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY, agency_vk);

    let mut wallet_key_string = String::new();
    match wallet_key {
        Some(x) => {
            wallet_key_string = format!("\"wallet_key\":\"{}\",", x);
            settings::set_config_value(settings::CONFIG_WALLET_KEY, &x)
        },
        None => (),
    };

    wallet::init_wallet(&wallet_name).unwrap();

    let seed = match seed {
        Some(x) => x,
        None => "".to_string(),
    };

    let seed_opt = if seed.len() > 0 {Some(seed.as_ref())} else {None};
    let (my_did, my_vk) = SignusUtils::create_and_store_my_did(wallet::get_wallet_handle(), seed_opt).unwrap();

    let issuer_seed = match issuer_seed {
        Some(x) => x,
        None => "".to_string(),
    };

    let issuer_seed_opt = if issuer_seed.len() > 0 {Some(issuer_seed.as_ref())} else {None};
    let (issuer_did, issuer_vk) = SignusUtils::create_and_store_my_did(wallet::get_wallet_handle(), issuer_seed_opt).unwrap();

    settings::set_config_value(settings::CONFIG_INSTITUTION_DID,&my_did);
    settings::set_config_value(settings::CONFIG_SDK_TO_REMOTE_VERKEY,&my_vk);

    /* STEP 1 - CONNECT */

    let url = format!("{}/agency/msg", endpoint);

    let payload = ConnectMsg {
        msg_type: MsgType { name: "CONNECT".to_string(), ver: "1.0".to_string(), },
        from_did: my_did.to_string(),
        from_vk: my_vk.to_string(),
    };
    let data = Bundled::create(encode::to_vec_named(&payload).unwrap()).encode().unwrap();
    let data = bundle_for_agency(data, &agency_did).unwrap();
    let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();

    trace!("deserializing connect response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: ConnectResponseMsg = Deserialize::deserialize(&mut de).unwrap();
    let agency_pw_vk = response.from_vk.to_owned();
    let agency_pw_did = response.from_did.to_owned();

    settings::set_config_value(settings::CONFIG_REMOTE_TO_SDK_VERKEY,&agency_pw_vk);

    /* STEP 2 - REGISTER */

    let payload = GenericMsg {
        msg_type: MsgType { name: "SIGNUP".to_string(), ver: "1.0".to_string(), },
    };

    let data = encode::to_vec_named(&payload).unwrap();
    let data = Bundled::create(data).encode().unwrap();
    let data = bundle_for_agency(data, &agency_pw_did).unwrap();
    let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();

    trace!("deserializing register response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: RegisterResponse = Deserialize::deserialize(&mut de).unwrap();

    /* STEP 3 - CREATE AGENT */
    let payload = GenericMsg {
        msg_type: MsgType { name: "CREATE_AGENT".to_string(), ver: "1.0".to_string(), },
    };

    let data = encode::to_vec_named(&payload).unwrap();
    let data = Bundled::create(data).encode().unwrap();
    let data = bundle_for_agency(data, &agency_pw_did).unwrap();
    let data = unbundle_from_agency(httpclient::post_u8(&data,&url).unwrap()).unwrap();

    trace!("deserializing provision response: {:?}", data);
    let mut de = Deserializer::new(&data[0][..]);
    let response: ConnectResponseMsg = Deserialize::deserialize(&mut de).unwrap();
    let agent_did = response.from_did;
    let agent_vk = response.from_vk;

    let final_config = format!("{{\
    {}\
    {}\
    \"agency_endpoint\":\"{}\",\
    \"agency_did\":\"{}\",\
    \"agency_verkey\":\"{}\",\
    \"sdk_to_remote_did\":\"{}\",\
    \"sdk_to_remote_verkey\":\"{}\",\
    \"institution_did\":\"{}\",\
    \"enterprise_verkey\":\"{}\",\
    \"remote_to_sdk_did\":\"{}\",\
    \"remote_to_sdk_verkey\":\"{}\",\
    \"institution_name\":\"<CHANGE_ME>\",\
    \"institution_logo_url\":\"<CHANGE_ME>\",\
    \"genesis_path\":\"<CHANGE_ME>\"\
    }}",
        wallet_key_string,
        wallet_name_string,
        endpoint,
        agency_did,
        agency_vk,
        my_did,
        my_vk,
        issuer_did,
        issuer_vk,
        agent_did,
        agent_vk);

    Ok(final_config.to_owned())
}


#[cfg(test)]
mod tests {

    use super::*;
    use utils::constants::{DEMO_ISSUER_PW_SEED, REGISTER_RESPONSE, PROVISION_RESPONSE};

    #[test]
    fn test_connect_register_provision() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let agency_did = "Ab8TvZa3Q19VNkQVzAWVL7";
        let agency_vk = "5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf";
        let host = "http://www.whocares.org";
        let wallet_key = Some("test_key".to_string());

        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());

        let result = connect_register_provision(&host, &agency_did, &agency_vk, None, wallet_key, None, None).unwrap();
        assert!(result.len() > 0);
        println!("result: {}", result);

        wallet::delete_wallet("test_connect_register_provision").unwrap();
    }

    #[ignore]
    #[test]
    fn test_real_connect_register_provision() {
        let config_path = "/tmp/test_real_agency_connect.json";

        let agency_did = "YRuVCckY6vfZfX9kcQZe3u";
        let agency_vk = "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v";
        let host = "https://enym-eagency.pdev.evernym.com";
        let wallet_name = "test_real_connect_register_provision";

        let result = connect_register_provision(&host, &agency_did, &agency_vk, Some(wallet_name.to_string()), None, Some(DEMO_ISSUER_PW_SEED.to_string()), None).unwrap();
        assert!(result.len() > 0);
        println!("result: {}", result);

        wallet::delete_wallet(&wallet_name).unwrap();
    }
}
