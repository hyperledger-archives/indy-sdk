use super::{ErrorCode, IndyHandle};

use std::ffi::CString;

use ffi::payments;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

pub struct Payment {}

impl Payment {
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

        let payment_method = c_str!(payment_method);
        let err = unsafe {
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
        };
        ResultHandler::empty(err, receiver)
    }

    pub fn create_payment_address(wallet_handle: i32, payment_method: &str, config: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let payment_method = c_str!(payment_method);
        let config = c_str!(config);

        let err = unsafe {
            payments::indy_create_payment_address(command_handle,
                                        wallet_handle,
                                        payment_method.as_ptr(),
                                        config.as_ptr(),
                                        cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn list_payment_addresses(wallet_handle: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = unsafe {
            payments::indy_list_payment_addresses(command_handle,
                                                  wallet_handle,
                                                  cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn add_request_fees(wallet_handle: IndyHandle, submitter_did: &str, req_json: &str, inputs_json: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let submitter_did = c_str!(submitter_did);
        let req_json = c_str!(req_json);
        let inputs_json = c_str!(inputs_json);
        let outputs_json = c_str!(outputs_json);

        let err = unsafe {
            payments::indy_add_request_fees(command_handle,
                                  wallet_handle,
                                  submitter_did.as_ptr(),
                                  req_json.as_ptr(),
                                  inputs_json.as_ptr(),
                                  outputs_json.as_ptr(),
                                  cb)
        };

        ResultHandler::two(err, receiver)
    }

    pub fn parse_response_with_fees(payment_method: &str,
                                    resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        let err = unsafe {
            payments::indy_parse_response_with_fees(command_handle,
                                                  payment_method.as_ptr(),
                                                    resp_json.as_ptr(),
                                                    cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn build_get_utxo_request(wallet_handle: IndyHandle, submitter_did: &str, payment_address: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string_string();

        let submitter_did = c_str!(submitter_did);
        let payment_address = c_str!(payment_address);

        let err = unsafe {
            payments::indy_build_get_utxo_request(command_handle,
                                        wallet_handle,
                                        submitter_did.as_ptr(),
                                        payment_address.as_ptr(),
                                        cb)
        };

        ResultHandler::two(err, receiver)
    }

    pub fn parse_get_utxo_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string();

        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        let err = unsafe {
            payments::indy_parse_get_utxo_response(command_handle,
                                         payment_method.as_ptr(),
                                         resp_json.as_ptr(),
                                         cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn build_payment_req(wallet_handle: IndyHandle, submitter_did: &str, inputs: &str, outputs: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string_string();

        let submitter_did = c_str!(submitter_did);
        let inputs = c_str!(inputs);
        let outputs = c_str!(outputs);

        let err = unsafe {
            payments::indy_build_payment_req(command_handle,
                                   wallet_handle,
                                   submitter_did.as_ptr(),
                                   inputs.as_ptr(),
                                   outputs.as_ptr(),
                                   cb)
        };

        ResultHandler::two(err, receiver)
    }

    pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string();

        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        let err = unsafe {
            payments::indy_parse_payment_response(command_handle,
                                        payment_method.as_ptr(),
                                        resp_json.as_ptr(),
                                        cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn build_mint_req(wallet_handle: IndyHandle, submitter_did: &str, outputs_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string_string();

        let submitter_did = c_str!(submitter_did);
        let outputs_json = c_str!(outputs_json);

        let err = unsafe {
            payments::indy_build_mint_req(command_handle,
                                wallet_handle,
                                submitter_did.as_ptr(),
                                outputs_json.as_ptr(),
                                cb)
        };

        ResultHandler::two(err, receiver)
    }

    pub fn build_set_txn_fees_req(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str, fees_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string();

        let submitter_did = c_str!(submitter_did);
        let payment_method = c_str!(payment_method);
        let fees_json = c_str!(fees_json);

        let err = unsafe {
            payments::indy_build_set_txn_fees_req(command_handle,
                                        wallet_handle,
                                        submitter_did.as_ptr(),
                                        payment_method.as_ptr(),
                                        fees_json.as_ptr(),
                                        cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn build_get_txn_fees_req(wallet_handle: IndyHandle, submitter_did: &str, payment_method: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string();

        let submitter_did = c_str!(submitter_did);
        let payment_method = c_str!(payment_method);

        let err = unsafe {
            payments::indy_build_get_txn_fees_req(command_handle,
                                        wallet_handle,
                                        submitter_did.as_ptr(),
                                        payment_method.as_ptr(),
                                        cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) =
            ClosureHandler::cb_ec_string();

        let payment_method = c_str!(payment_method);
        let resp_json = c_str!(resp_json);

        let err = unsafe {
            payments::indy_parse_get_txn_fees_response(command_handle,
                                             payment_method.as_ptr(),
                                             resp_json.as_ptr(),
                                             cb)
        };

        ResultHandler::one(err, receiver)
    }
}
