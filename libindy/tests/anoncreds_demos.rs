#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate named_type_derive;

#[macro_use]
extern crate derivative;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate byteorder;
extern crate indyrs as indy;
extern crate indyrs as api;
extern crate ursa;
extern crate uuid;
extern crate named_type;
extern crate rmp_serde;
extern crate rust_base58;
extern crate time;
extern crate serde;

#[macro_use]
mod utils;

use utils::{wallet, anoncreds};
use utils::anoncreds::{COMMON_MASTER_SECRET, CREDENTIAL1_ID, CREDENTIAL2_ID, CREDENTIAL3_ID};

use indy::ErrorCode;
use utils::inmem_wallet::InmemWallet;
use utils::constants::*;

use utils::domain::anoncreds::schema::Schema;
use utils::domain::anoncreds::credential_definition::CredentialDefinition;
use utils::domain::anoncreds::credential_attr_tag_policy::CredentialAttrTagPolicy;
use utils::domain::anoncreds::revocation_registry_definition::RevocationRegistryDefinition;
use utils::domain::anoncreds::credential_for_proof_request::{CredentialsForProofRequest};
use utils::domain::anoncreds::proof::Proof;
use utils::domain::anoncreds::revocation_state::RevocationState;
use utils::domain::anoncreds::revocation_registry::RevocationRegistry;

mod demos {
    use super::*;

    #[test]
    fn anoncreds_works_for_single_issuer_single_prover() {
        utils::setup("anoncreds_works_for_single_issuer_single_prover");

        //1. Create Issuer wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_single_issuer_single_prover").unwrap();

        //2. Create Prover wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_single_issuer_single_prover").unwrap();

        //3. Issuer creates Schema and Credential Definition
        let (schema_id, schema_json, cred_def_id, cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                                                                             ISSUER_DID,
                                                                                                             GVT_SCHEMA_NAME,
                                                                                                             GVT_SCHEMA_ATTRIBUTES);

        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Issuance credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &cred_def_id,
                                                 &cred_def_json);

