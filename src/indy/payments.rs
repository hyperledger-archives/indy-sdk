use ErrorCode;
use std::os::raw::c_char;

extern {
    #[no_mangle]
    pub fn indy_register_payment_method(command_handle: i32,
                                        payment_method: *const c_char,
                                        create_payment_address_callback: Option<extern fn(config: *const c_char,
                                                                                          payment_address_ptr: *const *const c_char) -> ErrorCode>,
                                        add_request_fees_callback: Option<extern fn(req_json: *const c_char,
                                                                                    inputs_json: *const c_char,
                                                                                    outputs_json: *const c_char) -> ErrorCode>,
                                        parse_response_with_fees_callback: Option<extern fn(resp_json: *const c_char,
                                                                                   utxo_ptr: *const *const c_char) -> ErrorCode>,
                                        build_get_utxo_request_callback: Option<extern fn(payment_address: *const c_char,
                                                                            get_utxo_txn_ptr: *const *const c_char) -> ErrorCode>,
                                        parse_get_utxo_response_callback: Option<extern fn(resp_json: *const c_char,
                                                                                  utxo_ptr: *const *const c_char) -> ErrorCode>,
                                        build_payment_req_callback: Option<extern fn(inputs_json: *const c_char,
                                                                            outpus_json: *const c_char,
                                                                            payment_req_ptr: *const *const c_char) -> ErrorCode>,
                                        parse_payment_response_callback: Option<extern fn(resp_json: *const c_char,
                                                                                 utxo_ptr: *const *const c_char) -> ErrorCode>,
                                        build_mint_req_callback: Option<extern fn(outputs_json: *const c_char,
                                                                         mint_req_ptr: *const *const c_char) -> ErrorCode>,
                                        build_set_txn_fees_req_callback: Option<extern fn(fees_json: *const c_char,
                                                                                 set_txn_fees_ptr: *const *const c_char) -> ErrorCode>,
                                        build_get_txn_fees_req_callback: Option<extern fn(get_txn_fees_ptr: *const *const c_char) -> ErrorCode>,
                                        parse_get_txn_fees_response_callback: Option<extern fn(resp_json: *const c_char,
                                                                                               fees_ptr: *const *const c_char) -> ErrorCode>,
                                        cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_sign_multi_request(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   resp_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        signed_request_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_create_payment_address(command_handle: i32,
                                   wallet_handle: i32,
                                   payment_method: *const c_char,
                                   config: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_address: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_payment_addresses(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<extern fn(command_handle_: i32,
                                                err: ErrorCode,
                                                payment_addresses_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_add_request_fees(command_handle: i32,
                             wallet_handle: i32,
                             submitter_did: *const c_char,
                             req_json: *const c_char,
                             inputs_json: *const c_char,
                             outputs_json: *const c_char,
                             cb: Option<extern fn(command_handle_: i32,
                                                  err: ErrorCode,
                                                  req_with_fees_json: *const c_char,
                                                  payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_utxo_request(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_address: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        get_utxo_txn_json: *const c_char,
                                                        payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_utxo_response(command_handle: i32,
                                    payment_method: *const c_char,
                                    resp_json: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: ErrorCode,
                                                         utxo_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_payment_req(command_handle: i32,
                              wallet_handle: i32,
                              submitter_did: *const c_char,
                              inputs_json: *const c_char,
                              outputs_json: *const c_char,
                              cb: Option<extern fn(command_handle_: i32,
                                                   err: ErrorCode,
                                                   payment_req_json: *const c_char,
                                                   payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_payment_response(command_handle: i32,
                                   payment_method: *const c_char,
                                   resp_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        utxo_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_mint_req(command_handle: i32,
                           wallet_handle: i32,
                           submitter_did: *const c_char,
                           outputs_json: *const c_char,
                           cb: Option<extern fn(command_handle_: i32,
                                                err: ErrorCode,
                                                mint_req_json: *const c_char,
                                                payment_method: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_set_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   fees_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        set_txn_fees_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        get_txn_fees_json: *const c_char)>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_txn_fees_response(command_handle: i32,
                                        payment_method: *const c_char,
                                        resp_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             fees_json: *const c_char)>) -> ErrorCode;
}
