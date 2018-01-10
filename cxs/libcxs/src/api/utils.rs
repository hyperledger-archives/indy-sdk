extern crate libc;

use self::libc::c_char;
use messages;
use std::ptr;
use utils::httpclient;
use utils::constants::*;
use utils::cstring::CStringUtils;


#[no_mangle]
pub extern fn cxs_provision_agent(endpoint:*const c_char, agent_did: *const c_char, agent_vk: *const c_char, wallet_name: *const c_char, seed: *const c_char) -> *mut c_char {

    check_useful_c_str!(endpoint, ptr::null_mut());
    check_useful_c_str!(agent_did, ptr::null_mut());
    check_useful_c_str!(agent_vk, ptr::null_mut());
    check_useful_c_str!(wallet_name, ptr::null_mut());
    check_useful_opt_c_str!(seed, ptr::null_mut());

    match messages::register::connect_register_provision(&endpoint, &agent_did, &agent_vk, &wallet_name, seed) {
        Err(e) => {
            error!("Provision Agent Error {}.", e);
            return ptr::null_mut();
        },
        Ok(s) => {
            info!("Provision Agent Successful");
            let msg = CStringUtils::string_to_cstring(s);

            msg.into_raw()
        },
    }
}

#[no_mangle]
pub extern fn cxs_set_next_agency_response(message_index: u32) {
    let message = match message_index {
        1 => CREATE_KEYS_RESPONSE.to_vec(),
        2 => UPDATE_PROFILE_RESPONSE.to_vec(),
        3 => GET_MESSAGES_RESPONSE.to_vec(),
        4 => UPDATE_CLAIM_RESPONSE.to_vec(),
        5 => UPDATE_PROOF_RESPONSE.to_vec(),
        6 => CLAIM_REQ_RESPONSE.to_vec(),
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
    use utils::wallet;

    #[test]
    fn test_provision_agent() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let agency_did = "Ab8TvZa3Q19VNkQVzAWVL7";
        let agency_vk = "5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf";
        let host = "https://enym-eagency.pdev.evernym.com";
        let wallet_name = "test_provision_agent";

        let c_did = CString::new(agency_did).unwrap().into_raw();
        let c_vk = CString::new(agency_vk).unwrap().into_raw();
        let c_host = CString::new(host).unwrap().into_raw();
        let c_wallet = CString::new(wallet_name).unwrap().into_raw();

        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());
        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec());
        httpclient::set_next_u8_response(PROVISION_RESPONSE.to_vec());
        let result = cxs_provision_agent(c_did, c_vk, c_host, c_wallet, ptr::null());

        let final_string;
        unsafe {
            let c_string = CString::from_raw(result);
            final_string = c_string.into_string().unwrap();
        }

        assert!(final_string.len() > 0);
        println!("result: {}", final_string);

        wallet::delete_wallet(&wallet_name).unwrap();
    }
}
