use indy::api::ErrorCode;
use std::os::raw::c_char;

type IndyPaymentCallback = extern fn(command_handle_: i32,
                                     err: ErrorCode,
                                     payment_address: *const c_char) -> ErrorCode;

pub extern fn create_payment_address_stub(command_handle: i32,
                                          config: *const c_char,
                                          wallet_handle: i32,
                                          cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn add_request_fees_stub(command_handle: i32,
                                    req_json: *const c_char,
                                    inputs_json: *const c_char,
                                    outputs_json: *const c_char,
                                    wallet_handle: i32,
                                    cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_response_with_fees(command_handle: i32,
                                       resp_json: *const c_char,
                                       cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_get_utxo_request_stub(command_handle: i32,
                                          payment_address: *const c_char,
                                          wallet_handle: i32,
                                          cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_get_utxo_response_stub(command_handle: i32,
                                           resp_json: *const c_char,
                                           cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_payment_req_stub(command_handle: i32,
                                     inputs_json: *const c_char,
                                     outputs_json: *const c_char,
                                     wallet_handle: i32,
                                     cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_mint_req_stub(command_handle: i32,
                                  outputs_json: *const c_char,
                                  wallet_handle: i32,
                                  cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_payment_response_stub(command_handle: i32,
                                          resp_json: *const c_char,
                                          cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_set_txn_fees_request_stub(command_handle: i32,
                                              fees_json: *const c_char,
                                              wallet_handle: i32,
                                              cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_get_txn_fees_request_stub(command_handle: i32,
                                              wallet_handle: i32,
                                              cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_get_txn_fees_response_stub(command_handle: i32,
                                               resp_json: *const c_char,
                                               cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}