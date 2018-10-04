use super::*;

use native::{CString, Error, Handle};

extern {
    #[no_mangle]
    pub fn indy_register_payment_method(command_handle: Handle,
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
                                        cb: Option<ResponseEmptyCB>) -> Error;
    #[no_mangle]
    pub fn indy_create_payment_address(command_handle: Handle,
                                       wallet_handle: Handle,
                                       payment_method: CString,
                                       config: CString,
                                       cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_list_payment_addresses(command_handle: Handle,
                                       wallet_handle: Handle,
                                       cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_add_request_fees(command_handle: Handle,
                                 wallet_handle: Handle,
                                 submitter_did: CString,
                                 req_json: CString,
                                 inputs_json: CString,
                                 outputs_json: CString,
                                 extra: CString,
                                 cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_response_with_fees(command_handle: Handle,
                                         payment_method: CString,
                                         resp_json: CString,
                                         cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_payment_sources_request(command_handle: Handle,
                                                  wallet_handle: Handle,
                                                  submitter_did: CString,
                                                  payment_address: CString,
                                                  cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_get_payment_sources_response(command_handle: Handle,
                                                   payment_method: CString,
                                                   resp_json: CString,
                                                   cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_payment_req(command_handle: Handle,
                                  wallet_handle: Handle,
                                  submitter_did: CString,
                                  inputs_json: CString,
                                  outputs_json: CString,
                                  extra: CString,
                                  cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_payment_response(command_handle: Handle,
                                       payment_method: CString,
                                       resp_json: CString,
                                       cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_mint_req(command_handle: Handle,
                               wallet_handle: Handle,
                               submitter_did: CString,
                               outputs_json: CString,
                               extra: CString,
                               cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_set_txn_fees_req(command_handle: Handle,
                                       wallet_handle: Handle,
                                       submitter_did: CString,
                                       payment_method: CString,
                                       fees_json: CString,
                                       cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_get_txn_fees_req(command_handle: Handle,
                                       wallet_handle: Handle,
                                       submitter_did: CString,
                                       payment_method: CString,
                                       cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_get_txn_fees_response(command_handle: Handle,
                                            payment_method: CString,
                                            resp_json: CString,
                                            cb: Option<ResponseStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_build_verify_payment_req(command_handle: Handle,
                                         wallet_handle: Handle,
                                         submitter_did: CString,
                                         receipt: CString,
                                         cb: Option<ResponseStringStringCB>) -> Error;
    #[no_mangle]
    pub fn indy_parse_verify_payment_response(command_handle: Handle,
                                              payment_method: CString,
                                              resp_json: CString,
                                              cb: Option<ResponseStringCB>) -> Error;
}

pub type CreatePaymentAddressCB = extern fn(command_handle: Handle,
                                            wallet_handle: Handle,
                                            config: CString,
                                            cb: Option<extern fn(command_handle_: Handle,
                                                                 err: Error,
                                                                 payment_address: CString) -> Error>) -> Error;
pub type AddRequestFeesCB = extern fn(command_handle: Handle,
                                      wallet_handle: Handle,
                                      submitter_did: CString,
                                      req_json: CString,
                                      inputs_json: CString,
                                      outputs_json: CString,
                                      extra: CString,
                                      cb: Option<extern fn(command_handle_: Handle,
                                                           err: Error,
                                                           req_with_fees_json: CString) -> Error>) -> Error;
pub type ParseResponseWithFeesCB = extern fn(command_handle: Handle,
                                             resp_json: CString,
                                             cb: Option<extern fn(command_handle_: Handle,
                                                                  err: Error,
                                                                  receipts_json: CString) -> Error>) -> Error;
pub type BuildGetPaymentSourcesRequestCB = extern fn(command_handle: Handle,
                                                     wallet_handle: Handle,
                                                     submitter_did: CString,
                                                     payment_address: CString,
                                                     cb: Option<extern fn(command_handle_: Handle,
                                                                          err: Error,
                                                                          get_sources_txn_json: CString) -> Error>) -> Error;
pub type ParseGetPaymentSourcesResponseCB = extern fn(command_handle: Handle,
                                                      resp_json: CString,
                                                      cb: Option<extern fn(command_handle_: Handle,
                                                                           err: Error,
                                                                           sources_json: CString) -> Error>) -> Error;
pub type BuildPaymentReqCB = extern fn(command_handle: Handle,
                                       wallet_handle: Handle,
                                       submitter_did: CString,
                                       inputs_json: CString,
                                       outputs_json: CString,
                                       extra: CString,
                                       cb: Option<extern fn(command_handle_: Handle,
                                                            err: Error,
                                                            payment_req_json: CString) -> Error>) -> Error;
pub type ParsePaymentResponseCB = extern fn(command_handle: Handle,
                                            resp_json: CString,
                                            cb: Option<extern fn(command_handle_: Handle,
                                                                 err: Error,
                                                                 receipts_json: CString) -> Error>) -> Error;
pub type BuildMintReqCB = extern fn(command_handle: Handle,
                                    wallet_handle: Handle,
                                    submitter_did: CString,
                                    outputs_json: CString,
                                    extra: CString,
                                    cb: Option<extern fn(command_handle_: Handle,
                                                         err: Error,
                                                         mint_req_json: CString) -> Error>) -> Error;
pub type BuildSetTxnFeesReqCB = extern fn(command_handle: Handle,
                                          wallet_handle: Handle,
                                          submitter_did: CString,
                                          fees_json: CString,
                                          cb: Option<extern fn(command_handle_: Handle,
                                                               err: Error,
                                                               set_txn_fees_json: CString) -> Error>) -> Error;
pub type BuildGetTxnFeesReqCB = extern fn(command_handle: Handle,
                                          wallet_handle: Handle,
                                          submitter_did: CString,
                                          cb: Option<extern fn(command_handle_: Handle,
                                                               err: Error,
                                                               get_txn_fees_json: CString) -> Error>) -> Error;
pub type ParseGetTxnFeesResponseCB = extern fn(command_handle: Handle,
                                               resp_json: CString,
                                               cb: Option<extern fn(command_handle_: Handle,
                                                                    err: Error,
                                                                    fees_json: CString) -> Error>) -> Error;
pub type BuildVerifyPaymentReqCB = extern fn(command_handle: Handle,
                                             wallet_handle: Handle,
                                             submitter_did: CString,
                                             receipt: CString,
                                             cb: Option<extern fn(command_handle_: Handle,
                                                           err: Error,
                                                           verify_txn_json: CString) -> Error>) -> Error;
pub type ParseVerifyPaymentResponseCB = extern fn(command_handle: Handle,
                                                  resp_json: CString,
                                                  cb: Option<extern fn(command_handle_: Handle,
                                                                err: Error,
                                                                txn_json: CString) -> Error>) -> Error;
