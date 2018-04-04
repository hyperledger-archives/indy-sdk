extern crate indy;
extern crate uuid;
extern crate time;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate indy_crypto;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::anoncreds::AnoncredsUtils;
use utils::blob_storage::BlobStorageUtils;
use utils::anoncreds::{COMMON_MASTER_SECRET, CREDENTIAL1_ID};
use utils::test::TestUtils;

use utils::constants::*;

use utils::domain::schema::Schema;
use utils::domain::credential_definition::CredentialDefinition;
use utils::domain::revocation_registry_definition::RevocationRegistryDefinition;
use utils::domain::proof::Proof;
use utils::domain::revocation_state::RevocationState;
use utils::domain::revocation_registry::RevocationRegistry;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;
use utils::did::DidUtils;
use std::thread;

#[cfg(feature = "revocation_tests")]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_demand() {
    TestUtils::cleanup_storage();

    // Open Pool
    let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

    // Issuer creates wallet, gets wallet handle
    let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Issuer create DID
    let (issuer_did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(issuer_wallet_handle, pool_handle).unwrap();

    // Prover creates wallet, gets wallet handle
    let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover create DID
    let (prover_did, prover_verkey) = DidUtils::create_my_did(prover_wallet_handle, "{}").unwrap();

    // Issuer publish Prover DID
    let nym_request = LedgerUtils::build_nym_request(&issuer_did, &prover_did, Some(&prover_verkey), None, None).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &nym_request).unwrap();

    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry

    // Issuer creates Schema
    let (_, schema_json) = AnoncredsUtils::issuer_create_schema(&issuer_did,
                                                                GVT_SCHEMA_NAME,
                                                                SCHEMA_VERSION,
                                                                GVT_SCHEMA_ATTRIBUTES).unwrap();

    // Issuer posts Schema to Ledger
    let schema_request = LedgerUtils::build_schema_request(&issuer_did, &schema_json).unwrap();
    let schema_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &schema_request).unwrap();

    // Issuer get Schema from Ledger
    let get_schema_data = json!({"name": GVT_SCHEMA_NAME, "version": SCHEMA_VERSION}).to_string();

    let get_schema_request = LedgerUtils::build_get_schema_request(&issuer_did, &issuer_did, &get_schema_data).unwrap();
    let get_schema_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_req_resp).unwrap();

    // !!IMPORTANT!!
    // It is important to get Schema from Ledger and parse it to get the correct schema JSON and correspondent id in Ledger
    // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Issuer creates CredentialDefinition
    let (cred_def_id, cred_def_json) = AnoncredsUtils::issuer_create_credential_definition(issuer_wallet_handle,
                                                                                           &issuer_did,
                                                                                           &schema_json,
                                                                                           TAG_1,
                                                                                           None,
                                                                                           &AnoncredsUtils::revocation_cred_def_config()).unwrap();

    // Issuer post CredentialDefinition to Ledger
    let cred_def_request = LedgerUtils::build_claim_def_txn(&issuer_did, &cred_def_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &cred_def_request).unwrap();

    // Issuer creates RevocationRegistry
    let tails_writer_config = AnoncredsUtils::tails_writer_config();
    let tails_writer_handle = BlobStorageUtils::open_writer("default", &tails_writer_config).unwrap();

    let (rev_reg_id, rev_reg_def_json, rev_reg_entry_json) =
        AnoncredsUtils::indy_issuer_create_and_store_revoc_reg(issuer_wallet_handle,
                                                               &issuer_did,
                                                               None,
                                                               TAG_1,
                                                               &cred_def_id,
                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#,
                                                               tails_writer_handle).unwrap();

    // Issuer posts RevocationRegistryDefinition to Ledger
    let rev_reg_def_request = LedgerUtils::build_revoc_reg_def_request(&issuer_did, &rev_reg_def_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_def_request).unwrap();

    // Issuer posts RevocationRegistryEntry to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &rev_reg_entry_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();


    // Issuance Credential for Prover

    // Prover creates Master Secret
    AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

    // Issuer creates Credential Offer
    let cred_offer_json = AnoncredsUtils::issuer_create_credential_offer(issuer_wallet_handle, &cred_def_id).unwrap();

    // Prover gets CredentialDefinition from Ledger
    let get_cred_def_request = LedgerUtils::build_get_claim_def_txn(&prover_did, schema_id.parse::<i32>().unwrap(), &SIGNATURE_TYPE, &issuer_did).unwrap();
    let get_cred_def_response = LedgerUtils::submit_request(pool_handle, &get_cred_def_request).unwrap();
    let (cred_def_id, cred_def_json) = LedgerUtils::parse_get_claim_def_response(&get_cred_def_response).unwrap();

    // Prover creates Credential Request
    let (cred_req_json, cred_req_metadata_json) = AnoncredsUtils::prover_create_credential_req(prover_wallet_handle,
                                                                                               &prover_did,
                                                                                               &cred_offer_json,
                                                                                               &cred_def_json,
                                                                                               COMMON_MASTER_SECRET).unwrap();

    // Issuer creates TailsReader
    let blob_storage_reader_handle = BlobStorageUtils::open_reader(TYPE, &tails_writer_config).unwrap();


    // Issuer creates Credential
    let (cred_json, cred_rev_id, revoc_reg_delta_json) = AnoncredsUtils::issuer_create_credential(issuer_wallet_handle,
                                                                                                  &cred_offer_json,
                                                                                                  &cred_req_json,
                                                                                                  &AnoncredsUtils::gvt_credential_values_json(),
                                                                                                  Some(&rev_reg_id),
                                                                                                  Some(blob_storage_reader_handle)).unwrap();
    let revoc_reg_delta_json = revoc_reg_delta_json.unwrap();
    let cred_rev_id = cred_rev_id.unwrap();

    // Issuer posts RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &revoc_reg_delta_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

    // Prover gets RevocationRegistryDefinition
    let get_rev_reg_def_request = LedgerUtils::build_get_revoc_reg_def_request(&prover_did, &rev_reg_id).unwrap();
    let get_rev_reg_def_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();
    let (rev_reg_id, revoc_reg_def_json) = LedgerUtils::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();

    // Prover store received Credential
    AnoncredsUtils::prover_store_credential(prover_wallet_handle,
                                            CREDENTIAL1_ID,
                                            &cred_req_json,
                                            &cred_req_metadata_json,
                                            &cred_json,
                                            &cred_def_json,
                                            Some(&revoc_reg_def_json)).unwrap();

    // Verifying Prover Credential
    thread::sleep(std::time::Duration::from_secs(3));

    let to = time::get_time().sec;

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
           "non_revoked": json!({ "to": to.clone() })
        }).to_string();

    // Prover gets Claims for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
    let credential = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &rev_reg_id, None, to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let timestamp = to;
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp as u64,
                                                                 &cred_rev_id).unwrap();

    // Prover gets Schema from Ledger
    let get_schema_data = json!({"name": GVT_SCHEMA_NAME, "version": SCHEMA_VERSION}).to_string();
    let get_schema_request = LedgerUtils::build_get_schema_request(&prover_did, &issuer_did, &get_schema_data).unwrap();
    let get_schema_response = LedgerUtils::submit_request(pool_handle, &get_schema_request).unwrap();
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Prover creates Proof
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

    let cred_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

    let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

    let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         COMMON_MASTER_SECRET,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();
    let proof: Proof = serde_json::from_str(&proof_json).unwrap();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &rev_reg_id, timestamp).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

    // Verifier verifies proof
    assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

    let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

    let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&rev_reg_json).unwrap()
            })
        }).to_string();

    let valid = AnoncredsUtils::verifier_verify_proof(&proof_request,
                                                      &proof_json,
                                                      &schemas_json,
                                                      &cred_defs_json,
                                                      &rev_reg_defs_json,
                                                      &rev_regs_json).unwrap();
    assert!(valid);

    // Issuer revokes credential
    let rev_reg_delta_json = AnoncredsUtils::issuer_revoke_credential(issuer_wallet_handle, blob_storage_reader_handle, &rev_reg_id, &cred_rev_id).unwrap();

    // Issuer post RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &rev_reg_delta_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

    // Verifying Prover Credential after Revocation
    thread::sleep(std::time::Duration::from_secs(3));

    let from = to;
    let to = time::get_time().sec;

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &rev_reg_id, Some(from), to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let timestamp = to;
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp as u64,
                                                                 &cred_rev_id).unwrap();

    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp })
             })
        }).to_string();

    let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

    let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         COMMON_MASTER_SECRET,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &rev_reg_id, timestamp).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

    let timestamp = to;
    let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&rev_reg_json).unwrap()
            })
        }).to_string();

    let valid = AnoncredsUtils::verifier_verify_proof(&proof_request,
                                                      &proof_json,
                                                      &schemas_json,
                                                      &cred_defs_json,
                                                      &rev_reg_defs_json,
                                                      &rev_regs_json).unwrap();
    assert!(!valid);


    WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover_wallet_handle).unwrap();

    TestUtils::cleanup_storage();
}

