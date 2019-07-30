use super::*;

use {CString, Error, CommandHandle, WalletHandle};

extern {
    #[no_mangle]
    pub fn indy_register_payment_method(command_handle: CommandHandle,
                                        payment_method: CString,
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
                                        cb: Option<ResponseEmptyCB>) -> Error;

    #[no_mangle]
    pub fn indy_create_payment_address(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       payment_method: CString,
                                       config: CString,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_list_payment_addresses(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_add_request_fees(command_handle: CommandHandle,
                                 wallet_handle: WalletHandle,
                                 submitter_did: CString,
                                 req_json: CString,
                                 inputs_json: CString,
                                 outputs_json: CString,
                                 extra: CString,
                                 cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_response_with_fees(command_handle: CommandHandle,
                                         payment_method: CString,
                                         resp_json: CString,
                                         cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_payment_sources_with_from_request(command_handle: CommandHandle,
                                                            wallet_handle: WalletHandle,
                                                            submitter_did: CString,
                                                            payment_address: CString,
                                                            from: i64,
                                                            cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_payment_sources_with_from_response(command_handle: CommandHandle,
                                                   payment_method: CString,
                                                   resp_json: CString,
                                                   cb: Option<ResponseStringI64CB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_payment_sources_request(command_handle: CommandHandle,
                                                  wallet_handle: WalletHandle,
                                                  submitter_did: CString,
                                                  payment_address: CString,
                                                  cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_payment_sources_response(command_handle: CommandHandle,
                                                   payment_method: CString,
                                                   resp_json: CString,
                                                   cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_payment_req(command_handle: CommandHandle,
                                  wallet_handle: WalletHandle,
                                  submitter_did: CString,
                                  inputs_json: CString,
                                  outputs_json: CString,
                                  extra: CString,
                                  cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_payment_response(command_handle: CommandHandle,
                                       payment_method: CString,
                                       resp_json: CString,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_prepare_payment_extra_with_acceptance_data(command_handle: CommandHandle,
                                                           extra_json: CString,
                                                           text: CString,
                                                           version: CString,
                                                           hash: CString,
                                                           acc_mech_type: CString,
                                                           time_of_acceptance: u64,
                                                           cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_mint_req(command_handle: CommandHandle,
                               wallet_handle: WalletHandle,
                               submitter_did: CString,
                               outputs_json: CString,
                               extra: CString,
                               cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_set_txn_fees_req(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       submitter_did: CString,
                                       payment_method: CString,
                                       fees_json: CString,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_txn_fees_req(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       submitter_did: CString,
                                       payment_method: CString,
                                       cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_txn_fees_response(command_handle: CommandHandle,
                                            payment_method: CString,
                                            resp_json: CString,
                                            cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_verify_payment_req(command_handle: CommandHandle,
                                         wallet_handle: WalletHandle,
                                         submitter_did: CString,
                                         receipt: CString,
                                         cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_verify_payment_response(command_handle: CommandHandle,
                                              payment_method: CString,
                                              resp_json: CString,
                                              cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_get_request_info(command_handle: CommandHandle,
                                 get_auth_rule_resp_json: CString,
                                 requester_info_json: CString,
                                 fees_json: CString,
                                 cb: Option<ResponseStringCB>) -> Error;
    pub fn indy_sign_with_address(command_handle: CommandHandle,
                                  wallet_handle: WalletHandle,
                                  address: CString,
                                  message_raw: BString,
                                  message_len: u32,
                                  cb: Option<ResponseSliceCB>) -> Error;
    #[no_mangle]
    pub fn indy_verify_with_address(command_handle: CommandHandle,
                                    address: CString,
                                    message_raw: BString,
                                    message_len: u32,
                                    signature_raw: BString,
                                    signature_len: u32,
                                    cb: Option<ResponseBoolCB>) -> Error;
    
}

pub type CreatePaymentAddressCB = extern fn(command_handle: CommandHandle,
                                            wallet_handle: WalletHandle,
                                            config: CString,
                                            cb: Option<extern fn(command_handle_: CommandHandle,
                                                                 err: Error,
                                                                 payment_address: CString) -> Error>) -> Error;
pub type AddRequestFeesCB = extern fn(command_handle: CommandHandle,
                                      wallet_handle: WalletHandle,
                                      submitter_did: CString,
                                      req_json: CString,
                                      inputs_json: CString,
                                      outputs_json: CString,
                                      extra: CString,
                                      cb: Option<extern fn(command_handle_: CommandHandle,
                                                           err: Error,
                                                           req_with_fees_json: CString) -> Error>) -> Error;
pub type ParseResponseWithFeesCB = extern fn(command_handle: CommandHandle,
                                             resp_json: CString,
                                             cb: Option<extern fn(command_handle_: CommandHandle,
                                                                  err: Error,
                                                                  receipts_json: CString) -> Error>) -> Error;
pub type BuildGetPaymentSourcesRequestCB = extern fn(command_handle: CommandHandle,
                                                     wallet_handle: WalletHandle,
                                                     submitter_did: CString,
                                                     payment_address: CString,
                                                     from: i64,
                                                     cb: Option<extern fn(command_handle_: CommandHandle,
                                                                          err: Error,
                                                                          get_sources_txn_json: CString) -> Error>) -> Error;
pub type ParseGetPaymentSourcesResponseCB = extern fn(command_handle: CommandHandle,
                                                      resp_json: CString,
                                                      cb: Option<extern fn(command_handle_: CommandHandle,
                                                                           err: Error,
                                                                           sources_json: CString,
                                                                           next: i64) -> Error>) -> Error;
pub type BuildPaymentReqCB = extern fn(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       submitter_did: CString,
                                       inputs_json: CString,
                                       outputs_json: CString,
                                       extra: CString,
                                       cb: Option<extern fn(command_handle_: CommandHandle,
                                                            err: Error,
                                                            payment_req_json: CString) -> Error>) -> Error;
pub type ParsePaymentResponseCB = extern fn(command_handle: CommandHandle,
                                            resp_json: CString,
                                            cb: Option<extern fn(command_handle_: CommandHandle,
                                                                 err: Error,
                                                                 receipts_json: CString) -> Error>) -> Error;
pub type BuildMintReqCB = extern fn(command_handle: CommandHandle,
                                    wallet_handle: WalletHandle,
                                    submitter_did: CString,
                                    outputs_json: CString,
                                    extra: CString,
                                    cb: Option<extern fn(command_handle_: CommandHandle,
                                                         err: Error,
                                                         mint_req_json: CString) -> Error>) -> Error;
pub type BuildSetTxnFeesReqCB = extern fn(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          submitter_did: CString,
                                          fees_json: CString,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
                                                               err: Error,
                                                               set_txn_fees_json: CString) -> Error>) -> Error;
pub type BuildGetTxnFeesReqCB = extern fn(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          submitter_did: CString,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
                                                               err: Error,
                                                               get_txn_fees_json: CString) -> Error>) -> Error;
pub type ParseGetTxnFeesResponseCB = extern fn(command_handle: CommandHandle,
                                               resp_json: CString,
                                               cb: Option<extern fn(command_handle_: CommandHandle,
                                                                    err: Error,
                                                                    fees_json: CString) -> Error>) -> Error;
pub type BuildVerifyPaymentReqCB = extern fn(command_handle: CommandHandle,
                                             wallet_handle: WalletHandle,
                                             submitter_did: CString,
                                             receipt: CString,
                                             cb: Option<extern fn(command_handle_: CommandHandle,
                                                                  err: Error,
                                                                  verify_txn_json: CString) -> Error>) -> Error;
pub type ParseVerifyPaymentResponseCB = extern fn(command_handle: CommandHandle,
                                                  resp_json: CString,
                                                  cb: Option<extern fn(command_handle_: CommandHandle,
                                                                       err: Error,
                                                                       txn_json: CString) -> Error>) -> Error;

pub type SignWithAddressCB = extern fn (command_handle: CommandHandle, wallet_handle: WalletHandle,
                                        address: CString,
                                        message_raw: BString, message_len: u32,
                                        cb: Option<extern fn(command_handle: i32, err: Error, raw: BString, len: u32)>) -> Error;

pub type VerifyWithAddressCB = extern fn (command_handle: i32, address: CString,
                                          message_raw: BString, message_len: u32,
                                          signature_raw: BString, signature_len: u32,
                                          cb: Option<extern fn(command_handle: i32, err: Error, result: bool)>) -> Error;

