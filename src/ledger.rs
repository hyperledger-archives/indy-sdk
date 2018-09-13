use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;
use std::ptr::null;

use native::ledger;
use native::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseStringStringCB,
          ResponseStringStringU64CB};

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

pub struct Ledger {}

impl Ledger {
    /// Signs and submits request message to validator pool.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign), and sends signed request message
    /// to validator pool (see Pool::write_request).
    ///
    /// # Arguments
    /// * `pool_handle` - pool handle (created by Pool::open_ledger).
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    ///
    /// # Returns
    /// Request result as json.
    pub fn sign_and_submit_request(pool_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_sign_and_submit_request(command_handle, pool_handle, wallet_handle, submitter_did, request_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Signs and submits request message to validator pool.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign), and sends signed request message
    /// to validator pool (see Pool::write_request).
    ///
    /// # Arguments
    /// * `pool_handle` - pool handle (created by Pool::open_ledger).
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn sign_and_submit_request_timeout(pool_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_sign_and_submit_request(command_handle, pool_handle, wallet_handle, submitter_did, request_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Signs and submits request message to validator pool.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign), and sends signed request message
    /// to validator pool (see Pool::write_request).
    ///
    /// # Arguments
    /// * `pool_handle` - pool handle (created by Pool::open_ledger).
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn sign_and_submit_request_async<F: 'static>(pool_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_sign_and_submit_request(command_handle, pool_handle, wallet_handle, submitter_did, request_json, cb)
    }

    fn _sign_and_submit_request(command_handle: IndyHandle, pool_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let request_json = c_str!(request_json);

        ErrorCode::from(unsafe {
            ledger::indy_sign_and_submit_request(command_handle,
                                                 pool_handle,
                                                 wallet_handle,
                                                 submitter_did.as_ptr(),
                                                 request_json.as_ptr(),
                                                 cb)
        })
    }

    /// Publishes request message to validator pool (no signing, unlike Ledger::sign_and_submit_request).
    ///
    /// The request is sent to the validator pool as is. It's assumed that it's already prepared.
    ///
    /// # Arguments
    /// * `pool_handle` - pool handle (created by Pool::open_ledger).
    /// * `request_json` - Request data json.
    ///
    /// # Returns
    /// Request result as json.
    pub fn submit_request(pool_handle: IndyHandle, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_submit_request(command_handle, pool_handle, request_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Publishes request message to validator pool (no signing, unlike Ledger::sign_and_submit_request).
    ///
    /// The request is sent to the validator pool as is. It's assumed that it's already prepared.
    ///
    /// # Arguments
    /// * `pool_handle` - pool handle (created by Pool::open_ledger).
    /// * `request_json` - Request data json.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn submit_request_timeout(pool_handle: IndyHandle, request_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_submit_request(command_handle, pool_handle, request_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Publishes request message to validator pool (no signing, unlike Ledger::sign_and_submit_request).
    ///
    /// The request is sent to the validator pool as is. It's assumed that it's already prepared.
    ///
    /// # Arguments
    /// * `pool_handle` - pool handle (created by Pool::open_ledger).
    /// * `request_json` - Request data json.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn submit_request_async<F: 'static>(pool_handle: IndyHandle, request_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_submit_request(command_handle, pool_handle, request_json, cb)
    }

    fn _submit_request(command_handle: IndyHandle, pool_handle: IndyHandle, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let request_json = c_str!(request_json);

        ErrorCode::from(unsafe { ledger::indy_submit_request(command_handle, pool_handle, request_json.as_ptr(), cb) })
    }

    pub fn submit_action(pool_handle: IndyHandle, request_json: &str, nodes: &str, wait_timeout: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_submit_action(command_handle, pool_handle, request_json, nodes, wait_timeout, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn submit_action_timeout(pool_handle: IndyHandle, request_json: &str, nodes: &str, wait_timeout: i32, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_submit_action(command_handle, pool_handle, request_json, nodes, wait_timeout, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn submit_action_async<F: 'static>(pool_handle: IndyHandle, request_json: &str, nodes: &str, wait_timeout: i32, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_submit_action(command_handle, pool_handle, request_json, nodes, wait_timeout, cb)
    }

    fn _submit_action(command_handle: IndyHandle, pool_handle: IndyHandle, request_json: &str, nodes: &str, wait_timeout: i32, cb: Option<ResponseStringCB>) -> ErrorCode {
        let request_json = c_str!(request_json);
        let nodes = c_str!(nodes);

        ErrorCode::from(unsafe {
          ledger::indy_submit_action(command_handle, pool_handle, request_json.as_ptr(), nodes.as_ptr(), wait_timeout, cb)
        })
    }

    /// Signs request message.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    ///
    /// # Returns
    /// Signed request json.
    pub fn sign_request(wallet_handle: IndyHandle, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_sign_request(command_handle, wallet_handle, submitter_did, request_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Signs request message.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Signed request json.
    pub fn sign_request_timeout(wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_sign_request(command_handle, wallet_handle, submitter_did, request_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Signs request message.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn sign_request_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_sign_request(command_handle, wallet_handle, submitter_did, request_json, cb)
    }

    fn _sign_request(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let request_json = c_str!(request_json);

        ErrorCode::from(unsafe { ledger::indy_sign_request(command_handle, wallet_handle, submitter_did.as_ptr(), request_json.as_ptr(), cb) })
    }

    /// Multi signs request message.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    ///
    /// # Returns
    /// Signed request json.
    pub fn multi_sign_request(wallet_handle: IndyHandle, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_multi_sign_request(command_handle, wallet_handle, submitter_did, request_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Multi signs request message.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Signed request json.
    pub fn multi_sign_request_timeout(wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_multi_sign_request(command_handle, wallet_handle, submitter_did, request_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Multi signs request message.
    ///
    /// Adds submitter information to passed request json, signs it with submitter
    /// sign key (see Crypto::sign).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handle (created by Wallet::open).
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `request_json` - Request data json.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn multi_sign_request_async<F: 'static>(wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_multi_sign_request(command_handle, wallet_handle, submitter_did, request_json, cb)
    }

    fn _multi_sign_request(command_handle: IndyHandle, wallet_handle: IndyHandle, submitter_did: &str, request_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let request_json = c_str!(request_json);

        ErrorCode::from(unsafe { ledger::indy_multi_sign_request(command_handle, wallet_handle, submitter_did.as_ptr(), request_json.as_ptr(), cb) })
    }

    /// Builds a request to get a DDO.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet
    /// * `target_did` - Id of Identity stored in secured Wallet.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_ddo_request(submitter_did: Option<&str>, target_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_ddo_request(command_handle, submitter_did, target_did, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a request to get a DDO.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `target_did` - Id of Identity stored in secured Wallet.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_ddo_request_timeout(submitter_did: Option<&str>, target_did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_ddo_request(command_handle, submitter_did, target_did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a request to get a DDO.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `target_did` - Id of Identity stored in secured Wallet.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_ddo_request_async<F: 'static>(submitter_did: Option<&str>, target_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_ddo_request(command_handle, submitter_did, target_did, cb)
    }

    fn _build_get_ddo_request(command_handle: IndyHandle, submitter_did: Option<&str>, target_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let target_did = c_str!(target_did);

        ErrorCode::from(unsafe { ledger::indy_build_get_ddo_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), target_did.as_ptr(), cb) })
    }

    /// Builds a NYM request. Request to create a new NYM record for a specific user.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `verkey` - Target identity verification key as base58-encoded string.
    /// * `data`
    /// * `role` - Role of a user NYM record:
    ///                             null (common USER)
    ///                             TRUSTEE
    ///                             STEWARD
    ///                             TRUST_ANCHOR
    ///                             empty string to reset role
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>, data: Option<&str>, role: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_nym_request(command_handle, submitter_did, target_did, verkey, data, role, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a NYM request. Request to create a new NYM record for a specific user.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `verkey` - Target identity verification key as base58-encoded string.
    /// * `data`
    /// * `role` - Role of a user NYM record:
    ///                             null (common USER)
    ///                             TRUSTEE
    ///                             STEWARD
    ///                             TRUST_ANCHOR
    ///                             empty string to reset role
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_nym_request_timeout(submitter_did: &str, target_did: &str, verkey: Option<&str>, data: Option<&str>, role: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_nym_request(command_handle, submitter_did, target_did, verkey, data, role, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a NYM request. Request to create a new NYM record for a specific user.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `verkey` - Target identity verification key as base58-encoded string.
    /// * `data`
    /// * `role` - Role of a user NYM record:
    ///                             null (common USER)
    ///                             TRUSTEE
    ///                             STEWARD
    ///                             TRUST_ANCHOR
    ///                             empty string to reset role
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_nym_request_async<F: 'static>(submitter_did: &str, target_did: &str, verkey: Option<&str>, data: Option<&str>, role: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_nym_request(command_handle, submitter_did, target_did, verkey, data, role, cb)
    }

    fn _build_nym_request(command_handle: IndyHandle,
                          submitter_did: &str,
                          target_did: &str,
                          verkey: Option<&str>,
                          data: Option<&str>,
                          role: Option<&str>,
                          cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let target_did = c_str!(target_did);

        let verkey_str = opt_c_str!(verkey);
        let data_str = opt_c_str!(data);
        let role_str = opt_c_str!(role);

        ErrorCode::from(unsafe {
            ledger::indy_build_nym_request(command_handle,
                                           submitter_did.as_ptr(),
                                           target_did.as_ptr(),
                                           opt_c_ptr!(verkey, verkey_str),
                                           opt_c_ptr!(data, data_str),
                                           opt_c_ptr!(role, role_str),
                                           cb)
        })
    }

    /// Builds a GET_NYM request. Request to get information about a DID (NYM).
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_nym_request(submitter_did: Option<&str>, target_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_nym_request(command_handle, submitter_did, target_did, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_NYM request. Request to get information about a DID (NYM).
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_nym_request_timeout(submitter_did: Option<&str>, target_did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_nym_request(command_handle, submitter_did, target_did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_NYM request. Request to get information about a DID (NYM).
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_nym_request_async<F: 'static>(submitter_did: Option<&str>, target_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_nym_request(command_handle, submitter_did, target_did, cb)
    }

    fn _build_get_nym_request(command_handle: IndyHandle, submitter_did: Option<&str>, target_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let target_did = c_str!(target_did);

        ErrorCode::from(unsafe { ledger::indy_build_get_nym_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), target_did.as_ptr(), cb) })
    }

    /// Builds a GET_TXN request. Request to get any transaction by its seq_no.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the request submitter.
    /// * `ledger_type` - (Optional) type of the ledger the requested transaction belongs to:
    ///     DOMAIN - used default,
    ///     POOL,
    ///     CONFIG
    /// * `seq_no` - seq_no of transaction in ledger.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_txn_request(submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_txn_request(command_handle, submitter_did, ledger_type, seq_no, cb);
    
        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_TXN request. Request to get any transaction by its seq_no.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the request submitter.
    /// * `seq_no` - seq_no of transaction in ledger.
    /// * `ledger_type` - (Optional) type of the ledger the requested transaction belongs to:
    ///     DOMAIN - used default,
    ///     POOL,
    ///     CONFIG
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_txn_request_timeout(submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_txn_request(command_handle, submitter_did, ledger_type, seq_no, cb);
    
        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_TXN request. Request to get any transaction by its seq_no.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the request submitter.
    /// * `seq_no` - seq_no of transaction in ledger.
    /// * `ledger_type` - (Optional) type of the ledger the requested transaction belongs to:
    ///     DOMAIN - used default,
    ///     POOL,
    ///     CONFIG
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_txn_request_async<F: 'static>(submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_txn_request(command_handle, submitter_did, ledger_type, seq_no, cb)
    }

    fn _build_get_txn_request(command_handle: IndyHandle, submitter_did: Option<&str>, ledger_type: Option<&str>, seq_no: i32, cb: Option<ResponseStringCB>) ->  ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let ledger_type_str = opt_c_str!(ledger_type);

        ErrorCode::from(unsafe { ledger::indy_build_get_txn_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), opt_c_ptr!(ledger_type, ledger_type_str), seq_no, cb) })
    }

    /// Builds an ATTRIB request. Request to add attribute to a NYM record.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `hash` - (Optional) Hash of attribute data.
    /// * `raw` - (Optional) Json, where key is attribute name and value is attribute value.
    /// * `enc` - (Optional) Encrypted value attribute data.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_attrib_request(command_handle, submitter_did, target_did, hash, raw, enc, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds an ATTRIB request. Request to add attribute to a NYM record.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `hash` - (Optional) Hash of attribute data.
    /// * `raw` - (Optional) Json, where key is attribute name and value is attribute value.
    /// * `enc` - (Optional) Encrypted value attribute data.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_attrib_request_timeout(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_attrib_request(command_handle, submitter_did, target_did, hash, raw, enc, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds an ATTRIB request. Request to add attribute to a NYM record.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `hash` - (Optional) Hash of attribute data.
    /// * `raw` - (Optional) Json, where key is attribute name and value is attribute value.
    /// * `enc` - (Optional) Encrypted value attribute data.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_attrib_request_async<F: 'static>(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_attrib_request(command_handle, submitter_did, target_did, hash, raw, enc, cb)
    }

    fn _build_attrib_request(command_handle: IndyHandle, submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let target_did = c_str!(target_did);

        let hash_str = opt_c_str!(hash);
        let raw_str = opt_c_str!(raw);
        let enc_str = opt_c_str!(enc);

        ErrorCode::from(unsafe {
            ledger::indy_build_attrib_request(command_handle,
                                              submitter_did.as_ptr(),
                                              target_did.as_ptr(),
                                              opt_c_ptr!(hash, hash_str),
                                              opt_c_ptr!(raw, raw_str),
                                              opt_c_ptr!(enc, enc_str),
                                              cb)
        })
    }

    /// Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `raw` - (Optional) Requested attribute name.
    /// * `hash` - (Optional) Requested attribute hash.
    /// * `enc` - (Optional) Requested attribute encrypted value.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_attrib_request(submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_attrib_request(command_handle, submitter_did, target_did, raw, hash, enc, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `raw` - (Optional) Requested attribute name.
    /// * `hash` - (Optional) Requested attribute hash.
    /// * `enc` - (Optional) Requested attribute encrypted value.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_attrib_request_timeout(submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_attrib_request(command_handle, submitter_did, target_did, raw, hash, enc, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_ATTRIB request. Request to get information about an Attribute for the specified DID.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `target_did` - Target DID as base58-encoded string for 16 or 32 bit DID value.
    /// * `raw` - (Optional) Requested attribute name.
    /// * `hash` - (Optional) Requested attribute hash.
    /// * `enc` - (Optional) Requested attribute encrypted value.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_attrib_request_async<F: 'static>(submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_attrib_request(command_handle, submitter_did, target_did, raw, hash, enc, cb)
    }

    fn _build_get_attrib_request(command_handle: IndyHandle, submitter_did: Option<&str>, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let target_did = c_str!(target_did);

        let raw_str = opt_c_str!(raw);
        let hash_str = opt_c_str!(hash);
        let enc_str = opt_c_str!(enc);

        ErrorCode::from(unsafe {
            ledger::indy_build_get_attrib_request(command_handle,
                                                  opt_c_ptr!(submitter_did, submitter_did_str),
                                                  target_did.as_ptr(),
                                                  opt_c_ptr!(raw, raw_str),
                                                  opt_c_ptr!(hash, hash_str),
                                                  opt_c_ptr!(enc, enc_str),
                                                  cb)
        })
    }

    /// Builds a SCHEMA request. Request to add Credential's schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - Credential schema.
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string,
    ///     ver: Version of the Schema json
    /// }
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_schema_request(command_handle, submitter_did, data, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a SCHEMA request. Request to add Credential's schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - Credential schema.
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string,
    ///     ver: Version of the Schema json
    /// }
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_schema_request_timeout(submitter_did: &str, data: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_schema_request(command_handle, submitter_did, data, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a SCHEMA request. Request to add Credential's schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - Credential schema.
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string,
    ///     ver: Version of the Schema json
    /// }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_schema_request_async<F: 'static>(submitter_did: &str, data: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_schema_request(command_handle, submitter_did, data, cb)
    }

    fn _build_schema_request(command_handle: IndyHandle, submitter_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let data = c_str!(data);

        ErrorCode::from(unsafe { ledger::indy_build_schema_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb) })
    }

    /// Builds a GET_SCHEMA request. Request to get Credential's Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` - Schema ID in ledger
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_schema_request(submitter_did: Option<&str>, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_schema_request(command_handle, submitter_did, id, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_SCHEMA request. Request to get Credential's Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` - Schema ID in ledger
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_schema_request_timeout(submitter_did: Option<&str>, id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_schema_request(command_handle, submitter_did, id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_SCHEMA request. Request to get Credential's Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` - Schema ID in ledger
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_schema_request_async<F: 'static>(submitter_did: Option<&str>, id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_schema_request(command_handle, submitter_did, id, cb)
    }

    fn _build_get_schema_request(command_handle: IndyHandle, submitter_did: Option<&str>, id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let id = c_str!(id);

        ErrorCode::from(unsafe { ledger::indy_build_get_schema_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), id.as_ptr(), cb) })
    }

    /// Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_schema_response` - response of GET_SCHEMA request.
    ///
    /// # Returns
    /// Schema Id and Schema json.
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string
    ///     ver: Version of the Schema json
    /// }
    pub fn parse_get_schema_response(get_schema_response: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Ledger::_parse_get_schema_response(command_handle, get_schema_response, cb);

        ResultHandler::two(err, receiver)
    }

    /// Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_schema_response` - response of GET_SCHEMA request.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Schema Id and Schema json.
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string
    ///     ver: Version of the Schema json
    /// }
    pub fn parse_get_schema_response_timeout(get_schema_response: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Ledger::_parse_get_schema_response(command_handle, get_schema_response, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// Parse a GET_SCHEMA response to get Schema in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_schema_response` - response of GET_SCHEMA request.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_get_schema_response_async<F: 'static>(get_schema_response: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Ledger::_parse_get_schema_response(command_handle, get_schema_response, cb)
    }

    fn _parse_get_schema_response(command_handle: IndyHandle, get_schema_response: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let get_schema_response = c_str!(get_schema_response);

        ErrorCode::from(unsafe { ledger::indy_parse_get_schema_response(command_handle, get_schema_response.as_ptr(), cb) })
    }

    /// Builds an CRED_DEF request. Request to add a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - credential definition json
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the CredDef json
    /// }
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_cred_def_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_cred_def_request(command_handle, submitter_did, data, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds an CRED_DEF request. Request to add a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - credential definition json
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the CredDef json
    /// }
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_cred_def_request_timeout(submitter_did: &str, data: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_cred_def_request(command_handle, submitter_did, data, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds an CRED_DEF request. Request to add a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - credential definition json
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the CredDef json
    /// }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_cred_def_request_async<F: 'static>(submitter_did: &str, data: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_cred_def_request(command_handle, submitter_did, data, cb)
    }

    fn _build_cred_def_request(command_handle: IndyHandle, submitter_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let data = c_str!(data);

        ErrorCode::from(unsafe { ledger::indy_build_cred_def_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb) })
    }

    /// Builds a GET_CRED_DEF request. Request to get a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` - Credential Definition ID in ledger.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_cred_def_request(submitter_did: Option<&str>, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_cred_def_request(command_handle, submitter_did, id, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_CRED_DEF request. Request to get a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` - Credential Definition ID in ledger.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_cred_def_request_timeout(submitter_did: Option<&str>, id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_cred_def_request(command_handle, submitter_did, id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_CRED_DEF request. Request to get a Credential Definition (in particular, public key),
    /// that Issuer creates for a particular Credential Schema.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` - Credential Definition ID in ledger.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_cred_def_request_async<F: 'static>(submitter_did: Option<&str>, id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_cred_def_request(command_handle, submitter_did, id, cb)
    }

    fn _build_get_cred_def_request(command_handle: IndyHandle, submitter_did: Option<&str>, id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let id = c_str!(id);

        ErrorCode::from(unsafe { ledger::indy_build_get_cred_def_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), id.as_ptr(), cb) })
    }

    /// Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_cred_def_response` - response of GET_CRED_DEF request.
    ///
    /// # Returns
    /// Credential Definition Id and Credential Definition json.
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the Credential Definition json
    /// }
    pub fn parse_get_cred_def_response(get_cred_def_response: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Ledger::_parse_get_cred_def_response(command_handle, get_cred_def_response, cb);

        ResultHandler::two(err, receiver)
    }

    /// Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_cred_def_response` - response of GET_CRED_DEF request.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Credential Definition Id and Credential Definition json.
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the Credential Definition json
    /// }
    pub fn parse_get_cred_def_response_timeout(get_cred_def_response: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Ledger::_parse_get_cred_def_response(command_handle, get_cred_def_response, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// Parse a GET_CRED_DEF response to get Credential Definition in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_cred_def_response` - response of GET_CRED_DEF request.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_get_cred_def_response_async<F: 'static>(get_cred_def_response: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Ledger::_parse_get_cred_def_response(command_handle, get_cred_def_response, cb)
    }

    fn _parse_get_cred_def_response(command_handle: IndyHandle, get_cred_def_response: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let get_cred_def_response = c_str!(get_cred_def_response);

        ErrorCode::from(unsafe { ledger::indy_parse_get_cred_def_response(command_handle, get_cred_def_response.as_ptr(), cb) })
    }

    /// Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target Node's DID.  It differs from submitter_did field.
    /// * `data` - Data associated with the Node: {
    ///     alias: string - Node's alias
    ///     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
    ///     client_ip: string - (Optional) Node's client listener IP address.
    ///     client_port: string - (Optional) Node's client listener port.
    ///     node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
    ///     node_port: string - (Optional) The port other Nodes use to communicate with this Node.
    ///     services: array<string> - (Optional) The service of the Node. VALIDATOR is the only supported one now.
    /// }
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_node_request(command_handle, submitter_did, target_did, data, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target Node's DID.  It differs from submitter_did field.
    /// * `data` - Data associated with the Node: {
    ///     alias: string - Node's alias
    ///     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
    ///     client_ip: string - (Optional) Node's client listener IP address.
    ///     client_port: string - (Optional) Node's client listener port.
    ///     node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
    ///     node_port: string - (Optional) The port other Nodes use to communicate with this Node.
    ///     services: array<string> - (Optional) The service of the Node. VALIDATOR is the only supported one now.
    /// }
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_node_request_timeout(submitter_did: &str, target_did: &str, data: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_node_request(command_handle, submitter_did, target_did, data, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a NODE request. Request to add a new node to the pool, or updates existing in the pool.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `target_did` - Target Node's DID.  It differs from submitter_did field.
    /// * `data` - Data associated with the Node: {
    ///     alias: string - Node's alias
    ///     blskey: string - (Optional) BLS multi-signature key as base58-encoded string.
    ///     client_ip: string - (Optional) Node's client listener IP address.
    ///     client_port: string - (Optional) Node's client listener port.
    ///     node_ip: string - (Optional) The IP address other Nodes use to communicate with this Node.
    ///     node_port: string - (Optional) The port other Nodes use to communicate with this Node.
    ///     services: array<string> - (Optional) The service of the Node. VALIDATOR is the only supported one now.
    /// }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_node_request_async<F: 'static>(submitter_did: &str, target_did: &str, data: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_node_request(command_handle, submitter_did, target_did, data, cb)
    }

    fn _build_node_request(command_handle: IndyHandle, submitter_did: &str, target_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let target_did = c_str!(target_did);
        let data = c_str!(data);

        ErrorCode::from(unsafe { ledger::indy_build_node_request(command_handle, submitter_did.as_ptr(), target_did.as_ptr(), data.as_ptr(), cb) })
    }

    /// Builds a GET_VALIDATOR_INFO request.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_validator_info_request(submitter_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_validator_info_request(command_handle, submitter_did, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_VALIDATOR_INFO request.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_validator_info_request_timeout(submitter_did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_validator_info_request(command_handle, submitter_did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_VALIDATOR_INFO request.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_validator_info_request_async<F: 'static>(submitter_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_validator_info_request(command_handle, submitter_did, cb)
    }

    fn _build_get_validator_info_request(command_handle: IndyHandle, submitter_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);

        ErrorCode::from(unsafe {
          ledger::indy_build_get_validator_info_request(command_handle, submitter_did.as_ptr(), cb)
        })
    }

    /// Builds a POOL_CONFIG request. Request to change Pool's configuration.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `writes` - Whether any write requests can be processed by the pool
    ///         (if false, then pool goes to read-only state). True by default.
    /// * `force` - Whether we should apply transaction (for example, move pool to read-only state)
    ///        without waiting for consensus of this transaction.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_pool_config_request(submitter_did: &str, writes: bool, force: bool) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_pool_config_request(command_handle, submitter_did, writes, force, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a POOL_CONFIG request. Request to change Pool's configuration.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `writes` - Whether any write requests can be processed by the pool
    ///         (if false, then pool goes to read-only state). True by default.
    /// * `force` - Whether we should apply transaction (for example, move pool to read-only state)
    ///        without waiting for consensus of this transaction.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_pool_config_request_timeout(submitter_did: &str, writes: bool, force: bool, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_pool_config_request(command_handle, submitter_did, writes, force, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a POOL_CONFIG request. Request to change Pool's configuration.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `writes` - Whether any write requests can be processed by the pool
    ///         (if false, then pool goes to read-only state). True by default.
    /// * `force` - Whether we should apply transaction (for example, move pool to read-only state)
    ///        without waiting for consensus of this transaction.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_pool_config_request_async<F: 'static>(submitter_did: &str, writes: bool, force: bool, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_pool_config_request(command_handle, submitter_did, writes, force, cb)
    }

    fn _build_pool_config_request(command_handle: IndyHandle, submitter_did: &str, writes: bool, force: bool, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);

        ErrorCode::from(unsafe { ledger::indy_build_pool_config_request(command_handle, submitter_did.as_ptr(), writes, force, cb) })
    }

    /// Builds a POOL_RESTART request.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `action`- 
    /// * `datetime`- 
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_pool_restart_request(submitter_did: &str, action: &str, datetime: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_pool_restart_request(command_handle, submitter_did, action, datetime, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a POOL_RESTART request.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `action`- 
    /// * `datetime`- 
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_pool_restart_request_timeout(submitter_did: &str, action: &str, datetime: Option<&str>, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_pool_restart_request(command_handle, submitter_did, action, datetime, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a POOL_RESTART request.
    ///
    /// # Arguments
    /// * `submitter_did` - Id of Identity stored in secured Wallet.
    /// * `action`- 
    /// * `datetime`- 
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_pool_restart_request_async<F: 'static>(submitter_did: &str, action: &str, datetime: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_pool_restart_request(command_handle, submitter_did, action, datetime, cb)
    }

    fn _build_pool_restart_request(command_handle: IndyHandle, submitter_did: &str, action: &str, datetime: Option<&str>, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let action = c_str!(action);
        let datetime = opt_c_str!(datetime);

        ErrorCode::from(unsafe {
            ledger::indy_build_pool_restart_request(command_handle,
                                                    submitter_did.as_ptr(),
                                                    action.as_ptr(),
                                                    datetime.as_ptr(),
                                                    cb)
        })
    }

    /// Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
    /// It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `name` - Human-readable name for the upgrade.
    /// * `version` - The version of indy-node package we perform upgrade to.
    ///          Must be greater than existing one (or equal if reinstall flag is True).
    /// * `action` - Either start or cancel.
    /// * `sha256` - sha256 hash of the package.
    /// * `upgrade_timeout` - (Optional) Limits upgrade time on each Node.
    /// * `schedule` - (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
    /// * `justification` - (Optional) justification string for this particular Upgrade.
    /// * `reinstall` - Whether it's allowed to re-install the same version. False by default.
    /// * `force` - Whether we should apply transaction (schedule Upgrade) without waiting
    ///        for consensus of this transaction.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_pool_upgrade_request(submitter_did: &str,
                                      name: &str,
                                      version: &str,
                                      action: &str,
                                      sha256: &str,
                                      upgrade_timeout: Option<u32>,
                                      schedule: Option<&str>,
                                      justification: Option<&str>,
                                      reinstall: bool,
                                      force: bool,
                                      package: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_pool_upgrade_request(command_handle, submitter_did, name, version, action, sha256, upgrade_timeout, schedule, justification, reinstall, force, package, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
    /// It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `name` - Human-readable name for the upgrade.
    /// * `version` - The version of indy-node package we perform upgrade to.
    ///          Must be greater than existing one (or equal if reinstall flag is True).
    /// * `action` - Either start or cancel.
    /// * `sha256` - sha256 hash of the package.
    /// * `upgrade_timeout` - (Optional) Limits upgrade time on each Node.
    /// * `schedule` - (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
    /// * `justification` - (Optional) justification string for this particular Upgrade.
    /// * `reinstall` - Whether it's allowed to re-install the same version. False by default.
    /// * `force` - Whether we should apply transaction (schedule Upgrade) without waiting
    ///        for consensus of this transaction.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_pool_upgrade_request_timeout(submitter_did: &str,
                                              name: &str,
                                              version: &str,
                                              action: &str,
                                              sha256: &str,
                                              upgrade_timeout: Option<u32>,
                                              schedule: Option<&str>,
                                              justification: Option<&str>,
                                              reinstall: bool,
                                              force: bool,
                                              package: Option<&str>,
                                              timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_pool_upgrade_request(command_handle, submitter_did, name, version, action, sha256, upgrade_timeout, schedule, justification, reinstall, force, package, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a POOL_UPGRADE request. Request to upgrade the Pool (sent by Trustee).
    /// It upgrades the specified Nodes (either all nodes in the Pool, or some specific ones).
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `name` - Human-readable name for the upgrade.
    /// * `version` - The version of indy-node package we perform upgrade to.
    ///          Must be greater than existing one (or equal if reinstall flag is True).
    /// * `action` - Either start or cancel.
    /// * `sha256` - sha256 hash of the package.
    /// * `upgrade_timeout` - (Optional) Limits upgrade time on each Node.
    /// * `schedule` - (Optional) Schedule of when to perform upgrade on each node. Map Node DIDs to upgrade time.
    /// * `justification` - (Optional) justification string for this particular Upgrade.
    /// * `reinstall` - Whether it's allowed to re-install the same version. False by default.
    /// * `force` - Whether we should apply transaction (schedule Upgrade) without waiting
    ///        for consensus of this transaction.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_pool_upgrade_request_async<F: 'static>(submitter_did: &str,
                                                        name: &str,
                                                        version: &str,
                                                        action: &str,
                                                        sha256: &str,
                                                        upgrade_timeout: Option<u32>,
                                                        schedule: Option<&str>,
                                                        justification: Option<&str>,
                                                        reinstall: bool,
                                                        force: bool,
                                                        package: Option<&str>,
                                                        closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_pool_upgrade_request(command_handle, submitter_did, name, version, action, sha256, upgrade_timeout, schedule, justification, reinstall, force, package, cb)
    }

    fn _build_pool_upgrade_request(command_handle: IndyHandle,
                                   submitter_did: &str,
                                   name: &str,
                                   version: &str,
                                   action: &str,
                                   sha256: &str,
                                   upgrade_timeout: Option<u32>,
                                   schedule: Option<&str>,
                                   justification: Option<&str>,
                                   reinstall: bool,
                                   force: bool,
                                   package: Option<&str>,
                                   cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let name = c_str!(name);
        let version = c_str!(version);
        let action = c_str!(action);
        let sha256 = c_str!(sha256);
        let upgrade_timeout = upgrade_timeout.map(|t| t as i32).unwrap_or(-1);

        let schedule_str = opt_c_str!(schedule);
        let justification_str = opt_c_str!(justification);
        let package_str = opt_c_str!(package);

        ErrorCode::from(unsafe {
            ledger::indy_build_pool_upgrade_request(command_handle,
                                                    submitter_did.as_ptr(),
                                                    name.as_ptr(),
                                                    version.as_ptr(),
                                                    action.as_ptr(),
                                                    sha256.as_ptr(),
                                                    upgrade_timeout,
                                                    opt_c_ptr!(schedule, schedule_str),
                                                    opt_c_ptr!(justification, justification_str),
                                                    reinstall,
                                                    force,
                                                    opt_c_ptr!(package, package_str),
                                                    cb)
        })
    }

    /// Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
    /// to an exists credential definition.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - Revocation Registry data:
    ///     {
    ///         "id": string - ID of the Revocation Registry,
    ///         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    ///         "tag": string - Unique descriptive ID of the Registry,
    ///         "credDefId": string - ID of the corresponding CredentialDefinition,
    ///         "value": Registry-specific data {
    ///             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
    ///             "maxCredNum": number - Maximum number of credentials the Registry can serve.
    ///             "tailsHash": string - Hash of tails.
    ///             "tailsLocation": string - Location of tails file.
    ///             "publicKeys": <public_keys> - Registry's public key.
    ///         },
    ///         "ver": string - version of revocation registry definition json.
    ///     }
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_revoc_reg_def_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_revoc_reg_def_request(command_handle, submitter_did, data, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
    /// to an exists credential definition.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - Revocation Registry data:
    ///     {
    ///         "id": string - ID of the Revocation Registry,
    ///         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    ///         "tag": string - Unique descriptive ID of the Registry,
    ///         "credDefId": string - ID of the corresponding CredentialDefinition,
    ///         "value": Registry-specific data {
    ///             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
    ///             "maxCredNum": number - Maximum number of credentials the Registry can serve.
    ///             "tailsHash": string - Hash of tails.
    ///             "tailsLocation": string - Location of tails file.
    ///             "publicKeys": <public_keys> - Registry's public key.
    ///         },
    ///         "ver": string - version of revocation registry definition json.
    ///     }
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_revoc_reg_def_request_timeout(submitter_did: &str, data: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_revoc_reg_def_request(command_handle, submitter_did, data, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a REVOC_REG_DEF request. Request to add the definition of revocation registry
    /// to an exists credential definition.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `data` - Revocation Registry data:
    ///     {
    ///         "id": string - ID of the Revocation Registry,
    ///         "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    ///         "tag": string - Unique descriptive ID of the Registry,
    ///         "credDefId": string - ID of the corresponding CredentialDefinition,
    ///         "value": Registry-specific data {
    ///             "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
    ///             "maxCredNum": number - Maximum number of credentials the Registry can serve.
    ///             "tailsHash": string - Hash of tails.
    ///             "tailsLocation": string - Location of tails file.
    ///             "publicKeys": <public_keys> - Registry's public key.
    ///         },
    ///         "ver": string - version of revocation registry definition json.
    ///     }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_revoc_reg_def_request_async<F: 'static>(submitter_did: &str, data: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_revoc_reg_def_request(command_handle, submitter_did, data, cb)
    }

    fn _build_revoc_reg_def_request(command_handle: IndyHandle, submitter_did: &str, data: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let data = c_str!(data);

        ErrorCode::from(unsafe { ledger::indy_build_revoc_reg_def_request(command_handle, submitter_did.as_ptr(), data.as_ptr(), cb) })
    }

    /// Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
    /// that Issuer creates for a particular Credential Definition.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` -  ID of Revocation Registry Definition in ledger.
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_revoc_reg_def_request(submitter_did: Option<&str>, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_revoc_reg_def_request(command_handle, submitter_did, id, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
    /// that Issuer creates for a particular Credential Definition.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` -  ID of Revocation Registry Definition in ledger.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_revoc_reg_def_request_timeout(submitter_did: Option<&str>, id: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_revoc_reg_def_request(command_handle, submitter_did, id, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_REVOC_REG_DEF request. Request to get a revocation registry definition,
    /// that Issuer creates for a particular Credential Definition.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `id` -  ID of Revocation Registry Definition in ledger.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_revoc_reg_def_request_async<F: 'static>(submitter_did: Option<&str>, id: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_revoc_reg_def_request(command_handle, submitter_did, id, cb)
    }

    fn _build_get_revoc_reg_def_request(command_handle: IndyHandle, submitter_did: Option<&str>, id: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let id = c_str!(id);

        ErrorCode::from(unsafe { ledger::indy_build_get_revoc_reg_def_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), id.as_ptr(), cb) })
    }

    /// Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format
    /// compatible with Anoncreds API.
    ///
    /// #Params
    /// * `get_revoc_reg_def_response` - response of GET_REVOC_REG_DEF request.
    ///
    /// # Returns
    /// Revocation Registry Definition Id and Revocation Registry Definition json.
    /// {
    ///     "id": string - ID of the Revocation Registry,
    ///     "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    ///     "tag": string - Unique descriptive ID of the Registry,
    ///     "credDefId": string - ID of the corresponding CredentialDefinition,
    ///     "value": Registry-specific data {
    ///         "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
    ///         "maxCredNum": number - Maximum number of credentials the Registry can serve.
    ///         "tailsHash": string - Hash of tails.
    ///         "tailsLocation": string - Location of tails file.
    ///         "publicKeys": <public_keys> - Registry's public key.
    ///     },
    ///     "ver": string - version of revocation registry definition json.
    /// }
    pub fn parse_get_revoc_reg_def_response(get_revoc_reg_def_response: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Ledger::_parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response, cb);

        ResultHandler::two(err, receiver)
    }

    /// Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format
    /// compatible with Anoncreds API.
    ///
    /// #Params
    /// * `get_revoc_reg_def_response` - response of GET_REVOC_REG_DEF request.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Revocation Registry Definition Id and Revocation Registry Definition json.
    /// {
    ///     "id": string - ID of the Revocation Registry,
    ///     "revocDefType": string - Revocation Registry type (only CL_ACCUM is supported for now),
    ///     "tag": string - Unique descriptive ID of the Registry,
    ///     "credDefId": string - ID of the corresponding CredentialDefinition,
    ///     "value": Registry-specific data {
    ///         "issuanceType": string - Type of Issuance(ISSUANCE_BY_DEFAULT or ISSUANCE_ON_DEMAND),
    ///         "maxCredNum": number - Maximum number of credentials the Registry can serve.
    ///         "tailsHash": string - Hash of tails.
    ///         "tailsLocation": string - Location of tails file.
    ///         "publicKeys": <public_keys> - Registry's public key.
    ///     },
    ///     "ver": string - version of revocation registry definition json.
    /// }
    pub fn parse_get_revoc_reg_def_response_timeout(get_revoc_reg_def_response: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Ledger::_parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// Parse a GET_REVOC_REG_DEF response to get Revocation Registry Definition in the format
    /// compatible with Anoncreds API.
    ///
    /// #Params
    /// * `get_revoc_reg_def_response` - response of GET_REVOC_REG_DEF request.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_get_revoc_reg_def_response_async<F: 'static>(get_revoc_reg_def_response: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Ledger::_parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response, cb)
    }

    fn _parse_get_revoc_reg_def_response(command_handle: IndyHandle, get_revoc_reg_def_response: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let get_revoc_reg_def_response = c_str!(get_revoc_reg_def_response);

        ErrorCode::from(unsafe { ledger::indy_parse_get_revoc_reg_def_response(command_handle, get_revoc_reg_def_response.as_ptr(), cb) })
    }

    /// Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
    /// the new accumulator value and issued/revoked indices.
    /// This is just a delta of indices, not the whole list.
    /// So, it can be sent each time a new credential is issued/revoked.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `revoc_reg_def_id` - ID of the corresponding RevocRegDef.
    /// * `rev_def_type` - Revocation Registry type (only CL_ACCUM is supported for now).
    /// * `value` - Registry-specific data: {
    ///     value: {
    ///         prevAccum: string - previous accumulator value.
    ///         accum: string - current accumulator value.
    ///         issued: array<number> - an array of issued indices.
    ///         revoked: array<number> an array of revoked indices.
    ///     },
    ///     ver: string - version revocation registry entry json
    ///
    /// }
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_revoc_reg_entry_request(submitter_did: &str, revoc_reg_def_id: &str, rev_def_type: &str, value: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_revoc_reg_entry_request(command_handle, submitter_did, revoc_reg_def_id, rev_def_type, value, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
    /// the new accumulator value and issued/revoked indices.
    /// This is just a delta of indices, not the whole list.
    /// So, it can be sent each time a new credential is issued/revoked.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `revoc_reg_def_id` - ID of the corresponding RevocRegDef.
    /// * `rev_def_type` - Revocation Registry type (only CL_ACCUM is supported for now).
    /// * `value` - Registry-specific data: {
    ///     value: {
    ///         prevAccum: string - previous accumulator value.
    ///         accum: string - current accumulator value.
    ///         issued: array<number> - an array of issued indices.
    ///         revoked: array<number> an array of revoked indices.
    ///     },
    ///     ver: string - version revocation registry entry json
    ///
    /// }
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_revoc_reg_entry_request_timeout(submitter_did: &str, revoc_reg_def_id: &str, rev_def_type: &str, value: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_revoc_reg_entry_request(command_handle, submitter_did, revoc_reg_def_id, rev_def_type, value, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a REVOC_REG_ENTRY request.  Request to add the RevocReg entry containing
    /// the new accumulator value and issued/revoked indices.
    /// This is just a delta of indices, not the whole list.
    /// So, it can be sent each time a new credential is issued/revoked.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the submitter stored in secured Wallet.
    /// * `revoc_reg_def_id` - ID of the corresponding RevocRegDef.
    /// * `rev_def_type` - Revocation Registry type (only CL_ACCUM is supported for now).
    /// * `value` - Registry-specific data: {
    ///     value: {
    ///         prevAccum: string - previous accumulator value.
    ///         accum: string - current accumulator value.
    ///         issued: array<number> - an array of issued indices.
    ///         revoked: array<number> an array of revoked indices.
    ///     },
    ///     ver: string - version revocation registry entry json
    ///
    /// }
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_revoc_reg_entry_request_async<F: 'static>(submitter_did: &str, revoc_reg_def_id: &str, rev_def_type: &str, value: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_revoc_reg_entry_request(command_handle, submitter_did, revoc_reg_def_id, rev_def_type, value, cb)
    }

    fn _build_revoc_reg_entry_request(command_handle: IndyHandle, submitter_did: &str, revoc_reg_def_id: &str, rev_def_type: &str, value: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did = c_str!(submitter_did);
        let revoc_reg_def_id = c_str!(revoc_reg_def_id);
        let rev_def_type = c_str!(rev_def_type);
        let value = c_str!(value);

        ErrorCode::from(unsafe { ledger::indy_build_revoc_reg_entry_request(command_handle, submitter_did.as_ptr(), revoc_reg_def_id.as_ptr(), rev_def_type.as_ptr(), value.as_ptr(), cb) })
    }

    /// Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
    /// by ID. The state is defined by the given timestamp.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
    /// * `timestamp` - Requested time represented as a total number of seconds from Unix Epoch
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_revoc_reg_request(submitter_did: Option<&str>, revoc_reg_def_id: &str, timestamp: i64) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_revoc_reg_request(command_handle, submitter_did, revoc_reg_def_id, timestamp, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
    /// by ID. The state is defined by the given timestamp.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
    /// * `timestamp` - Requested time represented as a total number of seconds from Unix Epoch
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_revoc_reg_request_timeout(submitter_did: Option<&str>, revoc_reg_def_id: &str, timestamp: i64, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_revoc_reg_request(command_handle, submitter_did, revoc_reg_def_id, timestamp, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_REVOC_REG request. Request to get the accumulated state of the Revocation Registry
    /// by ID. The state is defined by the given timestamp.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
    /// * `timestamp` - Requested time represented as a total number of seconds from Unix Epoch
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_revoc_reg_request_async<F: 'static>(submitter_did: Option<&str>, revoc_reg_def_id: &str, timestamp: i64, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_revoc_reg_request(command_handle, submitter_did, revoc_reg_def_id, timestamp, cb)
    }

    fn _build_get_revoc_reg_request(command_handle: IndyHandle, submitter_did: Option<&str>, revoc_reg_def_id: &str, timestamp: i64, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let revoc_reg_def_id = c_str!(revoc_reg_def_id);

        ErrorCode::from(unsafe { ledger::indy_build_get_revoc_reg_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), revoc_reg_def_id.as_ptr(), timestamp, cb) })
    }

    /// Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_revoc_reg_response` - response of GET_REVOC_REG request.
    ///
    /// # Returns
    /// Revocation Registry Definition Id, Revocation Registry json and Timestamp.
    /// {
    ///     "value": Registry-specific data {
    ///         "accum": string - current accumulator value.
    ///     },
    ///     "ver": string - version revocation registry json
    /// }
    pub fn parse_get_revoc_reg_response(get_revoc_reg_response: &str) -> Result<(String, String, u64), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_u64();

        let err = Ledger::_parse_get_revoc_reg_response(command_handle, get_revoc_reg_response, cb);

        ResultHandler::three(err, receiver)
    }

    /// Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_revoc_reg_response` - response of GET_REVOC_REG request.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Revocation Registry Definition Id, Revocation Registry json and Timestamp.
    /// {
    ///     "value": Registry-specific data {
    ///         "accum": string - current accumulator value.
    ///     },
    ///     "ver": string - version revocation registry json
    /// }
    pub fn parse_get_revoc_reg_response_timeout(get_revoc_reg_response: &str, timeout: Duration) -> Result<(String, String, u64), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_u64();

        let err = Ledger::_parse_get_revoc_reg_response(command_handle, get_revoc_reg_response, cb);

        ResultHandler::three_timeout(err, receiver, timeout)
    }

    /// Parse a GET_REVOC_REG response to get Revocation Registry in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_revoc_reg_response` - response of GET_REVOC_REG request.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_get_revoc_reg_response_async<F: 'static>(get_revoc_reg_response: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String, u64) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string_u64(Box::new(closure));

        Ledger::_parse_get_revoc_reg_response(command_handle, get_revoc_reg_response, cb)
    }

    fn _parse_get_revoc_reg_response(command_handle: IndyHandle, get_revoc_reg_response: &str, cb: Option<ResponseStringStringU64CB>) -> ErrorCode {
        let get_revoc_reg_response = c_str!(get_revoc_reg_response);

        ErrorCode::from(unsafe { ledger::indy_parse_get_revoc_reg_response(command_handle,get_revoc_reg_response.as_ptr(), cb) })
    }

    /// Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
    /// The Delta is defined by from and to timestamp fields.
    /// If from is not specified, then the whole state till to will be returned.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
    /// * `from` - Requested time represented as a total number of seconds from Unix Epoch
    /// * `to` - Requested time represented as a total number of seconds from Unix Epoch
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_revoc_reg_delta_request(submitter_did: Option<&str>, revoc_reg_def_id: &str, from: i64, to: i64) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_revoc_reg_delta_request(command_handle, submitter_did, revoc_reg_def_id, from, to, cb);

        ResultHandler::one(err, receiver)
    }

    /// Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
    /// The Delta is defined by from and to timestamp fields.
    /// If from is not specified, then the whole state till to will be returned.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
    /// * `from` - Requested time represented as a total number of seconds from Unix Epoch
    /// * `to` - Requested time represented as a total number of seconds from Unix Epoch
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Request result as json.
    pub fn build_get_revoc_reg_delta_request_timeout(submitter_did: Option<&str>, revoc_reg_def_id: &str, from: i64, to: i64, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Ledger::_build_get_revoc_reg_delta_request(command_handle, submitter_did, revoc_reg_def_id, from, to, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Builds a GET_REVOC_REG_DELTA request. Request to get the delta of the accumulated state of the Revocation Registry.
    /// The Delta is defined by from and to timestamp fields.
    /// If from is not specified, then the whole state till to will be returned.
    ///
    /// # Arguments
    /// * `submitter_did` - DID of the read request sender.
    /// * `revoc_reg_def_id` -  ID of the corresponding Revocation Registry Definition in ledger.
    /// * `from` - Requested time represented as a total number of seconds from Unix Epoch
    /// * `to` - Requested time represented as a total number of seconds from Unix Epoch
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn build_get_revoc_reg_delta_request_async<F: 'static>(submitter_did: Option<&str>, revoc_reg_def_id: &str, from: i64, to: i64, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Ledger::_build_get_revoc_reg_delta_request(command_handle, submitter_did, revoc_reg_def_id, from, to, cb)
    }

    fn _build_get_revoc_reg_delta_request(command_handle: IndyHandle, submitter_did: Option<&str>, revoc_reg_def_id: &str, from: i64, to: i64, cb: Option<ResponseStringCB>) -> ErrorCode {
        let submitter_did_str = opt_c_str!(submitter_did);
        let revoc_reg_def_id = c_str!(revoc_reg_def_id);

        ErrorCode::from(unsafe { ledger::indy_build_get_revoc_reg_delta_request(command_handle, opt_c_ptr!(submitter_did, submitter_did_str), revoc_reg_def_id.as_ptr(), from, to, cb) })
    }

    /// Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_revoc_reg_response` - response of GET_REVOC_REG_DELTA request.
    ///
    /// # Returns
    /// Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
    /// {
    ///     "value": Registry-specific data {
    ///         prevAccum: string - previous accumulator value.
    ///         accum: string - current accumulator value.
    ///         issued: array<number> - an array of issued indices.
    ///         revoked: array<number> an array of revoked indices.
    ///     },
    ///     "ver": string - version revocation registry delta json
    /// }
    pub fn parse_get_revoc_reg_delta_response(get_revoc_reg_delta_response: &str) -> Result<(String, String, u64), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_u64();

        let err = Ledger::_parse_get_revoc_reg_delta_response(command_handle, get_revoc_reg_delta_response, cb);

        ResultHandler::three(err, receiver)
    }

    /// Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_revoc_reg_response` - response of GET_REVOC_REG_DELTA request.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Revocation Registry Definition Id, Revocation Registry Delta json and Timestamp.
    /// {
    ///     "value": Registry-specific data {
    ///         prevAccum: string - previous accumulator value.
    ///         accum: string - current accumulator value.
    ///         issued: array<number> - an array of issued indices.
    ///         revoked: array<number> an array of revoked indices.
    ///     },
    ///     "ver": string - version revocation registry delta json
    /// }
    pub fn parse_get_revoc_reg_delta_response_timeout(get_revoc_reg_delta_response: &str, timeout: Duration) -> Result<(String, String, u64), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string_u64();

        let err = Ledger::_parse_get_revoc_reg_delta_response(command_handle, get_revoc_reg_delta_response, cb);

        ResultHandler::three_timeout(err, receiver, timeout)
    }

    /// Parse a GET_REVOC_REG_DELTA response to get Revocation Registry Delta in the format compatible with Anoncreds API.
    ///
    /// # Arguments
    /// * `get_revoc_reg_response` - response of GET_REVOC_REG_DELTA request.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn parse_get_revoc_reg_delta_response_async<F: 'static>(get_revoc_reg_delta_response: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String, u64) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string_u64(Box::new(closure));

        Ledger::_parse_get_revoc_reg_delta_response(command_handle, get_revoc_reg_delta_response, cb)
    }

    fn _parse_get_revoc_reg_delta_response(command_handle: IndyHandle, get_revoc_reg_delta_response: &str, cb: Option<ResponseStringStringU64CB>) -> ErrorCode {
        let get_revoc_reg_delta_response = c_str!(get_revoc_reg_delta_response);

        ErrorCode::from(unsafe { ledger::indy_parse_get_revoc_reg_delta_response(command_handle,get_revoc_reg_delta_response.as_ptr(), cb) })
    }

    /// Register callbacks (see type description for `CustomTransactionParser` and `CustomFree`
    ///
    /// # Arguments
    /// * `txn_type` - type of transaction to apply `parse` callback.
    /// * `parse` - required callback to parse reply for state proof.
    /// * `free` - required callback to deallocate memory.
    ///
    /// # Returns
    /// Status of callbacks registration.
    pub fn register_transaction_parser_for_sp(txn_type: &str, parser: Option<ledger::CustomTransactionParser>, free: Option<ledger::CustomFree>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Ledger::_register_transaction_parser_for_sp(command_handle, txn_type, parser, free, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Register callbacks (see type description for `CustomTransactionParser` and `CustomFree`
    ///
    /// # Arguments
    /// * `txn_type` - type of transaction to apply `parse` callback.
    /// * `parse` - required callback to parse reply for state proof.
    /// * `free` - required callback to deallocate memory.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// Status of callbacks registration.
    pub fn register_transaction_parser_for_sp_timeout(txn_type: &str, parser: Option<ledger::CustomTransactionParser>, free: Option<ledger::CustomFree>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Ledger::_register_transaction_parser_for_sp(command_handle, txn_type, parser, free, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Register callbacks (see type description for `CustomTransactionParser` and `CustomFree`
    ///
    /// # Arguments
    /// * `txn_type` - type of transaction to apply `parse` callback.
    /// * `parse` - required callback to parse reply for state proof.
    /// * `free` - required callback to deallocate memory.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn register_transaction_parser_for_sp_async<F: 'static>(txn_type: &str, parser: Option<ledger::CustomTransactionParser>, free: Option<ledger::CustomFree>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Ledger::_register_transaction_parser_for_sp(command_handle, txn_type, parser, free, cb)
    }

    fn _register_transaction_parser_for_sp(command_handle: IndyHandle, txn_type: &str, parser: Option<ledger::CustomTransactionParser>, free: Option<ledger::CustomFree>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let txn_type = c_str!(txn_type);

        ErrorCode::from(unsafe {
          ledger::indy_register_transaction_parser_for_sp(command_handle, txn_type.as_ptr(), parser, free, cb)
        })
    }
}


