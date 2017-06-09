// TODO: FIXME: It must be removed after code layout stabilization!
#![allow(dead_code)]
extern crate sovrin;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
#[path = "utils/mod.rs"]
mod utils;

use utils::wallet::WalletUtils;
use utils::anoncreds::AnoncredsUtils;
use utils::anoncreds::{ClaimOffer, ProofClaimsJson};
use utils::test::TestUtils;
use std::collections::HashMap;

#[test]
fn anoncreds_works_for_single_issuer_single_prover() {
    TestUtils::cleanup_storage();

    let pool_name = "pool1";
    let issuer_wallet_name = "issuer_wallet";
    let prover_wallet_name = "prover_wallet";
    let xtype = "default";

    //1. Create Issuer wallet, get wallet handle
    let res = WalletUtils::create_and_open_wallet(pool_name, issuer_wallet_name, xtype);
    assert!(res.is_ok());
    let issuer_wallet_handle = res.unwrap();

    //2. Create Prover wallet, get wallet handle
    let res = WalletUtils::create_and_open_wallet(pool_name, prover_wallet_name, xtype);
    assert!(res.is_ok());
    let prover_wallet_handle = res.unwrap();

    //3. Issuer create claim definition
    let issuer_did = "some_issuer_did";
    let schema_seq_no = 1;
    let claim_def_seq_no = 1;
    let schema = AnoncredsUtils::get_gvt_schema_json(schema_seq_no);

    let res = AnoncredsUtils::create_claim_definition_and_set_link(issuer_wallet_handle, &schema, claim_def_seq_no);
    assert!(res.is_ok());
    let claim_def_json = res.unwrap();

    //4. Prover create Master Secret
    let master_secret_name = "prover_master_secret";

    let res = AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name);
    assert!(res.is_ok());

    //5. Prover store Claim Offer received from Issuer
    let claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, claim_def_seq_no);

    let res = AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json);
    assert!(res.is_ok());

    //6. Prover get Claim Offers
    let filter_json = format!("{{ \"issuer_did\":\"{}\"}}", issuer_did);

    let res = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json);
    assert!(res.is_ok());
    let claim_offers_json = res.unwrap();

    let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
    assert!(claim_offers.len() == 1);
    let claim_offer_json = serde_json::to_string(&claim_offers[0]).unwrap();

    //7. Prover create Claim Request
    let prover_did = "some_prover_did";
    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer_json,
                                                                &claim_def_json,
                                                                master_secret_name);
    assert!(res.is_ok());
    let claim_req = res.unwrap();

    //8. Issuer create Claim
    let claim_json = AnoncredsUtils::get_gvt_claim_json();
    let res = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                  &claim_req,
                                                  &claim_json);
    assert!(res.is_ok());
    let (_, xclaim_json) = res.unwrap();

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
    let claims: ProofClaimsJson = serde_json::from_str(&claims_json).unwrap();

    let claims_for_attr_1 = claims.attrs.get("attr1_uuid").unwrap();
    assert_eq!(1, claims_for_attr_1.len());
    let claim = claims_for_attr_1[0].clone();

    // 11. Prover create Proof
    let requested_claims_json = format!("{{\
                                          \"self_attested_attributes\":{{}},\
                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true]}},\
                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{}\"}}\
                                        }}", claim.claim_uuid, claim.claim_uuid);

    let schemas_json = format!("{{\"{}\":{}}}", claim.claim_uuid, schema);
    let claim_defs_json = format!("{{\"{}\":{}}}", claim.claim_uuid, claim_def_json);
    let revoc_regs_jsons = "{}";

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
    let res = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                    &proof_json,
                                                    &schemas_json,
                                                    &claim_defs_json,
                                                    &revoc_regs_jsons);
    assert!(res.is_ok());
    assert!(res.unwrap());

    TestUtils::cleanup_storage();
}

