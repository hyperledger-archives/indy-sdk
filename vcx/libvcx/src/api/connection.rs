use libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::threadpool::spawn;
use std::ptr;
use connection::{get_source_id, create_connection, create_connection_with_invite, connect, to_string, get_state, release, is_valid_handle, update_state, from_string, get_invite_details, delete_connection, process_acceptance_message};
use error::prelude::*;
use messages::get_message::Message;

/// Delete a Connection object and release its handle
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: handle of the connection to delete.
///
/// cb: Callback that provides feedback of the api call.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_assignments)]
pub extern fn vcx_connection_delete_connection(command_handle: u32,
                                               connection_handle: u32,
                                               cb: Option<extern fn(
                                                   xcommand_handle: u32,
                                                   err: u32)>) -> u32 {
    info!("vcx_delete_connection >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !is_valid_handle(connection_handle) {
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }
    trace!("vcx_connection_delete_connection(command_handle: {}, connection_handle: {})", command_handle, connection_handle);
    spawn(move|| {
        match delete_connection(connection_handle) {
            Ok(_) => {
                trace!("vcx_connection_delete_connection_cb(command_handle: {}, rc: {})", command_handle, error::SUCCESS.message);
                cb(command_handle, error::SUCCESS.code_num);
            },
            Err(e) => {
                trace!("vcx_connection_delete_connection_cb(command_handle: {}, rc: {})", command_handle, e);
                cb(command_handle, e.into());
            },
        }

        Ok(())
    });

    error::SUCCESS.code_num
}

