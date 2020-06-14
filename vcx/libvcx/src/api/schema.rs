use serde_json;
use libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use std::ptr;
use schema;
use settings;
use utils::threadpool::spawn;
use error::prelude::*;
use indy_sys::CommandHandle;

/// Create a new Schema object and publish corresponding record on the ledger
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// schema_name: Name of schema
///
/// version: Version of schema. A semver-compatible value like "1.0" is encouraged.
///
/// schema_data: A list of attributes that will make up the schema, represented
///    as a string containing a JSON array. The number of attributes should be
///    less or equal to 125, because larger arrays cause various downstream problems.
///    This limitation is an annoyance that we'd like to remove.
///
/// # Example schema_data -> "["attr1", "attr2", "attr3"]"
///
/// payment_handle: Reserved for future use (currently uses any address in the wallet)
///
/// cb: Callback that provides Schema handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_create(command_handle: CommandHandle,
                                source_id: *const c_char,
                                schema_name: *const c_char,
                                version: *const c_char,
                                schema_data: *const c_char,
                                _payment_handle: u32,
                                cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, credentialdef_handle: u32)>) -> u32 {
    info!("vcx_schema_create >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(schema_name, VcxErrorKind::InvalidOption);
    check_useful_c_str!(version, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(schema_data, VcxErrorKind::InvalidOption);

    let issuer_did = match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
        Ok(x) => x,
        Err(x) => return x.into()
    };
    trace!(target: "vcx", "vcx_schema_create(command_handle: {}, source_id: {}, schema_name: {},  schema_data: {})",
           command_handle, source_id, schema_name, schema_data);

    spawn(move || {
        match schema::create_and_publish_schema(&source_id,
                                                issuer_did,
                                                schema_name,
                                                version,
                                                schema_data) {
            Ok(x) => {
                trace!(target: "vcx", "vcx_schema_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, x, source_id);
                cb(command_handle, error::SUCCESS.code_num, x);
            }
            Err(x) => {
                warn!("vcx_schema_create_cb(command_handle: {}, rc: {}, handle: {}) source_id: {}",
                      command_handle, x, 0, source_id);
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Create a new Schema object that will be published by Endorser later.
///
/// Note that Schema can't be used for credential issuing until it will be published on the ledger.
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// schema_name: Name of schema
///
/// version: version of schema
///
/// schema_data: list of attributes that will make up the schema (the number of attributes should be less or equal than 125)
///
/// endorser: DID of the Endorser that will submit the transaction.
///
/// # Example schema_data -> "["attr1", "attr2", "attr3"]"
///
/// cb: Callback that provides Schema handle and Schema transaction that should be passed to Endorser for publishing.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_prepare_for_endorser(command_handle: CommandHandle,
                                              source_id: *const c_char,
                                              schema_name: *const c_char,
                                              version: *const c_char,
                                              schema_data: *const c_char,
                                              endorser: *const c_char,
                                              cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32,
                                                                   schema_handle: u32,
                                                                   schema_transaction: *const c_char)>) -> u32 {
    info!("vcx_schema_prepare_for_endorser >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(schema_name, VcxErrorKind::InvalidOption);
    check_useful_c_str!(version, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(schema_data, VcxErrorKind::InvalidOption);
    check_useful_c_str!(endorser, VcxErrorKind::InvalidOption);

    let issuer_did = match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
        Ok(x) => x,
        Err(x) => return x.into()
    };
    trace!(target: "vcx", "vcx_schema_prepare_for_endorser(command_handle: {}, source_id: {}, schema_name: {},  schema_data: {},  endorser: {})",
           command_handle, source_id, schema_name, schema_data, endorser);

    spawn(move || {
        match schema::prepare_schema_for_endorser(&source_id,
                                                  issuer_did,
                                                  schema_name,
                                                  version,
                                                  schema_data,
                                                  endorser) {
            Ok((handle, transaction)) => {
                trace!(target: "vcx", "vcx_schema_prepare_for_endorser(command_handle: {}, rc: {}, handle: {}, transaction: {}) source_id: {}",
                       command_handle, error::SUCCESS.message, handle, transaction, source_id);
                let transaction = CStringUtils::string_to_cstring(transaction);
                cb(command_handle, error::SUCCESS.code_num, handle, transaction.as_ptr());
            }
            Err(x) => {
                warn!("vcx_schema_prepare_for_endorser(command_handle: {}, rc: {}, handle: {}, transaction: {}) source_id: {}",
                      command_handle, x, 0, "", source_id);
                cb(command_handle, x.into(), 0, ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes the schema object and returns a json string of all its attributes
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// schema_handle: Schema handle that was provided during creation. Used to access schema object
///
/// cb: Callback that provides json string of the schema's attributes and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_serialize(command_handle: CommandHandle,
                                   schema_handle: u32,
                                   cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, schema_state: *const c_char)>) -> u32 {
    info!("vcx_schema_serialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = schema::get_source_id(schema_handle).unwrap_or_default();
    trace!("vcx_schema_serialize(command_handle: {}, schema_handle: {}) source_id: {}",
           command_handle, schema_handle, source_id);

    if !schema::is_valid_handle(schema_handle) {
        return VcxError::from(VcxErrorKind::InvalidSchemaHandle).into()
    };

    spawn(move || {
        match schema::to_string(schema_handle) {
            Ok(x) => {
                trace!("vcx_schema_serialize_cb(command_handle: {}, schema_handle: {}, rc: {}, state: {}) source_id: {}",
                       command_handle, schema_handle, error::SUCCESS.message, x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                warn!("vcx_schema_serialize_cb(command_handle: {}, schema_handle: {}, rc: {}, state: {}) source_id: {}",
                      command_handle, schema_handle, x, "null", source_id);
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a schema object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// schema_data: json string representing a schema object
///
/// cb: Callback that provides schema handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_deserialize(command_handle: CommandHandle,
                                     schema_data: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, schema_handle: u32)>) -> u32 {
    info!("vcx_schema_deserialize >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(schema_data, VcxErrorKind::InvalidOption);

    trace!("vcx_schema_deserialize(command_handle: {}, schema_data: {})", command_handle, schema_data);
    spawn(move || {
        match schema::from_string(&schema_data) {
            Ok(x) => {
                trace!("vcx_schema_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {}",
                       command_handle, error::SUCCESS.message, x, schema::get_source_id(x).unwrap_or_default());
                cb(command_handle, error::SUCCESS.code_num, x);
            }
            Err(x) => {
                warn!("vcx_schema_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {}",
                      command_handle, x, 0, "");
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Releases the schema object by de-allocating memory
///
/// #Params
/// schema_handle: Schema handle that was provided during creation. Used to access schema object
///
/// #Returns
/// Success
#[no_mangle]
pub extern fn vcx_schema_release(schema_handle: u32) -> u32 {
    info!("vcx_schema_release >>>");

    let source_id = schema::get_source_id(schema_handle).unwrap_or_default();
    match schema::release(schema_handle) {
        Ok(()) => {
            trace!("vcx_schema_release(schema_handle: {}, rc: {}), source_id: {}",
                   schema_handle, error::SUCCESS.message, source_id);
            error::SUCCESS.code_num
        }
        Err(e) => {
            warn!("vcx_schema_release(schema_handle: {}, rc: {}), source_id: {}",
                  schema_handle, e, source_id);
            e.into()
        }
    }
}

/// Retrieves schema's id
///
/// #Params
/// schema_handle: Schema handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides schema id and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_get_schema_id(command_handle: CommandHandle,
                                       schema_handle: u32,
                                       cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, schema_id: *const c_char)>) -> u32 {
    info!("vcx_schema_get_schema_id >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    trace!("vcx_schema_get_schema_id(command_handle: {}, schema_handle: {})", command_handle, schema_handle);
    if !schema::is_valid_handle(schema_handle) {
        return VcxError::from(VcxErrorKind::InvalidSchemaHandle).into()
    }

    spawn(move || {
        match schema::get_schema_id(schema_handle) {
            Ok(x) => {
                trace!("vcx_schema_get_schema_id(command_handle: {}, schema_handle: {}, rc: {}, schema_seq_no: {})",
                       command_handle, schema_handle, error::SUCCESS.message, x);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            }
            Err(x) => {
                warn!("vcx_schema_get_schema_id(command_handle: {}, schema_handle: {}, rc: {}, schema_seq_no: {})",
                      command_handle, schema_handle, x, "");
                cb(command_handle, x.into(), ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Retrieves all of the data associated with a schema on the ledger.
///
/// #Params
/// source_id: Enterprise's personal identification for the user.
///
/// schema_id: id of schema given during the creation of the schema
///
/// cb: Callback contains the error status (if the schema cannot be found)
/// and it will also contain a json string representing all of the data of a
/// schema already on the ledger.
///
/// # Example
/// schema -> {"data":["height","name","sex","age"],"name":"test-licence","payment_txn":null,"schema_id":"2hoqvcwupRTUNkXn6ArYzs:2:test-licence:4.4.4","source_id":"Test Source ID","state":1,"version":"4.4.4"}
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_get_attributes(command_handle: CommandHandle,
                                        source_id: *const c_char,
                                        schema_id: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, s_handle: u32, schema_attrs: *const c_char)>) -> u32 {
    info!("vcx_schema_get_attributes >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);
    check_useful_c_str!(source_id, VcxErrorKind::InvalidOption);
    check_useful_c_str!(schema_id, VcxErrorKind::InvalidOption);
    trace!("vcx_schema_get_attributes(command_handle: {}, source_id: {}, schema_id: {})",
           command_handle, source_id, schema_id);

    spawn(move || {
        match schema::get_schema_attrs(source_id, schema_id) {
            Ok((handle, data)) => {
                let data: serde_json::Value = serde_json::from_str(&data).unwrap();
                let data = data["data"].clone();
                trace!("vcx_schema_get_attributes_cb(command_handle: {}, rc: {}, handle: {}, attrs: {})",
                       command_handle, error::SUCCESS.message, handle, data);
                let msg = CStringUtils::string_to_cstring(data.to_string());
                cb(command_handle, error::SUCCESS.code_num, handle, msg.as_ptr());
            }
            Err(x) => {
                warn!("vcx_schema_get_attributes_cb(command_handle: {}, rc: {}, handle: {}, attrs: {})",
                      command_handle, x, 0, "");
                cb(command_handle, x.into(), 0, ptr::null_mut());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Retrieve the txn associated with paying for the schema
///
/// #param
/// handle: schema handle that was provided during creation.  Used to access schema object.
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
pub extern fn vcx_schema_get_payment_txn(command_handle: CommandHandle,
                                         handle: u32,
                                         cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, txn: *const c_char)>) -> u32 {
    info!("vcx_schema_get_payment_txn >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    trace!("vcx_schema_get_payment_txn(command_handle: {})", command_handle);

    spawn(move || {
        match schema::get_payment_txn(handle) {
            Ok(x) => {
                match serde_json::to_string(&x) {
                    Ok(x) => {
                        trace!("vcx_schema_get_payment_txn_cb(command_handle: {}, rc: {}, : {}), source_id: {:?}",
                               command_handle, error::SUCCESS.message, x, schema::get_source_id(handle).unwrap_or_default());

                        let msg = CStringUtils::string_to_cstring(x);
                        cb(command_handle, 0, msg.as_ptr());
                    }
                    Err(e) => {
                        let err = VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot serialize payment txn: {}", e));
                        error!("vcx_schema_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {:?}",
                               command_handle, err, "null", schema::get_source_id(handle).unwrap_or_default());
                        cb(command_handle, err.into(), ptr::null_mut());
                    }
                }
            }
            Err(x) => {
                error!("vcx_schema_get_payment_txn_cb(command_handle: {}, rc: {}, txn: {}), source_id: {:?}",
                       command_handle, x, "null", schema::get_source_id(handle).unwrap_or_default());
                cb(command_handle, x.into(), ptr::null());
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Checks if schema is published on the Ledger and updates the  state
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// schema_handle: Schema handle that was provided during creation. Used to access schema object
///
/// cb: Callback that provides most current state of the schema and error status of request
///     States:
///         0 = Built
///         1 = Published
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_update_state(command_handle: CommandHandle,
                                      schema_handle: u32,
                                      cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, state: u32)>) -> u32 {
    info!("vcx_schema_update_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = schema::get_source_id(schema_handle).unwrap_or_default();
    trace!("vcx_schema_update_state(command_handle: {}, schema_handle: {}) source_id: {}",
           command_handle, schema_handle, source_id);

    if !schema::is_valid_handle(schema_handle) {
        return VcxError::from(VcxErrorKind::InvalidSchemaHandle).into();
    };

    spawn(move || {
        match schema::update_state(schema_handle) {
            Ok(state) => {
                trace!("vcx_schema_update_state(command_handle: {}, rc: {}, state: {})",
                       command_handle, error::SUCCESS.message, state);
                cb(command_handle, error::SUCCESS.code_num, state);
            }
            Err(x) => {
                warn!("vcx_schema_update_state(command_handle: {}, rc: {}, state: {})",
                      command_handle, x, 0);
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}

/// Get the current state of the schema object
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// schema_handle: Schema handle that was provided during creation. Used to access schema object
///
/// cb: Callback that provides most current state of the schema and error status of request
///     States:
///         0 = Built
///         1 = Published
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_get_state(command_handle: CommandHandle,
                                   schema_handle: u32,
                                   cb: Option<extern fn(xcommand_handle: CommandHandle, err: u32, state: u32)>) -> u32 {
    info!("vcx_schema_get_state >>>");

    check_useful_c_callback!(cb, VcxErrorKind::InvalidOption);

    let source_id = schema::get_source_id(schema_handle).unwrap_or_default();
    trace!("vcx_schema_get_state(command_handle: {}, schema_handle: {}) source_id: {}",
           command_handle, schema_handle, source_id);

    if !schema::is_valid_handle(schema_handle) {
        return VcxError::from(VcxErrorKind::InvalidSchemaHandle).into();
    };

    spawn(move || {
        match schema::get_state(schema_handle) {
            Ok(state) => {
                trace!("vcx_schema_get_state(command_handle: {}, rc: {}, state: {})",
                       command_handle, error::SUCCESS.message, state);
                cb(command_handle, error::SUCCESS.code_num, state);
            }
            Err(x) => {
                warn!("vcx_schema_get_state(command_handle: {}, rc: {}, state: {})",
                      command_handle, x, 0);
                cb(command_handle, x.into(), 0);
            }
        };

        Ok(())
    });

    error::SUCCESS.code_num
}


#[cfg(test)]
mod tests {
    extern crate serde_json;
    extern crate rand;

    use super::*;
    #[allow(unused_imports)]
    use rand::Rng;
    use std::ffi::CString;
    use settings;
    #[allow(unused_imports)]
    use utils::constants::{SCHEMA_ID, SCHEMA_WITH_VERSION, DEFAULT_SCHEMA_ATTRS, DEFAULT_SCHEMA_ID, DEFAULT_SCHEMA_NAME};
    use api::return_types_u32;
    use utils::devsetup::*;
    use schema::tests::prepare_schema_data;
    #[cfg(feature = "pool_tests")]
    use schema::CreateSchema;
    use utils::timeout::TimeoutUtils;

    fn vcx_schema_create_c_closure(name: &str, version: &str, data: &str) -> Result<u32, u32> {
        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let rc = vcx_schema_create(cb.command_handle,
                                   CString::new("Test Source ID").unwrap().into_raw(),
                                   CString::new(name).unwrap().into_raw(),
                                   CString::new(version).unwrap().into_raw(),
                                   CString::new(data).unwrap().into_raw(),
                                   0,
                                   Some(cb.get_callback()));
        if rc != error::SUCCESS.code_num {
            return Err(rc);
        }

        let handle = cb.receive(TimeoutUtils::some_medium()).unwrap();
        Ok(handle)
    }

    fn vcx_schema_serialize_c_closure(handle: u32) -> String {
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_schema_serialize(cb.command_handle, handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        let schema_json = cb.receive(TimeoutUtils::some_short()).unwrap().unwrap();
        schema_json
    }

    #[test]
    fn test_vcx_create_schema_success() {
        let _setup = SetupMocks::init();

        let (_, schema_name, schema_version, data) = prepare_schema_data();
        let handle = vcx_schema_create_c_closure(&schema_name, &schema_version, &data).unwrap();
        assert!(handle > 0)
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_vcx_create_schema_with_pool() {
        let _setup = SetupLibraryWalletPoolZeroFees::init();

        let (_, schema_name, schema_version, data) = prepare_schema_data();
        let handle = vcx_schema_create_c_closure(&schema_name, &schema_version, &data).unwrap();
        assert!(handle > 0)
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_vcx_schema_get_attrs_with_pool() {
        let _setup = SetupLibraryWalletPoolZeroFees::init();

        let (schema_id, _) = ::utils::libindy::anoncreds::tests::create_and_write_test_schema(::utils::constants::DEFAULT_SCHEMA_ATTRS);

        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        assert_eq!(vcx_schema_get_attributes(cb.command_handle,
                                             CString::new("Test Source ID").unwrap().into_raw(),
                                             CString::new(schema_id).unwrap().into_raw(),
                                             Some(cb.get_callback())), error::SUCCESS.code_num);

        let (_err, attrs) = cb.receive(TimeoutUtils::some_short()).unwrap();
        let mut result_vec = vec!(attrs.clone().unwrap());
        let mut expected_vec = vec!(DEFAULT_SCHEMA_ATTRS);
        assert_eq!(result_vec.sort(), expected_vec.sort());
    }

    #[test]
    fn test_vcx_schema_serialize() {
        let _setup = SetupMocks::init();

        let (_, schema_name, schema_version, data) = prepare_schema_data();
        let handle = vcx_schema_create_c_closure(&schema_name, &schema_version, &data).unwrap();

        let _schema_json = vcx_schema_serialize_c_closure(handle);
    }

    #[test]
    fn test_vcx_schema_deserialize_succeeds() {
        let _setup = SetupMocks::init();

        let cb = return_types_u32::Return_U32_U32::new().unwrap();
        let err = vcx_schema_deserialize(cb.command_handle, CString::new(SCHEMA_WITH_VERSION).unwrap().into_raw(), Some(cb.get_callback()));
        assert_eq!(err, error::SUCCESS.code_num);
        let schema_handle = cb.receive(TimeoutUtils::some_short()).unwrap();
        assert!(schema_handle > 0);
    }

    #[test]
    fn test_vcx_schema_get_schema_id_succeeds() {
        let _setup = SetupMocks::init();

        let (_, schema_name, schema_version, data) = prepare_schema_data();
        let schema_handle = vcx_schema_create_c_closure(&schema_name, &schema_version, &data).unwrap();

        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(vcx_schema_get_schema_id(cb.command_handle, schema_handle, Some(cb.get_callback())), error::SUCCESS.code_num);
        let id = cb.receive(TimeoutUtils::some_short()).unwrap().unwrap();
        assert_eq!(DEFAULT_SCHEMA_ID, &id);
    }

    #[test]
    fn test_vcx_schema_get_attrs() {
        let _setup = SetupMocks::init();

        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        let data = r#"["height","name","sex","age"]"#;
        assert_eq!(vcx_schema_get_attributes(cb.command_handle,
                                             CString::new("Test Source ID").unwrap().into_raw(),
                                             CString::new(SCHEMA_ID).unwrap().into_raw(),
                                             Some(cb.get_callback())), error::SUCCESS.code_num);
        let (_handle, schema_data_as_string) = cb.receive(TimeoutUtils::some_short()).unwrap();
        let schema_data_as_string = schema_data_as_string.unwrap();
        let schema_as_json: serde_json::Value = serde_json::from_str(&schema_data_as_string).unwrap();
        assert_eq!(schema_as_json["data"].to_string(), data);
    }

    #[test]
    fn test_get_payment_txn() {
        let _setup = SetupMocks::init();

        let cb = return_types_u32::Return_U32_STR::new().unwrap();

        let (_, schema_name, schema_version, data) = prepare_schema_data();
        let handle = vcx_schema_create_c_closure(&schema_name, &schema_version, &data).unwrap();

        let _rc = vcx_schema_get_payment_txn(cb.command_handle, handle, Some(cb.get_callback()));
        let txn = cb.receive(TimeoutUtils::some_short()).unwrap();
        assert!(txn.is_some());
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_vcx_schema_serialize_contains_version() {
        let _setup = SetupLibraryWalletPoolZeroFees::init();

        let (_, schema_name, schema_version, data) = prepare_schema_data();
        let handle = vcx_schema_create_c_closure(&schema_name, &schema_version, &data).unwrap();

        let schema_json = vcx_schema_serialize_c_closure(handle);

        let j: serde_json::Value = serde_json::from_str(&schema_json).unwrap();
        let _schema: CreateSchema = serde_json::from_value(j["data"].clone()).unwrap();
        assert_eq!(j["version"], "1.0");
    }

    #[test]
    fn test_vcx_schema_release() {
        let _setup = SetupMocks::init();

        let (_, schema_name, schema_version, data) = prepare_schema_data();
        let handle = vcx_schema_create_c_closure(&schema_name, &schema_version, &data).unwrap();

        let unknown_handle = handle + 1;
        assert_eq!(vcx_schema_release(unknown_handle), error::INVALID_SCHEMA_HANDLE.code_num);
    }

    #[test]
    fn test_vcx_prepare_schema_success() {
        let _setup = SetupMocks::init();

        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
        let cb = return_types_u32::Return_U32_U32_STR::new().unwrap();
        assert_eq!(vcx_schema_prepare_for_endorser(cb.command_handle,
                                                   CString::new("Test Source ID").unwrap().into_raw(),
                                                   CString::new("Test Schema").unwrap().into_raw(),
                                                   CString::new("0.0").unwrap().into_raw(),
                                                   CString::new("[att1, att2]").unwrap().into_raw(),
                                                   CString::new("V4SGRU86Z58d6TV7PBUe6f").unwrap().into_raw(),
                                                   Some(cb.get_callback())), error::SUCCESS.code_num);
        let (_handle, schema_transaction) = cb.receive(TimeoutUtils::some_short()).unwrap();
        let schema_transaction = schema_transaction.unwrap();
        let schema_transaction: serde_json::Value = serde_json::from_str(&schema_transaction).unwrap();
        let expected_schema_transaction: serde_json::Value = serde_json::from_str(::utils::constants::REQUEST_WITH_ENDORSER).unwrap();
        assert_eq!(expected_schema_transaction, schema_transaction);
    }

    #[test]
    fn test_vcx_schema_get_state() {
        let _setup = SetupMocks::init();

        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (handle, _) = schema::prepare_schema_for_endorser("testid", did, "name".to_string(), "1.0".to_string(), "[\"name\":\"male\"]".to_string(), "V4SGRU86Z58d6TV7PBUe6f".to_string()).unwrap();
        {
            let cb = return_types_u32::Return_U32_U32::new().unwrap();
            let _rc = vcx_schema_get_state(cb.command_handle, handle, Some(cb.get_callback()));
            assert_eq!(cb.receive(TimeoutUtils::some_medium()).unwrap(), ::api::PublicEntityStateType::Built as u32)
        }
        {
            let cb = return_types_u32::Return_U32_U32::new().unwrap();
            let _rc = vcx_schema_update_state(cb.command_handle, handle, Some(cb.get_callback()));
            assert_eq!(cb.receive(TimeoutUtils::some_medium()).unwrap(), ::api::PublicEntityStateType::Published as u32);
        }
        {
            let cb = return_types_u32::Return_U32_U32::new().unwrap();
            let _rc = vcx_schema_get_state(cb.command_handle, handle, Some(cb.get_callback()));
            assert_eq!(cb.receive(TimeoutUtils::some_medium()).unwrap(), ::api::PublicEntityStateType::Published as u32)
        }
    }
}
