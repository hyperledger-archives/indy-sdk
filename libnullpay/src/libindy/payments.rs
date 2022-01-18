use ErrorCode;
use libc::c_char;
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
                                      extra: *const c_char,
                                      cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type ParseResponseWithFeesCB = extern fn(command_handle: i32,
                                             resp_json: *const c_char,
                                             cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type BuildGetPaymentSourcesRequestCB = extern fn(command_handle: i32,
                                                     wallet_handle: i32,
                                                     submitter_did: *const c_char,
                                                     payment_address: *const c_char,
                                                     from: i64,
                                                     cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type ParseGetPaymentSourcesResponseCB = extern fn(command_handle: i32,
                                                      resp_json: *const c_char,
                                                      cb: Option<extern fn(command_handle_: i32,
                                                                           err: ErrorCode,
                                                                           sources_json: *const c_char,
                                                                           next: i64) -> ErrorCode>) -> ErrorCode;

pub type BuildPaymentReqCB = extern fn(command_handle: i32,
                                       wallet_handle: i32,
                                       submitter_did: *const c_char,
                                       inputs_json: *const c_char,
                                       outputs_json: *const c_char,
                                       extra: *const c_char,
                                       cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type ParsePaymentResponseCB = extern fn(command_handle: i32,
                                            resp_json: *const c_char,
                                            cb: Option<IndyPaymentCallback>) -> ErrorCode;

pub type BuildMintReqCB = extern fn(command_handle: i32,
                                    wallet_handle: i32,
                                    submitter_did: *const c_char,
                                    outputs_json: *const c_char,
                                    extra: *const c_char,
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

pub type BuildVerifyPaymentReqCB = extern fn(command_handle: i32,
                                             wallet_handle: i32,
                                             submitter_did: *const c_char,
                                             receipt: *const c_char,
                                             cb: Option<extern fn(command_handle_: i32,
                                                                  err: ErrorCode,
                                                                  verify_txn_json: *const c_char) -> ErrorCode>) -> ErrorCode;

pub type ParseVerifyPaymentResponseCB = extern fn(command_handle: i32,
                                                  resp_json: *const c_char,
                                                  cb: Option<extern fn(command_handle_: i32,
                                                                       err: ErrorCode,
                                                                       txn_json: *const c_char) -> ErrorCode>) -> ErrorCode;

pub type SignWithAddressCB = extern fn (command_handle: i32, wallet_handle: i32,
                                        address: *const c_char,
                                        message_raw: *const u8, message_len: u32,
                                        cb: Option<extern fn(command_handle: i32, err: ErrorCode, raw: *const u8, len: u32)>) -> ErrorCode;
pub type VerifyWithAddressCB = extern fn (command_handle: i32, address: *const c_char,
                                          message_raw: *const u8, message_len: u32,
                                          signature_raw: *const u8, signature_len: u32,
                                          cb: Option<extern fn(command_handle: i32, err: ErrorCode, result: bool)>) -> ErrorCode;

pub fn register_payment_method(
    payment_method: *const c_char,
    create_payment_address: CreatePaymentAddressCB,
    add_request_fees: AddRequestFeesCB,
    parse_response_with_fees: ParseResponseWithFeesCB,
    build_get_payment_sources_request: BuildGetPaymentSourcesRequestCB,
    parse_get_payment_sources_response: ParseGetPaymentSourcesResponseCB,
    build_payment_req: BuildPaymentReqCB,
    parse_payment_response: ParsePaymentResponseCB,
    build_mint_req: BuildMintReqCB,
    build_set_txn_fees_req: BuildSetTxnFeesReqCB,
    build_get_txn_fees_req: BuildGetTxnFeesReqCB,
    parse_get_txn_fees_response: ParseGetTxnFeesResponseCB,
    build_verify_payment_req: BuildVerifyPaymentReqCB,
    parse_verify_payment_response: ParseVerifyPaymentResponseCB,
    sign_with_address: SignWithAddressCB,
    verify_with_address: VerifyWithAddressCB
) -> ErrorCode {
    let (sender, receiver) = channel();

    let closure: Box<dyn FnMut(ErrorCode) + Send> = Box::new(move |err| {
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
            Some(build_get_payment_sources_request),
            Some(parse_get_payment_sources_response),
            Some(build_payment_req),
            Some(parse_payment_response),
            Some(build_mint_req),
            Some(build_set_txn_fees_req),
            Some(build_get_txn_fees_req),
            Some(parse_get_txn_fees_response),
            Some(build_verify_payment_req),
            Some(parse_verify_payment_response),
            Some(sign_with_address),
            Some(verify_with_address),
            cb,
        );
    }

    receiver.recv().unwrap()
}

pub fn list_payment_addresses(wallet_handle: i32,
                              cb: Box<dyn FnMut(ErrorCode, String) + Send>, ) -> ErrorCode {
    let (command_handle, cb) = callbacks::closure_to_cb_ec_string(cb);

    unsafe {
        indy_list_payment_addresses(command_handle,
                                    wallet_handle,
                                    cb,
        )
    }
}

extern {
    pub fn indy_register_payment_method(
        command_handle: i32,
        payment_method: *const c_char,
        create_payment_address: Option<CreatePaymentAddressCB>,
        add_request_fees: Option<AddRequestFeesCB>,
        parse_response_with_fees: Option<ParseResponseWithFeesCB>,
        build_get_payment_sources_request: Option<BuildGetPaymentSourcesRequestCB>,
        parse_get_payment_sources_response: Option<ParseGetPaymentSourcesResponseCB>,
        build_payment_req: Option<BuildPaymentReqCB>,
        parse_payment_response: Option<ParsePaymentResponseCB>,
        build_mint_req: Option<BuildMintReqCB>,
        build_set_txn_fees_req: Option<BuildSetTxnFeesReqCB>,
        build_get_txn_fees_req: Option<BuildGetTxnFeesReqCB>,
        parse_get_txn_fees_response: Option<ParseGetTxnFeesResponseCB>,
        build_verify_payment_req: Option<BuildVerifyPaymentReqCB>,
        parse_verify_payment_response: Option<ParseVerifyPaymentResponseCB>,
        sign_with_address: Option<SignWithAddressCB>,
        verify_with_address: Option<VerifyWithAddressCB>,
        cb: Option<extern fn(command_handle_: i32, err: ErrorCode)>) -> ErrorCode;

    fn indy_list_payment_addresses(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_addresses_json: *const c_char)>) -> ErrorCode;
}


