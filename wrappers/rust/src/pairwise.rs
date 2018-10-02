use {ErrorCode, IndyHandle};

use std::ffi::CString;
use std::time::Duration;
use std::ptr::null;

use utils::callbacks::ClosureHandler;
use utils::results::ResultHandler;

use native::pairwise;
use native::{ResponseEmptyCB,
          ResponseStringCB,
          ResponseBoolCB};

pub struct Pairwise {}

impl Pairwise {
    pub fn does_exist(wallet_handle: IndyHandle, their_did: &str) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

        let err = Pairwise::_does_exist(command_handle, wallet_handle, their_did, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn does_exist_timeout(wallet_handle: IndyHandle, their_did: &str, timeout: Duration) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

        let err = Pairwise::_does_exist(command_handle, wallet_handle, their_did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn does_exist_async<F: 'static>(wallet_handle: IndyHandle, their_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, bool) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_bool(Box::new(closure));

        Pairwise::_does_exist(command_handle, wallet_handle, their_did, cb)
    }

    fn _does_exist(command_handle: IndyHandle, wallet_handle: IndyHandle, their_did: &str, cb: Option<ResponseBoolCB>) -> ErrorCode {
        let their_did = c_str!(their_did);

        ErrorCode::from(unsafe {
            pairwise::indy_is_pairwise_exists(command_handle, wallet_handle, their_did.as_ptr(), cb)
        })
    }

    pub fn create(wallet_handle: IndyHandle, their_did: &str, my_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pairwise::_create(command_handle, wallet_handle, their_did, my_did, metadata, cb);

        ResultHandler::empty(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn create_timeout(wallet_handle: IndyHandle, their_did: &str, my_did: &str, metadata: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pairwise::_create(command_handle, wallet_handle, their_did, my_did, metadata, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn create_async<F: 'static>(wallet_handle: IndyHandle, their_did: &str, my_did: &str, metadata: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pairwise::_create(command_handle, wallet_handle, their_did, my_did, metadata, cb)
    }

    fn _create(command_handle: IndyHandle, wallet_handle: IndyHandle, their_did: &str, my_did: &str, metadata: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let their_did = c_str!(their_did);
        let my_did = c_str!(my_did);
        let metadata_str = opt_c_str!(metadata);

        ErrorCode::from(unsafe {
            pairwise::indy_create_pairwise(command_handle, wallet_handle, their_did.as_ptr(), my_did.as_ptr(), opt_c_ptr!(metadata, metadata_str), cb)
        })
    }

    pub fn list(wallet_handle: IndyHandle) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pairwise::_list(command_handle, wallet_handle, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn list_timeout(wallet_handle: IndyHandle, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pairwise::_list(command_handle, wallet_handle, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn list_async<F: 'static>(wallet_handle: IndyHandle, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Pairwise::_list(command_handle, wallet_handle, cb)
    }

    fn _list(command_handle: IndyHandle, wallet_handle: IndyHandle, cb: Option<ResponseStringCB>) -> ErrorCode {
        ErrorCode::from(unsafe {
            pairwise::indy_list_pairwise(command_handle, wallet_handle, cb)
        })
    }

    pub fn get(wallet_handle: IndyHandle, their_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pairwise::_get(command_handle, wallet_handle, their_did, cb);

        ResultHandler::one(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn get_timeout(wallet_handle: IndyHandle, their_did: &str, timeout: Duration) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let err = Pairwise::_get(command_handle, wallet_handle, their_did, cb);

        ResultHandler::one_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn get_async<F: 'static>(wallet_handle: IndyHandle, their_did: &str, closure: F) -> ErrorCode where F: FnMut(ErrorCode, String) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec_string(Box::new(closure));

        Pairwise::_get(command_handle, wallet_handle, their_did, cb)
    }

    fn _get(command_handle: IndyHandle, wallet_handle: IndyHandle, their_did: &str, cb: Option<ResponseStringCB>) -> ErrorCode {
        let their_did = c_str!(their_did);

        ErrorCode::from(unsafe {
            pairwise::indy_get_pairwise(command_handle, wallet_handle, their_did.as_ptr(), cb)
        })
    }

    pub fn set_metadata(wallet_handle: IndyHandle, their_did: &str, metadata: Option<&str>) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pairwise::_set_metadata(command_handle, wallet_handle, their_did, metadata, cb);

        ResultHandler::empty(err, receiver)
    }

    /// * `timeout` - the maximum time this function waits for a response
    pub fn set_metadata_timeout(wallet_handle: IndyHandle, their_did: &str, metadata: Option<&str>, timeout: Duration) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let err = Pairwise::_set_metadata(command_handle, wallet_handle, their_did, metadata, cb);

        ResultHandler::empty_timeout(err, receiver, timeout)
    }

    /// * `closure` - the closure that is called when finished
    ///
    /// # Returns
    /// * `errorcode` - errorcode from calling ffi function. The closure receives the return result
    pub fn set_metadata_async<F: 'static>(wallet_handle: IndyHandle, their_did: &str, metadata: Option<&str>, closure: F) -> ErrorCode where F: FnMut(ErrorCode) + Send {
        let (command_handle, cb) = ClosureHandler::convert_cb_ec(Box::new(closure));

        Pairwise::_set_metadata(command_handle, wallet_handle, their_did, metadata, cb)
    }

    fn _set_metadata(command_handle: IndyHandle, wallet_handle: IndyHandle, their_did: &str, metadata: Option<&str>, cb: Option<ResponseEmptyCB>) -> ErrorCode {
        let their_did = c_str!(their_did);
        let metadata_str = opt_c_str!(metadata);

        ErrorCode::from(unsafe {
            pairwise::indy_set_pairwise_metadata(command_handle, wallet_handle, their_did.as_ptr(), opt_c_ptr!(metadata, metadata_str), cb)
        })
    }
}
