use indy::ErrorCode;
use indy::future::Future;
use indy::ledger;

pub struct Ledger {}

impl Ledger {
    pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        ledger::sign_and_submit_request(pool_handle, wallet_handle, submitter_did, request_json).wait()
    }

    pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
        ledger::submit_request(pool_handle, request_json).wait()
    }

    pub fn submit_action(pool_handle: i32, request_json: &str, nodes: Option<&str>, timeout: Option<i32>) -> Result<String, ErrorCode> {
        ledger::submit_action(pool_handle, request_json, nodes, timeout).wait()
    }

    pub fn sign_request(wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        ledger::sign_request(wallet_handle, submitter_did, request_json).wait()
    }

    pub fn multi_sign_request(wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        ledger::multi_sign_request(wallet_handle, submitter_did, request_json).wait()
    }

    pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, ErrorCode> {
        ledger::build_nym_request(submitter_did, target_did, verkey, data, role).wait()
    }

    pub fn build_get_nym_request(submitter_did: Option<&str>, target_did: &str) -> Result<String, ErrorCode> {
        ledger::build_get_nym_request(submitter_did, target_did).wait()
    }

    pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        ledger::build_attrib_request(submitter_did, target_did, hash, raw, enc).wait()
    }

    pub fn build_get_attrib_request(submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        ledger::build_get_attrib_request(submitter_did, target_did, raw, hash, enc).wait()
    }

    pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        ledger::build_schema_request(submitter_did, data).wait()
    }

    pub fn build_get_schema_request(submitter_did: Option<&str>, id: &str) -> Result<String, ErrorCode> {
        ledger::build_get_schema_request(submitter_did, id).wait()
    }

    pub fn build_cred_def_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        ledger::build_cred_def_request(submitter_did, data).wait()
    }

    pub fn build_get_validator_info_request(submitter_did: &str) -> Result<String, ErrorCode> {
        ledger::build_get_validator_info_request(submitter_did).wait()
    }

    pub fn build_get_cred_def_request(submitter_did: Option<&str>, id: &str) -> Result<String, ErrorCode> {
        ledger::build_get_cred_def_request(submitter_did, id).wait()
    }

    pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        ledger::build_node_request(submitter_did, target_did, data).wait()
    }

    pub fn indy_build_pool_config_request(submitter_did: &str, writes: bool, force: bool) -> Result<String, ErrorCode> {
        ledger::build_pool_config_request(submitter_did, writes, force).wait()
    }

    pub fn indy_build_pool_restart_request(submitter_did: &str, action: &str, datetime: Option<&str>) -> Result<String, ErrorCode> {
        ledger::build_pool_restart_request(submitter_did, action, datetime).wait()
    }

    pub fn indy_build_pool_upgrade_request(submitter_did: &str, name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                                           justification: Option<&str>, reinstall: bool, force: bool, package: Option<&str>) -> Result<String, ErrorCode> {
        ledger::build_pool_upgrade_request(submitter_did, name, version, action, sha256,
                                               timeout, schedule, justification,
                                               reinstall, force, package).wait()
    }
}