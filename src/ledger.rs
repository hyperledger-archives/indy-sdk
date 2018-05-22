use super::ErrorCode;

use std::ffi::CString;
use std::ptr::null;
use utils;
use indy::ledger;

pub struct Ledger {}

impl Ledger {
    pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let request_json = CString::new(request_json).unwrap();

        let err = unsafe {
            ledger::indy_sign_and_submit_request(command_handle,
                                         pool_handle,
                                         wallet_handle,
                                         submitter_did.as_ptr(),
                                         request_json.as_ptr(),
                                         cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let request_json = CString::new(request_json).unwrap();

        let err = unsafe {
            ledger::indy_submit_request(command_handle,
                                pool_handle,
                                request_json.as_ptr(),
                                cb)
        };

        utils::results::result_to_one(err, receiver)
    }


    pub fn multi_sign_request(wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let request_json = CString::new(request_json).unwrap();

        let err = unsafe {
            ledger::indy_multi_sign_request(command_handle,
                                    wallet_handle,
                                    submitter_did.as_ptr(),
                                    request_json.as_ptr(),
                                    cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let verkey_str = verkey.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let data_str = data.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let role_str = role.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let err = unsafe {
            ledger::indy_build_nym_request(command_handle,
                                   submitter_did.as_ptr(),
                                   target_did.as_ptr(),
                                   if verkey.is_some() { verkey_str.as_ptr() } else { null() },
                                   if data.is_some() { data_str.as_ptr() } else { null() },
                                   if role.is_some() { role_str.as_ptr() } else { null() },
                                   cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_get_nym_request(submitter_did: &str, target_did: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let err = unsafe {
            ledger::indy_build_get_nym_request(command_handle,
                                       submitter_did.as_ptr(),
                                       target_did.as_ptr(),
                                       cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let hash_str = hash.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let raw_str = raw.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let enc_str = enc.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            ledger::indy_build_attrib_request(command_handle,
                                      submitter_did.as_ptr(),
                                      target_did.as_ptr(),
                                      if hash.is_some() { hash_str.as_ptr() } else { null() },
                                      if raw.is_some() { raw_str.as_ptr() } else { null() },
                                      if enc.is_some() { enc_str.as_ptr() } else { null() },
                                      cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_get_attrib_request(submitter_did: &str, target_did: &str, raw: Option<&str>, hash: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let raw_str = raw.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let hash_str = hash.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let enc_str = enc.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            ledger::indy_build_get_attrib_request(command_handle,
                                          submitter_did.as_ptr(),
                                          target_did.as_ptr(),
                                          if raw.is_some() { raw_str.as_ptr() } else { null() },
                                          if hash.is_some() { hash_str.as_ptr() } else { null() },
                                          if enc.is_some() { enc_str.as_ptr() } else { null() },
                                          cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            ledger::indy_build_schema_request(command_handle,
                                      submitter_did.as_ptr(),
                                      data.as_ptr(),
                                      cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_get_schema_request(submitter_did: &str, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let id = CString::new(id).unwrap();

        let err = unsafe {
            ledger::indy_build_get_schema_request(command_handle,
                                          submitter_did.as_ptr(),
                                          id.as_ptr(),
                                          cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_cred_def_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            ledger::indy_build_cred_def_request(command_handle,
                                        submitter_did.as_ptr(),
                                        data.as_ptr(),
                                        cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_get_cred_def_request(submitter_did: &str, id: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let id = CString::new(id).unwrap();

        let err = unsafe {
            ledger::indy_build_get_cred_def_request(command_handle,
                                            submitter_did.as_ptr(),
                                            id.as_ptr(),
                                            cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();
        let data = CString::new(data).unwrap();

        let err = unsafe {
            ledger::indy_build_node_request(command_handle,
                                    submitter_did.as_ptr(),
                                    target_did.as_ptr(),
                                    data.as_ptr(),
                                    cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_pool_config_request(submitter_did: &str, writes: bool, force: bool) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();

        let err = unsafe {
            ledger::indy_build_pool_config_request(command_handle,
                                           submitter_did.as_ptr(),
                                           writes,
                                           force,
                                           cb)
        };

        utils::results::result_to_one(err, receiver)
    }

    pub fn build_pool_restart_request(submitter_did: &str, action: &str, datetime: Option<&str>) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let action = CString::new(action).unwrap();
        let datetime = datetime.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            ledger::indy_build_pool_restart_request(command_handle,
                                            submitter_did.as_ptr(),
                                            action.as_ptr(),
                                            datetime.as_ptr(),
                                            cb)
        };
        utils::results::result_to_one(err, receiver)
    }

    pub fn build_pool_upgrade_request(submitter_did: &str, name: &str, version: &str, action: &str, sha256: &str, timeout: Option<u32>, schedule: Option<&str>,
                                           justification: Option<&str>, reinstall: bool, force: bool) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = utils::callbacks::_closure_to_cb_ec_string();

        let submitter_did = CString::new(submitter_did).unwrap();
        let name = CString::new(name).unwrap();
        let version = CString::new(version).unwrap();
        let action = CString::new(action).unwrap();
        let sha256 = CString::new(sha256).unwrap();
        let timeout = timeout.map(|t| t as i32).unwrap_or(-1);

        let schedule_str = schedule.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let justification_str = justification.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err = unsafe {
            ledger::indy_build_pool_upgrade_request(command_handle,
                                            submitter_did.as_ptr(),
                                            name.as_ptr(),
                                            version.as_ptr(),
                                            action.as_ptr(),
                                            sha256.as_ptr(),
                                            timeout,
                                            if schedule.is_some() { schedule_str.as_ptr() } else { null() },
                                            if justification.is_some() { justification_str.as_ptr() } else { null() },
                                            reinstall,
                                            force,
                                            cb)
        };

        utils::results::result_to_one(err, receiver)
    }
}


