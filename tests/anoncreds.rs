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
use utils::test::TestUtils;
use std::collections::HashMap;
use utils::types::{
    ClaimDefinition,
    ClaimOffer,
    ProofClaimsJson,
    ClaimRequestJson
};

use sovrin::api::ErrorCode;


mod high_cases {
    use super::*;

    mod issuer_create_and_store_claim_def {
        use super::*;

        #[test]
        fn issuer_create_and_store_claim_def_works() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let schema = AnoncredsUtils::get_gvt_schema_json(1);

            let (claim_def_json, _) = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &schema, None, false).unwrap();

            let claim_def: ClaimDefinition = serde_json::from_str(&claim_def_json).unwrap();

            assert!(claim_def.public_key.r.len() == 4);
            assert!(claim_def.public_key.n.len() > 0);
            assert!(claim_def.public_key.s.len() > 0);
            assert!(claim_def.public_key.rms.len() > 0);
            assert!(claim_def.public_key.z.len() > 0);
            assert!(claim_def.public_key.rctxt.len() > 0);
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_invalid_schema() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let schema = r#"{"name":"name","version":"1.0"}"#;

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &schema, None, false);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_empty_schema_keys() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let schema = r#"{"name":"name","version":"1.0","seq_no":1,"keys":[]}"#;

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &schema, None, false);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_invalid_signature_type() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let schema = AnoncredsUtils::get_gvt_schema_json(1);

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &schema, Some("some_type"), false);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_invalid_wallet() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let schema = AnoncredsUtils::get_gvt_schema_json(1);

            let invalid_wallet_handle = wallet_handle + 1;
            let res = AnoncredsUtils::issuer_create_claim_definition(invalid_wallet_handle, &schema, None, false);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_store_claim_offer {
        use super::*;

        #[test]
        fn prover_store_claim_offer_works() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("HEJ9gvWX64wW7UD", 10, 10);

            AnoncredsUtils::prover_store_claim_offer(wallet_handle, &claim_offer_json).unwrap();
        }

        #[test]
        fn prover_store_claim_offer_works_for_invalid_json() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = r#"{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e"}"#;

            let res = AnoncredsUtils::prover_store_claim_offer(wallet_handle, &claim_offer_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_store_claim_offer_works_for_invalid_issuer_did() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = r#"{"issuer_did":"invalid_base58_string"}"#;

            let res = AnoncredsUtils::prover_store_claim_offer(wallet_handle, &claim_offer_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_store_claim_offer_works_for_invalid_wallet() {
            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);

            let res = AnoncredsUtils::prover_store_claim_offer(0, &claim_offer_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_get_claim_offers {
        use super::*;

        #[test]
        fn prover_get_claim_offers_works_for_empty_filter() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{}"#).unwrap();
            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 3);
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_issuer() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e"}"#).unwrap();
            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 2);
            assert!(claim_offers.contains(&ClaimOffer { issuer_did: "NcYxiDXkpYi6ov5FcYDi1e".to_string(), claim_def_seq_no: 1, schema_seq_no: 1 }));
            assert!(claim_offers.contains(&ClaimOffer { issuer_did: "NcYxiDXkpYi6ov5FcYDi1e".to_string(), claim_def_seq_no: 2, schema_seq_no: 2 }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_claim_def() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"claim_def_seq_no":2}"#).unwrap();
            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 1);
            assert!(claim_offers.contains(&ClaimOffer { issuer_did: "NcYxiDXkpYi6ov5FcYDi1e".to_string(), claim_def_seq_no: 2, schema_seq_no: 2 }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_schema() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"schema_seq_no":2}"#).unwrap();
            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 2);
            assert!(claim_offers.contains(&ClaimOffer { issuer_did: "NcYxiDXkpYi6ov5FcYDi1e".to_string(), claim_def_seq_no: 2, schema_seq_no: 2 }));
            assert!(claim_offers.contains(&ClaimOffer { issuer_did: "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW".to_string(), claim_def_seq_no: 3, schema_seq_no: 2 }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_issuer_and_claim_def() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","claim_def_seq_no":1}"#).unwrap();
            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 1);
            assert!(claim_offers.contains(&ClaimOffer { issuer_did: "NcYxiDXkpYi6ov5FcYDi1e".to_string(), claim_def_seq_no: 1, schema_seq_no: 1 }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_no_results() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"claim_def_seq_no":4}"#).unwrap();
            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 0);
        }

        #[test]
        fn prover_get_claim_offers_works_for_invalid_filter_json() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let res = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"claim_def_seq_no":"1"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_get_claim_offers_works_for_invalid_wallet_handle() {
            AnoncredsUtils::init_common_wallet();

            let res = AnoncredsUtils::prover_get_claim_offers(0, r#"{"claim_def_seq_no":"1"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }

        #[test]
        fn prover_get_claim_offers_works_for_different_wallets() {
            AnoncredsUtils::init_common_wallet();

            let wallet_handle_1 = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();
            let wallet_handle_2 = WalletUtils::create_and_open_wallet("pool1", "wallet2", "default").unwrap();

            let claim_offer_json_1 = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);
            let claim_offer_json_2 = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 2, 2);
            let claim_offer_json_3 = AnoncredsUtils::get_claim_offer("CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW", 3, 2);

            AnoncredsUtils::prover_store_claim_offer(wallet_handle_1, &claim_offer_json_1).unwrap();
            AnoncredsUtils::prover_store_claim_offer(wallet_handle_1, &claim_offer_json_2).unwrap();
            AnoncredsUtils::prover_store_claim_offer(wallet_handle_2, &claim_offer_json_3).unwrap();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle_1, r#"{"claim_def_seq_no":2}"#).unwrap();
            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 1);
            assert!(claim_offers.contains(&ClaimOffer { issuer_did: "NcYxiDXkpYi6ov5FcYDi1e".to_string(), claim_def_seq_no: 2, schema_seq_no: 2 }));
        }
    }

    mod prover_create_master_secret {
        use super::*;

        #[test]
        fn prover_create_master_secret_works() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            AnoncredsUtils::prover_create_master_secret(wallet_handle, "master_secret_name1").unwrap();
        }

        #[test]
        fn prover_create_master_secret_works_for_duplicate_name() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            AnoncredsUtils::prover_create_master_secret(wallet_handle, "master_secret_name2").unwrap();
            let res = AnoncredsUtils::prover_create_master_secret(wallet_handle, "master_secret_name2");
            assert_eq!(res.unwrap_err(), ErrorCode::AnoncredsMasterSecretDuplicateNameError);
        }

        #[test]
        fn prover_create_master_secret_works_invalid_wallet_handle() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = AnoncredsUtils::prover_create_master_secret(invalid_wallet_handle, "master_secret_name2");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_create_and_store_claim_req {
        use super::*;

        #[test]
        fn prover_create_and_store_claim_req_works() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e";
            let claim_def_seq_no = 1;
            let claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, claim_def_seq_no, 1);
            let claim_def = AnoncredsUtils::get_gvt_claim_def();

            let claim_req_json = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                                   "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                                   &claim_offer_json,
                                                                                   &claim_def,
                                                                                   "common_master_secret_name").unwrap();

            let claim_req: ClaimRequestJson = serde_json::from_str(&claim_req_json).unwrap();

            assert_eq!(claim_req.claim_def_seq_no, claim_def_seq_no);
            assert_eq!(claim_req.issuer_did, issuer_did);
            assert!(claim_req.claim_request.u.len() > 0);
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_claim_offer() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = r#"{"claim_def_seq_no":1}"#;
            let claim_def = AnoncredsUtils::get_gvt_claim_def();

            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                        claim_offer_json,
                                                                        &claim_def,
                                                                        "common_master_secret_name");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_claim_def() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);
            let claim_def = r#"{
                        "schema_seq_no":1,
                        "signature_type":"CL",
                        "public_key":{
                            "n":"121212",
                            "s":"432192"
                        }
                    }"#;

            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                        &claim_offer_json,
                                                                        claim_def,
                                                                        "common_master_secret_name");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_master_secret() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);
            let claim_def = AnoncredsUtils::get_gvt_claim_def();

            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                        &claim_offer_json,
                                                                        &claim_def,
                                                                        "invalid_master_secret_name");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_wallet() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);
            let claim_def = AnoncredsUtils::get_gvt_claim_def();

            let invalid_wallet_handle = wallet_handle + 1;
            let res = AnoncredsUtils::prover_create_and_store_claim_req(invalid_wallet_handle,
                                                                        "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                        &claim_offer_json,
                                                                        &claim_def,
                                                                        "common_master_secret_name");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }

        #[test]
        #[ignore] //TODO different claim_def_seq_no
        fn prover_create_and_store_claim_req_works_for_claim_def_does_not_correspond_offer() {
            let (wallet_handle, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 2, 2);
            let claim_def = AnoncredsUtils::get_gvt_claim_def();

            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                        &claim_offer_json,
                                                                        &claim_def,
                                                                        "common_master_secret_name");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod issuer_create_claim {
        use super::*;

        #[test]
        fn issuer_create_claim_works() {
            let (wallet_handle, claim_def_json) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);

            let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                              "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                              &claim_offer_json,
                                                                              &claim_def_json,
                                                                              "common_master_secret_name").unwrap();

            let claim_json = AnoncredsUtils::get_gvt_claim_json();
            AnoncredsUtils::issuer_create_claim(wallet_handle, &claim_req, &claim_json).unwrap();
        }

        #[test]
        fn issuer_create_claim_works_for_claim_does_not_correspond_claim_req() {
            let (wallet_handle, claim_def_json) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);

            let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                              "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                              &claim_offer_json,
                                                                              &claim_def_json,
                                                                              "common_master_secret_name").unwrap();

            let claim_json = AnoncredsUtils::get_xyz_claim_json();
            let res = AnoncredsUtils::issuer_create_claim(wallet_handle, &claim_req, &claim_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod prover_store_claim {
        use super::*;

        #[test]
        fn prover_store_claim_works() {
            let (wallet_handle, claim_def_json) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("NcYxiDXkpYi6ov5FcYDi1e", 1, 1);

            let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                              "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                                              &claim_offer_json,
                                                                              &claim_def_json,
                                                                              "common_master_secret_name").unwrap();

            let claim_json = AnoncredsUtils::get_gvt_claim_json();
            let (_, xclaim_json) = AnoncredsUtils::issuer_create_claim(wallet_handle, &claim_req, &claim_json).unwrap();

            AnoncredsUtils::prover_store_claim(wallet_handle, &xclaim_json).unwrap();
        }
    }

    mod verifier_verify_proof {
        use super::*;

        #[test]
        fn verifier_verify_proof_works_for_proof_does_not_correspond_proof_request() {
            TestUtils::cleanup_storage();

            //1. Create wallet, get wallet handle
            let wallet_handle = WalletUtils::create_and_open_wallet("pool1", "wallet1", "default").unwrap();

            //2. Issuer create claim definition
            let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e";
            let schema_seq_no = 1;
            let claim_def_seq_no = 1;
            let schema = AnoncredsUtils::get_gvt_schema_json(schema_seq_no);

            let claim_def_json = AnoncredsUtils::create_claim_definition_and_set_link(wallet_handle, &schema, claim_def_seq_no).unwrap();

            //3. Prover create Master Secret
            let master_secret_name = "prover_master_secret";

            AnoncredsUtils::prover_create_master_secret(wallet_handle, master_secret_name).unwrap();

            //4. Prover create Claim Request
            let prover_did = "BzfFCYk";
            let claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, claim_def_seq_no, schema_seq_no);
            let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                              prover_did,
                                                                              &claim_offer_json,
                                                                              &claim_def_json,
                                                                              master_secret_name).unwrap();

            //5. Issuer create Claim
            let claim_json = AnoncredsUtils::get_gvt_claim_json();
            let (_, xclaim_json) = AnoncredsUtils::issuer_create_claim(wallet_handle,
                                                                       &claim_req,
                                                                       &claim_json).unwrap();

            // 6. Prover store received Claim
            AnoncredsUtils::prover_store_claim(wallet_handle, &xclaim_json).unwrap();

            // 7. Prover gets Claims for Proof Request
            let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{}}\
                                }}", schema_seq_no);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req_json).unwrap();
            let claims: ProofClaimsJson = serde_json::from_str(&claims_json).unwrap();

            let claims_for_attr_1 = claims.attrs.get("attr1_uuid").unwrap();
            let claim = claims_for_attr_1[0].clone();

            // 8. Prover create Proof
            let requested_claims_json = format!("{{\
                                          \"self_attested_attributes\":{{}},\
                                          \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true]}},\
                                          \"requested_predicates\":{{}}\
                                        }}", claim.claim_uuid);

            let schemas_json = format!("{{\"{}\":{}}}", claim.claim_uuid, schema);
            let claim_defs_json = format!("{{\"{}\":{}}}", claim.claim_uuid, claim_def_json);
            let revoc_regs_jsons = "{}";

            let proof_json = AnoncredsUtils::prover_create_proof(wallet_handle,
                                                                 &proof_req_json,
                                                                 &requested_claims_json,
                                                                 &schemas_json,
                                                                 &master_secret_name,
                                                                 &claim_defs_json,
                                                                 &revoc_regs_jsons).unwrap();
            println!("proof_json {}", proof_json);

            // 9. Verifier verify proof
            let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}}}\
                                }}", schema_seq_no);

            let res = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                            &proof_json,
                                                            &schemas_json,
                                                            &claim_defs_json,
                                                            &revoc_regs_jsons);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }


        #[test]
        fn anoncreds_works_for_single_issuer_single_prover() {
            TestUtils::cleanup_storage();

            let pool_name = "pool1";
            let issuer_wallet_name = "issuer_wallet";
            let prover_wallet_name = "prover_wallet";
            let xtype = "default";

            //1. Create Issuer wallet, get wallet handle
            let issuer_wallet_handle = WalletUtils::create_and_open_wallet(pool_name, issuer_wallet_name, xtype).unwrap();

            //2. Create Prover wallet, get wallet handle
            let prover_wallet_handle = WalletUtils::create_and_open_wallet(pool_name, prover_wallet_name, xtype).unwrap();

            //3. Issuer create claim definition
            let issuer_did = "NcYxiDXkpYi6ov5FcYDi1e";
            let schema_seq_no = 1;
            let claim_def_seq_no = 1;
            let schema = AnoncredsUtils::get_gvt_schema_json(schema_seq_no);

            let claim_def_json = AnoncredsUtils::create_claim_definition_and_set_link(issuer_wallet_handle, &schema, claim_def_seq_no).unwrap();

            //4. Prover create Master Secret
            let master_secret_name = "prover_master_secret";

            AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name).unwrap();

            //5. Prover store Claim Offer received from Issuer
            let claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, claim_def_seq_no, schema_seq_no);

            AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json).unwrap();

            //6. Prover get Claim Offers
            let filter_json = format!("{{ \"issuer_did\":\"{}\"}}", issuer_did);

            let claim_offers_json = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json).unwrap();

            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
            assert!(claim_offers.len() == 1);
            let claim_offer_json = serde_json::to_string(&claim_offers[0]).unwrap();

            //7. Prover create Claim Request
            let prover_did = "BzfFCYk";
            let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                              prover_did,
                                                                              &claim_offer_json,
                                                                              &claim_def_json,
                                                                              master_secret_name).unwrap();

            //8. Issuer create Claim
            let claim_json = AnoncredsUtils::get_gvt_claim_json();
            let (_, xclaim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                       &claim_req,
                                                                       &claim_json).unwrap();

            // 9. Prover store received Claim
            AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xclaim_json).unwrap();

            // 10. Prover gets Claims for Proof Request
            let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}}}\
                                }}", schema_seq_no);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
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

            let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                                 &proof_req_json,
                                                                 &requested_claims_json,
                                                                 &schemas_json,
                                                                 &master_secret_name,
                                                                 &claim_defs_json,
                                                                 &revoc_regs_jsons).unwrap();
            println!("proof_json {}", proof_json);

            // 12. Verifier verify proof
            let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                              &proof_json,
                                                              &schemas_json,
                                                              &claim_defs_json,
                                                              &revoc_regs_jsons).unwrap();
            assert!(valid);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn anoncreds_works_for_multiply_issuer_single_prover() {
            TestUtils::cleanup_storage();
            LoggerUtils::init();

            let issuer1_did = "NcYxiDXkpYi6ov5FcYDi1e";
            let issuer2_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
            let prover_did = "BzfFCYk";

            let pool_name = "pool1";
            let issuer1_wallet_name = "issuer1_wallet";
            let issuer2_wallet_name = "issuer2_wallet";
            let prover_wallet_name = "prover_wallet";
            let xtype = "default";

            //1. Issuer1 create wallet, get wallet handles
            let issuer_gvt_wallet_handle = WalletUtils::create_and_open_wallet(pool_name, issuer1_wallet_name, xtype).unwrap();

            //2. Issuer2 create wallet, get wallet handles
            let issuer_xyz_wallet_handle = WalletUtils::create_and_open_wallet(pool_name, issuer2_wallet_name, xtype).unwrap();

            //3. Prover create wallet, get wallet handles
            let prover_wallet_handle = WalletUtils::create_and_open_wallet(pool_name, prover_wallet_name, xtype).unwrap();

            let mut schemas: HashMap<i32, String> = HashMap::new();
            let mut claim_defs: HashMap<i32, String> = HashMap::new();

            //4. Issuer1 create claim definition by gvt schema
            let gvt_schema_seq_no = 1;
            let gvt_claim_def_seq_no = 1;
            let gvt_schema = AnoncredsUtils::get_gvt_schema_json(gvt_schema_seq_no);

            let gvt_claim_def_json = AnoncredsUtils::create_claim_definition_and_set_link(issuer_gvt_wallet_handle, &gvt_schema, gvt_claim_def_seq_no).unwrap();

            schemas.insert(gvt_schema_seq_no, gvt_schema.clone());
            claim_defs.insert(gvt_claim_def_seq_no, gvt_claim_def_json.clone());


            //5. Issuer1 create claim definition by xyz schema
            let xyz_schema_seq_no = 2;
            let xyz_claim_def_seq_no = 2;
            let xyz_schema = AnoncredsUtils::get_xyz_schema_json(xyz_schema_seq_no);

            let xyz_claim_def_json = AnoncredsUtils::create_claim_definition_and_set_link(issuer_xyz_wallet_handle, &xyz_schema, xyz_claim_def_seq_no).unwrap();

            schemas.insert(xyz_schema_seq_no, xyz_schema.clone());
            claim_defs.insert(xyz_claim_def_seq_no, xyz_claim_def_json.clone());

            //6. Prover create Master Secret for Issuer1
            let master_secret_name_1 = "prover_master_secret_issuer_1";

            AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name_1).unwrap();

            //7. Prover create Master Secret for Issuer2
            let master_secret_name_2 = "prover_master_secret_issuer_2";

            AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name_2).unwrap();

            //8. Prover store Claim Offer received from Issuer1
            let issuer1_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer1_did, gvt_claim_def_seq_no, gvt_schema_seq_no);

            AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer1_claim_offer_json).unwrap();

            //9. Prover store Claim Offer received from Issuer2
            let issuer2_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer2_did, xyz_claim_def_seq_no, xyz_schema_seq_no);

            AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer2_claim_offer_json).unwrap();

            //10. Prover get Claim Offers
            let filter_json = "{}";

            let claim_offers_json = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json).unwrap();

            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
            assert_eq!(2, claim_offers.len());

            let claim_offer_1 = claim_offers[0].clone();
            let claim_offer_2 = claim_offers[1].clone();

            let claim_offer_1_json = serde_json::to_string(&claim_offer_1).unwrap();
            let claim_offer_2_json = serde_json::to_string(&claim_offer_2).unwrap();

            //11. Prover create Claim Request for gvt claim offer
            let claim_offer = if claim_offer_1.claim_def_seq_no == gvt_claim_def_seq_no { claim_offer_1_json.clone() } else { claim_offer_2_json.clone() };

            let gvt_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                                  prover_did,
                                                                                  &claim_offer,
                                                                                  &gvt_claim_def_json,
                                                                                  master_secret_name_1).unwrap();

            //12. Issuer create GVT Claim
            let gvt_claim_json = AnoncredsUtils::get_gvt_claim_json();
            let (_, gvt_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_gvt_wallet_handle,
                                                                          &gvt_claim_req,
                                                                          &gvt_claim_json).unwrap();

            //13. Prover store received GVT Claim
            AnoncredsUtils::prover_store_claim(prover_wallet_handle, &gvt_claim_json).unwrap();

            //14. Prover create Claim Request for xyz claim offer
            let claim_offer = if claim_offer_2.claim_def_seq_no == xyz_claim_def_seq_no { claim_offer_2_json.clone() } else { claim_offer_1_json.clone() };
            let xyz_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                                  prover_did,
                                                                                  &claim_offer,
                                                                                  &xyz_claim_def_json,
                                                                                  master_secret_name_1).unwrap();

            //15. Issuer create XYZ Claim
            let xyz_claim_json = AnoncredsUtils::get_xyz_claim_json();
            let (_, xyz_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_xyz_wallet_handle,
                                                                          &xyz_claim_req,
                                                                          &xyz_claim_json).unwrap();

            // 16. Prover store received XYZ Claim
            AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xyz_claim_json).unwrap();

            // 17. Prover gets Claims for Proof Request
            let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}},\
                                                         \"attr2_uuid\":{{\"schema_seq_no\":{},\"name\":\"status\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}},\
                                                              \"predicate2_uuid\":{{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}}}\
                                }}", gvt_schema_seq_no, xyz_schema_seq_no);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();

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


            let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                                 &proof_req_json,
                                                                 &requested_claims_json,
                                                                 &schemas_json,
                                                                 &master_secret_name_1,
                                                                 &claim_defs_json,
                                                                 &revoc_regs_jsons).unwrap();

            // 19. Verifier verify proof
            let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                              &proof_json,
                                                              &schemas_json,
                                                              &claim_defs_json,
                                                              &revoc_regs_jsons).unwrap();
            assert!(valid);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn anoncreds_works_for_single_issuer_multiply_claims_single_prover() {
            TestUtils::cleanup_storage();

            let issuer_did = "CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
            let prover_did = "BzfFCYk";

            let pool_name = "pool1";
            let issuer_wallet_name = "issuer_wallet";
            let prover_wallet_name = "prover_wallet";
            let xtype = "default";

            //1. Issuer create wallet, get wallet handles
            let issuer_wallet_handle = WalletUtils::create_and_open_wallet(pool_name, issuer_wallet_name, xtype).unwrap();

            //2. Prover create wallet, get wallet handles
            let prover_wallet_handle = WalletUtils::create_and_open_wallet(pool_name, prover_wallet_name, xtype).unwrap();

            let mut schemas: HashMap<i32, String> = HashMap::new();
            let mut claim_defs: HashMap<i32, String> = HashMap::new();

            //3. Issuer create claim definition by gvt schema
            let gvt_schema_seq_no = 1;
            let gvt_claim_def_seq_no = 1;
            let gvt_schema = AnoncredsUtils::get_gvt_schema_json(gvt_schema_seq_no);

            let gvt_claim_def_json = AnoncredsUtils::create_claim_definition_and_set_link(issuer_wallet_handle, &gvt_schema, gvt_claim_def_seq_no).unwrap();

            schemas.insert(gvt_schema_seq_no, gvt_schema.clone());
            claim_defs.insert(gvt_claim_def_seq_no, gvt_claim_def_json.clone());

            //4. Issuer create claim definition by xyz schema
            let xyz_schema_seq_no = 2;
            let xyz_claim_def_seq_no = 2;
            let xyz_schema = AnoncredsUtils::get_xyz_schema_json(xyz_schema_seq_no);

            let xyz_claim_def_json = AnoncredsUtils::create_claim_definition_and_set_link(issuer_wallet_handle, &xyz_schema, xyz_claim_def_seq_no).unwrap();

            schemas.insert(xyz_schema_seq_no, xyz_schema.clone());
            claim_defs.insert(xyz_claim_def_seq_no, xyz_claim_def_json.clone());

            //5. Prover create Master Secret for Issuer1
            let master_secret_name = "prover_master_secret_issuer";

            AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, master_secret_name).unwrap();

            //6. Prover store GVT Claim Offer received from Issuer
            let issuer_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, gvt_claim_def_seq_no, gvt_schema_seq_no);

            AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer_claim_offer_json).unwrap();

            //7. Prover store XYZ Claim Offer received from Issuer
            let issuer_claim_offer_json = AnoncredsUtils::get_claim_offer(issuer_did, xyz_claim_def_seq_no, xyz_schema_seq_no);

            AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &issuer_claim_offer_json).unwrap();

            //8. Prover get Claim Offers
            let filter_json = format!("{{ \"issuer_did\":\"{}\"}}", issuer_did);

            let claim_offers_json = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json).unwrap();

            let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
            assert_eq!(2, claim_offers.len());

            let claim_offer_1 = claim_offers[0].clone();
            let claim_offer_2 = claim_offers[1].clone();

            let claim_offer_1_json = serde_json::to_string(&claim_offer_1).unwrap();
            let claim_offer_2_json = serde_json::to_string(&claim_offer_2).unwrap();

            //9. Prover create Claim Request for gvt claim offer
            let claim_offer = if claim_offer_1.claim_def_seq_no == gvt_claim_def_seq_no { claim_offer_1_json.clone() } else { claim_offer_2_json.clone() };

            let gvt_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                                  prover_did,
                                                                                  &claim_offer,
                                                                                  &gvt_claim_def_json,
                                                                                  master_secret_name).unwrap();


            //10. Issuer create GVT Claim
            let gvt_claim_json = AnoncredsUtils::get_gvt_claim_json();
            let (_, gvt_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                          &gvt_claim_req,
                                                                          &gvt_claim_json).unwrap();

            //11. Prover store received GVT Claim
            AnoncredsUtils::prover_store_claim(prover_wallet_handle, &gvt_claim_json).unwrap();

            //12. Prover create Claim Request for xyz claim offer
            let claim_offer = if claim_offer_2.claim_def_seq_no == xyz_claim_def_seq_no { claim_offer_2_json.clone() } else { claim_offer_1_json.clone() };
            let xyz_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                                  prover_did,
                                                                                  &claim_offer,
                                                                                  &xyz_claim_def_json,
                                                                                  master_secret_name).unwrap();

            //13. Issuer create XYZ Claim
            let xyz_claim_json = AnoncredsUtils::get_xyz_claim_json();
            let (_, xyz_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                          &xyz_claim_req,
                                                                          &xyz_claim_json).unwrap();

            //14. Prover store received XYZ Claim
            AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xyz_claim_json).unwrap();

            //15. Prover gets Claims for Proof Request
            let proof_req_json = format!("{{\
                                   \"nonce\":\"123432421212\",\
                                   \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}}}},\
                                   \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}},\
                                                              \"predicate2_uuid\":{{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}}}\
                                }}", gvt_schema_seq_no);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();

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

            let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                                 &proof_req_json,
                                                                 &requested_claims_json,
                                                                 &schemas_json,
                                                                 &master_secret_name,
                                                                 &claim_defs_json,
                                                                 &revoc_regs_jsons).unwrap();

            //17. Verifier verify proof
            let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                              &proof_json,
                                                              &schemas_json,
                                                              &claim_defs_json,
                                                              &revoc_regs_jsons).unwrap();
            assert!(valid);

            TestUtils::cleanup_storage();
        }
    }
}







