extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;

#[cfg(feature = "local_nodes_pool")]
use utils::pool::PoolUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;

use indy::api::ErrorCode;
use indy::api::anoncreds::*;
use indy::api::crypto::*;
#[cfg(feature = "local_nodes_pool")]
use indy::api::ledger::*;
#[cfg(feature = "local_nodes_pool")]
use indy::api::pool::*;
use indy::api::wallet::*;
use indy::api::did::*;

use utils::callback::CallbackUtils;

use std::ptr::null;
use std::sync::mpsc::channel;
use std::ffi::CString;
use utils::types::ClaimsForProofRequest;

#[cfg(feature = "local_nodes_pool")]
use std::thread;

#[test]
fn anoncreds_demo_works() {
    TestUtils::cleanup_storage();

    let (create_wallet_sender, create_wallet_receiver) = channel();
    let (open_wallet_sender, open_wallet_receiver) = channel();
    let (issuer_create_claim_definition_sender, issuer_create_claim_definition_receiver) = channel();
    let (issuer_create_claim_offer_sender, issuer_create_claim_offer_receiver) = channel();
    let (prover_create_master_secret_sender, prover_create_master_secret_receiver) = channel();
    let (prover_create_claim_req_sender, prover_create_claim_req_receiver) = channel();
    let (issuer_create_claim_sender, issuer_create_claim_receiver) = channel();
    let (prover_store_claim_sender, prover_store_claim_receiver) = channel();
    let (prover_get_claims_for_proof_req_sender, prover_get_claims_for_proof_req_receiver) = channel();
    let (prover_create_proof_sender, prover_create_proof_receiver) = channel();
    let (verifier_verify_proof_sender, verifier_verify_proof_receiver) = channel();
    let (close_wallet_sender, close_wallet_receiver) = channel();

    let issuer_create_claim_definition_cb = Box::new(move |err, claim_def_json| {
        issuer_create_claim_definition_sender.send((err, claim_def_json)).unwrap();
    });
    let issuer_create_claim_offer_cb = Box::new(move |err, claim_offer_json| {
        issuer_create_claim_offer_sender.send((err, claim_offer_json)).unwrap();
    });
    let create_wallet_cb = Box::new(move |err| {
        create_wallet_sender.send(err).unwrap();
    });
    let open_wallet_cb = Box::new(move |err, handle| {
        open_wallet_sender.send((err, handle)).unwrap();
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
    let close_wallet_cb = Box::new(move |err_code| close_wallet_sender.send(err_code).unwrap());

    let (issuer_create_claim_definition_command_handle, create_claim_definition_callback) = CallbackUtils::closure_to_issuer_create_claim_definition_cb(issuer_create_claim_definition_cb);
    let (issuer_create_claim_offer_command_handle, issuer_create_claim_offer_callback) = CallbackUtils::closure_to_issuer_create_claim_offer_cb(issuer_create_claim_offer_cb);
    let (create_wallet_command_handle, create_wallet_callback) = CallbackUtils::closure_to_create_wallet_cb(create_wallet_cb);
    let (open_wallet_command_handle, open_wallet_callback) = CallbackUtils::closure_to_open_wallet_cb(open_wallet_cb);
    let (prover_create_master_secret_command_handle, prover_create_master_secret_callback) = CallbackUtils::closure_to_prover_create_master_secret_cb(prover_create_master_secret_cb);
    let (prover_create_claim_req_command_handle, prover_create_claim_req_callback) = CallbackUtils::closure_to_prover_create_claim_req_cb(prover_create_claim_req_cb);
    let (issuer_create_claim_command_handle, issuer_create_claim_callback) = CallbackUtils::closure_to_issuer_create_claim_cb(issuer_create_claim_cb);
    let (prover_store_claim_command_handle, prover_store_claim_callback) = CallbackUtils::closure_to_prover_store_claim_cb(prover_store_claim_cb);
    let (prover_get_claims_for_proof_req_handle, prover_get_claims_for_proof_req_callback) = CallbackUtils::closure_to_prover_get_claims_for_proof_req_cb(prover_get_claims_for_proof_req_cb);
    let (prover_create_proof_handle, prover_create_proof_callback) = CallbackUtils::closure_to_prover_create_proof_cb(prover_create_proof_cb);
    let (verifier_verify_proof_handle, verifier_verify_proof_callback) = CallbackUtils::closure_to_verifier_verify_proof_cb(verifier_verify_proof_cb);
    let (close_wallet_command_handle, close_wallet_callback) = CallbackUtils::closure_to_delete_wallet_cb(close_wallet_cb);

    let pool_name = "pool_1";
    let wallet_name = "issuer_wallet1";
    let xtype = "default";

    //TODO CREATE ISSUER, PROVER, VERIFIER WALLETS
    //1. Creates Wallet
    let err =
        indy_create_wallet(create_wallet_command_handle,
                           CString::new(pool_name).unwrap().as_ptr(),
                           CString::new(wallet_name).unwrap().as_ptr(),
                           CString::new(xtype).unwrap().as_ptr(),
                           null(),
                           null(),
                           create_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = create_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    //2. Opens Wallet
    let err =
        indy_open_wallet(open_wallet_command_handle,
                         CString::new(wallet_name).unwrap().as_ptr(),
                         null(),
                         null(),
                         open_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, wallet_handle) = open_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let schema_seq_no = 1;
    let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e";
    let prover_did = "VsKV7grR1BUE29mG2Fm2kX";

    let schema = format!(r#"{{
                                    "seqNo":{},
                                    "dest":"{}",
                                    "data":{{
                                        "name":"gvt",
                                        "version":"1.0",
                                        "attr_names":["age","sex","height","name"]
                                    }}
                                 }}"#, schema_seq_no, issuer_did);

    // 3. Issuer create Claim Definition for Schema
    let err =
        indy_issuer_create_and_store_claim_def(issuer_create_claim_definition_command_handle,
                                               wallet_handle,
                                               CString::new(issuer_did.clone()).unwrap().as_ptr(),
                                               CString::new(schema.clone()).unwrap().as_ptr(),
                                               null(),
                                               false,
                                               create_claim_definition_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claim_def_json) = issuer_create_claim_definition_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let master_secret_name = "master_secret";

    // 4. Prover create Master Secret
    let err =
        indy_prover_create_master_secret(prover_create_master_secret_command_handle,
                                         wallet_handle,
                                         CString::new(master_secret_name).unwrap().as_ptr(),
                                         prover_create_master_secret_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = prover_create_master_secret_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 5. Issuer create Claim Offer
    let err =
        indy_issuer_create_claim_offer(issuer_create_claim_offer_command_handle,
                                      wallet_handle,
                                      CString::new(schema.clone()).unwrap().as_ptr(),
                                      CString::new(issuer_did.clone()).unwrap().as_ptr(),
                                      CString::new(prover_did.clone()).unwrap().as_ptr(),
                                      issuer_create_claim_offer_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claim_offer_json) = issuer_create_claim_offer_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 6. Prover create Claim Request
    let err =
        indy_prover_create_and_store_claim_req(prover_create_claim_req_command_handle,
                                               wallet_handle,
                                               CString::new(prover_did).unwrap().as_ptr(),
                                               CString::new(claim_offer_json).unwrap().as_ptr(),
                                               CString::new(claim_def_json.clone()).unwrap().as_ptr(),
                                               CString::new(master_secret_name).unwrap().as_ptr(),
                                               prover_create_claim_req_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claim_req_json) = prover_create_claim_req_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let claim_json = r#"{
                               "sex":["male","5944657099558967239210949258394887428692050081607692519917050011144233115103"],
                               "name":["Alex","1139481716457488690172217916278103335"],
                               "height":["175","175"],
                               "age":["28","28"]
                             }"#;

    // 7. Issuer create Claim for Claim Request
    let err =
        indy_issuer_create_claim(issuer_create_claim_command_handle,
                                 wallet_handle,
                                 CString::new(claim_req_json).unwrap().as_ptr(),
                                 CString::new(claim_json).unwrap().as_ptr(),
                                 -1,
                                 issuer_create_claim_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, _, xclaim_json) = issuer_create_claim_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 7. Prover process and store Claim
    let err =
        indy_prover_store_claim(prover_store_claim_command_handle,
                                wallet_handle,
                                CString::new(xclaim_json).unwrap().as_ptr(),
                                null(),
                                prover_store_claim_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = prover_store_claim_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    let proof_req_json = format!(r#"{{
                                               "nonce":"123432421212",
                                               "name":"proof_req_1",
                                               "version":"0.1",
                                               "requested_attrs":{{
                                                    "attr1_referent":{{
                                                        "name":"name",
                                                        "restrictions":[{{"issuer_did":"{}",
                                                                        "schema_key":{{
                                                                            "name":"gvt",
                                                                            "version":"1.0",
                                                                            "did":"{}"
                                                                        }}
                                                        }}]
                                                    }}
                                               }},
                                               "requested_predicates":{{
                                                   "predicate1_referent":{{
                                                       "attr_name":"age",
                                                       "p_type":">=",
                                                       "value":18
                                                   }}
                                               }}
                                           }}"#, issuer_did, issuer_did);

    // 8. Prover gets Claims for Proof Request
    let err =
        indy_prover_get_claims_for_proof_req(prover_get_claims_for_proof_req_handle,
                                             wallet_handle,
                                             CString::new(proof_req_json.clone()).unwrap().as_ptr(),
                                             prover_get_claims_for_proof_req_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, claims_json) = prover_get_claims_for_proof_req_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);
    let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
    let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
    assert_eq!(1, claims_for_attr_1.len());

    let claim = claims_for_attr_1[0].clone();

    let requested_claims_json = format!(r#"{{
                                                    "self_attested_attributes":{{}},
                                                    "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                    "requested_predicates":{{"predicate1_referent":"{}"}}
                                                  }}"#, claim.referent, claim.referent);

    let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema);
    let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
    let revoc_regs_jsons = "{}";

    // 9. Prover create Proof for Proof Request
    let err =
        indy_prover_create_proof(prover_create_proof_handle,
                                 wallet_handle,
                                 CString::new(proof_req_json.clone()).unwrap().as_ptr(),
                                 CString::new(requested_claims_json).unwrap().as_ptr(),
                                 CString::new(schemas_json.clone()).unwrap().as_ptr(),
                                 CString::new(master_secret_name).unwrap().as_ptr(),
                                 CString::new(claim_defs_json.clone()).unwrap().as_ptr(),
                                 CString::new(revoc_regs_jsons.clone()).unwrap().as_ptr(),
                                 prover_create_proof_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, proof_json) = prover_create_proof_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 10. Verifier verify proof
    let err =
        indy_verifier_verify_proof(verifier_verify_proof_handle,
                                   CString::new(proof_req_json).unwrap().as_ptr(),
                                   CString::new(proof_json).unwrap().as_ptr(),
                                   CString::new(schemas_json).unwrap().as_ptr(),
                                   CString::new(claim_defs_json).unwrap().as_ptr(),
                                   CString::new(revoc_regs_jsons).unwrap().as_ptr(),
                                   verifier_verify_proof_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, result) = verifier_verify_proof_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);
    assert!(result);

    // 11. Close wallet
    let res = indy_close_wallet(close_wallet_command_handle, wallet_handle, close_wallet_callback);
    assert_eq!(res, ErrorCode::Success);
    let res = close_wallet_receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
    assert_eq!(res, ErrorCode::Success);

    TestUtils::cleanup_storage();
}

