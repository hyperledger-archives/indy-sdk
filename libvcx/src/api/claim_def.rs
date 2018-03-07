extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use std::thread;
use std::ptr;
use claim_def;
use settings;

/// Create a new ClaimDef object that can create claim definitions on the ledger
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// claimdef_name: Name of claim definitions
///
/// schema_seq_no: The schema sequence number to create claimdef against
///
/// issuer_did: did corresponding to entity issuing a claim. Needs to have Trust Anchor permissions on ledger
///
/// create_non_revoc: Todo: need to add what this done. Right now, provide false
///
/// cb: Callback that provides ClaimDef handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_claimdef_create(command_handle: u32,
                                  source_id: *const c_char,
                                  claimdef_name: *const c_char,
                                  schema_seq_no: u32,
                                  issuer_did: *const c_char,
                                  create_non_revoc: bool,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claimdef_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    let issuer_did: String = if !issuer_did.is_null() {
        check_useful_c_str!(issuer_did, error::INVALID_OPTION.code_num);
        issuer_did.to_owned()
    } else {
        match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
            Ok(x) => x,
            Err(x) => return x
        }
    };
    info!("vcx_claim_def_create(command_handle: {}, source_id: {}, claimdef_name: {} schema_seq_no: {}, issuer_did: {}, create_non_rev: {})",
          command_handle,
          source_id,
          claimdef_name,
          schema_seq_no,
          issuer_did,
          create_non_revoc);

    thread::spawn( move|| {
        let ( rc, handle) = match claim_def::create_new_claimdef(source_id,
                                                                 claimdef_name,
                                                                 schema_seq_no,
                                                                 issuer_did,
                                                                 create_non_revoc) {
            Ok(x) => {
                info!("vcx_claim_def_create_cb(command_handle: {}, rc: {}, claimdef_handle: {})",
                      command_handle, error_string(0), x);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_claim_def_create_cb(command_handle: {}, rc: {}, claimdef_handle: {})",
                      command_handle, error_string(x), 0);
                (x, 0)
            },
        };
        cb(command_handle, rc, handle);
    });
    error::SUCCESS.code_num
}

