extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::anoncreds::AnoncredsUtils;
use utils::anoncreds::COMMON_MASTER_SECRET;
use utils::test::TestUtils;
use std::collections::HashMap;
use utils::types::*;

use indy::api::ErrorCode;
use utils::inmem_wallet::InmemWallet;
use utils::constants::*;

mod high_cases {
    use super::*;

    mod issuer_create_and_store_claim_def {
        use super::*;

        #[test]
        fn issuer_create_and_store_claim_def_works() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(wallet_handle,
                                                                                ISSUER_DID,
                                                                                &AnoncredsUtils::gvt_schema_json(),
                                                                                None, false).unwrap();

            serde_json::from_str::<ClaimDefinition>(&claim_def_json).unwrap();
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_invalid_wallet() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::issuer_create_claim_definition(invalid_wallet_handle,
                                                                     ISSUER_DID,
                                                                     &AnoncredsUtils::gvt_schema_json(),
                                                                     None, false);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_store_claim_offer {
        use super::*;

        #[test]
        fn prover_store_claim_offer_works() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            AnoncredsUtils::prover_store_claim_offer(wallet_handle, &AnoncredsUtils::gvt_claim_offer()).unwrap();
        }

        #[test]
        fn prover_store_claim_offer_works_for_invalid_json() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let claim_offer_json = format!(r#"{{"issuer_did":"{}"}}"#, ISSUER_DID);

            let res = AnoncredsUtils::prover_store_claim_offer(wallet_handle, &claim_offer_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_store_claim_offer_works_for_invalid_wallet() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_store_claim_offer(invalid_wallet_handle, &AnoncredsUtils::gvt_claim_offer());
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_get_claim_offers {
        use super::*;

        #[test]
        fn prover_get_claim_offers_works_for_empty_filter() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{}"#).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 3);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::xyz_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_issuer() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, &format!(r#"{{"issuer_did":"{}"}}"#, ISSUER_DID)).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 2);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::xyz_schema_key() }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle,
                                                                       &format!(r#"{{"schema_key":{}}}"#, AnoncredsUtils::gvt_schema_key_json())).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 2);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_schema_name() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"schema_key":{"name":"gvt"}}"#).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 2);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_schema_version() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"schema_key":{"version":"1.0"}}"#).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 3);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::xyz_schema_key() }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_schema_did() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, &format!(r#"{{"schema_key":{{"did":"{}"}}}}"#, ISSUER_DID)).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 1);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::xyz_schema_key() }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_filter_by_issuer_and_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle,
                                                                       &format!(r#"{{"issuer_did":"{}","schema_key":{}}}"#, ISSUER_DID, AnoncredsUtils::gvt_schema_key_json())).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 1);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
        }

        #[test]
        fn prover_get_claim_offers_works_for_no_results() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"issuer_did":"didaacdsfds"}"#).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 0);
        }

        #[test]
        fn prover_get_claim_offers_works_for_invalid_wallet_handle() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_get_claim_offers(invalid_wallet_handle, &format!(r#"{{"issuer_did":"{}"}}"#, ISSUER_DID));
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }

        #[test]
        fn prover_get_claim_offers_for_custom_wallet() {
            InmemWallet::cleanup();

            WalletUtils::register_wallet_type(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();

            AnoncredsUtils::prover_store_claim_offer(wallet_handle, &AnoncredsUtils::gvt_claim_offer()).unwrap();
            AnoncredsUtils::prover_store_claim_offer(wallet_handle, &AnoncredsUtils::xyz_claim_offer()).unwrap();
            AnoncredsUtils::prover_store_claim_offer(wallet_handle, &AnoncredsUtils::get_claim_offer(DID, &AnoncredsUtils::gvt_schema_key())).unwrap();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle, &format!(r#"{{"issuer_did":"{}"}}"#, ISSUER_DID)).unwrap();

            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 2);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::gvt_schema_key() }));
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::xyz_schema_key() }));

