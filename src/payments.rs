use super::{ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;

use ffi::payments;
use ffi::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseStringStringCB};

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

pub struct Payment {}

impl Payment {
    /// Register custom payment implementation.
    ///
    /// It allows library user to provide custom payment method implementation as set of handlers.
    ///
    /// # Arguments
    /// * `command_handle` - Command handle to map callback to caller context.
    /// * `payment_method` - The type of payment method also used as sub-prefix for fully resolvable payment address format ("sov" - for example)
    /// * `create_payment_address` - "create_payment_address" operation handler
    /// * `add_request_fees` - "add_request_fees" operation handler
    /// * `build_get_utxo_request` - "build_get_utxo_request" operation handler
    /// * `parse_get_utxo_response` - "parse_get_utxo_response" operation handler
    /// * `build_payment_req` - "build_payment_req" operation handler
    /// * `build_mint_req` - "build_mint_req" operation handler
    /// * `build_set_txn_fees_req` - "build_set_txn_fees_req" operation handler
    /// * `build_get_txn_fees_req` - "build_get_txn_fees_req" operation handler
    /// * `parse_get_txn_fees_response` - "parse_get_txn_fees_response" operation handler
    pub fn register(payment_method: &str,
                    create_payment_address: payments::CreatePaymentAddressCB,
                    add_request_fees: payments::AddRequestFeesCB,
                    parse_response_with_fees: payments::ParseResponseWithFeesCB,
                    build_get_utxo_request: payments::BuildGetUTXORequestCB,
                    parse_get_utxo_response: payments::ParseGetUTXOResponseCB,
                    build_payment_req: payments::BuildPaymentRequestCB,
                    parse_payment_response: payments::ParsePaymentResponseCB,
                    build_mint_req: payments::BuildMintRequestCB,
                    build_set_txn_fees_req: payments::BuildSetTxnFeesRequestCB,
                    build_get_txn_fees_req: payments::BuildGetTxnFeesRequestCB,
                    parse_get_txn_fees_response: payments::ParseGetTxnFeesResponseCB) -> Result<(), ErrorCode> {

        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Payment::_register(command_handle,
                                     payment_method,
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
                                     parse_get_txn_fees_response,
                                     cb);

        ResultHandler::empty(err, receiver)
    }

