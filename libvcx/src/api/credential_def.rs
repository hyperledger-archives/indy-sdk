extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use std::thread;
use std::ptr;
use credential_def;
use settings;
use error::ToErrorCode;

/// Create a new CredentialDef object that can create credential definitions on the ledger
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// credentialdef_name: Name of credential definitions
///
/// schema_seq_no: The schema sequence number to create credentialdef against
///
/// issuer_did: did corresponding to entity issuing a credential. Needs to have Trust Anchor permissions on ledger
///
/// create_non_revoc: Todo: need to add what this done. Right now, provide false
///
/// cb: Callback that provides CredentialDef handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credentialdef_create(command_handle: u32,
                                  source_id: *const c_char,
                                  credentialdef_name: *const c_char,
                                  schema_seq_no: u32,
                                  issuer_did: *const c_char,
                                  create_non_revoc: bool,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, credentialdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credentialdef_name, error::INVALID_OPTION.code_num);
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
    info!("vcx_credential_def_create(command_handle: {}, source_id: {}, credentialdef_name: {} schema_seq_no: {}, issuer_did: {}, create_non_rev: {})",
          command_handle,
          source_id,
          credentialdef_name,
          schema_seq_no,
          issuer_did,
          create_non_revoc);

    thread::spawn( move|| {
        let ( rc, handle) = match credential_def::create_new_credentialdef(source_id,
                                                                 credentialdef_name,
                                                                 schema_seq_no,
                                                                 issuer_did,
                                                                 create_non_revoc) {
            Ok(x) => {
                info!("vcx_credential_def_create_cb(command_handle: {}, rc: {}, credentialdef_handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, credential_def::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_credential_def_create_cb(command_handle: {}, rc: {}, credentialdef_handle: {}), source_id: {:?}",
                      command_handle, error_string(x.to_error_code()), 0, "");
                (x.to_error_code(), 0)
            },
        };
        cb(command_handle, rc, handle);
    });
    error::SUCCESS.code_num
}

