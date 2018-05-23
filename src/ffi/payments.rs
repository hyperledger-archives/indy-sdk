use ErrorCode;
use std::os::raw::c_char;

pub type EmptyResponseCB = extern fn(xcommand_handle: i32, err: ErrorCode);

pub type PaymentResponseCB = extern fn(xcommand_handle: i32,
                                             err: ErrorCode,
                                             json: *const c_char);
pub type PaymentsMethodResponseCB = extern fn(xcommand_handle: i32,
                                                    err: ErrorCode,
                                                    json: *const c_char,
                                                    payment_method: *const c_char);

pub type PaymentResponseApiCB = extern fn(xcommand_handle: i32,
                                             err: ErrorCode,
                                             json: *const c_char) -> ErrorCode;

pub type CreatePaymentAddressCB = extern fn(command_handle: i32,
                                                  wallet_handle: i32,
                                                  config: *const c_char,
                                                  cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type AddRequestFeesCB = extern fn(command_handle: i32,
                                            wallet_handle: i32,
                                            submitter_did: *const c_char,
                                            req_json: *const c_char,
                                            inputs_json: *const c_char,
                                            outputs_json: *const c_char,
                                            cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type ParseResponseWithFeesCB = extern fn(command_handle: i32,
                                                   resp_json: *const c_char,
                                                   cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type BuildGetUTXORequestCB = extern fn(command_handle: i32,
                                                 wallet_handle: i32,
                                                 submitter_did: *const c_char,
                                                 payment_address: *const c_char,
                                                 cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type ParseGetUTXOResponseCB = extern fn(command_handle: i32,
                                                  resp_json: *const c_char,
                                                  cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type BuildPaymentRequestCB = extern fn(command_handle: i32,
                                                 wallet_handle: i32,
                                                 submitter_did: *const c_char,
                                                 inputs_json: *const c_char,
                                                 outputs_json: *const c_char,
                                                 cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type ParsePaymentResponseCB = extern fn(command_handle: i32,
                                                  resp_json: *const c_char,
                                                  cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type BuildMintRequestCB = extern fn(command_handle: i32,
                                              wallet_handle: i32,
                                              submitter_did: *const c_char,
                                              outputs_json: *const c_char,
                                              cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type BuildSetTxnFeesRequestCB = extern fn(command_handle: i32,
                                                    wallet_handle: i32,
                                                    submitter_did: *const c_char,
                                                    fees_json: *const c_char,
                                                    cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type BuildGetTxnFeesRequestCB = extern fn(command_handle: i32,
                                                    wallet_handle: i32,
                                                    submitter_did: *const c_char,
                                                    cb: Option<PaymentResponseApiCB>) -> ErrorCode;
pub type ParseGetTxnFeesResponseCB = extern fn(command_handle: i32,
                                                     resp_json: *const c_char,
                                                     cb: Option<PaymentResponseApiCB>) -> ErrorCode;

extern {
    #[no_mangle]
    pub fn indy_register_payment_method(command_handle: i32,
                                        payment_method: *const c_char,
                                        create_payment_address: Option<CreatePaymentAddressCB>,
                                        add_request_fees: Option<AddRequestFeesCB>,
                                        parse_response_with_fees_callback: Option<ParseResponseWithFeesCB>,
                                        build_get_utxo_request_callback: Option<BuildGetUTXORequestCB>,
                                        parse_get_utxo_response_callback: Option<ParseGetUTXOResponseCB>,
                                        build_payment_req_callback: Option<BuildPaymentRequestCB>,
                                        parse_payment_response_callback: Option<ParsePaymentResponseCB>,
                                        build_mint_req_callback: Option<BuildMintRequestCB>,
                                        build_set_txn_fees_req_callback: Option<BuildSetTxnFeesRequestCB>,
                                        build_get_txn_fees_req_callback: Option<BuildGetTxnFeesRequestCB>,
                                        parse_get_txn_fees_response_callback: Option<ParseGetTxnFeesResponseCB>,
                                        cb: Option<EmptyResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_create_payment_address(command_handle: i32,
                                   wallet_handle: i32,
                                   payment_method: *const c_char,
                                   config: *const c_char,
                                   cb: Option<PaymentResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_list_payment_addresses(command_handle: i32,
                                   wallet_handle: i32,
                                   cb: Option<PaymentResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_add_request_fees(command_handle: i32,
                             wallet_handle: i32,
                             submitter_did: *const c_char,
                             req_json: *const c_char,
                             inputs_json: *const c_char,
                             outputs_json: *const c_char,
                             cb: Option<PaymentsMethodResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_response_with_fees(command_handle: i32,
                                         payment_method: *const c_char,
                                         resp_json: *const c_char,
                                         cb: Option<PaymentResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_utxo_request(command_handle: i32,
                                       wallet_handle: i32,
                                       submitter_did: *const c_char,
                                       payment_address: *const c_char,
                                       cb: Option<PaymentsMethodResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_utxo_response(command_handle: i32,
                                    payment_method: *const c_char,
                                    resp_json: *const c_char,
                                    cb: Option<PaymentResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_payment_req(command_handle: i32,
                              wallet_handle: i32,
                              submitter_did: *const c_char,
                              inputs_json: *const c_char,
                              outputs_json: *const c_char,
                              cb: Option<PaymentsMethodResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_payment_response(command_handle: i32,
                                   payment_method: *const c_char,
                                   resp_json: *const c_char,
                                   cb: Option<PaymentResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_mint_req(command_handle: i32,
                           wallet_handle: i32,
                           submitter_did: *const c_char,
                           outputs_json: *const c_char,
                           cb: Option<PaymentsMethodResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_set_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   fees_json: *const c_char,
                                   cb: Option<PaymentResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_build_get_txn_fees_req(command_handle: i32,
                                   wallet_handle: i32,
                                   submitter_did: *const c_char,
                                   payment_method: *const c_char,
                                   cb: Option<PaymentResponseCB>) -> ErrorCode;

    #[no_mangle]
    pub fn indy_parse_get_txn_fees_response(command_handle: i32,
                                        payment_method: *const c_char,
                                        resp_json: *const c_char,
                                        cb: Option<PaymentResponseCB>) -> ErrorCode;
}
