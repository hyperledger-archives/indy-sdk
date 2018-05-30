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
/// credentialdef_name: Name of credential definition
///
/// schema_id: The schema id given during the creation of the schema
///
/// issuer_did: did corresponding to entity issuing a credential. Needs to have Trust Anchor permissions on ledger
///
/// tag: way to create a unique credential def with the same schema and issuer did.
///
//Todo: Provide more info about the config
/// config: revocation info
///
/// cb: Callback that provides CredentialDef handle and error status of request.
///
/// payment_handle: future use (currently uses any address in wallet)
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credentialdef_create(command_handle: u32,
                                       source_id: *const c_char,
                                       credentialdef_name: *const c_char,
                                       schema_id: *const c_char,
                                       issuer_did: *const c_char,
                                       tag: *const c_char,
                                       config: *const c_char,
                                       payment_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, credentialdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credentialdef_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(tag, error::INVALID_OPTION.code_num);
    check_useful_c_str!(config, error::INVALID_OPTION.code_num);

    let issuer_did: String = if !issuer_did.is_null() {
        check_useful_c_str!(issuer_did, error::INVALID_OPTION.code_num);
        issuer_did.to_owned()
    } else {
        match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
            Ok(x) => x,
            Err(x) => return x
        }
    };
    info!("vcx_credential_def_create(command_handle: {}, source_id: {}, credentialdef_name: {} schema_id: {}, issuer_did: {}, tag: {}, config: {})",
          command_handle,
          source_id,
          credentialdef_name,
          schema_id,
          issuer_did,
          tag,
          config);

    thread::spawn( move|| {
        let ( rc, handle) = match credential_def::create_new_credentialdef(source_id,
                                                                 credentialdef_name,
                                                                 issuer_did,
                                                                 schema_id,
                                                                 tag,
                                                                 config) {
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

/// Retrieves credential definition's id
///
/// #Params
/// cred_def_handle: CredDef handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides credential definition id and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credentialdef_get_cred_def_id(command_handle: u32, cred_def_handle: u32, cb: Option<extern fn(xcommand_handle: u32, err: u32, cred_def_id: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_credentialdef_get_cred_def_id(command_handle: {}, cred_def_handle: {})", command_handle, cred_def_handle);
    if !credential_def::is_valid_handle(cred_def_handle) {
        return error::INVALID_CREDENTIAL_DEF_HANDLE.code_num;
    }

    thread::spawn(move|| {
        match credential_def::get_cred_def_id(cred_def_handle) {
            Ok(x) => {
                info!("vcx_credentialdef_get_cred_def_id(command_handle: {}, cred_def_handle: {}, rc: {}, cred_def_id: {})",
                      command_handle, cred_def_handle, error_string(0), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_credentialdef_get_cred_def_id(command_handle: {}, cred_def_handle: {}, rc: {}, cred_def_id: {})",
                      command_handle, cred_def_handle, error_string(x), "");
                cb(command_handle, x, ptr::null_mut());
            },
        };
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

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::thread;
    use std::time::Duration;
    use settings;
    use utils::constants::{SCHEMA_ID};

    extern "C" fn create_cb(command_handle: u32, err: u32, credentialdef_handle: u32) {
        assert_eq!(err, 0);
        assert!(credentialdef_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn create_cb_err(command_handle: u32, err: u32, credentialdef_handle: u32) {
        assert_ne!(err, 0);
        println!("successfully called create_cb_err")
    }

    extern "C" fn create_cb_get_id(command_handle: u32, err: u32, cred_def_handle: u32) {
        assert_eq!(err, 0);
        assert!(cred_def_handle > 0);
        println!("successfully called create_cb_get_id");
        assert_eq!(vcx_credentialdef_get_cred_def_id(0, cred_def_handle, Some(get_id_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_id_cb(handle: u32, err: u32, id: *const c_char) {
        assert_eq!(err, 0);
        if id.is_null() {
            panic!("id is null");
        }
        check_useful_c_str!(id, ());
        println!("successfully called get_id_cb: {}", id);
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
        let expected = r#"{"id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1697","tag":"tag","name":"Test Credential Definition","source_id":"SourceId"}"#;
        let new = credential_def::to_string(credentialdef_handle).unwrap();
        let mut def1: credential_def::CredentialDef = serde_json::from_str(expected).unwrap();
        let def2: credential_def::CredentialDef = serde_json::from_str(&new).unwrap();
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
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            CString::new("6vkhW3L28AophhA68SSzRS").unwrap().into_raw(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_create_credentialdef_fails() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        assert_eq!(vcx_credentialdef_create(0,
                                            CString::new("Test Source ID").unwrap().into_raw(),
                                            CString::new("Test Credential Def").unwrap().into_raw(),
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            ptr::null(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(create_cb_err)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_credentialdef_serialize() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_credentialdef_create(0,
                                            CString::new("Test Source ID").unwrap().into_raw(),
                                            CString::new("Test Credential Def").unwrap().into_raw(),
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            ptr::null(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_credentialdef_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = r#"{"id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1697","tag":"tag","name":"Test Credential Definition","source_id":"SourceId"}"#;
        vcx_credentialdef_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_creddef_get_id(){
        set_default_and_enable_test_mode();
        assert_eq!(vcx_credentialdef_create(0,
                                            CString::new("Test Source ID").unwrap().into_raw(),
                                            CString::new("Test Credential Def").unwrap().into_raw(),
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            CString::new("6vkhW3L28AophhA68SSzRS").unwrap().into_raw(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(create_cb_get_id)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}
