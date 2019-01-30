use indy::ledger;
use indy::IndyError;
use indy::future::Future;

pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, IndyError> {
    ledger::submit_request(pool_handle, request_json).wait()
}

pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, IndyError> {
    ledger::sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json).wait()
}

pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: &str, alias: &str, role: &str) -> Result<String, IndyError> {
    ledger::build_nym_request(submitter_did, target_did, Some(verkey), Some(alias), Some(role)).wait()
}