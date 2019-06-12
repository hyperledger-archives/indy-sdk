use serde_json;
use libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use connection;
use credential;
use std::ptr;
use utils::threadpool::spawn;
use error::prelude::*;

/// Retrieves Payment Info from a Credential
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides Payment Info of a Credential
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credential_get_payment_info(command_handle: u32,
                                              credential_handle: u32,
                                              cb: Option<extern fn(xcommand_handle: u32, err: u32, *const c_char)>) -> u32 {
    info!("vcx_credential_get_payment_info >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    spawn(move || {
        match credential::get_payment_information(credential_handle) {
            Ok(p) => {
                match p {
                    Some(p) => {
                        let info = p.to_string().unwrap_or("{}".to_string());
                        trace!("vcx_credential_get_payment_info(command_handle: {}, rc: {}, msg: {})", command_handle, error::SUCCESS.code_num, info.clone());
                        let msg = CStringUtils::string_to_cstring(info);
                        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
                    }
                    None => {
                        let msg = CStringUtils::string_to_cstring(format!("{{}}"));
                        trace!("vcx_credential_get_payment_info(command_handle: {}, rc: {}, msg: {})", command_handle, error::SUCCESS.code_num, "{}");
                        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
                    }
                }
            }
            Err(e) => {
                warn!("vcx_credential_get_payment_info(command_handle: {}, rc: {}, msg: {})",
                      command_handle, e, "{}");
                cb(command_handle, e.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Create a Credential object that requests and receives a credential for an institution
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's personal identification for the credential, should be unique.
///
/// offer: credential offer received via "vcx_credential_get_offers"
///
/// # Example offer -> "[{"msg_type": "CREDENTIAL_OFFER","version": "0.1","to_did": "...","from_did":"...","credential": {"account_num": ["...."],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "...","credential_name": "Account Certificate","credential_id": "3675417066","msg_ref_id": "ymy5nth"}]
///
/// cb: Callback that provides credential handle or error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credential_create_with_offer(command_handle: u32,
                                               source_id: *const c_char,
                                               offer: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {
    info!("vcx_credential_create_with_offer >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(offer, VcxErrorKind::InvalidOption);

    trace!("vcx_credential_create_with_offer(command_handle: {}, source_id: {}, offer: {})",
           command_handle, source_id, secret!(&offer));

    spawn(move || {
        match credential::credential_create_with_offer(&source_id, &offer) {
            Ok(x) => {
                trace!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                       command_handle, source_id, error::SUCCESS.message, x);
                cb(command_handle, error::SUCCESS.code_num, x)
            }
            Err(x) => {
                warn!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {})",
                      command_handle, source_id, x, 0);
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Retrieve information about a stored credential in user's wallet, including credential id and the credential itself.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides error status of api call, or returns the credential in json format of "{uuid:credential}".
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_get_credential(command_handle: u32,
                                 credential_handle: u32,
                                 cb: Option<extern fn(xcommand_handle: u32, err: u32, credential: *const c_char)>) -> u32 {
    info!("vcx_get_credential >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    if !credential::is_valid_handle(credential_handle) {
        return VcxError::from(VcxErrorKind::InvalidCredentialHandle).into()
    }

    let source_id = credential::get_source_id(credential_handle).unwrap_or_default();
    trace!("vcx_get_credential(command_handle: {}, credential_handle: {}) source_id: {})",
           command_handle, credential_handle, source_id);

    spawn(move || {
        match credential::get_credential(credential_handle) {
            Ok(s) => {
                trace!("vcx_get_credential_cb(commmand_handle: {}, rc: {}, msg: {}) source_id: {}",
                       command_handle, error::SUCCESS.code_num, s, source_id);
                let msg = CStringUtils::string_to_cstring(s);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(e) => {
                error!("vcx_get_credential_cb(commmand_handle: {}, rc: {}, msg: {}) source_id: {}",
                       command_handle, e, "".to_string(), source_id);
                cb(command_handle, e.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Create a Credential object that requests and receives a credential for an institution
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Institution's personal identification for the credential, should be unique.
///
/// connection_handle: connection to query for credential offer
///
/// msg_id: msg_id that contains the credential offer
///
/// cb: Callback that provides credential handle or error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_variables, unused_mut)]
pub extern fn vcx_credential_create_with_msgid(command_handle: u32,
                                               source_id: *const c_char,
                                               connection_handle: u32,
                                               msg_id: *const c_char,
                                               cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32, offer: *const c_char)>) -> u32 {
    info!("vcx_credential_create_with_msgid >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(msg_id, VcxErrorKind::InvalidOption);

    trace!("vcx_credential_create_with_msgid(command_handle: {}, source_id: {}, connection_handle: {}, msg_id: {})",
           command_handle, source_id, connection_handle, msg_id);

    spawn(move || {
        match credential::get_credential_offer_msg(connection_handle, &msg_id) {
            Ok(offer) => {
                match credential::credential_create_with_offer(&source_id, &offer) {
                    Ok(handle) => {
                        let offer_string = match credential::get_credential_offer(handle) {
                            Ok(x) => x,
                            Err(_) => offer,
                        };
                        let c_offer = CStringUtils::string_to_cstring(offer_string);
                        trace!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {}) source_id: {}",
                               command_handle, source_id, error::SUCCESS.message, handle, source_id);
                        cb(command_handle, error::SUCCESS.code_num, handle, c_offer.as_ptr())
                    }
                    Err(e) => {
                        warn!("vcx_credential_create_with_offer_cb(command_handle: {}, source_id: {}, rc: {}, handle: {}) source_id: {}",
                              command_handle, source_id, e, 0, source_id);
                        cb(command_handle, e.into(), 0, ptr::null_mut());
                    }
                };
            }
            Err(e) => cb(command_handle, e.into(), 0, ptr::null_mut()),
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Send a credential request to the connection, called after having received a credential offer
///
/// #params
/// command_handle: command handle to map callback to user context
///
/// credential_handle: credential handle that was provided during creation. Used to identify credential object
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides error status of credential request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credential_send_request(command_handle: u32,
                                          credential_handle: u32,
                                          connection_handle: u32,
                                          payment_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32)>) -> u32 {
    info!("vcx_credential_send_request >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !credential::is_valid_handle(credential_handle) {
        return VcxError::from(VcxErrorKind::InvalidCredentialHandle).into()
    }

    if !connection::is_valid_handle(connection_handle) {
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    let source_id = credential::get_source_id(credential_handle).unwrap_or_default();
    trace!("vcx_credential_send_request(command_handle: {}, credential_handle: {}, connection_handle: {}), source_id: {:?}",
           command_handle, credential_handle, connection_handle, source_id);

    spawn(move || {
        match credential::send_credential_request(credential_handle, connection_handle) {
            Ok(x) => {
                trace!("vcx_credential_send_request_cb(command_handle: {}, rc: {}) source_id: {}",
                       command_handle, x.to_string(), source_id);
                cb(command_handle, x);
            }
            Err(e) => {
                warn!("vcx_credential_send_request_cb(command_handle: {}, rc: {}) source_id: {}",
                      command_handle, e, source_id);
                cb(command_handle, e.into());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Queries agency for credential offers from the given connection.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection to query for credential offers.
///
/// cb: Callback that provides any credential offers and error status of query
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credential_get_offers(command_handle: u32,
                                        connection_handle: u32,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_offers: *const c_char)>) -> u32 {
    info!("vcx_credential_get_offers >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !connection::is_valid_handle(connection_handle) {
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    trace!("vcx_credential_get_offers(command_handle: {}, connection_handle: {})",
           command_handle, connection_handle);

    spawn(move || {
        match credential::get_credential_offer_messages(connection_handle) {
            Ok(x) => {
                trace!("vcx_credential_get_offers_cb(command_handle: {}, rc: {}, msg: {})",
                       command_handle, x.to_string(), x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                error!("vcx_credential_get_offers_cb(command_handle: {}, rc: {}, msg: null)",
                       command_handle, x);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks for any state change in the credential and updates the the state attribute.  If it detects a credential it
/// will store the credential in the wallet and update the state.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_handle: Credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides most current state of the credential and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credential_update_state(command_handle: u32,
                                          credential_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_credential_update_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !credential::is_valid_handle(credential_handle) {
        return VcxError::from(VcxErrorKind::InvalidCredentialHandle).into()
    }

    let source_id = credential::get_source_id(credential_handle).unwrap_or_default();
    trace!("vcx_credential_update_state(command_handle: {}, credential_handle: {}), source_id: {:?}",
           command_handle, credential_handle, source_id);

    spawn(move || {
        match credential::update_state(credential_handle) {
            Ok(_) => (),
            Err(e) => {
                error!("vcx_credential_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, e, 0, source_id);
                cb(command_handle, e.into(), 0)
            }
        }

        let state = match credential::get_state(credential_handle) {
            Ok(s) => {
                trace!("vcx_credential_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, error::SUCCESS.message, s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            }
            Err(e) => {
                error!("vcx_credential_update_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, e, 0, source_id);
                cb(command_handle, e.into(), 0)
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Get the current state of the credential object
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Credential handle that was provided during creation.
///
/// cb: Callback that provides most current state of the credential and error status of request
///
/// #Returns
#[no_mangle]
pub extern fn vcx_credential_get_state(command_handle: u32,
                                       handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_credential_get_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !credential::is_valid_handle(handle) {
        return VcxError::from(VcxErrorKind::InvalidCredentialHandle).into()
    }

    let source_id = credential::get_source_id(handle).unwrap_or_default();
    trace!("vcx_credential_get_state(command_handle: {}, credential_handle: {}), source_id: {:?}",
           command_handle, handle, source_id);

    spawn(move || {
        match credential::get_state(handle) {
            Ok(s) => {
                trace!("vcx_credential_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, error::SUCCESS.message, s, source_id);
                cb(command_handle, error::SUCCESS.code_num, s)
            }
            Err(e) => {
                error!("vcx_credential_get_state_cb(command_handle: {}, rc: {}, state: {}), source_id: {:?}",
                       command_handle, e, 0, source_id);
                cb(command_handle, e.into(), 0)
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Takes the credential object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// handle: Credential handle that was provided during creation. Used to identify credential object
///
/// cb: Callback that provides json string of the credential's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credential_serialize(command_handle: u32,
                                       handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, data: *const c_char)>) -> u32 {
    info!("vcx_credential_serialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !credential::is_valid_handle(handle) {
        return VcxError::from(VcxErrorKind::InvalidCredentialHandle).into()
    }

    let source_id = credential::get_source_id(handle).unwrap_or_default();
    trace!("vcx_credential_serialize(command_handle: {}, credential_handle: {}), source_id: {:?}",
           command_handle, handle, source_id);

    spawn(move || {
        match credential::to_string(handle) {
            Ok(x) => {
                trace!("vcx_credential_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                       command_handle, error::SUCCESS.message, x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                error!("vcx_credential_serialize_cb(command_handle: {}, rc: {}, data: {}), source_id: {:?}",
                       command_handle, x, 0, source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing an credential object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// credential_data: json string representing a credential object
///
///
/// cb: Callback that provides credential handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_credential_deserialize(command_handle: u32,
                                         credential_data: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, handle: u32)>) -> u32 {
    info!("vcx_credential_deserialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(credential_data, VcxErrorKind::InvalidOption);

    trace!("vcx_credential_deserialize(command_handle: {}, credential_data: {})",
           command_handle, credential_data);

    spawn(move || {
        match credential::from_string(&credential_data) {
            Ok(x) => {
                trace!("vcx_credential_deserialize_cb(command_handle: {}, rc: {}, credential_handle: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, x, credential::get_source_id(x).unwrap_or_default());

                cb(command_handle, error::SUCCESS.code_num, x);
            }
            Err(x) => {
                error!("vcx_credential_deserialize_cb(command_handle: {}, rc: {}, credential_handle: {}) source_id: {}",
                       command_handle, x, 0, "");
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Releases the credential object by de-allocating memory
///
/// #Params
/// handle: Credential handle that was provided during creation. Used to access credential object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_credential_release(handle: u32) -> u32 {
    info!("vcx_credential_release >>>");

    let source_id = credential::get_source_id(handle).unwrap_or_default();
    match credential::release(handle) {
        Ok(_) => {
            trace!("vcx_credential_release(handle: {}, rc: {}), source_id: {:?}",
                   handle, error::SUCCESS.message, source_id);
            error::SUCCESS.code_num
        }

        Err(e) => {
            error!("vcx_credential_release(handle: {}, rc: {}), source_id: {:?}",
                   handle, e, source_id);
            e.into()
        }
    }
}

/// Retrieve the txn associated with paying for the credential
///
/// #param
/// handle: credential handle that was provided during creation.  Used to access credential object.
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
pub extern fn vcx_credential_get_payment_txn(command_handle: u32,
                                             handle: u32,
                                             cb: Option<extern fn(xcommand_handle: u32, err: u32, txn: *const c_char)>) -> u32 {
    info!("vcx_credential_get_payment_txn >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = credential::get_source_id(handle).unwrap_or_default();
    trace!("vcx_credential_get_payment_txn(command_handle: {}) source_id: {}", command_handle, source_id);

    spawn(move || {
        match credential::get_payment_txn(handle) {
            Ok(x) => {
                match serde_json::to_string(&x) {
                    Ok(x) => {
                        trace!("vcx_credential_get_payment_txn_cb(command_handle: {}, rc: {}, : {}), source_id: {}",
                               command_handle, error::SUCCESS.message, x, credential::get_source_id(handle).unwrap_or_default());

                        let msg = CStringUtils::string_to_cstring(x);
                        cb(command_handle, 0, msg.as_ptr());
                    }
                    Err(e) => {
                        let err = VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize payment txn: {:?}", e));
                        error!("vcx_credential_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {}",
                               command_handle, err, "null", credential::get_source_id(handle).unwrap_or_default());
                        cb(command_handle, err.into(), ptr::null_mut());
                    }
                }
            }
            Err(x) => {
                error!("vcx_credential_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {}",
                       command_handle, x, "null", credential::get_source_id(handle).unwrap_or_default());
                cb(command_handle, x.into(), ptr::null());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use std::ffi::CString;
    use std::time::Duration;
    use connection;
    use api::VcxStateType;
    use api::return_types_u32;
    use serde_json::Value;
    use utils::constants::{DEFAULT_SERIALIZED_CREDENTIAL, DEFAULT_SERIALIZE_VERSION};

    pub const BAD_CREDENTIAL_OFFER: &str = r#"{"version": "0.1","to_did": "LtMgSjtFcyPwenK9SHCyb8","from_did": "LtMgSjtFcyPwenK9SHCyb8","credential": {"account_num": ["8BEaoLf8TBmK4BUyX8WWnA"],"name_on_account": ["Alice"]},"schema_seq_no": 48,"issuer_did": "Pd4fnFtRBcMKRVC2go5w3j","credential_name": "Account Certificate","credential_id": "3675417066","msg_ref_id": "ymy5nth"}"#;

    #[test]
    fn test_vcx_credential_create_with_offer_success() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credential_create_with_offer(cb.command_handle,
                                                    CString::new("test_create").unwrap().into_raw(),
                                                    CString::new(::utils::constants::CREDENTIAL_OFFER_JSON).unwrap().into_raw(),
                                                    Some(cb.get_callback())), error::SUCCESS.code_num);
        assert!(cb.receive(Some(Duration::from_secs(10))).unwrap() > 0);
    }

    #[test]
    fn test_vcx_credential_create_with_offer_fails() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credential_create_with_offer(cb.command_handle,
                                                    CString::new("test_create").unwrap().into_raw(),
                                                    CString::new(BAD_CREDENTIAL_OFFER).unwrap().into_raw(),
                                                    Some(cb.get_callback())), error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(), Some(error::INVALID_JSON.code_num));
    }

    #[test]
    fn test_vcx_credential_serialize_and_deserialize() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let handle = credential::credential_create_with_offer("test_vcx_credential_serialize", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        assert_eq!(vcx_credential_serialize(cb.command_handle,
                                            handle,
                                            Some(cb.get_callback())), error::SUCCESS.code_num);
        let s = cb.receive(Some(Duration::from_secs(2))).unwrap().unwrap();
        let j: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(j["version"], DEFAULT_SERIALIZE_VERSION);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credential_deserialize(cb.command_handle,
                                              CString::new(s).unwrap().into_raw(),
                                              Some(cb.get_callback())), error::SUCCESS.code_num);
        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert!(handle > 0);
    }

    #[test]
    fn test_vcx_credential_send_request() {
        init!("true");
        let handle = credential::credential_create_with_offer("test_send_request", ::utils::constants::CREDENTIAL_OFFER_JSON).unwrap();
        assert_eq!(credential::get_state(handle).unwrap(), VcxStateType::VcxStateRequestReceived as u32);

        let connection_handle = connection::tests::build_test_connection();
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_credential_send_request(cb.command_handle, handle, connection_handle, 0, Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_credential_get_new_offers() {
        init!("true");
        let cxn = ::connection::tests::build_test_connection();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_credential_get_offers(cb.command_handle,
                                             cxn,
                                             Some(cb.get_callback())),
                   error::SUCCESS.code_num as u32);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_credential_create() {
        init!("true");
        let cxn = ::connection::tests::build_test_connection();
        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        assert_eq!(vcx_credential_create_with_msgid(cb.command_handle,
                                                    CString::new("test_vcx_credential_create").unwrap().into_raw(),
                                                    cxn,
                                                    CString::new("123").unwrap().into_raw(),
                                                    Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_credential_get_state() {
        init!("true");
        let handle = credential::from_string(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        assert!(handle > 0);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_credential_get_state(cb.command_handle, handle, Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateRequestReceived as u32);
    }

    #[test]
    fn test_vcx_credential_update_state() {
        init!("true");
        let cxn = ::connection::tests::build_test_connection();
        ::connection::connect(cxn, None).unwrap();
        let handle = credential::from_string(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        ::utils::httpclient::set_next_u8_response(::utils::constants::NEW_CREDENTIAL_OFFER_RESPONSE.to_vec());
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        assert_eq!(vcx_credential_update_state(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateRequestReceived as u32);
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_credential_send_request(cb.command_handle, handle, cxn, 0, Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_get_credential() {
        use utils::constants::FULL_CREDENTIAL_SERIALIZED;
        init!("true");
        let handle = credential::from_string(FULL_CREDENTIAL_SERIALIZED).unwrap();
        let bad_handle = 1123;
        let command_handle = 1111;
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_get_credential(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap().unwrap();

        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_get_credential(cb.command_handle, bad_handle, Some(cb.get_callback())), error::INVALID_CREDENTIAL_HANDLE.code_num);

        let handle = credential::from_string(DEFAULT_SERIALIZED_CREDENTIAL).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_get_credential(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        use utils::error::INVALID_STATE;
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).err(), Some(INVALID_STATE.code_num));
    }

    #[test]
    fn test_get_payment_txn() {
        init!("true");

        let handle = credential::from_string(::utils::constants::FULL_CREDENTIAL_SERIALIZED).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        vcx_credential_get_payment_txn(cb.command_handle, handle, Some(cb.get_callback()));
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_vcx_credential_release() {
        init!("true");
        let handle = credential::from_string(::utils::constants::FULL_CREDENTIAL_SERIALIZED).unwrap();
        let unknown_handle = handle + 1;
        assert_eq!(vcx_credential_release(unknown_handle), error::INVALID_CREDENTIAL_HANDLE.code_num);
    }
}
