use {ErrorCode, IndyHandle, IndyError};

use std::ffi::CString;
use std::ptr::null;

use futures::Future;

use ffi::payments;
use ffi::{ResponseStringCB,
          ResponseStringStringCB};

use utils::callbacks::{ClosureHandler, ResultHandler};

/// Create the payment address for specified payment method
///
/// This method generates private part of payment address
/// and stores it in a secure place. Ideally it should be
/// secret in libindy wallet (see crypto module).
///
/// Note that payment method should be able to resolve this
/// secret by fully resolvable payment address format.
///
/// # Arguments
/// * `wallet_handle` - wallet handle where to save new address
/// * `payment_method` - payment method to use (for example, 'sov')
/// * `config` - payment address config as json
///
/// # Example
/// config
/// {
///   seed: <str>, // allows deterministic creation of payment address
/// }
///
/// # Returns
/// * `payment_address` - public identifier of payment address in fully resolvable payment address format
pub fn create_payment_address(wallet_handle: IndyHandle, payment_method: &str, config: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _create_payment_address(command_handle, wallet_handle, payment_method, config, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _create_payment_address(command_handle: IndyHandle, wallet_handle: IndyHandle, payment_method: &str, config: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let config = c_str!(config);

    ErrorCode::from(unsafe { payments::indy_create_payment_address(command_handle, wallet_handle, payment_method.as_ptr(), config.as_ptr(), cb) })
}

/// Lists all payment addresses that are stored in the wallet
///
/// # Arguments
/// * `wallet_handle` - wallet to search for payment_addresses
///
/// # Returns
/// * `payment_addresses_json` - json array of string with json addresses
pub fn list_payment_addresses(wallet_handle: IndyHandle) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _list_payment_addresses(command_handle, wallet_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _list_payment_addresses(command_handle: IndyHandle, wallet_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
    ErrorCode::from(unsafe { payments::indy_list_payment_addresses(command_handle, wallet_handle, cb) })
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
/// # Arguments
/// * `wallet_handle` - wallet handle
/// * `submitter_did` - DID of request sender
/// * `req_json` - initial transaction request as json
/// * `inputs_json` - the list of UTXO inputs as json array
///
/// # Examples
/// inputs_json:
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
/// # Returns
/// * `req_with_fees_json` - modified Indy request with added fees info
/// * `payment_method`
pub fn add_request_fees(wallet_handle: IndyHandle,
                        submitter_did: Option<&str>,
                        req_json: &str,
                        inputs_json: &str,
                        outputs_json: &str,
                        extra: Option<&str>) -> Box<Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _add_request_fees(command_handle, wallet_handle, submitter_did, req_json, inputs_json, outputs_json, extra, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _add_request_fees(command_handle: IndyHandle,
                     wallet_handle: IndyHandle,
                     submitter_did: Option<&str>,
                     req_json: &str,
                     inputs_json: &str,
                     outputs_json: &str,
                     extra: Option<&str>,
                     cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let req_json = c_str!(req_json);
    let inputs_json = c_str!(inputs_json);
    let outputs_json = c_str!(outputs_json);
    let extra_str = opt_c_str!(extra);

    ErrorCode::from(unsafe {
        payments::indy_add_request_fees(command_handle,
                                        wallet_handle,
                                        opt_c_ptr!(submitter_did, submitter_did_str),
                                        req_json.as_ptr(),
                                        inputs_json.as_ptr(),
                                        outputs_json.as_ptr(),
                                        opt_c_ptr!(extra, extra_str),
                                        cb)
    })
}

/// Parses response for Indy request with fees.
///
/// # Arguments
/// * `payment_method`
/// * `resp_json`: response for Indy request with fees
///   Note: this param will be used to determine payment_method
///
/// # Returns
/// * `utxo_json` - parsed (payment method and node version agnostic) utxo info as json
///
/// # Example
/// utxo_json
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_response_with_fees(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_response_with_fees(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_response_with_fees(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })
}

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// # Arguments
/// * `wallet_handle` - wallet handle
/// * `submitter_did` - DID of request sender
/// * `payment_address` -: target payment address
///
/// # Returns
/// * `get_utxo_txn_json` - Indy request for getting UTXO list for payment address
/// * `payment_method`
pub fn build_get_payment_sources_request(wallet_handle: IndyHandle, submitter_did: Option<&str>, payment_address: &str) -> Box<Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) =
        ClosureHandler::cb_ec_string_string();

    let err = _build_get_payment_sources_request(command_handle, wallet_handle, submitter_did, payment_address, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_get_payment_sources_request(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: Option<&str>, payment_address: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let payment_address = c_str!(payment_address);

    ErrorCode::from(unsafe { payments::indy_build_get_payment_sources_request(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), payment_address.as_ptr(), cb) })
}

/// Parses response for Indy request for getting UTXO list.
///
/// # Arguments
/// * `payment_method`
/// * `resp_json` - response for Indy request for getting UTXO list
///   Note: this param will be used to determine payment_method
///
/// # Returns
/// * `utxo_json` - parsed (payment method and node version agnostic) utxo info as json:
/// # Examples:
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub fn parse_get_payment_sources_response(payment_method: &str, resp_json: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_get_payment_sources_response(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_get_payment_sources_response(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_get_payment_sources_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })
}

/// Builds Indy request for doing tokens payment
/// according to this payment method.
///
/// This method consumes set of UTXO inputs and outputs.
///
/// Format of inputs is specific for payment method. Usually it should reference payment transaction
/// with at least one output that corresponds to payment address that user owns.
///
/// # Arguments
/// * `wallet_handle` - wallet handle
/// * `submitter_did` - DID of request sender
/// * `inputs_json` - The list of UTXO inputs as json array:
///   ["input1", ...]
///   Note that each input should reference paymentAddress
/// * `outputs_json` - The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// # Returns
/// * `payment_req_json` - Indy request for doing tokens payment
/// * `payment_method`
pub fn build_payment_req(wallet_handle: IndyHandle, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>) -> Box<Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _build_payment_req(command_handle, wallet_handle, submitter_did, inputs, outputs, extra, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_payment_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let inputs = c_str!(inputs);
    let outputs = c_str!(outputs);
    let extra_str = opt_c_str!(extra);

    ErrorCode::from(unsafe {
        payments::indy_build_payment_req(command_handle,
                                         wallet_handle,
                                         opt_c_ptr!(submitter_did, submitter_did_str),
                                         inputs.as_ptr(),
                                         outputs.as_ptr(),
                                         opt_c_ptr!(extra, extra_str),
                                         cb)
    })
}

/// Parses response for Indy request for payment txn.
///
/// # Arguments
/// * `command_handle`
/// * `payment_method`
/// * `resp_json` - response for Indy request for payment txn
///   Note: this param will be used to determine payment_method
///
/// # Returns
/// * `utxo_json`  - parsed (payment method and node version agnostic) utxo info as jso-n
///   [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_payment_response(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_payment_response(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_payment_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })

}

/// Builds Indy request for doing tokens minting
/// according to this payment method.
///
/// # Arguments
/// * `wallet_handle` - wallet handle
/// * `submitter_did` - DID of request sender
/// * `outputs_json` - The list of UTXO outputs as json array:
///   [{
///     paymentAddress: <str>, // payment address used as output
///     amount: <int>, // amount of tokens to transfer to this payment address
///     extra: <str>, // optional data
///   }]
///
/// # Returns
/// * `mint_req_json`  - Indy request for doing tokens minting
/// * `payment_method`
pub fn build_mint_req(wallet_handle: IndyHandle, submitter_did: Option<&str>, outputs_json: &str, extra: Option<&str>) -> Box<Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _build_mint_req(command_handle, wallet_handle, submitter_did, outputs_json, extra, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_mint_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: Option<&str>, outputs_json: &str, extra: Option<&str>, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let outputs_json = c_str!(outputs_json);
    let extra_str = opt_c_str!(extra);

    ErrorCode::from(unsafe { payments::indy_build_mint_req(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), outputs_json.as_ptr(), opt_c_ptr!(extra, extra_str), cb) })
}

/// Builds Indy request for setting fees for transactions in the ledger
///
/// # Arguments
/// * `wallet_handle` - wallet handle
/// * `submitter_did` - DID of request sender
/// * `payment_method`
/// * `fees_json` - {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
///
/// # Returns
/// * `set_txn_fees_json`  - Indy request for setting fees for transactions in the ledger
pub fn build_set_txn_fees_req(wallet_handle: IndyHandle, submitter_did: Option<&str>, payment_method: &str, fees_json: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_set_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, fees_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_set_txn_fees_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: Option<&str>, payment_method: &str, fees_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let payment_method = c_str!(payment_method);
    let fees_json = c_str!(fees_json);

    ErrorCode::from(unsafe { payments::indy_build_set_txn_fees_req(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), payment_method.as_ptr(), fees_json.as_ptr(), cb) })
}

/// Builds Indy get request for getting fees for transactions in the ledger
///
/// # Arguments
/// * `command_handle`
/// * `wallet_handle` - wallet handle
/// * `submitter_did`  - DID of request sender
/// * `payment_method`
///
/// # Returns
/// * `get_txn_fees_json` - Indy request for getting fees for transactions in the ledger
pub fn build_get_txn_fees_req(wallet_handle: IndyHandle, submitter_did: Option<&str>, payment_method: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_txn_fees_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: Option<&str>, payment_method: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let payment_method = c_str!(payment_method);

    ErrorCode::from(unsafe { payments::indy_build_get_txn_fees_req(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), payment_method.as_ptr(), cb) })
}

/// Parses response for Indy request for getting fees
///
/// # Arguments
/// * `command_handle`
/// * `payment_method`
/// * `resp_json` - response for Indy request for getting fees
///
/// # Returns
/// * `fees_json`  {
///   txnType1: amount1,
///   txnType2: amount2,
///   .................
///   txnTypeN: amountN,
/// }
pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_get_txn_fees_response(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_get_txn_fees_response(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_get_txn_fees_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })
}

pub fn build_verify_payment_req(wallet_handle: IndyHandle, submitter_did: Option<&str>, receipt: &str) -> Box<Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _build_verify_req(command_handle, wallet_handle, submitter_did, receipt, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_verify_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: Option<&str>, receipt: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let receipt = c_str!(receipt);

    ErrorCode::from(unsafe {
      payments::indy_build_verify_payment_req(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), receipt.as_ptr(), cb)
    })
}

pub fn parse_verify_payment_response(payment_method: &str, resp_json: &str) -> Box<Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_verify_response(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_verify_response(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe {
      payments::indy_parse_verify_payment_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb)
    })
}