#[test]
#[cfg(feature = "local_nodes_pool")]
fn ledger_demo_works() {
    TestUtils::cleanup_storage();
    let my_wallet_name = "my_wallet";
    let their_wallet_name = "their_wallet";
    let wallet_type = "default";
    let pool_name = "pool_1";
    let c_pool_name = CString::new(pool_name).unwrap();

    let (submit_sender, submit_receiver) = channel();
    let (get_nym_sender, get_nym_receiver) = channel();
    let (create_sender, create_receiver) = channel();
    let (open_sender, open_receiver) = channel();
    let (create_my_wallet_sender, create_my_wallet_receiver) = channel();
    let (create_their_wallet_sender, create_their_wallet_receiver) = channel();
    let (open_my_wallet_sender, open_my_wallet_receiver) = channel();
    let (open_their_wallet_sender, open_their_wallet_receiver) = channel();
    let (create_and_store_my_did_sender, create_and_store_my_did_receiver) = channel();
    let (create_and_store_their_did_sender, create_and_store_their_did_receiver) = channel();
    let (store_their_did_sender, store_their_did_receiver) = channel();
    let (close_pool_sender, close_pool_receiver) = channel();
    let (close_my_wallet_sender, close_my_wallet_receiver) = channel();
    let (close_their_wallet_sender, close_their_wallet_receiver) = channel();

    let create_cb = Box::new(move |err| { create_sender.send(err).unwrap(); });
    let open_cb = Box::new(move |err, pool_handle| { open_sender.send((err, pool_handle)).unwrap(); });
    let send_cb = Box::new(move |err, resp| { submit_sender.send((err, resp)).unwrap(); });
    let get_nym_cb = Box::new(move |err, resp| { get_nym_sender.send((err, resp)).unwrap(); });
    let create_my_wallet_cb = Box::new(move |err| { create_my_wallet_sender.send(err).unwrap(); });
    let create_their_wallet_cb = Box::new(move |err| { create_their_wallet_sender.send(err).unwrap(); });
    let open_my_wallet_cb = Box::new(move |err, handle| { open_my_wallet_sender.send((err, handle)).unwrap(); });
    let open_their_wallet_cb = Box::new(move |err, handle| { open_their_wallet_sender.send((err, handle)).unwrap(); });
    let create_and_store_my_did_cb = Box::new(move |err, did, verkey| { create_and_store_my_did_sender.send((err, did, verkey)).unwrap(); });
    let create_and_store_their_did_cb = Box::new(move |err, did, verkey| { create_and_store_their_did_sender.send((err, did, verkey)).unwrap(); });
    let store_their_did_cb = Box::new(move |err| { store_their_did_sender.send((err)).unwrap(); });
    let close_pool_cb = Box::new(move |err_code| close_pool_sender.send(err_code).unwrap());
    let close_my_wallet_cb = Box::new(move |err_code| close_my_wallet_sender.send(err_code).unwrap());
    let close_their_wallet_cb = Box::new(move |err_code| close_their_wallet_sender.send(err_code).unwrap());

    let (open_command_handle, open_callback) = CallbackUtils::closure_to_open_pool_ledger_cb(open_cb);
    let (create_command_handle, create_callback) = CallbackUtils::closure_to_create_pool_ledger_cb(create_cb);
    let (send_command_handle, send_callback) = CallbackUtils::closure_to_send_tx_cb(send_cb);
    let (get_nym_command_handle, get_nym_callback) = CallbackUtils::closure_to_send_tx_cb(get_nym_cb);
    let (create_my_wallet_command_handle, create_my_wallet_callback) = CallbackUtils::closure_to_create_wallet_cb(create_my_wallet_cb);
    let (create_their_wallet_command_handle, create_their_wallet_callback) = CallbackUtils::closure_to_create_wallet_cb(create_their_wallet_cb);
    let (open_my_wallet_command_handle, open_my_wallet_callback) = CallbackUtils::closure_to_open_wallet_cb(open_my_wallet_cb);
    let (open_their_wallet_command_handle, open_their_wallet_callback) = CallbackUtils::closure_to_open_wallet_cb(open_their_wallet_cb);
    let (create_and_store_my_did_command_handle, create_and_store_my_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_my_did_cb);
    let (create_and_store_their_did_command_handle, create_and_store_their_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_their_did_cb);
    let (store_their_did_command_handle, store_their_did_callback) = CallbackUtils::closure_to_store_their_did_cb(store_their_did_cb);
    let (close_pool_command_handle, close_pool_callback) = CallbackUtils::closure_to_close_pool_ledger_cb(close_pool_cb);
    let (close_my_wallet_command_handle, close_my_wallet_callback) = CallbackUtils::closure_to_delete_wallet_cb(close_my_wallet_cb);
    let (close_their_wallet_command_handle, close_their_wallet_callback) = CallbackUtils::closure_to_delete_wallet_cb(close_their_wallet_cb);

    // 1. Create ledger config from genesis txn file
    let txn_file_path = PoolUtils::create_genesis_txn_file_for_test_pool(pool_name, None, None);
    let pool_config = PoolUtils::pool_config_json(txn_file_path.as_path());
    let c_pool_config = CString::new(pool_config).unwrap();

    let err = indy_create_pool_ledger_config(create_command_handle,
                                             c_pool_name.as_ptr(),
                                             c_pool_config.as_ptr(),
                                             create_callback);
    assert_eq!(err, ErrorCode::Success);
    let err = create_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
    assert_eq!(err, ErrorCode::Success);

    // 2. Open pool ledger
    let err = indy_open_pool_ledger(open_command_handle,
                                    c_pool_name.as_ptr(),
                                    null(),
                                    open_callback);
    assert_eq!(err, ErrorCode::Success);
    let (err, pool_handle) = open_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();
    assert_eq!(err, ErrorCode::Success);
    thread::sleep(TimeoutUtils::short_timeout());

    // 3. Create My Wallet
    let err =
        indy_create_wallet(create_my_wallet_command_handle,
                           CString::new(pool_name).unwrap().as_ptr(),
                           CString::new(my_wallet_name).unwrap().as_ptr(),
                           CString::new(wallet_type).unwrap().as_ptr(),
                           null(),
                           null(),
                           create_my_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = create_my_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 4. Open My Wallet. Gets My wallet handle
    let err =
        indy_open_wallet(open_my_wallet_command_handle,
                         CString::new(my_wallet_name).unwrap().as_ptr(),
                         null(),
                         null(),
                         open_my_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, my_wallet_handle) = open_my_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);


    // 5. Create Their Wallet
    let err =
        indy_create_wallet(create_their_wallet_command_handle,
                           CString::new(pool_name).unwrap().as_ptr(),
                           CString::new(their_wallet_name).unwrap().as_ptr(),
                           CString::new(wallet_type).unwrap().as_ptr(),
                           null(),
                           null(),
                           create_their_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = create_their_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 6. Open Their Wallet. Gets Their wallet handle
    let err =
        indy_open_wallet(open_their_wallet_command_handle,
                         CString::new(their_wallet_name).unwrap().as_ptr(),
                         null(),
                         null(),
                         open_their_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, their_wallet_handle) = open_their_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 7. Create My DID
    let my_did_json = "{}";
    let err =
        indy_create_and_store_my_did(create_and_store_my_did_command_handle,
                                     my_wallet_handle,
                                     CString::new(my_did_json).unwrap().as_ptr(),
                                     create_and_store_my_did_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, my_did, my_verkey) = create_and_store_my_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    info!("did {:?}", my_did);
    info!("verkey {:?}", my_verkey);
    assert_eq!(ErrorCode::Success, err);

    // 8. Create Their DID from Trustee1 seed
    let their_did_json = r#"{"seed":"000000000000000000000000Trustee1"}"#;
    let err =
        indy_create_and_store_my_did(create_and_store_their_did_command_handle,
                                     their_wallet_handle,
                                     CString::new(their_did_json).unwrap().as_ptr(),
                                     create_and_store_their_did_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, their_did, their_verkey) = create_and_store_their_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    info!("their_did {:?}", their_did);
    info!("their_verkey {:?}", their_verkey);
    assert_eq!(ErrorCode::Success, err);

    // 9. Store Their DID
    let their_identity_json = format!(r#"{{"did":"{}",
                                        "verkey":"{}"
                                      }}"#,
                                      their_did, their_verkey);
    let err =
        indy_store_their_did(store_their_did_command_handle,
                             my_wallet_handle,
                             CString::new(their_identity_json).unwrap().as_ptr(),
                             store_their_did_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = store_their_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 10. Prepare NYM transaction
    let nym_req_id = PoolUtils::get_req_id();
    let nym_txn_req = Request {
        identifier: their_did.clone(),
        operation: Operation {
            dest: my_did.clone(),
            type_: "1".to_string(),
        },
        protocol_version: 1,
        req_id: nym_req_id,
        signature: None,
    };

    // 11. Send NYM request with signing
    let msg = serde_json::to_string(&nym_txn_req).unwrap();
    let req = CString::new(msg).unwrap();
    let did_for_sign = CString::new(their_did).unwrap();
    let err = indy_sign_and_submit_request(send_command_handle,
                                           pool_handle,
                                           their_wallet_handle,
                                           did_for_sign.as_ptr(),
                                           req.as_ptr(),
                                           send_callback);
    assert_eq!(err, ErrorCode::Success);
    let (err, resp) = submit_receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
    assert_eq!(err, ErrorCode::Success);
    let nym_resp: Reply = serde_json::from_str(&resp).unwrap();
    info!("nym_resp_raw : {:?}", resp);
    info!("nym_resp     : {:?}", nym_resp);

    // 12. Prepare and send GET_NYM request
    let get_nym_req_id = PoolUtils::get_req_id();
    let get_nym_txn = Request {
        req_id: get_nym_req_id,
        signature: None,
        identifier: my_verkey.clone(),
        operation: Operation {
            type_: "105".to_string(),
            dest: my_did.clone(),
        },
        protocol_version: 1,
    };

    let request = serde_json::to_string(&get_nym_txn).unwrap();
    let req = CString::new(request).unwrap();
    let err = indy_submit_request(get_nym_command_handle,
                                  pool_handle,
                                  req.as_ptr(),
                                  get_nym_callback);
    assert_eq!(err, ErrorCode::Success);
    let (err, resp) = get_nym_receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
    assert_eq!(err, ErrorCode::Success);
    let get_nym_resp: Reply = serde_json::from_str(&resp).unwrap();
    let get_nym_resp_data: ReplyResultData = serde_json::from_str(&get_nym_resp.result.data.as_ref().unwrap()).unwrap();
    info!("get_nym_resp {:?}\n{:?}\n{:?}", resp, get_nym_resp, get_nym_resp_data);

    assert_eq!(get_nym_resp_data.dest, my_did);

    // 13. Close pool
    let res = indy_close_pool_ledger(close_pool_command_handle, pool_handle, close_pool_callback);
    assert_eq!(res, ErrorCode::Success);
    let res = close_pool_receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
    assert_eq!(res, ErrorCode::Success);

    // 14. Close my wallet
    let res = indy_close_wallet(close_my_wallet_command_handle, my_wallet_handle, close_my_wallet_callback);
    assert_eq!(res, ErrorCode::Success);
    let res = close_my_wallet_receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
    assert_eq!(res, ErrorCode::Success);

    // 15. Close their wallet
    let res = indy_close_wallet(close_their_wallet_command_handle, their_wallet_handle, close_their_wallet_callback);
    assert_eq!(res, ErrorCode::Success);
    let res = close_their_wallet_receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
    assert_eq!(res, ErrorCode::Success);

    TestUtils::cleanup_storage();

    #[derive(Serialize, Eq, PartialEq, Debug)]
    #[serde(rename_all = "camelCase")]
    struct Request {
        req_id: u64,
        identifier: String,
        operation: Operation,
        protocol_version: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
    }

    #[derive(Serialize, Eq, PartialEq, Debug)]
    struct Operation {
        #[serde(rename = "type")]
        type_: String,
        dest: String,
    }

    #[derive(Deserialize, Eq, PartialEq, Debug)]
    struct Reply {
        op: String,
        result: ReplyResult,
    }

    #[derive(Deserialize, Eq, PartialEq, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ReplyResult {
        identifier: String,
        req_id: u64,
        data: Option<String>
    }

    #[derive(Deserialize, Eq, PartialEq, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ReplyResultData {
        dest: String,
        identifier: String,
        role: Option<String>,
    }
}

