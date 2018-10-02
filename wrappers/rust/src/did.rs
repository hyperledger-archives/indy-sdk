use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;

use native::did;
use native::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseStringStringCB};

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

pub struct Did {}

impl Did {
    /// Creates keys (signing and encryption keys) for a new
    /// DID (owned by the caller of the library).
    /// Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
    /// Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
    /// and encrypt transactions.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `did_json` - Identity information as json.
    ///
    ///  # Examples
    /// `did_json`
    /// {
    ///     "did": string, (optional;
    ///             if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
    ///             if not provided and cid is true then the full verkey will be used as a new DID;
    ///             if provided, then keys will be replaced - key rotation use case)
    ///     "seed": string, (optional; if not provide then a random one will be created)
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    ///     "cid": bool, (optional; if not set then false is used;)
    /// }
    ///
    /// # Returns
    ///   * `did` - DID generated and stored in the wallet
    ///   * `verkey` - The DIDs verification key
    pub fn new(wallet_handle: IndyHandle, did_json: &str) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Did::_new(command_handle, wallet_handle, did_json, cb);

        ResultHandler::two(err, receiver)
    }

    /// Creates keys (signing and encryption keys) for a new
    /// DID (owned by the caller of the library).
    /// Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
    /// Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
    /// and encrypt transactions.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `did_json` - Identity information as json.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    ///  # Examples
    /// `did_json`
    /// {
    ///     "did": string, (optional;
    ///             if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
    ///             if not provided and cid is true then the full verkey will be used as a new DID;
    ///             if provided, then keys will be replaced - key rotation use case)
    ///     "seed": string, (optional; if not provide then a random one will be created)
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    ///     "cid": bool, (optional; if not set then false is used;)
    /// }
    ///
    /// # Returns
    ///   * `did` - DID generated and stored in the wallet
    ///   * `verkey` - The DIDs verification key
    pub fn new_timeout(wallet_handle: IndyHandle, did_json: &str, timeout: Duration) -> Result<(String, String), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_string();

        let err = Did::_new(command_handle, wallet_handle, did_json, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// Creates keys (signing and encryption keys) for a new
    /// DID (owned by the caller of the library).
    /// Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
    /// Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
    /// and encrypt transactions.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `did_json` - Identity information as json.
    /// * `closure` - the closure that is called when finished
    ///
    ///  # Examples
    /// `did_json`
    /// {
    ///     "did": string, (optional;
    ///             if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
    ///             if not provided and cid is true then the full verkey will be used as a new DID;
    ///             if provided, then keys will be replaced - key rotation use case)
    ///     "seed": string, (optional; if not provide then a random one will be created)/**/
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    ///     "cid": bool, (optional; if not set then false is used;)
    /// }
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn new_async<F: 'static>(wallet_handle: IndyHandle, did_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_string(Box::new(closure));

        Did::_new(command_handle, wallet_handle, did_json, cb)
    }

    fn _new(command_handle: IndyHandle, wallet_handle: IndyHandle, did_json: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let did_json = c_str!(did_json);

        ErrorCode::from(unsafe { did::indy_create_and_store_my_did(command_handle, wallet_handle, did_json.as_ptr(), cb) })
    }

    /// Generated temporary keys (signing and encryption keys) for an existing
    /// DID (owned by the caller of the library).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `tgt_did` - DID to replace keys.
    /// * `identity_json` - Identity information as json.
    /// # Example
    /// * `identity_json`- 
    /// {
    ///     "seed": string, (optional; if not provide then a random one will be created)
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    /// }
    ///
    /// # Returns
    /// * `verkey` - The DIDs verification key
    pub fn replace_keys_start(wallet_handle: IndyHandle, tgt_did: &str, identity_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_replace_keys_start(command_handle, wallet_handle, tgt_did, identity_json, cb);

        ResultHandler::one(err, receiver)
    }

    /// Generated temporary keys (signing and encryption keys) for an existing
    /// DID (owned by the caller of the library).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `tgt_did` - DID to replace keys.
    /// * `identity_json` - Identity information as json.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Example
    /// * `identity_json`- 
    /// {
    ///     "seed": string, (optional; if not provide then a random one will be created)
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    /// }
    ///
    /// # Returns
    /// * `verkey` - The DIDs verification key
    pub fn replace_keys_start_timeout(wallet_handle: IndyHandle, tgt_did: &str, identity_json: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_replace_keys_start(command_handle, wallet_handle, tgt_did, identity_json, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Generated temporary keys (signing and encryption keys) for an existing
    /// DID (owned by the caller of the library).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `tgt_did` - DID to replace keys.
    /// * `identity_json` - Identity information as json.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Example
    /// * `identity_json`- 
    /// {
    ///     "seed": string, (optional; if not provide then a random one will be created)
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    /// }
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn replace_keys_start_async<F: 'static>(wallet_handle: IndyHandle, tgt_did: &str, identity_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Did::_replace_keys_start(command_handle, wallet_handle, tgt_did, identity_json, cb)
    }

    fn _replace_keys_start(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, identity_json: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let tgt_did = c_str!(tgt_did);
        let identity_json = c_str!(identity_json);

        ErrorCode::from(unsafe { did::indy_replace_keys_start(command_handle, wallet_handle, tgt_did.as_ptr(), identity_json.as_ptr(), cb) })
    }

    /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `tgt_did` - DID stored in the wallet
    pub fn replace_keys_apply(wallet_handle: IndyHandle, tgt_did: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_replace_keys_apply(command_handle, wallet_handle, tgt_did, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `tgt_did` - DID stored in the wallet
    /// * `timeout` - the maximum time this function waits for a response
    pub fn replace_keys_apply_timeout(wallet_handle: IndyHandle, tgt_did: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_replace_keys_apply(command_handle, wallet_handle, tgt_did, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `tgt_did` - DID stored in the wallet
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn replace_keys_apply_async<F: 'static>(wallet_handle: IndyHandle, tgt_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Did::_replace_keys_apply(command_handle, wallet_handle, tgt_did, cb)
    }
    
    fn _replace_keys_apply(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let tgt_did = c_str!(tgt_did);

        ErrorCode::from(unsafe { did::indy_replace_keys_apply(command_handle, wallet_handle, tgt_did.as_ptr(), cb) })
    }

    /// Saves their DID for a pairwise connection in a secured Wallet,
    /// so that it can be used to verify transaction.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `identity_json` - Identity information as json.
    /// # Example:
    /// * `identity_json`
    ///     {
    ///        "did": string, (required)
    ///        "verkey": string (optional, can be avoided if did is cryptonym: did == verkey),
    ///     }
    pub fn store_their_did(wallet_handle: IndyHandle, identity_json: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_store_their_did(command_handle, wallet_handle, identity_json, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Saves their DID for a pairwise connection in a secured Wallet,
    /// so that it can be used to verify transaction.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `identity_json` - Identity information as json.
    /// * `timeout` - the maximum time this function waits for a response
    /// # Example:
    /// * `identity_json`
    ///     {
    ///        "did": string, (required)
    ///        "verkey": string (optional, can be avoided if did is cryptonym: did == verkey),
    ///     }
    pub fn store_their_did_timeout(wallet_handle: IndyHandle, identity_json: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_store_their_did(command_handle, wallet_handle, identity_json, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Saves their DID for a pairwise connection in a secured Wallet,
    /// so that it can be used to verify transaction.
    ///
    /// # Arguments
    /// * `wallet_handle` - wallet handler (created by Wallet::open).
    /// * `identity_json` - Identity information as json.
    /// * `closure` - the closure that is called when finished
    /// # Example:
    /// * `identity_json`
    ///     {
    ///        "did": string, (required)
    ///        "verkey": string (optional, can be avoided if did is cryptonym: did == verkey),
    ///     }
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn store_their_did_async<F: 'static>(wallet_handle: IndyHandle, identity_json: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Did::_store_their_did(command_handle, wallet_handle, identity_json, cb)
    }

    fn _store_their_did(command_handle: IndyHandle, wallet_handle: IndyHandle, identity_json: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let identity_json = c_str!(identity_json);

        ErrorCode::from(unsafe { did::indy_store_their_did(command_handle, wallet_handle, identity_json.as_ptr(), cb) })
    }

    /// Returns ver key (key id) for the given DID.
    ///
    /// "Did::get_ver_key" call follow the idea that we resolve information about their DID from
    /// the ledger with cache in the local wallet. The "indy_Wallet::open" call has freshness parameter
    /// that is used for checking the freshness of cached pool value.
    ///
    /// Note if you don't want to resolve their DID info from the ledger you can use
    /// "Did::get_ver_key" call instead that will look only to the local wallet and skip
    /// freshness checking.
    ///
    /// Note that "Did::new" makes similar wallet record as "Key::create".
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// # Arguments
    /// * `pool_handle` - Pool handle (created by Pool::open).
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve key.
    ///
    /// # Returns
    /// * `key` - The DIDs ver key (key id).
    pub fn get_ver_key(pool_handle: IndyHandle, wallet_handle: IndyHandle, did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_ver_key(command_handle, pool_handle, wallet_handle, did, cb);

        ResultHandler::one(err, receiver)
    }

    /// Returns ver key (key id) for the given DID.
    ///
    /// "Did::get_ver_key" call follow the idea that we resolve information about their DID from
    /// the ledger with cache in the local wallet. The "indy_Wallet::open" call has freshness parameter
    /// that is used for checking the freshness of cached pool value.
    ///
    /// Note if you don't want to resolve their DID info from the ledger you can use
    /// "Did::get_ver_key" call instead that will look only to the local wallet and skip
    /// freshness checking.
    ///
    /// Note that "Did::new" makes similar wallet record as "Key::create".
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// # Arguments
    /// * `pool_handle` - Pool handle (created by Pool::open).
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve key.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `key` - The DIDs ver key (key id).
    pub fn get_ver_key_timeout(pool_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_ver_key(command_handle, pool_handle, wallet_handle, did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Returns ver key (key id) for the given DID.
    ///
    /// "Did::get_ver_key" call follow the idea that we resolve information about their DID from
    /// the ledger with cache in the local wallet. The "indy_Wallet::open" call has freshness parameter
    /// that is used for checking the freshness of cached pool value.
    ///
    /// Note if you don't want to resolve their DID info from the ledger you can use
    /// "Did::get_ver_key" call instead that will look only to the local wallet and skip
    /// freshness checking.
    ///
    /// Note that "Did::new" makes similar wallet record as "Key::create".
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// # Arguments
    /// * `pool_handle` - Pool handle (created by Pool::open).
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve key.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_ver_key_async<F: 'static>(pool_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Did::_get_ver_key(command_handle, pool_handle, wallet_handle, did, cb)
    }
    
    fn _get_ver_key(command_handle: IndyHandle, pool_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let did = c_str!(did);

        ErrorCode::from(unsafe { did::indy_key_for_did(command_handle, pool_handle, wallet_handle, did.as_ptr(), cb) })
    }

    /// Returns ver key (key id) for the given DID.
    ///
    /// "Did::get_ver_key_did" call looks data stored in the local wallet only and skips freshness
    /// checking.
    ///
    /// Note if you want to get fresh data from the ledger you can use "Did::get_ver_key" call
    /// instead.
    ///
    /// Note that "Did::new" makes similar wallet record as "Key::create".
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve key.
    ///
    /// # Returns
    /// * `key` - The DIDs ver key (key id).
    pub fn get_ver_key_local(wallet_handle: IndyHandle, did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_ver_key_local(command_handle, wallet_handle, did, cb);

        ResultHandler::one(err, receiver)
    }

    /// Returns ver key (key id) for the given DID.
    ///
    /// "Did::get_ver_key_did" call looks data stored in the local wallet only and skips freshness
    /// checking.
    ///
    /// Note if you want to get fresh data from the ledger you can use "Did::get_ver_key" call
    /// instead.
    ///
    /// Note that "Did::new" makes similar wallet record as "Key::create".
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve key.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `key` - The DIDs ver key (key id).
    pub fn get_ver_key_local_timeout(wallet_handle: IndyHandle, did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_ver_key_local(command_handle, wallet_handle, did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Returns ver key (key id) for the given DID.
    ///
    /// "Did::get_ver_key_did" call looks data stored in the local wallet only and skips freshness
    /// checking.
    ///
    /// Note if you want to get fresh data from the ledger you can use "Did::get_ver_key" call
    /// instead.
    ///
    /// Note that "Did::new" makes similar wallet record as "Key::create".
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve key.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_ver_key_local_async<F: 'static>(wallet_handle: IndyHandle, did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Did::_get_ver_key_local(command_handle, wallet_handle, did, cb)
    }
    
    fn _get_ver_key_local(command_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let did = c_str!(did);

        ErrorCode::from(unsafe { did::indy_key_for_local_did(command_handle, wallet_handle, did.as_ptr(), cb) })
    }

    /// Set/replaces endpoint information for the given DID.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve endpoint.
    /// * `address` -  The DIDs endpoint address.
    /// * `transport_key` - The DIDs transport key (ver key, key id).
    pub fn set_endpoint(wallet_handle: IndyHandle, did: &str, address: &str, transport_key: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_set_endpoint(command_handle, wallet_handle, did, address, transport_key, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Set/replaces endpoint information for the given DID.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve endpoint.
    /// * `address` -  The DIDs endpoint address.
    /// * `transport_key` - The DIDs transport key (ver key, key id).
    /// * `timeout` - the maximum time this function waits for a response
    pub fn set_endpoint_timeout(wallet_handle: IndyHandle, did: &str, address: &str, transport_key: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_set_endpoint(command_handle, wallet_handle, did, address, transport_key, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Set/replaces endpoint information for the given DID.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve endpoint.
    /// * `address` -  The DIDs endpoint address.
    /// * `transport_key` - The DIDs transport key (ver key, key id).
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn set_endpoint_async<F: 'static>(wallet_handle: IndyHandle, did: &str, address: &str, transport_key: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Did::_set_endpoint(command_handle, wallet_handle, did, address, transport_key, cb)
    }
    
    fn _set_endpoint(command_handle: IndyHandle, wallet_handle: IndyHandle, did: &str, address: &str, transport_key: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let did = c_str!(did);
        let address = c_str!(address);
        let transport_key = c_str!(transport_key);

        ErrorCode::from(unsafe { did::indy_set_endpoint_for_did(command_handle, wallet_handle, did.as_ptr(), address.as_ptr(), transport_key.as_ptr(), cb) })
    }

    /// Returns endpoint information for the given DID.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve endpoint.
    ///
    /// # Returns
    /// * `endpoint` - The DIDs endpoint.
    /// * `transport_vk` - The DIDs transport key (ver key, key id).
    pub fn get_endpoint(wallet_handle: IndyHandle, pool_handle: IndyHandle, did: &str) -> Result<(String, Option<String>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string();

        let err = Did::_get_endpoint(command_handle, wallet_handle, pool_handle, did, cb);

        ResultHandler::two(err, receiver)
    }

    /// Returns endpoint information for the given DID.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve endpoint.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    /// * `endpoint` - The DIDs endpoint.
    /// * `transport_vk` - The DIDs transport key (ver key, key id).
    pub fn get_endpoint_timeout(wallet_handle: IndyHandle, pool_handle: IndyHandle, did: &str, timeout: Duration) -> Result<(String, Option<String>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_opt_string();

        let err = Did::_get_endpoint(command_handle, wallet_handle, pool_handle, did, cb);

        ResultHandler::two_timeout(err, receiver, timeout)
    }

    /// Returns endpoint information for the given DID.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to resolve endpoint.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_endpoint_async<F: 'static>(wallet_handle: IndyHandle, pool_handle: IndyHandle, did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String, Option<String>) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string_opt_string(Box::new(closure));

        Did::_get_endpoint(command_handle, wallet_handle, pool_handle, did, cb)
    }
    
    fn _get_endpoint(command_handle: IndyHandle, wallet_handle: IndyHandle, pool_handle: IndyHandle, did: &str, cb: Option<ResponseStringStringCB>) -> ErrorCode {
        let did = c_str!(did);

        ErrorCode::from(unsafe { did::indy_get_endpoint_for_did(command_handle, wallet_handle, pool_handle, did.as_ptr(), cb) })
    }

    /// Saves/replaces the meta information for the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - the DID to store metadata.
    /// * `metadata`  - the meta information that will be store with the DID.
    pub fn set_metadata(wallet_handle: IndyHandle, tgt_did: &str, metadata: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_set_metadata(command_handle, wallet_handle, tgt_did, metadata, cb);

        ResultHandler::empty(err, receiver)
    }

    /// Saves/replaces the meta information for the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - the DID to store metadata.
    /// * `metadata`  - the meta information that will be store with the DID.
    /// * `timeout` - the maximum time this function waits for a response
    pub fn set_metadata_timeout(wallet_handle: IndyHandle, tgt_did: &str, metadata: &str, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Did::_set_metadata(command_handle, wallet_handle, tgt_did, metadata, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// Saves/replaces the meta information for the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - the DID to store metadata.
    /// * `metadata`  - the meta information that will be store with the DID.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn set_metadata_async<F: 'static>(wallet_handle: IndyHandle, tgt_did: &str, metadata: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Did::_set_metadata(command_handle, wallet_handle, tgt_did, metadata, cb)
    }
    
    fn _set_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, metadata: &str, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let tgt_did = c_str!(tgt_did);
        let metadata = c_str!(metadata);

        ErrorCode::from(unsafe { did::indy_set_did_metadata(command_handle, wallet_handle, tgt_did.as_ptr(), metadata.as_ptr(), cb) })
    }

    /// Retrieves the meta information for the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did`  - The DID to retrieve metadata.
    ///
    /// #Returns
    /// * `metadata`  - The meta information stored with the DID; Can be null if no metadata was saved for this DID.
    pub fn get_metadata(wallet_handle: IndyHandle, tgt_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_metadata(command_handle, wallet_handle, tgt_did, cb);

        ResultHandler::one(err, receiver)
    }

    /// Retrieves the meta information for the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did`  - The DID to retrieve metadata.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// #Returns
    /// * `metadata`  - The meta information stored with the DID; Can be null if no metadata was saved for this DID.
    pub fn get_metadata_timeout(wallet_handle: IndyHandle, tgt_did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_metadata(command_handle, wallet_handle, tgt_did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Retrieves the meta information for the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did`  - The DID to retrieve metadata.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_metadata_async<F: 'static>(wallet_handle: IndyHandle, tgt_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Did::_get_metadata(command_handle, wallet_handle, tgt_did, cb)
    }
    
    fn _get_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, tgt_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let tgt_did = c_str!(tgt_did);

        ErrorCode::from(unsafe { did::indy_get_did_metadata(command_handle, wallet_handle, tgt_did.as_ptr(), cb) })
    }

    /// Retrieves the information about the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to retrieve information.
    ///
    /// # Returns
    ///  * `did_with_meta` -  {
    ///     "did": string - DID stored in the wallet,
    ///     "verkey": string - The DIDs transport key (ver key, key id),
    ///     "metadata": string - The meta information stored with the DID
    ///   }
    pub fn get_my_metadata(wallet_handle: IndyHandle, my_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_my_metadata(command_handle, wallet_handle, my_did, cb);

        ResultHandler::one(err, receiver)
    }

    /// Retrieves the information about the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to retrieve information.
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    ///  * `did_with_meta` -  {
    ///     "did": string - DID stored in the wallet,
    ///     "verkey": string - The DIDs transport key (ver key, key id),
    ///     "metadata": string - The meta information stored with the DID
    ///   }
    pub fn get_my_metadata_timeout(wallet_handle: IndyHandle, my_did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_get_my_metadata(command_handle, wallet_handle, my_did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Retrieves the information about the giving DID in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `did` - The DID to retrieve information.
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_my_metadata_async<F: 'static>(wallet_handle: IndyHandle, my_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Did::_get_my_metadata(command_handle, wallet_handle, my_did, cb)
    }
    
    fn _get_my_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, my_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let my_did = c_str!(my_did);

        ErrorCode::from(unsafe { did::indy_get_my_did_with_meta(command_handle, wallet_handle, my_did.as_ptr(), cb) })
    }

    /// Retrieves the information about all DIDs stored in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    ///
    /// # Returns
    ///  * `dids` - [{
    ///     "did": string - DID stored in the wallet,
    ///     "verkey": string - The DIDs transport key (ver key, key id).,
    ///     "metadata": string - The meta information stored with the DID
    ///   }]
    pub fn list_with_metadata(wallet_handle: IndyHandle) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_list_with_metadata(command_handle, wallet_handle, cb);

        ResultHandler::one(err, receiver)
    }

    /// Retrieves the information about all DIDs stored in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// # Returns
    ///  * `dids` - [{
    ///     "did": string - DID stored in the wallet,
    ///     "verkey": string - The DIDs transport key (ver key, key id).,
    ///     "metadata": string - The meta information stored with the DID
    ///   }]
    pub fn list_with_metadata_timeout(wallet_handle: IndyHandle, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_list_with_metadata(command_handle, wallet_handle, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Retrieves the information about all DIDs stored in the wallet.
    ///
    /// # Arguments
    /// * `wallet_handle` - Wallet handle (created by Wallet::open).
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn list_with_metadata_async<F: 'static>(wallet_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Did::_list_with_metadata(command_handle, wallet_handle, cb)
    }
    
    fn _list_with_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
        ErrorCode::from(unsafe { did::indy_list_my_dids_with_meta(command_handle, wallet_handle, cb) })
    }

    /// Retrieves abbreviated verkey if it is possible otherwise return full verkey.
    ///
    /// # Arguments
    /// * `tgt_did` - DID.
    /// * `full_verkey` - The DIDs verification key,
    ///
    /// #Returns
    ///  * `verkey` - The DIDs verification key in either abbreviated or full form
    pub fn abbreviate_verkey(tgt_did: &str, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_abbreviate_verkey(command_handle, tgt_did, verkey, cb);

        ResultHandler::one(err, receiver)
    }

    /// Retrieves abbreviated verkey if it is possible otherwise return full verkey.
    ///
    /// # Arguments
    /// * `tgt_did` - DID.
    /// * `full_verkey` - The DIDs verification key,
    /// * `timeout` - the maximum time this function waits for a response
    ///
    /// #Returns
    ///  * `verkey` - The DIDs verification key in either abbreviated or full form
    pub fn abbreviate_verkey_timeout(tgt_did: &str, verkey: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Did::_abbreviate_verkey(command_handle, tgt_did, verkey, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// Retrieves abbreviated verkey if it is possible otherwise return full verkey.
    ///
    /// # Arguments
    /// * `tgt_did` - DID.
    /// * `full_verkey` - The DIDs verification key,
    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn abbreviate_verkey_async<F: 'static>(tgt_did: &str, verkey: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Did::_abbreviate_verkey(command_handle, tgt_did, verkey, cb)
    }
    
    fn _abbreviate_verkey(command_handle: IndyHandle, tgt_did: &str, verkey: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let tgt_did = c_str!(tgt_did);
        let verkey = c_str!(verkey);

        ErrorCode::from(unsafe { did::indy_abbreviate_verkey(command_handle, tgt_did.as_ptr(), verkey.as_ptr(), cb) })
    }
}
