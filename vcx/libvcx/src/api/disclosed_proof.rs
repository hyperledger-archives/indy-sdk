use libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use connection;
use disclosed_proof;
use std::ptr;
use utils::threadpool::spawn;
use error::prelude::*;

/// Create a proof for fulfilling a corresponding proof request
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's identification for the proof, should be unique.
///
/// req: proof request received via "vcx_get_proof_requests"
///
/// cb: Callback that provides proof handle or error status
///
/// #Returns
/// Error code as u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_disclosed_proof_create_with_request(command_handle: u32,
                                                      source_id: *const c_char,
                                                      proof_req: *const c_char,
                                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {
    info!("vcx_disclosed_proof_create_with_request >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(proof_req, VcxErrorKind::InvalidOption);

    trace!("vcx_disclosed_proof_create_with_request(command_handle: {}, source_id: {}, proof_req: {})",
           command_handle, source_id, proof_req);

    spawn(move || {
        match disclosed_proof::create_proof(&source_id, &proof_req) {
            Ok(x) => {
                trace!("vcx_disclosed_proof_create_with_request_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                       command_handle,error::SUCCESS.message, x, source_id);
                cb(command_handle, 0, x);
            }
            Err(x) => {
                error!("vcx_disclosed_proof_create_with_request_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                       command_handle, x, 0, source_id);
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Create a proof for fulfilling a corresponding proof request
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's personal identification for the proof, should be unique.
///
/// connection: connection to query for proof request
///
/// msg_id: msg_id that contains the proof request
///
/// cb: Callback that provides proof handle and proof request or error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_disclosed_proof_create_with_msgid(command_handle: u32,
                                                    source_id: *const c_char,
                                                    connection_handle: u32,
                                                    msg_id: *const c_char,
                                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32, proof_req: *const c_char)>) -> u32 {
    info!("vcx_disclosed_proof_create_with_msgid >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(msg_id, VcxErrorKind::InvalidOption);

    trace!("vcx_disclosed_proof_create_with_msgid(command_handle: {}, source_id: {}, connection_handle: {}, msg_id: {})",
           command_handle, source_id, connection_handle, msg_id);

    spawn(move || {
        match disclosed_proof::get_proof_request(connection_handle, &msg_id) {
            Ok(request) => {
                match disclosed_proof::create_proof(&source_id, &request) {
                    Ok(handle) => {
                        trace!("vcx_disclosed_proof_create_with_msgid_cb(command_handle: {}, rc: {}, handle: {}, proof_req: {}) source_id: {}",
                               command_handle, error::SUCCESS.message, handle, request, source_id);
                        let msg = CStringUtils::string_to_cstring(request);
                        cb(command_handle, error::SUCCESS.code_num, handle, msg.as_ptr())
                    }
                    Err(e) => {
                        warn!("vcx_disclosed_proof_create_with_msgid_cb(command_handle: {}, rc: {}, handle: {}, proof_req: {}) source_id: {}",
                              command_handle, e, 0, request, source_id);
                        let msg = CStringUtils::string_to_cstring(request);
                        cb(command_handle, e.into(), 0, msg.as_ptr());
                    }
                };
            }
            Err(e) => cb(command_handle, e.into(), 0, ptr::null()),
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Send a proof to the connection, called after having received a proof request
///
/// #params
/// command_handle: command handle to map callback to API user context.
///
/// proof_handle: proof handle that was provided duration creation.  Used to identify proof object.
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of proof send request
///
/// #Returns
/// Error code as u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_send_proof(command_handle: u32,
                                             proof_handle: u32,
                                             connection_handle: u32,
                                             cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    info!("vcx_disclosed_proof_send_proof >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidDisclosedProofHandle).into()
    }

    if !connection::is_valid_handle(connection_handle) {
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_disclosed_proof_send_proof(command_handle: {}, proof_handle: {}, connection_handle: {}) source_id: {}",
           command_handle, proof_handle, connection_handle, source_id);

    spawn(move || {
        let err = match disclosed_proof::send_proof(proof_handle, connection_handle) {
            Ok(x) => {
                trace!("vcx_disclosed_proof_send_proof_cb(command_handle: {}, rc: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, source_id);
                cb(command_handle, error::SUCCESS.code_num);
            }
            Err(x) => {
                error!("vcx_disclosed_proof_send_proof_cb(command_handle: {}, rc: {}) source_id: {}",
                       command_handle, x, source_id);
                cb(command_handle, x.into());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Queries agency for proof requests from the given connection.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection to query for proof requests.
///
/// cb: Callback that provides any proof requests and error status of query
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_get_requests(command_handle: u32,
                                               connection_handle: u32,
                                               cb: Option<extern fn(xcommand_handle: u32, err: u32, requests: *const c_char)>) -> u32 {
    info!("vcx_disclosed_proof_get_requests >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !connection::is_valid_handle(connection_handle) {
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    trace!("vcx_disclosed_proof_get_requests(command_handle: {}, connection_handle: {})",
           command_handle, connection_handle);

    spawn(move || {
        match disclosed_proof::get_proof_request_messages(connection_handle, None) {
            Ok(x) => {
                trace!("vcx_disclosed_proof_get_requests_cb(command_handle: {}, rc: {}, msg: {})",
                       command_handle, error::SUCCESS.message, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                error!("vcx_disclosed_proof_get_requests_cb(command_handle: {}, rc: {}, msg: {})",
                       command_handle, error::SUCCESS.message, x);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Get the current state of the disclosed proof object
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access disclosed proof object
///
/// cb: Callback that provides most current state of the disclosed proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_get_state(command_handle: u32,
                                            proof_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_disclosed_proof_get_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidDisclosedProofHandle).into()
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_disclosed_proof_get_state(command_handle: {}, proof_handle: {}), source_id: {:?}",
           command_handle, proof_handle, source_id);

    spawn(move || {
        match disclosed_proof::get_state(proof_handle) {
            Ok(s) => {
                trace!("vcx_disclosed_proof_get_state_cb(command_handle: {}, rc: {}, state: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            }
            Err(e) => {
                error!("vcx_disclosed_proof_get_state_cb(command_handle: {}, rc: {}, state: {}) source_id: {}",
                       command_handle, e, 0, source_id);
                cb(command_handle, e.into(), 0)
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks for any state change in the disclosed proof and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Credential handle that was provided during creation. Used to identify disclosed proof object
///
/// cb: Callback that provides most current state of the disclosed proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_update_state(command_handle: u32,
                                               proof_handle: u32,
                                               cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_disclosed_proof_update_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidDisclosedProofHandle).into()
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_disclosed_proof_update_state(command_handle: {}, proof_handle: {}) source_id: {}",
           command_handle, proof_handle, source_id);

    spawn(move || {
        match disclosed_proof::update_state(proof_handle) {
            Ok(s) => {
                trace!("vcx_disclosed_proof_update_state_cb(command_handle: {}, rc: {}, state: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            }
            Err(e) => {
                error!("vcx_disclosed_proof_update_state_cb(command_handle: {}, rc: {}, state: {}) source_id: {}",
                       command_handle, e, 0, source_id);
                cb(command_handle, e.into(), 0)
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the disclosed proof object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// handle: Proof handle that was provided during creation. Used to identify the disclosed proof object
///
/// cb: Callback that provides json string of the disclosed proof's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_serialize(command_handle: u32,
                                            proof_handle: u32,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, data: *const c_char)>) -> u32 {
    info!("vcx_disclosed_proof_serialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidDisclosedProofHandle).into()
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_disclosed_proof_serialize(command_handle: {}, proof_handle: {}) source_id: {}",
           command_handle, proof_handle, source_id);

    spawn(move || {
        match disclosed_proof::to_string(proof_handle) {
            Ok(x) => {
                trace!("vcx_disclosed_proof_serialize_cb(command_handle: {}, rc: {}, data: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                error!("vcx_disclosed_proof_serialize_cb(command_handle: {}, rc: {}, data: {}) source_id: {}",
                       command_handle, x, 0, source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an disclosed proof object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// data: json string representing a disclosed proof object
///
///
/// cb: Callback that provides handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_deserialize(command_handle: u32,
                                              proof_data: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {
    info!("vcx_disclosed_proof_deserialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(proof_data, VcxErrorKind::InvalidOption);

    trace!("vcx_disclosed_proof_deserialize(command_handle: {}, proof_data: {})",
           command_handle, proof_data);

    spawn(move || {
        match disclosed_proof::from_string(&proof_data) {
            Ok(x) => {
                trace!("vcx_disclosed_proof_deserialize_cb(command_handle: {}, rc: {}, proof_handle: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, x, disclosed_proof::get_source_id(x).unwrap_or_default());

                cb(command_handle, 0, x);
            }
            Err(x) => {
                error!("vcx_disclosed_proof_deserialize_cb(command_handle: {}, rc: {}, proof_handle: {}) source_id: {}",
                       command_handle, x, 0, "");
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the disclosed proof object and returns a json string of all credentials matching associated proof request from wallet
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// handle: Proof handle that was provided during creation. Used to identify the disclosed proof object
///
/// cb: Callback that provides json string of the credentials in wallet associated with proof request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_retrieve_credentials(command_handle: u32,
                                                       proof_handle: u32,
                                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, data: *const c_char)>) -> u32 {
    info!("vcx_disclosed_proof_retrieve_credentials >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidDisclosedProofHandle).into()
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_disclosed_proof_retrieve_credentials(command_handle: {}, proof_handle: {}) source_id: {}",
           command_handle, proof_handle, source_id);

    spawn(move || {
        match disclosed_proof::retrieve_credentials(proof_handle) {
            Ok(x) => {
                trace!("vcx_disclosed_proof_retrieve_credentials(command_handle: {}, rc: {}, data: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                error!("vcx_disclosed_proof_retrieve_credentials(command_handle: {}, rc: {}, data: {}) source_id: {}",
                       command_handle, x, 0, source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the disclosed proof object and generates a proof from the selected credentials and self attested attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
///
/// handle: Proof handle that was provided during creation. Used to identify the disclosed proof object
///
/// selected_credentials: a json string with a credential for each proof request attribute.
///     List of possible credentials for each attribute is returned from vcx_disclosed_proof_retrieve_credentials,
///         (user needs to select specific credential to use from list of credentials)
///         {
///             "attrs":{
///                 String:{// Attribute key: This may not be the same as the attr name ex. "age_1" where attribute name is "age"
///                     "credential": {
///                         "cred_info":{
///                             "referent":String,
///                             "attrs":{ String: String }, // ex. {"age": "111", "name": "Bob"}
///                             "schema_id": String,
///                             "cred_def_id": String,
///                             "rev_reg_id":Option<String>,
///                             "cred_rev_id":Option<String>,
///                             },
///                         "interval":Option<{to: Option<u64>, from:: Option<u64>}>
///                     }, // This is the exact credential information selected from list of
///                        // credentials returned from vcx_disclosed_proof_retrieve_credentials
///                     "tails_file": Option<"String">, // Path to tails file for this credential
///                 },
///            },
///           "predicates":{ TODO: will be implemented as part of IS-1095 ticket. }
///        }
///     // selected_credentials can be empty "{}" if the proof only contains self_attested_attrs
///
/// self_attested_attrs: a json string with attributes self attested by user
/// # Examples self_attested_attrs -> "{"self_attested_attr_0":"attested_val"}" | "{}"
///
/// cb: Callback that returns error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_disclosed_proof_generate_proof(command_handle: u32,
                                                 proof_handle: u32,
                                                 selected_credentials: *const c_char,
                                                 self_attested_attrs: *const c_char,
                                                 cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    info!("vcx_disclosed_proof_generate_proof >>>");

    check_useful_c_str!(selected_credentials, VcxErrorKind::InvalidOption);
    check_useful_c_str!(self_attested_attrs, VcxErrorKind::InvalidOption);
    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !disclosed_proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidDisclosedProofHandle).into()
    }

    let source_id = disclosed_proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_disclosed_proof_generate_proof(command_handle: {}, proof_handle: {}, selected_credentials: {}, self_attested_attrs: {}) source_id: {}",
           command_handle, proof_handle, selected_credentials, self_attested_attrs, source_id);

    spawn(move || {
        match disclosed_proof::generate_proof(proof_handle, selected_credentials, self_attested_attrs) {
            Ok(_) => {
                trace!("vcx_disclosed_proof_generate_proof(command_handle: {}, rc: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, source_id);
                cb(command_handle, error::SUCCESS.code_num);
            }
            Err(x) => {
                error!("vcx_disclosed_proof_generate_proof(command_handle: {}, rc: {}) source_id: {}",
                       command_handle, x, source_id);
                cb(command_handle, x.into());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Releases the disclosed proof object by de-allocating memory
///
/// #Params
/// handle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_disclosed_proof_release(handle: u32) -> u32 {
    info!("vcx_disclosed_proof_release >>>");

    let source_id = disclosed_proof::get_source_id(handle).unwrap_or_default();
    match disclosed_proof::release(handle) {
        Ok(_) => {
            trace!("vcx_disclosed_proof_release(handle: {}, rc: {}), source_id: {:?}",
                   handle, error::SUCCESS.message, source_id);
            error::SUCCESS.code_num
        }
        Err(e) => {
            error!("vcx_disclosed_proof_release(handle: {}, rc: {}), source_id: {:?}",
                   handle, e, source_id);
            e.into()
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::time::Duration;
    use connection;
    use api::VcxStateType;
    use utils::constants::DEFAULT_SERIALIZE_VERSION;
    use api::return_types_u32;
    use serde_json::Value;

    pub const BAD_PROOF_REQUEST: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","claim": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","claim_name": "Account Certificate","claim_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    #[test]
    fn test_vcx_proof_create_with_request_success() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_create_with_request(cb.command_handle,
                                                           CString::new("test_create").unwrap().into_raw(),
                                                           CString::new(::utils::constants::PROOF_REQUEST_JSON).unwrap().into_raw(),
                                                           Some(cb.get_callback())), error::SUCCESS.code_num);
        assert!(cb.receive(Some(Duration::from_secs(10))).unwrap() > 0);
    }

    #[test]
    fn test_vcx_proof_create_with_request() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_create_with_request(
            cb.command_handle,
            CString::new("test_create").unwrap().into_raw(),
            CString::new(BAD_PROOF_REQUEST).unwrap().into_raw(),
            Some(cb.get_callback())), error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(), Some(error::INVALID_JSON.code_num));
    }

    #[test]
    fn test_create_with_msgid() {
        init!("true");
        let cxn = ::connection::tests::build_test_connection();
        ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec());
        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        assert_eq!(vcx_disclosed_proof_create_with_msgid(cb.command_handle,
                                                         CString::new("test_create_with_msgid").unwrap().into_raw(),
                                                         cxn,
                                                         CString::new("123").unwrap().into_raw(),
                                                         Some(cb.get_callback())), error::SUCCESS.code_num);
        let (handle, disclosed_proof) = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert!(handle > 0 && disclosed_proof.is_some());
    }

    #[test]
    fn test_vcx_disclosed_proof_release() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let handle = disclosed_proof::create_proof("1", ::utils::constants::PROOF_REQUEST_JSON).unwrap();
        let unknown_handle = handle + 1;
        let err = vcx_disclosed_proof_release(unknown_handle);
        assert_eq!(err, error::INVALID_DISCLOSED_PROOF_HANDLE.code_num);
    }

    #[test]
    fn test_vcx_disclosed_proof_serialize_and_deserialize() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let handle = disclosed_proof::create_proof("1", ::utils::constants::PROOF_REQUEST_JSON).unwrap();
        assert_eq!(vcx_disclosed_proof_serialize(cb.command_handle,
                                                 handle,
                                                 Some(cb.get_callback())), error::SUCCESS.code_num);
        let s = cb.receive(Some(Duration::from_secs(2))).unwrap().unwrap();
        let j: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(j["version"], DEFAULT_SERIALIZE_VERSION);

        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_deserialize(cb.command_handle,
                                                   CString::new(s).unwrap().into_raw(),
                                                   Some(cb.get_callback())),
                   error::SUCCESS.code_num);

        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_vcx_send_proof() {
        init!("true");

        let handle = disclosed_proof::create_proof("1", ::utils::constants::PROOF_REQUEST_JSON).unwrap();
        assert_eq!(disclosed_proof::get_state(handle).unwrap(), VcxStateType::VcxStateRequestReceived as u32);

        let connection_handle = connection::tests::build_test_connection();

        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_send_proof(cb.command_handle, handle, connection_handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_proof_get_requests() {
        init!("true");
        let cxn = ::connection::tests::build_test_connection();
        ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_PROOF_REQUEST_RESPONSE.to_vec());
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_disclosed_proof_get_requests(cb.command_handle, cxn, Some(cb.get_callback())), error::SUCCESS.code_num as u32);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_proof_get_state() {
        init!("true");
        let handle = disclosed_proof::create_proof("1", ::utils::constants::PROOF_REQUEST_JSON).unwrap();
        assert!(handle > 0);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_get_state(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        let state = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert_eq!(state, VcxStateType::VcxStateRequestReceived as u32);
    }

    #[test]
    fn test_vcx_disclosed_proof_retrieve_credentials() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_create_with_request(cb.command_handle,
                                                           CString::new("test_create").unwrap().into_raw(),
                                                           CString::new(::utils::constants::PROOF_REQUEST_JSON).unwrap().into_raw(),
                                                           Some(cb.get_callback())), error::SUCCESS.code_num);
        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_disclosed_proof_retrieve_credentials(cb.command_handle,
                                                            handle,
                                                            Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let credentials = cb.receive(None).unwrap().unwrap();
    }

    #[test]
    fn test_vcx_disclosed_proof_generate_proof() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_create_with_request(cb.command_handle,
                                                           CString::new("test_create").unwrap().into_raw(),
                                                           CString::new(::utils::constants::PROOF_REQUEST_JSON).unwrap().into_raw(),
                                                           Some(cb.get_callback())), error::SUCCESS.code_num);
        let proof_handle = cb.receive(Some(Duration::from_secs(10))).unwrap();
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_disclosed_proof_generate_proof(cb.command_handle,
                                                      proof_handle,
                                                      CString::new("{}").unwrap().into_raw(),
                                                      CString::new("{}").unwrap().into_raw(),
                                                      Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }
}