            WalletUtils::close_wallet(wallet_handle).unwrap();
            InmemWallet::cleanup();
        }
    }

    mod prover_create_master_secret {
        use super::*;

        #[test]
        fn prover_create_master_secret_works() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            AnoncredsUtils::prover_create_master_secret(wallet_handle, COMMON_MASTER_SECRET).unwrap();
        }

        #[test]
        fn prover_create_master_secret_works_invalid_wallet_handle() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_create_master_secret(invalid_wallet_handle, COMMON_MASTER_SECRET);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_create_and_store_claim_req {
        use super::*;

        #[test]
        fn prover_create_and_store_claim_req_works() {
            let (wallet_handle, claim_def, claim_offer, _, _) = AnoncredsUtils::init_common_wallet();

            AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                              DID,
                                                              &claim_offer,
                                                              &claim_def,
                                                              COMMON_MASTER_SECRET).unwrap();
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_wallet() {
            let (wallet_handle, claim_def, claim_offer, _, _) = AnoncredsUtils::init_common_wallet();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_create_and_store_claim_req(invalid_wallet_handle,
                                                                        DID,
                                                                        &claim_offer,
                                                                        &claim_def,
                                                                        COMMON_MASTER_SECRET);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_claim_def_not_correspond_to_claim_offer() {
            let (wallet_handle, claim_def, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = AnoncredsUtils::get_claim_offer(DID, &AnoncredsUtils::xyz_schema_key());

            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        DID,
                                                                        &claim_offer_json,
                                                                        &claim_def,
                                                                        COMMON_MASTER_SECRET);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod issuer_create_claim {
        use super::*;

        #[test]
        fn issuer_create_claim_works() {
            let (wallet_handle, _, _, claim_req, _) = AnoncredsUtils::init_common_wallet();

            AnoncredsUtils::issuer_create_claim(wallet_handle,
                                                &claim_req,
                                                &AnoncredsUtils::gvt_claim_values_json(),
                                                None).unwrap();
        }

        #[test]
        fn issuer_create_claim_works_for_claim_does_not_correspond_to_claim_val() {
            let (wallet_handle, _, _, claim_req, _) = AnoncredsUtils::init_common_wallet();

            let res = AnoncredsUtils::issuer_create_claim(wallet_handle,
                                                          &claim_req,
                                                          &AnoncredsUtils::xyz_claim_values_json(),
                                                          None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_claim_works_for_for_invalid_wallet_handle() {
            let (wallet_handle, _, _, claim_req, _) = AnoncredsUtils::init_common_wallet();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::issuer_create_claim(invalid_wallet_handle,
                                                          &claim_req,
                                                          &AnoncredsUtils::gvt_claim_values_json(),
                                                          None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_store_claim {
        use super::*;

        #[test]
        fn prover_store_claim_worksa() {
            let (wallet_handle, claim_def_json, claim_offer, _, _) = AnoncredsUtils::init_common_wallet();
            let prover_wallet_handle = WalletUtils::create_and_open_wallet("proverWallet", None).unwrap();

            AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

            let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                              DID_MY1,
                                                                              claim_offer,
                                                                              claim_def_json,
                                                                              COMMON_MASTER_SECRET).unwrap();

            let (_, claim_json) = AnoncredsUtils::issuer_create_claim(wallet_handle, &claim_req,
                                                                      &AnoncredsUtils::gvt_claim_values_json(),
                                                                      None).unwrap();

            AnoncredsUtils::prover_store_claim(prover_wallet_handle, &claim_json, None).unwrap();
        }

        #[test]
        fn prover_store_claim_works_for_invalid_wallet_handle() {
            let (wallet_handle, _, _, _, claim_json) = AnoncredsUtils::init_common_wallet();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_store_claim(invalid_wallet_handle, &claim_json, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_get_claims {
        use super::*;

        #[test]
        fn prover_get_claims_works_for_empty_filter() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, r#"{}"#).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 3);
        }

        #[test]
        fn prover_get_claims_works_for_filter_by_issuer_did() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, &format!(r#"{{"issuer_did":"{}"}}"#, ISSUER_DID)).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 2);
            assert_eq!(claims[0].issuer_did, ISSUER_DID);
            assert_eq!(claims[1].issuer_did, ISSUER_DID);
        }

        #[test]
        fn prover_get_claims_works_for_filter_by_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, &format!(r#"{{"schema_key":{}}}"#, &AnoncredsUtils::gvt_schema_key_json())).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 2);
            assert_eq!(claims[0].schema_key, AnoncredsUtils::gvt_schema_key());
            assert_eq!(claims[1].schema_key, AnoncredsUtils::gvt_schema_key());
        }

        #[test]
        fn prover_get_claims_works_for_filter_by_schema_name() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, r#"{"schema_key":{"name":"gvt"}}"#).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 2);
            assert_eq!(claims[0].schema_key, AnoncredsUtils::gvt_schema_key());
            assert_eq!(claims[1].schema_key, AnoncredsUtils::gvt_schema_key());
        }

        #[test]
        fn prover_get_claims_works_for_filter_by_schema_version() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, r#"{"schema_key":{"version":"1.0"}}"#).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 3);
        }

        #[test]
        fn prover_get_claims_works_for_filter_by_schema_did() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, &format!(r#"{{"schema_key":{{"did":"{}"}}}}"#, ISSUER_DID)).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 1);
            assert_eq!(claims[0].schema_key, AnoncredsUtils::xyz_schema_key());
        }

        #[test]
        fn prover_get_claims_works_for_filter_by_issuer_did_and_schema_seq_no() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, &format!(r#"{{"issuer_did":"{}", "schema_key":{}}}"#, ISSUER_DID, &AnoncredsUtils::gvt_schema_key_json())).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 1);
            assert_eq!(claims[0].issuer_did, ISSUER_DID);
            assert_eq!(claims[0].schema_key, AnoncredsUtils::gvt_schema_key());
        }

        #[test]
        fn prover_get_claims_works_for_empty_result() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims = AnoncredsUtils::prover_get_claims(wallet_handle, r#"{"issuer_did":"issuerdid"}"#).unwrap();
            let claims: Vec<ClaimInfo> = serde_json::from_str(&claims).unwrap();

            assert_eq!(claims.len(), 0);
        }

        #[test]
        fn prover_get_claims_works_for_invalid_wallet_handle() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_get_claims(invalid_wallet_handle, r#"{}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    //NOTE: There are following claim stored in wallet:
    // {"issuer_did": ISSUER_DID, "schema_seq_no": GVT_SEQ_NO}
    // {"issuer_did": ISSUER_DID, "schema_seq_no": XYZ_SEQ_NO}
    // {"issuer_did": DID, "schema_seq_no": GVT_SEQ_NO}
    mod prover_get_claims_for_proof_req {
        use super::*;

        #[test]
        fn prover_get_claims_for_proof_req_works_for_empty_req() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                        "nonce":"123432421212",
                                        "name":"proof_req_1",
                                        "version":"0.1",
                                        "requested_attrs":{},
                                        "requested_predicates":{}
                                   }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 0);
            assert_eq!(claims.predicates.len(), 0);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_only() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{
                                                "attr1_referent":{"name":"name"}
                                            },
                                            "requested_predicates":{}
                                       }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_in_upper_case() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{
                                                "attr1_referent":{"name":"NAME"}
                                            },
                                            "requested_predicates":{}
                                       }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_contains_spaces() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{
                                                "attr1_referent":{"name":" name "}
                                            },
                                            "requested_predicates":{}
                                       }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_issuer() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"issuer_did":"{}"}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, ISSUER_DID);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 1);
        }

        #[test]
        fn prover_get_claims_for_proof_rea_works_for_revealed_attr_for_multiple_issuers() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"issuer_did":"{}"}},
                                                                            {{"issuer_did":"{}"}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, ISSUER_DID, DID);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"schema_key":{}}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, AnoncredsUtils::gvt_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_schema_name() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                "nonce":"123432421212",
                                "name":"proof_req_1",
                                "version":"0.1",
                                "requested_attrs":{
                                    "attr1_referent":{
                                        "name":"name",
                                        "restrictions":[{"schema_key":{"name":"gvt"}}]
                                    }
                                },
                                "requested_predicates":{}
                             }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_schema_version() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                "nonce":"123432421212",
                                "name":"proof_req_1",
                                "version":"0.1",
                                "requested_attrs":{
                                    "attr1_referent":{
                                        "name":"name",
                                        "restrictions":[{"schema_key":{"version":"1.0"}}]
                                    }
                                },
                                "requested_predicates":{}
                             }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_schema_did() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{{
                                                "attr1_referent":{{
                                                    "name":"name",
                                                    "restrictions":[{{"schema_key":{{"did":"{}"}}}}]
                                                }}
                                            }},
                                            "requested_predicates":{{}}
                                        }}"#, DID_TRUSTEE);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_schema_or_specific_issuer() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"schema_key":{}}},
                                                                            {{"issuer_did":"{}"}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, AnoncredsUtils::gvt_schema_key_json(), ISSUER_DID);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_rea_works_for_revealed_attr_for_multiple_schemas() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"schema_key":{}}},
                                                                            {{"schema_key":{}}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, AnoncredsUtils::gvt_schema_key_json(), AnoncredsUtils::xyz_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_specific_issuer_and_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"issuer_did":"{}", "schema_key":{}}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, ISSUER_DID, AnoncredsUtils::gvt_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 1);
        }

        #[test]
        fn prover_get_claims_for_proof_rea_works_for_revealed_attr_for_multiple_specific_issuer_schema_pairs() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"issuer_did":"{}", "schema_key":{}}},
                                                                            {{"issuer_did":"{}", "schema_key":{}}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, ISSUER_DID, AnoncredsUtils::gvt_schema_key_json(), DID, AnoncredsUtils::gvt_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_rea_works_for_revealed_attr_for_specific_issuer_schema_pair_or_issuer() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"issuer_did":"{}", "schema_key":{}}},
                                                                            {{"issuer_did":"{}"}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, ISSUER_DID, AnoncredsUtils::gvt_schema_key_json(), DID);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{},
                                            "requested_predicates":{
                                                "predicate1_referent":{"attr_name":"age","p_type":">=","value":18}
                                            }
                                       }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_attribute_in_upper_case() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{},
                                            "requested_predicates":{
                                                "predicate1_referent":{"attr_name":"AGE","p_type":">=","value":18}
                                            }
                                       }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_attribute_contains_spaces() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{},
                                            "requested_predicates":{
                                                "predicate1_referent":{"attr_name":" age ","p_type":">=","value":18}
                                            }
                                       }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_specific_issuer() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,"restrictions":[{{"issuer_did":"{}"}}]
                                                        }}
                                                    }}
                                                 }}"#, ISSUER_DID);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 1);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_multiple_issuers() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,
                                                            "restrictions":[{{"issuer_did":"{}"}},{{"issuer_did":"{}"}}]
                                                        }}
                                                    }}
                                                 }}"#, ISSUER_DID, DID);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_specific_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,
                                                            "restrictions":[{{"schema_key":{}}}]
                                                        }}
                                                    }}
                                                 }}"#, AnoncredsUtils::gvt_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_multiple_schemas() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,
                                                            "restrictions":[{{"schema_key":{}}},{{"schema_key":{}}}]
                                                        }}
                                                    }}
                                                 }}"#, AnoncredsUtils::gvt_schema_key_json(), AnoncredsUtils::xyz_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_specific_issuer_schema_pair() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,
                                                            "restrictions":[{{"issuer_did":"{}", "schema_key":{}}}]
                                                        }}
                                                    }}
                                                 }}"#, ISSUER_DID, AnoncredsUtils::gvt_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 1);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_multiple_issuer_schema_pairs() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,
                                                            "restrictions":[{{"issuer_did":"{}", "schema_key":{}}},
                                                                            {{"issuer_did":"{}", "schema_key":{}}}]
                                                        }}
                                                    }}
                                                 }}"#, ISSUER_DID, AnoncredsUtils::gvt_schema_key_json(), DID, AnoncredsUtils::gvt_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_multiple_revealed_attrs_and_predicates() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{"nonce":"123432421212",
                                        "name":"proof_req_1",
                                        "version":"0.1",
                                        "requested_attrs":{
                                            "attr1_referent":{"name":"name"},
                                            "attr2_referent":{"name":"status"}
                                        },
                                        "requested_predicates":{
                                            "predicate1_referent":{"attr_name":"age","p_type":">=","value":18},
                                            "predicate2_referent":{"attr_name":"height","p_type":">=","value":160}
                                        }}"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();

            assert_eq!(claims.attrs.len(), 2);
            assert_eq!(claims.predicates.len(), 2);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 2);

            let claims_for_attr_2 = claims.attrs.get("attr2_referent").unwrap();
            assert_eq!(claims_for_attr_2.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 2);

            let claims_for_predicate_2 = claims.predicates.get("predicate2_referent").unwrap();
            assert_eq!(claims_for_predicate_2.len(), 2);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_not_found_attribute() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{"nonce":"123432421212",
                                        "name":"proof_req_1",
                                        "version":"0.1",
                                        "requested_attrs":{
                                            "attr1_referent":{
                                                "name":"some_attr"
                                            }
                                        },
                                        "requested_predicates":{}
                                       }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 0);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_not_found_predicate_attribute() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{},
                                            "requested_predicates":{
                                                "predicate1_referent":{
                                                    "attr_name":"weight","p_type":">=","value":58
                                                }
                                            }
                                        }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();

            assert_eq!(claims.attrs.len(), 0);
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 0);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_not_satisfied_predicate() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{},
                                            "requested_predicates":{
                                                "predicate1_referent":{
                                                    "attr_name":"age","p_type":">=","value":58
                                                }
                                            }
                                        }"#;

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();

            assert_eq!(claims.attrs.len(), 0);
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 0);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_other_issuer() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                "nonce":"123432421212",
                                                "name":"proof_req_1",
                                                "version":"0.1",
                                                "requested_attrs":{{
                                                    "attr1_referent":{{
                                                        "name":"name",
                                                        "restrictions":[{{"issuer_did":"{}"}}]
                                                    }}
                                                }},
                                                "requested_predicates":{{}}
                                             }}"#, DID_TRUSTEE);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 0);
        }


        pub const OTHER_SCHEMA_KEY_JSON: &'static str = r#"{"name":"name", "version":"1.0", "did":"didasasdvcxvzc"}"#;

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_other_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"schema_key":{}}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, OTHER_SCHEMA_KEY_JSON);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 0);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_revealed_attr_for_other_issuer_schema_pair() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{
                                                        "attr1_referent":{{
                                                            "name":"name",
                                                            "restrictions":[{{"issuer_did":"{}", "schema_key":{}}}]
                                                        }}
                                                    }},
                                                    "requested_predicates":{{}}
                                                 }}"#, DID_TRUSTEE, AnoncredsUtils::gvt_schema_key_json());

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.attrs.len(), 1);

            let claims_for_attr_1 = claims.attrs.get("attr1_referent").unwrap();
            assert_eq!(claims_for_attr_1.len(), 0);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_other_issuer() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,"restrictions":[{{"issuer_did":"{}"}}]
                                                        }}
                                                    }}
                                                 }}"#, DID_MY2);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 0);
        }


        #[test]
        fn prover_get_claims_for_proof_req_works_for_predicate_for_other_schema() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = format!(r#"{{
                                                    "nonce":"123432421212",
                                                    "name":"proof_req_1",
                                                    "version":"0.1",
                                                    "requested_attrs":{{}},
                                                    "requested_predicates":{{
                                                        "predicate1_referent":{{
                                                            "attr_name":"age","p_type":">=","value":18,
                                                            "restrictions":[{{"schema_key":{}}}]
                                                        }}
                                                    }}
                                                 }}"#, OTHER_SCHEMA_KEY_JSON);

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req).unwrap();

            let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
            assert_eq!(claims.predicates.len(), 1);

            let claims_for_predicate_1 = claims.predicates.get("predicate1_referent").unwrap();
            assert_eq!(claims_for_predicate_1.len(), 0);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_invalid_wallet_handle() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                            "nonce":"123432421212",
                                            "name":"proof_req_1",
                                            "version":"0.1",
                                            "requested_attrs":{},
                                            "requested_predicates":{
                                                "predicate1_referent":{
                                                    "attr_name":"age","p_type":">=","value":58
                                                }
                                            }
                                        }"#;

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_get_claims_for_proof_req(invalid_wallet_handle, &proof_req);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod prover_create_proof_works {
        use super::*;

        #[test]
        fn prover_create_proof_works() {
            let (wallet_handle, claim_def_json, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, AnoncredsUtils::proof_request_attr_and_predicate()).unwrap();
            let claim_for_attr = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");
            let claim_for_predicate = AnoncredsUtils::get_claim_for_predicate_referent(&claims_json, "predicate1_referent");

            let requested_claims_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                  "requested_predicates":{{"predicate1_referent":"{}"}}
                                                }}"#, claim_for_attr.referent, claim_for_predicate.referent);

            let schemas_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, claim_def_json);
            let revoc_regs_json = "{}";

            AnoncredsUtils::prover_create_proof(wallet_handle,
                                                AnoncredsUtils::proof_request_attr_and_predicate(),
                                                &requested_claims_json,
                                                &schemas_json,
                                                COMMON_MASTER_SECRET,
                                                &claim_defs_json,
                                                &revoc_regs_json).unwrap();
        }

        #[test]
        fn prover_create_proof_works_for_using_not_satisfy_claim() {
            let (wallet_handle, claim_def_json, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, AnoncredsUtils::proof_request_attr_and_predicate()).unwrap();
            let claim_for_attr = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

            let proof_req = r#"{
                                        "nonce":"123432421212",
                                        "name":"proof_req_1",
                                        "version":"0.1",
                                        "requested_attrs":{
                                            "attr1_referent":{"name":"some_attr"}
                                        },
                                        "requested_predicates":{}
                                    }"#;

            let requested_claims_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                  "requested_predicates":{{}}
                                                }}"#, claim_for_attr.referent);

            let schemas_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, claim_def_json);
            let revoc_regs_json = "{}";

            let res = AnoncredsUtils::prover_create_proof(wallet_handle,
                                                          &proof_req,
                                                          &requested_claims_json,
                                                          &schemas_json,
                                                          COMMON_MASTER_SECRET,
                                                          &claim_defs_json,
                                                          &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_create_proof_works_for_invalid_wallet_handle() {
            let (wallet_handle, claim_def_json, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, AnoncredsUtils::proof_request_attr_and_predicate()).unwrap();
            let claim_for_attr = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

            let requested_claims_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                  "requested_predicates":{{}}
                                                }}"#, claim_for_attr.referent);

            let schemas_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, claim_def_json);
            let revoc_regs_jsons = "{}";

            let invalid_wallet_handle = wallet_handle + 100;
            let res = AnoncredsUtils::prover_create_proof(invalid_wallet_handle,
                                                          AnoncredsUtils::proof_request_attr_and_predicate(),
                                                          &requested_claims_json,
                                                          &schemas_json,
                                                          COMMON_MASTER_SECRET,
                                                          &claim_defs_json,
                                                          &revoc_regs_jsons);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);
        }
    }

    mod verifier_verify_proof {
        use super::*;

        #[test]
        fn verifier_verify_proof_works_for_correct_proof() {
            let schemas_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::claim_def_json());
            let revoc_regs_json = "{}";

            let valid = AnoncredsUtils::verifier_verify_proof(AnoncredsUtils::proof_request_attr_and_predicate(),
                                                              AnoncredsUtils::proof_json(),
                                                              &schemas_json,
                                                              &claim_defs_json,
                                                              &revoc_regs_json).unwrap();
            assert!(valid);
        }

        #[test]
        fn verifier_verify_proof_works_for_proof_does_not_correspond_to_request() {
            let schemas_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::claim_def_json());
            let revoc_regs_json = "{}";

            let other_proof_req_json = r#"{
                                                      "nonce":"123432421212",
                                                      "name":"proof_req_1",
                                                      "version":"0.1",
                                                      "requested_attrs":{
                                                        "attr1_referent":{"name":"sex"}
                                                      },
                                                      "requested_predicates":{
                                                        "predicate1_referent":{"attr_name":"age","p_type":">=","value":18}
                                                      }
                                                  }"#;

            let res = AnoncredsUtils::verifier_verify_proof(&other_proof_req_json,
                                                            AnoncredsUtils::proof_json(),
                                                            &schemas_json,
                                                            &claim_defs_json,
                                                            &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn verifier_verify_proof_works_for_wrong_proof() {
            let schemas_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::claim_def_json());
            let revoc_regs_json = "{}";
            let proof_json = r#"{"proof":{"proofs":{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{"primary_proof":{"eq_proof":{"revealed_attrs":{"name":"1139481716457488690172217916278103335"},"a_prime":"80401564260558483983794628158664845806393125691167675024527906210615204776868092566789307767601325086260531777605457298059939671624755239928848057947875953445797869574854365751051663611984607735255307096920094357120779812375573500489773454634756645206823074153240319316758529163584251907107473703779754778699279153037094140428648169418133281187947677937472972061954089873405836249023133445286756991574802740614183730141450546881449500189789970102133738133443822618072337620343825908790734460412932921199267304555521397418007577171242880211812703320270140386219809818196744216958369397014610013338422295772654405475023","e":"31151798717381512709903464053695613005379725796031086912986270617392167764097422442809244590980303622977555221812111085160553241592792901","v":"524407431684833626723631303096063196973911986967748096669183384949467719053669910411426601230736351335262754473490498825342793551112426427823428399937548938048089615644972537564428344526295733169691240937176356626523864731701111189536269488496019586818879697981955044502664124964896796783428945944075084807859935155837238670987272778459356531608865162828109489758902085206073584532002909678902616210042778963974064479140826712481297584040209095459963718975102750913306565864485279810056629704077428898739021040190774575868853629858297299392839284660771662690107106553362040805152261505268111067408422298806905178826507224233050991301274817252924123120887017757639206512015559321675322509820081151404696713509158685022511201565062671933414307463988209696457343022378430051265752251403461414881325357657438328740471164157220698425309006894962942640219890219594168419276308074677144722217081026358892787770650248878952483621","m":{"age":"10477979077744818183854012231360633424177093192344587159214818537659704987539982653663361680650769087122324965941845552897155693994859927792964720675888893623940580527766661802170","sex":"15368219775809326116045200104269422566086585069798988383076685221700842794654771075432385446820819836777771517356551059931242867733879324915651894894695726945279462946826404864068","height":"268172143999991481637372321419290603042446269013750825098514042757459298040087626745653681785038933035820421862976371452111736537699176931068992453946771945552540798204580069806"},"m1":"119095745403940293668103184388411799541118279558928018597628509118163496000813590825371995586347826189221837428823000332905316924389185590810015031744029496470545254805993327676570037596326743185389101389800942263689809725968264069601565478411709555274081560719927118853299543998608664701485475703881376151770","m2":"3166313665375815600922385342096456465402430622944571045536207479553790085339726549928012930073803465171492637049498407367742103524723152099973753540483894420905314750248333232361"},"ge_proofs":[{"u":{"2":"6494171529848192644197417834173236605253723188808961394289041396341136802965710957759175642924978223517091081898946519122412445399638640485278379079647638538597635045303985779767","0":"7739508859260491061487569748588091139318989278758566530899756574128579312557203413565436003310787878172471425996601979342157451689172171025305431595131816910273398879776841751855","3":"9424758820140378077609053635383940574362083113571024891496206162696034958494400871955445981458978146571146602763357500412840538526390475379772903513687358736287298159312524034159","1":"9011979414559555265454106061917684716953356440811838475257096756618761731111646531136628099710567973381801256908067529269805992222342928842825929421929485785888403149296320711642"},"r":{"DELTA":"2119857977629302693157808821351328058251440215802746362450951329352726877165815663955490999790457576333458830301801261754696823614762123890412904169206391143688952648566814660498520188221060505840151491403269696751525874990487604723445355651918681212361562384420233903265612599812725766212744963540390806334870022328290970137051148373040320927100063898502086531019924715927190306801273252711777648467224661735618842887006436195147540705753550974655689586750013569294343535843195025962867299786380033532422131203367401906988124836294104501525520053613392691214421562815044433237816093079784307397782961917892254668290115653012265908717124278607660504580036193346698672079435538219972121355893074219968755049500875222141","2":"879097501989202140886939888802566536179834329508897124489020677433754766947767937608431979796722207676629625451150104784909666168153917345813160237337412296010679353735699663083287427507870244565918756969618964144516025526404618052053542009438548457492400344119561349471929199757453154204191407620539220514897529346602664135146454509169680801061111878075145734123580343470361019624175036825631373890661124315134340427076598351080893567995392248394683875116715114577054906406649006122102488431184007790011073389768061904597267545895265921673106871142463561948479668876241841045522543174660428236658891636170119227855493059358614089146415798861053408542832475696099851160385105386001523305465829676723036394820593263477","0":"1724016272047416140958096373304304971004826284109046259544344355102178044512441391364907122486655755929044720001281832600729467778103556397960700809066582436321515744527550472324028227472294258045699756170293405547851344921626775854114063087070898499913846456795761213291925373770081490280103876827479351849800210782799381740073719081199000612284788683993320623339686128531187019125095700122135094060470612862911102824801065698176788174959069186600426519872015152034176356923049531650418553748519941342115963599848111324793380438600664408464987023646615003553912544410140730587797458882329021327455905737414352355326238028222782957735440607899424838572541602600159016542488644761584240884783618700311735467659132540546","3":"2317535203964314926167241523636020444600002667629517624482931328850422196008281300859516069440995466415138723103558631951648519232327284208990029010060986032518946759289078833125920310350676484457972303378558158127406345804560689086460633931717939234025886786468170219981598030245042011840614339386724945679531091642132820284896626191109974537171662283750959028046143650291367908660204201563611944187723824430780626387525165408619587771059635528553832034409311888615502905143628507219523591091412192645348525327725381323865648645828460581593542176351568614465903523790649219812666979685223535464526901006270478687017672202058914176692964406859722580270696925877498058525086810338471380117323227744481903228027847825795","1":"1119193929864813751243160041764170298897380522230946444206167281178657213260394833843687899872857393015947283159245092452814155776571829885921814072299525859857844030379558685168895306445277750249341844789101670896570226707650318347992386244538723699686941887792682779028216548922683313576597384354842537728667739985216662699631842296096507821667149950956179957306177525178260912379909156360834120816956949271530622510333943914411903103069247646327625753995178999023427645468623522280255892736633780185163496867644317005801241786702434621502492159672660131289312665511793827552317714835658019088880972220344126692027952749318018900669839090109361161616086319604439015851316798257015063653414161203599184730094765941653"},"mj":"10477979077744818183854012231360633424177093192344587159214818537659704987539982653663361680650769087122324965941845552897155693994859927792964720675888893623940580527766661802170","alpha":"46280660038407959140964701167450659223532556136388451390393713283900546119670373626221864441898929302821705811144923685080534692512705456699843367809872982836890616398604933641265111106644805368974824737276965928297120628041257593166650593538539384316563258781595629888673792430276007730792093088812056156937735120078929629310611907731935101448992312370312134173482115524436767558802102266208152808607480693236511858269018733175523724309089010048330044458187371675333889670055578652283806685440133357512406700879353713629795062705271430695988191782837658895477702634883214188598350625843489120361660836956958750828038278027538830855628653513539929730230905015331221220017847248793929813230252015802389329428995718799619565984669228143200627972926117282688854152516298117476837960100343260648687249027349308513966440386556698667484082658689","t":{"DELTA":"46814992964714978733007076702016837564951956529003697497847838781899848384824991374342901164708655443686022921583406187082133141084994843502230809550055933825660668160300304112671478218513259983054489597176651737200716259733573469298437873515151377206364940530308167934399245072298875358347931404742292788785586833114480704138718996633638362933821933388459210678374952072108333767698704767907612549860590824123780096225591372365712106060039646448181221691765233478768574198237963457485496438076793333937013217675591500849193742006533651525421426481898699626618796271544860105422331629265388419155909716261466161258430","2":"59423006413504086085782234600502410213751379553855471973440165009200961757474676407242673622935614782362911290590560535490636029324125251850583605745046201217673654522625983661578962623803698461459190578519097656221453474955879823750445359506290522280566225253310030053812918275525607874059407284653434046369835156477189219911810464401689041140506062300317020407969423270374033482533711564673658146930272487464489365713112043565257807490520178903336328210031106311280471651300486164966423437275272281777742004535722142265580037959473078313965482591454009972765788975683031385823798895914265841131145707278751512534120","0":"56510878078818710798555570103060159621941668074271797077206591818472978018558098567975838757566260370093327989369045722406190165972775356924844244889146946158949660988214890388299203816110339909687790860564719380865809705044646711632599987968183128514431910561478715003212633874423067294596323864121737000450543142072142652163818450299889830999149821558252183477517484127000480272695698860647674027831262149565273068850774090998356019534296579838685977022988536930596918054160990243868372150609770079720240227817149126735182138479851227052696211125454858584118346950878092387488482897777914362341820607560926173967363","3":"63511079416489489495396586813126304469185174450150717746314545118902972011091412254834718868134635251731510764117528579641756327883640004345178347120290107941107152421856942264968771810665927914509411385404403747487862696526824127219640807008235054362138760656969613951620938020257273816713908815343872804442748694361381399025862438391456307852482826748664499083370705834755863016895566228300904018909174673301643617543662527772400085378252706897979609427451977654028887889811453690146157824251379525221390697200211891556653698308665831075787991412401737090471273439878635073797691350863566834141222438011402987450926","1":"30348838247529448929141877305241172943867610065951047292188826263950046630912426030349276970628525991007036685038199133783991618544554063310358191845473212966131475853690378885426974792306638181168558731807811629973716711132134244797541560013139884391800841941607502149630914097258613821336239993125960064136287579351403225717114920758719152701696123905042695943045383536065833292374624566478931465135875411483860059753175449604448434619593495399051968638830805689355610877075130302742512428461286121237297212174164897833936610857614962734658136750299346971377383141235020438750748045568800723867413392427848651081274"},"predicate":{"attr_name":"age","p_type":"GE","value":18}}]},"non_revoc_proof":null}},"aggregated_proof":{"c_hash":"81135772044295974649282368084258333955993271555081206390568996949836231116301","c_list":[[240,104,187,71,84,144,129,123,12,181,215,233,27,55,56,54,94,57,17,42,111,42,112,234,192,23,226,103,118,198,189,175,175,1,102,64,128,100,221,201,134,106,83,239,69,43,150,172,95,206,145,224,207,239,39,193,30,200,90,125,175,125,59,47,250,224,193,21,64,112,101,131,128,249,96,165,73,33,174,64,69,252,209,158,130,53,23,158,217,173,69,51,12,145,70,174,15,206,13,181,50,246,50,110,223,65,250,44,39,33,8,47,169,242,147,3,190,164,110,20,68,5,142,133,38,198,151,161,167,0,219,128,126,120,190,23,153,22,250,78,114,241,252,181,74,142,65,123,225,153,75,159,78,84,28,110,203,105,231,238,75,138,121,233,75,163,221,69,106,143,1,217,251,43,147,252,189,122,19,124,189,180,206,91,165,199,41,172,233,102,14,91,162,254,16,142,60,230,39,200,208,236,101,69,101,152,233,217,100,206,31,120,211,191,90,56,205,40,180,120,47,210,224,86,153,34,86,237,204,11,183,227,0,224,15,201,32,228,4,210,43,156,68,246,137,150,103,197,191,150,155,181,78,5,134,58],[1,214,184,139,205,251,132,131,8,186,140,58,211,242,134,120,121,253,128,192,10,252,172,101,44,26,119,56,212,8,248,71,19,96,59,12,233,191,63,187,217,35,191,160,127,247,189,247,229,111,252,101,126,10,142,252,238,215,211,137,137,164,114,186,255,199,183,50,103,9,158,63,134,140,162,154,188,109,52,31,92,78,38,228,0,60,225,100,239,88,114,95,48,71,7,117,168,45,45,177,178,62,87,197,98,174,123,249,26,237,179,12,63,182,46,218,183,148,163,222,179,159,146,56,142,190,122,100,211,6,86,237,10,7,111,186,27,66,95,252,108,247,203,1,111,60,13,218,104,63,128,125,197,11,201,138,33,122,37,31,163,123,120,132,65,122,208,60,80,87,113,183,28,31,74,106,18,79,52,245,113,184,94,202,72,223,8,128,209,43,77,237,119,208,255,144,26,76,223,77,177,131,237,49,150,251,53,150,115,33,254,237,185,15,140,234,205,99,248,252,171,245,192,104,151,194,190,186,249,180,246,9,169,165,0,221,7,107,39,67,58,178,176,99,212,40,247,49,127,7,94,5,170,65,154,28,104],[1,247,26,202,244,120,131,95,151,52,56,38,141,232,178,50,61,45,235,61,12,68,11,180,174,222,110,211,141,253,198,204,248,192,40,99,237,1,45,170,79,208,3,13,135,89,195,65,3,228,224,146,181,198,14,79,78,237,168,81,108,151,68,12,88,242,120,200,120,193,253,51,167,140,43,175,59,18,160,190,233,21,213,135,162,76,38,48,163,110,155,197,97,93,211,183,95,42,172,249,98,59,161,136,70,39,142,48,242,44,154,103,186,161,214,215,0,254,166,150,111,71,242,102,209,125,25,65,144,223,211,137,223,239,50,96,185,171,120,155,171,98,204,23,102,253,68,141,91,240,127,170,199,249,217,165,164,37,174,212,159,232,140,196,216,140,205,102,84,104,220,223,9,249,75,245,78,157,245,203,235,154,73,34,77,12,227,138,93,105,178,114,255,210,88,216,202,64,69,128,220,211,113,51,15,185,103,236,52,187,49,29,162,20,35,21,65,188,33,46,11,172,59,15,221,36,33,213,14,121,36,218,76,80,97,197,83,64,145,73,194,43,233,144,251,86,112,209,230,67,234,116,172,219,123,50,46],[1,114,216,159,37,214,198,117,230,153,15,176,95,20,29,134,179,207,209,35,101,193,47,54,130,141,78,213,54,167,31,73,105,177,129,135,6,135,45,107,103,16,133,187,74,217,42,40,1,214,60,70,78,245,86,82,150,75,91,235,181,249,129,147,202,15,86,250,222,240,203,236,102,39,53,147,79,178,124,184,97,73,65,136,74,29,219,182,83,167,221,203,32,200,243,130,65,234,133,181,203,35,86,21,123,170,74,174,5,132,1,149,77,141,158,193,249,130,37,53,253,234,228,144,66,152,232,246,26,193,6,53,139,45,231,173,115,87,89,61,197,9,96,73,229,189,49,44,203,214,156,139,58,153,77,13,90,35,157,130,184,150,161,69,145,157,4,206,52,216,227,233,113,202,54,154,153,100,83,97,135,88,197,227,42,52,28,221,91,117,56,183,198,102,231,37,232,226,136,142,115,218,175,45,221,143,130,215,184,39,102,172,126,253,152,108,254,241,17,98,70,223,191,138,251,227,243,32,180,190,223,69,135,0,97,105,115,189,221,134,26,159,32,210,172,233,7,65,238,77,203,159,181,188,203,159,190]]}},"requested_proof":{"revealed_attrs":{"attr1_referent":["claim::58479554-187f-40d9-b0a5-a95cfb0338c3","Alex","1139481716457488690172217916278103335"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{"predicate1_referent":"claim::58479554-187f-40d9-b0a5-a95cfb0338c3"}},"identifiers":{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{"issuer_did":"NcYxiDXkpYi6ov5FcYDi1e","schema_key":{"name":"gvt","version":"1.0","did":"NcYxiDXkpYi6ov5FcYDi1e"}}}}"#;

            let valid = AnoncredsUtils::verifier_verify_proof(AnoncredsUtils::proof_request_attr_and_predicate(),
                                                              &proof_json,
                                                              &schemas_json,
                                                              &claim_defs_json,
                                                              &revoc_regs_json).unwrap();
            assert!(!valid);
        }
    }
}

