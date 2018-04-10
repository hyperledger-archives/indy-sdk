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
use utils::anoncreds::{COMMON_MASTER_SECRET, CREDENTIAL1_ID, CREDENTIAL2_ID, CREDENTIAL3_ID};
use utils::test::TestUtils;

use utils::constants::*;

use utils::domain::schema::Schema;
use utils::domain::credential_definition::CredentialDefinition;
use utils::domain::credential_offer::CredentialOffer;
use utils::domain::credential::Credential;
use utils::domain::revocation_registry_definition::RevocationRegistryDefinition;
use utils::domain::proof::Proof;
use utils::domain::revocation_state::RevocationState;
use utils::domain::revocation_registry::RevocationRegistry;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;
use utils::did::DidUtils;

use indy_crypto::utils::json::JsonDecodable;

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
    let (schema_id, schema_json) = AnoncredsUtils::issuer_create_schema(&issuer_did,
                                                                        GVT_SCHEMA_NAME,
                                                                        SCHEMA_VERSION,
                                                                        GVT_SCHEMA_ATTRIBUTES).unwrap();

    // !!IMPORTANT!!
    // It is important Post and Get Schema from Ledger and parse it to get the correct Schema JSON and correspondent it seq_no in Ledger
    // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)

    // Issuer posts Schema to Ledger
    let schema_request = LedgerUtils::build_schema_request(&issuer_did, &schema_json).unwrap();
    let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &schema_request).unwrap();

    // Issuer gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&issuer_did, &schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_response).unwrap();
    let (_, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Issuer creates CredentialDefinition
    let (cred_def_id, cred_def_json) = AnoncredsUtils::issuer_create_credential_definition(issuer_wallet_handle,
                                                                                           &issuer_did,
                                                                                           &schema_json,
                                                                                           TAG_1,
                                                                                           None,
                                                                                           &AnoncredsUtils::revocation_cred_def_config()).unwrap();

    // Issuer post CredentialDefinition to Ledger
    let cred_def_request = LedgerUtils::build_cred_def_txn(&issuer_did, &cred_def_json).unwrap();
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
    let cred_offer = CredentialOffer::from_json(&cred_offer_json).unwrap();

    // Prover gets CredentialDefinition from Ledger
    let get_cred_def_request = LedgerUtils::build_get_cred_def_txn(&prover_did, &cred_offer.cred_def_id).unwrap();
    let get_cred_def_response = LedgerUtils::submit_request(pool_handle, &get_cred_def_request).unwrap();
    let (cred_def_id, cred_def_json) = LedgerUtils::parse_get_cred_def_response(&get_cred_def_response).unwrap();

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
    let credential = Credential::from_json(&cred_json).unwrap();
    let get_rev_reg_def_request = LedgerUtils::build_get_revoc_reg_def_request(&prover_did, &credential.rev_reg_id.unwrap()).unwrap();
    let get_rev_reg_def_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();
    let (_, revoc_reg_def_json) = LedgerUtils::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();

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

    let to = time::get_time().sec as u64;

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

    // Prover gets Credentials for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
    let cred_info = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &cred_info.rev_reg_id.clone().unwrap(), None, to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp,
                                                                 &cred_info.cred_rev_id.clone().unwrap()).unwrap();

    // Prover gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&prover_did, &cred_info.schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request(pool_handle, &get_schema_request).unwrap();
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Prover creates Proof
    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": cred_info.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": cred_info.referent, "timestamp": timestamp })
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

    // Verifier gets required entities from Ledger
    assert_eq!(1, proof.identifiers.len());
    let identifier = proof.identifiers[0].clone();

    // Verifier gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(DID_MY1, &identifier.schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request(pool_handle, &get_schema_request).unwrap();
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Verifier gets CredentialDefinition from Ledger
    let get_cred_def_request = LedgerUtils::build_get_cred_def_txn(DID_MY1, &identifier.cred_def_id).unwrap();
    let get_cred_def_response = LedgerUtils::submit_request(pool_handle, &get_cred_def_request).unwrap();
    let (cred_def_id, cred_def_json) = LedgerUtils::parse_get_cred_def_response(&get_cred_def_response).unwrap();

    // Verifier gets RevocationRegistryDefinition from Ledger
    let get_rev_reg_def_request = LedgerUtils::build_get_revoc_reg_def_request(&DID_MY1, &identifier.rev_reg_id.clone().unwrap()).unwrap();
    let get_rev_reg_def_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();
    let (_, revoc_reg_def_json) = LedgerUtils::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &identifier.rev_reg_id.clone().unwrap(), identifier.timestamp.unwrap()).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json, timestamp) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

    // Verifier verifies proof
    assert_eq!("Alex", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

    let schemas_json = json!({
            schema_id.clone(): serde_json::from_str::<Schema>(&schema_json).unwrap()
        }).to_string();

    let cred_defs_json = json!({
            cred_def_id.clone(): serde_json::from_str::<CredentialDefinition>(&cred_def_json).unwrap()
        }).to_string();

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

    // Issuer revokes cred_info
    let rev_reg_delta_json = AnoncredsUtils::issuer_revoke_credential(issuer_wallet_handle, blob_storage_reader_handle, &rev_reg_id, &cred_rev_id).unwrap();

    // Issuer post RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &rev_reg_delta_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

    // Verifying Prover Credential after Revocation
    thread::sleep(std::time::Duration::from_secs(3));

    let from = to;
    let to = time::get_time().sec as u64;

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &cred_info.rev_reg_id.clone().unwrap(), Some(from), to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let rev_state_json = AnoncredsUtils::update_revocation_state(blob_storage_reader_handle,
                                                                 &rev_state_json,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp,
                                                                 &cred_info.cred_rev_id.clone().unwrap()).unwrap();

    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": cred_info.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({
                "predicate1_referent": json!({ "cred_id": cred_info.referent, "timestamp": timestamp })
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
    let proof: Proof = serde_json::from_str(&proof_json).unwrap();
    let identifier = proof.identifiers[0].clone();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &identifier.rev_reg_id.unwrap(), identifier.timestamp.unwrap()).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json, timestamp) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

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

    PoolUtils::close(pool_handle).unwrap();

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
    let (schema_id, schema_json) = AnoncredsUtils::issuer_create_schema(&issuer_did,
                                                                        GVT_SCHEMA_NAME,
                                                                        SCHEMA_VERSION,
                                                                        GVT_SCHEMA_ATTRIBUTES).unwrap();

    // !!IMPORTANT!!
    // It is important Post and Get Schema from Ledger and parse it to get the correct Schema JSON and correspondent it seq_no in Ledger
    // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)

    // Issuer posts Schema to Ledger
    let schema_request = LedgerUtils::build_schema_request(&issuer_did, &schema_json).unwrap();
    let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &schema_request).unwrap();

    // Issuer gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&issuer_did, &schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_response).unwrap();
    let (_, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Issuer creates CredentialDefinition
    let (cred_def_id, cred_def_json) = AnoncredsUtils::issuer_create_credential_definition(issuer_wallet_handle,
                                                                                           &issuer_did,
                                                                                           &schema_json,
                                                                                           TAG_1,
                                                                                           None,
                                                                                           &AnoncredsUtils::revocation_cred_def_config()).unwrap();

    // Issuer post CredentialDefinition to Ledger
    let cred_def_request = LedgerUtils::build_cred_def_txn(&issuer_did, &cred_def_json).unwrap();
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
    let get_cred_def_request = LedgerUtils::build_get_cred_def_txn(&prover_did, &cred_def_id).unwrap();
    let get_cred_def_response = LedgerUtils::submit_request(pool_handle, &get_cred_def_request).unwrap();
    let (cred_def_id, cred_def_json) = LedgerUtils::parse_get_cred_def_response(&get_cred_def_response).unwrap();

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

    let to = time::get_time().sec as u64;

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

    // Prover gets Credentials for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover_wallet_handle, &proof_request).unwrap();
    let credential = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &rev_reg_id, None, to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp,
                                                                 &cred_rev_id).unwrap();

    // Prover gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&prover_did, &schema_id).unwrap();
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
    let (rev_reg_id, rev_reg_json, timestamp) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

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
    let to = time::get_time().sec as u64;

    // Prover gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover_did, &rev_reg_id, Some(from), to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover creates RevocationState
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp,
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
    let (rev_reg_id, rev_reg_json, timestamp) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

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

    PoolUtils::close(pool_handle).unwrap();

    TestUtils::cleanup_storage();
}