#[test]
fn anoncreds_works_for_multiply_issuer_single_prover() {
    TestUtils::cleanup_storage();

    let issuer1_did = "some_issuer1_did";
    let issuer2_did = "some_issuer2_did";
    let prover_did = "some_prover_did";

    let pool_name = "pool1";
    let issuer1_wallet_name = "issuer1_wallet";
    let issuer2_wallet_name = "issuer2_wallet";
    let prover_wallet_name = "prover_wallet";
    let xtype = "default";

    //1. Issuer1 create wallet, get wallet handles
    let res = WalletUtils::create_and_open_wallet(pool_name, issuer1_wallet_name, xtype);
    assert!(res.is_ok());
    let issuer_gvt_wallet_handle = res.unwrap();

    //2. Issuer2 create wallet, get wallet handles
    let res = WalletUtils::create_and_open_wallet(pool_name, issuer2_wallet_name, xtype);
    assert!(res.is_ok());
    let issuer_xyz_wallet_handle = res.unwrap();

    //3. Prover create wallet, get wallet handles
    let res = WalletUtils::create_and_open_wallet(pool_name, prover_wallet_name, xtype);
    assert!(res.is_ok());
    let prover_wallet_handle = res.unwrap();

    let mut schemas: HashMap<i32, String> = HashMap::new();
    let mut claim_defs: HashMap<i32, String> = HashMap::new();

    //4. Issuer1 create claim definition by gvt schema
    let gvt_schema_seq_no = 1;
    let gvt_claim_def_seq_no = 1;
    let gvt_schema = AnoncredsUtils::get_gvt_schema_json(gvt_schema_seq_no);

    let res = AnoncredsUtils::create_claim_definition_and_set_link(issuer_gvt_wallet_handle, &gvt_schema, gvt_claim_def_seq_no);
    assert!(res.is_ok());
    let gvt_claim_def_json = res.unwrap();

    schemas.insert(gvt_schema_seq_no, gvt_schema.clone());
    claim_defs.insert(gvt_claim_def_seq_no, gvt_claim_def_json.clone());


    //5. Issuer1 create claim definition by xyz schema
    let xyz_schema_seq_no = 2;
    let xyz_claim_def_seq_no = 2;
    let xyz_schema = AnoncredsUtils::get_xyz_schema_json(xyz_schema_seq_no);

    let res = AnoncredsUtils::create_claim_definition_and_set_link(issuer_xyz_wallet_handle, &xyz_schema, xyz_claim_def_seq_no);
    assert!(res.is_ok());
    let xyz_claim_def_json = res.unwrap();

    schemas.insert(xyz_schema_seq_no, xyz_schema.clone());
    claim_defs.insert(xyz_claim_def_seq_no, xyz_claim_def_json.clone());

    //6. Prover create Master Secret for Issuer1
    let master_secret_name_1 = "prover_master_secret_issuer_1";

    let res = AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name_1);
    assert!(res.is_ok());

    //7. Prover create Master Secret for Issuer2
    let master_secret_name_2 = "prover_master_secret_issuer_2";

    let res = AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name_2);
    assert!(res.is_ok());

    //8. Prover store Claim Offer received from Issuer1
    let issuer1_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer1_did, gvt_claim_def_seq_no);

    let res = AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer1_claim_offer_json);
    assert!(res.is_ok());

    //9. Prover store Claim Offer received from Issuer2
    let issuer2_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer2_did, xyz_claim_def_seq_no);

    let res = AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer2_claim_offer_json);
    assert!(res.is_ok());

    //10. Prover get Claim Offers
    let filter_json = "{}";

    let res = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json);
    assert!(res.is_ok());
    let claim_offers_json = res.unwrap();

    let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
    assert_eq!(2, claim_offers.len());

    let claim_offer_1 = claim_offers[0].clone();
    let claim_offer_2 = claim_offers[1].clone();

    let claim_offer_1_json = serde_json::to_string(&claim_offer_1).unwrap();
    let claim_offer_2_json = serde_json::to_string(&claim_offer_2).unwrap();

    //11. Prover create Claim Request for gvt claim offer
    let claim_offer = if claim_offer_1.claim_def_seq_no == gvt_claim_def_seq_no { claim_offer_1_json.clone() } else { claim_offer_2_json.clone() };

    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer,
                                                                &gvt_claim_def_json,
                                                                master_secret_name_1);
    assert!(res.is_ok());
    let gvt_claim_req = res.unwrap();

    //12. Issuer create GVT Claim
    let gvt_claim_json = AnoncredsUtils::get_gvt_claim_json();
    let res = AnoncredsUtils::issuer_create_claim(issuer_gvt_wallet_handle,
                                                  &gvt_claim_req,
                                                  &gvt_claim_json);
    assert!(res.is_ok());
    let (_, gvt_claim_json) = res.unwrap();

    //13. Prover store received GVT Claim
    let res = AnoncredsUtils::prover_store_claim(prover_wallet_handle, &gvt_claim_json);
    assert!(res.is_ok());

    //14. Prover create Claim Request for xyz claim offer
    let claim_offer = if claim_offer_2.claim_def_seq_no == xyz_claim_def_seq_no { claim_offer_2_json.clone() } else { claim_offer_1_json.clone() };
    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer,
                                                                &xyz_claim_def_json,
                                                                master_secret_name_1);
    assert!(res.is_ok());
    let xyz_claim_req = res.unwrap();

    //15. Issuer create XYZ Claim
    let xyz_claim_json = AnoncredsUtils::get_xyz_claim_json();
    let res = AnoncredsUtils::issuer_create_claim(issuer_xyz_wallet_handle,
                                                  &xyz_claim_req,
                                                  &xyz_claim_json);
    assert!(res.is_ok());
    let (_, xyz_claim_json) = res.unwrap();

    // 16. Prover store received XYZ Claim
    let res = AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xyz_claim_json);
    assert!(res.is_ok());


    // 17. Prover gets Claims for Proof Request
    let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}},\
                                                         \"attr2_uuid\":{{\"schema_seq_no\":{},\"name\":\"status\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}},\
                                                              \"predicate2_uuid\":{{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}}}\
                                }}", gvt_schema_seq_no, xyz_schema_seq_no);

    let res = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json);
    assert!(res.is_ok());
    let claims_json = res.unwrap();

    let claims: ProofClaimsJson = serde_json::from_str(&claims_json).unwrap();

    let claims_for_attr_1 = claims.attrs.get("attr1_uuid").unwrap();
    let claims_for_attr_2 = claims.attrs.get("attr2_uuid").unwrap();
    assert_eq!(1, claims_for_attr_1.len());
    assert_eq!(1, claims_for_attr_2.len());

    let claim_for_attr_1 = claims_for_attr_1[0].clone();
    let claim_for_attr_2 = claims_for_attr_2[0].clone();

    let claims_for_predicate_1 = claims.predicates.get("predicate1_uuid").unwrap();
    let claims_for_predicate_2 = claims.predicates.get("predicate2_uuid").unwrap();
    assert_eq!(1, claims_for_predicate_1.len());
    assert_eq!(1, claims_for_predicate_2.len());

    let claim_for_predicate_1 = claims_for_predicate_1[0].clone();
    let claim_for_predicate_2 = claims_for_predicate_2[0].clone();


    // 18. Prover create Proof
    let requested_claims_json = format!("{{\
                                          \"self_attested_attributes\":{{}},\
                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true],\
                                                                \"attr2_uuid\":[\"{}\",true]}},\
                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{}\", \
                                                                     \"predicate2_uuid\":\"{}\"}}\
                                        }}",
                                        claim_for_attr_1.claim_uuid, claim_for_attr_2.claim_uuid,
                                        claim_for_predicate_1.claim_uuid, claim_for_predicate_2.claim_uuid);

    let unique_claims = AnoncredsUtils::get_unique_claims(&claims);

    let schemas_json = format!("{{\"{}\":{}, \"{}\":{}}}",
                               unique_claims[0].claim_uuid,
                               schemas.get(&unique_claims[0].schema_seq_no).unwrap(),
                               unique_claims[1].claim_uuid,
                               schemas.get(&unique_claims[1].schema_seq_no).unwrap());


    let claim_defs_json = format!("{{\"{}\":{}, \"{}\":{}}}",
                                  unique_claims[0].claim_uuid,
                                  claim_defs.get(&unique_claims[0].claim_def_seq_no).unwrap(),
                                  unique_claims[1].claim_uuid,
                                  claim_defs.get(&unique_claims[1].claim_def_seq_no).unwrap());
    let revoc_regs_jsons = "{}";


    let res = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                  &proof_req_json,
                                                  &requested_claims_json,
                                                  &schemas_json,
                                                  &master_secret_name_1,
                                                  &claim_defs_json,
                                                  &revoc_regs_jsons);
    assert!(res.is_ok());
    let proof_json = res.unwrap();

    // 19. Verifier verify proof
    let res = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                    &proof_json,
                                                    &schemas_json,
                                                    &claim_defs_json,
                                                    &revoc_regs_jsons);
    assert!(res.is_ok());
    assert!(res.unwrap());

    TestUtils::cleanup_storage();
}

