use super::ErrorCode;
use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_register_payment_method(
        command_handle: i32,
        payment_method: *const c_char,
//TODO: replace cb to IndyPaymentCallback
        create_payment_address: Option<extern fn(_command_handle: i32,
                                                 config: *const c_char,
                                                 wallet_handle: i32,
                                                 _cb: IndyPaymentCallback) -> ErrorCode>,
        add_request_fees: Option<extern fn(_command_handle: i32,
                                           _wallet_handle: i32,
                                           req_json: *const c_char,
                                           inputs_json: *const c_char,
                                           outputs_json: *const c_char,
                                           _cb: IndyPaymentCallback) -> ErrorCode>,
        parse_response_with_fees: Option<extern fn(_command_handle: i32,
                                                   resp_json: *const c_char,
                                                   _cb: IndyPaymentCallback) -> ErrorCode>,
        build_get_utxo_request: Option<extern fn(_command_handle: i32,
                                                 _wallet_handle: i32,
                                                 payment_address: *const c_char,
                                                 _cb: IndyPaymentCallback) -> ErrorCode>,
        parse_get_utxo_response: Option<extern fn(_command_handle: i32,
                                                  resp_json: *const c_char,
                                                  _cb: IndyPaymentCallback) -> ErrorCode>,
        build_payment_req: Option<extern fn(_command_handle: i32,
                                            _wallet_handle: i32,
                                            inputs_json: *const c_char,
                                            outputs_json: *const c_char,
                                            _cb: IndyPaymentCallback) -> ErrorCode>,
        parse_payment_response: Option<extern fn(_command_handle: i32,
                                                 resp_json: *const c_char,
                                                 _cb: IndyPaymentCallback) -> ErrorCode>,
        build_mint_req: Option<extern fn(_command_handle: i32,
                                         _wallet_handle: i32,
                                         outputs_json: *const c_char,
                                         _cb: IndyPaymentCallback) -> ErrorCode>,
        build_set_txn_fees_req: Option<extern fn(_command_handle: i32,
                                                 _wallet_handle: i32,
                                                 fees_json: *const c_char,
                                                 _cb: IndyPaymentCallback) -> ErrorCode>,
        build_get_txn_fees_req: Option<extern fn(_command_handle: i32,
                                                 _wallet_handle: i32,
                                                 _cb: IndyPaymentCallback) -> ErrorCode>,
        parse_get_txn_fees_response: Option<extern fn(_command_handle: i32,
                                                      resp_json: *const c_char,
                                                      _cb: IndyPaymentCallback) -> ErrorCode>,

        cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode;
}



pub type IndyPaymentCallback = Option<extern fn(command_handle_: i32,
                                                err: ErrorCode,
                                                payment_address: *const c_char) -> ErrorCode>;
