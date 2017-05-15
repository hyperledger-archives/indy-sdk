// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::wallet::WalletUtils;
use utils::anoncreds::AnoncredsUtils;
use utils::test::TestUtils;
use utils::timeout::TimeoutUtils;

use sovrin::api::ErrorCode;

use utils::callback::CallbackUtils;

use std::ptr::null;
use std::sync::mpsc::{channel};
use std::ffi::{CString};

#[test]
fn anoncreds_single_issuer_single_prover() {
    TestUtils::cleanup_storage();

    let pool_name = "pool1";
    let issuer_wallet_name = "issuer_wallet";
    let prover_wallet_name = "prover_wallet";
    let verifier_wallet_name = "verifier_wallet";
    let xtype = "default";

    //1. Create Issuer, Prover, Verifier wallets, get wallet handles
    let res = AnoncredsUtils::create_issuer_prover_verifier_wallets(pool_name,
                                                                    issuer_wallet_name,
                                                                    prover_wallet_name,
                                                                    verifier_wallet_name,
                                                                    xtype);
    assert!(res.is_ok());

    let (issuer_wallet_handle,prover_wallet_handle,verifier_wallet_handle) = res.unwrap();

    //2. Issuer create claim definition
    let issuer_did = "some_issuer_did";
    let schema_seq_no = 1;
    let schema = format!("{{\
                            \"name\":\"gvt\",\
                            \"version\":\"1.0\",\
                            \"attribute_names\":[\"age\",\"sex\",\"height\",\"name\"],\
                            \"seq_no\":{}\
                         }}", schema_seq_no);

    let res = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, &schema);
    assert!(res.is_ok());
    let (claim_def_json, claim_def_uuid) = res.unwrap();

    //3 Set link between  claim_def_seq_no and claim_def_uuid
    let claim_def_seq_no = 1;
    let res = WalletUtils::wallet_set_seq_no_for_value(issuer_wallet_handle, &claim_def_uuid, claim_def_seq_no);
    assert!(res.is_ok());

    //4 Prover create Master Secret
    let master_secret_name = "prover_master_secret";

    let res = AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name);
    assert!(res.is_ok());

    //5 Prover store Claim Offer received from Issuer
    let claim_offer_json = format!("{{ \"issuer_did\":\"{}\", \"claim_def_seq_no\":{} }}", issuer_did, claim_def_seq_no);

    let res = AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json);
    assert!(res.is_ok());

    //6 Prover get Claim Offers
    let filter_json = format!("{{ \"issuer_did\":\"{}\"}}", issuer_did);

    let res = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json);
    assert!(res.is_ok());
    let claim_offers_json = res.unwrap();

    let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
    assert!(claim_offers.len() > 0);
    let claim_offer = claim_offers[0].clone();

    let claim_offer_json = serde_json::to_string(&claim_offer).unwrap();

    //7 Prover create Claim Request
    let prover_did = "some_prover_did";
    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer_json,
                                                                &claim_def_json,
                                                                master_secret_name);
    assert!(res.is_ok());
    let claim_req = res.unwrap();

    //8 Issuer create Claim
    let claim_json = "{\
                           \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
                           \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
                           \"height\":[\"175\",\"175\"],\
                           \"age\":[\"28\",\"28\"]\
                     }";
    let res = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                  &claim_req,
                                                  &claim_json);
    assert!(res.is_ok());
    let (revoc_reg_update_json, xclaim_json) = res.unwrap();

    // 9. Prover store received Claim
    let res = AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xclaim_json);
    res.unwrap();
    assert!(res.is_ok());

    // 10. Prover gets Claims for Proof Request
    let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}}}\
                                }}", schema_seq_no);

    let res = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json);
    assert!(res.is_ok());
    let claims_json = res.unwrap();

    let requested_claims_json = format!("{{\
                                          \"self_attested_attributes\":{{}},\
                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true]}},\
                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{}\"}}\
                                        }}", claim_def_seq_no, claim_def_seq_no);

    // 11. Prover create Proof
    let schemas_json = format!("{{\"{}\":{}}}", claim_def_seq_no, schema);
    let claim_defs_json = format!("{{\"{}\":{}}}", claim_def_seq_no, claim_def_json);
    let revoc_regs_jsons = "{}";

    let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}}}\
                                }}", schema_seq_no);

    let res = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                  &proof_req_json,
                                                  &requested_claims_json,
                                                  &schemas_json,
                                                  &master_secret_name,
                                                  &claim_defs_json,
                                                  &revoc_regs_jsons);
    assert!(res.is_ok());
    let proof_json = res.unwrap();

    // 12. Verifier verify proof
    let res = AnoncredsUtils::verifier_verify_proof(prover_wallet_handle,
                                                    &proof_req_json,
                                                    &proof_json,
                                                    &schemas_json,
                                                    &claim_defs_json,
                                                    &revoc_regs_jsons);
    assert!(res.is_ok());
    assert!(res.unwrap());

    TestUtils::cleanup_storage();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ClaimOffer {
    pub issuer_did: String,
    pub claim_def_seq_no: i32
}