#[test]
fn anoncreds_works_for_single_issuer_multiply_claims_single_prover() {
    TestUtils::cleanup_storage();

    let issuer_did = "some_issuer1_did";
    let prover_did = "some_prover_did";

    let pool_name = "pool1";
    let issuer_wallet_name = "issuer_wallet";
    let prover_wallet_name = "prover_wallet";
    let xtype = "default";

    //1. Issuer create wallet, get wallet handles
    let res = WalletUtils::create_and_open_wallet(pool_name, issuer_wallet_name, xtype);
    assert!(res.is_ok());
    let issuer_wallet_handle = res.unwrap();

    //2. Prover create wallet, get wallet handles
    let res = WalletUtils::create_and_open_wallet(pool_name, prover_wallet_name, xtype);
    assert!(res.is_ok());
    let prover_wallet_handle = res.unwrap();

    let mut schemas: HashMap<i32, String> = HashMap::new();
    let mut claim_defs: HashMap<i32, String> = HashMap::new();

    //3. Issuer create claim definition by gvt schema
    let gvt_schema_seq_no = 1;
    let gvt_claim_def_seq_no = 1;
    let gvt_schema = AnoncredsUtils::get_gvt_schema_json(gvt_schema_seq_no);

    let res = AnoncredsUtils::create_claim_definition_and_set_link(issuer_wallet_handle, &gvt_schema, gvt_claim_def_seq_no);
    assert!(res.is_ok());
    let gvt_claim_def_json = res.unwrap();

    schemas.insert(gvt_schema_seq_no, gvt_schema.clone());
    claim_defs.insert(gvt_claim_def_seq_no, gvt_claim_def_json.clone());

    //4. Issuer create claim definition by xyz schema
    let xyz_schema_seq_no = 2;
    let xyz_claim_def_seq_no = 2;
    let xyz_schema = AnoncredsUtils::get_xyz_schema_json(xyz_schema_seq_no);

    let res = AnoncredsUtils::create_claim_definition_and_set_link(issuer_wallet_handle, &xyz_schema, xyz_claim_def_seq_no);
    assert!(res.is_ok());
    let xyz_claim_def_json = res.unwrap();

    schemas.insert(xyz_schema_seq_no, xyz_schema.clone());
    claim_defs.insert(xyz_claim_def_seq_no, xyz_claim_def_json.clone());

    //5. Prover create Master Secret for Issuer1
    let master_secret_name = "prover_master_secret_issuer";

    let res = AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name);
    assert!(res.is_ok());

    //6. Prover store GVT Claim Offer received from Issuer
    let issuer_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, gvt_claim_def_seq_no);

    let res = AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer_claim_offer_json);
    assert!(res.is_ok());

    //7. Prover store XYZ Claim Offer received from Issuer
    let issuer_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, xyz_claim_def_seq_no);

    let res = AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer_claim_offer_json);
    assert!(res.is_ok());

    //8. Prover get Claim Offers
    let filter_json = format!("{{ \"issuer_did\":\"{}\"}}", issuer_did);

    let res = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json);
    assert!(res.is_ok());
    let claim_offers_json = res.unwrap();

    let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
    assert_eq!(2, claim_offers.len());

    let claim_offer_1 = claim_offers[0].clone();
    let claim_offer_2 = claim_offers[1].clone();

    let claim_offer_1_json = serde_json::to_string(&claim_offer_1).unwrap();
    let claim_offer_2_json = serde_json::to_string(&claim_offer_2).unwrap();

    //9. Prover create Claim Request for gvt claim offer
    let claim_offer = if claim_offer_1.claim_def_seq_no == gvt_claim_def_seq_no { claim_offer_1_json.clone() } else { claim_offer_2_json.clone() };

    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer,
                                                                &gvt_claim_def_json,
                                                                master_secret_name);
    assert!(res.is_ok());
    let gvt_claim_req = res.unwrap();

    //10. Issuer create GVT Claim
    let gvt_claim_json = AnoncredsUtils::get_gvt_claim_json();
    let res = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                  &gvt_claim_req,
                                                  &gvt_claim_json);
    assert!(res.is_ok());
    let (_, gvt_claim_json) = res.unwrap();

    //11. Prover store received GVT Claim
    let res = AnoncredsUtils::prover_store_claim(prover_wallet_handle, &gvt_claim_json);
    assert!(res.is_ok());

    //12. Prover create Claim Request for xyz claim offer
    let claim_offer = if claim_offer_2.claim_def_seq_no == xyz_claim_def_seq_no { claim_offer_2_json.clone() } else { claim_offer_1_json.clone() };
    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer,
                                                                &xyz_claim_def_json,
                                                                master_secret_name);
    assert!(res.is_ok());
    let xyz_claim_req = res.unwrap();

    //13. Issuer create XYZ Claim
    let xyz_claim_json = AnoncredsUtils::get_xyz_claim_json();
    let res = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                  &xyz_claim_req,
                                                  &xyz_claim_json);
    assert!(res.is_ok());
    let (_, xyz_claim_json) = res.unwrap();

    //14. Prover store received XYZ Claim
    let res = AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xyz_claim_json);
    assert!(res.is_ok());


    //15. Prover gets Claims for Proof Request
    let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}},\
                                                              \"predicate2_uuid\":{{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}}}\
                                }}", gvt_schema_seq_no);

    let res = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json);
    assert!(res.is_ok());
    let claims_json = res.unwrap();

    let claims: ProofClaimsJson = serde_json::from_str(&claims_json).unwrap();

    assert_eq!(1, claims.attrs.len());
    assert_eq!(2, claims.predicates.len());

    let claims_for_attr_1 = claims.attrs.get("attr1_uuid").unwrap();
    assert_eq!(1, claims_for_attr_1.len());

    let claim_for_attr_1 = claims_for_attr_1[0].clone();

    let claims_for_predicate_1 = claims.predicates.get("predicate1_uuid").unwrap();
    let claims_for_predicate_2 = claims.predicates.get("predicate2_uuid").unwrap();

    assert_eq!(1, claims_for_predicate_1.len());
    assert_eq!(1, claims_for_predicate_2.len());

    let claim_for_predicate_1 = claims_for_predicate_1[0].clone();
    let claim_for_predicate_2 = claims_for_predicate_2[0].clone();

    //16. Prover create Proof
    let requested_claims_json = format!("{{\
                                          \"self_attested_attributes\":{{}},\
                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true]}},\
                                          \"requested_predicates\":{{\"predicate1_uuid\":\"{}\", \
                                                                     \"predicate2_uuid\":\"{}\"}}\
                                        }}",
                                        claim_for_attr_1.claim_uuid,
                                        claim_for_predicate_1.claim_uuid, claim_for_predicate_2.claim_uuid);

    let unique_claims = AnoncredsUtils::get_unique_claims(&claims);

    let schemas_json = format!("{{\"{}\":{}, \"{}\":{}}}",
                               unique_claims[0].claim_uuid,
                               schemas.get(&unique_claims[0].schema_seq_no).unwrap(),
                               unique_claims[1].claim_uuid,
                               schemas.get(&unique_claims[1].schema_seq_no).unwrap());


    let claim_defs_json = format!("{{\"{}\":{}, \"{}\":{}}}",
                                  unique_claims[0].claim_uuid,
                                  claim_defs.get(&unique_claims[0].claim_def_seq_no).unwrap(),
                                  unique_claims[1].claim_uuid,
                                  claim_defs.get(&unique_claims[1].claim_def_seq_no).unwrap());
    let revoc_regs_jsons = "{}";

    let res = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                  &proof_req_json,
                                                  &requested_claims_json,
                                                  &schemas_json,
                                                  &master_secret_name,
                                                  &claim_defs_json,
                                                  &revoc_regs_jsons);
    //assert!(res.is_ok());
    let proof_json = res.unwrap();

    //17. Verifier verify proof
    let res = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                    &proof_json,
                                                    &schemas_json,
                                                    &claim_defs_json,
                                                    &revoc_regs_jsons);
    assert!(res.is_ok());
    assert!(res.unwrap());

    TestUtils::cleanup_storage();
}