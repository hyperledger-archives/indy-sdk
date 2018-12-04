extern crate libc;
extern crate serde_json;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use std::ptr;
use credential_def;
use settings;
use error::ToErrorCode;
use utils::threadpool::spawn;

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
    info!("vcx_credentialdef_create >>>");

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
    trace!("vcx_credential_def_create(command_handle: {}, source_id: {}, credentialdef_name: {} schema_id: {}, issuer_did: {}, tag: {}, config: {})",
          command_handle,
          source_id,
          credentialdef_name,
          schema_id,
          issuer_did,
          tag,
          config);

    spawn(move|| {
        let ( rc, handle) = match credential_def::create_new_credentialdef(source_id,
                                                                 credentialdef_name,
                                                                 issuer_did,
                                                                 schema_id,
                                                                 tag,
                                                                 config) {
            Ok(x) => {
                trace!("vcx_credential_def_create_cb(command_handle: {}, rc: {}, credentialdef_handle: {}), source_id: {:?}",
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

        Ok(())
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
    info!("vcx_credentialdef_serialize >>>");

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = credential_def::get_source_id(credentialdef_handle).unwrap_or_default();
    trace!("vcx_credentialdef_serialize(command_handle: {}, credentialdef_handle: {}), source_id: {:?}",
          command_handle, credentialdef_handle, source_id);

    if !credential_def::is_valid_handle(credentialdef_handle) {
        return error::INVALID_CREDENTIAL_DEF_HANDLE.code_num;
    };

    spawn(move|| {
        match credential_def::to_string(credentialdef_handle) {
            Ok(x) => {
                trace!("vcx_credentialdef_serialize_cb(command_handle: {}, credentialdef_handle: {}, rc: {}, state: {}), source_id: {:?}",
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

        Ok(())
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
    info!("vcx_credentialdef_deserialize >>>");

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(credentialdef_data, error::INVALID_OPTION.code_num);

    trace!("vcx_credentialdef_deserialize(command_handle: {}, credentialdef_data: {})", command_handle, credentialdef_data);

    spawn(move|| {
        let (rc, handle) = match credential_def::from_string(&credentialdef_data) {
            Ok(x) => {
                trace!("vcx_credentialdef_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {}",
                      command_handle, error_string(0), x, credential_def::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(e) => {
                let error_code = e.to_error_code();
                warn!("vcx_credentialdef_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {}",
                      command_handle, error_code, 0, "");
                (error_code, 0)
            },
        };
        cb(command_handle, rc, handle);

        Ok(())
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
pub extern fn vcx_credentialdef_get_cred_def_id(command_handle: u32,
                                                cred_def_handle: u32,
                                                cb: Option<extern fn(xcommand_handle: u32, err: u32, cred_def_id: *const c_char)>) -> u32 {
    info!("vcx_credentialdef_get_cred_def_id >>>");

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = credential_def::get_source_id(cred_def_handle).unwrap_or_default();
    trace!("vcx_credentialdef_get_cred_def_id(command_handle: {}, cred_def_handle: {}) source_id: {}", command_handle, cred_def_handle, source_id);
    if !credential_def::is_valid_handle(cred_def_handle) {
        return error::INVALID_CREDENTIAL_DEF_HANDLE.code_num;
    }

    spawn(move|| {
        match credential_def::get_cred_def_id(cred_def_handle) {
            Ok(x) => {
                trace!("vcx_credentialdef_get_cred_def_id(command_handle: {}, cred_def_handle: {}, rc: {}, cred_def_id: {}) source_id: {}",
                      command_handle, cred_def_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_credentialdef_get_cred_def_id(command_handle: {}, cred_def_handle: {}, rc: {}, cred_def_id: {}) source_id: {}",
                      command_handle, cred_def_handle, x.to_string(), "", source_id);
                cb(command_handle, x.to_error_code(), ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Retrieve the txn associated with paying for the credential_def
///
/// #param
/// handle: credential_def handle that was provided during creation.  Used to access credential_def object.
///
/// #Callback returns
/// PaymentTxn json
/// example: {
///         "amount":25,
///         "inputs":[
///             "pay:null:1_3FvPC7dzFbQKzfG",
///             "pay:null:1_lWVGKc07Pyc40m6"
///         ],
///         "outputs":[
///             {"recipient":"pay:null:FrSVC3IrirScyRh","amount":5,"extra":null},
///             {"recipient":"pov:null:OsdjtGKavZDBuG2xFw2QunVwwGs5IB3j","amount":25,"extra":null}
///         ]
///     }
#[no_mangle]
pub extern fn vcx_credentialdef_get_payment_txn(command_handle: u32,
                                         handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, txn: *const c_char)>) -> u32 {
    info!("vcx_credentialdef_get_payment_txn >>>");

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = credential_def::get_source_id(handle).unwrap_or_default();
    trace!("vcx_credentialdef_get_payment_txn(command_handle: {}) source_id: {}", command_handle, source_id);

    spawn(move|| {
        match credential_def::get_payment_txn(handle) {
            Ok(x) => {
                match serde_json::to_string(&x) {
                    Ok(x) => {
                        trace!("vcx_credentialdef_get_payment_txn_cb(command_handle: {}, rc: {}, : {}), source_id: {}",
                              command_handle, error_string(0), x, credential_def::get_source_id(handle).unwrap_or_default());

                        let msg = CStringUtils::string_to_cstring(x);
                        cb(command_handle, 0, msg.as_ptr());
                    }
                    Err(_) => {
                        error!("vcx_credentialdef_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {}",
                               command_handle, error_string(error::INVALID_JSON.code_num), "null", credential_def::get_source_id(handle).unwrap_or_default());
                        cb(command_handle, error::INVALID_JSON.code_num, ptr::null_mut());
                    }
                }
            },
            Err(x) => {
                error!("vcx_credentialdef_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {}",
                       command_handle, x.to_string(), "null", credential_def::get_source_id(handle).unwrap_or_default());
                cb(command_handle, x.to_error_code(), ptr::null());
            },
        };

        Ok(())
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
    info!("vcx_credentialdef_release >>>");

    let source_id = credential_def::get_source_id(credentialdef_handle).unwrap_or_default();
    match credential_def::release(credentialdef_handle) {
        Ok(_) => trace!("vcx_credentialdef_release(credentialdef_handle: {}, rc: {}), source_id: {}",
                      credentialdef_handle, error_string(0), source_id),
        Err(x) => warn!("vcx_credentialdef_release(credentialdef_handle: {}, rc: {}), source_id: {}",
                        credentialdef_handle, x.to_string(), source_id),
    };
    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::time::Duration;
    use settings;
    use api::return_types_u32;
    use utils::constants::{SCHEMA_ID};

    #[test]
    fn test_vcx_create_credentialdef_success() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credentialdef_create(cb.command_handle,
                                            CString::new("Test Source ID").unwrap().into_raw(),
                                            CString::new("Test Credential Def").unwrap().into_raw(),
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            CString::new("6vkhW3L28AophhA68SSzRS").unwrap().into_raw(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_create_credentialdef_fails() {
        init!("false");
        settings::set_defaults();
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credentialdef_create(cb.command_handle,
                                            CString::new("Test Source ID").unwrap().into_raw(),
                                            CString::new("Test Credential Def").unwrap().into_raw(),
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            ptr::null(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(cb.get_callback())), error::SUCCESS.code_num);
        assert!(cb.receive(Some(Duration::from_secs(10))).is_err());
    }

    #[test]
    fn test_vcx_credentialdef_serialize() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credentialdef_create(cb.command_handle,
                                            CString::new("Test Source ID").unwrap().into_raw(),
                                            CString::new("Test Credential Def").unwrap().into_raw(),
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            ptr::null(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(cb.get_callback())), error::SUCCESS.code_num);

        let handle = cb.receive(Some(Duration::from_secs(10))).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_credentialdef_serialize(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        assert!(cb.receive(Some(Duration::from_secs(10))).is_ok());
    }

    #[test]
    fn test_vcx_credentialdef_deserialize_succeeds() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let original = r#"{"version":"1.0", "data": {"id":"2hoqvcwupRTUNkXn6ArYzs:3:CL:1697","tag":"tag","name":"Test Credential Definition","source_id":"SourceId"}}"#;
        assert_eq!(vcx_credentialdef_deserialize(cb.command_handle,
                                      CString::new(original).unwrap().into_raw(),
                                      Some(cb.get_callback())), error::SUCCESS.code_num);

        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert!(handle > 0);

    }

    #[test]
    fn test_vcx_creddef_get_id(){
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credentialdef_create(cb.command_handle,
                                            CString::new("Test Source ID").unwrap().into_raw(),
                                            CString::new("Test Credential Def").unwrap().into_raw(),
                                            CString::new(SCHEMA_ID).unwrap().into_raw(),
                                            CString::new("6vkhW3L28AophhA68SSzRS").unwrap().into_raw(),
                                            CString::new("tag").unwrap().into_raw(),
                                            CString::new("{}").unwrap().into_raw(),
                                            0,
                                            Some(cb.get_callback())), error::SUCCESS.code_num);
        let handle = cb.receive(Some(Duration::from_secs(10))).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_credentialdef_get_cred_def_id(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_get_payment_txn() {
        init!("true");
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let handle = credential_def::create_new_credentialdef("sid".to_string(),
                                                              "name".to_string(),
                                                              did,SCHEMA_ID.to_string(),
                                                              "tag".to_string(),"{}".to_string()).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_credentialdef_get_payment_txn(cb.command_handle, handle, Some(cb.get_callback()));
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }
}