#[test]
fn crypto_demo_works() {
    TestUtils::cleanup_storage();

    let (create_wallet_sender, create_wallet_receiver) = channel();
    let (open_wallet_sender, open_wallet_receiver) = channel();
    let (create_and_store_did_sender, create_and_store_did_receiver) = channel();
    let (sign_sender, sign_receiver) = channel();
    let (verify_sender, verify_receiver) = channel();
    let (close_wallet_sender, close_wallet_receiver) = channel();

    let create_wallet_cb = Box::new(move |err| { create_wallet_sender.send(err).unwrap(); });
    let open_wallet_cb = Box::new(move |err, handle| { open_wallet_sender.send((err, handle)).unwrap(); });
    let create_and_store_did_cb = Box::new(move |err, did, verkey| { create_and_store_did_sender.send((err, did, verkey)).unwrap(); });
    let sign_cb = Box::new(move |err, signature| { sign_sender.send((err, signature)).unwrap(); });
    let verify_cb = Box::new(move |err, valid| { verify_sender.send((err, valid)).unwrap(); });
    let close_wallet_cb = Box::new(move |err_code| close_wallet_sender.send(err_code).unwrap());

    let (create_wallet_command_handle, create_wallet_callback) = CallbackUtils::closure_to_create_wallet_cb(create_wallet_cb);
    let (open_wallet_command_handle, open_wallet_callback) = CallbackUtils::closure_to_open_wallet_cb(open_wallet_cb);
    let (create_and_store_did_command_handle, create_and_store_did_callback) = CallbackUtils::closure_to_create_and_store_my_did_cb(create_and_store_did_cb);
    let (sign_command_handle, sign_callback) = CallbackUtils::closure_to_sign_cb(sign_cb);
    let (verify_command_handle, verify_callback) = CallbackUtils::closure_to_verify_signature_cb(verify_cb);
    let (close_wallet_command_handle, close_wallet_callback) = CallbackUtils::closure_to_delete_wallet_cb(close_wallet_cb);

    let pool_name = "pool_1";
    let wallet_name = "wallet_1";
    let xtype = "default";

    // 1. Create Wallet
    let err =
        indy_create_wallet(create_wallet_command_handle,
                           CString::new(pool_name).unwrap().as_ptr(),
                           CString::new(wallet_name).unwrap().as_ptr(),
                           CString::new(xtype).unwrap().as_ptr(),
                           null(),
                           null(),
                           create_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let err = create_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 2. Open Wallet. Gets wallet handle
    let err =
        indy_open_wallet(open_wallet_command_handle,
                         CString::new(wallet_name).unwrap().as_ptr(),
                         null(),
                         null(),
                         open_wallet_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, wallet_handle) = open_wallet_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 3. Create DID
    let did_json = "{}";
    let err =
        indy_create_and_store_my_did(create_and_store_did_command_handle,
                                     wallet_handle,
                                     CString::new(did_json).unwrap().as_ptr(),
                                     create_and_store_did_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, _, verkey) = create_and_store_did_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 4. Sign message
    let message = r#"{
        "reqId":1495034346617224651,
        "identifier":"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
        "operation":{
            "type":"1",
            "dest":"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB"
        }
    }"#;

    let message_ptr = message.as_ptr() as *const u8;
    let message_len = message.len() as u32;

    let err =
        indy_crypto_sign(sign_command_handle,
                         wallet_handle,
                         CString::new(verkey.clone()).unwrap().as_ptr(),
                         message_ptr,
                         message_len,
                         sign_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, signature) = sign_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert_eq!(ErrorCode::Success, err);

    // 5. Verify message
    let err =
        indy_crypto_verify(verify_command_handle,
                           CString::new(verkey).unwrap().as_ptr(),
                           message_ptr,
                           message_len,
                           signature.as_ptr() as *const u8,
                           signature.len() as u32,
                           verify_callback);

    assert_eq!(ErrorCode::Success, err);
    let (err, valid) = verify_receiver.recv_timeout(TimeoutUtils::long_timeout()).unwrap();
    assert!(valid);
    assert_eq!(ErrorCode::Success, err);

    // 6. Close Wallet
    let res = indy_close_wallet(close_wallet_command_handle, wallet_handle, close_wallet_callback);
    assert_eq!(res, ErrorCode::Success);
    let res = close_wallet_receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();
    assert_eq!(res, ErrorCode::Success);

    TestUtils::cleanup_storage();
}
