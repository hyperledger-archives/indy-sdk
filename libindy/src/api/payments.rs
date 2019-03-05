extern crate libc;

use self::libc::c_char;
use api::{ErrorCode, CommandHandle, WalletHandle};
use commands::{Command, CommandExecutor};
use commands::payments::PaymentsCommand;
use services::payments::PaymentsMethodCBs;
use errors::prelude::*;
use utils::ctypes;

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
pub type CreatePaymentAddressCB = extern fn(command_handle: CommandHandle,
                                            wallet_handle: WalletHandle,
                                            config: *const c_char,
                                            cb: Option<extern fn(command_handle_: CommandHandle,
                                                                 err: ErrorCode,
                                                                 payment_address: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Modifies Indy request by adding information how to pay fees for this transaction
/// according to this payment method.
///
/// This method consumes set of inputs and outputs. The difference between inputs balance
/// and outputs balance is the fee for this transaction.
///
/// Not that this method also produces correct fee signatures.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// req_json: initial transaction request as json
/// inputs_json: The list of payment sources as json array:
///   ["source1", ...]
///   Note that each source should reference payment address
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// req_with_fees_json - modified Indy request with added fees info
pub type AddRequestFeesCB = extern fn(command_handle: CommandHandle,
                                      wallet_handle: WalletHandle,
                                      submitter_did: *const c_char,
                                      req_json: *const c_char,
                                      inputs_json: *const c_char,
                                      outputs_json: *const c_char,
                                      extra: *const c_char,
                                      cb: Option<extern fn(command_handle_: CommandHandle,
                                                           err: ErrorCode,
                                                           req_with_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request with fees.
///
/// #Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request with fees
///
/// #Returns
/// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
///   [{
///      receipt: <str>, // receipt that can be used for payment referencing and verification
///      recipient: <str>, //payment address for this recipient
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
pub type ParseResponseWithFeesCB = extern fn(command_handle: CommandHandle,
                                             resp_json: *const c_char,
                                             cb: Option<extern fn(command_handle_: CommandHandle,
                                                                  err: ErrorCode,
                                                                  receipts_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for getting sources list for payment address
/// according to this payment method.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// payment_address: target payment address
///
/// #Returns
/// get_sources_txn_json - Indy request for getting sources list for payment address
pub type BuildGetPaymentSourcesRequestCB = extern fn(command_handle: CommandHandle,
                                                     wallet_handle: WalletHandle,
                                                     submitter_did: *const c_char,
                                                     payment_address: *const c_char,
                                                     cb: Option<extern fn(command_handle_: CommandHandle,
                                                                          err: ErrorCode,
                                                                          get_sources_txn_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for getting sources list.
///
/// #Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for getting sources list
///
/// #Returns
/// sources_json - parsed (payment method and node version agnostic) sources info as json:
///   [{
///      source: <str>, // source input
///      paymentAddress: <str>, //payment address for this source
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
pub type ParseGetPaymentSourcesResponseCB = extern fn(command_handle: CommandHandle,
                                                      resp_json: *const c_char,
                                                      cb: Option<extern fn(command_handle_: CommandHandle,
                                                                           err: ErrorCode,
                                                                           sources_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for doing payment
/// according to this payment method.
///
/// This method consumes set of inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// inputs_json: The list of payment sources as json array:
///   ["source1", ...]
///   Note that each source should reference payment address
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// payment_req_json - Indy request for doing payment
pub type BuildPaymentReqCB = extern fn(command_handle: CommandHandle,
                                       wallet_handle: WalletHandle,
                                       submitter_did: *const c_char,
                                       inputs_json: *const c_char,
                                       outputs_json: *const c_char,
                                       extra: *const c_char,
                                       cb: Option<extern fn(command_handle_: CommandHandle,
                                                            err: ErrorCode,
                                                            payment_req_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for payment txn.
///
/// #Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for payment txn
///
/// #Returns
/// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
///   [{
///      receipt: <str>, // receipt that can be used for payment referencing and verification
///      recipient: <str>, //payment address for this receipt
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
pub type ParsePaymentResponseCB = extern fn(command_handle: CommandHandle,
                                            resp_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: CommandHandle,
                                                                 err: ErrorCode,
                                                                 receipts_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for doing minting
/// according to this payment method.
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// mint_req_json - Indy request for doing minting
pub type BuildMintReqCB = extern fn(command_handle: CommandHandle,
                                    wallet_handle: WalletHandle,
                                    submitter_did: *const c_char,
                                    outputs_json: *const c_char,
                                    extra: *const c_char,
                                    cb: Option<extern fn(command_handle_: CommandHandle,
                                                         err: ErrorCode,
                                                         mint_req_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for setting fees for transactions in the ledger
///
/// # Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
///
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
pub type BuildSetTxnFeesReqCB = extern fn(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          submitter_did: *const c_char,
                                          fees_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
                                                               err: ErrorCode,
                                                               set_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// # Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
pub type BuildGetTxnFeesReqCB = extern fn(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          submitter_did: *const c_char,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
                                                               err: ErrorCode,
                                                               get_txn_fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses response for Indy request for getting fees
///
/// # Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for getting fees
///
/// # Return
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
pub type ParseGetTxnFeesResponseCB = extern fn(command_handle: CommandHandle,
                                               resp_json: *const c_char,
                                               cb: Option<extern fn(command_handle_: CommandHandle,
                                                                    err: ErrorCode,
                                                                    fees_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Builds Indy request for getting information to verify the payment receipt
///
/// # Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// receipt: payment receipt to verify
///
/// # Return
/// verify_txn_json -- request to be sent to ledger
pub type BuildVerifyPaymentReqCB = extern fn(command_handle: CommandHandle,
                                             wallet_handle: WalletHandle,
                                             submitter_did: *const c_char,
                                             receipt: *const c_char,
                                             cb: Option<extern fn(command_handle_: CommandHandle,
                                                           err: ErrorCode,
                                                           verify_txn_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Parses Indy response with information to verify receipt
///
/// # Params
/// command_handle: command handle to map callback to context
/// resp_json: response for Indy request for information to verify the payment receipt
///
/// # Return
/// txn_json: {
///     sources: [<str>, ]
///     receipts: [ {
///         recipient: <str>, // payment address of recipient
///         receipt: <str>, // receipt that can be used for payment referencing and verification
///         amount: <int>, // amount
///     }, ]
///     extra: <str>, //optional data
/// }
pub type ParseVerifyPaymentResponseCB = extern fn(command_handle: CommandHandle,
                                                  resp_json: *const c_char,
                                                  cb: Option<extern fn(command_handle_: CommandHandle,
                                                                err: ErrorCode,
                                                                txn_json: *const c_char) -> ErrorCode>) -> ErrorCode;

/// Register custom payment implementation.
///
/// It allows library user to provide custom payment method implementation as set of handlers.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: The type of payment method also used as sub-prefix for fully resolvable payment address format ("sov" - for example)
/// create_payment_address: "create_payment_address" operation handler
/// add_request_fees: "add_request_fees" operation handler
/// parse_response_with_fees: "parse_response_with_fees" operation handler
/// build_get_payment_sources_request: "build_get_payment_sources_request" operation handler
/// parse_get_payment_sources_response: "parse_get_payment_sources_response" operation handler
/// build_payment_req: "build_payment_req" operation handler
/// parse_payment_response: "parse_payment_response" operation handler
/// build_mint_req: "build_mint_req" operation handler
/// build_set_txn_fees_req: "build_set_txn_fees_req" operation handler
/// build_get_txn_fees_req: "build_get_txn_fees_req" operation handler
/// parse_get_txn_fees_response: "parse_get_txn_fees_response" operation handler
/// build_verify_payment_req: "build_verify_payment_req" operation handler
/// parse_verify_payment_response: "parse_verify_payment_response" operation handler
///
/// #Returns
/// Error code
#[no_mangle]
pub extern fn indy_register_payment_method(command_handle: CommandHandle,
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
                                           cb: Option<extern fn(command_handle_: CommandHandle,
                                                                err: ErrorCode)>) -> ErrorCode {
    trace!("indy_register_payment_method: >>> payment_method: {:?}", payment_method);

    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_callback!(create_payment_address, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(add_request_fees, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(parse_response_with_fees, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(build_get_payment_sources_request, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(parse_get_payment_sources_response, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(build_payment_req, ErrorCode::CommonInvalidParam8);
    check_useful_c_callback!(parse_payment_response, ErrorCode::CommonInvalidParam9);
    check_useful_c_callback!(build_mint_req, ErrorCode::CommonInvalidParam10);
    check_useful_c_callback!(build_set_txn_fees_req, ErrorCode::CommonInvalidParam11);
    check_useful_c_callback!(build_get_txn_fees_req, ErrorCode::CommonInvalidParam12);
    check_useful_c_callback!(parse_get_txn_fees_response, ErrorCode::CommonInvalidParam13);
    check_useful_c_callback!(build_verify_payment_req, ErrorCode::CommonInvalidParam14);
    check_useful_c_callback!(parse_verify_payment_response, ErrorCode::CommonInvalidParam15);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam16);

    trace!("indy_register_payment_method: entities >>> payment_method: {:?}", payment_method);

    let cbs = PaymentsMethodCBs::new(
        create_payment_address,
        add_request_fees,
        parse_response_with_fees,
        build_get_payment_sources_request,
        parse_get_payment_sources_response,
        build_payment_req,
        parse_payment_response,
        build_mint_req,
        build_set_txn_fees_req,
        build_get_txn_fees_req,
        parse_get_txn_fees_response,
        build_verify_payment_req,
        parse_verify_payment_response,
    );
    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::RegisterMethod(
                    payment_method,
                    cbs,
                    Box::new(move |result| {
                        cb(command_handle, result.into());
                    }))
            ));

    let res = prepare_result!(result);

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
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet handle where to save new address
/// payment_method: payment method to use (for example, 'sov')
/// config: payment address config as json:
///   {
///     seed: <str>, // allows deterministic creation of payment address
///   }
///
/// #Returns
/// payment_address - public identifier of payment address in fully resolvable payment address format
#[no_mangle]
pub extern fn indy_create_payment_address(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          payment_method: *const c_char,
                                          config: *const c_char,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
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
                        let (err, address) = prepare_result_1!(result, String::new());
                        trace!("indy_create_payment_address: address: {:?}", address);
                        let address = ctypes::string_to_cstring(address);
                        cb(command_handle, err, address.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_create_payment_address: <<< res: {:?}", res);

    res
}

/// Lists all payment addresses that are stored in the wallet
///
/// #Params
/// command_handle: command handle to map callback to context
/// wallet_handle: wallet to search for payment_addresses in
///
/// #Returns
/// payment_addresses_json - json array of string with json addresses
#[no_mangle]
pub extern fn indy_list_payment_addresses(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
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
                        let (err, addresses_json) = prepare_result_1!(result, String::new());
                        trace!("indy_list_payment_address: addresses_json: {:?}", addresses_json);
                        let addresses_json = ctypes::string_to_cstring(addresses_json);
                        cb(command_handle, err, addresses_json.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_list_payment_address: <<< res: {:?}", res);

    res
}

/// Modifies Indy request by adding information how to pay fees for this transaction
/// according to this payment method.
///
/// This method consumes set of inputs and outputs. The difference between inputs balance
/// and outputs balance is the fee for this transaction.
///
/// Not that this method also produces correct fee signatures.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// req_json: initial transaction request as json
/// inputs_json: The list of payment sources as json array:
///   ["source1", ...]
///     - each input should reference paymentAddress
///     - this param will be used to determine payment_method
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// req_with_fees_json - modified Indy request with added fees info
/// payment_method - used payment method
#[no_mangle]
pub extern fn indy_add_request_fees(command_handle: CommandHandle,
                                    wallet_handle: WalletHandle,
                                    submitter_did: *const c_char,
                                    req_json: *const c_char,
                                    inputs_json: *const c_char,
                                    outputs_json: *const c_char,
                                    extra: *const c_char,
                                    cb: Option<extern fn(command_handle_: CommandHandle,
                                                         err: ErrorCode,
                                                         req_with_fees_json: *const c_char,
                                                         payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_add_request_fees: >>> wallet_handle: {:?}, submitter_did: {:?}, req_json: {:?}, inputs_json: {:?}, outputs_json: {:?}, extra: {:?}",
           wallet_handle, submitter_did, req_json, inputs_json, outputs_json, extra);
    check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(req_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidParam5);
    check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidParam6);
    check_useful_opt_c_str!(extra, ErrorCode::CommonInvalidParam7);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam8);

    trace!("indy_add_request_fees: entities >>> wallet_handle: {:?}, submitter_did: {:?}, req_json: {:?}, inputs_json: {:?}, outputs_json: {:?}, extra: {:?}",
           wallet_handle, submitter_did, req_json, inputs_json, outputs_json, extra);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::AddRequestFees(
                    wallet_handle,
                    submitter_did,
                    req_json,
                    inputs_json,
                    outputs_json,
                    extra,
                    Box::new(move |result| {
                        let (err, req_with_fees_json, payment_method) = prepare_result_2!(result, String::new(), String::new());
                        trace!("indy_add_request_fees: req_with_fees_json: {:?}, payment_method: {:?}", req_with_fees_json, payment_method);
                        let req_with_fees_json = ctypes::string_to_cstring(req_with_fees_json);
                        let payment_method = ctypes::string_to_cstring(payment_method);
                        cb(command_handle, err, req_with_fees_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_add_request_fees: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request with fees.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: payment method to use
/// resp_json: response for Indy request with fees
///
/// #Returns
/// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
///   [{
///      receipt: <str>, // receipt that can be used for payment referencing and verification
///      recipient: <str>, //payment address of recipient
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_response_with_fees(command_handle: CommandHandle,
                                            payment_method: *const c_char,
                                            resp_json: *const c_char,
                                            cb: Option<extern fn(command_handle_: CommandHandle,
                                                                 err: ErrorCode,
                                                                 receipts_json: *const c_char)>) -> ErrorCode {
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
                        let (err, receipts_json) = prepare_result_1!(result, String::new());
                        trace!("indy_parse_response_with_fees: receipts_json: {:?}", receipts_json);
                        let receipts_json = ctypes::string_to_cstring(receipts_json);
                        cb(command_handle, err, receipts_json.as_ptr());
                    }))
            ));
    let res = prepare_result!(result);

    trace!("indy_parse_response_with_fees: <<< res: {:?}", res);

    res
}

/// Builds Indy request for getting sources list for payment address
/// according to this payment method.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// payment_address: target payment address
///
/// #Returns
/// get_sources_txn_json - Indy request for getting sources list for payment address
/// payment_method - used payment method
#[no_mangle]
pub extern fn indy_build_get_payment_sources_request(command_handle: CommandHandle,
                                                     wallet_handle: WalletHandle,
                                                     submitter_did: *const c_char,
                                                     payment_address: *const c_char,
                                                     cb: Option<extern fn(command_handle_: CommandHandle,
                                                                          err: ErrorCode,
                                                                          get_sources_txn_json: *const c_char,
                                                                          payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_build_get_payment_sources_request: >>> wallet_handle: {:?}, submitter_did: {:?}, payment_address: {:?}", wallet_handle, submitter_did, payment_address);
    check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(payment_address, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_build_get_payment_sources_request: entities >>> wallet_handle: {:?}, submitter_did: {:?}, payment_address: {:?}", wallet_handle, submitter_did, payment_address);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildGetPaymentSourcesRequest(
                    wallet_handle,
                    submitter_did,
                    payment_address,
                    Box::new(move |result| {
                        let (err, get_sources_txn_json, payment_method) = prepare_result_2!(result, String::new(), String::new());
                        trace!("indy_build_get_payment_sources_request: get_sources_txn_json: {:?}, payment_method: {:?}", get_sources_txn_json, payment_method);
                        let get_sources_txn_json = ctypes::string_to_cstring(get_sources_txn_json);
                        let payment_method = ctypes::string_to_cstring(payment_method);
                        cb(command_handle, err, get_sources_txn_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_build_get_payment_sources_request: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request for getting sources list.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: payment method to use.
/// resp_json: response for Indy request for getting sources list
///
/// #Returns
/// sources_json - parsed (payment method and node version agnostic) sources info as json:
///   [{
///      source: <str>, // source input
///      paymentAddress: <str>, //payment address for this source
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_get_payment_sources_response(command_handle: CommandHandle,
                                                      payment_method: *const c_char,
                                                      resp_json: *const c_char,
                                                      cb: Option<extern fn(command_handle_: CommandHandle,
                                                                           err: ErrorCode,
                                                                           sources_json: *const c_char)>) -> ErrorCode {
    trace!("indy_parse_get_payment_sources_response: >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(resp_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_parse_get_payment_sources_response: entities >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::ParseGetPaymentSourcesResponse(
                    payment_method,
                    resp_json,
                    Box::new(move |result| {
                        let (err, sources_json) = prepare_result_1!(result, String::new());
                        trace!("indy_parse_get_payment_sources_response: sources_json: {:?}", sources_json);
                        let sources_json = ctypes::string_to_cstring(sources_json);
                        cb(command_handle, err, sources_json.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_parse_get_payment_sources_response: <<< res: {:?}", res);

    res
}

/// Builds Indy request for doing payment
/// according to this payment method.
///
/// This method consumes set of inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// inputs_json: The list of payment sources as json array:
///   ["source1", ...]
///   Note that each source should reference payment address
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for payment operation
///
/// #Returns
/// payment_req_json - Indy request for doing payment
/// payment_method - used payment method
#[no_mangle]
pub extern fn indy_build_payment_req(command_handle: CommandHandle,
                                     wallet_handle: WalletHandle,
                                     submitter_did: *const c_char,
                                     inputs_json: *const c_char,
                                     outputs_json: *const c_char,
                                     extra: *const c_char,
                                     cb: Option<extern fn(command_handle_: CommandHandle,
                                                          err: ErrorCode,
                                                          payment_req_json: *const c_char,
                                                          payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_build_payment_req: >>> wallet_handle: {:?}, submitter_did: {:?}, inputs_json: {:?}, outputs_json: {:?}, extra: {:?}",
           wallet_handle, submitter_did, inputs_json, outputs_json, extra);
    check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(inputs_json, ErrorCode::CommonInvalidParam4);
    check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidParam5);
    check_useful_opt_c_str!(extra, ErrorCode::CommonInvalidParam6);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam7);

    trace!("indy_build_payment_req: entities >>> wallet_handle: {:?}, submitter_did: {:?}, inputs_json: {:?}, outputs_json: {:?}, extra: {:?}",
           wallet_handle, submitter_did, inputs_json, outputs_json, extra);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildPaymentReq(
                    wallet_handle,
                    submitter_did,
                    inputs_json,
                    outputs_json,
                    extra,
                    Box::new(move |result| {
                        let (err, payment_req_json, payment_method) = prepare_result_2!(result, String::new(), String::new());
                        trace!("indy_build_payment_req: payment_req_json: {:?}, payment_method: {:?}", payment_req_json, payment_method);
                        let payment_req_json = ctypes::string_to_cstring(payment_req_json);
                        let payment_method = ctypes::string_to_cstring(payment_method);
                        cb(command_handle, err, payment_req_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_build_payment_req: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request for payment txn.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: payment method to use
/// resp_json: response for Indy request for payment txn
///
/// #Returns
/// receipts_json - parsed (payment method and node version agnostic) receipts info as json:
///   [{
///      receipt: <str>, // receipt that can be used for payment referencing and verification
///      recipient: <str>, // payment address of recipient
///      amount: <int>, // amount
///      extra: <str>, // optional data from payment transaction
///   }]
#[no_mangle]
pub extern fn indy_parse_payment_response(command_handle: CommandHandle,
                                          payment_method: *const c_char,
                                          resp_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
                                                               err: ErrorCode,
                                                               receipts_json: *const c_char)>) -> ErrorCode {
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
                        let (err, receipts_json) = prepare_result_1!(result, String::new());
                        trace!("indy_parse_payment_response: receipts_json: {:?}", receipts_json);
                        let receipts_json = ctypes::string_to_cstring(receipts_json);
                        cb(command_handle, err, receipts_json.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_parse_payment_response: <<< res: {:?}", res);

    res
}

/// Builds Indy request for doing minting
/// according to this payment method.
///
/// #Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// outputs_json: The list of outputs as json array:
///   [{
///     recipient: <str>, // payment address of recipient
///     amount: <int>, // amount
///   }]
/// extra: // optional information for mint operation
///
/// #Returns
/// mint_req_json - Indy request for doing minting
/// payment_method - used payment method
#[no_mangle]
pub extern fn indy_build_mint_req(command_handle: CommandHandle,
                                  wallet_handle: WalletHandle,
                                  submitter_did: *const c_char,
                                  outputs_json: *const c_char,
                                  extra: *const c_char,
                                  cb: Option<extern fn(command_handle_: CommandHandle,
                                                       err: ErrorCode,
                                                       mint_req_json: *const c_char,
                                                       payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_build_mint_req: >>> wallet_handle: {:?}, submitter_did: {:?}, outputs_json: {:?}, extra: {:?}", wallet_handle, submitter_did, outputs_json, extra);
    check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(outputs_json, ErrorCode::CommonInvalidParam4);
    check_useful_opt_c_str!(extra, ErrorCode::CommonInvalidParam5);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam6);

    trace!("indy_build_mint_req: entities >>> wallet_handle: {:?}, submitter_did: {:?}, outputs_json: {:?}, extra: {:?}", wallet_handle, submitter_did, outputs_json, extra);

    let result =
        CommandExecutor::instance().send(
            Command::Payments(
                PaymentsCommand::BuildMintReq(
                    wallet_handle,
                    submitter_did,
                    outputs_json,
                    extra,
                    Box::new(move |result| {
                        let (err, mint_req_json, payment_method) = prepare_result_2!(result, String::new(), String::new());
                        trace!("indy_build_mint_req: mint_req_json: {:?}, payment_method: {:?}", mint_req_json, payment_method);
                        let mint_req_json = ctypes::string_to_cstring(mint_req_json);
                        let payment_method = ctypes::string_to_cstring(payment_method);
                        cb(command_handle, err, mint_req_json.as_ptr(), payment_method.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_build_mint_req: <<< res: {:?}", res);

    res
}

/// Builds Indy request for setting fees for transactions in the ledger
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// payment_method: payment method to use
/// fees_json {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
/// # Return
/// set_txn_fees_json - Indy request for setting fees for transactions in the ledger
#[no_mangle]
pub extern fn indy_build_set_txn_fees_req(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          submitter_did: *const c_char,
                                          payment_method: *const c_char,
                                          fees_json: *const c_char,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
                                                               err: ErrorCode,
                                                               set_txn_fees_json: *const c_char)>) -> ErrorCode {
    trace!("indy_build_set_txn_fees_req: >>> wallet_handle: {:?}, submitter_did: {:?}, payment_method: {:?}, fees_json: {:?}", wallet_handle, submitter_did, payment_method, fees_json);
    check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
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
                        let (err, set_txn_fees_json) = prepare_result_1!(result, String::new());
                        trace!("indy_build_set_txn_fees_req: set_txn_fees_json: {:?}", set_txn_fees_json);
                        let set_txn_fees_json = ctypes::string_to_cstring(set_txn_fees_json);
                        cb(command_handle, err, set_txn_fees_json.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_build_set_txn_fees_req: <<< res: {:?}", res);

    res
}

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// payment_method: payment method to use
///
/// # Return
/// get_txn_fees_json - Indy request for getting fees for transactions in the ledger
#[no_mangle]
pub extern fn indy_build_get_txn_fees_req(command_handle: CommandHandle,
                                          wallet_handle: WalletHandle,
                                          submitter_did: *const c_char,
                                          payment_method: *const c_char,
                                          cb: Option<extern fn(command_handle_: CommandHandle,
                                                               err: ErrorCode,
                                                               get_txn_fees_json: *const c_char)>) -> ErrorCode {
    trace!("indy_build_get_txn_fees_req: >>> wallet_handle: {:?}, submitter_did: {:?}, payment_method: {:?}", wallet_handle, submitter_did, payment_method);
    check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
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
                        let (err, get_txn_fees_json) = prepare_result_1!(result, String::new());
                        trace!("indy_build_get_txn_fees_req: entities >>> get_txn_fees_json: {:?}", get_txn_fees_json);
                        let get_txn_fees_json = ctypes::string_to_cstring(get_txn_fees_json);
                        cb(command_handle, err, get_txn_fees_json.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_build_get_txn_fees_req: <<< res: {:?}", res);

    res
}

/// Parses response for Indy request for getting fees
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: payment method to use
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
pub extern fn indy_parse_get_txn_fees_response(command_handle: CommandHandle,
                                               payment_method: *const c_char,
                                               resp_json: *const c_char,
                                               cb: Option<extern fn(command_handle_: CommandHandle,
                                                                    err: ErrorCode,
                                                                    fees_json: *const c_char)>) -> ErrorCode {
    trace!("indy_parse_get_txn_fees_response: >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(resp_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_parse_get_txn_fees_response: entities >>> payment_method: {:?}, resp_json: {:?}", payment_method, resp_json);

    let result =
        CommandExecutor::instance()
            .send(Command::Payments(
                PaymentsCommand::ParseGetTxnFeesResponse(
                    payment_method,
                    resp_json,
                    Box::new(move |result| {
                        let (err, fees_json) = prepare_result_1!(result, String::new());
                        trace!("indy_parse_get_txn_fees_response: fees_json: {:?}", fees_json);
                        let fees_json = ctypes::string_to_cstring(fees_json);
                        cb(command_handle, err, fees_json.as_ptr());
                    }))
            ));

    let res = prepare_result!(result);

    trace!("indy_parse_get_txn_fees_response: <<< res: {:?}", res);

    res
}

/// Builds Indy request for information to verify the payment receipt
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// wallet_handle: wallet handle
/// submitter_did: (Optional) DID of request sender
/// receipt: payment receipt to verify
///
/// # Return
/// verify_txn_json: Indy request for verification receipt
/// payment_method: used payment method
#[no_mangle]
pub extern fn indy_build_verify_payment_req(command_handle: CommandHandle,
                                            wallet_handle: WalletHandle,
                                            submitter_did: *const c_char,
                                            receipt: *const c_char,
                                            cb: Option<extern fn(command_handle_: CommandHandle,
                                                         err: ErrorCode,
                                                         verify_txn_json: *const c_char,
                                                         payment_method: *const c_char)>) -> ErrorCode {
    trace!("indy_build_verify_payment_req: >>> wallet_handle {:?}, submitter_did: {:?}, receipt: {:?}", wallet_handle, submitter_did, receipt);
    check_useful_opt_c_str!(submitter_did, ErrorCode::CommonInvalidParam3);
    check_useful_c_str!(receipt, ErrorCode::CommonInvalidParam4);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam5);

    trace!("indy_build_verify_payment_req: entities >>> wallet_handle {:?}, submitter_did: {:?}, receipt: {:?}", wallet_handle, submitter_did, receipt);

    let result = CommandExecutor::instance()
        .send(Command::Payments(
            PaymentsCommand::BuildVerifyPaymentReq(
                wallet_handle,
                submitter_did,
                receipt,
                Box::new(move |result| {
                    let (err, verify_txn_json, payment_method) = prepare_result_2!(result, String::new(), String::new());
                    trace!("indy_build_verify_payment_req: verify_txn_json: {:?}, payment_method: {:?}", verify_txn_json, payment_method);
                    let verify_txn_json = ctypes::string_to_cstring(verify_txn_json);
                    let payment_method = ctypes::string_to_cstring(payment_method);
                    cb(command_handle, err, verify_txn_json.as_ptr(), payment_method.as_ptr());
                })
            )));

    let result = prepare_result!(result);

    trace!("indy_build_verify_payment_req: <<< result: {:?}", result);

    result
}

/// Parses Indy response with information to verify receipt
///
/// # Params
/// command_handle: Command handle to map callback to caller context.
/// payment_method: payment method to use
/// resp_json: response of the ledger for verify txn
///
/// # Return
/// txn_json: {
///     sources: [<str>, ]
///     receipts: [ {
///         recipient: <str>, // payment address of recipient
///         receipt: <str>, // receipt that can be used for payment referencing and verification
///         amount: <int>, // amount
///     } ],
///     extra: <str>, //optional data
/// }
#[no_mangle]
pub extern fn indy_parse_verify_payment_response(command_handle: CommandHandle,
                                                 payment_method: *const c_char,
                                                 resp_json: *const c_char,
                                                 cb: Option<extern fn(command_handle_: CommandHandle,
                                                              err: ErrorCode,
                                                              txn_json: *const c_char)>) -> ErrorCode {
    trace!("indy_parse_verify_payment_response: >>> resp_json: {:?}", resp_json);
    check_useful_c_str!(payment_method, ErrorCode::CommonInvalidParam2);
    check_useful_c_str!(resp_json, ErrorCode::CommonInvalidParam3);
    check_useful_c_callback!(cb, ErrorCode::CommonInvalidParam4);

    trace!("indy_parse_verify_payment_response: entities >>> resp_json: {:?}", resp_json);

    let result = CommandExecutor::instance()
        .send(Command::Payments(
            PaymentsCommand::ParseVerifyPaymentResponse(
                payment_method,
                resp_json,
                Box::new(move |result| {
                    let (err, txn_json) = prepare_result_1!(result, String::new());
                    trace!("indy_parse_verify_payment_response: txn_json: {:?}", txn_json);
                    let txn_json = ctypes::string_to_cstring(txn_json);
                    cb(command_handle, err, txn_json.as_ptr());
                })
            )));

    let result = prepare_result!(result);

    trace!("indy_parse_verify_payment_response: <<< result: {:?}", result);

    result
}