#[cfg(feature = "revocation_tests")]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_demand_three_credentials_post_entry_three_times_proving_first() {
    TestUtils::cleanup_storage();

    // Open Pool
    let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

    // Issuer creates wallet, gets wallet handle
    let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Issuer create DID
    let (issuer_did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(issuer_wallet_handle, pool_handle).unwrap();

    // Prover creates wallet, gets wallet handle
    let prover1_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover2 creates wallet, gets wallet handle
    let prover2_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover3 creates wallet, gets wallet handle
    let prover3_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover1 create DID
    let (prover1_did, _) = DidUtils::create_my_did(prover1_wallet_handle, "{}").unwrap();

    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry

    // Issuer creates Schema
    let (schema_id, schema_json) = AnoncredsUtils::issuer_create_schema(&issuer_did,
                                                                        GVT_SCHEMA_NAME,
                                                                        SCHEMA_VERSION,
                                                                        GVT_SCHEMA_ATTRIBUTES).unwrap();

    // !!IMPORTANT!!
    // It is important Post and Get Schema from Ledger and parse it to get the correct Schema JSON and correspondent it seq_no in Ledger
    // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)

    // Issuer posts Schema to Ledger
    let schema_request = LedgerUtils::build_schema_request(&issuer_did, &schema_json).unwrap();
    let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &schema_request).unwrap();

    // Issuer gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&issuer_did, &schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_response).unwrap();
    let (_, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Issuer creates CredentialDefinition
    let (cred_def_id, cred_def_json) = AnoncredsUtils::issuer_create_credential_definition(issuer_wallet_handle,
                                                                                           &issuer_did,
                                                                                           &schema_json,
                                                                                           TAG_1,
                                                                                           None,
                                                                                           &AnoncredsUtils::revocation_cred_def_config()).unwrap();

    // Issuer post CredentialDefinition to Ledger
    let cred_def_request = LedgerUtils::build_cred_def_txn(&issuer_did, &cred_def_json).unwrap();
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

    // Issuer creates TailsReader
    let blob_storage_reader_handle = BlobStorageUtils::open_reader(TYPE, &tails_writer_config).unwrap();

    // Gets CredentialDefinition from Ledger
    let get_cred_def_request = LedgerUtils::build_get_cred_def_txn(&prover1_did, &cred_def_id).unwrap();
    let get_cred_def_response = LedgerUtils::submit_request(pool_handle, &get_cred_def_request).unwrap();
    let (cred_def_id, cred_def_json) = LedgerUtils::parse_get_cred_def_response(&get_cred_def_response).unwrap();

    // Gets RevocationRegistryDefinition
    let get_rev_reg_def_request = LedgerUtils::build_get_revoc_reg_def_request(&prover1_did, &rev_reg_id).unwrap();
    let get_rev_reg_def_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();
    let (rev_reg_id, revoc_reg_def_json) = LedgerUtils::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();

    /*ISSUANCE CREDENTIAL FOR PROVER1*/
    // Prover1 creates Master Secret
    let prover1_master_secret_id = "prover1_master_secret";
    AnoncredsUtils::prover_create_master_secret(prover1_wallet_handle, prover1_master_secret_id).unwrap();

    let (prover1_cred_rev_id, revoc_reg_delta1_json) = AnoncredsUtils::multi_steps_create_revocation_credential(
        prover1_master_secret_id,
        prover1_wallet_handle,
        issuer_wallet_handle,
        CREDENTIAL1_ID,
        &AnoncredsUtils::gvt_credential_values_json(),
        &cred_def_id,
        &cred_def_json,
        &rev_reg_id,
        &revoc_reg_def_json,
        blob_storage_reader_handle,
    );
    let revoc_reg_delta1_json = revoc_reg_delta1_json.unwrap();

    // Issuer posts RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &revoc_reg_delta1_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();


    /*ISSUANCE CREDENTIAL FOR PROVER2*/
    // Prover2 creates Master Secret
    let prover2_master_secret_id = "prover2_master_secret";
    AnoncredsUtils::prover_create_master_secret(prover2_wallet_handle, prover2_master_secret_id).unwrap();

    let (_, revoc_reg_delta2_json) = AnoncredsUtils::multi_steps_create_revocation_credential(
        prover2_master_secret_id,
        prover2_wallet_handle,
        issuer_wallet_handle,
        CREDENTIAL2_ID,
        &AnoncredsUtils::gvt2_credential_values_json(),
        &cred_def_id,
        &cred_def_json,
        &rev_reg_id,
        &revoc_reg_def_json,
        blob_storage_reader_handle,
    );
    let revoc_reg_delta2_json = revoc_reg_delta2_json.unwrap();

    // Issuer posts RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &revoc_reg_delta2_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

    /*ISSUANCE CREDENTIAL FOR PROVER3*/
    // Prover3 creates Master Secret
    let prover3_master_secret_id = "prover3_master_secret";
    AnoncredsUtils::prover_create_master_secret(prover3_wallet_handle, prover3_master_secret_id).unwrap();

    let (_, revoc_reg_delta3_json) = AnoncredsUtils::multi_steps_create_revocation_credential(
        prover3_master_secret_id,
        prover3_wallet_handle,
        issuer_wallet_handle,
        CREDENTIAL3_ID,
        &AnoncredsUtils::gvt3_credential_values_json(),
        &cred_def_id,
        &cred_def_json,
        &rev_reg_id,
        &revoc_reg_def_json,
        blob_storage_reader_handle,
    );
    let revoc_reg_delta3_json = revoc_reg_delta3_json.unwrap();

    // Issuer posts RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &revoc_reg_delta3_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

    // Verifying Prover1 Credential
    thread::sleep(std::time::Duration::from_secs(3));

    let to = time::get_time().sec as u64;

    let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({}),
           "non_revoked": json!({ "to": to.clone() })
        }).to_string();

    // Prover1 gets Credentials for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover1_wallet_handle, &proof_request).unwrap();
    let credential = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover1 gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover1_did, &rev_reg_id, None, to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover1 creates RevocationState
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp as u64,
                                                                 &prover1_cred_rev_id).unwrap();

    // Prover1 gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&prover1_did, &schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request(pool_handle, &get_schema_request).unwrap();
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Prover1 creates Proof
    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({})
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

    let proof_json = AnoncredsUtils::prover_create_proof(prover1_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover1_master_secret_id,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();
    let proof: Proof = serde_json::from_str(&proof_json).unwrap();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &rev_reg_id, timestamp).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json, timestamp) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

    // Verifier verifies proof from Prover1
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

    WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover1_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover2_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover3_wallet_handle).unwrap();

    PoolUtils::close(pool_handle).unwrap();

    TestUtils::cleanup_storage();
}

