use indy::IndyError;
use indy::payments;
use indy::future::Future;

use indy::WalletHandle;

pub struct Payment {}

impl Payment {
    pub fn create_payment_address(wallet_handle: WalletHandle, payment_method: &str, config: &str) -> Result<String, IndyError> {
        payments::create_payment_address(wallet_handle, payment_method, config).wait()
    }

    pub fn list_payment_addresses(wallet_handle: WalletHandle) -> Result<String, IndyError> {
        payments::list_payment_addresses(wallet_handle).wait()
    }

    pub fn add_request_fees(wallet_handle: WalletHandle, submitter_did: Option<&str>, req_json: &str, inputs_json: &str, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
        payments::add_request_fees(wallet_handle, submitter_did, req_json, inputs_json, outputs_json, extra).wait()
    }

    pub fn build_get_payment_sources_request(wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_address: &str) -> Result<(String, String), IndyError> {
        payments::build_get_payment_sources_with_from_request(wallet_handle, submitter_did, payment_address, None).wait()
    }

    pub fn parse_get_payment_sources_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
        payments::parse_get_payment_sources_with_from_response(payment_method, resp_json).wait().map(|(s, _)| s)
    }

    pub fn build_payment_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, inputs: &str, outputs: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
        payments::build_payment_req(wallet_handle, submitter_did, inputs, outputs, extra).wait()
    }

    pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
        payments::parse_payment_response(payment_method, resp_json).wait()
    }

    pub fn build_mint_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
        payments::build_mint_req(wallet_handle, submitter_did, outputs_json, extra).wait()
    }

    pub fn build_set_txn_fees_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_method: &str, fees_json: &str) -> Result<String, IndyError> {
        payments::build_set_txn_fees_req(wallet_handle, submitter_did, payment_method, fees_json).wait()
    }

    pub fn build_get_txn_fees_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, payment_method: &str) -> Result<String, IndyError> {
        payments::build_get_txn_fees_req(wallet_handle, submitter_did, payment_method).wait()
    }

    pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
        payments::parse_get_txn_fees_response(payment_method, resp_json).wait()
    }

    pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
        payments::parse_response_with_fees(payment_method, resp_json).wait()
    }

    pub fn build_verify_payment_req(wallet_handle: WalletHandle, submitter_did: Option<&str>, receipt: &str) -> Result<(String, String), IndyError> {
        payments::build_verify_payment_req(wallet_handle, submitter_did, receipt).wait()
    }

    pub fn parse_verify_payment_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
        payments::parse_verify_payment_response(payment_method, resp_json).wait()
    }

    pub fn prepare_payment_extra_with_acceptance_data(extra_json: Option<&str>, text: Option<&str>, version: Option<&str>,
                                                      taa_digest: Option<&str>, mechanism: &str, time: u64) -> Result<String, IndyError> {
        payments::prepare_extra_with_acceptance_data(extra_json, text, version, taa_digest, mechanism, time).wait()
    }

    pub fn sign_with_address(wallet_handle: WalletHandle, address: &str, input: &str) -> Result<Vec<u8>, IndyError> {
        payments::sign_with_address(wallet_handle, address, input.as_bytes()).wait()
    }

    pub fn verify_with_address(address: &str, input: &str, signature: &[u8]) -> Result<bool, IndyError> {
        payments::verify_with_address(address, input.as_bytes(), signature).wait()
    }
}
