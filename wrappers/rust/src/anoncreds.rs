use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;
use std::ptr::null;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

use native::anoncreds;
use native::{ResponseStringStringCB,
          ResponseI32UsizeCB,
          ResponseStringStringStringCB,
          ResponseStringCB,
          ResponseI32CB,
          ResponseEmptyCB,
          ResponseBoolCB};

pub struct Issuer {}

impl Issuer {
    pub fn create_schema(issuer_did: &str, name: &str, version: &str, attrs: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Issuer::_create_schema(command_handle, issuer_did, name, version, attrs, cb);

        ResultHandler::two(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_schema_timeout(issuer_did: &str, name: &str, version: &str, attrs: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Issuer::_create_schema(command_handle, issuer_did, name, version, attrs, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_schema_async<F: 'static>(issuer_did: &str, name: &str, version: &str, attrs: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Issuer::_create_schema(command_handle, issuer_did, name, version, attrs, cb)
    }

    fn _create_schema(command_handle: IndyHandle, issuer_did: &str, name: &str, version: &str, attrs: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let issuer_did = c_str!(issuer_did);
        let name = c_str!(name);
        let version = c_str!(version);
        let attrs = c_str!(attrs);

        ErrorCode::from(unsafe {
          anoncreds::indy_issuer_create_schema(command_handle, issuer_did.as_ptr(), name.as_ptr(), version.as_ptr(), attrs.as_ptr(), cb)
        })
    }

    pub fn create_and_store_credential_def(wallet_handle: IndyHandle, issuer_did: &str, schema_json: &str, tag: &str, signature_type: Option<&str>, config_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Issuer::_create_and_store_credential_def(command_handle, wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb);

        ResultHandler::two(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_and_store_credential_def_timeout(wallet_handle: IndyHandle, issuer_did: &str, schema_json: &str, tag: &str, signature_type: Option<&str>, config_json: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Issuer::_create_and_store_credential_def(command_handle, wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_and_store_credential_def_async<F: 'static>(wallet_handle: IndyHandle, issuer_did: &str, schema_json: &str, tag: &str, signature_type: Option<&str>, config_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Issuer::_create_and_store_credential_def(command_handle, wallet_handle, issuer_did, schema_json, tag, signature_type, config_json, cb)
    }

    fn _create_and_store_credential_def(command_handle: IndyHandle, wallet_handle: IndyHandle, issuer_did: &str, schema_json: &str, tag: &str, signature_type: Option<&str>, config_json: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let issuer_did = c_str!(issuer_did);
        let schema_json = c_str!(schema_json);
        let tag = c_str!(tag);
        let signature_type_str = opt_c_str!(signature_type);
        let config_json = c_str!(config_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_issuer_create_and_store_credential_def(command_handle, wallet_handle, issuer_did.as_ptr(), schema_json.as_ptr(), tag.as_ptr(), opt_c_ptr!(signature_type, signature_type_str), config_json.as_ptr(), cb)
        })
    }

    pub fn create_and_store_revoc_reg(wallet_handle: IndyHandle, issuer_did: &str, revoc_def_type: Option<&str>, tag: &str, cred_def_id: &str, config_json: &str, tails_writer_handle: IndyHandle) -> Result<(String, String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_string();

        let err = Issuer::_create_and_store_revoc_reg(command_handle, wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb);

        ResultHandler::three(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_and_store_revoc_reg_timeout(wallet_handle: IndyHandle, issuer_did: &str, revoc_def_type: Option<&str>, tag: &str, cred_def_id: &str, config_json: &str, tails_writer_handle: IndyHandle, timeout: Duration) -> Result<(String, String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_string();

        let err = Issuer::_create_and_store_revoc_reg(command_handle, wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb);

        ResultHandler::three_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_and_store_revoc_reg_async<F: 'static>(wallet_handle: IndyHandle, issuer_did: &str, revoc_def_type: Option<&str>, tag: &str, cred_def_id: &str, config_json: &str, tails_writer_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string_string(Box::new(closure));

        Issuer::_create_and_store_revoc_reg(command_handle, wallet_handle, issuer_did, revoc_def_type, tag, cred_def_id, config_json, tails_writer_handle, cb)
    }

    fn _create_and_store_revoc_reg(command_handle: IndyHandle, wallet_handle: IndyHandle, issuer_did: &str, revoc_def_type: Option<&str>, tag: &str, cred_def_id: &str, config_json: &str, tails_writer_handle: IndyHandle, cb: Option<ResponseStringStringStringCB>) -> ErrorCode {
        let issuer_did = c_str!(issuer_did);
        let revoc_def_type_str = opt_c_str!(revoc_def_type);
        let tag = c_str!(tag);
        let cred_def_id = c_str!(cred_def_id);
        let config_json = c_str!(config_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_issuer_create_and_store_revoc_reg(command_handle, wallet_handle, issuer_did.as_ptr(), opt_c_ptr!(revoc_def_type, revoc_def_type_str), tag.as_ptr(), cred_def_id.as_ptr(), config_json.as_ptr(), tails_writer_handle, cb)
        })
    }

    pub fn create_credential_offer(wallet_handle: IndyHandle, cred_def_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Issuer::_create_credential_offer(command_handle, wallet_handle, cred_def_id, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_credential_offer_timeout(wallet_handle: IndyHandle, cred_def_id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Issuer::_create_credential_offer(command_handle, wallet_handle, cred_def_id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_credential_offer_async<F: 'static>(wallet_handle: IndyHandle, cred_def_id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Issuer::_create_credential_offer(command_handle, wallet_handle, cred_def_id, cb)
    }

    fn _create_credential_offer(command_handle: IndyHandle, wallet_handle: IndyHandle, cred_def_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let cred_def_id = c_str!(cred_def_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_issuer_create_credential_offer(command_handle, wallet_handle, cred_def_id.as_ptr(), cb)
        })
    }

    pub fn create_credential(wallet_handle: IndyHandle, cred_offer_json: &str, cred_req_json: &str, cred_values_json: &str, rev_reg_id: Option<&str>, blob_storage_reader_handle: IndyHandle) -> Result<(String, Option<String>, Option<String>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string_opt_string();

        let err = Issuer::_create_credential(command_handle, wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb);

        ResultHandler::three(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_credential_timeout(wallet_handle: IndyHandle, cred_offer_json: &str, cred_req_json: &str, cred_values_json: &str, rev_reg_id: Option<&str>, blob_storage_reader_handle: IndyHandle, timeout: Duration) -> Result<(String, Option<String>, Option<String>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string_opt_string();

        let err = Issuer::_create_credential(command_handle, wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb);

        ResultHandler::three_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_credential_async<F: 'static>(wallet_handle: IndyHandle, cred_offer_json: &str, cred_req_json: &str, cred_values_json: &str, rev_reg_id: Option<&str>, blob_storage_reader_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, Option<String>, Option<String>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_opt_string_opt_string(Box::new(closure));

        Issuer::_create_credential(command_handle, wallet_handle, cred_offer_json, cred_req_json, cred_values_json, rev_reg_id, blob_storage_reader_handle, cb)
    }

    fn _create_credential(command_handle: IndyHandle, wallet_handle: IndyHandle, cred_offer_json: &str, cred_req_json: &str, cred_values_json: &str, rev_reg_id: Option<&str>, blob_storage_reader_handle: IndyHandle, cb: Option<ResponseStringStringStringCB>) -> ErrorCode {
        let cred_offer_json = c_str!(cred_offer_json);
        let cred_req_json = c_str!(cred_req_json);
        let cred_values_json = c_str!(cred_values_json);
        let rev_reg_id_str = opt_c_str!(rev_reg_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_issuer_create_credential(command_handle, wallet_handle, cred_offer_json.as_ptr(), cred_req_json.as_ptr(), cred_values_json.as_ptr(), opt_c_ptr!(rev_reg_id, rev_reg_id_str), blob_storage_reader_handle, cb)
        })
    }

    pub fn revoke_credential(wallet_handle: IndyHandle, blob_storage_reader_cfg_handle: IndyHandle, rev_reg_id: &str, cred_revoc_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Issuer::_revoke_credential(command_handle, wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id, cred_revoc_id, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn revoke_credential_timeout(wallet_handle: IndyHandle, blob_storage_reader_cfg_handle: IndyHandle, rev_reg_id: &str, cred_revoc_id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Issuer::_revoke_credential(command_handle, wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id, cred_revoc_id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn revoke_credential_async<F: 'static>(wallet_handle: IndyHandle, blob_storage_reader_cfg_handle: IndyHandle, rev_reg_id: &str, cred_revoc_id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Issuer::_revoke_credential(command_handle, wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id, cred_revoc_id, cb)
    }

    fn _revoke_credential(command_handle: IndyHandle, wallet_handle: IndyHandle, blob_storage_reader_cfg_handle: IndyHandle, rev_reg_id: &str, cred_revoc_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let rev_reg_id = c_str!(rev_reg_id);
        let cred_revoc_id = c_str!(cred_revoc_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_issuer_revoke_credential(command_handle, wallet_handle, blob_storage_reader_cfg_handle, rev_reg_id.as_ptr(), cred_revoc_id.as_ptr(), cb)
        })
    }

    pub fn merge_revocation_registry_deltas(rev_reg_delta_json: &str, other_rev_reg_delta_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Issuer::_merge_revocation_registry_deltas(command_handle, rev_reg_delta_json, other_rev_reg_delta_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn merge_revocation_registry_deltas_timeout(rev_reg_delta_json: &str, other_rev_reg_delta_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Issuer::_merge_revocation_registry_deltas(command_handle, rev_reg_delta_json, other_rev_reg_delta_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn merge_revocation_registry_deltas_async<F: 'static>(rev_reg_delta_json: &str, other_rev_reg_delta_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Issuer::_merge_revocation_registry_deltas(command_handle, rev_reg_delta_json, other_rev_reg_delta_json, cb)
    }

    fn _merge_revocation_registry_deltas(command_handle: IndyHandle, rev_reg_delta_json: &str, other_rev_reg_delta_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let rev_reg_delta_json = c_str!(rev_reg_delta_json);
        let other_rev_reg_delta_json = c_str!(other_rev_reg_delta_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_issuer_merge_revocation_registry_deltas(command_handle, rev_reg_delta_json.as_ptr(), other_rev_reg_delta_json.as_ptr(), cb)
        })
    }
}

pub struct Prover {}

impl Prover {
    pub fn create_master_secret(wallet_handle: IndyHandle, master_secret_id: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_create_master_secret(command_handle, wallet_handle, master_secret_id, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_master_secret_timeout(wallet_handle: IndyHandle, master_secret_id: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_create_master_secret(command_handle, wallet_handle, master_secret_id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_master_secret_async<F: 'static>(wallet_handle: IndyHandle, master_secret_id: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::_create_master_secret(command_handle, wallet_handle, master_secret_id, cb)
    }

    fn _create_master_secret(command_handle: IndyHandle, wallet_handle: IndyHandle, master_secret_id: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let master_secret_id_str = opt_c_str!(master_secret_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_create_master_secret(command_handle, wallet_handle, opt_c_ptr!(master_secret_id, master_secret_id_str), cb)
        })
    }

    pub fn get_credential(wallet_handle: IndyHandle, cred_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_get_credential(command_handle, wallet_handle, cred_id, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn get_credential_timeout(wallet_handle: IndyHandle, cred_id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_get_credential(command_handle, wallet_handle, cred_id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_credential_async<F: 'static>(wallet_handle: IndyHandle, cred_id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::_get_credential(command_handle, wallet_handle, cred_id, cb)
    }

    fn _get_credential(command_handle: IndyHandle, wallet_handle: IndyHandle, cred_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let cred_id = c_str!(cred_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_get_credential(command_handle, wallet_handle, cred_id.as_ptr(), cb)
        })
    }

    pub fn create_credential_req(wallet_handle: IndyHandle, prover_did: &str, cred_offer_json: &str, cred_def_json: &str, master_secret_id: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Prover::_create_credential_req(command_handle, wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb);

        ResultHandler::two(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_credential_req_timeout(wallet_handle: IndyHandle, prover_did: &str, cred_offer_json: &str, cred_def_json: &str, master_secret_id: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Prover::_create_credential_req(command_handle, wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_credential_req_async<F: 'static>(wallet_handle: IndyHandle, prover_did: &str, cred_offer_json: &str, cred_def_json: &str, master_secret_id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Prover::_create_credential_req(command_handle, wallet_handle, prover_did, cred_offer_json, cred_def_json, master_secret_id, cb)
    }

    fn _create_credential_req(command_handle: IndyHandle, wallet_handle: IndyHandle, prover_did: &str, cred_offer_json: &str, cred_def_json: &str, master_secret_id: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let prover_did = c_str!(prover_did);
        let cred_offer_json = c_str!(cred_offer_json);
        let cred_def_json = c_str!(cred_def_json);
        let master_secret_id = c_str!(master_secret_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_create_credential_req(command_handle, wallet_handle, prover_did.as_ptr(), cred_offer_json.as_ptr(), cred_def_json.as_ptr(), master_secret_id.as_ptr(), cb)
        })
    }

    pub fn store_credential(wallet_handle: IndyHandle, cred_id: Option<&str>, cred_req_metadata_json: &str, cred_json: &str, cred_def_json: &str, rev_reg_def_json: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_store_credential(command_handle, wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn store_credential_timeout(wallet_handle: IndyHandle, cred_id: Option<&str>, cred_req_metadata_json: &str, cred_json: &str, cred_def_json: &str, rev_reg_def_json: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_store_credential(command_handle, wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn store_credential_async<F: 'static>(wallet_handle: IndyHandle, cred_id: Option<&str>, cred_req_metadata_json: &str, cred_json: &str, cred_def_json: &str, rev_reg_def_json: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::_store_credential(command_handle, wallet_handle, cred_id, cred_req_metadata_json, cred_json, cred_def_json, rev_reg_def_json, cb)
    }

    fn _store_credential(command_handle: IndyHandle, wallet_handle: IndyHandle, cred_id: Option<&str>, cred_req_metadata_json: &str, cred_json: &str, cred_def_json: &str, rev_reg_def_json: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let cred_id_str = opt_c_str!(cred_id);
        let cred_req_metadata_json = c_str!(cred_req_metadata_json);
        let cred_json = c_str!(cred_json);
        let cred_def_json = c_str!(cred_def_json);
        let rev_reg_def_json_str = opt_c_str!(rev_reg_def_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_store_credential(command_handle, wallet_handle, opt_c_ptr!(cred_id, cred_id_str), cred_req_metadata_json.as_ptr(), cred_json.as_ptr(), cred_def_json.as_ptr(), opt_c_ptr!(rev_reg_def_json, rev_reg_def_json_str), cb)
        })
    }

    pub fn get_credentials(wallet_handle: IndyHandle, filter_json: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_get_credentials(command_handle, wallet_handle, filter_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn get_credentials_timeout(wallet_handle: IndyHandle, filter_json: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_get_credentials(command_handle, wallet_handle, filter_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_credentials_async<F: 'static>(wallet_handle: IndyHandle, filter_json: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::_get_credentials(command_handle, wallet_handle, filter_json, cb)
    }

    fn _get_credentials(command_handle: IndyHandle, wallet_handle: IndyHandle, filter_json: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let filter_json_str = opt_c_str!(filter_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_get_credentials(command_handle, wallet_handle, opt_c_ptr!(filter_json, filter_json_str), cb)
        })
    }

    pub fn search_credentials(wallet_handle: IndyHandle, query_json: Option<&str>) -> Result<(i32, usize), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32_usize();

        let err = Prover::_search_credentials(command_handle, wallet_handle, query_json, cb);

        ResultHandler::two(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn search_credentials_timeout(wallet_handle: IndyHandle, query_json: Option<&str>, timeout: Duration) -> Result<(i32, usize), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32_usize();

        let err = Prover::_search_credentials(command_handle, wallet_handle, query_json, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn search_credentials_async<F: 'static>(wallet_handle: IndyHandle, query_json: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, i32, usize) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32_usize(Box::new(closure));

        Prover::_search_credentials(command_handle, wallet_handle, query_json, cb)
    }

    fn _search_credentials(command_handle: IndyHandle, wallet_handle: IndyHandle, query_json: Option<&str>, cb: Option<ResponseI32UsizeCB>) -> ErrorCode {
        let query_json_str = opt_c_str!(query_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_search_credentials(command_handle, wallet_handle, opt_c_ptr!(query_json, query_json_str), cb)
        })
    }

    pub fn fetch_credentials(search_handle: IndyHandle, count: usize) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_fetch_credentials(command_handle, search_handle, count, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn fetch_credentials_timeout(search_handle: IndyHandle, count: usize, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_fetch_credentials(command_handle, search_handle, count, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn fetch_credentials_async<F: 'static>(search_handle: IndyHandle, count: usize, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::_fetch_credentials(command_handle, search_handle, count, cb)
    }

    fn _fetch_credentials(command_handle: IndyHandle, search_handle: IndyHandle, count: usize, cb: Option<ResponseStringCB>) -> ErrorCode {

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_fetch_credentials(command_handle, search_handle, count, cb)
        })
    }

    pub fn close_credentials_search(search_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Prover::_close_credentials_search(command_handle, search_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn close_credentials_search_timeout(search_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Prover::_close_credentials_search(command_handle, search_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn close_credentials_search_async<F: 'static>(search_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Prover::_close_credentials_search(command_handle, search_handle, cb)
    }

    fn _close_credentials_search(command_handle: IndyHandle, search_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_close_credentials_search(command_handle, search_handle, cb)
        })
    }

    pub fn get_credentials_for_proof_req(wallet_handle: IndyHandle, proof_request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_get_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn get_credentials_for_proof_req_timeout(wallet_handle: IndyHandle, proof_request_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_get_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_credentials_for_proof_req_async<F: 'static>(wallet_handle: IndyHandle, proof_request_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::_get_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, cb)
    }

    fn _get_credentials_for_proof_req(command_handle: IndyHandle, wallet_handle: IndyHandle, proof_request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let proof_request_json = c_str!(proof_request_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_get_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json.as_ptr(), cb)
        })
    }

    pub fn search_credentials_for_proof_req(wallet_handle: IndyHandle, proof_request_json: &str, extra_query_json: Option<&str>) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Prover::_search_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, extra_query_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn search_credentials_for_proof_req_timeout(wallet_handle: IndyHandle, proof_request_json: &str, extra_query_json: Option<&str>, timeout: Duration) -> Result<i32, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_i32();

        let err = Prover::_search_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, extra_query_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn search_credentials_for_proof_req_async<F: 'static>(wallet_handle: IndyHandle, proof_request_json: &str, extra_query_json: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, i32) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_i32(Box::new(closure));

        Prover::_search_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json, extra_query_json, cb)
    }

    fn _search_credentials_for_proof_req(command_handle: IndyHandle, wallet_handle: IndyHandle, proof_request_json: &str, extra_query_json: Option<&str>, cb: Option<ResponseI32CB>) -> ErrorCode {
        let proof_request_json = c_str!(proof_request_json);
        let extra_query_json_str = opt_c_str!(extra_query_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_search_credentials_for_proof_req(command_handle, wallet_handle, proof_request_json.as_ptr(), opt_c_ptr!(extra_query_json, extra_query_json_str), cb)
        })
    }

    pub fn _fetch_credentials_for_proof_req(search_handle: IndyHandle, item_referent: &str, count: usize) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::__fetch_credentials_for_proof_req(command_handle, search_handle, item_referent, count, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn _fetch_credentials_for_proof_req_timeout(search_handle: IndyHandle, item_referent: &str, count: usize, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::__fetch_credentials_for_proof_req(command_handle, search_handle, item_referent, count, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn _fetch_credentials_for_proof_req_async<F: 'static>(search_handle: IndyHandle, item_referent: &str, count: usize, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::__fetch_credentials_for_proof_req(command_handle, search_handle, item_referent, count, cb)
    }

    fn __fetch_credentials_for_proof_req(command_handle: IndyHandle, search_handle: IndyHandle, item_referent: &str, count: usize, cb: Option<ResponseStringCB>) -> ErrorCode {
        let item_referent = c_str!(item_referent);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_fetch_credentials_for_proof_req(command_handle, search_handle, item_referent.as_ptr(), count, cb)
        })
    }

    pub fn _close_credentials_search_for_proof_req(search_handle: IndyHandle) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Prover::__close_credentials_search_for_proof_req(command_handle, search_handle, cb);

        ResultHandler::empty(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn _close_credentials_search_for_proof_req_timeout(search_handle: IndyHandle, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Prover::__close_credentials_search_for_proof_req(command_handle, search_handle, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn _close_credentials_search_for_proof_req_async<F: 'static>(search_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Prover::__close_credentials_search_for_proof_req(command_handle, search_handle, cb)
    }

    fn __close_credentials_search_for_proof_req(command_handle: IndyHandle, search_handle: IndyHandle, cb: Option<ResponseEmptyCB>) -> ErrorCode {

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_close_credentials_search_for_proof_req(command_handle, search_handle, cb)
        })
    }

    pub fn create_proof(wallet_handle: IndyHandle, proof_req_json: &str, requested_credentials_json: &str, master_secret_id: &str, schemas_json: &str, credential_defs_json: &str, rev_states_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_create_proof(command_handle, wallet_handle, proof_req_json, requested_credentials_json, master_secret_id, schemas_json, credential_defs_json, rev_states_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_proof_timeout(wallet_handle: IndyHandle, proof_req_json: &str, requested_credentials_json: &str, master_secret_id: &str, schemas_json: &str, credential_defs_json: &str, rev_states_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Prover::_create_proof(command_handle, wallet_handle, proof_req_json, requested_credentials_json, master_secret_id, schemas_json, credential_defs_json, rev_states_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_proof_async<F: 'static>(wallet_handle: IndyHandle, proof_req_json: &str, requested_credentials_json: &str, master_secret_id: &str, schemas_json: &str, credential_defs_json: &str, rev_states_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Prover::_create_proof(command_handle, wallet_handle, proof_req_json, requested_credentials_json, master_secret_id, schemas_json, credential_defs_json, rev_states_json, cb)
    }

    fn _create_proof(command_handle: IndyHandle, wallet_handle: IndyHandle, proof_req_json: &str, requested_credentials_json: &str, master_secret_id: &str, schemas_json: &str, credential_defs_json: &str, rev_states_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let proof_req_json = c_str!(proof_req_json);
        let requested_credentials_json = c_str!(requested_credentials_json);
        let master_secret_id = c_str!(master_secret_id);
        let schemas_json = c_str!(schemas_json);
        let credential_defs_json = c_str!(credential_defs_json);
        let rev_states_json = c_str!(rev_states_json);

        ErrorCode::from(unsafe {
          anoncreds::indy_prover_create_proof(command_handle, wallet_handle, proof_req_json.as_ptr(), requested_credentials_json.as_ptr(), master_secret_id.as_ptr(), schemas_json.as_ptr(), credential_defs_json.as_ptr(), rev_states_json.as_ptr(), cb)
        })
    }
}

pub struct Verifier {}

impl Verifier {
    pub fn verify_proof(proof_request_json: &str, proof_json: &str, schemas_json: &str, credential_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

        let err = Verifier::_verify_proof(command_handle, proof_request_json, proof_json, schemas_json, credential_defs_json, rev_reg_defs_json, rev_regs_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn verify_proof_timeout(proof_request_json: &str, proof_json: &str, schemas_json: &str, credential_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str, timeout: Duration) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

        let err = Verifier::_verify_proof(command_handle, proof_request_json, proof_json, schemas_json, credential_defs_json, rev_reg_defs_json, rev_regs_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn verify_proof_async<F: 'static>(proof_request_json: &str, proof_json: &str, schemas_json: &str, credential_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, bool) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_bool(Box::new(closure));

        Verifier::_verify_proof(command_handle, proof_request_json, proof_json, schemas_json, credential_defs_json, rev_reg_defs_json, rev_regs_json, cb)
    }

    fn _verify_proof(command_handle: IndyHandle, proof_request_json: &str, proof_json: &str, schemas_json: &str, credential_defs_json: &str, rev_reg_defs_json: &str, rev_regs_json: &str, cb: Option<ResponseBoolCB>) -> ErrorCode {
        let proof_request_json = c_str!(proof_request_json);
        let proof_json = c_str!(proof_json);
        let schemas_json = c_str!(schemas_json);
        let credential_defs_json = c_str!(credential_defs_json);
        let rev_reg_defs_json = c_str!(rev_reg_defs_json);
        let rev_regs_json = c_str!(rev_regs_json);

        ErrorCode::from(unsafe {
            anoncreds::indy_verifier_verify_proof(command_handle, proof_request_json.as_ptr(), proof_json.as_ptr(), schemas_json.as_ptr(), credential_defs_json.as_ptr(), rev_reg_defs_json.as_ptr(), rev_regs_json.as_ptr(), cb)
        })
    }
}

pub struct AnonCreds {}

impl AnonCreds {
    pub fn create_revocation_state(blob_storage_reader_handle: IndyHandle, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = AnonCreds::_create_revocation_state(command_handle, blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_revocation_state_timeout(blob_storage_reader_handle: IndyHandle, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = AnonCreds::_create_revocation_state(command_handle, blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_revocation_state_async<F: 'static>(blob_storage_reader_handle: IndyHandle, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        AnonCreds::_create_revocation_state(command_handle, blob_storage_reader_handle, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb)
    }

    fn _create_revocation_state(command_handle: IndyHandle, blob_storage_reader_handle: IndyHandle, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let rev_reg_def_json = c_str!(rev_reg_def_json);
        let rev_reg_delta_json = c_str!(rev_reg_delta_json);
        let cred_rev_id = c_str!(cred_rev_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_create_revocation_state(command_handle, blob_storage_reader_handle, rev_reg_def_json.as_ptr(), rev_reg_delta_json.as_ptr(), timestamp, cred_rev_id.as_ptr(), cb)
        })
    }

    pub fn update_revocation_state(blob_storage_reader_handle: IndyHandle, rev_state_json: &str, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = AnonCreds::_update_revocation_state(command_handle, blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn update_revocation_state_timeout(blob_storage_reader_handle: IndyHandle, rev_state_json: &str, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = AnonCreds::_update_revocation_state(command_handle, blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn update_revocation_state_async<F: 'static>(blob_storage_reader_handle: IndyHandle, rev_state_json: &str, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        AnonCreds::_update_revocation_state(command_handle, blob_storage_reader_handle, rev_state_json, rev_reg_def_json, rev_reg_delta_json, timestamp, cred_rev_id, cb)
    }

    fn _update_revocation_state(command_handle: IndyHandle, blob_storage_reader_handle: IndyHandle, rev_state_json: &str, rev_reg_def_json: &str, rev_reg_delta_json: &str, timestamp: u64, cred_rev_id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let rev_state_json = c_str!(rev_state_json);
        let rev_reg_def_json = c_str!(rev_reg_def_json);
        let rev_reg_delta_json = c_str!(rev_reg_delta_json);
        let cred_rev_id = c_str!(cred_rev_id);

        ErrorCode::from(unsafe {
          anoncreds::indy_update_revocation_state(command_handle, blob_storage_reader_handle, rev_state_json.as_ptr(), rev_reg_def_json.as_ptr(), rev_reg_delta_json.as_ptr(), timestamp, cred_rev_id.as_ptr(), cb)
        })
    }
}
