use sovrin::api::ErrorCode;
use sovrin::api::anoncreds::{
    sovrin_issuer_create_and_store_claim_def
};

use utils::callback::CallbackUtils;
use utils::wallet::WalletUtils;

use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;

pub struct AnoncredsUtils {}

impl AnoncredsUtils {
    pub fn create_claim_definition_and_set_link(wallet_handle: i32, schema: &str, claim_def_seq_no: i32) -> Result<String, ErrorCode> {
        let (claim_def_json, claim_def_uuid) = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &schema)?;
        WalletUtils::wallet_set_seq_no_for_value(wallet_handle, &claim_def_uuid, claim_def_seq_no)?;
        Ok(claim_def_json)
    }

    pub fn issuer_create_claim_definition(wallet_handle: i32, schema: &str) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_def_json, claim_def_uuid| {
            sender.send((err, claim_def_json, claim_def_uuid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_definition_cb(cb);

        let schema = CString::new(schema).unwrap();

        let err =
            sovrin_issuer_create_and_store_claim_def(command_handle,
                                                     wallet_handle,
                                                     schema.as_ptr(),
                                                     null(),
                                                     false,
                                                     cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_def_json, claim_def_uuid) = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((claim_def_json, claim_def_uuid))
    }
}