        //6. Proof request
        let proof_req_json = r#"{
                                       "nonce":"123432421212",
                                       "name":"proof_req_1",
                                       "version":"0.1",
                                       "requested_attributes":{
                                            "attr1_referent":{
                                                "name":"name"
                                            },
                                            "attr2_referent":{
                                                "name":"sex"
                                            },
                                            "attr3_referent":{"name":"phone"}
                                       },
                                       "requested_predicates":{
                                            "predicate1_referent":{"name":"age","p_type":">=","p_value":18}
                                       }
                                    }"#;

        //7. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        //8. Prover creates Proof
        let self_attested_value = "8-800-300";
        let requested_credentials_json = format!(r#"{{
                                                  "self_attested_attributes":{{"attr3_referent":"{}"}},
                                                  "requested_attributes":{{
                                                        "attr1_referent":{{ "cred_id":"{}", "revealed":true }},
                                                        "attr2_referent":{{ "cred_id":"{}", "revealed":false }}
                                                  }},
                                                  "requested_predicates":{{
                                                        "predicate1_referent":{{ "cred_id":"{}" }}
                                                  }}
                                                }}"#, self_attested_value, credential.referent, credential.referent, credential.referent);

        let schemas_json = json!({schema_id: serde_json::from_str::<Schema>(&schema_json).unwrap()}).to_string();
        let cred_defs_json = json!({cred_def_id: serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()}).to_string();
        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_req_json,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &cred_defs_json,
                                                        &rev_states_json).unwrap();

        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //9. Verifier verifies proof
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);
        assert_eq!(0, proof.requested_proof.unrevealed_attrs.get("attr2_referent").unwrap().sub_proof_index);
        assert_eq!(self_attested_value, proof.requested_proof.self_attested_attrs.get("attr3_referent").unwrap());

        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_req_json,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &cred_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_single_issuer_single_prover");
    }

    #[test]
    fn anoncreds_works_for_plugged_wallet() {
        utils::setup("anoncreds_works_for_plugged_wallet");
        InmemWallet::cleanup();

        //1. Registers new wallet type
        wallet::register_wallet_storage(INMEM_TYPE, false).unwrap();

        //2. Creates and opens Issuer wallet
        let issuer_wallet_config = json!({
            "id": "custom_issuer_wallet",
            "storage_type": INMEM_TYPE,
        }).to_string();

        wallet::create_wallet(&issuer_wallet_config, WALLET_CREDENTIALS).unwrap();
        let issuer_wallet_handle = wallet::open_wallet(&issuer_wallet_config, WALLET_CREDENTIALS).unwrap();

        //3. Creates and opens Prover wallet
        let prover_wallet_config = json!({
            "id": "custom_prover_wallet",
            "storage_type": INMEM_TYPE,
        }).to_string();

        wallet::create_wallet(&prover_wallet_config, WALLET_CREDENTIALS).unwrap();
        let prover_wallet_handle = wallet::open_wallet(&prover_wallet_config, WALLET_CREDENTIALS).unwrap();

        //4. Issuer creates Schema and Credential Definition
        let (schema_id, schema_json, cred_def_id, cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                                                                             ISSUER_DID,
                                                                                                             GVT_SCHEMA_NAME,
                                                                                                             GVT_SCHEMA_ATTRIBUTES);

        //5. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //6. Issuance credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &cred_def_id,
                                                 &cred_def_json);

        //7. Proof request
        let proof_req_json = r#"{
                                       "nonce":"123432421212",
                                       "name":"proof_req_1",
                                       "version":"0.1",
                                       "requested_attributes":{
                                            "attr1_referent":{
                                                "name":"name"
                                            }
                                       },
                                       "requested_predicates":{}
                                    }"#;

        //8. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        //9. Prover creates Proof
        let requested_credentials_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attributes":{{
                                                        "attr1_referent":{{ "cred_id":"{}", "revealed":true }}
                                                  }},
                                                  "requested_predicates":{{
                                                  }}
                                                }}"#, credential.referent);

        let schemas_json = json!({schema_id: serde_json::from_str::<Schema>(&schema_json).unwrap()}).to_string();
        let cred_defs_json = json!({cred_def_id: serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()}).to_string();
        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_req_json,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &cred_defs_json,
                                                        &rev_states_json).unwrap();

        //10. Verifier verifies Proof
        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_req_json,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &cred_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();
        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        InmemWallet::cleanup();
        utils::tear_down("anoncreds_works_for_plugged_wallet");
    }

    #[test]
    fn anoncreds_works_for_multiple_issuer_single_prover() {
        utils::setup("anoncreds_works_for_multiple_issuer_single_prover");

        //1. Issuer1 creates wallet, gets wallet handles
        let (issuer_gvt_wallet_handle, issuer_gvt_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_multiple_issuer_single_prover").unwrap();

        //2. Issuer2 creates wallet, gets wallet handles
        let (issuer_xyz_wallet_handle, issuer_xyz_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_multiple_issuer_single_prover").unwrap();

        //3. Prover creates wallet, gets wallet handles
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_multiple_issuer_single_prover").unwrap();

        //4. Issuer1 creates GVT Schema and Credential Definition
        let (gvt_schema_id, gvt_schema,
            gvt_cred_def_id, gvt_cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_gvt_wallet_handle,
                                                                                            ISSUER_DID,
                                                                                            GVT_SCHEMA_NAME,
                                                                                            GVT_SCHEMA_ATTRIBUTES);

        //5. Issuer2 creates XYZ Schema and Credential Definition
        let (xyz_schema_id, xyz_schema,
            xyz_cred_def_id, xyz_cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_xyz_wallet_handle,
                                                                                            DID_MY2,
                                                                                            XYZ_SCHEMA_NAME,
                                                                                            XYZ_SCHEMA_ATTRIBUTES);

        //6. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //7. Issuer1 issue GVT Credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_gvt_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &gvt_cred_def_id,
                                                 &gvt_cred_def_json);

        //8. Issuer2 issue XYZ Credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_xyz_wallet_handle,
                                                 CREDENTIAL2_ID,
                                                 &anoncreds::xyz_credential_values_json(),
                                                 &xyz_cred_def_id,
                                                 &xyz_cred_def_json);

        //9. Proof request
        let proof_req_json = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name",
                   "restrictions": json!({ "cred_def_id": gvt_cred_def_id })
               })   ,
               "attr2_referent": json!({
                   "name":"status",
                   "restrictions": json!({ "cred_def_id": xyz_cred_def_id })
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18, "restrictions": json!({ "cred_def_id": gvt_cred_def_id })}),
               "predicate2_referent": json!({ "name":"period", "p_type":">=", "p_value":5 }),
           }),
        }).to_string();

        //10. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();

        let credential_for_attr_1 = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");
        let credential_for_attr_2 = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr2_referent");
        let credential_for_predicate_1 = anoncreds::get_credential_for_predicate_referent(&credentials_json, "predicate1_referent");
        let credential_for_predicate_2 = anoncreds::get_credential_for_predicate_referent(&credentials_json, "predicate2_referent");

        //11. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential_for_attr_1.referent, "revealed":true }),
                "attr2_referent": json!({ "cred_id": credential_for_attr_2.referent, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential_for_predicate_1.referent }),
                "predicate2_referent": json!({ "cred_id": credential_for_predicate_2.referent })
             })
        }).to_string();

        let schemas_json = json!({
            gvt_schema_id: serde_json::from_str::<Schema>(&gvt_schema).unwrap(),
            xyz_schema_id: serde_json::from_str::<Schema>(&xyz_schema).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            gvt_cred_def_id: serde_json::from_str::<CredentialDefinition>(&gvt_cred_def_json).unwrap(),
            xyz_cred_def_id: serde_json::from_str::<CredentialDefinition>(&xyz_cred_def_json).unwrap()
        }).to_string();
        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_req_json,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();
        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //12. Verifier verifies proof
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);
        assert_eq!("partial", proof.requested_proof.revealed_attrs.get("attr2_referent").unwrap().raw);

        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_req_json,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();
        wallet::close_and_delete_wallet(issuer_gvt_wallet_handle, &issuer_gvt_wallet_config).unwrap();
        wallet::close_and_delete_wallet(issuer_xyz_wallet_handle, &issuer_xyz_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_multiple_issuer_single_prover");
    }

    #[test]
    fn anoncreds_works_for_single_issuer_multiple_credentials_single_prover() {
        utils::setup("anoncreds_works_for_single_issuer_multiple_credentials_single_prover");

        //1. Issuer creates wallet, gets wallet handles
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_single_issuer_multiple_credentials_single_prover").unwrap();

        //2. Prover creates wallet, gets wallet handles
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_single_issuer_multiple_credentials_single_prover").unwrap();

        //3. Issuer creates GVT Schema and Credential Definition
        let (gvt_schema_id, gvt_schema,
            gvt_cred_def_id, gvt_cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                                                            ISSUER_DID,
                                                                                            GVT_SCHEMA_NAME,
                                                                                            GVT_SCHEMA_ATTRIBUTES);

        //4. Issuer creates XYZ Schema and Credential Definition
        let (xyz_schema_id, xyz_schema,
            xyz_cred_def_id, xyz_cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                                                            ISSUER_DID,
                                                                                            XYZ_SCHEMA_NAME,
                                                                                            XYZ_SCHEMA_ATTRIBUTES);

        //5. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //6. Issuer issue GVT Credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &gvt_cred_def_id,
                                                 &gvt_cred_def_json);

        //7. Issuer issue XYZ Credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL2_ID,
                                                 &anoncreds::xyz_credential_values_json(),
                                                 &xyz_cred_def_id,
                                                 &xyz_cred_def_json);

        //8. Proof Request
        let proof_req_json = json!({
            "nonce":"123432421212",
            "name":"proof_req_1",
            "version":"0.1",
            "requested_attributes": json!({
                "attr1_referent": json!({
                    "name":"name",
                    "restrictions": json!({ "cred_def_id": gvt_cred_def_id })
                })   ,
                "attr2_referent": json!({
                    "name":"status",
                    "restrictions": json!({ "cred_def_id": xyz_cred_def_id })
                })
            }),
            "requested_predicates": json!({
                "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 }),
                "predicate2_referent": json!({ "name":"period", "p_type":">=", "p_value":5 }),
            }),
        }).to_string();

        //9. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();

        let credential_for_attr_1 = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");
        let credential_for_attr_2 = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr2_referent");
        let credential_for_predicate_1 = anoncreds::get_credential_for_predicate_referent(&credentials_json, "predicate1_referent");
        let credential_for_predicate_2 = anoncreds::get_credential_for_predicate_referent(&credentials_json, "predicate2_referent");

        //10. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential_for_attr_1.referent, "revealed":true }),
                "attr2_referent": json!({ "cred_id": credential_for_attr_2.referent, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential_for_predicate_1.referent }),
                "predicate2_referent": json!({ "cred_id": credential_for_predicate_2.referent })
             })
        }).to_string();

        let schemas_json = json!({
            gvt_schema_id: serde_json::from_str::<Schema>(&gvt_schema).unwrap(),
            xyz_schema_id: serde_json::from_str::<Schema>(&xyz_schema).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            gvt_cred_def_id: serde_json::from_str::<CredentialDefinition>(&gvt_cred_def_json).unwrap(),
            xyz_cred_def_id: serde_json::from_str::<CredentialDefinition>(&xyz_cred_def_json).unwrap()
        }).to_string();
        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_req_json,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();
        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //11. Verifier verifies proof
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);
        assert_eq!("partial", proof.requested_proof.revealed_attrs.get("attr2_referent").unwrap().raw);

        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_req_json,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();
        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_single_issuer_multiple_credentials_single_prover");
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_revocation_proof_issuance_by_demand() {
        utils::setup("anoncreds_works_for_revocation_proof_issuance_by_demand");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_issuance_by_demand").unwrap();

        //2. Prover creates wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_issuance_by_demand").unwrap();

        //3 Issuer creates Schema, Credential Definition and Revocation Registry
        let (schema_id, schema_json,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, _,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);

        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();


        //5. Issuance Credential
        let (cred_rev_id, revoc_reg_delta_json) = anoncreds::multi_steps_create_revocation_credential(
            COMMON_MASTER_SECRET,
            prover_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL1_ID,
            &anoncreds::gvt_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );
        let revoc_reg_delta_json = revoc_reg_delta_json.unwrap();

        //6. Prover gets Credentials for Proof Request
        let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
           "non_revoked": json!({ "from":80, "to":100 })
        }).to_string();

        //7. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        //8. Prover creates RevocationState
        let timestamp = 100;
        let rev_state_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                &revoc_reg_def_json,
                                                                &revoc_reg_delta_json,
                                                                timestamp,
                                                                &cred_rev_id).unwrap();

        //9. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id.clone(): serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_request,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();
        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //10. Verifier verifies proof
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_revocation_proof_issuance_by_demand");
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_revocation_proof_issuance_by_default() {
        utils::setup("anoncreds_works_for_revocation_proof_issuance_by_default");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_issuance_by_default").unwrap();

        //2. Prover creates wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_issuance_by_default").unwrap();

        //3 Issuer creates Schema, Credential Definition and Revocation Registry
        let (schema_id, schema_json,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, revoc_reg_entry_json,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_BY_DEFAULT"}"#);

        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        let (cred_rev_id, _) = anoncreds::multi_steps_create_revocation_credential(
            COMMON_MASTER_SECRET,
            prover_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL1_ID,
            &anoncreds::gvt_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );

        //5. Proof Request
        let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
           "non_revoked": json!({ "from":80, "to":100 })
        }).to_string();

        //6. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        //7. Prover creates Revocation State
        let timestamp = 100;

        let rev_state_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                &revoc_reg_def_json,
                                                                &revoc_reg_entry_json,
                                                                timestamp,
                                                                &cred_rev_id).unwrap();

        //8. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id.clone(): serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_request,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();
        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //9. Verifier verifies proof
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_entry_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_revocation_proof_issuance_by_default");
    }

    #[test]
    fn verifier_verify_proof_works_for_proof_does_not_correspond_proof_request_attr_and_predicate() {
        utils::setup("verifier_verify_proof_works_for_proof_does_not_correspond_proof_request_attr_and_predicate");

        // 1. Creates wallet, gets wallet handle
        let (wallet_handle, wallet_config) = wallet::create_and_open_default_wallet("verifier_verify_proof_works_for_proof_does_not_correspond_proof_request_attr_and_predicate").unwrap();

        // 2. Issuer creates Schema and Credential Definition
        let (schema_id, schema_json, cred_def_id, cred_def_json) = anoncreds::multi_steps_issuer_preparation(wallet_handle,
                                                                                                             ISSUER_DID,
                                                                                                             GVT_SCHEMA_NAME,
                                                                                                             GVT_SCHEMA_ATTRIBUTES);

        // 3. Prover creates Master Secret
        anoncreds::prover_create_master_secret(wallet_handle, COMMON_MASTER_SECRET).unwrap();

        // 4. Issuer issue Credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 wallet_handle,
                                                 wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &cred_def_id,
                                                 &cred_def_json);

        // 5. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(wallet_handle, &anoncreds::proof_request_attr()).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        // 6. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "revealed":true })
             }),
             "requested_predicates": json!({ })
        }).to_string();

        let schemas_json = json!({
            schema_id: serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id: serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(wallet_handle,
                                                        &anoncreds::proof_request_attr(),
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();

        // 7. Verifier verifies proof
        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let res = anoncreds::verifier_verify_proof(&anoncreds::proof_request_attr_and_predicate(),
                                                   &proof_json,
                                                   &schemas_json,
                                                   &credential_defs_json,
                                                   &rev_reg_defs_json,
                                                   &rev_regs_json);
        assert_code!(ErrorCode::CommonInvalidStructure, res);

        wallet::close_and_delete_wallet(wallet_handle, &wallet_config).unwrap();

        utils::tear_down("verifier_verify_proof_works_for_proof_does_not_correspond_proof_request_attr_and_predicate");
    }

    #[test]
    fn anoncreds_works_for_requested_attribute_in_upper_case() {
        utils::setup("anoncreds_works_for_requested_attribute_in_upper_case");

        //1. Create Issuer wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_requested_attribute_in_upper_case").unwrap();

        //2. Create Prover wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_requested_attribute_in_upper_case").unwrap();

        //3. Issuer creates Schema and Credential Definition
        let (schema_id, schema_json, cred_def_id, cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                                                                             ISSUER_DID,
                                                                                                             GVT_SCHEMA_NAME,
                                                                                                             GVT_SCHEMA_ATTRIBUTES);

        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Issuer issue Credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &cred_def_id,
                                                 &cred_def_json);

        //6. Prover gets Credentials for Proof Request
        let proof_req_json = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"  NAME"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"AGE", "p_type":">=", "p_value":18 })
           })
        }).to_string();

        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        //7. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential.referent })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id: serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id: serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_req_json,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();

        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //8. Verifier verifies proof
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_req_json,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_requested_attribute_in_upper_case");
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_revocation_proof_for_issuance_and_proving_three_credential() {
        utils::setup("anoncreds_works_for_revocation_proof_for_issuance_and_proving_three_credential");

        // Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_for_issuance_and_proving_three_credential").unwrap();

        // Prover1 creates wallet, gets wallet handle
        let (prover1_wallet_handle, prover1_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_for_issuance_and_proving_three_credential").unwrap();

        // Prover2 creates wallet, gets wallet handle
        let (prover2_wallet_handle, prover2_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_for_issuance_and_proving_three_credential").unwrap();

        // Prover3 creates wallet, gets wallet handle
        let (prover3_wallet_handle, prover3_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_revocation_proof_for_issuance_and_proving_three_credential").unwrap();

        //3 Issuer creates Schema, Credential Definition and Revocation Registry
        let (schema_id, schema_json,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, _,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);

        // ISSUANCE CREDENTIAL FOR PROVER1

        // Prover1 creates Master Secret
        let prover1_master_secret_id = "prover1_master_secret";
        anoncreds::prover_create_master_secret(prover1_wallet_handle, prover1_master_secret_id).unwrap();

        let (prover1_cred_rev_id, revoc_reg_delta1_json) = anoncreds::multi_steps_create_revocation_credential(
            prover1_master_secret_id,
            prover1_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL1_ID,
            &anoncreds::gvt_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );
        let revoc_reg_delta1_json = revoc_reg_delta1_json.unwrap();

        // ISSUANCE CREDENTIAL FOR PROVER2
        // Prover2 creates Master Secret
        let prover2_master_secret_id = "prover2_master_secret";
        anoncreds::prover_create_master_secret(prover2_wallet_handle, prover2_master_secret_id).unwrap();

        let (prover2_cred_rev_id, revoc_reg_delta2_json) = anoncreds::multi_steps_create_revocation_credential(
            prover2_master_secret_id,
            prover2_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL2_ID,
            &anoncreds::gvt2_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );
        let revoc_reg_delta2_json = revoc_reg_delta2_json.unwrap();

        // Issuer merge Revocation Registry Deltas
        let revoc_reg_delta_json = anoncreds::issuer_merge_revocation_registry_deltas(&revoc_reg_delta1_json,
                                                                                      &revoc_reg_delta2_json).unwrap();

        //ISSUANCE CREDENTIAL FOR PROVER3
        // Prover3 creates Master Secret
        let prover3_master_secret_id = "prover3_master_secret";
        anoncreds::prover_create_master_secret(prover3_wallet_handle, prover3_master_secret_id).unwrap();

        let (prover3_cred_rev_id, revoc_reg_delta3_json) = anoncreds::multi_steps_create_revocation_credential(
            prover3_master_secret_id,
            prover3_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL3_ID,
            &anoncreds::gvt3_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );
        let revoc_reg_delta3_json = revoc_reg_delta3_json.unwrap();

        // Issuer merge Revocation Registry Deltas
        let revoc_reg_delta_json = anoncreds::issuer_merge_revocation_registry_deltas(&revoc_reg_delta_json, &revoc_reg_delta3_json).unwrap();

        //PROVER1 PROVING REQUEST
        let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
           "non_revoked": json!({ "from":80, "to":100 })
        }).to_string();

        // Prover1 gets Credentials for Proof Request
        let prover1_credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover1_wallet_handle, &proof_request).unwrap();
        let prover1_credential = anoncreds::get_credential_for_attr_referent(&prover1_credentials_json, "attr1_referent");

        // Prover1 creates RevocationState
        let timestamp = 80;
        let prover1_rev_state_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                        &revoc_reg_def_json,
                                                                        &revoc_reg_delta_json,
                                                                        timestamp,
                                                                        &prover1_cred_rev_id).unwrap();

        // Prover1 creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": prover1_credential.referent, "timestamp": timestamp, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": prover1_credential.referent, "timestamp": timestamp })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id.clone(): serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&prover1_rev_state_json).unwrap()
            })
        }).to_string();

        let proof1_json = anoncreds::prover_create_proof(prover1_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover1_master_secret_id,
                                                         &schemas_json,
                                                         &credential_defs_json,
                                                         &rev_states_json).unwrap();

        // Verifier verifies proof from Prover1
        let proof: Proof = serde_json::from_str(&proof1_json).unwrap();
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof1_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);


        //PROVER2 PROVING REQUEST

        // Prover2 gets Credentials for Proof Request
        let prover2_credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover2_wallet_handle, &proof_request).unwrap();
        let prover2_credential = anoncreds::get_credential_for_attr_referent(&prover2_credentials_json, "attr1_referent");

        // Prover2 creates RevocationState
        let timestamp = 90;
        let prover2_rev_state_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                        &revoc_reg_def_json,
                                                                        &revoc_reg_delta_json,
                                                                        timestamp,
                                                                        &prover2_cred_rev_id).unwrap();

        // Prover2 creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": prover2_credential.referent, "timestamp": timestamp, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": prover2_credential.referent, "timestamp": timestamp })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id.clone(): serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&prover2_rev_state_json).unwrap()
            })
        }).to_string();

        let proof2_json = anoncreds::prover_create_proof(prover2_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover2_master_secret_id,
                                                         &schemas_json,
                                                         &credential_defs_json,
                                                         &rev_states_json).unwrap();

        // Verifier verifies proof from Prover2
        let proof: Proof = serde_json::from_str(&proof2_json).unwrap();
        assert_eq!("Alexander", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof2_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);


        // PROVING REQUEST

        // Prover3 gets Credentials for Proof Request
        let prover3_credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover3_wallet_handle, &proof_request).unwrap();
        let prover3_credential = anoncreds::get_credential_for_attr_referent(&prover3_credentials_json, "attr1_referent");

        // Prover3 creates RevocationState
        let timestamp = 100;
        let prover3_rev_state_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                        &revoc_reg_def_json,
                                                                        &revoc_reg_delta_json,
                                                                        timestamp,
                                                                        &prover3_cred_rev_id).unwrap();

        // Prover3 creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": prover3_credential.referent, "timestamp": timestamp, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": prover3_credential.referent, "timestamp": timestamp })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id.clone(): serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&prover3_rev_state_json).unwrap()
            })
        }).to_string();

        let proof3_json = anoncreds::prover_create_proof(prover3_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover3_master_secret_id,
                                                         &schemas_json,
                                                         &credential_defs_json,
                                                         &rev_states_json).unwrap();

        // Verifier verifies proof from Prover2
        let proof: Proof = serde_json::from_str(&proof3_json).unwrap();
        assert_eq!("Artem", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof3_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover1_wallet_handle, &prover1_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover2_wallet_handle, &prover2_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover3_wallet_handle, &prover3_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_revocation_proof_for_issuance_and_proving_three_credential");
    }

    #[test]
    fn anoncreds_works_for_twice_entry_of_attribute_from_different_credential() {
        utils::setup("anoncreds_works_for_twice_entry_of_attribute_from_different_credential");

        //1. Issuer1 creates wallet, gets wallet handles
        let (issuer_gvt_wallet_handle, issuer_gvt_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_twice_entry_of_attribute_from_different_credential").unwrap();

        //2. Issuer2 creates wallet, gets wallet handles
        let (issuer_abc_wallet_handle, issuer_abc_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_twice_entry_of_attribute_from_different_credential").unwrap();

        //3. Prover creates wallet, gets wallet handles
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_twice_entry_of_attribute_from_different_credential").unwrap();

        //4. Issuer creates Schema and Credential Definition
        let (gvt_schema_id, gvt_schema, gvt_cred_def_id, gvt_cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_gvt_wallet_handle,
                                                                                                                        ISSUER_DID,
                                                                                                                        GVT_SCHEMA_NAME,
                                                                                                                        GVT_SCHEMA_ATTRIBUTES);

        //5. Issuer creates Schema and Credential Definition
        let (abc_schema_id, abc_schema, abc_cred_def_id, abc_cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_abc_wallet_handle,
                                                                                                                        ISSUER_DID,
                                                                                                                        "abc",
                                                                                                                        r#"["name", "second_name", "experience"]"#);

        //6. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //7. Issuer1 issue GVT Credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_gvt_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &gvt_cred_def_id,
                                                 &gvt_cred_def_json);

        //8. Issuer2 issue ABC Credential for Prover
        //   note that encoding is not standardized by Indy except that 32-bit integers are encoded as themselves. IS-786
        let abc_cred_values = r#"{
            "name": {"raw":"Alexander", "encoded": "126328542632549235769221"},
            "second_name": {"raw":"Park", "encoded": "42935129364832492914638245934"},
            "experience": {"raw":"5", "encoded": "5"}
        }"#;

        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_abc_wallet_handle,
                                                 CREDENTIAL2_ID,
                                                 abc_cred_values,
                                                 &abc_cred_def_id,
                                                 &abc_cred_def_json);

        //9. Verifier asks attributes with same name but from different Credentials
        let proof_req_json = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name",
                   "restrictions": json!({ "cred_def_id": gvt_cred_def_id })
               })   ,
               "attr2_referent": json!({
                   "name":"name",
                   "restrictions": json!({ "cred_def_id": abc_cred_def_id })
               })
           }),
           "requested_predicates": json!({}),
        }).to_string();

        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();

        let credential_for_attr_1 = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");
        let credential_for_attr_2 = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr2_referent");

        //10. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential_for_attr_1.referent, "revealed":true }),
                "attr2_referent": json!({ "cred_id": credential_for_attr_2.referent, "revealed":true })
             }),
             "requested_predicates": json!({})
        }).to_string();

        let schemas_json = json!({
            gvt_schema_id: serde_json::from_str::<Schema>(&gvt_schema).unwrap(),
            abc_schema_id: serde_json::from_str::<Schema>(&abc_schema).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            gvt_cred_def_id: serde_json::from_str::<CredentialDefinition>(&gvt_cred_def_json).unwrap(),
            abc_cred_def_id: serde_json::from_str::<CredentialDefinition>(&abc_cred_def_json).unwrap()
        }).to_string();
        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_req_json,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();
        let proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //11. Verifier verifies proof
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);
        assert_eq!("Alexander", proof.requested_proof.revealed_attrs.get("attr2_referent").unwrap().raw);

        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_req_json,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();
        wallet::close_and_delete_wallet(issuer_gvt_wallet_handle, &issuer_gvt_wallet_config).unwrap();
        wallet::close_and_delete_wallet(issuer_abc_wallet_handle, &issuer_abc_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_twice_entry_of_attribute_from_different_credential");
    }

    #[test]
    fn anoncreds_works_for_twice_entry_of_credential_for_different_witness() {
        utils::setup("anoncreds_works_for_twice_entry_of_credential_for_different_witness");

        // Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_twice_entry_of_credential_for_different_witness").unwrap();

        // Prover1 creates wallet, gets wallet handle
        let (prover1_wallet_handle, prover1_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_twice_entry_of_credential_for_different_witness").unwrap();

        // Prover2 creates wallet, gets wallet handle
        let (prover2_wallet_handle, prover2_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_twice_entry_of_credential_for_different_witness").unwrap();

        // Issuer creates Schema, Credential Definition and Revocation Registry
        let (schema_id, schema_json,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, _,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);

        // ISSUANCE CREDENTIAL FOR PROVER1

        // Prover1 creates Master Secret
        let prover1_master_secret_id = "prover1_master_secret";
        anoncreds::prover_create_master_secret(prover1_wallet_handle, prover1_master_secret_id).unwrap();

        let timestamp1 = time::get_time().sec as u64;

        let (prover1_cred_rev_id, revoc_reg_delta1_json) = anoncreds::multi_steps_create_revocation_credential(
            prover1_master_secret_id,
            prover1_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL1_ID,
            &anoncreds::gvt_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );
        let revoc_reg_delta1_json = revoc_reg_delta1_json.unwrap();

        // ISSUANCE CREDENTIAL FOR PROVER2
        // Prover2 creates Master Secret
        let prover2_master_secret_id = "prover2_master_secret";
        anoncreds::prover_create_master_secret(prover2_wallet_handle, prover2_master_secret_id).unwrap();

        let timestamp2 = time::get_time().sec as u64 + 100;

        let (_, revoc_reg_delta2_json) = anoncreds::multi_steps_create_revocation_credential(
            prover2_master_secret_id,
            prover2_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL2_ID,
            &anoncreds::gvt2_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );
        let revoc_reg_delta2_json = revoc_reg_delta2_json.unwrap();

        // Issuer merge Revocation Registry Deltas
        let revoc_reg_delta_json = anoncreds::issuer_merge_revocation_registry_deltas(&revoc_reg_delta1_json, &revoc_reg_delta2_json).unwrap();

        //PROVER1 PROVING REQUEST
        let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name",
                   "non_revoked": json!({ "to":timestamp1 + 1 })
               }),
               "attr2_referent": json!({
                   "name":"name",
                   "non_revoked": json!({  "from":timestamp1, "to":timestamp2 + 1 })
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
           "non_revoked": json!({ "from":timestamp1, "to":timestamp2 })
        }).to_string();

        // Prover1 gets Credentials for Proof Request
        let prover1_credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover1_wallet_handle, &proof_request).unwrap();
        let credentials: CredentialsForProofRequest = serde_json::from_str(&prover1_credentials_json).unwrap();

        let prover1_credential = credentials.attrs.get("attr1_referent").unwrap()[0].clone();
        let prover2_credential = credentials.attrs.get("attr2_referent").unwrap()[0].clone();

        assert_ne!(prover1_credential.interval, prover2_credential.interval);

        // Prover1 creates RevocationState for Timestamp 1
        let prover1_rev_state_1_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                          &revoc_reg_def_json,
                                                                          &revoc_reg_delta1_json,
                                                                          timestamp1,
                                                                          &prover1_cred_rev_id).unwrap();

        // Prover1 creates RevocationState for Timestamp 2
        let prover1_rev_state_2_json = anoncreds::update_revocation_state(blob_storage_reader_handle,
                                                                          &prover1_rev_state_1_json,
                                                                          &revoc_reg_def_json,
                                                                          &revoc_reg_delta_json,
                                                                          timestamp2,
                                                                          &prover1_cred_rev_id).unwrap();

        // Prover1 creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": prover1_credential.cred_info.referent, "timestamp": timestamp1, "revealed":true }),
                "attr2_referent": json!({ "cred_id": prover2_credential.cred_info.referent, "timestamp": timestamp2, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": prover2_credential.cred_info.referent, "timestamp": timestamp2 })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id.clone(): serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp1.to_string(): serde_json::from_str::<RevocationState>(&prover1_rev_state_1_json).unwrap(),
                timestamp2.to_string(): serde_json::from_str::<RevocationState>(&prover1_rev_state_2_json).unwrap()
            })
        }).to_string();

        let proof1_json = anoncreds::prover_create_proof(prover1_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover1_master_secret_id,
                                                         &schemas_json,
                                                         &credential_defs_json,
                                                         &rev_states_json).unwrap();

        // Verifier verifies proof from Prover1
        let proof: Proof = serde_json::from_str(&proof1_json).unwrap();
        assert_eq!(2, proof.requested_proof.revealed_attrs.len());
        assert_eq!(2, proof.identifiers.len());

        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);
        assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr2_referent").unwrap().raw);

        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp1.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta1_json).unwrap(),
                timestamp2.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof1_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover1_wallet_handle, &prover1_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover2_wallet_handle, &prover2_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_twice_entry_of_credential_for_different_witness");
    }

    #[test]
    #[ignore] //FIXME
    fn anoncreds_works_for_misused_witness() {
        //???
        // ignore requested timestamp in proof request
        // - provide valid proof for invalid time
        // - provide hacked proof: specify requested timestamp, actually use invalid TS
        unimplemented!();
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_issuance_on_demand_revocation_strategy_revoke_credential() {
        utils::setup("anoncreds_works_for_issuance_on_demand_revocation_strategy_revoke_credential");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_on_demand_revocation_strategy_revoke_credential").unwrap();

        //2. Prover creates wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_on_demand_revocation_strategy_revoke_credential").unwrap();

        //3 Issuer creates Schema, Credential Definition and Revocation Registry
        let (schema_id, schema_json,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, _,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);

        //4. Issuance Credential for Prover
        // Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        let (cred_rev_id, revoc_reg_delta_json) = anoncreds::multi_steps_create_revocation_credential(
            COMMON_MASTER_SECRET,
            prover_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL1_ID,
            &anoncreds::gvt_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );
        let revoc_reg_delta_json = revoc_reg_delta_json.unwrap();

        //5. Prover gets Credentials for Proof Request
        let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
           "non_revoked": json!({ "from":80, "to":100 })
        }).to_string();

        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        //6. Prover creates RevocationState
        let timestamp = 100;
        let rev_state_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                &revoc_reg_def_json,
                                                                &revoc_reg_delta_json,
                                                                timestamp,
                                                                &cred_rev_id).unwrap();

        //7. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp":timestamp, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential.referent, "timestamp":timestamp })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id: serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id: serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_request,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();

        //8. Verifier verifies proof before it will be revoked
        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        //9. Issuer revokes credential
        let revoc_reg_delta_json = anoncreds::issuer_revoke_credential(issuer_wallet_handle,
                                                                       blob_storage_reader_handle,
                                                                       &rev_reg_id,
                                                                       &cred_rev_id).unwrap();

        //10. Verifier verifies proof after that was revoked
        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(!valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_issuance_on_demand_revocation_strategy_revoke_credential");
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_issuance_by_default_revocation_strategy_revoke_credential() {
        utils::setup("anoncreds_works_for_issuance_by_default_revocation_strategy_revoke_credential");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_default_revocation_strategy_revoke_credential").unwrap();

        //2. Prover creates wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_default_revocation_strategy_revoke_credential").unwrap();

        //3 Issuer creates Schema, Credential Definition and Revocation Registry
        let (schema_id, schema_json,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, rev_reg_entry_json,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               &anoncreds::issuance_by_default_rev_reg_config());
        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Issuance Credential for Prover
        let (cred_rev_id, _) = anoncreds::multi_steps_create_revocation_credential(
            COMMON_MASTER_SECRET,
            prover_wallet_handle,
            issuer_wallet_handle,
            CREDENTIAL1_ID,
            &anoncreds::gvt_credential_values_json(),
            &cred_def_id,
            &cred_def_json,
            &rev_reg_id,
            &revoc_reg_def_json,
            blob_storage_reader_handle,
        );

        //6. Prover gets Credentials for Proof Request
        let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({
               "predicate1_referent": json!({ "name":"age", "p_type":">=", "p_value":18 })
           }),
           "non_revoked": json!({ "from":80, "to":100 })
        }).to_string();

        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
        let credential = anoncreds::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

        //7. Prover creates RevocationState
        let timestamp = 100;
        let rev_state_json = anoncreds::create_revocation_state(blob_storage_reader_handle,
                                                                &revoc_reg_def_json,
                                                                &rev_reg_entry_json,
                                                                timestamp,
                                                                &cred_rev_id).unwrap();

        //8. Prover creates Proof
        let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp":timestamp, "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential.referent, "timestamp":timestamp })
             })
        }).to_string();

        let schemas_json = json!({
            schema_id: serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

        let credential_defs_json = json!({
            cred_def_id: serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

        let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_request,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &credential_defs_json,
                                                        &rev_states_json).unwrap();

        //9. Verifier verifies proof before it will be revoked
        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&rev_reg_entry_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        //10. Issuer revokes credential
        let revoc_reg_delta_json = anoncreds::issuer_revoke_credential(issuer_wallet_handle,
                                                                       blob_storage_reader_handle,
                                                                       &rev_reg_id,
                                                                       &cred_rev_id).unwrap();

        //11. Verifier verifies proof after that was revoked
        let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

        let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&revoc_reg_delta_json).unwrap()
            })
        }).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_request,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &credential_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(!valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle,  &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_issuance_by_default_revocation_strategy_revoke_credential");
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_issuance_by_demand_revocation_strategy_for_full_revocation_registry() {
        utils::setup("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_full_revocation_registry");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_full_revocation_registry").unwrap();

        //2. Prover creates wallet, gets wallet handle
        let (prover_1_wallet_handle, prover_1_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_full_revocation_registry").unwrap();

        //3. Prover creates wallet, gets wallet handle
        let (prover_2_wallet_handle, prover_2_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_full_revocation_registry").unwrap();

        //4. Prover creates wallet, gets wallet handle
        let (prover_3_wallet_handle, prover_3_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_full_revocation_registry").unwrap();

        //5 Issuer creates Schema, Credential Definition and Revocation Registry
        let (_, _,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, _,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":2, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);

        //6. Prover1 creates Master Secret
        let prover_1_master_secret = "prover1_master_secret";
        anoncreds::prover_create_master_secret(prover_1_wallet_handle, prover_1_master_secret).unwrap();


        anoncreds::multi_steps_create_revocation_credential(prover_1_master_secret,
                                                            prover_1_wallet_handle,
                                                            issuer_wallet_handle,
                                                            CREDENTIAL1_ID,
                                                            &anoncreds::gvt_credential_values_json(),
                                                            &cred_def_id,
                                                            &cred_def_json,
                                                            &rev_reg_id,
                                                            &revoc_reg_def_json,
                                                            blob_storage_reader_handle);

        //7. Issuance Credential for Prover2
        // Prover2 creates Master Secret
        let prover_2_master_secret = "prover2_master_secret";
        anoncreds::prover_create_master_secret(prover_2_wallet_handle, prover_2_master_secret).unwrap();

        anoncreds::multi_steps_create_revocation_credential(prover_2_master_secret,
                                                            prover_2_wallet_handle,
                                                            issuer_wallet_handle,
                                                            CREDENTIAL2_ID,
                                                            &anoncreds::gvt2_credential_values_json(),
                                                            &cred_def_id,
                                                            &cred_def_json,
                                                            &rev_reg_id,
                                                            &revoc_reg_def_json,
                                                            blob_storage_reader_handle);

        //8. Issuance Credential for Prover3
        let prover_3_master_secret = "prover_3_master_secret";
        anoncreds::prover_create_master_secret(prover_3_wallet_handle, prover_3_master_secret).unwrap();

        let cred_offer_json = anoncreds::issuer_create_credential_offer(issuer_wallet_handle, &cred_def_id).unwrap();

        let (cred_req_json, _) = anoncreds::prover_create_credential_req(prover_3_wallet_handle,
                                                                         DID_MY2,
                                                                         &cred_offer_json,
                                                                         &cred_def_json,
                                                                         prover_3_master_secret).unwrap();

        let res = anoncreds::issuer_create_credential(issuer_wallet_handle,
                                                      &cred_offer_json,
                                                      &cred_req_json,
                                                      &anoncreds::gvt_credential_values_json(),
                                                      Some(&rev_reg_id),
                                                      Some(blob_storage_reader_handle));
        assert_code!(ErrorCode::AnoncredsRevocationRegistryFullError, res);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_1_wallet_handle, &prover_1_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_2_wallet_handle, &prover_2_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_3_wallet_handle, &prover_3_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_full_revocation_registry");
    }


    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_issuance_by_default_revocation_strategy_for_full_revocation_registry() {
        utils::setup("anoncreds_works_for_issuance_by_default_revocation_strategy_for_full_revocation_registry");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_default_revocation_strategy_for_full_revocation_registry").unwrap();

        //2. Prover creates wallet, gets wallet handle
        let (prover_1_wallet_handle, prover_1_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_default_revocation_strategy_for_full_revocation_registry").unwrap();

        //3. Prover creates wallet, gets wallet handle
        let (prover_2_wallet_handle, prover_2_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_default_revocation_strategy_for_full_revocation_registry").unwrap();

        //4. Prover creates wallet, gets wallet handle
        let (prover_3_wallet_handle, prover_3_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_default_revocation_strategy_for_full_revocation_registry").unwrap();

        //5 Issuer creates Schema, Credential Definition and Revocation Registry
        let (_, _,
            cred_def_id, cred_def_json,
            rev_reg_id, revoc_reg_def_json, _,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":2, "issuance_type":"ISSUANCE_BY_DEFAULT"}"#);

        // Prover1 creates Master Secret
        let prover_1_master_secret = "prover1_master_secret";
        anoncreds::prover_create_master_secret(prover_1_wallet_handle, prover_1_master_secret).unwrap();

        anoncreds::multi_steps_create_revocation_credential(prover_1_master_secret,
                                                            prover_1_wallet_handle,
                                                            issuer_wallet_handle,
                                                            CREDENTIAL1_ID,
                                                            &anoncreds::gvt_credential_values_json(),
                                                            &cred_def_id,
                                                            &cred_def_json,
                                                            &rev_reg_id,
                                                            &revoc_reg_def_json,
                                                            blob_storage_reader_handle);

        //9. Issuance Credential for Prover2
        // Prover2 creates Master Secret
        let prover_2_master_secret = "prover2_master_secret";
        anoncreds::prover_create_master_secret(prover_2_wallet_handle, prover_2_master_secret).unwrap();

        anoncreds::multi_steps_create_revocation_credential(prover_2_master_secret,
                                                            prover_2_wallet_handle,
                                                            issuer_wallet_handle,
                                                            CREDENTIAL2_ID,
                                                            &anoncreds::gvt2_credential_values_json(),
                                                            &cred_def_id,
                                                            &cred_def_json,
                                                            &rev_reg_id,
                                                            &revoc_reg_def_json,
                                                            blob_storage_reader_handle);

        //10. Issuance Credential for Prover3
        let prover_3_master_secret = "prover_3_master_secret";
        anoncreds::prover_create_master_secret(prover_3_wallet_handle, prover_3_master_secret).unwrap();

        let cred_offer_json = anoncreds::issuer_create_credential_offer(issuer_wallet_handle, &cred_def_id).unwrap();

        let (cred_req_json, _) = anoncreds::prover_create_credential_req(prover_3_wallet_handle,
                                                                         DID_MY2,
                                                                         &cred_offer_json,
                                                                         &cred_def_json,
                                                                         prover_3_master_secret).unwrap();

        let res = anoncreds::issuer_create_credential(issuer_wallet_handle,
                                                      &cred_offer_json,
                                                      &cred_req_json,
                                                      &anoncreds::gvt_credential_values_json(),
                                                      Some(&rev_reg_id),
                                                      Some(blob_storage_reader_handle));
        assert_code!(ErrorCode::AnoncredsRevocationRegistryFullError, res);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_1_wallet_handle, &prover_1_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_2_wallet_handle, &prover_2_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_3_wallet_handle, &prover_3_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_issuance_by_default_revocation_strategy_for_full_revocation_registry");
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_issuance_by_demand_revocation_strategy_for_revoke_not_issued_credential_id() {
        utils::setup("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_revoke_not_issued_credential_id");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_revoke_not_issued_credential_id").unwrap();

        //2. Issuer creates schema
        let (_, schema_json) = anoncreds::issuer_create_schema(ISSUER_DID,
                                                               GVT_SCHEMA_NAME,
                                                               SCHEMA_VERSION,
                                                               GVT_SCHEMA_ATTRIBUTES).unwrap();

        //3. Issuer creates credential definition
        let (cred_def_id, _) = anoncreds::issuer_create_credential_definition(issuer_wallet_handle,
                                                                              ISSUER_DID,
                                                                              &schema_json,
                                                                              TAG_1,
                                                                              None,
                                                                              Some(&anoncreds::revocation_cred_def_config())).unwrap();

        //4. Issuer creates revocation registry for 2 Credentials
        let tails_writer_config = anoncreds::tails_writer_config();
        let tails_writer_handle = utils::blob_storage::open_writer("default", &tails_writer_config).unwrap();

        let (rev_reg_id, _, _) =
            anoncreds::issuer_create_and_store_revoc_reg(issuer_wallet_handle,
                                                         &ISSUER_DID,
                                                         None,
                                                         TAG_1,
                                                         &cred_def_id,
                                                         r#"{"max_cred_num":2, "issuance_type":"ISSUANCE_ON_DEMAND"}"#,
                                                         tails_writer_handle).unwrap();

        let blob_storage_reader_handle = utils::blob_storage::open_reader(TYPE, &tails_writer_config).unwrap();

        //5. Issuer revokes Credential by not issued id
        let cred_rev_id = "100";
        let res = anoncreds::issuer_revoke_credential(issuer_wallet_handle, blob_storage_reader_handle, &rev_reg_id, cred_rev_id);
        assert_code!(ErrorCode::AnoncredsInvalidUserRevocId, res);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_issuance_by_demand_revocation_strategy_for_revoke_not_issued_credential_id");
    }

    #[cfg(feature = "revocation_tests")]
    #[test]
    fn anoncreds_works_for_issuance_by_default_revocation_strategy_for_revoke_not_issued_credential_id() {
        utils::setup("anoncreds_works_for_issuance_by_default_revocation_strategy_for_revoke_not_issued_credential_id");

        //1. Issuer creates wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_issuance_by_default_revocation_strategy_for_revoke_not_issued_credential_id").unwrap();

        //2 Issuer creates Schema, Credential Definition and Revocation Registry
        let (_, _,
            _, _,
            rev_reg_id, _, _,
            blob_storage_reader_handle) = anoncreds::multi_steps_issuer_revocation_preparation(issuer_wallet_handle,
                                                                                               ISSUER_DID,
                                                                                               GVT_SCHEMA_NAME,
                                                                                               GVT_SCHEMA_ATTRIBUTES,
                                                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);

        //3. Issuer revokes Credential by not issued id
        let cred_rev_id = 10.to_string();
        let res = anoncreds::issuer_revoke_credential(issuer_wallet_handle, blob_storage_reader_handle, &rev_reg_id, &cred_rev_id);
        assert_code!(ErrorCode::AnoncredsInvalidUserRevocId, res);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_issuance_by_default_revocation_strategy_for_revoke_not_issued_credential_id");
    }


    #[test]
    fn anoncreds_works_for_multiple_requested_predicates_from_one_credential() {
        utils::setup("anoncreds_works_for_multiple_requested_predicates_from_one_credential");

        //1. Create Issuer wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_multiple_requested_predicates_from_one_credential").unwrap();

        //2. Create Prover wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_multiple_requested_predicates_from_one_credential").unwrap();

        //3. Issuer creates Schema and Credential Definition
        let attr_names = r#"["task1",
                                  "task2",
                                  "task3",
                                  "task4",
                                  "task5",
                                  "6*_task",
                                  "7*_task",
                                  "bonus",
                                  "average",
                                  "aggregated",
                                  "total"]"#;
        let (schema_id, schema_json, cred_def_id, cred_def_json) =
            anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                      ISSUER_DID,
                                                      "test",
                                                      attr_names);

        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Issuance credential for Prover
        let cred_values = r#"{
            "task1": {"raw":"8", "encoded": "8"},
            "task2": {"raw":"8", "encoded": "8"},
            "task3": {"raw":"10", "encoded": "10"},
            "task4": {"raw":"9", "encoded": "9"},
            "task5": {"raw":"7", "encoded": "7"},
            "6*_task": {"raw":"8", "encoded": "8"},
            "7*_task": {"raw":"9", "encoded": "9"},
            "bonus": {"raw":"5", "encoded": "5"},
            "average": {"raw":"9", "encoded": "9"},
            "aggregated": {"raw":"9", "encoded": "9"},
            "total": {"raw":"77", "encoded": "77"}
        }"#;

        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 cred_values,
                                                 &cred_def_id,
                                                 &cred_def_json);

        //6. Proof request
        let proof_req_json = r#"{
                                       "nonce":"123432421212",
                                       "name":"proof_req_1",
                                       "version":"0.1",
                                       "requested_attributes":{},
                                       "requested_predicates":{
                                            "predicate1_referent":{"name":"task1","p_type":">=","p_value":5},
                                            "predicate2_referent":{"name":"task2","p_type":">=","p_value":7},
                                            "predicate3_referent":{"name":"task3","p_type":">=","p_value":7},
                                            "predicate4_referent":{"name":"task4","p_type":">=","p_value":8},
                                            "predicate5_referent":{"name":"task5","p_type":">=","p_value":5},
                                            "predicate6_referent":{"name":"6*_task","p_type":">=","p_value":7},
                                            "predicate7_referent":{"name":"7*_task","p_type":">=","p_value":7},
                                            "predicate8_referent":{"name":"bonus","p_type":">=","p_value":3},
                                            "predicate9_referent":{"name":"average","p_type":">=","p_value":8},
                                            "predicate10_referent":{"name":"aggregated","p_type":">=","p_value":7},
                                            "predicate11_referent":{"name":"total","p_type":">=","p_value":70}
                                       }
                                    }"#;

        //7. Prover gets Credentials for Proof Request
        let credentials_json = anoncreds::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json).unwrap();
        let credential = anoncreds::get_credential_for_predicate_referent(&credentials_json, "predicate1_referent");

        //8. Prover creates Proof
        let requested_credentials_json = format!(r#"{{
                                                  "self_attested_attributes":{{}},
                                                  "requested_attributes":{{}},
                                                  "requested_predicates":{{
                                                        "predicate1_referent":{{ "cred_id":"{}" }},
                                                        "predicate2_referent":{{ "cred_id":"{}" }},
                                                        "predicate3_referent":{{ "cred_id":"{}" }},
                                                        "predicate4_referent":{{ "cred_id":"{}" }},
                                                        "predicate5_referent":{{ "cred_id":"{}" }},
                                                        "predicate6_referent":{{ "cred_id":"{}" }},
                                                        "predicate7_referent":{{ "cred_id":"{}" }},
                                                        "predicate8_referent":{{ "cred_id":"{}" }},
                                                        "predicate9_referent":{{ "cred_id":"{}" }},
                                                        "predicate10_referent":{{ "cred_id":"{}" }},
                                                        "predicate11_referent":{{ "cred_id":"{}" }}
                                                  }}
                                                }}"#, credential.referent, credential.referent, credential.referent, credential.referent, credential.referent,
                                                 credential.referent, credential.referent, credential.referent, credential.referent, credential.referent, credential.referent);

        let schemas_json = json!({schema_id: serde_json::from_str::<Schema>(&schema_json).unwrap()}).to_string();
        let cred_defs_json = json!({cred_def_id: serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()}).to_string();
        let rev_states_json = json!({}).to_string();

        let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                                        &proof_req_json,
                                                        &requested_credentials_json,
                                                        COMMON_MASTER_SECRET,
                                                        &schemas_json,
                                                        &cred_defs_json,
                                                        &rev_states_json).unwrap();

        let _proof: Proof = serde_json::from_str(&proof_json).unwrap();

        //9. Verifier verifies proof
        let rev_reg_defs_json = json!({}).to_string();
        let rev_regs_json = json!({}).to_string();

        let valid = anoncreds::verifier_verify_proof(&proof_req_json,
                                                     &proof_json,
                                                     &schemas_json,
                                                     &cred_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();
        assert!(valid);

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_multiple_requested_predicates_from_one_credential");
    }

    #[test]
    fn anoncreds_works_for_credential_attr_tag_policy() {
        utils::setup("anoncreds_works_for_credential_attr_tag_policy");

        //1. Create Issuer wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_credential_attr_tag_policy").unwrap();

        //2. Create Prover wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_credential_attr_tag_policy").unwrap();

        //3. Issuer creates Schema and Credential Definition
        let (_schema_id, _schema_json, cred_def_id, cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                                                                               ISSUER_DID,
                                                                                                               GVT_SCHEMA_NAME,
                                                                                                               GVT_SCHEMA_ATTRIBUTES);

        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Prover gets (default) credential attr tag policy
        let mut catpol_json = anoncreds::prover_get_credential_attr_tag_policy(prover_wallet_handle, &cred_def_id).unwrap();
        assert_eq!(catpol_json, "null");

        //6. Prover sets credential attr tag policy
        let taggable_attrs = r#"["name", "height", "age"]"#;
        anoncreds::prover_set_credential_attr_tag_policy(prover_wallet_handle, &cred_def_id, Some(taggable_attrs), false).unwrap();

        //7. Prover gets credential attr tag policy
        catpol_json = anoncreds::prover_get_credential_attr_tag_policy(prover_wallet_handle, &cred_def_id).unwrap();
        let catpol = serde_json::from_str::<CredentialAttrTagPolicy>(&catpol_json).unwrap();
        assert!(catpol.is_taggable("name"));
        assert!(catpol.is_taggable("height"));
        assert!(catpol.is_taggable("age"));

        //8. Issuance credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &cred_def_id,
                                                 &cred_def_json);

        //9. Prover searches on tagged attribute
        let mut filter_json = json!({
            "attr::name::marker": "1",
        }).to_string();

        let (search_handle, count) = anoncreds::prover_search_credentials(prover_wallet_handle, &filter_json).unwrap();
        assert_eq!(count, 1);
        anoncreds::prover_close_credentials_search(search_handle).unwrap();

        //10. Prover searches on untagged attribute
        filter_json = json!({
            "attr::sex::marker": "1",
        }).to_string();

        let (search_handle, count) = anoncreds::prover_search_credentials(prover_wallet_handle, &filter_json).unwrap();
        assert_eq!(count, 0);
        anoncreds::prover_close_credentials_search(search_handle).unwrap();

        //11. Prover clears credential attr tag policy retroactively (restoring default tag-all)
        anoncreds::prover_set_credential_attr_tag_policy(prover_wallet_handle, &cred_def_id, None, true).unwrap();

        //12. Prover searches on formerly untagged attribute, but which default policy now tags
        let (search_handle, count) = anoncreds::prover_search_credentials(prover_wallet_handle, &filter_json).unwrap();
        assert_eq!(count, 1);
        anoncreds::prover_close_credentials_search(search_handle).unwrap();

        //13. Prover deletes credential
        anoncreds::prover_delete_credential(prover_wallet_handle, CREDENTIAL1_ID).unwrap();

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_credential_attr_tag_policy");
    }

    #[test]
    fn anoncreds_works_for_credential_deletion() {
        utils::setup("anoncreds_works_for_credential_deletion");

        //1. Create Issuer wallet, gets wallet handle
        let (issuer_wallet_handle, issuer_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_credential_deletion").unwrap();

        //2. Create Prover wallet, gets wallet handle
        let (prover_wallet_handle, prover_wallet_config) = wallet::create_and_open_default_wallet("anoncreds_works_for_credential_deletion").unwrap();

        //3. Issuer creates Schema and Credential Definition
        let (_schema_id, _schema_json, cred_def_id, cred_def_json) = anoncreds::multi_steps_issuer_preparation(issuer_wallet_handle,
                                                                                                               ISSUER_DID,
                                                                                                               GVT_SCHEMA_NAME,
                                                                                                               GVT_SCHEMA_ATTRIBUTES);

        //4. Prover creates Master Secret
        anoncreds::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

        //5. Issuance credential for Prover
        anoncreds::multi_steps_create_credential(COMMON_MASTER_SECRET,
                                                 prover_wallet_handle,
                                                 issuer_wallet_handle,
                                                 CREDENTIAL1_ID,
                                                 &anoncreds::gvt_credential_values_json(),
                                                 &cred_def_id,
                                                 &cred_def_json);

        //6. Prover gets credential by identifier
        let mut cred = anoncreds::prover_get_credential(prover_wallet_handle, CREDENTIAL1_ID);
        assert!(cred.is_ok());

        //7. Prover deletes credential
        anoncreds::prover_delete_credential(prover_wallet_handle, CREDENTIAL1_ID).unwrap();

        //8. Prover cannot get deleted credential by identifier
        cred = anoncreds::prover_get_credential(prover_wallet_handle, CREDENTIAL1_ID);
        assert!(cred.is_err());

        wallet::close_and_delete_wallet(issuer_wallet_handle, &issuer_wallet_config).unwrap();
        wallet::close_and_delete_wallet(prover_wallet_handle, &prover_wallet_config).unwrap();

        utils::tear_down("anoncreds_works_for_credential_deletion");
    }
}
