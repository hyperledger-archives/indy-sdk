use super::ErrorCode;

use libc::c_char;
use std::ffi::CString;
use std::ptr::null;

pub struct Payment {}

impl Payment {
    pub fn create_payment_address(wallet_handle: i32, payment_method: &str, config: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let config = CString::new(config).unwrap();

        let err = unsafe {
            indy_create_payment_address(command_handle,
                                        wallet_handle,
                                        payment_method.as_ptr(),
                                        config.as_ptr(),
                                        cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn list_payment_addresses(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string();

        let err = unsafe {
            indy_list_payment_addresses(command_handle,
                                        wallet_handle,
                                        cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn add_request_fees(wallet_handle: i32, submitter_did: Option<&str>, req_json: &str, inputs_json: &str, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec_string_string();

        let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let req_json = CString::new(req_json).unwrap();
        let inputs_json = CString::new(inputs_json).unwrap();
        let outputs_json = CString::new(outputs_json).unwrap();
        let extra_str = extra.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_add_request_fees(command_handle,
                                  wallet_handle,
                                  if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
                                  req_json.as_ptr(),
                                  inputs_json.as_ptr(),
                                  outputs_json.as_ptr(),
                                  if extra.is_some() { extra_str.as_ptr() } else { null() },
                                  cb)
        };

        super::results::result_to_string_string(err, receiver)
    }

    pub fn build_get_payment_sources_request(wallet_handle: i32, submitter_did: Option<&str>, payment_address: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string_string();

        let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let payment_address = CString::new(payment_address).unwrap();

        let err = unsafe {
            indy_build_get_payment_sources_request(command_handle,
                                                   wallet_handle,
                                                   if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
                                                   payment_address.as_ptr(),
                                                   cb)
        };

        super::results::result_to_string_string(err, receiver)
    }


    pub fn parse_get_payment_sources_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = unsafe {
            indy_parse_get_payment_sources_response(command_handle,
                                                    payment_method.as_ptr(),
                                                    resp_json.as_ptr(),
                                                    cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_payment_req(wallet_handle: i32, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string_string();

        let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let inputs = CString::new(inputs).unwrap();
        let outputs = CString::new(outputs).unwrap();
        let extra_str = extra.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_build_payment_req(command_handle,
                                   wallet_handle,
                                   if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
                                   inputs.as_ptr(),
                                   outputs.as_ptr(),
                                   if extra.is_some() { extra_str.as_ptr() } else { null() },
                                   cb)
        };

        super::results::result_to_string_string(err, receiver)
    }

    pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = unsafe {
            indy_parse_payment_response(command_handle,
                                        payment_method.as_ptr(),
                                        resp_json.as_ptr(),
                                        cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_mint_req(wallet_handle: i32, submitter_did: Option<&str>, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string_string();

        let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let outputs_json = CString::new(outputs_json).unwrap();
        let extra_str = extra.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            indy_build_mint_req(command_handle,
                                wallet_handle,
                                if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
                                outputs_json.as_ptr(),
                                if extra.is_some() { extra_str.as_ptr() } else { null() },
                                cb)
        };

        super::results::result_to_string_string(err, receiver)
    }

    pub fn build_set_txn_fees_req(wallet_handle: i32, submitter_did: Option<&str>, payment_method: &str, fees_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string();

        let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let payment_method = CString::new(payment_method).unwrap();
        let fees_json = CString::new(fees_json).unwrap();

        let err = unsafe {
            indy_build_set_txn_fees_req(command_handle,
                                        wallet_handle,
                                        if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
                                        payment_method.as_ptr(),
                                        fees_json.as_ptr(),
                                        cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_get_txn_fees_req(wallet_handle: i32, submitter_did: Option<&str>, payment_method: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string();

        let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let payment_method = CString::new(payment_method).unwrap();

        let err = unsafe {
            indy_build_get_txn_fees_req(command_handle,
                                        wallet_handle,
                                        if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
                                        payment_method.as_ptr(),
                                        cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = unsafe {
            indy_parse_get_txn_fees_response(command_handle,
                                             payment_method.as_ptr(),
                                             resp_json.as_ptr(),
                                             cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = unsafe {
            indy_parse_response_with_fees(command_handle,
                                          payment_method.as_ptr(),
                                          resp_json.as_ptr(),
                                          cb)
        };

        super::results::result_to_string(err, receiver)
    }

    pub fn build_verify_payment_req(wallet_handle: i32, submitter_did: Option<&str>, receipt: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string_string();

        let submitter_did_str = submitter_did.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let receipt = CString::new(receipt).unwrap();

        let err = unsafe {
            indy_build_verify_payment_req(command_handle,
                                          wallet_handle,
                                          if submitter_did.is_some() { submitter_did_str.as_ptr() } else { null() },
                                          receipt.as_ptr(),
                                          cb)
        };

        super::results::result_to_string_string(err, receiver)
    }

    pub fn parse_verify_payment_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            super::callbacks::_closure_to_cb_ec_string();

        let payment_method = CString::new(payment_method).unwrap();
        let resp_json = CString::new(resp_json).unwrap();

        let err = unsafe {
            indy_parse_verify_payment_response(command_handle,
                                               payment_method.as_ptr(),
                                               resp_json.as_ptr(),
                                               cb)
        };

        super::results::result_to_string(err, receiver)
    }
}

extern {
    #[no_mangle]
    fn indy_create_payment_address(command_handle: i32,
                                   wallet_handle: i32,
                                   payment_method: *const c_char,
                                   config: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_address: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_list_payment_addresses(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_addresses_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_add_request_fees(command_handle: i32,
                             wallet_handle: i32,
                             submitter_did: *const c_char,
                             req_json: *const c_char,
                             inputs_json: *const c_char,
                             outputs_json: *const c_char,
                             extra: *const c_char,
                             cb: Option<extern fn(command_handle_: i32,
                                                  err: ErrorCode,
                                                  req_with_fees_json: *const c_char,
                                                  payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_payment_sources_request(command_handle: i32,
                                              wallet_handle: i32,
                                              submitter_did: *const c_char,
                                              payment_address: *const c_char,
                                              cb: Option<extern fn(command_handle_: i32,
                                                                   err: ErrorCode,
                                                                   get_sources_txn_json: *const c_char,
                                                                   payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_parse_get_payment_sources_response(command_handle: i32,
                                               payment_method: *const c_char,
                                               resp_json: *const c_char,
                                               cb: Option<extern fn(command_handle_: i32,
                                                                    err: ErrorCode,
                                                                    sources_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_payment_req(command_handle: i32,
                              wallet_handle: i32,
                              submitter_did: *const c_char,
                              inputs_json: *const c_char,
                              outputs_json: *const c_char,
                              extra: *const c_char,
                              cb: Option<extern fn(command_handle_: i32,
                                                   err: ErrorCode,
                                                   payment_req_json: *const c_char,
                                                   payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_parse_payment_response(command_handle: i32,
                                   payment_method: *const c_char,
                                   resp_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        receipts_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_mint_req(command_handle: i32,
                           wallet_handle: i32,
                           submitter_did: *const c_char,
                           outputs_json: *const c_char,
                           extra: *const c_char,
                           cb: Option<extern fn(command_handle_: i32,
                                                err: ErrorCode,
                                                mint_req_json: *const c_char,
                                                payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_set_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   fees_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        set_txn_fees_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_get_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        get_txn_fees_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_parse_get_txn_fees_response(command_handle: i32,
                                        payment_method: *const c_char,
                                        resp_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             fees_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_parse_response_with_fees(command_handle: i32,
                                     payment_method: *const c_char,
                                     resp_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32,
                                                          err: ErrorCode,
                                                          receipts_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_build_verify_payment_req(command_handle: i32,
                                     wallet_handle: i32,
                                     submitter_did: *const c_char,
                                     receipt: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32,
                                                  err: ErrorCode,
                                                  verify_txn_json: *const c_char,
                                                  payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    fn indy_parse_verify_payment_response(command_handle: i32,
                                          payment_method: *const c_char,
                                          resp_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                       err: ErrorCode,
                                                       txn_json: *const c_char)>) -> ErrorCode;
}
