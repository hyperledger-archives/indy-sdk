use libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use proof;
use connection;
use std::ptr;
use utils::threadpool::spawn;
use error::prelude::*;
use messages::get_message::Message;

/// Create a new Proof object that requests a proof for an enterprise
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// requested_attrs: Describes requested attribute
///     {
///         "name": string, // attribute name, (case insensitive and ignore spaces)
///         "restrictions":  (filter_json) {
///            "schema_id": string, (Optional)
///            "schema_issuer_did": string, (Optional)
///            "schema_name": string, (Optional)
///            "schema_version": string, (Optional)
///            "issuer_did": string, (Optional)
///            "cred_def_id": string, (Optional)
///        },
///         "non_revoked": {
///             "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///             "to": Optional<(u64)>
///                 //Requested time represented as a total number of seconds from Unix Epoch, Optional
///         }
///     }
///
/// # Example requested_attrs -> "[{"name":"attrName","restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// requested_predicates: predicate specifications prover must provide claim for
///          { // set of requested predicates
///             "name": attribute name, (case insensitive and ignore spaces)
///             "p_type": predicate type (Currently ">=" only)
///             "p_value": int predicate value
///             "restrictions": Optional<filter_json>, // see above
///             "non_revoked": Optional<{
///                 "from": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///                 "to": Optional<(u64)> Requested time represented as a total number of seconds from Unix Epoch, Optional
///             }>
///          },
///
/// # Example requested_predicates -> "[{"name":"attrName","p_type":"GE","p_value":9,"restrictions":["issuer_did":"did","schema_id":"id","schema_issuer_did":"did","schema_name":"name","schema_version":"1.1.1","cred_def_id":"id"}]]"
///
/// revocation_interval:  Optional<<revocation_interval>>, // see below,
///                        // If specified, prover must proof non-revocation
///                        // for date in this interval for each attribute
///                        // (can be overridden on attribute level)
///     from: Optional<u64> // timestamp of interval beginning
///     to: Optional<u64> // timestamp of interval beginning
///         // Requested time represented as a total number of seconds from Unix Epoch, Optional
/// # Examples config ->  "{}" | "{"to": 123} | "{"from": 100, "to": 123}"
///
///
///
///
/// cb: Callback that provides proof handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_create(command_handle: u32,
                               source_id: *const c_char,
                               requested_attrs: *const c_char,
                               requested_predicates: *const c_char,
                               revocation_interval: *const c_char,
                               name: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {
    info!("vcx_proof_create >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(requested_attrs, VcxErrorKind::InvalidOption);
    check_useful_c_str!(requested_predicates, VcxErrorKind::InvalidOption);
    check_useful_c_str!(name, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(revocation_interval, VcxErrorKind::InvalidOption);

    trace!("vcx_proof_create(command_handle: {}, source_id: {}, requested_attrs: {}, requested_predicates: {}, revocation_interval: {}, name: {})",
          command_handle, source_id, requested_attrs, requested_predicates, revocation_interval, name);

    spawn(move|| {
        let ( rc, handle) = match proof::create_proof(source_id, requested_attrs, requested_predicates, revocation_interval, name) {
            Ok(x) => {
                trace!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, error::SUCCESS.message, x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x, 0,x);
                (x.into(), 0)
            },
        };
        cb(command_handle, rc, handle);

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks for any state change and updates the proof state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides most current state of the proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_update_state(command_handle: u32,
                                     proof_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_proof_update_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_proof_update_state(command_handle: {}, proof_handle: {}) source_id: {}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidProofHandle).into()
    }

    spawn(move|| {
        match proof::update_state(proof_handle, None) {
            Ok(x) => {
                trace!("vcx_proof_update_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}) source_id: {}",
                      command_handle, error::SUCCESS.message, proof_handle, x, source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(x) => {
                warn!("vcx_proof_update_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}) source_id: {}",
                      command_handle, x, proof_handle, 0, source_id);
                cb(command_handle, x.into(), 0);
            }
        }

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks for any state change from the given message and updates the proof state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// message: String containing updated status
///
/// cb: Callback that provides most current state of the proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_update_state_with_message(command_handle: u32,
                                                  proof_handle: u32,
                                                  message: *const c_char,
                                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_proof_update_state_with_message >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(message, VcxErrorKind::InvalidOption);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_proof_update_state_with_message(command_handle: {}, proof_handle: {}) source_id: {}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidProofHandle).into()
    }

    let message: Message = match serde_json::from_str(&message) {
        Ok(x) => x,
        Err(_) => return VcxError::from(VcxErrorKind::InvalidJson).into(),
    };

    spawn(move|| {
        match proof::update_state(proof_handle, Some(message)) {
            Ok(x) => {
                trace!("vcx_proof_update_state_with_message_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}) source_id: {}",
                      command_handle, error::SUCCESS.message, proof_handle, x, source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(x) => {
                warn!("vcx_proof_update_state_with_message_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}) source_id: {}",
                      command_handle, x, proof_handle, 0, source_id);
                cb(command_handle, x.into(), 0);
            }
        }

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Get the current state of the proof object from the given message
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides most current state of the proof and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_get_state(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_proof_get_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_proof_get_state(command_handle: {}, proof_handle: {}), source_id: {}",
          command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidProofHandle).into()
    }

    spawn(move|| {
        match proof::get_state(proof_handle) {
            Ok(x) => {
                trace!("vcx_proof_get_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}) source_id: {}",
                      command_handle, error::SUCCESS.message, proof_handle, x, source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(x) => {
                warn!("vcx_proof_get_state_cb(command_handle: {}, rc: {}, proof_handle: {}, state: {}) source_id: {}",
                      command_handle, x, proof_handle, 0, source_id);
                cb(command_handle, x.into(), 0);
            }
        }

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the proof object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides json string of the proof's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_serialize(command_handle: u32,
                                  proof_handle: u32,
                                  cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state: *const c_char)>) -> u32 {
    info!("vcx_proof_serialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_proof_serialize(command_handle: {}, proof_handle: {}) source_id: {}", command_handle, proof_handle, source_id);

    if !proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidProofHandle).into()
    };

    spawn(move|| {
        match proof::to_string(proof_handle) {
            Ok(x) => {
                trace!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}) source_id: {}",
                      command_handle, proof_handle, error::SUCCESS.message, x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_proof_serialize_cb(command_handle: {}, proof_handle: {}, rc: {}, state: {}) source_id: {}",
                      command_handle, proof_handle, x, "null", source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a proof object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_data: json string representing a proof object
///
/// cb: Callback that provides proof handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_deserialize(command_handle: u32,
                                    proof_data: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_handle: u32)>) -> u32 {
    info!("vcx_proof_deserialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(proof_data, VcxErrorKind::InvalidOption);

    trace!("vcx_proof_deserialize(command_handle: {}, proof_data: {})",
          command_handle, proof_data);

    spawn(move|| {
        let (rc, handle) = match proof::from_string(&proof_data) {
            Ok(x) => {
                trace!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, error::SUCCESS.message, x, proof::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_proof_deserialize_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x, 0, "");
                (x.into(), 0)
            },
        };
        cb(command_handle, rc, handle);

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Releases the proof object by de-allocating memory
///
/// #Params
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_proof_release(proof_handle: u32) -> u32 {
    info!("vcx_proof_release >>>");

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    match proof::release(proof_handle) {
        Ok(_) => {
            trace!("vcx_proof_release(proof_handle: {}, rc: {}), source_id: {}",
                       proof_handle, error::SUCCESS.message, source_id);
            error::SUCCESS.code_num
        },
        Err(e) => {
            warn!("vcx_proof_release(proof_handle: {}, rc: {}), source_id: {}",
                       proof_handle, e, source_id);
            e.into()
        },
    }
}

/// Sends a proof request to pairwise connection
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to access proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: provides any error status of the proof_request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_proof_send_request(command_handle: u32,
                                     proof_handle: u32,
                                     connection_handle: u32,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    info!("vcx_proof_send_request >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_proof_send_request(command_handle: {}, proof_handle: {}, connection_handle: {}) source_id: {}",
          command_handle, proof_handle, connection_handle, source_id);
    if !proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidProofHandle).into()
    }

    if !connection::is_valid_handle(connection_handle) {
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    spawn(move|| {
        let err = match proof::send_proof_request(proof_handle, connection_handle) {
            Ok(x) => {
                trace!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {}) source_id: {}",
                      command_handle, 0, proof_handle, source_id);
                x
            },
            Err(x) => {
                warn!("vcx_proof_send_request_cb(command_handle: {}, rc: {}, proof_handle: {}) source_id: {}",
                      command_handle, x, proof_handle, source_id);
                x.into()
            },
        };

        cb(command_handle,err);

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Get Proof
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Proof handle that was provided during creation. Used to identify proof object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides Proof attributes and error status of sending the credential
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_get_proof(command_handle: u32,
                            proof_handle: u32,
                            connection_handle: u32,
                            cb: Option<extern fn(xcommand_handle: u32, err: u32, proof_state:u32, response_data: *const c_char)>) -> u32 {
    info!("vcx_get_proof >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = proof::get_source_id(proof_handle).unwrap_or_default();
    trace!("vcx_get_proof(command_handle: {}, proof_handle: {}, connection_handle: {}) source_id: {}",
          command_handle, proof_handle, connection_handle, source_id);
    if !proof::is_valid_handle(proof_handle) {
        return VcxError::from(VcxErrorKind::InvalidProofHandle).into()
    }

    if !connection::is_valid_handle(connection_handle) {
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    spawn(move|| {
        //update the state to see if proof has come, ignore any errors
        match proof::update_state(proof_handle, None) {
            Ok(_) => (),
            Err(_) => (),
        };

        match proof::get_proof(proof_handle) {
            Ok(x) => {
                trace!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {}) source_id: {}", command_handle, proof_handle, 0, x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, proof::get_proof_state(proof_handle).unwrap_or(0), msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_get_proof_cb(command_handle: {}, proof_handle: {}, rc: {}, proof: {}) source_id: {}", command_handle, proof_handle, x, "null", source_id);
                cb(command_handle, x.into(), proof::get_proof_state(proof_handle).unwrap_or(0), ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


#[allow(unused_variables)]
pub extern fn vcx_proof_accepted(proof_handle: u32, response_data: *const c_char) -> u32 {
    info!("vcx_proof_accepted >>>");
    error::SUCCESS.code_num
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use std::str;
    use std::time::Duration;
    use std::thread;
    use proof;
    use connection;
    use api::{ ProofStateType, return_types_u32, VcxStateType };
    use utils::constants::*;

    static DEFAULT_PROOF_NAME: &'static str = "PROOF_NAME";

    fn create_proof_util() -> (return_types_u32::Return_U32_U32, u32) {
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_proof_create(cb.command_handle,
                                    CString::new(DEFAULT_PROOF_NAME).unwrap().into_raw(),
                                    CString::new(REQUESTED_ATTRS).unwrap().into_raw(),
                                    CString::new(REQUESTED_PREDICATES).unwrap().into_raw(),
                                  CString::new(r#"{"support_revocation":false}"#).unwrap().into_raw(),
                                    CString::new("optional").unwrap().into_raw(),
                                    Some(cb.get_callback()));
        (cb, rc)
    }

    #[test]
    fn test_vcx_create_proof_success() {
        init!("true");
        let (cb, rc) = create_proof_util();
        assert_eq!(rc, error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_create_proof_fails() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_proof_create(cb.command_handle,
                                    ptr::null(),
                                    ptr::null(),
                                    ptr::null(),
                                    CString::new(r#"{"support_revocation":false}"#).unwrap().into_raw(),
                                    ptr::null(),
                                    None),
                   error::INVALID_OPTION.code_num);
    }

    #[test]
    fn test_vcx_proof_serialize() {
        init!("true");
        let (cb, rc) = create_proof_util();
        assert_eq!(rc, error::SUCCESS.code_num);
        let proof_handle = cb.receive(Some(Duration::from_secs(10))).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_proof_serialize(cb.command_handle,
                                       proof_handle,
                                       Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_proof_deserialize_succeeds() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let original = r#"{"nonce":"123456","version":"1.0","handle":1,"msg_uid":"","ref_msg_id":"","name":"Name Data","prover_vk":"","agent_did":"","agent_vk":"","remote_did":"","remote_vk":"","prover_did":"8XFh8yBzrpJQmNyZzgoTqB","requested_attrs":"{\"attrs\":[{\"name\":\"person name\"},{\"schema_seq_no\":1,\"name\":\"address_1\"},{\"schema_seq_no\":2,\"issuer_did\":\"ISSUER_DID2\",\"name\":\"address_2\"},{\"schema_seq_no\":1,\"name\":\"city\"},{\"schema_seq_no\":1,\"name\":\"state\"},{\"schema_seq_no\":1,\"name\":\"zip\"}]}","requested_predicates":"{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18,\"schema_seq_no\":1,\"issuer_did\":\"DID1\"}","source_id":"source id","state":2,"proof_state":0,"proof":null,"proof_request":null,"revocation_interval":{}}"#;
        assert_eq!(vcx_proof_deserialize(cb.command_handle,
                                         CString::new(PROOF_OFFER_SENT).unwrap().into_raw(),
                                         Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let handle = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_proof_update_state() {
        init!("true");

        let (cb, rc) = create_proof_util();
        assert_eq!(rc, error::SUCCESS.code_num);
        let proof_handle = cb.receive(Some(Duration::from_secs(10))).unwrap();

        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_proof_update_state(cb.command_handle,
                                          proof_handle,
                                          Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let state = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert_eq!(state, VcxStateType::VcxStateInitialized as u32);
    }

    #[test]
    fn test_vcx_proof_send_request() {
        init!("true");

        let (cb, rc) = create_proof_util();
        assert_eq!(rc, error::SUCCESS.code_num);
        let proof_handle = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert_eq!(proof::get_state(proof_handle).unwrap(),VcxStateType::VcxStateInitialized as u32);

        let connection_handle = ::connection::tests::build_test_connection();
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_proof_send_request(cb.command_handle,
                                          proof_handle,
                                          connection_handle,
                                          Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();

        assert_eq!(proof::get_state(proof_handle).unwrap(),VcxStateType::VcxStateOfferSent as u32);

        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_proof_update_state_with_message(cb.command_handle,
                                                       proof_handle,
                                                       CString::new(PROOF_RESPONSE_STR).unwrap().into_raw(),
                                                       Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let state = cb.receive(Some(Duration::from_secs(10))).unwrap();

        assert_eq!(proof::get_state(proof_handle).unwrap(),VcxStateType::VcxStateAccepted as u32);
    }

    #[test]
    fn test_get_proof_fails_when_not_ready_with_proof() {
        init!("true");
        let (cb, rc) = create_proof_util();
        assert_eq!(rc, error::SUCCESS.code_num);
        let proof_handle = cb.receive(Some(Duration::from_secs(10))).unwrap();
        let connection_handle = connection::tests::build_test_connection();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB").unwrap();

        thread::sleep(Duration::from_millis(300));
        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        assert_eq!(vcx_get_proof(cb.command_handle,
                                 proof_handle,
                                 connection_handle,
                                 Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).is_err();
    }

    #[test]
    fn test_get_proof_returns_proof_with_proof_state_invalid() {
        init!("true");
        let connection_handle = connection::tests::build_test_connection();
        connection::set_pw_did(connection_handle, "XXFh7yBzrpJQmNyZzgoTqB").unwrap();
        let proof_handle = proof::from_string(PROOF_WITH_INVALID_STATE).unwrap();
        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        assert_eq!(vcx_get_proof(cb.command_handle,
                                 proof_handle,
                                 connection_handle,
                                 Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        let (state, _) = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert_eq!(state, ProofStateType::ProofInvalid as u32);
        vcx_proof_release(proof_handle);
        let unknown_handle = proof_handle + 1;
        assert_eq!(vcx_proof_release(unknown_handle), error::INVALID_PROOF_HANDLE.code_num);
    }

    #[test]
    fn test_vcx_connection_get_state() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let handle = proof::from_string(PROOF_OFFER_SENT).unwrap();
        assert!(handle > 0);
        let rc = vcx_proof_get_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        let state = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert_eq!(state, VcxStateType::VcxStateOfferSent as u32);
    }
}
