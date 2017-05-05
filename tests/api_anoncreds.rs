extern crate sovrin;

#[macro_use]
extern crate lazy_static;

#[path = "utils/mod.rs"]
mod utils;

use sovrin::api::ErrorCode;
use sovrin::api::anoncreds::{
    sovrin_issuer_create_and_store_claim_def,
    sovrin_issuer_create_claim,
    sovrin_prover_create_master_secret,
    sovrin_prover_create_and_store_claim_req,
    sovrin_prover_store_claim,
    sovrin_prover_get_claims_for_proof_req,
    sovrin_prover_create_proof,
    sovrin_verifier_verify_proof
};
use sovrin::api::wallet::{sovrin_create_wallet, sovrin_open_wallet, sovrin_wallet_set_seq_no_for_value};

#[path = "../src/utils/environment.rs"]
mod environment;

use std::ptr::null;

use utils::callbacks::CallbacksHelpers;

use std::sync::mpsc::{channel};
use std::ffi::{CString};


#[test]
fn sovrin_anoncreds_demo() {
    let (create_wallet_sender, create_wallet_receiver) = channel();
    let (open_wallet_sender, open_wallet_receiver) = channel();
    let (issuer_create_claim_definition_sender, issuer_create_claim_definition_receiver) = channel();
    let (wallet_set_seq_no_for_value_sender, wallet_set_seq_no_for_value_receiver) = channel();
    let (prover_create_master_secret_sender, prover_create_master_secret_receiver) = channel();
    let (prover_create_claim_req_sender, prover_create_claim_req_receiver) = channel();
    let (issuer_create_claim_sender, issuer_create_claim_receiver) = channel();
    let (prover_store_claim_sender, prover_store_claim_receiver) = channel();
    let (prover_get_claims_for_proof_req_sender, prover_get_claims_for_proof_req_receiver) = channel();
    let (prover_create_proof_sender, prover_create_proof_receiver) = channel();
    let (verifier_verify_proof_sender, verifier_verify_proof_receiver) = channel();

    let issuer_create_claim_definition_cb = Box::new(move |err, claim_def_json, claim_def_uuid| {
        issuer_create_claim_definition_sender.send((err, claim_def_json, claim_def_uuid)).unwrap();
    });
    let create_wallet_cb = Box::new(move |err| {
        create_wallet_sender.send(err).unwrap();
    });
    let open_wallet_cb = Box::new(move |err, handle| {
        open_wallet_sender.send((err, handle)).unwrap();
    });
    let wallet_set_seq_no_for_value_cb = Box::new(move |err| {
        wallet_set_seq_no_for_value_sender.send(err).unwrap();
    });
    let prover_create_master_secret_cb = Box::new(move |err| {
        prover_create_master_secret_sender.send(err).unwrap();
    });
    let prover_create_claim_req_cb = Box::new(move |err, claim_req_json| {
        prover_create_claim_req_sender.send((err, claim_req_json)).unwrap();
    });
    let issuer_create_claim_cb = Box::new(move |err, revoc_reg_update_json, xclaim_json| {
        issuer_create_claim_sender.send((err, revoc_reg_update_json, xclaim_json)).unwrap();
    });
    let prover_store_claim_cb = Box::new(move |err| {
        prover_store_claim_sender.send(err).unwrap();
    });
    let prover_get_claims_for_proof_req_cb = Box::new(move |err, claims_json| {
        prover_get_claims_for_proof_req_sender.send((err, claims_json)).unwrap();
    });
    let prover_create_proof_cb = Box::new(move |err, proof_json| {
        prover_create_proof_sender.send((err, proof_json)).unwrap();
    });
    let verifier_verify_proof_cb = Box::new(move |err, valid| {
        verifier_verify_proof_sender.send((err, valid)).unwrap();
    });

    let (issuer_create_claim_definition_command_handle, create_claim_definition_callback) = CallbacksHelpers::closure_to_issuer_create_claim_definition_cb(issuer_create_claim_definition_cb);
    let (create_wallet_command_handle, create_wallet_callback) = CallbacksHelpers::closure_to_create_wallet_cb(create_wallet_cb);
    let (open_wallet_command_handle, open_wallet_callback) = CallbacksHelpers::closure_to_open_wallet_cb(open_wallet_cb);
    let (wallet_set_seq_no_for_value_command_handle, wallet_set_seq_no_for_value_callback) = CallbacksHelpers::closure_to_wallet_set_seq_no_for_value_cb(wallet_set_seq_no_for_value_cb);
    let (prover_create_master_secret_command_handle, prover_create_master_secret_callback) = CallbacksHelpers::closure_to_prover_create_master_secret_cb(prover_create_master_secret_cb);
    let (prover_create_claim_req_command_handle, prover_create_claim_req_callback) = CallbacksHelpers::closure_to_prover_create_claim_req_cb(prover_create_claim_req_cb);
    let (issuer_create_claim_command_handle, issuer_create_claim_callback) = CallbacksHelpers::closure_to_issuer_create_claim_cb(issuer_create_claim_cb);
    let (prover_store_claim_command_handle, prover_store_claim_callback) = CallbacksHelpers::closure_to_prover_store_claim_cb(prover_store_claim_cb);
    let (prover_get_claims_for_proof_req_handle, prover_get_claims_for_proof_req_callback) = CallbacksHelpers::closure_to_prover_get_claims_for_proof_req_cb(prover_get_claims_for_proof_req_cb);
    let (prover_create_proof_handle, prover_create_proof_callback) = CallbacksHelpers::closure_to_prover_create_proof_cb(prover_create_proof_cb);
    let (verifier_verify_proof_handle, verifier_verify_proof_callback) = CallbacksHelpers::closure_to_verifier_verify_proof_cb(verifier_verify_proof_cb);

    let pool_name = "pool1";
    let name = "wallet79";
    let xtype = "default";

    let err =
        sovrin_create_wallet(create_wallet_command_handle,
                             CString::new(pool_name).unwrap().as_ptr(),
                             CString::new(name).unwrap().as_ptr(),
                             CString::new(xtype).unwrap().as_ptr(),
                             null(),
                             null(),
                             create_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = create_wallet_receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);

    let err =
        sovrin_open_wallet(open_wallet_command_handle,
                           CString::new(name).unwrap().as_ptr(),
                           null(),
                           null(),
                           open_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, wallet_handle) = open_wallet_receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);

    let schema = "{\
            \"name\":\"gvt\",\
            \"version\":\"1.0\",\
            \"attribute_names\":[\"age\",\"sex\",\"height\",\"name\"],\
            \"seq_no\":1\
        }";

    let err =
        sovrin_issuer_create_and_store_claim_def(issuer_create_claim_definition_command_handle,
                                                 wallet_handle,
                                                 CString::new(schema).unwrap().as_ptr(),
                                                 null(),
                                                 false,
                                                 create_claim_definition_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claim_def_json, claim_def_uuid) = issuer_create_claim_definition_receiver.recv().unwrap();
    println!("claim_def_json {:?}", claim_def_json);
    assert_eq!(ErrorCode::Success, err);

    let claim_def_seq_no = 1;

    let err = sovrin_wallet_set_seq_no_for_value(wallet_set_seq_no_for_value_command_handle,
                                                 wallet_handle,
                                                 CString::new(claim_def_uuid).unwrap().as_ptr(),
                                                 claim_def_seq_no,
                                                 wallet_set_seq_no_for_value_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = wallet_set_seq_no_for_value_receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);

    //TODO USE ISSUER WALLET AND PROVER WALLET

    let master_secret_name = "name";

    let err =
        sovrin_prover_create_master_secret(prover_create_master_secret_command_handle,
                                           wallet_handle,
                                           CString::new(master_secret_name).unwrap().as_ptr(),
                                           prover_create_master_secret_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = prover_create_master_secret_receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);

    let prover_did = "some_prover_did";
    let claim_offer_json = "{\"issuer_did\":\"some_issuer_did\",\"claim_def_seq_no\":1}";

    let err =
        sovrin_prover_create_and_store_claim_req(prover_create_claim_req_command_handle,
                                                 wallet_handle,
                                                 CString::new(prover_did).unwrap().as_ptr(),
                                                 CString::new(claim_offer_json).unwrap().as_ptr(),
                                                 CString::new(claim_def_json.clone()).unwrap().as_ptr(),
                                                 CString::new(master_secret_name).unwrap().as_ptr(),
                                                 prover_create_claim_req_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claim_req_json) = prover_create_claim_req_receiver.recv().unwrap();
    println!("claim_req_json {:?}", claim_req_json);
    assert_eq!(ErrorCode::Success, err);

    let claim_json = "{\"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
                       \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
                       \"height\":[\"175\",\"175\"],\
                       \"age\":[\"28\",\"28\"]\
                       }";

    let err =
        sovrin_issuer_create_claim(issuer_create_claim_command_handle,
                                   wallet_handle,
                                   CString::new(claim_req_json).unwrap().as_ptr(),
                                   CString::new(claim_json).unwrap().as_ptr(),
                                   None,
                                   None,
                                   issuer_create_claim_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, revoc_reg_update_json, xclaim_json) = issuer_create_claim_receiver.recv().unwrap();
    println!("xclaim_json {:?}", xclaim_json);
    assert_eq!(ErrorCode::Success, err);

    let err =
        sovrin_prover_store_claim(prover_store_claim_command_handle,
                                  wallet_handle,
                                  CString::new(xclaim_json).unwrap().as_ptr(),
                                  prover_store_claim_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = prover_store_claim_receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);

    let proof_req_json = "{\"nonce\":\"123432421212\",\
                           \"requested_attrs\":{\"1\":{\"schema_seq_no\":1,\"name\":\"name\"}},\
                           \"requested_predicates\":{\"1\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}\
                         }";

    let err =
        sovrin_prover_get_claims_for_proof_req(prover_get_claims_for_proof_req_handle,
                                               wallet_handle,
                                               CString::new(proof_req_json).unwrap().as_ptr(),
                                               prover_get_claims_for_proof_req_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claims_json) = prover_get_claims_for_proof_req_receiver.recv().unwrap();
    println!("claims_json {:?}", claims_json);
    assert_eq!(ErrorCode::Success, err);

    let requested_claims_json = "{\"self_attested_attributes\":{},\
                                  \"requested_attrs\":{\"1\":[\"1\",true]},\
                                  \"requested_predicates\":{\"1\":\"1\"}\
                                }";

    let schemas_json = format!("{{\"{}\":{}}}", claim_def_seq_no, schema);
    let claim_defs_json = format!("{{\"{}\":{}}}", claim_def_seq_no, claim_def_json);
    let revoc_regs_jsons = "{}";

    let err =
        sovrin_prover_create_proof(prover_create_proof_handle,
                                   wallet_handle,
                                   CString::new(proof_req_json).unwrap().as_ptr(),
                                   CString::new(requested_claims_json).unwrap().as_ptr(),
                                   CString::new(schemas_json.clone()).unwrap().as_ptr(),
                                   CString::new(master_secret_name).unwrap().as_ptr(),
                                   CString::new(claim_defs_json.clone()).unwrap().as_ptr(),
                                   CString::new(revoc_regs_jsons.clone()).unwrap().as_ptr(),
                                   prover_create_proof_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, proof_json) = prover_create_proof_receiver.recv().unwrap();
    println!("proof_json {:?}", proof_json);
    assert_eq!(ErrorCode::Success, err);

    let err =
        sovrin_verifier_verify_proof(verifier_verify_proof_handle,
                                     wallet_handle,
                                     CString::new(proof_req_json).unwrap().as_ptr(),
                                     CString::new(proof_json).unwrap().as_ptr(),
                                     CString::new(schemas_json).unwrap().as_ptr(),
                                     CString::new(claim_defs_json).unwrap().as_ptr(),
                                     CString::new(revoc_regs_jsons).unwrap().as_ptr(),
                                     verifier_verify_proof_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, result) = verifier_verify_proof_receiver.recv().unwrap();
    assert_eq!(ErrorCode::Success, err);
    assert!(result);
}
