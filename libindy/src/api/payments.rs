extern crate libc;

use self::libc::c_char;
use api::ErrorCode;
use commands::{Command, CommandExecutor};
use commands::payments::PaymentsCommand;
use errors::ToErrorCode;
use services::payments::PaymentsMethodCBs;
use utils::cstring::CStringUtils;

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
/// wallet_handle: wallet handle where keys for signature are stored
/// config: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
///
/// #Returns
/// payment_address - public identifier of payment address in fully resolvable payment address format
pub type CreatePaymentAddressCB = extern fn(command_handle: i32,
                                            wallet_handle: i32,
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
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
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
pub type AddRequestFeesCB = extern fn(command_handle: i32,
                                      wallet_handle: i32,
                                      submitter_did: *const c_char,
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
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub type ParseResponseWithFeesCB = extern fn(command_handle: i32,
                                             resp_json: *const c_char,
                                             cb: Option<extern fn(command_handle_: i32,
                                                                  err: ErrorCode,
                                                                  utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// payment_address: target payment address
///
/// #Returns
/// get_utxo_txn_json - Indy request for getting UTXO list for payment address
pub type BuildGetUTXORequestCB = extern fn(command_handle: i32,
                                           wallet_handle: i32,
                                           submitter_did: *const c_char,
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
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub type ParseGetUTXOResponseCB = extern fn(command_handle: i32,
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
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
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
pub type BuildPaymentReqCB = extern fn(command_handle: i32,
                                       wallet_handle: i32,
                                       submitter_did: *const c_char,
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
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub type ParsePaymentResponseCB = extern fn(command_handle: i32,
                                            resp_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: ErrorCode,
                                                                 utxo_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for doing tokens minting
/// according to this payment method.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// outputs_json: The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// #Returns
/// mint_req_json - Indy request for doing tokens minting
pub type BuildMintReqCB = extern fn(command_handle: i32,
                                    wallet_handle: i32,
                                    submitter_did: *const c_char,
                                    outputs_json: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: ErrorCode,
                                                         mint_req_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for setting fees for transactions in the ledger
///
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
///
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
pub type BuildSetTxnFeesReqCB = extern fn(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          fees_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               set_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
pub type BuildGetTxnFeesReqCB = extern fn(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
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
pub type ParseGetTxnFeesResponseCB = extern fn(command_handle: i32,
                                               resp_json: *const c_char,
                                               cb: Option<extern fn(command_handle_: i32,
                                                                    err: ErrorCode,
                                                                    fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Register custom payment implementation.
///
/// It allows library user to provide custom payment method implementation as set of handlers.
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
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
                                                                err: ErrorCode)>) -> ErrorCode {
    trace!("indy_register_payment_method: >>> payment_method: {:?}", payment_method);

    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(create_payment_address, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(add_request_fees, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(parse_response_with_fees, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(build_get_utxo_request, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(parse_get_utxo_response, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(build_payment_req, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(parse_payment_response, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(build_mint_req, ErrorCode::CommonInvalidParam10);
    check_useful_c_callback!(build_set_txn_fees_req, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(build_get_txn_fees_req, ErrorCode::CommonInvalidParam12);
    check_useful_c_callback!(parse_get_txn_fees_response, ErrorCode::CommonInvalidParam13);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam14);

    trace!("indy_register_payment_method: entities >>> payment_method: {:?}", payment_method);

    let cbs = PaymentsMethodCBs::new(
        create_payment_address,
        add_request_fees,
        parse_response_with_fees,
        build_get_utxo_request,
        parse_get_utxo_response,
        build_payment_req,
        parse_payment_response,
        build_mint_req,
        build_set_txn_fees_req,
        build_get_txn_fees_req,
        parse_get_txn_fees_response
    );
    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::RegisterMethod(
                    payment_method,
                    cbs,
                    Box::new(move |result| {
                        cb(command_handle, result.to_error_code());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_register_payment_method: <<< res: {:?}", res);

    res
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
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle where to save new address
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
                                          wallet_handle: i32,
                                          payment_method: *const c_char,
                                          config: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               payment_address: *const c_char)>) -> ErrorCode {
    trace!("indy_create_payment_address: >>> wallet_handle: {:?}, payment_method: {:?}, config: {:?}", wallet_handle, payment_method, config);

    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(config, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_create_payment_address: entities >>> wallet_handle: {:?}, payment_method: {:?}, config: {:?}", wallet_handle, payment_method, config);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::CreateAddress(
                    wallet_handle,
                    payment_method,
                    config,
                    Box::new(move |result| {
                        let (err, address) = result_to_err_code_1!(result, String::new());
                        trace!("indy_create_payment_address: address: {:?}", address);
                        let address = CStringUtils::string_to_cstring(address);
                        cb(command_handle, err, address.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_create_payment_address: <<< res: {:?}", res);

    res
}

/// Lists all payment addresses that are stored in the wallet
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet to search for payment_addresses in
///
/// #Returns
/// payment_addresses_json - json array of string with json addresses
#[no_mangle]
pub extern fn indy_list_payment_addresses(command_handle: i32,
                                          wallet_handle: i32,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               payment_addresses_json: *const c_char)>) -> ErrorCode {
    trace!("indy_list_payment_address: >>> wallet_handle: {:?}", wallet_handle);

    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam3);

    trace!("indy_list_payment_address: entities >>> wallet_handle: {:?}", wallet_handle);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::ListAddresses(
                    wallet_handle,
                    Box::new(move |result| {
                        let (err, addresses_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_list_payment_address: addresses_json: {:?}", addresses_json);
                        let addresses_json = CStringUtils::string_to_cstring(addresses_json);
                        cb(command_handle, err, addresses_json.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_list_payment_address: <<< res: {:?}", res);

    res
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
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
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
                                    wallet_handle: i32,
                                    submitter_did: *const c_char,
                                    req_json: *const c_char,
                                    inputs_json: *const c_char,
                                    outputs_json: *const c_char,
                                    cb: Option<extern fn(command_handle_: i32,
                                                         err: ErrorCode,
                                                         req_with_fees_json: *const c_char,
                                                         payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_add_request_fees: >>> wallet_handle: {:?}, submitter_did: {:?}, req_json: {:?}, inputs_json: {:?}, outputs_json: {:?}", wallet_handle, submitter_did, req_json, inputs_json, outputs_json);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(req_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_add_request_fees: entities >>> wallet_handle: {:?}, submitter_did: {:?}, req_json: {:?}, inputs_json: {:?}, outputs_json: {:?}", wallet_handle, submitter_did, req_json, inputs_json, outputs_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::AddRequestFees(
                    wallet_handle,
                    submitter_did,
                    req_json,
                    inputs_json,
                    outputs_json,
                    Box::new(move |result| {
                        let (err, req_with_fees_json, payment_method) = result_to_err_code_2!(result, String::new(), String::new());
                        trace!("indy_add_request_fees: req_with_fees_json: {:?}, payment_method: {:?}", req_with_fees_json, payment_method);
                        let req_with_fees_json = CStringUtils::string_to_cstring(req_with_fees_json);
                        let payment_method = CStringUtils::string_to_cstring(payment_method);
                        cb(command_handle, err, req_with_fees_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_add_request_fees: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request with fees.
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
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
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_response_with_fees(command_handle: i32,
                                            payment_method: *const c_char,
                                            resp_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: i32,
                                                                 err: ErrorCode,
                                                                 utxo_json: *const c_char)>) -> ErrorCode {
    trace!("indy_parse_response_with_fees: >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(resp_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_parse_response_with_fees: entities >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::ParseResponseWithFees(
                    payment_method, resp_json, Box::new(move |result| {
                        let (err, utxo_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_parse_response_with_fees: utxo_json: {:?}", utxo_json);
                        let utxo_json = CStringUtils::string_to_cstring(utxo_json);
                        cb(command_handle, err, utxo_json.as_ptr());
                    }))
            ));
    let res = result_to_err_code!(result);

    trace!("indy_parse_response_with_fees: <<< res: {:?}", res);

    res
}

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// payment_address: target payment address
///
/// #Returns
/// get_utxo_txn_json - Indy request for getting UTXO list for payment address
/// payment_method
#[no_mangle]
pub extern fn indy_build_get_utxo_request(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          payment_address: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               get_utxo_txn_json: *const c_char,
                                                               payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_build_get_utxo_request: >>> wallet_handle: {:?}, submitter_did: {:?}, payment_address: {:?}", wallet_handle, submitter_did, payment_address);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(payment_address, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_build_get_utxo_request: entities >>> wallet_handle: {:?}, submitter_did: {:?}, payment_address: {:?}", wallet_handle, submitter_did, payment_address);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildGetUtxoRequest(
                    wallet_handle,
                    submitter_did,
                    payment_address,
                    Box::new(move |result| {
                        let (err, get_utxo_txn_json, payment_method) = result_to_err_code_2!(result, String::new(), String::new());
                        trace!("indy_build_get_utxo_request: get_utxo_txn_json: {:?}, payment_method: {:?}", get_utxo_txn_json, payment_method);
                        let get_utxo_txn_json = CStringUtils::string_to_cstring(get_utxo_txn_json);
                        let payment_method = CStringUtils::string_to_cstring(payment_method);
                        cb(command_handle, err, get_utxo_txn_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_build_get_utxo_request: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request for getting UTXO list.
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params
/// resp_json: response for Indy request for getting UTXO list
///   Note: this param will be used to determine payment_method
///
/// #Returns
/// utxo_json - parsed (payment method and node version agnostic) utxo info as json:
///   [{
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_get_utxo_response(command_handle: i32,
                                           payment_method: *const c_char,
                                           resp_json: *const c_char,
                                           cb: Option<extern fn(command_handle_: i32,
                                                                err: ErrorCode,
                                                                utxo_json: *const c_char)>) -> ErrorCode {
    trace!("indy_parse_get_utxo_response: >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(resp_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_parse_get_utxo_response: entities >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::ParseGetUtxoResponse(
                    payment_method,
                    resp_json,
                    Box::new(move |result| {
                        let (err, utxo_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_parse_get_utxo_response: utxo_json: {:?}", utxo_json);
                        let utxo_json = CStringUtils::string_to_cstring(utxo_json);
                        cb(command_handle, err, utxo_json.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_parse_get_utxo_response: <<< res: {:?}", res);

    res
}

/// Builds Indy request for doing tokens payment
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
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
                                     wallet_handle: i32,
                                     submitter_did: *const c_char,
                                     inputs_json: *const c_char,
                                     outputs_json: *const c_char,
                                     cb: Option<extern fn(command_handle_: i32,
                                                          err: ErrorCode,
                                                          payment_req_json: *const c_char,
                                                          payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_build_payment_req: >>> wallet_handle: {:?}, submitter_did: {:?}, inputs_json: {:?}, outputs_json: {:?}", wallet_handle, submitter_did, inputs_json, outputs_json);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_build_payment_req: entities >>> wallet_handle: {:?}, submitter_did: {:?}, inputs_json: {:?}, outputs_json: {:?}", wallet_handle, submitter_did, inputs_json, outputs_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildPaymentReq(
                    wallet_handle,
                    submitter_did,
                    inputs_json,
                    outputs_json,
                    Box::new(move |result| {
                        let (err, payment_req_json, payment_method) = result_to_err_code_2!(result, String::new(), String::new());
                        trace!("indy_build_payment_req: payment_req_json: {:?}, payment_method: {:?}", payment_req_json, payment_method);
                        let payment_req_json = CStringUtils::string_to_cstring(payment_req_json);
                        let payment_method = CStringUtils::string_to_cstring(payment_method);
                        cb(command_handle, err, payment_req_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_build_payment_req: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request for payment txn.
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
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
///      txo: <str>, // UTXO input
///      paymentAddress: <str>, //payment address for this UTXO
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_payment_response(command_handle: i32,
                                          payment_method: *const c_char,
                                          resp_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               utxo_json: *const c_char)>) -> ErrorCode {
    trace!("indy_parse_payment_response: >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(resp_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_parse_payment_response: entities >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::ParsePaymentResponse(
                    payment_method,
                    resp_json,
                    Box::new(move |result| {
                        let (err, utxo_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_parse_payment_response: utxo_json: {:?}", utxo_json);
                        let utxo_json = CStringUtils::string_to_cstring(utxo_json);
                        cb(command_handle, err, utxo_json.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_parse_payment_response: <<< res: {:?}", res);

    res
}

/// Builds Indy request for doing tokens minting
/// according to this payment method.
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// #Params
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
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
                                  wallet_handle: i32,
                                  submitter_did: *const c_char,
                                  outputs_json: *const c_char,
                                  cb: Option<extern fn(command_handle_: i32,
                                                       err: ErrorCode,
                                                       mint_req_json: *const c_char,
                                                       payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_build_mint_req: >>> wallet_handle: {:?}, submitter_did: {:?}, outputs_json: {:?}", wallet_handle, submitter_did, outputs_json);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_build_mint_req: entities >>> wallet_handle: {:?}, submitter_did: {:?}, outputs_json: {:?}", wallet_handle, submitter_did, outputs_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildMintReq(
                    wallet_handle,
                    submitter_did,
                    outputs_json,
                    Box::new(move |result| {
                        let (err, mint_req_json, payment_method) = result_to_err_code_2!(result, String::new(), String::new());
                        trace!("indy_build_mint_req: mint_req_json: {:?}, payment_method: {:?}", mint_req_json, payment_method);
                        let mint_req_json = CStringUtils::string_to_cstring(mint_req_json);
                        let payment_method = CStringUtils::string_to_cstring(payment_method);
                        cb(command_handle, err, mint_req_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_build_mint_req: <<< res: {:?}", res);

    res
}

/// Builds Indy request for setting fees for transactions in the ledger
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
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
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          payment_method: *const c_char,
                                          fees_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               set_txn_fees_json: *const c_char)>) -> ErrorCode {
    trace!("indy_build_set_txn_fees_req: >>> wallet_handle: {:?}, submitter_did: {:?}, payment_method: {:?}, fees_json: {:?}", wallet_handle, submitter_did, payment_method, fees_json);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(fees_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_build_set_txn_fees_req: entitites >>> wallet_handle: {:?}, submitter_did: {:?}, payment_method: {:?}, fees_json: {:?}", wallet_handle, submitter_did, payment_method, fees_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildSetTxnFeesReq(
                    wallet_handle,
                    submitter_did,
                    payment_method,
                    fees_json,
                    Box::new(move |result| {
                        let (err, set_txn_fees_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_build_set_txn_fees_req: set_txn_fees_json: {:?}", set_txn_fees_json);
                        let set_txn_fees_json = CStringUtils::string_to_cstring(set_txn_fees_json);
                        cb(command_handle, err, set_txn_fees_json.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_build_set_txn_fees_req: <<< res: {:?}", res);

    res
}

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
///
/// # Params
/// command_handle
/// wallet_handle: wallet handle
/// submitter_did : DID of request sender
/// payment_method
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
#[no_mangle]
pub extern fn indy_build_get_txn_fees_req(command_handle: i32,
                                          wallet_handle: i32,
                                          submitter_did: *const c_char,
                                          payment_method: *const c_char,
                                          cb: Option<extern fn(command_handle_: i32,
                                                               err: ErrorCode,
                                                               get_txn_fees_json: *const c_char)>) -> ErrorCode {
    trace!("indy_build_get_txn_fees_req: >>> wallet_handle: {:?}, submitter_did: {:?}, payment_method: {:?}", wallet_handle, submitter_did, payment_method);
    check_useful_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_build_get_txn_fees_req: entities >>> wallet_handle: {:?}, submitter_did: {:?}, payment_method: {:?}", wallet_handle, submitter_did, payment_method);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildGetTxnFeesReq(
                    wallet_handle,
                    submitter_did,
                    payment_method,
                    Box::new(move |result| {
                        let (err, get_txn_fees_json) = result_to_err_code_1!(result, String::new());
                        trace!("indy_build_get_txn_fees_req: entities >>> get_txn_fees_json: {:?}", get_txn_fees_json);
                        let get_txn_fees_json = CStringUtils::string_to_cstring(get_txn_fees_json);
                        cb(command_handle, err, get_txn_fees_json.as_ptr());
                    }))
            ));

    let res = result_to_err_code!(result);

    trace!("indy_build_get_txn_fees_req: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request for getting fees
///
/// Note this endpoint is EXPERIMENTAL. Function signature and behaviour may change
/// in the future releases.
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
                                                                    fees_json: *const c_char)>) -> ErrorCode {
    trace!("indy_parse_get_txn_fees_response: >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(resp_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_parse_get_txn_fees_response: entities >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);

    let result = CommandExecutor::instance().send(Command::Payments(
        PaymentsCommand::ParseGetTxnFeesResponse(payment_method, resp_json, Box::new(move |result| {
            let (err, fees_json) = result_to_err_code_1!(result, String::new());
            trace!("indy_parse_get_txn_fees_response: fees_json: {:?}", fees_json);
            let fees_json = CStringUtils::string_to_cstring(fees_json);
            cb(command_handle, err, fees_json.as_ptr());
        }))
    ));

    let res = result_to_err_code!(result);

    trace!("indy_parse_get_txn_fees_response: <<< res: {:?}", res);

    res
}