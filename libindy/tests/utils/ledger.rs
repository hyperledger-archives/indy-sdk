extern crate time;

use indy::api::ErrorCode;
use indy::api::ledger::{
    indy_sign_and_submit_request,
    indy_submit_request,
    indy_sign_request,
    indy_build_get_ddo_request,
    indy_build_attrib_request,
    indy_build_get_attrib_request,
    indy_build_get_nym_request,
    indy_build_schema_request,
    indy_build_get_schema_request,
    indy_build_claim_def_txn,
    indy_build_get_claim_def_txn,
    indy_build_node_request,
    indy_build_nym_request,
    indy_build_get_txn_request
};

use utils::callback::CallbackUtils;
use utils::timeout::TimeoutUtils;

use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;

pub struct LedgerUtils {}

impl LedgerUtils {
    pub fn sign_and_submit_request(pool_handle: i32, wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_result_json| {
            sender.send((err, request_result_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_sign_and_submit_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let request_json = CString::new(request_json).unwrap();

        let err =
            indy_sign_and_submit_request(command_handle,
                                           pool_handle,
                                           wallet_handle,
                                           submitter_did.as_ptr(),
                                           request_json.as_ptr(),
                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_result_json) = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_result_json)
    }

    pub fn submit_request(pool_handle: i32, request_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_result_json| {
            sender.send((err, request_result_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_submit_request_cb(cb);

        let request_json = CString::new(request_json).unwrap();

        let err =
            indy_submit_request(command_handle,
                                pool_handle,
                                request_json.as_ptr(),
                                cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_result_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_result_json)
    }

    pub fn sign_request(wallet_handle: i32, submitter_did: &str, request_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_result_json| {
            sender.send((err, request_result_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_sign_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let request_json = CString::new(request_json).unwrap();

        let err =
            indy_sign_request(command_handle,
                              wallet_handle,
                              submitter_did.as_ptr(),
                              request_json.as_ptr(),
                              cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_result_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_result_json)
    }

    pub fn build_get_ddo_request(submitter_did: &str, target_did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let err =
            indy_build_get_ddo_request(command_handle,
                                         submitter_did.as_ptr(),
                                         target_did.as_ptr(),
                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_nym_request(submitter_did: &str, target_did: &str, verkey: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let verkey_str = verkey.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let data_str = data.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let role_str = role.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let err =
            indy_build_nym_request(command_handle,
                                     submitter_did.as_ptr(),
                                     target_did.as_ptr(),
                                     if verkey.is_some() { verkey_str.as_ptr() } else { null() },
                                     if data.is_some() { data_str.as_ptr() } else { null() },
                                     if role.is_some() { role_str.as_ptr() } else { null() },
                                     cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_attrib_request(submitter_did: &str, target_did: &str, hash: Option<&str>, raw: Option<&str>, enc: Option<&str>) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let hash_str = hash.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let raw_str = raw.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let enc_str = enc.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_build_attrib_request(command_handle,
                                        submitter_did.as_ptr(),
                                        target_did.as_ptr(),
                                        if hash.is_some() { hash_str.as_ptr() } else { null() },
                                        if raw.is_some() { raw_str.as_ptr() } else { null() },
                                        if enc.is_some() { enc_str.as_ptr() } else { null() },
                                        cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_get_attrib_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();
        let data = CString::new(data).unwrap();

        let err =
            indy_build_get_attrib_request(command_handle,
                                            submitter_did.as_ptr(),
                                            target_did.as_ptr(),
                                            data.as_ptr(),
                                            cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_get_nym_request(submitter_did: &str, target_did: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();

        let err =
            indy_build_get_nym_request(command_handle,
                                         submitter_did.as_ptr(),
                                         target_did.as_ptr(),
                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_schema_request(submitter_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let data = CString::new(data).unwrap();

        let err =
            indy_build_schema_request(command_handle,
                                        submitter_did.as_ptr(),
                                        data.as_ptr(),
                                        cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_get_schema_request(submitter_did: &str, dest: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let dest = CString::new(dest).unwrap();
        let data = CString::new(data).unwrap();

        let err =
            indy_build_get_schema_request(command_handle,
                                            submitter_did.as_ptr(),
                                            dest.as_ptr(),
                                            data.as_ptr(),
                                            cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_claim_def_txn(submitter_did: &str, xref: i32, signature_type: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let signature_type = CString::new(signature_type).unwrap();
        let data = CString::new(data).unwrap();

        let err =
            indy_build_claim_def_txn(command_handle,
                                       submitter_did.as_ptr(),
                                       xref,
                                       signature_type.as_ptr(),
                                       data.as_ptr(),
                                       cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_get_claim_def_txn(submitter_did: &str, xref: i32, signature_type: &str, origin: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let signature_type = CString::new(signature_type).unwrap();
        let origin = CString::new(origin).unwrap();

        let err =
            indy_build_get_claim_def_txn(command_handle,
                                           submitter_did.as_ptr(),
                                           xref,
                                           signature_type.as_ptr(),
                                           origin.as_ptr(),
                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_node_request(submitter_did: &str, target_did: &str, data: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();
        let target_did = CString::new(target_did).unwrap();
        let data = CString::new(data).unwrap();

        let err =
            indy_build_node_request(command_handle,
                                      submitter_did.as_ptr(),
                                      target_did.as_ptr(),
                                      data.as_ptr(),
                                      cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }

    pub fn build_get_txn_request(submitter_did: &str, data: i32) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, request_json| {
            sender.send((err, request_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_build_request_cb(cb);

        let submitter_did = CString::new(submitter_did).unwrap();

        let err =
            indy_build_get_txn_request(command_handle,
                                         submitter_did.as_ptr(),
                                         data,
                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, request_json) = receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(request_json)
    }
}
