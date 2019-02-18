use indy::IndyError;
use indy::payments;
use indy::future::Future;

pub fn create_payment_address(wallet_handle: i32, payment_method: &str, config: &str) -> Result<String, IndyError> {
    payments::create_payment_address(wallet_handle, payment_method, config).wait()
}

pub fn list_payment_addresses(wallet_handle: i32) -> Result<String, IndyError> {
    payments::list_payment_addresses(wallet_handle).wait()
}

pub fn add_request_fees(wallet_handle: i32, submitter_did: &str, req_json: &str, inputs_json: &str, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
    payments::add_request_fees(wallet_handle, Some(submitter_did), req_json, inputs_json, outputs_json, extra).wait()
}

pub fn parse_response_with_fees(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_response_with_fees(payment_method, resp_json).wait()
}

pub fn build_get_payment_sources_request(wallet_handle: i32, submitter_did: &str, payment_address: &str) -> Result<(String, String), IndyError> {
    payments::build_get_payment_sources_request(wallet_handle, Some(submitter_did), payment_address).wait()
}


pub fn parse_get_payment_sources_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_get_payment_sources_response(payment_method, resp_json).wait()
}

pub fn build_payment_req(wallet_handle: i32, submitter_did: &str, inputs: &str, outputs: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
    payments::build_payment_req(wallet_handle, Some(submitter_did), inputs, outputs, extra).wait()
}

pub fn parse_payment_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_payment_response(payment_method, resp_json).wait()
}

pub fn build_mint_req(wallet_handle: i32, submitter_did: &str, outputs_json: &str, extra: Option<&str>) -> Result<(String, String), IndyError> {
    payments::build_mint_req(wallet_handle, Some(submitter_did), outputs_json, extra).wait()
}

pub fn build_set_txn_fees_req(wallet_handle: i32, submitter_did: &str, payment_method: &str, fees_json: &str) -> Result<String, IndyError> {
    payments::build_set_txn_fees_req(wallet_handle, Some(submitter_did), payment_method, fees_json).wait()
}

pub fn build_get_txn_fees_req(wallet_handle: i32, submitter_did: &str, payment_method: &str) -> Result<String, IndyError> {
    payments::build_get_txn_fees_req(wallet_handle, Some(submitter_did), payment_method).wait()
}

pub fn parse_get_txn_fees_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_get_txn_fees_response(payment_method, resp_json).wait()
}

pub fn build_verify_payment_req(wallet_handle: i32, submitter_did: &str, receipt: &str) -> Result<(String, String), IndyError> {
    payments::build_verify_payment_req(wallet_handle, Some(submitter_did), receipt).wait()
}

pub fn parse_verify_payment_response(payment_method: &str, resp_json: &str) -> Result<String, IndyError> {
    payments::parse_verify_payment_response(payment_method, resp_json).wait()
}
