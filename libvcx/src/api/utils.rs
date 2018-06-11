extern crate libc;
extern crate serde_json;

use self::libc::c_char;
use messages;
use std::ptr;
use std::thread;
use utils::httpclient;
use utils::constants::*;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    agency_url: String,
    agency_did: String,
    agency_verkey: String,
    wallet_name: Option<String>,
    wallet_key: String,
    agent_seed: Option<String>,
    enterprise_seed: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateAgentInfo {
    id: String,
    value: String,
}

/// Provision an agent in the agency, populate configuration and wallet for this agent.
/// NOTE: for asynchronous call use vcx_agent_provision_async
///
/// #Params
/// json: configuration
///
/// #Returns
/// Configuration (wallet also populated), on error returns NULL

#[no_mangle]
pub extern fn vcx_provision_agent(json: *const c_char) -> *mut c_char {
    check_useful_c_str!(json, ptr::null_mut());
    let my_config: Config = match serde_json::from_str(&json) {
        Ok(x) => x,
        Err(x) => {
            return ptr::null_mut()
        },
    };

    match messages::agent_utils::connect_register_provision(&my_config.agency_url,
                                                         &my_config.agency_did,
                                                         &my_config.agency_verkey,
                                                         my_config.wallet_name,
                                                         my_config.agent_seed,
                                                         my_config.enterprise_seed,
                                                         &my_config.wallet_key, None, None, None) {
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

/// Provision an agent in the agency, populate configuration and wallet for this agent.
/// NOTE: for synchronous call use vcx_provision_agent
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// json: configuration
///
/// cb: Callback that provides configuration or error status
///
/// #Returns
/// Configuration (wallet also populated), on error returns NULL

#[no_mangle]
pub extern fn vcx_agent_provision_async(command_handle : u32,
                               json: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, config: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(json, error::INVALID_OPTION.code_num);

    let my_config: Config = match serde_json::from_str(&json) {
        Ok(x) => x,
        Err(x) => {
            return error::INVALID_JSON.code_num
        },
    };

    info!("vcx_agent_provision_async(command_handle: {}, json: {})",
          command_handle, json);

    thread::spawn(move|| {
        match messages::agent_utils::connect_register_provision(&my_config.agency_url,
                                                                &my_config.agency_did,
                                                                &my_config.agency_verkey,
                                                                my_config.wallet_name,
                                                                my_config.agent_seed,
                                                                my_config.enterprise_seed,
                                                                &my_config.wallet_key, None, None, None) {
            Err(e) => {
                error!("vcx_agent_provision_async_cb(command_handle: {}, rc: {}, config: NULL", command_handle, error_string(e));
                cb(command_handle, 0, ptr::null_mut());
            },
            Ok(s) => {
                info!("vcx_agent_provision_async_cb(command_handle: {}, rc: {}, config: {})",
                      command_handle, error_string(0), s);
                let msg = CStringUtils::string_to_cstring(s);
                cb(command_handle, 0, msg.as_ptr());
            },
        }
    });

    error::SUCCESS.code_num
}

/// Update information on the agent (ie, comm method and type)
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// json: updated configuration
///
/// cb: Callback that provides configuration or error status
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_agent_update_info(command_handle: u32,
                                    json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(json, error::INVALID_OPTION.code_num);

    info!("vcx_agent_update_info(command_handle: {}, json: {})",
          command_handle, json);

    let agent_info: UpdateAgentInfo = match serde_json::from_str(&json) {
        Ok(x) => x,
        Err(x) => {
            return error::INVALID_OPTION.code_num
        },
    };

    thread::spawn(move|| {
        match messages::agent_utils::update_agent_info(&agent_info.id, &agent_info.value){
            Ok(x) => {
                info!("vcx_agent_update_info_cb(command_handle: {}, rc: {})",
                      command_handle, error::error_string(0));
                cb(command_handle, error::SUCCESS.code_num);
            },
            Err(e) => {
                error!("vcx_agent_update_info_cb(command_handle: {}, rc: {})",
                      command_handle, error::error_string(e));
                cb(command_handle, e);
            },
        };
    });

    error::SUCCESS.code_num
}

/// Get ledger fees from the sovrin network
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// cb: Callback that provides the fee structure for the sovrin network
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_ledger_get_fees(command_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, fees: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    info!("vcx_ledger_get_fees(command_handle: {})",
          command_handle);

    thread::spawn(move|| {
        match ::utils::libindy::payments::get_ledger_fees() {
            Ok(x) => {
                info!("vcx_ledger_get_fees_cb(command_handle: {}, rc: {}, fees: {})",
                      command_handle, error::error_string(0), x);

                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(e) => {
                warn!("vcx_ledget_get_fees_cb(command_handle: {}, rc: {}, fees: {})",
                      command_handle, error_string(e), "null");

                cb(command_handle, e, ptr::null_mut());
            },
        };
    });

    error::SUCCESS.code_num
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
    use std::time::Duration;

    #[test]
    fn test_provision_agent() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let json_string = r#"{"agency_url":"https://enym-eagency.pdev.evernym.com","agency_did":"Ab8TvZa3Q19VNkQVzAWVL7","agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":null}"#;
        let c_json = CString::new(json_string).unwrap().into_raw();

        let result = vcx_provision_agent(c_json);

        check_useful_c_str!(result,());
        assert!(result.len() > 0);
    }

    extern "C" fn generic_cb(command_handle: u32, err: u32, config: *const c_char) {
        if err != 0 {panic!("generic_cb failed")}
        check_useful_c_str!(config, ());
        println!("successfully called generic_cb: {}", config);
    }

    #[test]
    fn test_create_agent() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let json_string = r#"{"agency_url":"https://enym-eagency.pdev.evernym.com","agency_did":"Ab8TvZa3Q19VNkQVzAWVL7","agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":"key"}"#;
        let c_json = CString::new(json_string).unwrap().into_raw();

        let result = vcx_agent_provision_async(0, c_json, Some(generic_cb));
        assert_eq!(0, result);
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_create_agent_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec()); //set response garbage
        let json_string = r#"{"agency_url":"https://enym-eagency.pdev.evernym.com","agency_did":"Ab8TvZa3Q19VNkQVzAWVL7","agency_verkey":"5LXaR43B1aQyeh94VBP8LG1Sgvjk7aNfqiksBCSjwqbf","wallet_name":"test_provision_agent","agent_seed":null,"enterprise_seed":null,"wallet_key":null}"#;
        let c_json = CString::new(json_string).unwrap().into_raw();

        let result = vcx_agent_provision_async(0, c_json, Some(generic_cb));

        thread::sleep(Duration::from_secs(1));
    }

    extern "C" fn update_cb(command_handle: u32, err: u32) {
        if err != 0 {panic!("update_cb failed")}
        println!("successfully called update_cb")
    }

    #[test]
    fn test_update_agent_info() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let json_string = r#"{"id":"123","value":"value"}"#;
        let c_json = CString::new(json_string).unwrap().into_raw();

        let result = vcx_agent_update_info(0, c_json, Some(update_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_update_agent_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        httpclient::set_next_u8_response(REGISTER_RESPONSE.to_vec()); //set response garbage
        let json_string = r#"{"id":"123"}"#;
        let c_json = CString::new(json_string).unwrap().into_raw();

        let result = vcx_agent_update_info(0, c_json, Some(update_cb));
        assert_eq!(result,error::INVALID_OPTION.code_num);
    }

    #[test]
    fn test_get_ledger_fees() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");

        let result = vcx_ledger_get_fees(0, Some(generic_cb));
        assert_eq!(result,0);
        thread::sleep(Duration::from_secs(1));
    }
}
