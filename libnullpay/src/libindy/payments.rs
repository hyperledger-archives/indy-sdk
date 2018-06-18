use ErrorCode;
use std::os::raw::c_char;
use utils::callbacks;
use std::sync::mpsc::channel;

pub type IndyPaymentCallback = extern fn(command_handle_: i32,
                                         err: ErrorCode,
                                         payment_address: *const c_char) -> ErrorCode;

pub type CreatePaymentAddressCB = extern fn(command_handle: i32,
                                            wallet_handle: i32,
                                            config: *const c_char,
                                            cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type AddRequestFeesCB = extern fn(command_handle: i32,
                                      wallet_handle: i32,
                                      submitter_did: *const c_char,
                                      req_json: *const c_char,
                                      inputs_json: *const c_char,
                                      outputs_json: *const c_char,
                                      cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type ParseResponseWithFeesCB = extern fn(command_handle: i32,
                                             resp_json: *const c_char,
                                             cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type BuildGetUTXORequestCB = extern fn(command_handle: i32,
                                           wallet_handle: i32,
                                           submitter_did: *const c_char,
                                           payment_address: *const c_char,
                                           cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type ParseGetUTXOResponseCB = extern fn(command_handle: i32,
                                            resp_json: *const c_char,
                                            cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type BuildPaymentReqCB = extern fn(command_handle: i32,
                                       wallet_handle: i32,
                                       submitter_did: *const c_char,
                                       inputs_json: *const c_char,
                                       outputs_json: *const c_char,
                                       cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type ParsePaymentResponseCB = extern fn(command_handle: i32,
                                            resp_json: *const c_char,
                                            cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type BuildMintReqCB = extern fn(command_handle: i32,
                                    wallet_handle: i32,
                                    submitter_did: *const c_char,
                                    outputs_json: *const c_char,
                                    cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type BuildSetTxnFeesReqCB = extern fn(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          fees_json: *const c_char,
                                          cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type BuildGetTxnFeesReqCB = extern fn(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type ParseGetTxnFeesResponseCB = extern fn(command_handle: i32,
                                               resp_json: *const c_char,
                                               cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub fn register_payment_method(
    payment_method: *const c_char,
    create_payment_address: CreatePaymentAddressCB,
    add_request_fees: AddRequestFeesCB,
    parse_response_with_fees: ParseResponseWithFeesCB,
    build_get_utxo_request: BuildGetUTXORequestCB,
    parse_get_utxo_response: ParseGetUTXOResponseCB,
    build_payment_req: BuildPaymentReqCB,
    parse_payment_response: ParsePaymentResponseCB,
    build_mint_req: BuildMintReqCB,
    build_set_txn_fees_req: BuildSetTxnFeesReqCB,
    build_get_txn_fees_req: BuildGetTxnFeesReqCB,
    parse_get_txn_fees_response: ParseGetTxnFeesResponseCB,
) -> ErrorCode {
    let (sender, receiver) = channel();

    let closure: Box<FnMut(ErrorCode) + Send> = Box::new(move |err| {
        sender.send(err).unwrap();
    });

    let (cmd_handle, cb) = callbacks::closure_to_cb_ec(closure);

    unsafe {
        indy_register_payment_method(
            cmd_handle,
            payment_method,
            Some(create_payment_address),
            Some(add_request_fees),
            Some(parse_response_with_fees),
            Some(build_get_utxo_request),
            Some(parse_get_utxo_response),
            Some(build_payment_req),
            Some(parse_payment_response),
            Some(build_mint_req),
            Some(build_set_txn_fees_req),
            Some(build_get_txn_fees_req),
            Some(parse_get_txn_fees_response),
            cb,
        );
    }

    receiver.recv().unwrap()
}

pub fn list_payment_addresses(wallet_handle: i32,
                              cb: Box<FnMut(ErrorCode, String) + Send>, ) -> ErrorCode {
    let (command_handle, cb) = callbacks::closure_to_cb_ec_string(cb);

    unsafe {
        indy_list_payment_addresses(command_handle,
                                    wallet_handle,
                                    cb,
        )
    }
}

extern {
    #[no_mangle]
    pub fn indy_register_payment_method(
        command_handle: i32,
        payment_method: *const c_char,
        create_payment_address: Option<CreatePaymentAddressCB>,
        add_request_fees: Option<AddRequestFeesCB>,
        parse_response_with_fees: Option<ParseResponseWithFeesCB>,
        build_get_utxo_request: Option<BuildGetUTXORequestCB>,
        parse_get_utxo_response: Option<ParseGetUTXOResponseCB>,
        build_payment_req: Option<BuildPaymentReqCB>,
        parse_payment_response: Option<ParsePaymentResponseCB>,
        build_mint_req: Option<BuildMintReqCB>,
        build_set_txn_fees_req: Option<BuildSetTxnFeesReqCB>,
        build_get_txn_fees_req: Option<BuildGetTxnFeesReqCB>,
        parse_get_txn_fees_response: Option<ParseGetTxnFeesResponseCB>,
        cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode;

    #[no_mangle]
    fn indy_list_payment_addresses(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_addresses_json: *const c_char)>) -> ErrorCode;
}


