use {ErrorCode, IndyError};

use std::ffi::CString;
use std::ptr::null;

use futures::Future;

use ffi::payments;
use ffi::{ResponseStringCB,
          ResponseStringStringCB,
          ResponseStringI64CB,
          ResponseSliceCB,
          ResponseBoolCB,
          WalletHandle,
          CommandHandle
};

use utils::callbacks::{ClosureHandler, ResultHandler};
use futures::IntoFuture;

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
pub fn create_payment_address(wallet_handle: WalletHandle, payment_method: &str, config: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _create_payment_address(command_handle, wallet_handle, payment_method, config, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _create_payment_address(command_handle: CommandHandle, wallet_handle: WalletHandle, payment_method: &str, config: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
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
pub fn list_payment_addresses(wallet_handle: WalletHandle) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _list_payment_addresses(command_handle, wallet_handle, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _list_payment_addresses(command_handle: CommandHandle, wallet_handle: WalletHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
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
pub fn add_request_fees(wallet_handle: WalletHandle,
                        submitter_did: Option<&str>,
                        req_json: &str,
                        inputs_json: &str,
                        outputs_json: &str,
                        extra: Option<&str>) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _add_request_fees(command_handle, wallet_handle, submitter_did, req_json, inputs_json, outputs_json, extra, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _add_request_fees(command_handle: CommandHandle,
                     wallet_handle: WalletHandle,
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
pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_response_with_fees(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_response_with_fees(command_handle: CommandHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_response_with_fees(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })
}

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
/// Deprecated. This function will be most likely be removed with Indy SDK 2.0 version
///
/// # Arguments
/// * `wallet_handle` - wallet handle
/// * `submitter_did` - DID of request sender
/// * `payment_address` -: target payment address
///
/// # Returns
/// * `get_utxo_txn_json` - Indy request for getting UTXO list for payment address
/// * `payment_method`
#[deprecated(since="2.0.0", note="please use `parse_get_payment_sources_with_from_response` instead")]
pub fn build_get_payment_sources_request(wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_address: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) =
        ClosureHandler::cb_ec_string_string();

    let err = _build_get_payment_sources_request(command_handle, wallet_handle, submitter_did, payment_address, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_get_payment_sources_request(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_address: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let payment_address = c_str!(payment_address);

    ErrorCode::from(unsafe { payments::indy_build_get_payment_sources_request(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), payment_address.as_ptr(), cb) })
}

/// Builds Indy request for getting UTXO list for payment address
/// according to this payment method.
///
/// # Arguments
/// * `wallet_handle` - wallet handle
/// * `submitter_did` - DID of request sender
/// * `payment_address` - target payment address
/// * `from` - shift to the next slice of payment sources
///
/// # Returns
/// * `get_utxo_txn_json` - Indy request for getting UTXO list for payment address
/// * `payment_method`
pub fn build_get_payment_sources_with_from_request(wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_address: &str, from: Option<i64>) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) =
        ClosureHandler::cb_ec_string_string();

    let err = _build_get_payment_sources_with_from_request(command_handle, wallet_handle, submitter_did, payment_address, from, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_get_payment_sources_with_from_request(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_address: &str, from: Option<i64>, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let payment_address = c_str!(payment_address);

    ErrorCode::from(unsafe { payments::indy_build_get_payment_sources_with_from_request(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), payment_address.as_ptr(), from.unwrap_or(-1), cb) })
}

/// Parses response for Indy request for getting UTXO list.
/// Deprecated. This function will be most likely be removed with Indy SDK 2.0 version
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
#[deprecated(since="2.0.0", note="please use `parse_get_payment_sources_with_from_response` instead")]
pub fn parse_get_payment_sources_response(payment_method: &str, resp_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_get_payment_sources_response(command_handle, payment_method, resp_json, cb);

    Box::new(ResultHandler::str(command_handle, err, receiver))
}

fn _parse_get_payment_sources_response(command_handle: CommandHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_get_payment_sources_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })
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
///   sources -- [{
///      input: <str>, // UTXO input
///      amount: <int>, // amount of tokens in this input
///      extra: <str>, // optional data from payment transaction
///   }]
///   next -- pointer to the next slice of payment sources
pub fn parse_get_payment_sources_with_from_response(payment_method: &str, resp_json: &str) -> Box<dyn Future<Item=(String, Option<i64>), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_i64();

    let err = _parse_get_payment_sources_with_from_response(command_handle, payment_method, resp_json, cb);

    Box::new(ResultHandler::str_i64(command_handle, err, receiver).map(|(s, i)| (s, if i >= 0 {Some(i)} else {None})).into_future())
}

fn _parse_get_payment_sources_with_from_response(command_handle: CommandHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringI64CB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_get_payment_sources_with_from_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })
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
pub fn build_payment_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _build_payment_req(command_handle, wallet_handle, submitter_did, inputs, outputs, extra, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_payment_req(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>, cb: Option<ResponseStringStringCB>) -> ErrorCode {
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
pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_payment_response(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_payment_response(command_handle: CommandHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_payment_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })

}

/// Append payment extra JSON with TAA acceptance data
///
/// This function may calculate digest by itself or consume it as a parameter.
/// If all text, version and taa_digest parameters are specified, a check integrity of them will be done.
///
/// # Arguments
/// * `extra_json`: original extra json.
/// * `text` and `version`: (optional) raw data about TAA from ledger.
///     These parameters should be passed together.
///     These parameters are required if taa_digest parameter is omitted.
/// * `taa_digest`: (optional) digest on text and version.
///     Digest is sha256 hash calculated on concatenated strings: version || text.
///     This parameter is required if text and version parameters are omitted.
/// * `mechanism`: mechanism how user has accepted the TAA
/// * `time`: UTC timestamp when user has accepted the TAA
///
/// # Returns
/// Updated extra result as json.
pub fn prepare_extra_with_acceptance_data(extra_json: Option<&str>,
                                          text: Option<&str>,
                                          version: Option<&str>,
                                          taa_digest: Option<&str>,
                                          mechanism: &str,
                                          time: u64) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _prepare_extra_with_acceptance_data(command_handle, extra_json, text, version, taa_digest, mechanism, time, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _prepare_extra_with_acceptance_data(command_handle: CommandHandle,
                                       extra_json: Option<&str>,
                                       text: Option<&str>,
                                       version: Option<&str>,
                                       taa_digest: Option<&str>,
                                       mechanism: &str,
                                       time: u64,
                                       cb: Option<ResponseStringCB>) -> ErrorCode {
    let extra_str = opt_c_str!(extra_json);
    let text_str = opt_c_str!(text);
    let version_str = opt_c_str!(version);
    let taa_digest_str = opt_c_str!(taa_digest);
    let mechanism = c_str!(mechanism);

    ErrorCode::from(unsafe {
        payments::indy_prepare_payment_extra_with_acceptance_data(command_handle,
                                                                  opt_c_ptr!(extra_json, extra_str),
                                                                  opt_c_ptr!(text, text_str),
                                                                  opt_c_ptr!(version, version_str),
                                                                  opt_c_ptr!(taa_digest, taa_digest_str),
                                                                  mechanism.as_ptr(),
                                                                  time,
                                                                  cb)
    })
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
pub fn build_mint_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, outputs_json: &str, extra: Option<&str>) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _build_mint_req(command_handle, wallet_handle, submitter_did, outputs_json, extra, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_mint_req(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: Option<&str>, outputs_json: &str, extra: Option<&str>, cb: Option<ResponseStringStringCB>) -> ErrorCode {
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
pub fn build_set_txn_fees_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_method: &str, fees_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_set_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, fees_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_set_txn_fees_req(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_method: &str, fees_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
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
pub fn build_get_txn_fees_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_method: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _build_get_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _build_get_txn_fees_req(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_method: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
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
pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_get_txn_fees_response(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_get_txn_fees_response(command_handle: CommandHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe { payments::indy_parse_get_txn_fees_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) })
}

pub fn build_verify_payment_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, receipt: &str) -> Box<dyn Future<Item=(String, String), Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

    let err = _build_verify_req(command_handle, wallet_handle, submitter_did, receipt, cb);

    ResultHandler::str_str(command_handle, err, receiver)
}

fn _build_verify_req(command_handle: CommandHandle, wallet_handle: WalletHandle, submitter_did: Option<&str>, receipt: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
    let submitter_did_str = opt_c_str!(submitter_did);
    let receipt = c_str!(receipt);

    ErrorCode::from(unsafe {
        payments::indy_build_verify_payment_req(command_handle, wallet_handle, opt_c_ptr!(submitter_did, submitter_did_str), receipt.as_ptr(), cb)
    })
}

pub fn parse_verify_payment_response(payment_method: &str, resp_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _parse_verify_response(command_handle, payment_method, resp_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _parse_verify_response(command_handle: CommandHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let payment_method = c_str!(payment_method);
    let resp_json = c_str!(resp_json);

    ErrorCode::from(unsafe {
        payments::indy_parse_verify_payment_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb)
    })
}

/// Gets request requirements (with minimal price) correspondent to specific auth rule
/// in case the requester can perform this action.
///
/// EXPERIMENTAL
///
/// If the requester does not match to the request constraints `TransactionNotAllowed` error will be thrown.
///
/// # Arguments
/// * `get_auth_rule_response_json`: response on GET_AUTH_RULE request returning action constraints set on the ledger.
/// * `requester_info_json`: {
///     "role": string (optional) - role of a user which can sign a transaction.
///     "sig_count": u64 - number of signers.
///     "is_owner": bool (optional) - if user is an owner of transaction (false by default).
///     "is_off_ledger_signature": bool (optional) - if user did is unknow for ledger (false by default).
/// }
/// * `fees_json`: fees set on the ledger (result of `parse_get_txn_fees_response`).
///
/// # Returns
/// * `request_info_json`: request info if a requester match to the action auth rule.
/// {
///     "price": u64 - tokens amount required for action performing,
///     "requirements": [{
///         "role": string (optional) - role of users who should sign,
///         "sig_count": string - count of signers,
///         "need_to_be_owner": bool - if requester need to be owner,
///         "off_ledger_signature": bool - allow signature of unknow for ledger did (false by default).
///     }]
/// }
///
pub fn get_request_info(get_auth_rule_resp_json: &str, requester_info_json: &str, fees_json: &str) -> Box<dyn Future<Item=String, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

    let err = _get_request_info(command_handle, get_auth_rule_resp_json, requester_info_json, fees_json, cb);

    ResultHandler::str(command_handle, err, receiver)
}

fn _get_request_info(command_handle: CommandHandle, get_auth_rule_resp_json: &str, requester_info_json: &str, fees_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
    let get_auth_rule_resp_json = c_str!(get_auth_rule_resp_json);
    let requester_info_json = c_str!(requester_info_json);
    let fees_json = c_str!(fees_json);

    ErrorCode::from(unsafe {
        payments::indy_get_request_info(command_handle, get_auth_rule_resp_json.as_ptr(), requester_info_json.as_ptr(), fees_json.as_ptr(), cb)
    })
}

pub fn sign_with_address(wallet_handle: WalletHandle, address: &str, message: &[u8]) -> Box<dyn Future<Item=Vec<u8>, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_slice();

    let err = _sign_with_address(command_handle, wallet_handle, address, message, cb);

    ResultHandler::slice(command_handle, err, receiver)
}

fn _sign_with_address(command_handle: CommandHandle, wallet_handle: WalletHandle, address: &str, message: &[u8], cb: Option<ResponseSliceCB>) -> ErrorCode {
    let address = c_str!(address);
    ErrorCode::from(unsafe {
        payments::indy_sign_with_address(command_handle, wallet_handle, address.as_ptr(),
                         message.as_ptr() as *const u8,
                         message.len() as u32,
                         cb)
    })
}

pub fn verify_with_address(address: &str, message: &[u8], signature: &[u8]) -> Box<dyn Future<Item=bool, Error=IndyError>> {
    let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

    let err = _verify_with_address(command_handle, address, message, signature, cb);

    ResultHandler::bool(command_handle, err, receiver)
}

fn _verify_with_address(command_handle: CommandHandle, address: &str, message: &[u8], signature: &[u8], cb: Option<ResponseBoolCB>) -> ErrorCode {
    let address = c_str!(address);

    ErrorCode::from(unsafe {
        payments::indy_verify_with_address(command_handle, address.as_ptr(),
                                           message.as_ptr() as *const u8, message.len() as u32,
                                           signature.as_ptr() as *const u8, signature.len() as u32,
                                           cb)
    })
}