/// -> Create a Connection object that provides a pairwise connection for an institution's user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user
///
/// cb: Callback that provides connection handle and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
#[allow(unused_assignments)]
pub extern fn vcx_connection_create(command_handle: u32,
                                    source_id: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: u32, err: u32, connection_handle: u32)>) -> u32 {
    info!("vcx_connection_create >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);

    trace!("vcx_connection_create(command_handle: {}, source_id: {})", command_handle, source_id);

    spawn(move|| {
        match create_connection(&source_id) {
            Ok(handle) => {
                trace!("vcx_connection_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, error::SUCCESS.message, handle, source_id);
                cb(command_handle, error::SUCCESS.code_num, handle);
            },
            Err(x) => {
                warn!("vcx_connection_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x, 0, source_id);
                cb(command_handle, x.into(), 0);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Create a Connection object from the given invite_details that provides a pairwise connection.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: institution's personal identification for the user
///
/// invite_details: Provided via the other end of the connection calling "vcx_connection_connect" or "vcx_connection_invite_details"
///
/// cb: Callback that provides connection handle and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_create_with_invite(command_handle: u32,
                                                source_id: *const c_char,
                                                invite_details: *const c_char,
                                                cb: Option<extern fn(xcommand_handle: u32, err: u32, credential_handle: u32)>) -> u32 {
    info!("vcx_connection_create_with_invite >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(invite_details, VcxErrorKind::InvalidOption);
    trace!("vcx_connection_create_with_invite(command_handle: {}, source_id: {})", command_handle, source_id);
    spawn(move|| {
        match create_connection_with_invite(&source_id, &invite_details) {
            Ok(handle) => {
                trace!("vcx_connection_create_with_invite_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, error::SUCCESS.message, handle, source_id);
                cb(command_handle, error::SUCCESS.code_num, handle);
            },
            Err(x) => {
                warn!("vcx_connection_create_with_invite_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x, 0, source_id);
                cb(command_handle, x.into(), 0);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Establishes connection between institution and its user
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection handle that identifies connection object
///
/// connection_options: Provides details indicating if the connection will be established by text or QR Code
///
/// # Examples connection_options -> "{"connection_type":"SMS","phone":"123","use_public_did":true}" OR: "{"connection_type":"QR","phone":"","use_public_did":false}"
///
/// cb: Callback that provides error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_connect(command_handle:u32,
                                     connection_handle: u32,
                                     connection_options: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, invite_details: *const c_char)>) -> u32 {
    info!("vcx_connection_connect >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    let options = if !connection_options.is_null() {
        check_useful_opt_c_str!(connection_options, VcxErrorKind::InvalidOption);
        connection_options.to_owned()
    }
    else {
        None
    };

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    trace!("vcx_connection_connect(command_handle: {}, connection_handle: {}, connection_options: {:?}), source_id: {:?}",
          command_handle, connection_handle, options, source_id);

    spawn(move|| {
        match connect(connection_handle, options) {
            Ok(_) => {
                match get_invite_details(connection_handle,true) {
                    Ok(x) => {
                        trace!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                              command_handle, connection_handle, error::SUCCESS.message, x, source_id);
                        let msg = CStringUtils::string_to_cstring(x);
                        cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
                    },
                    Err(e) => {
                        warn!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                              command_handle, connection_handle, error::SUCCESS.message, "null", source_id); // TODO: why Success?????
                        cb(command_handle, error::SUCCESS.code_num, ptr::null_mut());
                    },
                }
            },
            Err(x) => {
                warn!("vcx_connection_connect_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}, source_id: {})",
                      command_handle, connection_handle, x, "null", source_id);
                cb(command_handle,x.into(), ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the Connection object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: Connection handle that identifies pairwise connection
///
/// cb: Callback that provides json string of the connection's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_serialize(command_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, serialized_data: *const c_char)>) -> u32 {
    info!("vcx_connection_serialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    trace!("vcx_connection_serialize(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    spawn(move|| {
        match to_string(connection_handle) {
            Ok(json) => {
                trace!("vcx_connection_serialize_cb(command_handle: {}, connection_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, connection_handle, error::SUCCESS.message, json, source_id);
                let msg = CStringUtils::string_to_cstring(json);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_connection_serialize_cb(command_handle: {}, connection_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, connection_handle, x, "null", source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a connection object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_data: json string representing a connection object
///
/// cb: Callback that provides credential handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_deserialize(command_handle: u32,
                                      connection_data: *const c_char,
                                      cb: Option<extern fn(xcommand_handle: u32, err: u32, connection_handle: u32)>) -> u32 {
    info!("vcx_connection_deserialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(connection_data, VcxErrorKind::InvalidOption);

    trace!("vcx_connection_deserialize(command_handle: {}, connection_data: {})", command_handle, connection_data);

    spawn(move|| {
        let (rc, handle) = match from_string(&connection_data) {
            Ok(x) => {
                let source_id = get_source_id(x).unwrap_or_default();
                trace!("vcx_connection_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error::SUCCESS.message, x, source_id);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_connection_deserialize_cb(command_handle: {}, rc: {}, handle: {} )",
                      command_handle, x, 0);
                (x.into(), 0)
            },
        };

        cb(command_handle, rc, handle);

        Ok(())
    });

    error::SUCCESS.code_num
}


/// Checks for any state change in the connection and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: was provided during creation. Used to identify connection object
///
/// cb: Callback that provides most current state of the credential and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_update_state(command_handle: u32,
                                          connection_handle: u32,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_connection_update_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    trace!("vcx_connection_update_state(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    spawn(move|| {
        let rc = match update_state(connection_handle, None) {
            Ok(x) => {
                trace!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      command_handle, error::SUCCESS.message, connection_handle, get_state(connection_handle), source_id);
                x
            },
            Err(x) => {
                warn!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      command_handle, x, connection_handle, get_state(connection_handle), source_id);
                x.into()
            },
        };
        let state = get_state(connection_handle);
        cb(command_handle, rc, state);

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks the message any state change and updates the the state attribute
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: was provided during creation. Used to identify connection object
///
/// message: message to process
///
/// cb: Callback that provides most current state of the credential and error status of request
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_update_state_with_message(command_handle: u32,
                                          connection_handle: u32,
                                          message: *const c_char,
                                          cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_connection_update_state_with_message >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(message, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    trace!("vcx_connection_update_state(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    let message: Message = match serde_json::from_str(&message) {
        Ok(x) => x,
        Err(_) => return VcxError::from(VcxErrorKind::InvalidJson).into(),
    };

    spawn(move|| {
        let rc = match process_acceptance_message(connection_handle, message) {
            Ok(x) => {
                trace!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      command_handle, error::SUCCESS.message, connection_handle, get_state(connection_handle), source_id);
                x
            },
            Err(x) => {
                warn!("vcx_connection_update_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
                      command_handle, x, connection_handle, get_state(connection_handle), source_id);
                x.into()
            },
        };
        let state = get_state(connection_handle);
        cb(command_handle, rc, state);

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Get the current state of the connection object
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// proof_handle: Connection handle that was provided during creation. Used to access connection object
///
/// cb: Callback that provides most current state of the connection and error status of request
///
/// #Returns
#[no_mangle]
pub extern fn vcx_connection_get_state(command_handle: u32,
                                       connection_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: u32, err: u32, state: u32)>) -> u32 {
    info!("vcx_connection_get_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    trace!("vcx_connection_get_state(command_handle: {}, connection_handle: {}), source_id: {:?}",
          command_handle, connection_handle, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    spawn(move|| {
        trace!("vcx_connection_get_state_cb(command_handle: {}, rc: {}, connection_handle: {}, state: {}), source_id: {:?}",
              command_handle, error::SUCCESS.message, connection_handle, get_state(connection_handle), source_id);
        cb(command_handle, error::SUCCESS.code_num, get_state(connection_handle));

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Gets the current connection details
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: was provided during creation. Used to identify connection object
///
/// abbreviated: abbreviated connection details for QR codes or not
///
/// cb: Callback that provides the json string of details
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_invite_details(command_handle: u32,
                                            connection_handle: u32,
                                            abbreviated: bool,
                                            cb: Option<extern fn(xcommand_handle: u32, err: u32, details: *const c_char)>) -> u32 {
    info!("vcx_connection_invite_details >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    trace!("vcx_connection_invite_details(command_handle: {}, connection_handle: {}, abbreviated: {}), source_id: {:?}",
          command_handle, connection_handle, abbreviated, source_id);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_get_state - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    spawn(move|| {
        match get_invite_details(connection_handle, abbreviated){
            Ok(str) => {
                trace!("vcx_connection_invite_details_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}), source_id: {:?}",
                      command_handle, connection_handle, error::SUCCESS.message, str, source_id);
                let msg = CStringUtils::string_to_cstring(str);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_connection_invite_details_cb(command_handle: {}, connection_handle: {}, rc: {}, details: {}, source_id: {:?})",
                      command_handle, connection_handle, x, "null", source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Send a message to the specified connection
///
/// #params
///
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: connection to receive the message
///
/// msg: actual message to send
///
/// send_msg_options:
///     {
///         msg_type: String, // type of message to send
///         msg_title: String, // message title (user notification)
///         ref_msg_id: Option<String>, // If responding to a message, id of the message
///     }
///
/// cb: Callback that provides array of matching messages retrieved
///
/// #Returns
/// Error code as a u32

#[no_mangle]
pub extern fn vcx_connection_send_message(command_handle: u32,
                               connection_handle: u32,
                               msg: *const c_char,
                               send_msg_options: *const c_char,
                               cb: Option<extern fn(xcommand_handle: u32, err: u32, msg_id: *const c_char)>) -> u32 {
    info!("vcx_message_send >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(msg, VcxErrorKind::InvalidOption);
    check_useful_c_str!(send_msg_options, VcxErrorKind::InvalidOption);

    trace!("vcx_message_send(command_handle: {}, connection_handle: {}, msg: {}, send_msg_options: {})",
           command_handle, connection_handle, msg, send_msg_options);

    spawn(move|| {
        match ::messages::send_message::send_generic_message(connection_handle, &msg, &send_msg_options) {
            Ok(x) => {
                trace!("vcx_connection_send_message_cb(command_handle: {}, rc: {}, msg_id: {})",
                    command_handle, error::SUCCESS.message, x);

                let msg_id = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg_id.as_ptr());
            },
            Err(e) => {
                warn!("vcx_connection_send_message_cb(command_handle: {}, rc: {})",
                      command_handle, e);

                cb(command_handle, e.into(), ptr::null_mut());
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Generate a signature for the specified data
///
/// #params
///
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: connection to receive the message
///
/// data_raw: raw data buffer for signature
///
/// data:len: length of data buffer
///
/// cb: Callback that provides the generated signature
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_sign_data(command_handle: u32,
                                  connection_handle: u32,
                                  data_raw: *const u8,
                                  data_len: u32,
                                  cb: Option<extern fn(command_handle_: u32,
                                                       err: u32,
                                                       signature_raw: *const u8,
                                                       signature_len: u32)>) -> u32  {
    trace!("vcx_connection_sign_data: >>> connection_handle: {}, data_raw: {:?}, data_len: {}",
           connection_handle, data_raw, data_len);

    check_useful_c_byte_array!(data_raw, data_len, VcxErrorKind::InvalidOption, VcxErrorKind::InvalidOption);
    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    trace!("vcx_connection_sign_data: entities >>> connection_handle: {}, data_raw: {:?}, data_len: {}",
           connection_handle, data_raw, data_len);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_sign - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    let vk = match ::connection::get_pw_verkey(connection_handle) {
        Ok(x) => x,
        Err(e) => return e.into(),
    };

    spawn (move || {
        match ::utils::libindy::crypto::sign(&vk, &data_raw) {
            Ok(x) => {
                trace!("vcx_connection_sign_data_cb(command_handle: {}, connection_handle: {}, rc: {}, signature: {:?})",
                       command_handle, connection_handle, error::SUCCESS.message, x);

                let (signature_raw, signature_len) = ::utils::cstring::vec_to_pointer(&x);
                cb(command_handle, error::SUCCESS.code_num, signature_raw, signature_len);
            },
            Err(e) => {
                warn!("vcx_messages_sign_data_cb(command_handle: {}, rc: {}, signature: null)",
                      command_handle, e);

                cb(command_handle, e.into(), ptr::null_mut(), 0);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Verify the signature is valid for the specified data
///
/// #params
///
/// command_handle: command handle to map callback to user context.
///
/// connection_handle: connection to receive the message
///
/// data_raw: raw data buffer for signature
///
/// data_len: length of data buffer
///
/// signature_raw: raw data buffer for signature
///
/// signature_len: length of data buffer
///
/// cb: Callback that specifies whether the signature was valid or not
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_connection_verify_signature(command_handle: u32,
                                  connection_handle: u32,
                                  data_raw: *const u8,
                                  data_len: u32,
                                  signature_raw: *const u8,
                                  signature_len: u32,
                                  cb: Option<extern fn(command_handle_: u32,
                                                       err: u32,
                                                       valid: bool)>) -> u32 {
    trace!("vcx_connection_verify_signature: >>> connection_handle: {}, data_raw: {:?}, data_len: {}, signature_raw: {:?}, signature_len: {}",
           connection_handle, data_raw, data_len, signature_raw, signature_len);

    check_useful_c_byte_array!(data_raw, data_len, VcxErrorKind::InvalidOption, VcxErrorKind::InvalidOption);
    check_useful_c_byte_array!(signature_raw, signature_len, VcxErrorKind::InvalidOption, VcxErrorKind::InvalidOption);
    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    trace!("vcx_connection_verify_signature: entities >>> connection_handle: {}, data_raw: {:?}, data_len: {}, signature_raw: {:?}, signature_len: {}",
           connection_handle, data_raw, data_len, signature_raw, signature_len);

    if !is_valid_handle(connection_handle) {
        error!("vcx_connection_verify_signature - invalid handle");
        return VcxError::from(VcxErrorKind::InvalidConnectionHandle).into()
    }

    let vk = match ::connection::get_their_pw_verkey(connection_handle) {
        Ok(x) => x,
        Err(e) => return e.into(),
    };

    spawn (move || {
        match ::utils::libindy::crypto::verify(&vk, &data_raw, &signature_raw) {
            Ok(x) => {
                trace!("vcx_connection_verify_signature_cb(command_handle: {}, rc: {}, valid: {})",
                       command_handle, error::SUCCESS.message, x);

                cb(command_handle, error::SUCCESS.code_num, x);
            },
            Err(e) => {
                warn!("vcx_connection_verify_signature_cb(command_handle: {}, rc: {}, valid: {})",
                      command_handle, e, false);

                cb(command_handle, e.into(), false);
            },
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Releases the connection object by de-allocating memory
///
/// #Params
/// connection_handle: was provided during creation. Used to identify connection object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_connection_release(connection_handle: u32) -> u32 {
    info!("vcx_connection_release >>>");

    let source_id = get_source_id(connection_handle).unwrap_or_default();
    match release(connection_handle) {
        Ok(_) => {
            trace!("vcx_connection_release(connection_handle: {}, rc: {}), source_id: {:?}",
                       connection_handle, error::SUCCESS.message, source_id);
            error::SUCCESS.code_num
        },
        Err(e) => {
            warn!("vcx_connection_release(connection_handle: {}), rc: {}), source_id: {:?}",
                        connection_handle, e, source_id);
            e.into()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;
    use connection::tests::build_test_connection;
    use utils::error;
    use std::time::Duration;
    use api::{return_types_u32, VcxStateType};
    use utils::httpclient;
    use utils::constants::{GET_MESSAGES_RESPONSE, INVITE_ACCEPTED_RESPONSE};
    use utils::error::SUCCESS;

    #[test]
    fn test_vcx_connection_create() {
        init!("true");
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_connection_create(cb.command_handle,
                                       CString::new("test_create").unwrap().into_raw(),
                                       Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert!(cb.receive(Some(Duration::from_secs(10))).unwrap() > 0);
    }

    #[test]
    fn test_vcx_connection_create_fails() {
        init!("true");
        let rc = vcx_connection_create(0,
                                       CString::new("test_create_fails").unwrap().into_raw(),
                                       None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_connection_create(cb.command_handle,
                                       ptr::null(),
                                       Some(cb.get_callback()));
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    #[test]
    fn test_vcx_connection_connect() {
        init!("true");
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_connect(cb.command_handle, 0, CString::new("{}").unwrap().into_raw(),Some(cb.get_callback()));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
        let handle = build_test_connection();
        assert!(handle > 0);
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_connect(cb.command_handle,handle, CString::new("{}").unwrap().into_raw(),Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        let invite_details = cb.receive(Some(Duration::from_secs(10))).unwrap();
        assert!(invite_details.is_some());
    }

    #[test]
    fn test_vcx_connection_update_state() {
        init!("true");
        let handle = build_test_connection();
        assert!(handle > 0);
        connect(handle,None).unwrap();
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        httpclient::set_next_u8_response(GET_MESSAGES_RESPONSE.to_vec());
        let rc = vcx_connection_update_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateAccepted as u32);
    }

    #[test]
    fn test_vcx_connection_update_state_with_message() {
        init!("true");
        let handle = build_test_connection();
        assert!(handle > 0);
        connect(handle,None).unwrap();
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_connection_update_state_with_message(cb.command_handle,handle,CString::new(INVITE_ACCEPTED_RESPONSE).unwrap().into_raw(), Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateAccepted as u32);
    }

    #[test]
    fn test_vcx_connection_update_state_fails() {
        init!("true");
        let rc = vcx_connection_update_state(0,0,None);
        assert_eq!(rc, error::INVALID_OPTION.code_num);
    }

    #[test]
    fn test_vcx_connection_serialize() {
        init!("true");
        let handle = build_test_connection();
        assert!(handle > 0);

        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_serialize(cb.command_handle,handle, Some(cb.get_callback()));
        assert_eq!(rc, 0);

        // unwraps on the option, if none, then serializing failed and panic! ensues.
        cb.receive(Some(Duration::from_secs(10))).unwrap().unwrap();
    }

    #[test]
    fn test_vcx_connection_release() {
        init!("true");
        let handle = build_test_connection();
        assert!(handle > 0);

        let rc = vcx_connection_release(handle);
        assert_eq!(rc, error::SUCCESS.code_num);
        let unknown_handle = handle + 1;
        assert_eq!(vcx_connection_release(unknown_handle), error::INVALID_CONNECTION_HANDLE.code_num);
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let rc = vcx_connection_connect(0,handle, CString::new("{}").unwrap().into_raw(),Some(cb.get_callback()));
        assert_eq!(rc, error::INVALID_CONNECTION_HANDLE.code_num);
    }

    #[test]
    fn test_vcx_connection_deserialize_succeeds() {
        init!("true");
        let string = ::utils::constants::DEFAULT_CONNECTION;
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let err = vcx_connection_deserialize(cb.command_handle,
                                                CString::new(string).unwrap().into_raw(),
                                                Some(cb.get_callback()));
        assert_eq!(err, SUCCESS.code_num);
        let handle = cb.receive(Some(Duration::from_secs(2))).unwrap();
        assert!(handle>0);
    }

    #[test]
    fn test_vcx_connection_get_state() {
        init!("true");
        let handle = build_test_connection();
        assert!(handle > 0);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        connect(handle, None).unwrap();
        httpclient::set_next_u8_response(GET_MESSAGES_RESPONSE.to_vec());
        let rc = vcx_connection_update_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateAccepted as u32);
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_connection_get_state(cb.command_handle,handle,Some(cb.get_callback()));
        assert_eq!(rc, error::SUCCESS.code_num);
        assert_eq!(cb.receive(Some(Duration::from_secs(10))).unwrap(), VcxStateType::VcxStateAccepted as u32)
    }

    #[test]
    fn test_vcx_connection_delete_connection() {
        init!("true");
        let connection_handle = build_test_connection();
        connect(connection_handle, Some("{}".to_string())).unwrap();
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(vcx_connection_delete_connection(cb.command_handle, connection_handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    #[test]
    fn test_send_message() {
        init!("true");

        let msg = CString::new("MESSAGE").unwrap().into_raw();
        let send_msg_options = CString::new(json!({"msg_type":"type", "msg_title": "title", "ref_msg_id":null}).to_string()).unwrap().into_raw();
        let connection_handle = ::connection::tests::build_test_connection();
        ::connection::set_state(connection_handle, VcxStateType::VcxStateAccepted).unwrap();
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_connection_send_message(cb.command_handle, connection_handle, msg, send_msg_options, Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    extern "C" fn test_sign_cb(command_handle: u32, error: u32, signature: *const u8, signature_length: u32) {
        assert_eq!(error, error::SUCCESS.code_num);
    }

    #[test]
    fn test_sign() {
        use std::thread;
        init!("true");

        let msg = format!("My message");;
        let msg_len = msg.len();

        let connection_handle = ::connection::tests::build_test_connection();
        assert_eq!(vcx_connection_sign_data(0, connection_handle, CString::new(msg).unwrap().as_ptr() as *const u8, msg_len as u32, Some(test_sign_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(2));
    }

    extern "C" fn test_verify_cb(command_handle: u32, error: u32, valid: bool) {
        assert_eq!(valid, true);
    }

    #[test]
    fn test_verify_signature() {
        use std::thread;
        init!("true");

        let msg = format!("My message");
        let msg_len = msg.len();

        let signature = format!("signature");
        let signature_length = signature.len();

        let connection_handle = ::connection::tests::build_test_connection();
        assert_eq!(vcx_connection_verify_signature(0,
                                                   connection_handle,
                                                   CString::new(msg).unwrap().as_ptr() as *const u8,
                                                   msg_len as u32,
                                                   CString::new(signature).unwrap().as_ptr() as *const u8,
                                                   signature_length as u32,
                                                   Some(test_verify_cb)), error::SUCCESS.code_num);

        thread::sleep(Duration::from_secs(2));
    }
}