mod medium_cases {
    use super::*;

    mod issuer_create_and_store_claim_def {
        use super::*;

        #[test]
        fn issuer_create_and_store_claim_def_works_for_invalid_schema() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let schema = r#"{"seqNo":1, "name":"name","version":"1.0", "attr_names":["name"]}"#;

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, ISSUER_DID, &schema, None, false);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_invalid_did() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle,
                                                                     INVALID_IDENTIFIER,
                                                                     &AnoncredsUtils::gvt_schema_json(),
                                                                     None, false);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_empty_schema_attr_names() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let schema = r#"{"seqNo":1, "identifier":"NcYxiDXkpYi6ov5FcYDi1e", "data":{"name":"name","version":"1.0","attr_names":[]}}"#;

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &schema, ISSUER_DID, None, false);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_correct_signature_type() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            AnoncredsUtils::issuer_create_claim_definition(wallet_handle,
                                                           ISSUER_DID,
                                                           &AnoncredsUtils::gvt_schema_json(),
                                                           Some(SIGNATURE_TYPE), false).unwrap();
        }

        #[test]
        fn issuer_create_and_store_claim_def_works_for_invalid_signature_type() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle,
                                                                     ISSUER_DID,
                                                                     &AnoncredsUtils::gvt_schema_json(),
                                                                     Some("some_type"), false);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }


        #[test]
        fn issuer_create_and_store_claim_def_works_for_duplicate() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            AnoncredsUtils::issuer_create_claim_definition(wallet_handle,
                                                           ISSUER_DID,
                                                           &AnoncredsUtils::gvt_schema_json(),
                                                           None, false).unwrap();

            let res = AnoncredsUtils::issuer_create_claim_definition(wallet_handle,
                                                                     ISSUER_DID,
                                                                     &AnoncredsUtils::gvt_schema_json(),
                                                                     None, false);

            assert_eq!(res.unwrap_err(), ErrorCode::AnoncredsClaimDefAlreadyExistsError);
        }
    }

    mod prover_store_claim_offer {
        use super::*;

        #[test]
        fn prover_store_claim_offer_works_for_invalid_issuer_did() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let claim_offer_json = AnoncredsUtils::get_claim_offer("invalid_base58_string", &AnoncredsUtils::gvt_schema_key());

            let res = AnoncredsUtils::prover_store_claim_offer(wallet_handle, &claim_offer_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod prover_get_claim_offers {
        use super::*;

        #[test]
        fn prover_get_claim_offers_works_for_invalid_filter_json() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let res = AnoncredsUtils::prover_get_claim_offers(wallet_handle, r#"{"schema_key":"gvt"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_get_claim_offers_works_for_different_wallets() {
            AnoncredsUtils::init_common_wallet();

            let wallet_handle_1 = WalletUtils::create_and_open_wallet("pool1", None).unwrap();
            let wallet_handle_2 = WalletUtils::create_and_open_wallet("pool1", None).unwrap();


            AnoncredsUtils::prover_store_claim_offer(wallet_handle_1, &AnoncredsUtils::gvt_claim_offer()).unwrap();
            AnoncredsUtils::prover_store_claim_offer(wallet_handle_2, &AnoncredsUtils::xyz_claim_offer()).unwrap();
            AnoncredsUtils::prover_store_claim_offer(wallet_handle_2,
                                                     &AnoncredsUtils::get_claim_offer(DID, &AnoncredsUtils::xyz_schema_key())).unwrap();

            let claim_offers = AnoncredsUtils::prover_get_claim_offers(wallet_handle_2, &format!(r#"{{"issuer_did":"{}"}}"#, ISSUER_DID)).unwrap();
            let claim_offers: Vec<ClaimOfferInfo> = serde_json::from_str(&claim_offers).unwrap();

            assert_eq!(claim_offers.len(), 1);
            assert!(claim_offers.contains(&ClaimOfferInfo { issuer_did: ISSUER_DID.to_string(), schema_key: AnoncredsUtils::xyz_schema_key() }));
        }
    }

    mod prover_create_master_secret {
        use super::*;

        #[test]
        fn prover_create_master_secret_works_for_duplicate_name() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            AnoncredsUtils::prover_create_master_secret(wallet_handle, COMMON_MASTER_SECRET).unwrap();
            let res = AnoncredsUtils::prover_create_master_secret(wallet_handle, COMMON_MASTER_SECRET);
            assert_eq!(res.unwrap_err(), ErrorCode::AnoncredsMasterSecretDuplicateNameError);
        }

        #[test]
        fn prover_create_master_secret_works_for_empty_name() {
            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let res = AnoncredsUtils::prover_create_master_secret(wallet_handle, "");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidParam3);
        }
    }

    mod prover_create_and_store_claim_req {
        use super::*;

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_claim_offer() {
            let (wallet_handle, claim_def, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_offer_json = format!(r#"{{"schema_key":{}}}"#, AnoncredsUtils::gvt_schema_key_json());

            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        DID,
                                                                        &claim_offer_json,
                                                                        &claim_def,
                                                                        COMMON_MASTER_SECRET);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_claim_def() {
            let (wallet_handle, _, claim_offer, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_def = r#"{
                        "schema_seq_no":1,
                        "signature_type":"CL",
                        "primary":{
                            "n":"121212",
                            "s":"432192"
                        }
                    }"#;
            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        DID,
                                                                        &claim_offer,
                                                                        claim_def,
                                                                        COMMON_MASTER_SECRET);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_create_and_store_claim_req_works_for_invalid_master_secret() {
            let (wallet_handle, claim_def, claim_offer, _, _) = AnoncredsUtils::init_common_wallet();

            let res = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                        DID,
                                                                        &claim_offer,
                                                                        &claim_def,
                                                                        "invalid_master_secret_name");
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);
        }
    }

    mod issuer_create_claim {
        use super::*;

        #[test]
        fn issuer_create_claim_works_for_for_invalid_claim_req_json() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_req = format!(r#"{{
                                                "blinded_ms":{{"ur":null}},
                                                "prover_did":"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW",
                                                "issuer_did":"{}",
                                                "schema_key":{}
                                              }}"#, ISSUER_DID, AnoncredsUtils::gvt_schema_key_json());

            let res = AnoncredsUtils::issuer_create_claim(wallet_handle,
                                                          &claim_req,
                                                          &AnoncredsUtils::gvt_claim_values_json(),
                                                          None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn issuer_create_claim_works_for_for_invalid_claim_values_json() {
            let (wallet_handle, _, claim_offer, _, _) = AnoncredsUtils::init_common_wallet();

            let claim_values_json = r#"{
                                       "sex":"male",
                                       "name":"Alex",
                                       "height":"175",
                                       "age":"28"
                                     }"#;

            let res = AnoncredsUtils::issuer_create_claim(wallet_handle,
                                                          &claim_offer,
                                                          &claim_values_json,
                                                          None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod prover_store_claim {
        use super::*;

        #[test]
        fn prover_store_claim_works_without_claim_req() {
            let (wallet_handle, _, _, _, claim) = AnoncredsUtils::init_common_wallet();

            let mut claim: Claim = serde_json::from_str(&claim).unwrap();
            claim.issuer_did = DID_MY2.to_string();
            let claim_json = serde_json::to_string(&claim).unwrap();

            let res = AnoncredsUtils::prover_store_claim(wallet_handle, &claim_json, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);
        }

        #[test]
        fn prover_store_claim_works_for_invalid_claim_json() {
            let (wallet_handle, claim_def_json, claim_offer, _, _) = AnoncredsUtils::init_common_wallet();

            AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                              DID_MY1,
                                                              &claim_offer,
                                                              &claim_def_json,
                                                              COMMON_MASTER_SECRET).unwrap();

            let claim_json = format!(r#"{{
                                                   "values":{},
                                                   "issuer_did":"{}",
                                                   "revoc_reg_seq_no":null,
                                                   "schema_key":{}
                                               }}"#,
                                     AnoncredsUtils::gvt_claim_values_json(), ISSUER_DID, AnoncredsUtils::gvt_schema_key_json());

            let res = AnoncredsUtils::prover_store_claim(wallet_handle, &claim_json, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod prover_get_claims {
        use super::*;

        #[test]
        fn prover_get_claims_works_for_invalid_json() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let res = AnoncredsUtils::prover_get_claims(wallet_handle, r#"{"schema_key": "name"}"#);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod prover_get_claims_for_proof_req {
        use super::*;

        #[test]
        fn prover_get_claims_for_proof_req_works_for_invalid_proof_req() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                    "nonce":"123432421212",
                                    "name":"proof_req_1",
                                    "version":"0.1",
                                    "requested_predicates":{}
                              }"#;

            let res = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_invalid_predicate() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                        "nonce":"123432421212",
                                        "name":"proof_req_1",
                                        "version":"0.1",
                                        "requested_attrs":{},
                                        "requested_predicates":{"predicate1_referent":{"attr_name":"age"}}
                                    }"#;

            let res = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_get_claims_for_proof_req_works_for_invalid_predicate_type() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let proof_req = r#"{
                                        "nonce":"123432421212",
                                        "name":"proof_req_1",
                                        "version":"0.1",
                                        "requested_attrs":{},
                                        "requested_predicates":{"predicate1_referent":{"attr_name":"age","p_type":"<=","value":58}}
                                    }"#;

            let res = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, &proof_req);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod prover_create_proof_works {
        use super::*;

        #[test]
        fn prover_create_proof_works_for_invalid_master_secret() {
            let (wallet_handle, claim_def_json, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, AnoncredsUtils::proof_request_attr_and_predicate()).unwrap();
            let claim_for_attr = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

            let requested_claims_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                  "requested_predicates":{{}}
                                                }}"#, claim_for_attr.referent);

            let schemas_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, claim_def_json);
            let revoc_regs_json = "{}";

            let res = AnoncredsUtils::prover_create_proof(wallet_handle,
                                                          AnoncredsUtils::proof_request_attr_and_predicate(),
                                                          &requested_claims_json,
                                                          &schemas_json,
                                                          "invalid_master_secret_name",
                                                          &claim_defs_json,
                                                          &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletNotFoundError);
        }

        #[test]
        fn prover_create_proof_works_for_invalid_schemas_json() {
            let (wallet_handle, claim_def_json, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, AnoncredsUtils::proof_request_attr_and_predicate()).unwrap();
            let claim_for_attr = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

            let requested_claims_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                  "requested_predicates":{{}}
                                                }}"#, claim_for_attr.referent);

            let schemas_json = r#"{}"#;
            let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, claim_def_json);
            let revoc_regs_json = "{}";

            let res = AnoncredsUtils::prover_create_proof(wallet_handle,
                                                          AnoncredsUtils::proof_request_attr_and_predicate(),
                                                          &requested_claims_json,
                                                          &schemas_json,
                                                          COMMON_MASTER_SECRET,
                                                          &claim_defs_json,
                                                          &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_create_proof_works_for_invalid_claim_defs_json() {
            let (wallet_handle, _, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, AnoncredsUtils::proof_request_attr_and_predicate()).unwrap();
            let claim_for_attr = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

            let requested_claims_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                  "requested_predicates":{{}}
                                                }}"#, claim_for_attr.referent);

            let schemas_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = r#"{}"#;
            let revoc_regs_json = "{}";

            let res = AnoncredsUtils::prover_create_proof(wallet_handle,
                                                          AnoncredsUtils::proof_request_attr_and_predicate(),
                                                          &requested_claims_json,
                                                          &schemas_json,
                                                          COMMON_MASTER_SECRET,
                                                          &claim_defs_json,
                                                          &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn prover_create_proof_works_for_invalid_requested_claims_json() {
            let (wallet_handle, claim_def_json, _, _, _) = AnoncredsUtils::init_common_wallet();

            let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle, AnoncredsUtils::proof_request_attr_and_predicate()).unwrap();
            let claim_for_attr = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

            let requested_claims_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_predicates":{{}}
                                                }}"#);

            let schemas_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim_for_attr.referent, claim_def_json);
            let revoc_regs_json = "{}";

            let res = AnoncredsUtils::prover_create_proof(wallet_handle,
                                                          AnoncredsUtils::proof_request_attr_and_predicate(),
                                                          &requested_claims_json,
                                                          &schemas_json,
                                                          COMMON_MASTER_SECRET,
                                                          &claim_defs_json,
                                                          &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }

    mod verifier_verify_proof {
        use super::*;

        #[test]
        fn verifier_verify_proof_works_for_invalid_proof_json_format() {
            let schemas_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::gvt_schema_json());
            let claim_defs_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::claim_def_json());
            let revoc_regs_json = "{}";
            let proof_json = r#"{"proof":{"proofs":{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{"primary_proof":{"eq_proof":{"revealed_attrs":{"name":"1139481716457488690172217916278103335"},"a_prime":"80401564260558483983794628158664845806393125691167675024527906210615204776868092566789307767601325086260531777605457298059939671624755239928848057947875953445797869574854365751051663611984607735255307096920094357120779812375573500489773454634756645206823074153240319316758529163584251907107473703779754778699279153037094140428648169418133281187947677937472972061954089873405836249023133445286756991574802740614183730141450546881449500189789970102133738133443822618072337620343825908790734460412932921199267304555521397418007577171242880211812703320270140386219809818196744216958369397014610013338422295772654405475023","e":"31151798717381512709903464053695613005379725796031086912986270617392167764097422442809244590980303622977555221812111085160553241592792901","v":"524407431684833626723631303096063196973911986967748096669183384949467719053669910411426601230736351335262754473490498825342793551112426427823428399937548938048089615644972537564428344526295733169691240937176356626523864731701111189536269488496019586818879697981955044502664124964896796783428945944075084807859935155837238670987272778459356531608865162828109489758902085206073584532002909678902616210042778963974064479140826712481297584040209095459963718975102750913306565864485279810056629704077428898739021040190774575868853629858297299392839284660771662690107106553362040805152261505268111067408422298806905178826507224233050991301274817252924123120887017757639206512015559321675322509820081151404696713509158685022511201565062671933414307463988209696457343022378430051265752251403461414881325357657438328740471164157220698425309006894962942640219890219594168419276308074677144722217081026358892787770650248878952483621","m":{"age":"10477979077744818183854012231360633424177093192344587159214818537659704987539982653663361680650769087122324965941845552897155693994859927792964720675888893623940580527766661802170","sex":"15368219775809326116045200104269422566086585069798988383076685221700842794654771075432385446820819836777771517356551059931242867733879324915651894894695726945279462946826404864068","height":"268172143999991481637372321419290603042446269013750825098514042757459298040087626745653681785038933035820421862976371452111736537699176931068992453946771945552540798204580069806"},"m1":"119095745403940293668103184388411799541118279558928018597628509118163496000813590825371995586347826189221837428823000332905316924389185590810015031744029496470545254805993327676570037596326743185389101389800942263689809725968264069601565478411709555274081560719927118853299543998608664701485475703881376151770","m2":"3166313665375815600922385342096456465402430622944571045536207479553790085339726549928012930073803465171492637049498407367742103524723152099973753540483894420905314750248333232361"},"ge_proofs":[{"u":{"2":"6494171529848192644197417834173236605253723188808961394289041396341136802965710957759175642924978223517091081898946519122412445399638640485278379079647638538597635045303985779767","0":"7739508859260491061487569748588091139318989278758566530899756574128579312557203413565436003310787878172471425996601979342157451689172171025305431595131816910273398879776841751855","3":"9424758820140378077609053635383940574362083113571024891496206162696034958494400871955445981458978146571146602763357500412840538526390475379772903513687358736287298159312524034159","1":"9011979414559555265454106061917684716953356440811838475257096756618761731111646531136628099710567973381801256908067529269805992222342928842825929421929485785888403149296320711642"},"r":{"DELTA":"2119857977629302693157808821351328058251440215802746362450951329352726877165815663955490999790457576333458830301801261754696823614762123890412904169206391143688952648566814660498520188221060505840151491403269696751525874990487604723445355651918681212361562384420233903265612599812725766212744963540390806334870022328290970137051148373040320927100063898502086531019924715927190306801273252711777648467224661735618842887006436195147540705753550974655689586750013569294343535843195025962867299786380033532422131203367401906988124836294104501525520053613392691214421562815044433237816093079784307397782961917892254668290115653012265908717124278607660504580036193346698672079435538219972121355893074219968755049500875222141","2":"879097501989202140886939888802566536179834329508897124489020677433754766947767937608431979796722207676629625451150104784909666168153917345813160237337412296010679353735699663083287427507870244565918756969618964144516025526404618052053542009438548457492400344119561349471929199757453154204191407620539220514897529346602664135146454509169680801061111878075145734123580343470361019624175036825631373890661124315134340427076598351080893567995392248394683875116715114577054906406649006122102488431184007790011073389768061904597267545895265921673106871142463561948479668876241841045522543174660428236658891636170119227855493059358614089146415798861053408542832475696099851160385105386001523305465829676723036394820593263477","0":"1724016272047416140958096373304304971004826284109046259544344355102178044512441391364907122486655755929044720001281832600729467778103556397960700809066582436321515744527550472324028227472294258045699756170293405547851344921626775854114063087070898499913846456795761213291925373770081490280103876827479351849800210782799381740073719081199000612284788683993320623339686128531187019125095700122135094060470612862911102824801065698176788174959069186600426519872015152034176356923049531650418553748519941342115963599848111324793380438600664408464987023646615003553912544410140730587797458882329021327455905737414352355326238028222782957735440607899424838572541602600159016542488644761584240884783618700311735467659132540546","3":"2317535203964314926167241523636020444600002667629517624482931328850422196008281300859516069440995466415138723103558631951648519232327284208990029010060986032518946759289078833125920310350676484457972303378558158127406345804560689086460633931717939234025886786468170219981598030245042011840614339386724945679531091642132820284896626191109974537171662283750959028046143650291367908660204201563611944187723824430780626387525165408619587771059635528553832034409311888615502905143628507219523591091412192645348525327725381323865648645828460581593542176351568614465903523790649219812666979685223535464526901006270478687017672202058914176692964406859722580270696925877498058525086810338471380117323227744481903228027847825795","1":"1119193929864813751243160041764170298897380522230946444206167281178657213260394833843687899872857393015947283159245092452814155776571829885921814072299525859857844030379558685168895306445277750249341844789101670896570226707650318347992386244538723699686941887792682779028216548922683313576597384354842537728667739985216662699631842296096507821667149950956179957306177525178260912379909156360834120816956949271530622510333943914411903103069247646327625753995178999023427645468623522280255892736633780185163496867644317005801241786702434621502492159672660131289312665511793827552317714835658019088880972220344126692027952749318018900669839090109361161616086319604439015851316798257015063653414161203599184730094765941653"},"mj":"10477979077744818183854012231360633424177093192344587159214818537659704987539982653663361680650769087122324965941845552897155693994859927792964720675888893623940580527766661802170","alpha":"46280660038407959140964701167450659223532556136388451390393713283900546119670373626221864441898929302821705811144923685080534692512705456699843367809872982836890616398604933641265111106644805368974824737276965928297120628041257593166650593538539384316563258781595629888673792430276007730792093088812056156937735120078929629310611907731935101448992312370312134173482115524436767558802102266208152808607480693236511858269018733175523724309089010048330044458187371675333889670055578652283806685440133357512406700879353713629795062705271430695988191782837658895477702634883214188598350625843489120361660836956958750828038278027538830855628653513539929730230905015331221220017847248793929813230252015802389329428995718799619565984669228143200627972926117282688854152516298117476837960100343260648687249027349308513966440386556698667484082658689","t":{"DELTA":"46814992964714978733007076702016837564951956529003697497847838781899848384824991374342901164708655443686022921583406187082133141084994843502230809550055933825660668160300304112671478218513259983054489597176651737200716259733573469298437873515151377206364940530308167934399245072298875358347931404742292788785586833114480704138718996633638362933821933388459210678374952072108333767698704767907612549860590824123780096225591372365712106060039646448181221691765233478768574198237963457485496438076793333937013217675591500849193742006533651525421426481898699626618796271544860105422331629265388419155909716261466161258430","2":"59423006413504086085782234600502410213751379553855471973440165009200961757474676407242673622935614782362911290590560535490636029324125251850583605745046201217673654522625983661578962623803698461459190578519097656221453474955879823750445359506290522280566225253310030053812918275525607874059407284653434046369835156477189219911810464401689041140506062300317020407969423270374033482533711564673658146930272487464489365713112043565257807490520178903336328210031106311280471651300486164966423437275272281777742004535722142265580037959473078313965482591454009972765788975683031385823798895914265841131145707278751512534120","0":"56510878078818710798555570103060159621941668074271797077206591818472978018558098567975838757566260370093327989369045722406190165972775356924844244889146946158949660988214890388299203816110339909687790860564719380865809705044646711632599987968183128514431910561478715003212633874423067294596323864121737000450543142072142652163818450299889830999149821558252183477517484127000480272695698860647674027831262149565273068850774090998356019534296579838685977022988536930596918054160990243868372150609770079720240227817149126735182138479851227052696211125454858584118346950878092387488482897777914362341820607560926173967363","3":"63511079416489489495396586813126304469185174450150717746314545118902972011091412254834718868134635251731510764117528579641756327883640004345178347120290107941107152421856942264968771810665927914509411385404403747487862696526824127219640807008235054362138760656969613951620938020257273816713908815343872804442748694361381399025862438391456307852482826748664499083370705834755863016895566228300904018909174673301643617543662527772400085378252706897979609427451977654028887889811453690146157824251379525221390697200211891556653698308665831075787991412401737090471273439878635073797691350863566834141222438011402987450926","1":"30348838247529448929141877305241172943867610065951047292188826263950046630912426030349276970628525991007036685038199133783991618544554063310358191845473212966131475853690378885426974792306638181168558731807811629973716711132134244797541560013139884391800841941607502149630914097258613821336239993125960064136287579351403225717114920758719152701696123905042695943045383536065833292374624566478931465135875411483860059753175449604448434619593495399051968638830805689355610877075130302742512428461286121237297212174164897833936610857614962734658136750299346971377383141235020438750748045568800723867413392427848651081274"},"predicate":{"attr_name":"age","p_type":"GE","value":18}}]},"non_revoc_proof":null}},"aggregated_proof":{"c_hash":"81135772044295974649282368084258333955993271555081206390568996949836231116301","c_list":[[2,124,231,47,189,36,247,160,61,220,165,35,97,165,203,185,133,253,81,239,67,127,156,49,189,16,140,30,177,161,221,54,154,0,127,143,98,212,114,193,188,85,206,171,198,140,9,192,10,254,218,120,201,182,40,141,80,35,81,148,204,192,41,5,186,33,50,77,211,163,124,130,32,219,193,167,79,43,181,76,19,249,53,79,70,221,205,36,180,50,120,255,161,227,196,204,71,106,221,131,220,7,73,86,128,208,48,58,123,63,82,24,170,141,143,56,221,96,151,108,105,38,185,243,224,112,177,101,195,87,208,201,39,123,165,125,92,104,234,188,54,92,31,158,178,152,52,205,26,156,237,241,23,15,76,220,168,32,175,230,157,197,225,70,57,237,8,81,13,17,95,70,143,56,162,223,203,8,48,153,51,51,118,116,32,139,187,222,146,86,165,111,125,107,203,18,212,28,168,22,62,69,204,207,122,148,25,30,92,120,83,214,116,221,204,120,230,70,128,139,181,110,69,93,253,240,69,16,113,224,246,41,142,0,83,237,186,4,50,156,206,199,89,74,96,168,249,240,101,16,103,234,162,219,52,218,207],[1,191,167,2,151,36,61,136,184,172,120,86,127,88,109,119,56,21,167,171,217,221,24,64,246,237,255,152,81,183,201,191,59,234,213,101,254,91,33,205,120,71,215,144,160,243,145,109,19,151,241,46,135,132,50,143,219,207,197,35,89,103,83,212,96,83,222,101,55,57,220,161,252,115,39,62,46,160,30,138,221,89,125,66,114,150,5,95,63,10,55,107,102,73,40,69,41,6,57,0,64,226,152,66,181,149,251,50,28,53,18,26,221,5,188,67,125,184,190,200,56,92,132,201,242,211,37,2,43,6,146,88,228,120,204,190,4,118,134,106,118,110,249,145,175,165,116,197,200,183,207,215,197,79,207,203,29,182,231,151,248,233,107,41,79,234,250,27,33,33,107,102,240,47,37,230,243,185,93,192,52,31,73,211,11,173,150,92,194,154,172,247,221,206,129,85,193,105,172,140,201,40,240,200,28,94,1,96,204,175,113,170,46,134,229,111,215,208,237,252,84,50,249,41,214,79,38,194,23,212,7,164,153,217,23,252,32,114,145,58,189,118,104,131,84,184,115,175,199,227,219,117,23,113,113,180,3],[240,104,187,71,84,144,129,123,12,181,215,233,27,55,56,54,94,57,17,42,111,42,112,234,192,23,226,103,118,198,189,175,175,1,102,64,128,100,221,201,134,106,83,239,69,43,150,172,95,206,145,224,207,239,39,193,30,200,90,125,175,125,59,47,250,224,193,21,64,112,101,131,128,249,96,165,73,33,174,64,69,252,209,158,130,53,23,158,217,173,69,51,12,145,70,174,15,206,13,181,50,246,50,110,223,65,250,44,39,33,8,47,169,242,147,3,190,164,110,20,68,5,142,133,38,198,151,161,167,0,219,128,126,120,190,23,153,22,250,78,114,241,252,181,74,142,65,123,225,153,75,159,78,84,28,110,203,105,231,238,75,138,121,233,75,163,221,69,106,143,1,217,251,43,147,252,189,122,19,124,189,180,206,91,165,199,41,172,233,102,14,91,162,254,16,142,60,230,39,200,208,236,101,69,101,152,233,217,100,206,31,120,211,191,90,56,205,40,180,120,47,210,224,86,153,34,86,237,204,11,183,227,0,224,15,201,32,228,4,210,43,156,68,246,137,150,103,197,191,150,155,181,78,5,134,58],[1,214,184,139,205,251,132,131,8,186,140,58,211,242,134,120,121,253,128,192,10,252,172,101,44,26,119,56,212,8,248,71,19,96,59,12,233,191,63,187,217,35,191,160,127,247,189,247,229,111,252,101,126,10,142,252,238,215,211,137,137,164,114,186,255,199,183,50,103,9,158,63,134,140,162,154,188,109,52,31,92,78,38,228,0,60,225,100,239,88,114,95,48,71,7,117,168,45,45,177,178,62,87,197,98,174,123,249,26,237,179,12,63,182,46,218,183,148,163,222,179,159,146,56,142,190,122,100,211,6,86,237,10,7,111,186,27,66,95,252,108,247,203,1,111,60,13,218,104,63,128,125,197,11,201,138,33,122,37,31,163,123,120,132,65,122,208,60,80,87,113,183,28,31,74,106,18,79,52,245,113,184,94,202,72,223,8,128,209,43,77,237,119,208,255,144,26,76,223,77,177,131,237,49,150,251,53,150,115,33,254,237,185,15,140,234,205,99,248,252,171,245,192,104,151,194,190,186,249,180,246,9,169,165,0,221,7,107,39,67,58,178,176,99,212,40,247,49,127,7,94,5,170,65,154,28,104],[1,247,26,202,244,120,131,95,151,52,56,38,141,232,178,50,61,45,235,61,12,68,11,180,174,222,110,211,141,253,198,204,248,192,40,99,237,1,45,170,79,208,3,13,135,89,195,65,3,228,224,146,181,198,14,79,78,237,168,81,108,151,68,12,88,242,120,200,120,193,253,51,167,140,43,175,59,18,160,190,233,21,213,135,162,76,38,48,163,110,155,197,97,93,211,183,95,42,172,249,98,59,161,136,70,39,142,48,242,44,154,103,186,161,214,215,0,254,166,150,111,71,242,102,209,125,25,65,144,223,211,137,223,239,50,96,185,171,120,155,171,98,204,23,102,253,68,141,91,240,127,170,199,249,217,165,164,37,174,212,159,232,140,196,216,140,205,102,84,104,220,223,9,249,75,245,78,157,245,203,235,154,73,34,77,12,227,138,93,105,178,114,255,210,88,216,202,64,69,128,220,211,113,51,15,185,103,236,52,187,49,29,162,20,35,21,65,188,33,46,11,172,59,15,221,36,33,213,14,121,36,218,76,80,97,197,83,64,145,73,194,43,233,144,251,86,112,209,230,67,234,116,172,219,123,50,46],[1,114,216,159,37,214,198,117,230,153,15,176,95,20,29,134,179,207,209,35,101,193,47,54,130,141,78,213,54,167,31,73,105,177,129,135,6,135,45,107,103,16,133,187,74,217,42,40,1,214,60,70,78,245,86,82,150,75,91,235,181,249,129,147,202,15,86,250,222,240,203,236,102,39,53,147,79,178,124,184,97,73,65,136,74,29,219,182,83,167,221,203,32,200,243,130,65,234,133,181,203,35,86,21,123,170,74,174,5,132,1,149,77,141,158,193,249,130,37,53,253,234,228,144,66,152,232,246,26,193,6,53,139,45,231,173,115,87,89,61,197,9,96,73,229,189,49,44,203,214,156,139,58,153,77,13,90,35,157,130,184,150,161,69,145,157,4,206,52,216,227,233,113,202,54,154,153,100,83,97,135,88,197,227,42,52,28,221,91,117,56,183,198,102,231,37,232,226,136,142,115,218,175,45,221,143,130,215,184,39,102,172,126,253,152,108,254,241,17,98,70,223,191,138,251,227,243,32,180,190,223,69,135,0,97,105,115,189,221,134,26,159,32,210,172,233,7,65,238,77,203,159,181,188,203,159,190]]}},"requested_proof":{"revealed_attrs":{"attr1_referent":["claim::58479554-187f-40d9-b0a5-a95cfb0338c3","Alex","1139481716457488690172217916278103335"]},"unrevealed_attrs":{},"self_attested_attrs":{},"predicates":{"predicate1_referent":"claim::58479554-187f-40d9-b0a5-a95cfb0338c3"}}}"#;

            let res = AnoncredsUtils::verifier_verify_proof(AnoncredsUtils::proof_request_attr_and_predicate(),
                                                            &proof_json,
                                                            &schemas_json,
                                                            &claim_defs_json,
                                                            &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn verifier_verify_proof_works_for_invalid_schemas() {
            let schemas_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::xyz_schema_json());
            let claim_defs_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::claim_def_json());
            let revoc_regs_json = "{}";

            let res = AnoncredsUtils::verifier_verify_proof(AnoncredsUtils::proof_request_attr_and_predicate(),
                                                            AnoncredsUtils::proof_json(),
                                                            &schemas_json,
                                                            &claim_defs_json,
                                                            &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }

        #[test]
        fn verifier_verify_proof_works_for_invalid_claim_defs() {
            let claim_defs_json = format!("{{}}");
            let schemas_json = format!(r#"{{"claim::58479554-187f-40d9-b0a5-a95cfb0338c3":{}}}"#, AnoncredsUtils::xyz_schema_json());
            let revoc_regs_json = "{}";

            let res = AnoncredsUtils::verifier_verify_proof(AnoncredsUtils::proof_request_attr_and_predicate(),
                                                            AnoncredsUtils::proof_json(),
                                                            &schemas_json,
                                                            &claim_defs_json,
                                                            &revoc_regs_json);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);
        }
    }
}