#[cfg(feature = "revocation_tests")]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_default() {
    TestUtils::cleanup_storage();

    // Open Pool
    let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

    // Issuer creates wallet, gets wallet handle
    let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Issuer create DID
    let (issuer_did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(issuer_wallet_handle, pool_handle).unwrap();

    // Prover creates wallet, gets wallet handle
    let prover_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover create DID
    let (prover_did, prover_verkey) = DidUtils::create_my_did(prover_wallet_handle, "{}").unwrap();

    // Issuer publish Prover DID
    let nym_request = LedgerUtils::build_nym_request(&issuer_did, &prover_did, Some(&prover_verkey), None, None).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &nym_request).unwrap();

    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry

    // Issuer creates Schema
    let (_, schema_json) = AnoncredsUtils::issuer_create_schema(&issuer_did,
                                                                GVT_SCHEMA_NAME,
                                                                SCHEMA_VERSION,
                                                                GVT_SCHEMA_ATTRIBUTES).unwrap();

    // Issuer posts Schema to Ledger
    let schema_request = LedgerUtils::build_schema_request(&issuer_did, &schema_json).unwrap();
    let schema_req_resp = LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &schema_request).unwrap();

    // Issuer get Schema from Ledger
    let get_schema_data = json!({"name": GVT_SCHEMA_NAME, "version": SCHEMA_VERSION}).to_string();

    let get_schema_request = LedgerUtils::build_get_schema_request(&issuer_did, &issuer_did, &get_schema_data).unwrap();
    let get_schema_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_req_resp).unwrap();

    // !!IMPORTANT!!
    // It is important to get Schema from Ledger and parse it to get the correct schema JSON and correspondent id in Ledger
    // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Issuer creates CredentialDefinition
    let (cred_def_id, cred_def_json) = AnoncredsUtils::issuer_create_credential_definition(issuer_wallet_handle,
                                                                                           &issuer_did,
                                                                                           &schema_json,
                                                                                           TAG_1,
                                                                                           None,
                                                                                           &AnoncredsUtils::revocation_cred_def_config()).unwrap();

    // Issuer post CredentialDefinition to Ledger
    let cred_def_request = LedgerUtils::build_claim_def_txn(&issuer_did, &cred_def_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &cred_def_request).unwrap();

    // Issuer creates RevocationRegistry
    let tails_writer_config = AnoncredsUtils::tails_writer_config();
    let tails_writer_handle = BlobStorageUtils::open_writer("default", &tails_writer_config).unwrap();

    let (rev_reg_id, rev_reg_def_json, rev_reg_entry_json) =
        AnoncredsUtils::indy_issuer_create_and_store_revoc_reg(issuer_wallet_handle,
                                                               &issuer_did,
                                                               None,
                                                               TAG_1,
                                                               &cred_def_id,
                                                               r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_BY_DEFAULT"}"#,
                                                               tails_writer_handle).unwrap();

    // Issuer posts RevocationRegistryDefinition to Ledger
    let rev_reg_def_request = LedgerUtils::build_revoc_reg_def_request(&issuer_did, &rev_reg_def_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_def_request).unwrap();

    // Issuer posts RevocationRegistryEntry to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &rev_reg_entry_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();


    // Issuance Credential for Prover

    // Prover creates Master Secret
    AnoncredsUtils::prover_create_master_secret(prover_wallet_handle, COMMON_MASTER_SECRET).unwrap();

    // Issuer creates Credential Offer
    let cred_offer_json = AnoncredsUtils::issuer_create_credential_offer(issuer_wallet_handle, &cred_def_id).unwrap();

    // Prover gets CredentialDefinition from Ledger
    let get_cred_def_request = LedgerUtils::build_get_claim_def_txn(&prover_did, schema_id.parse::<i32>().unwrap(), &SIGNATURE_TYPE, &issuer_did).unwrap();
    let get_cred_def_response = LedgerUtils::submit_request(pool_handle, &get_cred_def_request).unwrap();
    let (cred_def_id, cred_def_json) = LedgerUtils::parse_get_claim_def_response(&get_cred_def_response).unwrap();

    // Prover creates Credential Request
    let (cred_req_json, cred_req_metadata_json) = AnoncredsUtils::prover_create_credential_req(prover_wallet_handle,
                                                                                               &prover_did,
                                                                                               &cred_offer_json,
                                                                                               &cred_def_json,
                                                                                               COMMON_MASTER_SECRET).unwrap();

    // Issuer creates TailsReader
    let blob_storage_reader_handle = BlobStorageUtils::open_reader(TYPE, &tails_writer_config).unwrap();


    // Issuer creates Credential
    // Issuer must not post rev_reg_delta to ledger for ISSUANCE_BY_DEFAULT strategy
    let (cred_json, cred_rev_id, _) = AnoncredsUtils::issuer_create_credential(issuer_wallet_handle,
                                                                                                  &cred_offer_json,
                                                                                                  &cred_req_json,
                                                                                                  &AnoncredsUtils::gvt_credential_values_json(),
                                                                                                  Some(&rev_reg_id),
                                                                                                  Some(blob_storage_reader_handle)).unwrap();
    let cred_rev_id = cred_rev_id.unwrap();

    // Prover gets RevocationRegistryDefinition
    let get_rev_reg_def_request = LedgerUtils::build_get_revoc_reg_def_request(&prover_did, &rev_reg_id).unwrap();
    let get_rev_reg_def_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();
    let (rev_reg_id, revoc_reg_def_json) = LedgerUtils::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();

    // Prover store received Credential
    AnoncredsUtils::prover_store_credential(prover_wallet_handle,
                                            CREDENTIAL1_ID,
                                            &cred_req_json,
                                            &cred_req_metadata_json,
                                            &cred_json,
                                            &cred_def_json,
                                            Some(&revoc_reg_def_json)).unwrap();

    // Verifying Prover Credential
    thread::sleep(std::time::Duration::from_secs(3));

    let to = time::get_time().sec;

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
           "non_revoked": json!({ "to": to.clone() })
        }).to_string();

    // Prover gets Claims for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
    let credential = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &rev_reg_id, None, to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let timestamp = to;
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp as u64,
                                                                 &cred_rev_id).unwrap();

    // Prover gets Schema from Ledger
    let get_schema_data = json!({"name": GVT_SCHEMA_NAME, "version": SCHEMA_VERSION}).to_string();
    let get_schema_request = LedgerUtils::build_get_schema_request(&prover_did, &issuer_did, &get_schema_data).unwrap();
    let get_schema_response = LedgerUtils::submit_request(pool_handle, &get_schema_request).unwrap();
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Prover creates Proof
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

    let cred_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

    let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

    let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         COMMON_MASTER_SECRET,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();
    let proof: Proof = serde_json::from_str(&proof_json).unwrap();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &rev_reg_id, timestamp).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

    // Verifier verifies proof
    assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

    let rev_reg_defs_json = json!({
            rev_reg_id.clone(): serde_json::from_str::<RevocationRegistryDefinition>(&revoc_reg_def_json).unwrap()
        }).to_string();

    let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&rev_reg_json).unwrap()
            })
        }).to_string();

    let valid = AnoncredsUtils::verifier_verify_proof(&proof_request,
                                                      &proof_json,
                                                      &schemas_json,
                                                      &cred_defs_json,
                                                      &rev_reg_defs_json,
                                                      &rev_regs_json).unwrap();
    assert!(valid);

    // Issuer revokes credential
    let rev_reg_delta_json = AnoncredsUtils::issuer_revoke_credential(issuer_wallet_handle, blob_storage_reader_handle, &rev_reg_id, &cred_rev_id).unwrap();

    // Issuer post RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &rev_reg_delta_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

    // Verifying Prover Credential after Revocation
    thread::sleep(std::time::Duration::from_secs(3));

    let from = to;
    let to = time::get_time().sec;

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &rev_reg_id, Some(from), to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let timestamp = to;
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp as u64,
                                                                 &cred_rev_id).unwrap();

    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp })
             })
        }).to_string();

    let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

    let proof_json = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         COMMON_MASTER_SECRET,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &rev_reg_id, timestamp).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

    let timestamp = to;
    let rev_regs_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationRegistry>(&rev_reg_json).unwrap()
            })
        }).to_string();

    let valid = AnoncredsUtils::verifier_verify_proof(&proof_request,
                                                      &proof_json,
                                                      &schemas_json,
                                                      &cred_defs_json,
                                                      &rev_reg_defs_json,
                                                      &rev_regs_json).unwrap();
    assert!(!valid);


    WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover_wallet_handle).unwrap();

    TestUtils::cleanup_storage();
}
