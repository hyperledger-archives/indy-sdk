extern crate libc;

use self::libc::c_char;
use utils::cstring::CStringUtils;
use utils::error;
use utils::error::error_string;
use std::thread;
use std::ptr;
use schema;
use settings;

/// Create a new Schema object that can create or look up schemas on the ledger
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// source_id: Enterprise's personal identification for the user.
///
/// schema_name: Name of schema
///
/// schema_data: list of attributes that will make up the schema
///
/// # Example schema_data -> "["attr1", "attr2", "attr3"]"
///
/// cb: Callback that provides Schema handle and error status of request.
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_create(command_handle: u32,
                                source_id: *const c_char,
                                schema_name: *const c_char,
                                schema_data: *const c_char,
                                cb: Option<extern fn(xcommand_handle: u32, err: u32, claimdef_handle: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_name, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_data, error::INVALID_OPTION.code_num);
    let issuer_did = match settings::get_config_value(settings::CONFIG_INSTITUTION_DID) {
        Ok(x) => x,
        Err(x) => return x
    };
    info!("vcx_schema_create(command_handle: {}, source_id: {}, schema_name: {},  schema_data: {})",
          command_handle, source_id, schema_name, schema_data);

    thread::spawn( move|| {
        let ( rc, handle) = match schema::create_new_schema(&source_id,
                                                                 schema_name,
                                                                 issuer_did,
                                                                 schema_data) {
            Ok(x) => {
                info!("vcx_schema_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, &source_id);
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_schema_create_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, &source_id);
                (x, 0) },
        };

        cb(command_handle, rc, handle);
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
pub extern fn vcx_schema_serialize(command_handle: u32,
                                   schema_handle: u32,
                                   cb: Option<extern fn(xcommand_handle: u32, err: u32, schema_state: *const c_char)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    let source_id = schema::get_source_id(schema_handle).unwrap_or_default();
    info!("vcx_schema_serialize(command_handle: {}, schema_handle: {}), source_id: {:?}",
          command_handle, schema_handle, source_id);

    if !schema::is_valid_handle(schema_handle) {
        return error::INVALID_SCHEMA_HANDLE.code_num;
    };

    thread::spawn( move|| {
        match schema::to_string(schema_handle) {
            Ok(x) => {
                info!("vcx_schema_serialize_cb(command_handle: {}, schema_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, schema_handle, error_string(0), x, source_id);
                let msg = CStringUtils::string_to_cstring(x);
                cb(command_handle, error::SUCCESS.code_num, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_schema_serialize_cb(command_handle: {}, schema_handle: {}, rc: {}, state: {}), source_id: {:?}",
                      command_handle, schema_handle, error_string(x), "null", source_id);
                cb(command_handle, x, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}

/// Takes a json string representing a schema object and recreates an object matching the json
///
/// #Params
/// command_handle: command handle to map callback to user context.
///
/// schema: json string representing a schema object
///
/// # Examples schema -> {"data":{"seqNo":15,"identifier":"4fUDR9R7fjwELRvH9JT6HH","txnTime":1510246647,"type":"101","data":{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}},"handle":1,"name":"schema_name","source_id":"testId","sequence_num":306}
///
/// cb: Callback that provides schema handle and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_deserialize(command_handle: u32,
                                     schema_data: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: u32, err: u32, schema_handle: u32)>) -> u32 {

    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(schema_data, error::INVALID_OPTION.code_num);

    info!("vcx_schema_deserialize(command_handle: {}, schema_data: {})", command_handle, schema_data);
    thread::spawn( move|| {
        let (rc, handle) = match schema::from_string(&schema_data) {
            Ok(x) => {
                info!("vcx_schema_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(0), x, schema::get_source_id(x).unwrap_or_default());
                (error::SUCCESS.code_num, x)
            },
            Err(x) => {
                warn!("vcx_schema_deserialize_cb(command_handle: {}, rc: {}, handle: {}), source_id: {:?}",
                      command_handle, error_string(x), 0, "");
                (x, 0)
            },
        };
        cb(command_handle, rc, handle);
    });

    error::SUCCESS.code_num
}

/// Releases the schema object by de-allocating memory
///
/// #Params
/// schema_handle: Schema handle that was provided during creation. Used to access schema object
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_release(schema_handle: u32) -> u32 {
    info!("vcx_schema_release(schema_handle: {}), source_id: {:?}",
          schema_handle, schema::get_source_id(schema_handle).unwrap_or_default());
    schema::release(schema_handle)
}

/// Retrieves schema's sequence number
///
/// #Params
/// schema_handle: Schema handle that was provided during creation. Used to access proof object
///
/// cb: Callback that provides schema number and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_get_sequence_no(command_handle: u32,
                                         schema_handle: u32,
                                         cb: Option<extern fn(xcommand_handle: u32, err: u32, sequence_no: u32)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);

    info!("vcx_schema_get_sequence_no(command_handle: {}, schema_handle: {})", command_handle, schema_handle);
    if !schema::is_valid_handle(schema_handle) {
        return error::INVALID_SCHEMA_HANDLE.code_num;
    }

    thread::spawn(move|| {
        let (schema_no, rc) = match schema::get_sequence_num(schema_handle) {
            Ok(x) => {
                info!("vcx_schema_get_sequence_no_cb(command_handle: {}, schema_handle: {}, rc: {}, schema_seq_no: {})",
                      command_handle, schema_handle, error_string(0), x);
                (x, error::SUCCESS.code_num)
            },
            Err(x) => {
                warn!("vcx_schema_get_sequence_no_cb(command_handle: {}, schema_handle: {}, rc: {}, schema_seq_no: {})",
                      command_handle, schema_handle, error_string(x), 0);
                (0, x)
            },
        };
        cb(command_handle, rc, schema_no);
    });

    error::SUCCESS.code_num
}

/// Retrieves schema's attributes
///
/// #Params
/// sequence_no: The schema sequence number for wanted schema
///
/// source_id: Enterprise's personal identification for the user.
///
/// cb: Callback that provides schema number and provides error status
///
/// #Returns
/// Error code as a u32
#[no_mangle]
pub extern fn vcx_schema_get_attributes(command_handle: u32,
                                        source_id: *const c_char,
                                        sequence_no: u32,
                                        cb: Option<extern fn(xcommand_handle: u32, err: u32, s_handle: u32, schema_attrs: *const c_char)>) -> u32 {
    check_useful_c_callback!(cb, error::INVALID_OPTION.code_num);
    check_useful_c_str!(source_id, error::INVALID_OPTION.code_num);
    info!("vcx_schema_get_attributes(command_handle: {}, source_id: {}, sequence_no: {})",
          command_handle, source_id, sequence_no);

    thread::spawn( move|| {
        match schema::get_schema_attrs(source_id, sequence_no) {
            Ok((handle, data)) => {
                info!("vcx_schema_get_attributes_cb(command_handle: {}, rc: {}, handle: {}, attrs: {})",
                      command_handle, error_string(0), handle, data);
                let msg = CStringUtils::string_to_cstring(data);
                cb(command_handle, error::SUCCESS.code_num, handle, msg.as_ptr());
            },
            Err(x) => {
                warn!("vcx_schema_get_attributes_cb(command_handle: {}, rc: {}, handle: {}, attrs: {})",
                      command_handle, error_string(x), 0, "");
                cb(command_handle, x, 0, ptr::null_mut());
            },
        };

    });

    error::SUCCESS.code_num
}


#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    use api::claim_def::vcx_claimdef_create;
    use std::ffi::CString;
    use std::thread;
    use std::time::Duration;
    use settings;
    use utils::libindy::pool;
    use utils::libindy::signus::SignusUtils;
    use utils::constants::{ DEMO_AGENT_PW_SEED, DEMO_ISSUER_PW_SEED, SCHEMA_TXN };
    use utils::libindy::wallet::{init_wallet, get_wallet_handle, delete_wallet};

    extern "C" fn create_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_cb")
    }

    extern "C" fn create_cb_err(command_handle: u32, err: u32, schema_handle: u32) {
        assert_ne!(err, 0);
        println!("successfully called create_cb_err")
    }

    extern "C" fn create_and_serialize_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_and_serialize_cb");
        assert_eq!(vcx_schema_serialize(0, schema_handle, Some(serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn get_attrs_cb(command_handle: u32, err: u32, handle: u32, schema_data: *const c_char) {
        assert_eq!(err, 0);
        assert!(handle > 0);
        if schema_data.is_null() {
            panic!("schema_data is null");
        }
        check_useful_c_str!(schema_data, ());
        let mut data = r#""data":{"name":"New Claim - Claim5","version":"1.0","attr_names":["New Claim","claim5","a5","b5","c5","d5"]}"#;
        if settings::test_indy_mode_enabled() {
            data = SCHEMA_TXN;
        }
        assert!(schema_data.contains(&data));
        println!("successfully called get_attrs_cb: {}", schema_data);
    }


    extern "C" fn create_cb_get_seq_no(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_cb_get_seq_no");
        assert_eq!(vcx_schema_get_sequence_no(0, schema_handle, Some(get_seq_no_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    extern "C" fn create_schema_and_claimdef_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called create_schema_and_claimdef_cb");
        let schema_seq_no = schema::get_sequence_num(schema_handle).unwrap();
        println!("created schema with schema_seq_no: {}", schema_seq_no);
        assert_eq!(vcx_claimdef_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Claim Def").unwrap().into_raw(),
                                       schema_seq_no,
                                       ptr::null(),
                                       false,
                                       Some(create_cb)),
                   error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(800));
    }

    extern "C" fn serialize_cb(handle: u32, err: u32, schema_str: *const c_char) {
        assert_eq!(err, 0);
        if schema_str.is_null() {
            panic!("schema_str is null");
        }
        check_useful_c_str!(schema_str, ());
        println!("successfully called serialize_cb: {}", schema_str);
    }

    extern "C" fn get_seq_no_cb(handle: u32, err: u32, schema_no: u32) {
        assert_eq!(err, 0);
        assert_eq!(schema_no, 299);
        println!("successfully called get_seq_no_cb: {}", schema_no);
    }

    extern "C" fn deserialize_cb(command_handle: u32, err: u32, schema_handle: u32) {
        assert_eq!(err, 0);
        assert!(schema_handle > 0);
        println!("successfully called deserialize_cb");
        let expected = r#"{"data":{"seqNo":15,"identifier":"4fUDR9R7fjwELRvH9JT6HH","txnTime":1510246647,"type":"101","data":{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}},"name":"schema_name","source_id":"testId","sequence_num":306}"#;
        let new = schema::to_string(schema_handle).unwrap();
        assert_eq!(expected, new);
    }

    fn set_default_and_enable_test_mode() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "true");
    }

    #[test]
    fn test_vcx_create_schema_success() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_schema_create(0,
                                       CString::new("Test Source ID").unwrap().into_raw(),
                                       CString::new("Test Schema").unwrap().into_raw(),
                                       CString::new("{}").unwrap().into_raw(),
                                       Some(create_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[ignore]
    #[test]
    fn test_vcx_create_schema_with_pool() {
        settings::set_defaults();
        pool::open_sandbox_pool();
        init_wallet("a_test_wallet").unwrap();
        let wallet_handle = get_wallet_handle();
        let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_ISSUER_PW_SEED)).unwrap();
        SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_AGENT_PW_SEED)).unwrap();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        assert_eq!(vcx_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     Some(create_schema_and_claimdef_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(1));
        delete_wallet("a_test_wallet").unwrap();
    }

    #[ignore]
    #[test]
    fn test_vcx_create_schema_and_create_claimdef_with_pool() {
        settings::set_defaults();
        pool::open_sandbox_pool();
        init_wallet("a_test_wallet").unwrap();
        let wallet_handle = get_wallet_handle();
        let (my_did, _) = SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_ISSUER_PW_SEED)).unwrap();
        SignusUtils::create_and_store_my_did(wallet_handle, Some(DEMO_AGENT_PW_SEED)).unwrap();
        settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        let data = r#"{"name":"Claim For Driver's License","version":"1.0","attr_names":["address1","address2","city","state","zip"]}"#.to_string();
        assert_eq!(vcx_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     Some(create_schema_and_claimdef_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_secs(60));
        delete_wallet("a_test_wallet").unwrap();
    }

    #[ignore]
    #[test]
    fn test_vcx_schema_get_attrs_with_pool() {
        settings::set_defaults();
        pool::open_sandbox_pool();
        init_wallet("a_test_wallet").unwrap();
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        assert_eq!(vcx_schema_get_attributes(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     116,
                                     Some(get_attrs_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
        delete_wallet("a_test_wallet").unwrap();
    }

    #[test]
    fn test_vcx_schema_serialize() {
        set_default_and_enable_test_mode();
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        assert_eq!(vcx_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new(data).unwrap().into_raw(),
                                     Some(create_and_serialize_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_schema_deserialize_succeeds() {
        set_default_and_enable_test_mode();
        let original = r#"{"data":{"seqNo":15,"identifier":"4fUDR9R7fjwELRvH9JT6HH","txnTime":1510246647,"type":"101","data":{"name":"Home Address","version":"0.1","attr_names":["address1","address2","city","state","zip"]}},"handle":1,"name":"schema_name","source_id":"testId","sequence_num":306}"#;
        vcx_schema_deserialize(0,CString::new(original).unwrap().into_raw(), Some(deserialize_cb));
        thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test_vcx_schema_get_schema_no_succeeds() {
        set_default_and_enable_test_mode();
        assert_eq!(vcx_schema_create(0,
                                     CString::new("Test Source ID").unwrap().into_raw(),
                                     CString::new("Test Schema").unwrap().into_raw(),
                                     CString::new("{}").unwrap().into_raw(),
                                     Some(create_cb_get_seq_no)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));

    }

    #[test]
    fn test_vcx_schema_get_attrs() {
        set_default_and_enable_test_mode();
        let data = r#"{"name":"name","version":"1.0","attr_names":["name","male"]}"#.to_string();
        assert_eq!(vcx_schema_get_attributes(0,
                                             CString::new("Test Source ID").unwrap().into_raw(),
                                             116,
                                             Some(get_attrs_cb)), error::SUCCESS.code_num);
        thread::sleep(Duration::from_millis(200));
    }
}
