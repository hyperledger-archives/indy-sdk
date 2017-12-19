use super::ErrorCode;

use libc::c_char;
use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;

pub struct Ledger {}

impl Ledger {
    pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

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
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

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
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

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
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

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
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

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

    pub fn build_get_attrib_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            indy_build_get_attrib_request(command_handle,
                                          submitter_did.as_ptr(),
                                          target_did.as_ptr(),
                                          data.as_ptr(),
                                          cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

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

    pub fn build_get_schema_request(submitter_did: &str, dest: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

        let submitter_did = CString::new(submitter_did).unwrap();
        let dest = CString::new(dest).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            indy_build_get_schema_request(command_handle,
                                          submitter_did.as_ptr(),
                                          dest.as_ptr(),
                                          data.as_ptr(),
                                          cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_claim_def_txn(submitter_did: &str, xref: i32, signature_type: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

        let submitter_did = CString::new(submitter_did).unwrap();
        let signature_type = CString::new(signature_type).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            indy_build_claim_def_txn(command_handle,
                                     submitter_did.as_ptr(),
                                     xref,
                                     signature_type.as_ptr(),
                                     data.as_ptr(),
                                     cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_claim_def_txn(submitter_did: &str, xref: i32, signature_type: &str, origin: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

        let submitter_did = CString::new(submitter_did).unwrap();
        let signature_type = CString::new(signature_type).unwrap();
        let origin = CString::new(origin).unwrap();

        let err = unsafe {
            indy_build_get_claim_def_txn(command_handle,
                                         submitter_did.as_ptr(),
                                         xref,
                                         signature_type.as_ptr(),
                                         origin.as_ptr(),
                                         cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let (command_handle, cb) = super::callbacks::_closure_to_cb_ec_string(sender);

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
                                     data: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_schema_request(command_handle: i32,
                                 submitter_did: *const c_char,
                                 data: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_schema_request(command_handle: i32,
                                     submitter_did: *const c_char,
                                     dest: *const c_char,
                                     data: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_claim_def_txn(command_handle: i32,
                                submitter_did: *const c_char,
                                xref: i32,
                                signature_type: *const c_char,
                                data: *const c_char,
                                cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_claim_def_txn(command_handle: i32,
                                    submitter_did: *const c_char,
                                    xref: i32,
                                    signature_type: *const c_char,
                                    origin: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_node_request(command_handle: i32,
                               submitter_did: *const c_char,
                               target_did: *const c_char,
                               data: *const c_char,
                               cb: Option<extern fn(xcommand_handle: i32, err: ErrorCode, request_json: *const c_char)>) -> ErrorCode;
}