/// Takes the claimdef object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claimdef_handle: Claimdef handle that was provided during creation. Used to access claimdef object
///
/// cb: Callback that provides json string of the claimdef's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_claimdef_serialize(command_handle: u32,
                                     claimdef_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_claimdef_serialize(command_handle: {}, claimdef_handle: {})", command_handle, claimdef_handle);

    if !claim_def::is_valid_handle(claimdef_handle) {
        return error::INVALID_CLAIM_DEF_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match claim_def::to_string(claimdef_handle) {
            Ok(x) => {
                info!("vcx_claimdef_serialize_cb(command_handle: {}, claimdef_handle: {}, rc: {}, state: {})",
                      command_handle, claimdef_handle, error_string(0), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_claimdef_serialize_cb(command_handle: {}, claimdef_handle: {}, rc: {}, state: {})",
                      command_handle, claimdef_handle, error_string(x), "null");
                cb(command_handle, x, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a claimdef object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// claimdef_data: json string representing a claimdef object
///
/// # Examples claimdef -> {"source_id":"test id","claim_def":{"ref":15,"origin":"4fUDR9R7fjwELRvH9JT6HH","signature_type":"CL","data":{"primary":{"n":"9","s":"5","rms":"4","r":{"city":"6","address2":"8","address1":"7","state":"6","zip":"1"},"rctxt":"7","z":"7"},"revocation":null}},"handle":1378455216,"name":"NAME"}
///
/// cb: Callback that provides claimdef handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_claimdef_deserialize(command_handle: u32,
                                       claimdef_data: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(claimdef_data, error::INVALID_OPTION.code_num);

    info!("vcx_claimdef_deserialize(command_handle: {}, claimdef_data: {})", command_handle, claimdef_data);

    thread::spawn( move|| {
        let (rc, handle) = match claim_def::from_string(&claimdef_data) {
            Ok(x) => {
                info!("vcx_claimdef_deserialize_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(0), x);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_claimdef_deserialize_cb(command_handle: {}, rc: {}, handle: {})",
                      command_handle, error_string(x), 0);
                (x, 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

#[no_mangle]
pub extern fn vcx_claimdef_release(claimdef_handle: u32) -> u32 {
    info!("vcx_claimdef_release(claimdef_handle: {})", claimdef_handle);
    claim_def::release(claimdef_handle)
}

#[allow(unused_variables, unused_mut)]
pub extern fn vcx_claimdef_commit(claimdef_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_claimdef_get_sequence_no(claimdef_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_claimdef_get(claimdef_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::thread;
    use std::time::Duration;
    use settings;
    use utils::libindy::pool;
    use utils::libindy::wallet::{ init_wallet, get_wallet_handle };
    use utils::libindy::signus::SignusUtils;
    use utils::constants::{ DEMO_AGENT_PW_SEED, DEMO_ISSUER_PW_SEED };

    extern "C" fn create_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(claimdef_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn create_cb_err(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_ne!(err, 0);
        println!("successfully called create_cb_err")
    }

    extern "C" fn claim_def_on_ledger_err_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, error::CLAIM_DEF_ALREADY_CREATED.code_num);
        println!("successfully called claim_def_on_ledger_err_cb")
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(claimdef_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_claimdef_serialize(0,claimdef_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, claimdef_str: *const c_char) {
        assert_eq!(err, 0);
        if claimdef_str.is_null() {
            panic!("claimdef is null");
        }
        check_useful_c_str!(claimdef_str, ());
        println!("successfully called serialize_cb: {}", claimdef_str);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, claimdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(claimdef_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = "{\"claim_def\":{\"ref\":15,\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"signature_type\":\"CL\",\"data\":{\"primary\":{\"n\":\"9\",\"s\":\"5\",\"rms\":\"4\",\"r\":{\"zip\":\"1\",\"address1\":\"7\",\"address2\":\"8\",\"city\":\"6\",\"state\":\"6\"},\"rctxt\":\"7\",\"z\":\"7\"},\"revocation\":null}},\"handle\":1378455216,\"name\":\"NAME\",\"source_id\":\"test id\"}";
        let new = claim_def::to_string(claimdef_handle).unwrap();
        let def1: claim_def::CreateClaimDef = serde_json::from_str(expected).unwrap();
        let def2: claim_def::CreateClaimDef = serde_json::from_str(&new).unwrap();
        assert_eq!(def1,def2);
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_vcx_create_claimdef_success() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       15,
                                       CString::new("6vkhW3L28AophhA68SSzRS").unwrap().into_raw(),
                                       false,
                                       Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[ignore]
    #[test]
    fn test_vcx_create_claimdef_with_pool() {
        settings::set_defaults();
        pool::open_sandbox_pool();
        init_wallet("a_test_wallet").unwrap();
        let wallet_handle = get_wallet_handle();
        let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_ISSUER_PW_SEED)).unwrap();
        SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_AGENT_PW_SEED)).unwrap();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        assert_eq!(vcx_claimdef_create(0,
                                       CString::new("qqqqq").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       22,
                                       ptr::null(),
                                       false,
                                       Some(claim_def_on_ledger_err_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_vcx_create_claimdef_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(vcx_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       0,
                                       ptr::null(),
                                       false,
                                       Some(create_cb_err)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_claimdef_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       15,
                                       ptr::null(),
                                       false,
                                       Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_claimdef_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = "{\"source_id\":\"test id\",\"claim_def\":{\"ref\":15,\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"signature_type\":\"CL\",\"data\":{\"primary\":{\"n\":\"9\",\"s\":\"5\",\"rms\":\"4\",\"r\":{\"city\":\"6\",\"address2\":\"8\",\"address1\":\"7\",\"state\":\"6\",\"zip\":\"1\"},\"rctxt\":\"7\",\"z\":\"7\"},\"revocation\":null}},\"handle\":1378455216,\"name\":\"NAME\"}";
        vcx_claimdef_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }
}
