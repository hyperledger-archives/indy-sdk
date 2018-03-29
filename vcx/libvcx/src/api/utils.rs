extern crate libc;
extern crate serde_json;

use self::libc::c_char;
use messages::register::connect_register_provision;
use std::ptr;
use utils::httpclient;
use utils::constants::*;
use utils::cstring::CStringUtils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    agency_url: String,
    agency_did: String,
    agency_verkey: String,
    wallet_name: Option<String>,
    wallet_key: Option<String>,
    agent_seed: Option<String>,
    enterprise_seed: Option<String>,
}

#[no_mangle]
pub extern fn vcx_provision_agent(json:    *const c_char) -> *mut c_char {

    check_useful_c_str!(json, ptr::null_mut());
    let my_config: Config = match serde_json::from_str(&json) {
        Ok(x) => x,
        Err(x) => {
            return ptr::null_mut()
        },
    };

    match connect_register_provision(&my_config.agency_url, &my_config.agency_did, &my_config.agency_verkey, my_config.wallet_name, my_config.agent_seed, my_config.enterprise_seed, my_config.wallet_key) {
        Err(e) => {
            error!("Provision Agent Error {}.", e);
            return ptr::null_mut();
        },
        Ok(s) => {
            debug!("Provision Agent Successful");
            let msg = CStringUtils::string_to_cstring(s);

            msg.into_raw()
        },
    }
}

#[no_mangle]
pub extern fn vcx_set_next_agency_response(message_index: u32) {
    let message = match message_index {
        1 => CREATE_KEYS_RESPONSE.to_vec(),
        2 => UPDATE_PROFILE_RESPONSE.to_vec(),
        3 => GET_MESSAGES_RESPONSE.to_vec(),
        4 => UPDATE_CREDENTIAL_RESPONSE.to_vec(),
        5 => UPDATE_PROOF_RESPONSE.to_vec(),
        6 => CREDENTIAL_REQ_RESPONSE.to_vec(),
        7 => PROOF_RESPONSE.to_vec(),
        _ => Vec::new(),
    };

    httpclient::set_next_u8_response(message);
}

#[cfg(test)]
mod tests {

    use super::*;
    use settings;
    use std::ffi::CString;
    use utils::libindy::wallet;

    #[test]
    fn test_provision_agent() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        /*
        let agency_did = "Ab8TvZa3Q19VNkQVzAWVL7";
        let agency_vk = "5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf";
        let host = "https://enym-eagency.pdev.evernym.com";
        let wallet_name = "test_provision_agent";

        let c_did = CString::new(agency_did).unwrap().into_raw();
        let c_vk = CString::new(agency_vk).unwrap().into_raw();
        let c_host = CString::new(host).unwrap().into_raw();
        let c_wallet = CString::new(wallet_name).unwrap().into_raw();
        */

        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());
        let json_string = r#"{"agency_url":"https://enym-eagency.pdev.evernym.com","agency_did":"Ab8TvZa3Q19VNkQVzAWVL7","agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":null}"#;
        println!("json_string: {}",json_string);
        let c_json = CString::new(json_string).unwrap().into_raw();

        let result = vcx_provision_agent(c_json);
        //let result = vcx_provision_agent(c_did, c_vk, c_host, c_wallet, ptr::null(), ptr::null(), ptr::null());

        let final_string;
        unsafe {
            let c_string = CString::from_raw(result);
            final_string = c_string.into_string().unwrap();
        }

        assert!(final_string.len() > 0);
        println!("result: {}", final_string);

        wallet::delete_wallet("test_provision_agent").unwrap();
    }
}