/// Takes the credentialdef object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credentialdef_handle: Credentialdef handle that was provided during creation. Used to access credentialdef object
///
/// cb: Callback that provides json string of the credentialdef's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credentialdef_serialize(command_handle: u32,
                                     credentialdef_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, credentialdef_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = credential_def::get_source_id(credentialdef_handle).unwrap_or_default();
    info!("vcx_credentialdef_serialize(command_handle: {}, credentialdef_handle: {}), source_id: {:?}",
          command_handle, credentialdef_handle, source_id);

    if !credential_def::is_valid_handle(credentialdef_handle) {
        return error::INVALID_CREDENTIAL_DEF_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match credential_def::to_string(credentialdef_handle) {
            Ok(x) => {
                info!("vcx_credentialdef_serialize_cb(command_handle: {}, credentialdef_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, credentialdef_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_credentialdef_serialize_cb(command_handle: {}, credentialdef_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, credentialdef_handle, error_string(x), "null", source_id);
                cb(command_handle, x, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a credentialdef object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credentialdef_data: json string representing a credentialdef object
///
/// # Examples credentialdef -> {"source_id":"test id","credential_def":{"ref":15,"origin":"4fUDR9R7fjwELRvH9JT6HH","signature_type":"CL","data":{"primary":{"n":"9","s":"5","rms":"4","r":{"city":"6","address2":"8","address1":"7","state":"6","zip":"1"},"rctxt":"7","z":"7"},"revocation":null}},"handle":1378455216,"name":"NAME"}
///
/// cb: Callback that provides credentialdef handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credentialdef_deserialize(command_handle: u32,
                                       credentialdef_data: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, credentialdef_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credentialdef_data, error::INVALID_OPTION.code_num);

    info!("vcx_credentialdef_deserialize(command_handle: {}, credentialdef_data: {})", command_handle, credentialdef_data);

    thread::spawn( move|| {
        let (rc, handle) = match credential_def::from_string(&credentialdef_data) {
            Ok(x) => {
                info!("vcx_credentialdef_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, credential_def::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_credentialdef_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, "");
                (x, 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the credentialdef object by de-allocating memory
///
/// #Params
/// handle: Proof handle that was provided during creation. Used to access credential object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_credentialdef_release(credentialdef_handle: u32) -> u32 {
    let source_id = credential_def::get_source_id(credentialdef_handle).unwrap_or_default();
    match credential_def::release(credentialdef_handle) {
        Ok(_) => info!("vcx_credentialdef_release(credentialdef_handle: {}, rc: {}), source_id: {:?}",
                      credentialdef_handle, error_string(0), source_id),
        Err(x) => warn!("vcx_credentialdef_release(credentialdef_handle: {}, rc: {}), source_id: {:?}",
                        credentialdef_handle, error_string(x), source_id),
    };
    error::SUCCESS.code_num
}

#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credentialdef_commit(credentialdef_handle: u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credentialdef_get_sequence_no(credentialdef_handle: u32, sequence_no: *mut u32) -> u32 { error::SUCCESS.code_num }
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credentialdef_get(credentialdef_handle: u32, data: *mut c_char) -> u32 { error::SUCCESS.code_num }

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::thread;
    use std::time::Duration;
    use settings;
    use utils::libindy::pool;
    use utils::libindy::wallet::{ init_wallet, get_wallet_handle, delete_wallet };
    use utils::libindy::signus::SignusUtils;
    use utils::constants::{ DEMO_AGENT_PW_SEED, DEMO_ISSUER_PW_SEED };

    extern "C" fn create_cb(command_handle: u32, err: u32, credentialdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(credentialdef_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn create_cb_err(command_handle: u32, err: u32, credentialdef_handle: u32) {
        assert_ne!(err, 0);
        println!("successfully called create_cb_err")
    }

    extern "C" fn credential_def_on_ledger_err_cb(command_handle: u32, err: u32, credentialdef_handle: u32) {
        assert_eq!(err, error::CREDENTIAL_DEF_ALREADY_CREATED.code_num);
        println!("successfully called credential_def_on_ledger_err_cb")
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, credentialdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(credentialdef_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_credentialdef_serialize(0,credentialdef_handle,Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, credentialdef_str: *const c_char) {
        assert_eq!(err, 0);
        if credentialdef_str.is_null() {
            panic!("credentialdef is null");
        }
        check_useful_c_str!(credentialdef_str, ());
        println!("successfully called serialize_cb: {}", credentialdef_str);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, credentialdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(credentialdef_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = "{\"credential_def\":{\"ref\":15,\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"signature_type\":\"CL\",\"data\":{\"primary\":{\"n\":\"9\",\"s\":\"5\",\"rms\":\"4\",\"r\":{\"zip\":\"1\",\"address1\":\"7\",\"address2\":\"8\",\"city\":\"6\",\"state\":\"6\"},\"rctxt\":\"7\",\"z\":\"7\"},\"revocation\":null}},\"name\":\"NAME\",\"source_id\":\"test id\"}";
        let new = credential_def::to_string(credentialdef_handle).unwrap();
        let mut def1: credential_def::CreateCredentialDef = serde_json::from_str(expected).unwrap();
        let def2: credential_def::CreateCredentialDef = serde_json::from_str(&new).unwrap();
        def1.handle = def2.handle;
        assert_eq!(def1,def2);
    }

    fn set_default_and_enable_test_mode(){
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_vcx_create_credentialdef_success() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_credentialdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Credential Def").unwrap().into_raw(),
                                       15,
                                       CString::new("6vkhW3L28AophhA68SSzRS").unwrap().into_raw(),
                                       false,
                                       Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[ignore]
    #[test]
    fn test_vcx_create_credentialdef_with_pool() {
        settings::set_defaults();
        pool::open_sandbox_pool();
        init_wallet("test_vcx_create_credentialdef_with_pool").unwrap();
        let wallet_handle = get_wallet_handle();
        let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_ISSUER_PW_SEED)).unwrap();
        SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_AGENT_PW_SEED)).unwrap();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        assert_eq!(vcx_credentialdef_create(0,
                                       CString::new("qqqqq").unwrap().into_raw(),
                                       CString::new("Test Credential Def").unwrap().into_raw(),
                                       22,
                                       ptr::null(),
                                       false,
                                       Some(credential_def_on_ledger_err_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
        delete_wallet("test_vcx_create_credentialdef_with_pool").unwrap();
    }

    #[test]
    fn test_vcx_create_credentialdef_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(vcx_credentialdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Credential Def").unwrap().into_raw(),
                                       0,
                                       ptr::null(),
                                       false,
                                       Some(create_cb_err)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_credentialdef_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_credentialdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Credential Def").unwrap().into_raw(),
                                       15,
                                       ptr::null(),
                                       false,
                                       Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_credentialdef_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = "{\"source_id\":\"test id\",\"credential_def\":{\"ref\":15,\"origin\":\"4fUDR9R7fjwELRvH9JT6HH\",\"signature_type\":\"CL\",\"data\":{\"primary\":{\"n\":\"9\",\"s\":\"5\",\"rms\":\"4\",\"r\":{\"city\":\"6\",\"address2\":\"8\",\"address1\":\"7\",\"state\":\"6\",\"zip\":\"1\"},\"rctxt\":\"7\",\"z\":\"7\"},\"revocation\":null}},\"name\":\"NAME\"}";
        vcx_credentialdef_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }
}