#[cfg(feature = "revocation_tests")]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_demand_three_credentials_post_common_entry_proving_all() {
    TestUtils::cleanup_storage();

    // Open Pool
    let pool_handle = PoolUtils::create_and_open_pool_ledger(POOL).unwrap();

    // Issuer creates wallet, gets wallet handle
    let issuer_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Issuer create DID
    let (issuer_did, _) = DidUtils::create_store_and_publish_my_did_from_trustee(issuer_wallet_handle, pool_handle).unwrap();

    // Prover creates wallet, gets wallet handle
    let prover1_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover2 creates wallet, gets wallet handle
    let prover2_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover3 creates wallet, gets wallet handle
    let prover3_wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

    // Prover1 create DID
    let (prover1_did, _) = DidUtils::create_my_did(prover1_wallet_handle, "{}").unwrap();

    // Prover2 create DID
    let (prover2_did, _) = DidUtils::create_my_did(prover1_wallet_handle, "{}").unwrap();

    // Prover3 create DID
    let (prover3_did, _) = DidUtils::create_my_did(prover1_wallet_handle, "{}").unwrap();

    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry

    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry

    // Issuer creates Schema
    let (schema_id, schema_json) = AnoncredsUtils::issuer_create_schema(&issuer_did,
                                                                        GVT_SCHEMA_NAME,
                                                                        SCHEMA_VERSION,
                                                                        GVT_SCHEMA_ATTRIBUTES).unwrap();

    // !!IMPORTANT!!
    // It is important Post and Get Schema from Ledger and parse it to get the correct Schema JSON and correspondent it seq_no in Ledger
    // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)

    // Issuer posts Schema to Ledger
    let schema_request = LedgerUtils::build_schema_request(&issuer_did, &schema_json).unwrap();
    let schema_response = LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &schema_request).unwrap();

    // Issuer gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&issuer_did, &schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request_with_retries(pool_handle, &get_schema_request, &schema_response).unwrap();
    let (_, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Issuer creates CredentialDefinition
    let (cred_def_id, cred_def_json) = AnoncredsUtils::issuer_create_credential_definition(issuer_wallet_handle,
                                                                                           &issuer_did,
                                                                                           &schema_json,
                                                                                           TAG_1,
                                                                                           None,
                                                                                           &AnoncredsUtils::revocation_cred_def_config()).unwrap();

    // Issuer post CredentialDefinition to Ledger
    let cred_def_request = LedgerUtils::build_cred_def_txn(&issuer_did, &cred_def_json).unwrap();
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

    // Issuer creates TailsReader
    let blob_storage_reader_handle = BlobStorageUtils::open_reader(TYPE, &tails_writer_config).unwrap();

    // Gets CredentialDefinition from Ledger
    let get_cred_def_request = LedgerUtils::build_get_cred_def_txn(&prover1_did, &cred_def_id).unwrap();
    let get_cred_def_response = LedgerUtils::submit_request(pool_handle, &get_cred_def_request).unwrap();
    let (cred_def_id, cred_def_json) = LedgerUtils::parse_get_cred_def_response(&get_cred_def_response).unwrap();

    // Gets RevocationRegistryDefinition
    let get_rev_reg_def_request = LedgerUtils::build_get_revoc_reg_def_request(&prover1_did, &rev_reg_id).unwrap();
    let get_rev_reg_def_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_def_request).unwrap();
    let (rev_reg_id, revoc_reg_def_json) = LedgerUtils::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap();

    /*ISSUANCE CREDENTIAL FOR PROVER1*/
    // Prover1 creates Master Secret
    let prover1_master_secret_id = "prover1_master_secret";
    AnoncredsUtils::prover_create_master_secret(prover1_wallet_handle, prover1_master_secret_id).unwrap();

    let (prover1_cred_rev_id, revoc_reg_delta1_json) = AnoncredsUtils::multi_steps_create_revocation_credential(
        prover1_master_secret_id,
        prover1_wallet_handle,
        issuer_wallet_handle,
        CREDENTIAL1_ID,
        &AnoncredsUtils::gvt_credential_values_json(),
        &cred_def_id,
        &cred_def_json,
        &rev_reg_id,
        &revoc_reg_def_json,
        blob_storage_reader_handle,
    );
    let revoc_reg_delta1_json = revoc_reg_delta1_json.unwrap();

    /*ISSUANCE CREDENTIAL FOR PROVER2*/
    // Prover2 creates Master Secret
    let prover2_master_secret_id = "prover2_master_secret";
    AnoncredsUtils::prover_create_master_secret(prover2_wallet_handle, prover2_master_secret_id).unwrap();

    let (prover2_cred_rev_id, revoc_reg_delta2_json) = AnoncredsUtils::multi_steps_create_revocation_credential(
        prover2_master_secret_id,
        prover2_wallet_handle,
        issuer_wallet_handle,
        CREDENTIAL2_ID,
        &AnoncredsUtils::gvt2_credential_values_json(),
        &cred_def_id,
        &cred_def_json,
        &rev_reg_id,
        &revoc_reg_def_json,
        blob_storage_reader_handle,
    );
    let revoc_reg_delta2_json = revoc_reg_delta2_json.unwrap();

    // Issuer merge Revocation Registry Deltas
    let revoc_reg_delta_json = AnoncredsUtils::issuer_merge_revocation_registry_deltas(&revoc_reg_delta1_json, &revoc_reg_delta2_json).unwrap();

    /*ISSUANCE CREDENTIAL FOR PROVER3*/
    // Prover3 creates Master Secret
    let prover3_master_secret_id = "prover3_master_secret";
    AnoncredsUtils::prover_create_master_secret(prover3_wallet_handle, prover3_master_secret_id).unwrap();

    let (prover3_cred_rev_id, revoc_reg_delta3_json) = AnoncredsUtils::multi_steps_create_revocation_credential(
        prover3_master_secret_id,
        prover3_wallet_handle,
        issuer_wallet_handle,
        CREDENTIAL3_ID,
        &AnoncredsUtils::gvt3_credential_values_json(),
        &cred_def_id,
        &cred_def_json,
        &rev_reg_id,
        &revoc_reg_def_json,
        blob_storage_reader_handle,
    );
    let revoc_reg_delta3_json = revoc_reg_delta3_json.unwrap();

    // Issuer merge Revocation Registry Deltas
    let revoc_reg_delta_json = AnoncredsUtils::issuer_merge_revocation_registry_deltas(&revoc_reg_delta_json, &revoc_reg_delta3_json).unwrap();

    // Issuer posts merged RevocationRegistryDelta to Ledger
    let rev_reg_entry_request =
        LedgerUtils::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &revoc_reg_delta_json).unwrap();
    LedgerUtils::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

    // Verifying Prover1 Credential
    thread::sleep(std::time::Duration::from_secs(3));

    let to = time::get_time().sec as u64;

    let proof_request = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "attr1_referent": json!({
                   "name":"name"
               })
           }),
           "requested_predicates": json!({}),
           "non_revoked": json!({ "to": to.clone() })
        }).to_string();

    // Prover1 gets Credentials for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover1_wallet_handle, &proof_request).unwrap();
    let credential = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover1 gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover1_did, &rev_reg_id, None, to).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover1 creates RevocationState
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp,
                                                                 &prover1_cred_rev_id).unwrap();

    // Prover1 gets Schema from Ledger
    let get_schema_request = LedgerUtils::build_get_schema_request(&prover1_did, &schema_id).unwrap();
    let get_schema_response = LedgerUtils::submit_request(pool_handle, &get_schema_request).unwrap();
    let (schema_id, schema_json) = LedgerUtils::parse_get_schema_response(&get_schema_response).unwrap();

    // Prover1 creates Proof
    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({})
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

    let proof_json = AnoncredsUtils::prover_create_proof(prover1_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover1_master_secret_id,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();
    let proof: Proof = serde_json::from_str(&proof_json).unwrap();

    // Verifier gets RevocationRegistry from Ledger
    let get_rev_reg_req = LedgerUtils::build_get_revoc_reg_request(DID_MY1, &rev_reg_id, timestamp).unwrap();
    let get_rev_reg_resp = LedgerUtils::submit_request(pool_handle, &get_rev_reg_req).unwrap();
    let (rev_reg_id, rev_reg_json, timestamp) = LedgerUtils::parse_get_revoc_reg_response(&get_rev_reg_resp).unwrap();

    // Verifier verifies proof from Prover1
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

    // Verifying Prover2 Credential
    // Prover2 gets Credentials for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover2_wallet_handle, &proof_request).unwrap();
    let credential = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover2 gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover2_did, &rev_reg_id, None, timestamp).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover2 creates RevocationState
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp as u64,
                                                                 &prover2_cred_rev_id).unwrap();
    // Prover2 creates Proof
    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({})
        }).to_string();

    let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

    let proof_json = AnoncredsUtils::prover_create_proof(prover2_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover2_master_secret_id,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();
    let proof: Proof = serde_json::from_str(&proof_json).unwrap();

    // Verifier verifies proof from Prover2
    assert_eq!("Alexander", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

    let valid = AnoncredsUtils::verifier_verify_proof(&proof_request,
                                                      &proof_json,
                                                      &schemas_json,
                                                      &cred_defs_json,
                                                      &rev_reg_defs_json,
                                                      &rev_regs_json).unwrap();

    assert!(valid);


    // Verifying Prover3 Credential
    // Prover3 gets Credentials for Proof Request
    let credentials_json = AnoncredsUtils::prover_get_credentials_for_proof_req(prover3_wallet_handle, &proof_request).unwrap();
    let credential = AnoncredsUtils::get_credential_for_attr_referent(&credentials_json, "attr1_referent");

    // Prover3 gets RevocationRegistryDelta from Ledger
    let get_rev_reg_delta_request = LedgerUtils::build_get_revoc_reg_delta_request(&prover3_did, &rev_reg_id, None, timestamp).unwrap();
    let get_rev_reg_delta_response = LedgerUtils::submit_request(pool_handle, &get_rev_reg_delta_request).unwrap();
    let (rev_reg_id, revoc_reg_delta_json, timestamp) = LedgerUtils::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap();

    // Prover3 creates RevocationState
    let rev_state_json = AnoncredsUtils::create_revocation_state(blob_storage_reader_handle,
                                                                 &revoc_reg_def_json,
                                                                 &revoc_reg_delta_json,
                                                                 timestamp,
                                                                 &prover3_cred_rev_id).unwrap();
    // Prover3 creates Proof
    let requested_credentials_json = json!({
             "self_attested_attributes": json!({}),
             "requested_attributes": json!({
                "attr1_referent": json!({ "cred_id": credential.referent, "timestamp": timestamp,  "revealed":true })
             }),
             "requested_predicates": json!({})
        }).to_string();

    let rev_states_json = json!({
            rev_reg_id.clone(): json!({
                timestamp.to_string(): serde_json::from_str::<RevocationState>(&rev_state_json).unwrap()
            })
        }).to_string();

    let proof_json = AnoncredsUtils::prover_create_proof(prover3_wallet_handle,
                                                         &proof_request,
                                                         &requested_credentials_json,
                                                         prover3_master_secret_id,
                                                         &schemas_json,
                                                         &cred_defs_json,
                                                         &rev_states_json).unwrap();
    let proof: Proof = serde_json::from_str(&proof_json).unwrap();

    // Verifier verifies proof from Prover3
    assert_eq!("Artem", proof.requested_proof.revealed_attrs.get("attr1_referent").unwrap().raw);

    let valid = AnoncredsUtils::verifier_verify_proof(&proof_request,
                                                      &proof_json,
                                                      &schemas_json,
                                                      &cred_defs_json,
                                                      &rev_reg_defs_json,
                                                      &rev_regs_json).unwrap();

    assert!(valid);

    WalletUtils::close_wallet(issuer_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover1_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover2_wallet_handle).unwrap();
    WalletUtils::close_wallet(prover3_wallet_handle).unwrap();

    PoolUtils::close(pool_handle).unwrap();

    TestUtils::cleanup_storage();
}