use indy::api::ErrorCode;
use std::os::raw::c_char;

type IndyPaymentCallback = extern fn(command_handle_: i32,
                                     err: ErrorCode,
                                     payment_address: *const c_char) -> ErrorCode;

pub extern fn create_payment_address_stub(_command_handle: i32,
                                          _wallet_handle: i32,
                                          _config: *const c_char,
                                          _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn add_request_fees_stub(_command_handle: i32,
                                    _wallet_handle: i32,
                                    _req_json: *const c_char,
                                    _inputs_json: *const c_char,
                                    _outputs_json: *const c_char,
                                    _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_response_with_fees(_command_handle: i32,
                                       _resp_json: *const c_char,
                                       _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_get_utxo_request_stub(_command_handle: i32,
                                          _wallet_handle: i32,
                                          _payment_address: *const c_char,
                                          _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_get_utxo_response_stub(_command_handle: i32,
                                           _resp_json: *const c_char,
                                           _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_payment_req_stub(_command_handle: i32,
                                     _wallet_handle: i32,
                                     _inputs_json: *const c_char,
                                     _outputs_json: *const c_char,
                                     _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_mint_req_stub(_command_handle: i32,
                                  _wallet_handle: i32,
                                  _outputs_json: *const c_char,
                                  _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_payment_response_stub(_command_handle: i32,
                                          _resp_json: *const c_char,
                                          _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_set_txn_fees_request_stub(_command_handle: i32,
                                              _wallet_handle: i32,
                                              _fees_json: *const c_char,
                                              _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn build_get_txn_fees_request_stub(_command_handle: i32,
                                              _wallet_handle: i32,
                                              _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}

pub extern fn parse_get_txn_fees_response_stub(_command_handle: i32,
                                               _resp_json: *const c_char,
                                               _cb: Option<IndyPaymentCallback>) -> ErrorCode {
    ErrorCode::Success
}