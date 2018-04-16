use super::ErrorCode;

use libc::c_char;
use std::ffi::CString;
use std::ptr::null;

pub struct Ledger {}

impl Ledger {
    pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let request_json = CString::new(request_json).unwrap();

        let err = unsafe {
            indy_sign_and_submit_request(command_handle,
                                         pool_handle,
                                         wallet_handle,
                                         submitter_did.as_ptr(),
                                         request_json.as_ptr(),
                                         cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let request_json = CString::new(request_json).unwrap();

        let err = unsafe {
            indy_submit_request(command_handle,
                                pool_handle,
                                request_json.as_ptr(),
                                cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let verkey_str = verkey.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let data_str = data.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let role_str = role.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let err = unsafe {
            indy_build_nym_request(command_handle,
                                   submitter_did.as_ptr(),
                                   target_did.as_ptr(),
                                   if verkey.is_some() { verkey_str.as_ptr() } else { null() },
                                   if data.is_some() { data_str.as_ptr() } else { null() },
                                   if role.is_some() { role_str.as_ptr() } else { null() },
                                   cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_nym_request(submitter_did: &str, target_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let err = unsafe {
            indy_build_get_nym_request(command_handle,
                                       submitter_did.as_ptr(),
                                       target_did.as_ptr(),
                                       cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let hash_str = hash.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let raw_str = raw.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let enc_str = enc.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_build_attrib_request(command_handle,
                                      submitter_did.as_ptr(),
                                      target_did.as_ptr(),
                                      if hash.is_some() { hash_str.as_ptr() } else { null() },
                                      if raw.is_some() { raw_str.as_ptr() } else { null() },
                                      if enc.is_some() { enc_str.as_ptr() } else { null() },
                                      cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_attrib_request(submitter_did: &str, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let raw_str = raw.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let hash_str = hash.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let enc_str = enc.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_build_get_attrib_request(command_handle,
                                          submitter_did.as_ptr(),
                                          target_did.as_ptr(),
                                          if raw.is_some() { raw_str.as_ptr() } else { null() },
                                          if hash.is_some() { hash_str.as_ptr() } else { null() },
                                          if enc.is_some() { enc_str.as_ptr() } else { null() },
                                          cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            indy_build_schema_request(command_handle,
                                      submitter_did.as_ptr(),
                                      data.as_ptr(),
                                      cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_schema_request(submitter_did: &str, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let id = CString::new(id).unwrap();

        let err = unsafe {
            indy_build_get_schema_request(command_handle,
                                          submitter_did.as_ptr(),
                                          id.as_ptr(),
                                          cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_cred_def_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            indy_build_cred_def_request(command_handle,
                                        submitter_did.as_ptr(),
                                        data.as_ptr(),
                                        cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_cred_def_request(submitter_did: &str, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let id = CString::new(id).unwrap();

        let err = unsafe {
            indy_build_get_cred_def_request(command_handle,
                                            submitter_did.as_ptr(),
                                            id.as_ptr(),
                                            cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            indy_build_node_request(command_handle,
                                    submitter_did.as_ptr(),
                                    target_did.as_ptr(),
                                    data.as_ptr(),
                                    cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn indy_build_pool_config_request(submitter_did: &str, writes: bool, force: bool) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();

        let err = unsafe {
            indy_build_pool_config_request(command_handle,
                                           submitter_did.as_ptr(),
                                           writes,
                                           force,
                                           cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn indy_build_pool_restart_request(submitter_did: &str, action: &str, datetime: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let action = CString::new(action).unwrap();
        let datetime = CString::new(datetime).unwrap();

        let err = unsafe {
            indy_build_pool_restart_request(command_handle,
                                            submitter_did.as_ptr(),
                                            action.as_ptr(),
                                            datetime.as_ptr(),
                                            cb)
        };
        super::results::result_to_string(err, receiver)
    }

    pub fn indy_build_pool_upgrade_request(submitter_did: &str, name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                                           justification: Option<&str>, reinstall: bool, force: bool) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let name = CString::new(name).unwrap();
        let version = CString::new(version).unwrap();
        let action = CString::new(action).unwrap();
        let sha256 = CString::new(sha256).unwrap();
        let timeout = timeout.map(|t| t as i32).unwrap_or(-1);

        let schedule_str = schedule.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let justification_str = justification.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_build_pool_upgrade_request(command_handle,
                                            submitter_did.as_ptr(),
                                            name.as_ptr(),
                                            version.as_ptr(),
                                            action.as_ptr(),
                                            sha256.as_ptr(),
                                            timeout,
                                            if schedule.is_some() { schedule_str.as_ptr() } else { null() },
                                            if justification.is_some() { justification_str.as_ptr() } else { null() },
                                            reinstall,
                                            force,
                                            cb)
        };

        super::results::result_to_string(err, receiver)
    }
}


extern {
    #[no_mangle]
    fn indy_sign_and_submit_request(command_handle: i32,
                                    pool_handle: i32,
                                    wallet_handle: i32,
                                    submitter_did: *const c_char,
                                    request_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_result_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_submit_request(command_handle: i32,
                           pool_handle: i32,
                           request_json: *const c_char,
                           cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_result_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_nym_request(command_handle: i32,
                              submitter_did: *const c_char,
                              target_did: *const c_char,
                              verkey: *const c_char,
                              alias: *const c_char,
                              role: *const c_char,
                              cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_nym_request(command_handle: i32,
                                  submitter_did: *const c_char,
                                  target_did: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_attrib_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 target_did: *const c_char,
                                 hash: *const c_char,
                                 raw: *const c_char,
                                 enc: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_attrib_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     target_did: *const c_char,
                                     raw: *const c_char,
                                     hash: *const c_char,
                                     enc: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_schema_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 data: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_schema_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     id: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_cred_def_request(command_handle: i32,
                                   submitter_did: *const c_char,
                                   data: *const c_char,
                                   cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_cred_def_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       id: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_node_request(command_handle: i32,
                               submitter_did: *const c_char,
                               target_did: *const c_char,
                               data: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_pool_config_request(command_handle: i32,
                                      submitter_did: *const c_char,
                                      writes: bool,
                                      force: bool,
                                      cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_pool_restart_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       action: *const c_char,
                                       datetime: *const c_char,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_pool_upgrade_request(command_handle: i32,
                                       submitter_did: *const c_char,
                                       name: *const c_char,
                                       version: *const c_char,
                                       action: *const c_char,
                                       sha256: *const c_char,
                                       timeout: i32,
                                       schedule: *const c_char,
                                       justification: *const c_char,
                                       reinstall: bool,
                                       force: bool,
                                       cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;
}
