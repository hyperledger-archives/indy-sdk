use super::*;

use {Error, Handle, CString};

pub type PaymentResponseApiCB = extern fn(xcommand_handle: Handle,
                                         err: Error,
                                         json: CString) -> Error;

pub type CreatePaymentAddressCB = extern fn(command_handle: Handle,
                                                  wallet_handle: Handle,
                                                  config: CString,
                                                  cb: Option<PaymentResponseApiCB>) -> Error;
pub type AddRequestFeesCB = extern fn(command_handle: Handle,
                                            wallet_handle: Handle,
                                            submitter_did: CString,
                                            req_json: CString,
                                            inputs_json: CString,
                                            outputs_json: CString,
                                            cb: Option<PaymentResponseApiCB>) -> Error;
pub type ParseResponseWithFeesCB = extern fn(command_handle: Handle,
                                                   resp_json: CString,
                                                   cb: Option<PaymentResponseApiCB>) -> Error;
pub type BuildGetUTXORequestCB = extern fn(command_handle: Handle,
                                                 wallet_handle: Handle,
                                                 submitter_did: CString,
                                                 payment_address: CString,
                                                 cb: Option<PaymentResponseApiCB>) -> Error;
pub type ParseGetUTXOResponseCB = extern fn(command_handle: Handle,
                                                  resp_json: CString,
                                                  cb: Option<PaymentResponseApiCB>) -> Error;
pub type BuildPaymentRequestCB = extern fn(command_handle: Handle,
                                                 wallet_handle: Handle,
                                                 submitter_did: CString,
                                                 inputs_json: CString,
                                                 outputs_json: CString,
                                                 cb: Option<PaymentResponseApiCB>) -> Error;
pub type ParsePaymentResponseCB = extern fn(command_handle: Handle,
                                                  resp_json: CString,
                                                  cb: Option<PaymentResponseApiCB>) -> Error;
pub type BuildMintRequestCB = extern fn(command_handle: Handle,
                                              wallet_handle: Handle,
                                              submitter_did: CString,
                                              outputs_json: CString,
                                              cb: Option<PaymentResponseApiCB>) -> Error;
pub type BuildSetTxnFeesRequestCB = extern fn(command_handle: Handle,
                                                    wallet_handle: Handle,
                                                    submitter_did: CString,
                                                    fees_json: CString,
                                                    cb: Option<PaymentResponseApiCB>) -> Error;
pub type BuildGetTxnFeesRequestCB = extern fn(command_handle: Handle,
                                                    wallet_handle: Handle,
                                                    submitter_did: CString,
                                                    cb: Option<PaymentResponseApiCB>) -> Error;
pub type ParseGetTxnFeesResponseCB = extern fn(command_handle: Handle,
                                                     resp_json: CString,
                                                     cb: Option<PaymentResponseApiCB>) -> Error;

extern {
    #[no_mangle]
    pub fn indy_register_payment_method(command_handle: Handle,
                                        payment_method: CString,
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
                                 cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_response_with_fees(command_handle: Handle,
                                         payment_method: CString,
                                         resp_json: CString,
                                         cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_get_utxo_request(command_handle: Handle,
                                       wallet_handle: Handle,
                                       submitter_did: CString,
                                       payment_address: CString,
                                       cb: Option<ResponseStringStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_parse_get_utxo_response(command_handle: Handle,
                                    payment_method: CString,
                                    resp_json: CString,
                                    cb: Option<ResponseStringCB>) -> Error;

    #[no_mangle]
    pub fn indy_build_payment_req(command_handle: Handle,
                                  wallet_handle: Handle,
                                  submitter_did: CString,
                                  inputs_json: CString,
                                  outputs_json: CString,
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
}
