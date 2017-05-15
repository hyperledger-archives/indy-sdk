use sovrin::api::ErrorCode;
use sovrin::api::anoncreds::{
    sovrin_issuer_create_and_store_claim_def,
    sovrin_issuer_create_claim,
    sovrin_prover_create_master_secret,
    sovrin_prover_create_and_store_claim_req,
    sovrin_prover_store_claim,
    sovrin_prover_get_claims_for_proof_req,
    sovrin_prover_create_proof,
    sovrin_prover_store_claim_offer,
    sovrin_prover_get_claim_offers,
    sovrin_verifier_verify_proof
};

use utils::callback::CallbackUtils;
use utils::environment::EnvironmentUtils;
use utils::timeout::TimeoutUtils;
use utils::wallet::WalletUtils;

use std::fs;
use std::ffi::CString;
use std::io::Write;
use std::ptr::null;
use std::path::PathBuf;
use std::sync::mpsc::channel;

pub struct AnoncredsUtils {}

impl AnoncredsUtils {
    pub fn create_issuer_prover_verifier_wallets(pool_name: &str, issuer_wallet_name: &str,
                                                 prover_wallet_name: &str, verifier_wallet_name: &str,
                                                 xtype: &str) -> Result<(String, String, String)> {
        //1. Create Issuer wallet, get prover wallet handle
        let res = WalletUtils::create_wallet(pool_name, issuer_wallet_name, xtype);
        assert!(res.is_ok());

        let res = WalletUtils::open_wallet(issuer_wallet_name);
        assert!(res.is_ok());
        let issuer_wallet_handle = res.unwrap();

        //2. Create Prover wallet, get prover wallet handle
        let res = WalletUtils::create_wallet(pool_name, prover_wallet_name, xtype);
        assert!(res.is_ok());

        let res = WalletUtils::open_wallet(prover_wallet_name);
        assert!(res.is_ok());
        let prover_wallet_handle = res.unwrap();

        //3. Create Verifier wallet, get verifier wallet handle
        let res = WalletUtils::create_wallet(pool_name, verifier_wallet_name, xtype);
        assert!(res.is_ok());

        let res = WalletUtils::open_wallet(verifier_wallet_name);
        assert!(res.is_ok());
        let verifier_wallet_handle = res.unwrap();

        Ok((issuer_wallet_handle, prover_wallet_handle, verifier_wallet_handle))
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

    pub fn prover_create_master_secret(wallet_handle: i32, master_secret_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_create_master_secret_cb(cb);

        let master_secret_name = CString::new(master_secret_name).unwrap();

        let err = sovrin_prover_create_master_secret(command_handle,
                                                     wallet_handle,
                                                     master_secret_name.as_ptr(),
                                                     cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_store_claim_offer(wallet_handle: i32, claim_offer_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send((err)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_claim_offer_json_cb(cb);

        let claim_offer_json = CString::new(claim_offer_json).unwrap();

        let err = sovrin_prover_store_claim_offer(command_handle,
                                                  wallet_handle,
                                                  claim_offer_json.as_ptr(),
                                                  cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_get_claim_offers(wallet_handle: i32, filter_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_offers_json| {
            sender.send((err, claim_offers_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claim_offers_cb(cb);

        let filter_json = CString::new(filter_json).unwrap();

        let err = sovrin_prover_get_claim_offers(command_handle,
                                                 wallet_handle,
                                                 filter_json.as_ptr(),
                                                 cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_offers_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_offers_json)
    }

    pub fn prover_create_and_store_claim_req(wallet_handle: i32, prover_did: &str, claim_offer_json: &str,
                                             claim_def_json: &str, master_secret_name: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claim_req_json| {
            sender.send((err, claim_req_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_create_claim_req_cb(cb);

        let prover_did = CString::new(prover_did).unwrap();
        let claim_offer_json = CString::new(claim_offer_json).unwrap();
        let claim_def_json = CString::new(claim_def_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();

        let err = sovrin_prover_create_and_store_claim_req(command_handle,
                                                           wallet_handle,
                                                           prover_did.as_ptr(),
                                                           claim_offer_json.as_ptr(),
                                                           claim_def_json.as_ptr(),
                                                           master_secret_name.as_ptr(),
                                                           cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claim_req_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claim_req_json)
    }

    pub fn issuer_create_claim(wallet_handle: i32, claim_req_json: &str, claim_json: &str) -> Result<(String, String), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
            sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_issuer_create_claim_cb(cb);

        let claim_req_json = CString::new(claim_req_json).unwrap();
        let claim_json = CString::new(claim_json).unwrap();

        let err = sovrin_issuer_create_claim(command_handle,
                                             wallet_handle,
                                             claim_req_json.as_ptr(),
                                             claim_json.as_ptr(),
                                             None,
                                             None,
                                             cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, revoc_reg_update_json, xclaim_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((revoc_reg_update_json, xclaim_json))
    }

    pub fn prover_store_claim(wallet_handle: i32, claims_json: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_store_claim_cb(cb);

        let claims_json = CString::new(claims_json).unwrap();

        let err = sovrin_prover_store_claim(command_handle,
                                            wallet_handle,
                                            claims_json.as_ptr(),
                                            cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn prover_get_claims_for_proof_req(wallet_handle: i32, proof_request_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, claims_json| {
            sender.send((err, claims_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(cb);

        let proof_request_json = CString::new(proof_request_json).unwrap();

        let err = sovrin_prover_get_claims_for_proof_req(command_handle,
                                                         wallet_handle,
                                                         proof_request_json.as_ptr(),
                                                         cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, claims_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(claims_json)
    }

    pub fn prover_create_proof(wallet_handle: i32, proof_req_json: &str, requested_claims_json: &str,
                               schemas_json: &str, master_secret_name: &str, claim_defs_json: &str,
                               revoc_regs_json: &str) -> Result<String, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, proof_json| {
            sender.send((err, proof_json)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(cb);

        let proof_req_json = CString::new(proof_req_json).unwrap();
        let requested_claims_json = CString::new(requested_claims_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let master_secret_name = CString::new(master_secret_name).unwrap();
        let claim_defs_json = CString::new(claim_defs_json).unwrap();
        let revoc_regs_json = CString::new(revoc_regs_json).unwrap();

        let err = sovrin_prover_create_proof(command_handle,
                                             wallet_handle,
                                             proof_req_json.as_ptr(),
                                             requested_claims_json.as_ptr(),
                                             schemas_json.as_ptr(),
                                             master_secret_name.as_ptr(),
                                             claim_defs_json.as_ptr(),
                                             revoc_regs_json.as_ptr(),
                                             cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, proof_json) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(proof_json)
    }

    pub fn verifier_verify_proof(wallet_handle: i32, proof_request_json: &str, proof_json: &str,
                                 schemas_json: &str, claim_defs_json: &str, revoc_regs_json: &str) -> Result<bool, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, valid| {
            sender.send((err, valid)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_verifier_verify_proof_cb(cb);

        let proof_request_json = CString::new(proof_request_json).unwrap();
        let proof_json = CString::new(proof_json).unwrap();
        let schemas_json = CString::new(schemas_json).unwrap();
        let claim_defs_json = CString::new(claim_defs_json).unwrap();
        let revoc_regs_json = CString::new(revoc_regs_json).unwrap();

        let err = sovrin_verifier_verify_proof(command_handle,
                                               wallet_handle,
                                               proof_request_json.as_ptr(),
                                               proof_json.as_ptr(),
                                               schemas_json.as_ptr(),
                                               claim_defs_json.as_ptr(),
                                               revoc_regs_json.as_ptr(),
                                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, valid) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(valid)
    }
}