mod demos {
    use super::*;
    #[cfg(feature = "interoperability_tests")]
    use utils::types::ClaimDefinitionData;
    #[cfg(feature = "interoperability_tests")]
    use std::process::Command;
    #[cfg(feature = "interoperability_tests")]
    use std::io::prelude::*;
    #[cfg(feature = "interoperability_tests")]
    use std::net::TcpStream;
    #[cfg(feature = "interoperability_tests")]
    use std::{thread, time};

    #[cfg(feature = "interoperability_tests")]
    #[test]
    fn interoperability_test_pyindy_is_issuer() {
        TestUtils::cleanup_storage();

        //1. Create Prover wallet, get wallet handle
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, Some(TYPE)).unwrap();

        let schema = AnoncredsUtils::gvt_schema_json();

        //2. Prover create Master Secret
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //3. Prover store Claim Offer received from Issuer
        let claim_offer_json = AnoncredsUtils::get_claim_offer(ISSUER_DID, AnoncredsUtils::gvt_schema_key());
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json).unwrap();

        //4. Prover get Claim Offers
        let filter_json = format!(r#"{{"issuer_did":"{}"}}"#, ISSUER_DID);
        let claim_offers_json = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, &filter_json).unwrap();

        Command::new("python3")
            .arg("/home/indy/indy-anoncreds/anoncreds/test/test_interoperability_with_libindy_pyindy_is_issuer.py")
            .spawn().expect("failed to execute process");
        thread::sleep(time::Duration::from_millis(3000));

        let mut stream = TcpStream::connect("127.0.0.1:1234").unwrap();

        let _ = stream.write(r#"{"type":"get_claim_def"}"#.as_bytes());
        let mut buf = vec![0; 10240];
        stream.read(&mut buf).unwrap();
        buf.retain(|&element| element != 0);

        let claim_def_data: ClaimDefinitionData = serde_json::from_str(&String::from_utf8(buf).unwrap()).unwrap();

        let claim_def = ClaimDefinition {
            issuer_did: ISSUER_DID.to_string(),
            signature_type: "CL".to_string(),
            schema_seq_no: GVT_SEQ_NO,
            data: claim_def_data
        };

        let claim_def_json = serde_json::to_string(&claim_def).unwrap();

        //5. Prover create Claim Request
        let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                          DID_MY1,
                                                                          &claim_offer_json,
                                                                          &claim_def_json,
                                                                          COMMON_MASTER_SECRET).unwrap();

        let _ = stream.write(format!(r#"{{"type":"issue_claim", "data": {}}}"#, claim_req).as_bytes());
        let mut buf = vec![0; 10240];
        stream.read(&mut buf).unwrap();
        let _ = stream.write(r#"{"type":"close"}"#.as_bytes());
        buf.retain(|&element| element != 0);

        let mut claim_json: Claim = serde_json::from_str(&String::from_utf8(buf).unwrap()).unwrap();
        claim_json.schema_seq_no = Some(schema_seq_no);
        claim_json.issuer_did = Some(ISSUER_DID.to_string());

        // 6. Prover store received Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &serde_json::to_string(&claim_json).unwrap(), None).unwrap();

        // 7. Prover gets Claims for Proof Request
        let proof_req_json = r#"{{
                                   "nonce":"123432421212",
                                   "name":"proof_req_1",
                                   "version":"0.1",
                                   "requested_attrs":{{
                                        "attr1_referent":{{"name":"name"}},
                                        "attr2_referent":{{"name":"sex"}},
                                        "attr3_referent":{{"name":"phone"}}
                                   }},
                                   "requested_predicates":{{
                                        "predicate1_referent":{{"attr_name":"age","p_type":">=","value":18}}
                                   }}
                                }}"#;

        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let claim = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

        // 8. Prover create Proof
        let self_attested_value = "8-800-300";
        let requested_claims_json = format!(r#"{{
                                          "self_attested_attributes":{{"attr3_referent":"{}"}},
                                          "requested_attrs":{{
                                                "attr1_referent":["{}",true],
                                                "attr2_referent":["{}", false]
                                          }},
                                          "requested_predicates":{{"predicate1_referent":"{}"}}
                                        }}"#, self_attested_value, claim.referent, claim.referent, claim.referent);

        let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema);
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
        let revoc_regs_jsons = "{}";

        let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                             &proof_req_json,
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_jsons).unwrap();

        let proof: FullProof = serde_json::from_str(&proof_json).unwrap();

        let &(_, ref value, _) = proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap();
        assert_eq!(value, "Alex");

        proof.requested_proof.unrevealed_attrs.get("attr2_referent").unwrap();

        let value = proof.requested_proof.self_attested_attrs.get("attr3_referent").unwrap();
        assert_eq!(value, self_attested_value);

        // 9. Verifier verify proof
        let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                          &proof_json,
                                                          &schemas_json,
                                                          &claim_defs_json,
                                                          &revoc_regs_jsons).unwrap();
        assert!(valid);

        WalletUtils::close_wallet(prover_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[cfg(feature = "interoperability_tests")]
    #[test]
    fn interoperability_test_pyindy_is_verifier() {
        TestUtils::cleanup_storage();

        //1. Create Issuer wallet, get wallet handle
        let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, Some(TYPE)).unwrap();

        //2. Create Prover wallet, get wallet handle
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, Some(TYPE)).unwrap();

        //3. Issuer create claim definition
        let schema = AnoncredsUtils::gvt_schema_json();

        let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, &ISSUER_DID, &schema, None, false).unwrap();

        Command::new("python3")
            .arg("/home/indy/indy-anoncreds/anoncreds/test/test_interoperability_with_libindy_pyindy_is_verifier.py")
            .spawn().expect("failed to execute process");
        thread::sleep(time::Duration::from_millis(3000));

        let mut stream = TcpStream::connect("127.0.0.1:1234").unwrap();

        let _ = stream.write(format!(r#"{{"type":"receive_claim_def", "data": {}}}"#, claim_def_json).as_bytes());

        //4. Prover create Master Secret
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Prover store Claim Offer received from Issuer
        let claim_offer_json = AnoncredsUtils::get_claim_offer(ISSUER_DID, GVT_SEQ_NO);
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json).unwrap();

        //7. Prover create Claim Request
        let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                          DID_MY1,
                                                                          &claim_offer_json,
                                                                          &claim_def_json,
                                                                          COMMON_MASTER_SECRET).unwrap();

        //8. Issuer create Claim
        let claim_values_json = AnoncredsUtils::gvt_claim_values_json();
        let (_, xclaim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                   &claim_req,
                                                                   &claim_values_json, None).unwrap();

        // 9. Prover store received Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xclaim_json, None).unwrap();

        let _ = stream.write(r#"{"type":"get_proof_request"}"#.as_bytes());
        let mut buf = vec![0; 10240];
        stream.read(&mut buf).unwrap();
        buf.retain(|&element| element != 0);

        let proof_req_json = String::from_utf8(buf).unwrap();

        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let claim = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr_referent");

        // 11. Prover create Proof
        let self_attested_value = "value";
        let requested_claims_json = format!(r#"{{
                                              "self_attested_attributes":{{"self1":"{}"}},
                                              "requested_attrs":{{"attr_referent":["{}",true]}},
                                              "requested_predicates":{{"predicate_referent":"{}"}}
                                            }}"#, self_attested_value, claim.referent, claim.referent);

        let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema);
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
        let revoc_regs_jsons = "{}";

        let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                             &proof_req_json,
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_jsons).unwrap();

        let _ = stream.write(format!(r#"{{"type":"check_proof", "data": {}}}"#, proof_json).as_bytes());
        let mut buf = vec![0; 102400];
        stream.read(&mut buf).unwrap();
        let _ = stream.write(r#"{"type":"close"}"#.as_bytes());
        buf.retain(|&element| element != 0);

        let valid = String::from_utf8(buf).unwrap();
        assert_eq!("true", valid);

        WalletUtils::close_wallet(prover_wallet_handle).unwrap();
        WalletUtils::close_wallet(issuer_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[cfg(feature = "interoperability_tests")]
    #[test]
    fn interoperability_test_pyindy_is_prover() {
        TestUtils::cleanup_storage();

        let schema = AnoncredsUtils::gvt_schema_json();

        Command::new("python3")
            .arg("/home/indy/indy-anoncreds/anoncreds/test/test_interoperability_with_libindy_pyindy_is_prover.py")
            .spawn().expect("failed to execute process");
        thread::sleep(time::Duration::from_millis(3000));

        let mut stream = TcpStream::connect("127.0.0.1:1234").unwrap();

        let _ = stream.write(r#"{"type":"get_claim_def"}"#.as_bytes());
        let mut buf = vec![0; 10240];
        stream.read(&mut buf).unwrap();
        buf.retain(|&element| element != 0);

        let claim_def_data: ClaimDefinitionData = serde_json::from_str(&String::from_utf8(buf).unwrap()).unwrap();

        let claim_def = ClaimDefinition {
            issuer_did: ISSUER_DID.to_string(),
            signature_type: "CL".to_string(),
            schema_seq_no: GVT_SEQ_NO,
            data: claim_def_data
        };

        let claim_def_json = serde_json::to_string(&claim_def).unwrap();

        // 7. Prover gets Claims for Proof Request
        let proof_req_json = r#"{{
                                           "nonce":"123432421212",
                                           "name":"proof_req_1",
                                           "version":"0.1",
                                           "requested_attrs":{{
                                                "attr_referent":{{"name":"name"}}
                                           }},
                                           "requested_predicates":{{
                                                "predicate_referent":{{"attr_name":"age","p_type":">=","value":18}}
                                           }}
                                        }}"#;

        let _ = stream.write(format!(r#"{{"type":"get_proof", "data": {}}}"#, proof_req_json).as_bytes());
        let mut buf = vec![0; 102400];
        stream.read(&mut buf).unwrap();
        buf.retain(|&element| element != 0);

        let proof: FullProof = serde_json::from_str(&String::from_utf8(buf).unwrap()).unwrap();
        println!("proof: {:?}", proof);

        let _ = stream.write(r#"{"type":"close"}"#.as_bytes());
        let schemas_json = format!(r#"{{"{}":{}}}"#, 1, schema);

        let &(_, ref value, _) = proof.requested_proof.revealed_attrs.get("attr_referent").unwrap();
        assert_eq!(value, "Alex");

        let proof_json = serde_json::to_string(&proof).unwrap();
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, 1, claim_def_json);
        let revoc_regs_jsons = "{}";

        // 9. Verifier verify proof
        let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                          &proof_json,
                                                          &schemas_json,
                                                          &claim_defs_json,
                                                          &revoc_regs_jsons).unwrap();
        assert!(valid);

        TestUtils::cleanup_storage();
    }

    #[test]
    fn verifier_verify_proof_works_for_proof_does_not_correspond_proof_request_attr_and_predicate() {
        TestUtils::cleanup_storage();

        // 1. Creates wallet, gets wallet handle
        let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        // 2. Issuer creates claim definition
        let schema_json = AnoncredsUtils::gvt_schema_json();
        let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(wallet_handle, &ISSUER_DID, &schema_json, None, false).unwrap();

        // 3. Prover creates Master Secret
        AnoncredsUtils::prover_create_master_secret(wallet_handle, COMMON_MASTER_SECRET).unwrap();

        // 4. Issuer creates Claim Offer
        let claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(wallet_handle, &schema_json, ISSUER_DID, DID_MY1).unwrap();

        // 5. Prover creates Claim Request
        let claim_req_json = AnoncredsUtils::prover_create_and_store_claim_req(wallet_handle,
                                                                               DID_MY1,
                                                                               &claim_offer_json,
                                                                               &claim_def_json,
                                                                               COMMON_MASTER_SECRET).unwrap();

        // 6. Issuer creates Claim
        let (_, claim_json) = AnoncredsUtils::issuer_create_claim(wallet_handle,
                                                                  &claim_req_json,
                                                                  &AnoncredsUtils::gvt_claim_values_json(),
                                                                  None).unwrap();

        // 7. Prover stores received Claim
        AnoncredsUtils::prover_store_claim(wallet_handle, &claim_json, None).unwrap();

        // 8. Prover gets Claims for Proof Request

        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(wallet_handle,
                                                                          &AnoncredsUtils::proof_request_attr()).unwrap();
        let claim = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

        // 9. Prover creates Proof
        let requested_claims_json = format!(r#"{{
                                                          "self_attested_attributes":{{}},
                                                          "requested_attrs":{{"attr1_referent":["{}",true]}},
                                                          "requested_predicates":{{}}
                                                      }}"#, claim.referent);

        let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema_json);
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
        let revoc_regs_json = "{}";

        let proof_json = AnoncredsUtils::prover_create_proof(wallet_handle,
                                                             &AnoncredsUtils::proof_request_attr(),
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_json).unwrap();

        // 10. Verifier verifies proof
        let res = AnoncredsUtils::verifier_verify_proof(&AnoncredsUtils::proof_request_attr_and_predicate(),
                                                        &proof_json,
                                                        &schemas_json,
                                                        &claim_defs_json,
                                                        &revoc_regs_json);
        assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

        WalletUtils::close_wallet(wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[test]
    fn anoncreds_works_for_single_issuer_single_prover() {
        TestUtils::cleanup_storage();

        //1. Create Issuer wallet, gets wallet handle
        let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //2. Create Prover wallet, gets wallet handle
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //3. Issuer creates claim definition
        let schema_json = AnoncredsUtils::gvt_schema_json();
        let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, ISSUER_DID, &schema_json, None, false).unwrap();

        //4. Prover creates Master Secret
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Issuer creates Claim Offer
        let claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_wallet_handle, &schema_json, ISSUER_DID, DID_MY1).unwrap();

        //6. Prover stores Claim Offer received from Issuer
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json).unwrap();

        //7. Prover get Claim Offers
        let claim_offers_json = AnoncredsUtils::prover_get_claim_offers(prover_wallet_handle, "{}").unwrap();
        let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
        let claim_offer_json = serde_json::to_string(&claim_offers[0]).unwrap();

        //8. Prover creates Claim Request
        let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                          DID_MY1,
                                                                          &claim_offer_json,
                                                                          &claim_def_json,
                                                                          COMMON_MASTER_SECRET).unwrap();

        //9. Issuer creates Claim
        let (_, claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                  &claim_req,
                                                                  &AnoncredsUtils::gvt_claim_values_json(),
                                                                  None).unwrap();

        // 10. Prover stores received Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &claim_json, None).unwrap();

        // 11. Prover gets Claims for Proof Request
        let proof_req_json = format!(r#"{{
                                       "nonce":"123432421212",
                                       "name":"proof_req_1",
                                       "version":"0.1",
                                       "requested_attrs":{{
                                            "attr1_referent":{{
                                                "name":"name", "restrictions":[{{"schema_key":{}}}]
                                            }},
                                            "attr2_referent":{{
                                                "name":"sex", "restrictions":[{{"schema_key":{}}}]
                                            }},
                                            "attr3_referent":{{"name":"phone"}}
                                        }},
                                       "requested_predicates":{{
                                            "predicate1_referent":{{"attr_name":"age","p_type":">=","value":18}}
                                        }}
                                    }}"#, AnoncredsUtils::gvt_schema_key_json(), AnoncredsUtils::gvt_schema_key_json());

        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let claim = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

        // 12. Prover creates Proof
        let self_attested_value = "8-800-300";
        let requested_claims_json = format!(r#"{{
                                              "self_attested_attributes":{{"attr3_referent":"{}"}},
                                              "requested_attrs":{{
                                                    "attr1_referent":["{}", true],
                                                    "attr2_referent":["{}", false]
                                              }},
                                              "requested_predicates":{{
                                                    "predicate1_referent":"{}"
                                              }}
                                            }}"#, self_attested_value, claim.referent, claim.referent, claim.referent);

        let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema_json);
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
        let revoc_regs_json = "{}";

        let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                             &proof_req_json,
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_json).unwrap();

        let proof: FullProof = serde_json::from_str(&proof_json).unwrap();

        let &(_, ref value, _) = proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap();
        assert_eq!(value, "Alex");

        proof.requested_proof.unrevealed_attrs.get("attr2_referent").unwrap();

        let value = proof.requested_proof.self_attested_attrs.get("attr3_referent").unwrap();
        assert_eq!(value, self_attested_value);

        // 13. Verifier verifies proof
        let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                          &proof_json,
                                                          &schemas_json,
                                                          &claim_defs_json,
                                                          &revoc_regs_json).unwrap();
        assert!(valid);

        WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
        WalletUtils::close_wallet(prover_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[test]
    fn anoncreds_works_for_multiple_issuer_single_prover() {
        TestUtils::cleanup_storage();

        //1. Issuer1 creates wallet, gets wallet handles
        let issuer_gvt_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //2. Issuer2 creates wallet, gets wallet handles
        let issuer_xyz_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //3. Prover creates wallet, gets wallet handles
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        let mut schemas: HashMap<SchemaKey, String> = HashMap::new();
        let mut claim_defs: HashMap<SchemaKey, String> = HashMap::new();

        //4. Issuer1 creates claim definition by gvt schema
        let gvt_schema = AnoncredsUtils::gvt_schema_json();
        let gvt_claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_gvt_wallet_handle, ISSUER_DID, &gvt_schema, None, false).unwrap();

        schemas.insert(AnoncredsUtils::gvt_schema_key(), gvt_schema.clone());
        claim_defs.insert(AnoncredsUtils::gvt_schema_key(), gvt_claim_def_json.clone());

        //5. Issuer2 creates claim definition by xyz schema
        let xyz_schema = AnoncredsUtils::xyz_schema_json();
        let xyz_claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_xyz_wallet_handle, DID_MY2, &xyz_schema, None, false).unwrap();

        schemas.insert(AnoncredsUtils::xyz_schema_key(), xyz_schema.clone());
        claim_defs.insert(AnoncredsUtils::xyz_schema_key(), xyz_claim_def_json.clone());

        //6. Prover creates Master Secret for Issuer1
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //7. Issuer1 creates Claim Offer
        let gvt_claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_gvt_wallet_handle,
                                                                             &AnoncredsUtils::gvt_schema_json(),
                                                                             ISSUER_DID, DID_MY1).unwrap();

        //8. Prover stores Claim Offer received from Issuer1
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &gvt_claim_offer_json).unwrap();

        //9. Issuer2 creates Claim Offer
        let xyz_claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_xyz_wallet_handle,
                                                                             &AnoncredsUtils::xyz_schema_json(),
                                                                             DID_MY2, DID_MY1).unwrap();

        //10. Prover stores Claim Offer received from Issuer2
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &xyz_claim_offer_json).unwrap();

        //11. Prover creates Claim Request for gvt claim offer
        let gvt_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                              DID_MY1,
                                                                              &gvt_claim_offer_json,
                                                                              &gvt_claim_def_json,
                                                                              COMMON_MASTER_SECRET).unwrap();

        //12. Issuer creates GVT Claim
        let (_, gvt_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_gvt_wallet_handle,
                                                                      &gvt_claim_req,
                                                                      &AnoncredsUtils::gvt_claim_values_json(),
                                                                      None).unwrap();

        //13. Prover stores received GVT Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &gvt_claim_json, None).unwrap();

        //14. Prover creates Claim Request for xyz claim offer
        let xyz_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                              DID_MY1,
                                                                              &xyz_claim_offer_json,
                                                                              &xyz_claim_def_json,
                                                                              COMMON_MASTER_SECRET).unwrap();

        //15. Issuer creates XYZ Claim
        let (_, xyz_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_xyz_wallet_handle,
                                                                      &xyz_claim_req,
                                                                      &AnoncredsUtils::xyz_claim_values_json(),
                                                                      None).unwrap();

        //16. Prover stores received XYZ Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xyz_claim_json, None).unwrap();

        //17. Prover gets Claims for Proof Request
        let proof_req_json = format!(r#"{{
                                       "nonce":"123432421212",
                                       "name":"proof_req_1",
                                       "version":"0.1",
                                       "requested_attrs":{{
                                            "attr1_referent":{{
                                                "name":"name", "restrictions":[{{"schema_key":{}}}]
                                            }},
                                            "attr2_referent":{{
                                                "name":"status", "restrictions":[{{"schema_key":{}}}]
                                            }}
                                       }},
                                       "requested_predicates":{{
                                            "predicate1_referent":{{"attr_name":"age","p_type":">=","value":18}},
                                            "predicate2_referent":{{"attr_name":"period","p_type":">=","value":5}}
                                       }}
                                    }}"#, AnoncredsUtils::gvt_schema_key_json(), AnoncredsUtils::xyz_schema_key_json());

        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();

        let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
        let claim_for_attr_1 = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");
        let claim_for_attr_2 = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr2_referent");
        let claim_for_predicate_1 = AnoncredsUtils::get_claim_for_predicate_referent(&claims_json, "predicate1_referent");
        let claim_for_predicate_2 = AnoncredsUtils::get_claim_for_predicate_referent(&claims_json, "predicate2_referent");

        //18. Prover creates Proof
        let requested_claims_json = format!(r#"{{
                                              "self_attested_attributes":{{}},
                                              "requested_attrs":{{
                                                    "attr1_referent":["{}",true],
                                                    "attr2_referent":["{}",true]
                                              }},
                                              "requested_predicates":{{
                                                    "predicate1_referent":"{}",
                                                    "predicate2_referent":"{}"
                                              }}
                                            }}"#,
                                            claim_for_attr_1.referent, claim_for_attr_2.referent,
                                            claim_for_predicate_1.referent, claim_for_predicate_2.referent);

        let unique_claims = AnoncredsUtils::get_unique_claims(&claims);

        let schemas_json = format!(r#"{{"{}":{},"{}":{}}}"#,
                                   unique_claims[0].referent,
                                   schemas.get(&unique_claims[0].schema_key).unwrap(),
                                   unique_claims[1].referent,
                                   schemas.get(&unique_claims[1].schema_key).unwrap());

        let claim_defs_json = format!(r#"{{"{}":{},"{}":{}}}"#,
                                      unique_claims[0].referent,
                                      claim_defs.get(&unique_claims[0].schema_key).unwrap(),
                                      unique_claims[1].referent,
                                      claim_defs.get(&unique_claims[1].schema_key).unwrap());
        let revoc_regs_json = "{}";

        let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                             &proof_req_json,
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_json).unwrap();

        let proof: FullProof = serde_json::from_str(&proof_json).unwrap();

        let &(_, ref value, _) = proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap();
        assert_eq!(value, "Alex");

        let &(_, ref value, _) = proof.requested_proof.revealed_attrs.get("attr2_referent").unwrap();
        assert_eq!(value, "partial");

        //19. Verifier verifies proof
        let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                          &proof_json,
                                                          &schemas_json,
                                                          &claim_defs_json,
                                                          &revoc_regs_json).unwrap();
        assert!(valid);

        WalletUtils::close_wallet(prover_wallet_handle).unwrap();
        WalletUtils::close_wallet(issuer_gvt_wallet_handle).unwrap();
        WalletUtils::close_wallet(issuer_xyz_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[test]
    fn anoncreds_works_for_single_issuer_multiple_claims_single_prover() {
        TestUtils::cleanup_storage();

        //1. Issuer creates wallet, gets wallet handles
        let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //2. Prover creates wallet, gets wallet handles
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        let mut schemas: HashMap<SchemaKey, String> = HashMap::new();
        let mut claim_defs: HashMap<SchemaKey, String> = HashMap::new();

        //3. Issuer creates claim definition by gvt schema
        let gvt_schema = AnoncredsUtils::gvt_schema_json();
        let gvt_claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, ISSUER_DID, &gvt_schema, None, false).unwrap();

        schemas.insert(AnoncredsUtils::gvt_schema_key(), gvt_schema.clone());
        claim_defs.insert(AnoncredsUtils::gvt_schema_key(), gvt_claim_def_json.clone());

        //4. Issuer creates claim definition by xyz schema
        let xyz_schema = AnoncredsUtils::xyz_schema_json();
        let xyz_claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, ISSUER_DID, &xyz_schema, None, false).unwrap();

        schemas.insert(AnoncredsUtils::xyz_schema_key(), xyz_schema.clone());
        claim_defs.insert(AnoncredsUtils::xyz_schema_key(), xyz_claim_def_json.clone());

        //5. Prover creates Master Secret for Issuer1
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //6. Issuer1 creates Claim Offer
        let gvt_claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_wallet_handle,
                                                                             &AnoncredsUtils::gvt_schema_json(),
                                                                             ISSUER_DID, DID_MY1).unwrap();

        //7. Prover stores GVT Claim Offer received from Issuer
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &gvt_claim_offer_json).unwrap();

        //8. Issuer1 creates Claim Offer
        let xyz_claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_wallet_handle,
                                                                             &AnoncredsUtils::xyz_schema_json(),
                                                                             ISSUER_DID, DID_MY1).unwrap();

        //9. Prover stores XYZ Claim Offer received from Issuer
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &xyz_claim_offer_json).unwrap();

        //10. Prover creates Claim Request for gvt claim offer
        let gvt_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                              DID_MY1,
                                                                              &gvt_claim_offer_json,
                                                                              &gvt_claim_def_json,
                                                                              COMMON_MASTER_SECRET).unwrap();


        //11. Issuer creates GVT Claim
        let (_, gvt_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                      &gvt_claim_req,
                                                                      &AnoncredsUtils::gvt_claim_values_json(),
                                                                      None).unwrap();

        //12. Prover stores received GVT Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &gvt_claim_json, None).unwrap();

        //13. Prover creates Claim Request for xyz claim offer
        let xyz_claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                              DID_MY1,
                                                                              &xyz_claim_offer_json,
                                                                              &xyz_claim_def_json,
                                                                              COMMON_MASTER_SECRET).unwrap();

        //14. Issuer creates XYZ Claim
        let (_, xyz_claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                      &xyz_claim_req,
                                                                      &AnoncredsUtils::xyz_claim_values_json(),
                                                                      None).unwrap();

        //15. Prover stores received XYZ Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xyz_claim_json, None).unwrap();

        //16. Prover gets Claims for Proof Request
        let proof_req_json = r#"{
                                       "nonce":"123432421212",
                                       "name":"proof_req_1",
                                       "version":"0.1",
                                       "requested_attrs":{
                                            "attr1_referent":{"name":"name"}
                                       },
                                       "requested_predicates":{
                                            "predicate1_referent":{"attr_name":"age","p_type":">=","value":18},
                                            "predicate2_referent":{"attr_name":"period","p_type":">=","value":5}
                                       }
                                    }"#;

        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let claims: ClaimsForProofRequest = serde_json::from_str(&claims_json).unwrap();
        let claim_for_attr_1 = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");
        let claim_for_predicate_1 = AnoncredsUtils::get_claim_for_predicate_referent(&claims_json, "predicate1_referent");
        let claim_for_predicate_2 = AnoncredsUtils::get_claim_for_predicate_referent(&claims_json, "predicate2_referent");

        //17. Prover creates Proof
        let requested_claims_json = format!(r#"{{
                                              "self_attested_attributes":{{}},
                                              "requested_attrs":{{"attr1_referent":["{}",true]}},
                                              "requested_predicates":{{
                                                    "predicate1_referent":"{}",
                                                    "predicate2_referent":"{}"
                                              }}
                                            }}"#, claim_for_attr_1.referent, claim_for_predicate_1.referent, claim_for_predicate_2.referent);

        let unique_claims = AnoncredsUtils::get_unique_claims(&claims);

        let schemas_json = format!(r#"{{"{}":{},"{}":{}}}"#,
                                   unique_claims[0].referent,
                                   schemas.get(&unique_claims[0].schema_key).unwrap(),
                                   unique_claims[1].referent,
                                   schemas.get(&unique_claims[1].schema_key).unwrap());

        let claim_defs_json = format!(r#"{{"{}":{},"{}":{}}}"#,
                                      unique_claims[0].referent,
                                      claim_defs.get(&unique_claims[0].schema_key).unwrap(),
                                      unique_claims[1].referent,
                                      claim_defs.get(&unique_claims[1].schema_key).unwrap());
        let revoc_regs_json = "{}";

        let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                             &proof_req_json,
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_json).unwrap();

        let proof: FullProof = serde_json::from_str(&proof_json).unwrap();

        let &(_, ref value, _) = proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap();
        assert_eq!(value, "Alex");

        //18. Verifier verifies proof
        let valid = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                          &proof_json,
                                                          &schemas_json,
                                                          &claim_defs_json,
                                                          &revoc_regs_json).unwrap();
        assert!(valid);

        WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
        WalletUtils::close_wallet(prover_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_revocation_registry() {
        TestUtils::cleanup_storage();

        //1. Issuer creates wallet, gets wallet handle
        let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //2. Prover creates wallet, gets wallet handle
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //2. Issuer creates claim definition
        let schema_json = AnoncredsUtils::gvt_schema_json();
        let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, ISSUER_DID, &schema_json,
                                                                            None, true).unwrap();
        //3. Issuer creates revocation registry
        AnoncredsUtils::indy_issuer_create_and_store_revoc_reg(issuer_wallet_handle, &ISSUER_DID, &schema_json, 5).unwrap();

        //4. Prover creates Master Secret
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Issuer creates Claim Offer
        let claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_wallet_handle,
                                                                         &AnoncredsUtils::gvt_schema_json(),
                                                                         ISSUER_DID, DID_MY1).unwrap();

        //6. Prover stores Claim Offer received from Issuer
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json).unwrap();

        //7. Prover creates Claim Request
        let claim_req_json = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                               DID_MY1,
                                                                               &claim_offer_json,
                                                                               &claim_def_json,
                                                                               COMMON_MASTER_SECRET).unwrap();

        //8. Issuer creates Claim
        let (revoc_reg_update_json, claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                                      &claim_req_json,
                                                                                      &AnoncredsUtils::gvt_claim_values_json(),
                                                                                      Some(SEQ_NO)).unwrap();

        //9. Prover store received Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &claim_json, Some(&revoc_reg_update_json)).unwrap();

        //10. Prover gets Claims for Proof Request
        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle,
                                                                          &AnoncredsUtils::proof_request_attr()).unwrap();
        let claim = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

        //11. Prover creates Proof
        let requested_claims_json = format!(r#"{{
                                              "self_attested_attributes":{{}},
                                              "requested_attrs":{{"attr1_referent":["{}", true]}},
                                              "requested_predicates":{{}}
                                            }}"#, claim.referent);

        let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema_json);
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
        let revoc_regs_json = format!("{{\"{}\":{}}}", claim.referent, revoc_reg_update_json);

        let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                             &AnoncredsUtils::proof_request_attr(),
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_json).unwrap();

        //12. Verifier verifies proof
        let valid = AnoncredsUtils::verifier_verify_proof(&AnoncredsUtils::proof_request_attr(),
                                                          &proof_json,
                                                          &schemas_json,
                                                          &claim_defs_json,
                                                          &revoc_regs_json).unwrap();
        assert!(valid);

        WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
        WalletUtils::close_wallet(prover_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_claim_revoked_before_proof_created() {
        TestUtils::cleanup_storage();

        //1. Issuer creates wallet, gets wallet handle
        let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //2. Prover creates wallet, gets wallet handle
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //3. Issuer creates claim definition
        let schema = AnoncredsUtils::gvt_schema_json();
        let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, &ISSUER_DID, &schema,
                                                                            None, true).unwrap();
        //4. Issuer creates revocation registry
        AnoncredsUtils::indy_issuer_create_and_store_revoc_reg(issuer_wallet_handle, ISSUER_DID, &schema, 5).unwrap();

        //5. Prover creates Master Secret
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //6. Issuer creates Claim Offer
        let claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_wallet_handle,
                                                                         &AnoncredsUtils::gvt_schema_json(),
                                                                         ISSUER_DID, DID_MY1).unwrap();

        //7. Prover stores Claim Offer received from Issuer
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json).unwrap();

        //8. Prover creates Claim Request
        let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                          DID_MY1,
                                                                          &claim_offer_json,
                                                                          &claim_def_json,
                                                                          COMMON_MASTER_SECRET).unwrap();

        //9. Issuer creates Claim
        let (revoc_reg_update_json, claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                                      &claim_req,
                                                                                      &AnoncredsUtils::gvt_claim_values_json(),
                                                                                      Some(SEQ_NO)).unwrap();

        //10. Prover stores received Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &claim_json, Some(&revoc_reg_update_json)).unwrap();

        //11. Issuer revokes claim
        let revoc_reg_update_json = AnoncredsUtils::issuer_revoke_claim(issuer_wallet_handle, &ISSUER_DID, &schema, SEQ_NO as u32).unwrap();

        //12. Prover gets Claims for Proof Request
        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle,
                                                                          &AnoncredsUtils::proof_request_attr()).unwrap();
        let claim = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

        //13. Prover creates Proof
        let requested_claims_json = format!(r#"{{
                                              "self_attested_attributes":{{}},
                                              "requested_attrs":{{"attr1_referent":["{}", true]}},
                                              "requested_predicates":{{}}
                                            }}"#, claim.referent);

        let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema);
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
        let revoc_regs_json = format!("{{\"{}\":{}}}", claim.referent, revoc_reg_update_json);

        let res = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                      &AnoncredsUtils::proof_request_attr(),
                                                      &requested_claims_json,
                                                      &schemas_json,
                                                      COMMON_MASTER_SECRET,
                                                      &claim_defs_json,
                                                      &revoc_regs_json);
        assert_eq!(res.unwrap_err(), ErrorCode::AnoncredsClaimRevoked);

        WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
        WalletUtils::close_wallet(prover_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_claim_revoked_after_proof_created() {
        TestUtils::cleanup_storage();

        //1. Issuer creates wallet, get wallet handle
        let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //2. Prover creates wallet, gets wallet handle
        let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

        //3. Issuer creates claim definition
        let schema_json = AnoncredsUtils::gvt_schema_json();
        let claim_def_json = AnoncredsUtils::issuer_create_claim_definition(issuer_wallet_handle, &ISSUER_DID, &schema_json,
                                                                            None, true).unwrap();
        //4. Issuer creates revocation registry
        AnoncredsUtils::indy_issuer_create_and_store_revoc_reg(issuer_wallet_handle, &ISSUER_DID, &schema_json, 5).unwrap();

        //5. Prover creates Master Secret
        AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //6. Issuer creates Claim Offer
        let claim_offer_json = AnoncredsUtils::issuer_create_claim_offer(issuer_wallet_handle,
                                                                         &AnoncredsUtils::gvt_schema_json(),
                                                                         ISSUER_DID, DID_MY1).unwrap();

        //7. Prover stores Claim Offer received from Issuer
        AnoncredsUtils::prover_store_claim_offer(prover_wallet_handle, &claim_offer_json).unwrap();

        //8. Prover creates Claim Request
        let claim_req = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                          DID_MY1,
                                                                          &claim_offer_json,
                                                                          &claim_def_json,
                                                                          COMMON_MASTER_SECRET).unwrap();

        //9. Issuer creates Claim
        let (revoc_reg_update_json, claim_json) = AnoncredsUtils::issuer_create_claim(issuer_wallet_handle,
                                                                                      &claim_req,
                                                                                      &AnoncredsUtils::gvt_claim_values_json(),
                                                                                      Some(SEQ_NO)).unwrap();

        //10. Prover stores received Claim
        AnoncredsUtils::prover_store_claim(prover_wallet_handle, &claim_json, Some(&revoc_reg_update_json)).unwrap();

        //11. Prover gets Claims for Proof Request
        let claims_json = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle,
                                                                          &AnoncredsUtils::proof_request_attr()).unwrap();
        let claim = AnoncredsUtils::get_claim_for_attr_referent(&claims_json, "attr1_referent");

        //12. Prover create Proof
        let requested_claims_json = format!(r#"{{
                                              "self_attested_attributes":{{}},
                                              "requested_attrs":{{"attr1_referent":["{}", true]}},
                                              "requested_predicates":{{}}
                                            }}"#, claim.referent);

        let schemas_json = format!(r#"{{"{}":{}}}"#, claim.referent, schema_json);
        let claim_defs_json = format!(r#"{{"{}":{}}}"#, claim.referent, claim_def_json);
        let mut revoc_regs_json = format!("{{\"{}\":{}}}", claim.referent, revoc_reg_update_json);

        let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                             &AnoncredsUtils::proof_request_attr(),
                                                             &requested_claims_json,
                                                             &schemas_json,
                                                             COMMON_MASTER_SECRET,
                                                             &claim_defs_json,
                                                             &revoc_regs_json).unwrap();

        //13. Issuer revokes prover claim
        let revoc_reg_update_json = AnoncredsUtils::issuer_revoke_claim(issuer_wallet_handle, ISSUER_DID, &schema_json, SEQ_NO as u32).unwrap();
        revoc_regs_json = format!("{{\"{}\":{}}}", claim.referent, revoc_reg_update_json);

        //14. Verifier verifies proof
        let valid = AnoncredsUtils::verifier_verify_proof(&AnoncredsUtils::proof_request_attr(),
                                                          &proof_json,
                                                          &schemas_json,
                                                          &claim_defs_json,
                                                          &revoc_regs_json).unwrap();
        assert!(!valid);

        WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
        WalletUtils::close_wallet(prover_wallet_handle).unwrap();

        TestUtils::cleanup_storage();
    }
}