    /// Register custom payment implementation.
    ///
    /// It allows library user to provide custom payment method implementation as set of handlers.
    ///
    /// # Arguments
    /// * `command_handle` - Command handle to map callback to caller context.
    /// * `payment_method` - The type of payment method also used as sub-prefix for fully resolvable payment address format ("sov" - for example)
    /// * `create_payment_address` - "create_payment_address" operation handler
    /// * `add_request_fees` - "add_request_fees" operation handler
    /// * `build_get_utxo_request` - "build_get_utxo_request" operation handler
    /// * `parse_get_utxo_response` - "parse_get_utxo_response" operation handler
    /// * `build_payment_req` - "build_payment_req" operation handler
    /// * `build_mint_req` - "build_mint_req" operation handler
    /// * `build_set_txn_fees_req` - "build_set_txn_fees_req" operation handler
    /// * `build_get_txn_fees_req` - "build_get_txn_fees_req" operation handler
    /// * `parse_get_txn_fees_response` - "parse_get_txn_fees_response" operation handler
    /// * `timeout` - the maximum time this function waits for a response
    pub fn register_timeout(payment_method: &str,
                            create_payment_address: payments::CreatePaymentAddressCB,
                            add_request_fees: payments::AddRequestFeesCB,
                            parse_response_with_fees: payments::ParseResponseWithFeesCB,
                            build_get_utxo_request: payments::BuildGetUTXORequestCB,
                            parse_get_utxo_response: payments::ParseGetUTXOResponseCB,
                            build_payment_req: payments::BuildPaymentRequestCB,
                            parse_payment_response: payments::ParsePaymentResponseCB,
                            build_mint_req: payments::BuildMintRequestCB,
                            build_set_txn_fees_req: payments::BuildSetTxnFeesRequestCB,
                            build_get_txn_fees_req: payments::BuildGetTxnFeesRequestCB,
                            parse_get_txn_fees_response: payments::ParseGetTxnFeesResponseCB,
                            timeout: Duration) -> Result<(), ErrorCode> {

        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Payment::_register(command_handle,
                                     payment_method,
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
                                     parse_get_txn_fees_response,
                                     cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Register custom payment implementation.
    ///
    /// It allows library user to provide custom payment method implementation as set of handlers.
    ///
    /// # Arguments
    /// * `command_handle` - Command handle to map callback to caller context.
    /// * `payment_method` - The type of payment method also used as sub-prefix for fully resolvable payment address format ("sov" - for example)
    /// * `create_payment_address` - "create_payment_address" operation handler
    /// * `add_request_fees` - "add_request_fees" operation handler
    /// * `build_get_utxo_request` - "build_get_utxo_request" operation handler
    /// * `parse_get_utxo_response` - "parse_get_utxo_response" operation handler
    /// * `build_payment_req` - "build_payment_req" operation handler
    /// * `build_mint_req` - "build_mint_req" operation handler
    /// * `build_set_txn_fees_req` - "build_set_txn_fees_req" operation handler
    /// * `build_get_txn_fees_req` - "build_get_txn_fees_req" operation handler
    /// * `parse_get_txn_fees_response` - "parse_get_txn_fees_response" operation handler
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn register_async<F: 'static>(payment_method: &str,
                                      create_payment_address: payments::CreatePaymentAddressCB,
                                      add_request_fees: payments::AddRequestFeesCB,
                                      parse_response_with_fees: payments::ParseResponseWithFeesCB,
                                      build_get_utxo_request: payments::BuildGetUTXORequestCB,
                                      parse_get_utxo_response: payments::ParseGetUTXOResponseCB,
                                      build_payment_req: payments::BuildPaymentRequestCB,
                                      parse_payment_response: payments::ParsePaymentResponseCB,
                                      build_mint_req: payments::BuildMintRequestCB,
                                      build_set_txn_fees_req: payments::BuildSetTxnFeesRequestCB,
                                      build_get_txn_fees_req: payments::BuildGetTxnFeesRequestCB,
                                      parse_get_txn_fees_response: payments::ParseGetTxnFeesResponseCB,
                                      closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {

        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Payment::_register(command_handle,
                           payment_method,
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
                           parse_get_txn_fees_response,
                           cb)
    }

    
    fn _register(command_handle: IndyHandle,
                 payment_method: &str,
                 create_payment_address: payments::CreatePaymentAddressCB,
                 add_request_fees: payments::AddRequestFeesCB,
                 parse_response_with_fees: payments::ParseResponseWithFeesCB,
                 build_get_utxo_request: payments::BuildGetUTXORequestCB,
                 parse_get_utxo_response: payments::ParseGetUTXOResponseCB,
                 build_payment_req: payments::BuildPaymentRequestCB,
                 parse_payment_response: payments::ParsePaymentResponseCB,
                 build_mint_req: payments::BuildMintRequestCB,
                 build_set_txn_fees_req: payments::BuildSetTxnFeesRequestCB,
                 build_get_txn_fees_req: payments::BuildGetTxnFeesRequestCB,
                 parse_get_txn_fees_response: payments::ParseGetTxnFeesResponseCB,
                 cb: Option<ResponseEmptyCB>) -> ErrorCode {

        let payment_method = c_str!(payment_method);
        unsafe {
            payments::indy_register_payment_method(command_handle,
                                                   payment_method.as_ptr(),
                                                   Some(create_payment_address),
                                                   Some(add_request_fees),
                                                   Some(parse_response_with_fees),
                                                   Some(build_get_utxo_request),
                                                   Some(parse_get_utxo_response),
                                                   Some(build_payment_req),
                                                   Some(parse_payment_response),
                                                   Some(build_mint_req),
                                                   Some(build_set_txn_fees_req),
                                                   Some(build_get_txn_fees_req),
                                                   Some(parse_get_txn_fees_response),
                                                   cb)
        }
    }

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
    pub fn create_payment_address(wallet_handle: IndyHandle, payment_method: &str, config: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_create_payment_address(command_handle, wallet_handle, payment_method, config, cb);

        ResultHandler::one(err, receiver)
    }

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
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Example
    /// config
    /// {
    ///   seed: <str>, // allows deterministic creation of payment address
    /// }
    ///
    /// # Returns
    /// * `payment_address` - public identifier of payment address in fully resolvable payment address format
    pub fn create_payment_address_timeout(wallet_handle: IndyHandle, payment_method: &str, config: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_create_payment_address(command_handle, wallet_handle, payment_method, config, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

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
    /// * `closure` - the closure that is called when finished
    ///
    /// # Example
    /// config
    /// {
    ///   seed: <str>, // allows deterministic creation of payment address
    /// }
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_payment_address_async<F: 'static>(wallet_handle: IndyHandle, payment_method: &str, config: &str, closure: F) -> ErrorCode where F:FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Payment::_create_payment_address(command_handle, wallet_handle, payment_method, config, cb)
    }

    fn _create_payment_address(command_handle: IndyHandle, wallet_handle: IndyHandle, payment_method: &str, config: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let payment_method = c_str!(payment_method);
        let config = c_str!(config);

        unsafe { payments::indy_create_payment_address(command_handle, wallet_handle, payment_method.as_ptr(), config.as_ptr(), cb) }
    }

    /// Lists all payment addresses that are stored in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet to search for payment_addresses
    ///
    /// # Returns
    /// * `payment_addresses_json` - json array of string with json addresses
    pub fn list_payment_addresses(wallet_handle: IndyHandle) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_list_payment_addresses(command_handle, wallet_handle, cb);

        ResultHandler::one(err, receiver)
    }

    /// Lists all payment addresses that are stored in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet to search for payment_addresses
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `payment_addresses_json` - json array of string with json addresses
    pub fn list_payment_addresses_timeout(wallet_handle: IndyHandle, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_list_payment_addresses(command_handle, wallet_handle, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Lists all payment addresses that are stored in the wallet
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet to search for payment_addresses
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn list_payment_addresses_async<F: 'static>(wallet_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Payment::_list_payment_addresses(command_handle, wallet_handle, cb)
    }

    fn _list_payment_addresses(command_handle: IndyHandle, wallet_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
        unsafe {
            payments::indy_list_payment_addresses(command_handle, wallet_handle, cb)
        }
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
    pub fn add_request_fees(wallet_handle: IndyHandle, submitter_did: &str, req_json: &str, inputs_json: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Payment::_add_request_fees(command_handle, wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb);

        ResultHandler::two(err, receiver)
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
    /// * `timeout` - the maximum time this function waits for a response
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
    pub fn add_request_fees_timeout(wallet_handle: IndyHandle, submitter_did: &str, req_json: &str, inputs_json: &str, outputs_json: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Payment::_add_request_fees(command_handle, wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
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
    /// * `closure` - the closure that is called when finished
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
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn add_request_fees_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, req_json: &str, inputs_json: &str, outputs_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Payment::_add_request_fees(command_handle, wallet_handle, submitter_did, req_json, inputs_json, outputs_json, cb)
    }

    fn _add_request_fees(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, req_json: &str, inputs_json: &str, outputs_json: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let req_json = c_str!(req_json);
        let inputs_json = c_str!(inputs_json);
        let outputs_json = c_str!(outputs_json);

        unsafe {
            payments::indy_add_request_fees(command_handle,
                                            wallet_handle,
                                            submitter_did.as_ptr(),
                                            req_json.as_ptr(),
                                            inputs_json.as_ptr(),
                                            outputs_json.as_ptr(),
                                            cb)
        }
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
    pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_response_with_fees(command_handle, payment_method, resp_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Parses response for Indy request with fees.
    ///
    /// # Arguments
    /// * `payment_method`
    /// * `resp_json`: response for Indy request with fees
    /// * `timeout` - the maximum time this function waits for a response
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
    pub fn parse_response_with_fees_timeout(payment_method: &str, resp_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_response_with_fees(command_handle, payment_method, resp_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Parses response for Indy request with fees.
    ///
    /// # Arguments
    /// * `payment_method`
    /// * `resp_json`: response for Indy request with fees
    /// * `closure` - the closure that is called when finished
    ///   Note: this param will be used to determine payment_method
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_response_with_fees_async<F: 'static>(payment_method: &str, resp_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));
        Payment::_parse_response_with_fees(command_handle, payment_method, resp_json, cb)
    }

    fn _parse_response_with_fees(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        unsafe { payments::indy_parse_response_with_fees(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) }
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
    pub fn build_get_utxo_request(wallet_handle: IndyHandle, submitter_did: &str, payment_address: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string_string();

        let err = Payment::_build_get_utxo_request(command_handle, wallet_handle, submitter_did, payment_address, cb);

        ResultHandler::two(err, receiver)
    }

    /// Builds Indy request for getting UTXO list for payment address
    /// according to this payment method.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle
    /// * `submitter_did` - DID of request sender
    /// * `payment_address` -: target payment address
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `get_utxo_txn_json` - Indy request for getting UTXO list for payment address
    /// * `payment_method`
    pub fn build_get_utxo_request_timeout(wallet_handle: IndyHandle, submitter_did: &str, payment_address: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string_string();

        let err = Payment::_build_get_utxo_request(command_handle, wallet_handle, submitter_did, payment_address, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// Builds Indy request for getting UTXO list for payment address
    /// according to this payment method.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle
    /// * `submitter_did` - DID of request sender
    /// * `payment_address` -: target payment address
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_utxo_request_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, payment_address: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));
        Payment::_build_get_utxo_request(command_handle, wallet_handle, submitter_did, payment_address, cb)
    }

    fn _build_get_utxo_request(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, payment_address: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let payment_address = c_str!(payment_address);

        unsafe { payments::indy_build_get_utxo_request(command_handle, wallet_handle, submitter_did.as_ptr(), payment_address.as_ptr(), cb) }
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
    pub fn parse_get_utxo_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_get_utxo_response(command_handle, payment_method, resp_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Parses response for Indy request for getting UTXO list.
    ///
    /// # Arguments
    /// * `payment_method`
    /// * `resp_json` - response for Indy request for getting UTXO list
    /// * `timeout` - the maximum time this function waits for a response
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
    pub fn parse_get_utxo_response_timeout(payment_method: &str, resp_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_get_utxo_response(command_handle, payment_method, resp_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Parses response for Indy request for getting UTXO list.
    ///
    /// # Arguments
    /// * `payment_method`
    /// * `resp_json` - response for Indy request for getting UTXO list
    /// * `closure` - the closure that is called when finished
    ///   Note: this param will be used to determine payment_method
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_get_utxo_response_async<F: 'static>(payment_method: &str, resp_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send{
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Payment::_parse_get_utxo_response(command_handle, payment_method, resp_json, cb)
    }

    fn _parse_get_utxo_response(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        unsafe { payments::indy_parse_get_utxo_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) }
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
    pub fn build_payment_req(wallet_handle: IndyHandle, submitter_did: &str, inputs: &str, outputs: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();
        
        let err = Payment::_build_payment_req(command_handle, wallet_handle, submitter_did, inputs, outputs, cb);

        ResultHandler::two(err, receiver)
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
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `payment_req_json` - Indy request for doing tokens payment
    /// * `payment_method` 
    pub fn build_payment_req_timeout(wallet_handle: IndyHandle, submitter_did: &str, inputs: &str, outputs: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();
        
        let err = Payment::_build_payment_req(command_handle, wallet_handle, submitter_did, inputs, outputs, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
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
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_payment_req_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, inputs: &str, outputs: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));
        
        Payment::_build_payment_req(command_handle, wallet_handle, submitter_did, inputs, outputs, cb)
    }

    fn _build_payment_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, inputs: &str, outputs: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let inputs = c_str!(inputs);
        let outputs = c_str!(outputs);

        unsafe {
            payments::indy_build_payment_req(command_handle,
                                             wallet_handle,
                                             submitter_did.as_ptr(),
                                             inputs.as_ptr(),
                                             outputs.as_ptr(),
                                             cb)
        }
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
    pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_payment_response(command_handle, payment_method, resp_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Parses response for Indy request for payment txn.
    ///
    /// # Arguments
    /// * `command_handle` 
    /// * `payment_method` 
    /// * `resp_json` - response for Indy request for payment txn
    /// * `timeout` - the maximum time this function waits for a response
    ///   Note: this param will be used to determine payment_method
    ///
    /// # Returns
    /// * `utxo_json`  - parsed (payment method and node version agnostic) utxo info as jso-n
    ///   [{
    ///      input: <str>, // UTXO input
    ///      amount: <int>, // amount of tokens in this input
    ///      extra: <str>, // optional data from payment transaction
    ///   }]
    pub fn parse_payment_response_timeout(payment_method: &str, resp_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_payment_response(command_handle, payment_method, resp_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Parses response for Indy request for payment txn.
    ///
    /// # Arguments
    /// * `command_handle` 
    /// * `payment_method` 
    /// * `resp_json` - response for Indy request for payment txn
    /// * `closure` - the closure that is called when finished
    ///   Note: this param will be used to determine payment_method
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_payment_response_async<F: 'static>(payment_method: &str, resp_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Payment::_parse_payment_response(command_handle, payment_method, resp_json, cb)
    }

    fn _parse_payment_response(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        unsafe { payments::indy_parse_payment_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) }

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
    pub fn build_mint_req(wallet_handle: IndyHandle, submitter_did: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Payment::_build_mint_req(command_handle, wallet_handle, submitter_did, outputs_json, cb);

        ResultHandler::two(err, receiver)
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
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `mint_req_json`  - Indy request for doing tokens minting
    /// * `payment_method` 
    pub fn build_mint_req_timeout(wallet_handle: IndyHandle, submitter_did: &str, outputs_json: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Payment::_build_mint_req(command_handle, wallet_handle, submitter_did, outputs_json, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
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
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_mint_req_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, outputs_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Payment::_build_mint_req(command_handle, wallet_handle, submitter_did, outputs_json, cb)
    }

    fn _build_mint_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, outputs_json: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let outputs_json = c_str!(outputs_json);

        unsafe { payments::indy_build_mint_req(command_handle, wallet_handle, submitter_did.as_ptr(), outputs_json.as_ptr(), cb) }
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
    pub fn build_set_txn_fees_req(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, fees_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_build_set_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, fees_json, cb);

        ResultHandler::one(err, receiver)
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
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `set_txn_fees_json`  - Indy request for setting fees for transactions in the ledger
    pub fn build_set_txn_fees_req_timeout(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, fees_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_build_set_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, fees_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
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
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_set_txn_fees_req_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, fees_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Payment::_build_set_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, fees_json, cb)
    }

    fn _build_set_txn_fees_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, fees_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let payment_method = c_str!(payment_method);
        let fees_json = c_str!(fees_json);

        unsafe { payments::indy_build_set_txn_fees_req(command_handle, wallet_handle, submitter_did.as_ptr(), payment_method.as_ptr(), fees_json.as_ptr(), cb) }
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
    pub fn build_get_txn_fees_req(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_build_get_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, cb); 

        ResultHandler::one(err, receiver)
    }

    /// Builds Indy get request for getting fees for transactions in the ledger
    ///
    /// # Arguments
    /// * `command_handle` 
    /// * `wallet_handle` - wallet handle
    /// * `submitter_did`  - DID of request sender
    /// * `payment_method` 
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `get_txn_fees_json` - Indy request for getting fees for transactions in the ledger
    pub fn build_get_txn_fees_req_timeout(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_build_get_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, cb); 

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds Indy get request for getting fees for transactions in the ledger
    ///
    /// # Arguments
    /// * `command_handle` 
    /// * `wallet_handle` - wallet handle
    /// * `submitter_did`  - DID of request sender
    /// * `payment_method` 
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_txn_fees_req_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));
        Payment::_build_get_txn_fees_req(command_handle, wallet_handle, submitter_did, payment_method, cb)
    }

    fn _build_get_txn_fees_req(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let payment_method = c_str!(payment_method);

        unsafe { payments::indy_build_get_txn_fees_req(command_handle, wallet_handle, submitter_did.as_ptr(), payment_method.as_ptr(), cb) }
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
    pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_get_txn_fees_response(command_handle, payment_method, resp_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Parses response for Indy request for getting fees
    ///
    /// # Arguments
    /// * `command_handle` 
    /// * `payment_method` 
    /// * `resp_json` - response for Indy request for getting fees
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `fees_json`  {
    ///   txnType1: amount1,
    ///   txnType2: amount2,
    ///   .................
    ///   txnTypeN: amountN,
    /// }
    pub fn parse_get_txn_fees_response_timeout(payment_method: &str, resp_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Payment::_parse_get_txn_fees_response(command_handle, payment_method, resp_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Parses response for Indy request for getting fees
    ///
    /// # Arguments
    /// * `command_handle` 
    /// * `payment_method` 
    /// * `resp_json` - response for Indy request for getting fees
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_get_txn_fees_response_async<F: 'static>(payment_method: &str, resp_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Payment::_parse_get_txn_fees_response(command_handle, payment_method, resp_json, cb)
    }

    fn _parse_get_txn_fees_response(command_handle: IndyHandle, payment_method: &str, resp_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        unsafe { payments::indy_parse_get_txn_fees_response(command_handle, payment_method.as_ptr(), resp_json.as_ptr(), cb) }
    }
}
