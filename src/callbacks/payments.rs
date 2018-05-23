use ErrorCode;
use ffi::payments;

pub type PaymentResponseCallback = extern fn(xcommand_handle: i32,
                                             err: ErrorCode,
                                             json: &str);
pub type PaymentsMethodResponseCallback = extern fn(xcommand_handle: i32,
                                                    err: ErrorCode,
                                                    json: &str,
                                                    payment_method: &str);

pub type CreatePaymentAddressCallback = extern fn(wallet_handle: i32,
                                                  config: &str,
                                                  ) -> Result<String, ErrorCode>;

pub type AddRequestFeesCallback = extern fn(command_handle: i32,
                                            wallet_handle: i32,
                                            submitter_did: &str,
                                            req_json: &str,
                                            inputs_json: &str,
                                            outputs_json: &str,
                                            cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type ParseResponseWithFeesCallback = extern fn(command_handle: i32,
                                                   resp_json: &str,
                                                   cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type BuildGetUTXORequestCallback = extern fn(command_handle: i32,
                                                 wallet_handle: i32,
                                                 submitter_did: &str,
                                                 payment_address: &str,
                                                 cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type ParseGetUTXOResponseCallback = extern fn(command_handle: i32,
                                                  resp_json: &str,
                                                  cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type BuildPaymentRequestCallback = extern fn(command_handle: i32,
                                                 wallet_handle: i32,
                                                 submitter_did: &str,
                                                 inputs_json: &str,
                                                 outputs_json: &str,
                                                 cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type ParsePaymentResponseCallback = extern fn(command_handle: i32,
                                                  resp_json: &str,
                                                  cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type BuildMintRequestCallback = extern fn(command_handle: i32,
                                              wallet_handle: i32,
                                              submitter_did: &str,
                                              outputs_json: &str,
                                              cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type BuildSetTxnFeesRequestCallback = extern fn(command_handle: i32,
                                                    wallet_handle: i32,
                                                    submitter_did: &str,
                                                    fees_json: &str,
                                                    cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type BuildGetTxnFeesRequestCallback = extern fn(command_handle: i32,
                                                    wallet_handle: i32,
                                                    submitter_did: &str,
                                                    cb: Option<PaymentResponseCallback>) -> ErrorCode;
pub type ParseGetTxnFeesResponseCallback = extern fn(command_handle: i32,
                                                     resp_json: &str,
                                                     cb: Option<PaymentResponseCallback>) -> ErrorCode;
