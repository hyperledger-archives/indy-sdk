//TODO FIXME
#![allow(unused_variables)]
extern crate libc;

use self::libc::c_char;
use api::ErrorCode;

/// Create the payment address for this payment method.
///
/// This method generates private part of payment address
/// and stores it in a secure place. Ideally it should be
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// #Params
/// command_handle: command handle to map callback to context
/// config: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
///
/// #Returns
/// payment_address - public identifier of payment address in fully resolvable payment address format
type CreatePaymentAddressCB = extern fn(command_handle: i32,
                                        config: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             payment_address: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Modifies Indy request by adding information how to pay fees for this transaction
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs. The difference between inputs balance
/// and outputs balance is the fee for this transaction.
///
/// Not that this method also produces correct fee signatures.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// req_json: initial transaction request as json
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Note that each input should reference paymentAddress
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// req_with_fees_json - modified Indy request with added fees info
type AddRequestFeesCB = extern fn(command_handle: i32,
                                  req_json: *const c_char,
                                  inputs_json: *const c_char,
                                  outputs_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32,
                                                       err: ErrorCode,
                                                       req_with_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request with fees.
///
/// #Params
/// command_handle
/// resp_json: response for Indy request with fees
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
type ParseResponseWithFeesCB = extern fn(command_handle: i32,
                                         resp_json: *const c_char,
                                         cb: Option<extern fn(command_handle_: i32,
                                                              err: ErrorCode,
                                                              utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// #Params
/// payment_address: target payment address
///
/// #Returns
/// get_utxo_txn_json - Indy request for getting UTXO list for payment address
type BuildGetUTXORequestCB = extern fn(command_handle: i32,
                                       payment_address: *const c_char,
                                       cb: Option<extern fn(command_handle_: i32,
                                                            err: ErrorCode,
                                                            get_utxo_txn_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for getting UTXO list.
///
/// #Params
/// resp_json: response for Indy request for getting UTXO list
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
type ParseGetUTXOResponseCB = extern fn(command_handle: i32,
                                        resp_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for doing tokens payment
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Note that each input should reference paymentAddress
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// payment_req_json - Indy request for doing tokens payment
type BuildPaymentReqCB = extern fn(command_handle: i32,
                                   inputs_json: *const c_char,
                                   outputs_json: *const c_char,
                                   cb: Option<extern fn(command_handle_: i32,
                                                        err: ErrorCode,
                                                        payment_req_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for payment txn.
///
/// #Params
/// command_handle
/// resp_json: response for Indy request for payment txn
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
type ParsePaymentResponseCB = extern fn(command_handle: i32,
                                        resp_json: *const c_char,
                                        cb: Option<extern fn(command_handle_: i32,
                                                             err: ErrorCode,
                                                             utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for doing tokens minting
/// according to this payment method.
///
/// #Params
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// mint_req_json - Indy request for doing tokens minting
type BuildMintReqCB = extern fn(command_handle: i32,
                                outputs_json: *const c_char,
                                cb: Option<extern fn(command_handle_: i32,
                                                     err: ErrorCode,
                                                     mint_req_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for setting fees for transactions in the ledger
///
/// # Params
/// command_handle
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
///
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
type BuildSetTxnFeesReqCB = extern fn(command_handle: i32,
                                      fees_json: *const c_char,
                                      cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           set_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// # Params
/// command_handle
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
type BuildGetTxnFeesReqCB = extern fn(command_handle: i32,
                                      cb: Option<extern fn(command_handle_: i32,
                                                           err: ErrorCode,
                                                           get_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for getting fees
///
/// # Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request for getting fees
///
/// # Return
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
type ParseGetTxnFeesResponseCB = extern fn(command_handle: i32,
                                           resp_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Register custom payment implementation.
///
/// It allows library user to provide custom payment method implementation as set of handlers.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: The type of payment method also used as sub-prefix for fully resolvable payment address format ("sov" - for example)
/// create_payment_address: "create_payment_address" operation handler
/// add_request_fees: "add_request_fees" operation handler
/// build_get_utxo_request: "build_get_utxo_request" operation handler
/// parse_get_utxo_response: "parse_get_utxo_response" operation handler
/// build_payment_req: "build_payment_req" operation handler
/// build_mint_req: "build_mint_req" operation handler
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_register_payment_method(command_handle: i32,
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

                                           cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Create the payment address for specified payment method
///
///
/// This method generates private part of payment address
/// and stores it in a secure place. Ideally it should be
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// #Params
/// command_handle: command handle to map callback to context
/// payment_method: Payment method to use (for example, 'sov')
/// config: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
///
/// #Returns
/// payment_address - public identifier of payment address in fully resolvable payment address format
#[no_mangle]
pub extern fn indy_create_payment_address(command_handle: i32,
                                          payment_method: *const c_char,
                                          config: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               payment_address: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Modifies Indy request by adding information how to pay fees for this transaction
/// according to selected payment method.
///
/// Payment selection is performed by looking to o
///
/// This method consumes set of UTXO inputs and outputs. The difference between inputs balance
/// and outputs balance is the fee for this transaction.
///
/// Not that this method also produces correct fee signatures.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// req_json: initial transaction request as json
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Notes:
///     - each input should reference paymentAddress
///     - this param will be used to determine payment_method
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// req_with_fees_json - modified Indy request with added fees info
/// payment_method
#[no_mangle]
pub extern fn indy_add_request_fees(command_handle: i32,
                                    req_json: *const c_char,
                                    inputs_json: *const c_char,
                                    outputs_json: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: ErrorCode,
                                                         req_with_fees_json: *const c_char,
                                                         payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Parses response for Indy request with fees.
///
/// #Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request with fees
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_response_with_fees(command_handle: i32,
                                            payment_method: *const c_char,
                                            resp_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: ErrorCode,
                                                                 utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// #Params
/// payment_address: target payment address
///
/// #Returns
/// get_utxo_txn_json - Indy request for getting UTXO list for payment address
/// payment_method
#[no_mangle]
pub extern fn indy_build_get_utxo_request(command_handle: i32,
                                          payment_address: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               get_utxo_txn_json: *const c_char,
                                                               payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Parses response for Indy request for getting UTXO list.
///
/// #Params
/// resp_json: response for Indy request for getting UTXO list
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_get_utxo_response(command_handle: i32,
                                           payment_method: *const c_char,
                                           resp_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Builds Indy request for doing tokens payment
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// inputs_json: The list of UTXO inputs as json array:
///   ["input1", ...]
///   Note that each input should reference paymentAddress
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// payment_req_json - Indy request for doing tokens payment
/// payment_method
#[no_mangle]
pub extern fn indy_build_payment_req(command_handle: i32,
                                     inputs_json: *const c_char,
                                     outputs_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32,
                                                          err: ErrorCode,
                                                          payment_req_json: *const c_char,
                                                          payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Parses response for Indy request for payment txn.
///
/// #Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request for payment txn
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_payment_response(command_handle: i32,
                                          payment_method: *const c_char,
                                          resp_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Builds Indy request for doing tokens minting
/// according to this payment method.
///
/// #Params
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// mint_req_json - Indy request for doing tokens minting
/// payment_method
#[no_mangle]
pub extern fn indy_build_mint_req(command_handle: i32,
                                  outputs_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32,
                                                       err: ErrorCode,
                                                       mint_req_json: *const c_char,
                                                       payment_method: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Builds Indy request for setting fees for transactions in the ledger
///
/// # Params
/// command_handle
/// payment_method
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
#[no_mangle]
pub extern fn indy_build_set_txn_fees_req(command_handle: i32,
                                          payment_method: *const c_char,
                                          fees_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               set_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// # Params
/// command_handle
/// payment_method
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
#[no_mangle]
pub extern fn indy_build_get_txn_fees_req(command_handle: i32,
                                          payment_method: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               get_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}

/// Parses response for Indy request for getting fees
///
/// # Params
/// command_handle
/// payment_method
/// resp_json: response for Indy request for getting fees
///
/// # Return
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
#[no_mangle]
pub extern fn indy_parse_get_txn_fees_response(command_handle: i32,
                                               payment_method: *const c_char,
                                               resp_json: *const c_char,
                                               cb: Option<extern fn(command_handle_: i32,
                                                                    err: ErrorCode,
                                                                    fees_json: *const c_char) -> ErrorCode>) -> ErrorCode {
    unimplemented!();
}
