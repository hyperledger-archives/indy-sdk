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
extern crate core;

#[macro_use]
mod utils;

use utils::{wallet, anoncreds, blob_storage, pool, ledger, did};
use utils::anoncreds::{COMMON_MASTER_SECRET, CREDENTIAL1_ID};
#[cfg(any(feature = "force_full_interaction_tests", not(target_os = "android")))]
use utils::anoncreds::{CREDENTIAL2_ID, CREDENTIAL3_ID};

use utils::constants::*;

use utils::domain::anoncreds::schema::Schema;
use utils::domain::anoncreds::credential_definition::CredentialDefinition;
use utils::domain::anoncreds::credential_offer::CredentialOffer;
use utils::domain::anoncreds::credential::Credential;
use utils::domain::anoncreds::credential::CredentialInfo;
use utils::domain::anoncreds::revocation_registry_definition::RevocationRegistryDefinition;
use utils::domain::anoncreds::proof::Proof;
use utils::domain::anoncreds::revocation_state::RevocationState;
use utils::domain::anoncreds::revocation_registry::RevocationRegistry;

use std::thread;

use serde_json::Value;
use core::borrow::Borrow;


struct Pool{
    pool_handle : i32
}


struct Issuer{
    issuer_wallet_handle: i32,
    issuer_did : String,

    schema_id : String,
    cred_def_id : String,
    rev_reg_id : String,

    issuance_type: String,
    tails_writer_config: String
}


struct Prover{

    wallet_handle : i32,
    did: String,
    verkey : String,
    master_secret_id : String,
    cred_def_id : Option<String>,
    cred_req_metadata_json : Option<String>

}


struct Verifier{
    proof_request : String
}


impl Pool {


    pub fn new(pool_name: &str) -> Pool {
        Pool{ pool_handle : pool::create_and_open_pool_ledger(pool_name).unwrap() }
    }

    pub fn close(self) {
        let _ = pool::close(self.pool_handle);
    }


    pub fn submit_nym(&self, issuer_did: &str, issuer_wallet_handle: i32, prover_did: &str, prover_verkey : Option<&str>)
    {
        let nym_request = ledger::build_nym_request(issuer_did, prover_did, prover_verkey, None, None).unwrap();
        ledger::sign_and_submit_request(self.pool_handle, issuer_wallet_handle, &issuer_did, &nym_request).unwrap();
    }

    pub fn submit_schema(&self, issuer_did: &str, issuer_wallet_handle: i32, schema_json: &str) -> String {
        let schema_request = ledger::build_schema_request(issuer_did, schema_json).unwrap();
        ledger::sign_and_submit_request(self.pool_handle, issuer_wallet_handle, issuer_did, &schema_request).unwrap()
    }

    pub fn get_schema(&self, did: Option<&str>, schema_id: &str) -> (String, String){
        let get_schema_request = ledger::build_get_schema_request(did, schema_id).unwrap();
        let get_schema_response = ledger::submit_request(self.pool_handle, &get_schema_request).unwrap();
        ledger::parse_get_schema_response(&get_schema_response).unwrap()

    }

    pub fn submit_cred_def(&self, issuer_did: &str, issuer_wallet_handle: i32, cred_def_json: &str) -> String {
        let cred_def_request = ledger::build_cred_def_txn(issuer_did, cred_def_json).unwrap();
        ledger::sign_and_submit_request(self.pool_handle, issuer_wallet_handle, issuer_did, &cred_def_request).unwrap()

    }

    pub fn get_cred_def(&self, did : Option<&str>, cred_def_id : &str ) -> (String, String) /* (cred_def_id, cred_def_json) */{
        let get_cred_def_request = ledger::build_get_cred_def_request(did, cred_def_id).unwrap();
        let get_cred_def_response = ledger::submit_request(self.pool_handle, &get_cred_def_request).unwrap();
        ledger::parse_get_cred_def_response(&get_cred_def_response).unwrap()
    }

    pub fn submit_revoc_reg_def(&self, issuer_did: &str, issuer_wallet_handle: i32, rev_reg_def_json : &str) -> String {
        let rev_reg_def_request = ledger::build_revoc_reg_def_request(issuer_did, rev_reg_def_json).unwrap();
        ledger::sign_and_submit_request(self.pool_handle, issuer_wallet_handle, issuer_did, &rev_reg_def_request).unwrap()
    }

    pub fn get_revoc_reg_def(&self, did: Option<&str>, revoc_reg_def_id : &str) -> (String, String) /* revoc_reg_def_id, revo_reg_def_json */ {
        let get_rev_reg_def_request = ledger::build_get_revoc_reg_def_request(did, &revoc_reg_def_id).unwrap();
        let get_rev_reg_def_response = ledger::submit_request(self.pool_handle, &get_rev_reg_def_request).unwrap();
        ledger::parse_get_revoc_reg_def_response(&get_rev_reg_def_response).unwrap()
    }

    pub fn submit_revoc_reg_entry(&self, issuer_did: &str, issuer_wallet_handle: i32, rev_reg_id : &str, rev_reg_entry_json: &str) -> String{
        let rev_reg_entry_request =
            ledger::build_revoc_reg_entry_request(issuer_did, rev_reg_id, REVOC_REG_TYPE, rev_reg_entry_json).unwrap();
        ledger::sign_and_submit_request(self.pool_handle, issuer_wallet_handle, issuer_did, &rev_reg_entry_request).unwrap()

    }

    pub fn get_revoc_reg_delta(&self, did: Option<&str>, revoc_reg_def_id : &str, from : Option<u64>, to : u64) -> (String,String, u64) /* rev_reg_id, revoc_reg_delta_json, timestamp */ {
        let get_rev_reg_delta_request = ledger::build_get_revoc_reg_delta_request(did, revoc_reg_def_id, from, to).unwrap();
        let get_rev_reg_delta_response = ledger::submit_request(self.pool_handle, &get_rev_reg_delta_request).unwrap();
        ledger::parse_get_revoc_reg_delta_response(&get_rev_reg_delta_response).unwrap()
    }


}


impl Issuer {


    pub fn new(pool: &Pool) -> Issuer{

        let (wallet_handle, _wallet_config) = wallet::create_and_open_default_wallet(format!("wallet_for_pool_{}", pool.pool_handle).borrow()).unwrap();
        Issuer {
            // Issuer creates wallet, gets wallet handle
            issuer_wallet_handle: wallet_handle,

            // Issuer create DID
            issuer_did: did::create_store_and_publish_my_did_from_trustee(wallet_handle, pool.pool_handle).unwrap().0,

            schema_id: String::new(),
            rev_reg_id : String::new(),
            cred_def_id : String::new(),

            issuance_type : String::new(),
            tails_writer_config : anoncreds::tails_writer_config()

        }
    }

    // creates schema , credential definition and revocation registry
    pub fn create_initial_ledger_state(& mut self, pool : &Pool, revoc_registry_config : &str)
    {

        let revoc_reg_config_value : Value = serde_json::from_str(revoc_registry_config).unwrap();
        self.issuance_type = String::from(revoc_reg_config_value.as_object().unwrap().get("issuance_type").unwrap().as_str().unwrap());

        // Issuer creates Schema
        let (schema_id, schema_json) = anoncreds::issuer_create_schema(&self.issuer_did,
                                                                       GVT_SCHEMA_NAME,
                                                                       SCHEMA_VERSION,
                                                                       GVT_SCHEMA_ATTRIBUTES).unwrap();

        // !!IMPORTANT!!
        // It is important Post and Get Schema from Ledger and parse it to get the correct Schema JSON and correspondent it seq_no in Ledger
        // After that we can create CredentialDefinition for received Schema(not for result of indy_issuer_create_schema)
        let _schema_response = pool.submit_schema(&self.issuer_did, self.issuer_wallet_handle,&schema_json);

        ::std::thread::sleep(::std::time::Duration::from_secs(2));

        // Issuer gets Schema from Ledger
        let (_ , schema_json) = pool.get_schema(Some(&self.issuer_did),&schema_id);

        self.schema_id = schema_id;

        // Issuer creates CredentialDefinition
        let (cred_def_id, cred_def_json) = anoncreds::issuer_create_credential_definition(self.issuer_wallet_handle,
                                                                                          &self.issuer_did,
                                                                                          &schema_json,
                                                                                          TAG_1,
                                                                                          None,
                                                                                          Some(&anoncreds::revocation_cred_def_config())).unwrap();

        // Issuer post CredentialDefinition to Ledger
        pool.submit_cred_def(&self.issuer_did,self.issuer_wallet_handle,&cred_def_json);

        self.cred_def_id = cred_def_id;

        // Issuer creates RevocationRegistry
        let tails_writer_handle = blob_storage::open_writer("default", &self.tails_writer_config).unwrap();

        let (rev_reg_id, rev_reg_def_json, rev_reg_entry_json) =
            anoncreds::issuer_create_and_store_revoc_reg(self.issuer_wallet_handle,
                                                         &self.issuer_did,
                                                         None,
                                                         TAG_1,
                                                         &self.cred_def_id,
                                                         revoc_registry_config,
                                                         tails_writer_handle).unwrap();

        // Issuer posts RevocationRegistryDefinition to Ledger
        pool.submit_revoc_reg_def(&self.issuer_did, self.issuer_wallet_handle, &rev_reg_def_json);


        self.rev_reg_id = rev_reg_id;

        // Issuer posts RevocationRegistryEntry to Ledger
        pool.submit_revoc_reg_entry(&self.issuer_did,self.issuer_wallet_handle, &self.rev_reg_id, &rev_reg_entry_json);
    }

    pub fn make_credential_offer(&self) -> String
    {
        let cred_offer_json = anoncreds::issuer_create_credential_offer(self.issuer_wallet_handle, &self.cred_def_id).unwrap();
        cred_offer_json
    }

    pub fn issue_credential(&self, pool: &Pool, cred_offer_json: &str, cred_req_json: &str, cred_values_json: &str) -> (String, String, Option<String>)
    {

        // Issuer creates TailsReader
        let blob_storage_reader_handle = blob_storage::open_reader(TYPE, &self.tails_writer_config).unwrap();

        // Issuer creates Credential
        // NOte that  the function returns revoc_reg_delta_json as None in case
        // the revocation registry was created with the strategy ISSUANCE_BY_DEFAULT
        let (cred_json, cred_rev_id, revoc_reg_delta_json) = anoncreds::issuer_create_credential(self.issuer_wallet_handle,
                                                                                                 &cred_offer_json,
                                                                                                 &cred_req_json,
                                                                                                 cred_values_json,
                                                                                                 Some(&self.rev_reg_id),
                                                                                                 Some(blob_storage_reader_handle)).unwrap();


        // Issuer does not have to post rev_reg_delta to ledger in case of the strategy ISSUANCE_BY_DEFAULT
        if &self.issuance_type  == "ISSUANCE_ON_DEMAND" {
            pool.submit_revoc_reg_entry(&self.issuer_did, self.issuer_wallet_handle, &self.rev_reg_id, &revoc_reg_delta_json.clone().unwrap());
        }

        (cred_json, cred_rev_id.unwrap(), revoc_reg_delta_json)
    }

    pub fn revoke_credential(&self, pool : &Pool, cred_rev_id: &str) -> String
    {

        // Issuer creates TailsReader
        let blob_storage_reader_handle = blob_storage::open_reader(TYPE, &self.tails_writer_config).unwrap();

        // Issuer revokes cred_info
        let rev_reg_delta_json = anoncreds::issuer_revoke_credential(self.issuer_wallet_handle, blob_storage_reader_handle, &self.rev_reg_id, &cred_rev_id).unwrap();

        // Issuer post RevocationRegistryDelta to Ledger
        pool.submit_revoc_reg_entry(&self.issuer_did,self.issuer_wallet_handle,&self.rev_reg_id,&rev_reg_delta_json);

        rev_reg_delta_json
    }

    pub fn close(&self)
    {
        wallet::close_wallet(self.issuer_wallet_handle).unwrap();
    }
}


impl Prover
{
    pub fn new(master_secret_id : Option<&str>) -> Prover
    {
        // Prover creates wallet, gets wallet handle
        let (prover_wallet_handle, _prover_wallet_config) = wallet::create_and_open_default_wallet("interactions_prover").unwrap();
        // Prover create DID
        let (prover_did, prover_verkey) = did::create_my_did(prover_wallet_handle, "{}").unwrap();
        // Prover creates Master Secret
        let master_secret_id = master_secret_id.unwrap_or(COMMON_MASTER_SECRET);
        anoncreds::prover_create_master_secret(prover_wallet_handle, master_secret_id).unwrap();

        Prover{ wallet_handle: prover_wallet_handle,
                did: prover_did.clone(),
                verkey: prover_verkey.clone(),
                master_secret_id : String::from(master_secret_id),
                cred_def_id: None,
                cred_req_metadata_json : None
             }
    }


    pub fn make_credential_request(&mut self, pool: &Pool,  cred_offer_json :  &str ) -> String
    {
        // Prover gets CredentialDefinition from Ledger
        let cred_offer: CredentialOffer = serde_json::from_str(&cred_offer_json).unwrap();
        let (cred_def_id, cred_def_json) = pool.get_cred_def(Some(&self.did), &cred_offer.cred_def_id);
        self.cred_def_id = Some(cred_def_id);

        // Prover creates Credential Request
        let (cred_req_json, cred_req_metadata_json) = anoncreds::prover_create_credential_req(self.wallet_handle,
                                                                                              &self.did,
                                                                                              &cred_offer_json,
                                                                                              &cred_def_json,
                                                                                              &self.master_secret_id).unwrap();
        self.cred_req_metadata_json = Some(cred_req_metadata_json);
        cred_req_json
    }


    pub fn store_credentials(&self, pool: &Pool, cred_json: &str, cred_id: &str)
    {
        let credential: Credential = serde_json::from_str(&cred_json).unwrap();

        // Prover gets CredentialDefinition from Ledger
        let (_ , cred_def_json) = pool.get_cred_def(Some(&self.did), &self.cred_def_id.clone().unwrap());

        // Prover gets RevocationRegistryDefinition
        let (_, revoc_reg_def_json) = pool.get_revoc_reg_def(None,&credential.rev_reg_id.unwrap());

        // Prover stores received Credential
        anoncreds::prover_store_credential(self.wallet_handle,
                                           cred_id,
                                           &self.cred_req_metadata_json.clone().unwrap(),
                                           &cred_json,
                                           &cred_def_json,
                                           Some(&revoc_reg_def_json)).unwrap();
    }

    pub fn make_proof(&self, pool : &Pool, proof_request: &str, attr1_referent: &str, from: Option<u64>, to: u64 ) -> String
    {
        
        // Prover searches Credentials for Proof Request
        let search_handle  = anoncreds::prover_search_credentials_for_proof_req(self.wallet_handle, &proof_request, None).unwrap();
        let credentials_list = anoncreds::prover_fetch_next_credentials_for_proof_req(search_handle,attr1_referent,1).unwrap();

        let credentials_list_value : Value = serde_json::from_str(&credentials_list).unwrap();
        // extract first result of the search as Value
        let credentials_first = &credentials_list_value.as_array().unwrap()[0];
        // extract cred_info as Value from the result
        let cred_info_value = credentials_first.as_object().unwrap().get("cred_info").unwrap();

        let cred_info : CredentialInfo = serde_json::from_value(cred_info_value.clone()).unwrap();

        let _ = anoncreds::prover_close_credentials_search_for_proof_req(search_handle).unwrap();

        let schema_id = cred_info.schema_id;
        let cred_def_id = cred_info.cred_def_id;
        assert_eq!(cred_def_id, self.cred_def_id.clone().unwrap());
        let cred_rev_id = cred_info.cred_rev_id.clone().unwrap();
        let rev_reg_id = cred_info.rev_reg_id.clone().unwrap();


        // Prover gets Schema from Ledger
        let (_, schema_json) = pool.get_schema(None, &schema_id);

        // Prover gets CredentialDefinition from Ledger
        let (_ , cred_def_json) = pool.get_cred_def(Some(&self.did), &cred_def_id);

        // Prover gets RevocationRegistryDefinition
        let (_, revoc_reg_def_json) = pool.get_revoc_reg_def(None,&rev_reg_id);

        // Prover gets RevocationRegistryDelta from Ledger
        let (_, revoc_reg_delta_json, timestamp) =  pool.get_revoc_reg_delta(None,&rev_reg_id, from, to);

        // Prover creates RevocationState

        let prover_blob_storage_reader_handle = blob_storage::open_reader(TYPE, &anoncreds::tails_writer_config()).unwrap();
        let rev_state_json = anoncreds::create_revocation_state(prover_blob_storage_reader_handle,
                                                                &revoc_reg_def_json,
                                                                &revoc_reg_delta_json,
                                                                timestamp,
                                                                &cred_rev_id).unwrap();


        let proof_request_value : Value = serde_json::from_str(proof_request).unwrap();
        let requested_predicates = ! proof_request_value.as_object().unwrap().get("requested_predicates").unwrap().as_object().unwrap().is_empty();

        // Prover creates Proof
        let requested_credentials_json = if requested_predicates
            {
            json!({
                "self_attested_attributes": json!({}),
                "requested_attributes": json!({
                    attr1_referent.clone(): json!({ "cred_id": cred_info.referent, "timestamp": timestamp,  "revealed":true })
                }),
                "requested_predicates": json!({
                    "predicate1_referent": json!({ "cred_id": cred_info.referent, "timestamp": timestamp })
                })
            }).to_string()
            } else {
            json!({
                "self_attested_attributes": json!({}),
                "requested_attributes": json!({
                    "attr1_referent": json!({ "cred_id": cred_info.referent, "timestamp": timestamp,  "revealed":true })
                }),
                "requested_predicates": json!({})
            }).to_string()
        };

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

        let proof_json = anoncreds::prover_create_proof(self.wallet_handle,
                                                        &proof_request,
                                                        &requested_credentials_json,
                                                        &self.master_secret_id,
                                                        &schemas_json,
                                                        &cred_defs_json,
                                                        &rev_states_json).unwrap();

        proof_json
    }


    pub fn close(&self)
    {
        wallet::close_wallet(self.wallet_handle).unwrap();
    }
}




impl Verifier{


    pub fn new(proof_request: &String) -> Verifier {
        Verifier{
            proof_request: proof_request.clone()
        }
    }

    pub fn verify_revealed(&self, proof_json : &str, attr_name : &str, attr_value : &str)
    {
        let proof : Proof  = serde_json::from_str(&proof_json).unwrap();

        assert_eq!(attr_value, proof.requested_proof.revealed_attrs.get(attr_name).unwrap().raw)

    }

    pub fn verify(&self, pool: &Pool, proof_json : &str) -> bool
    {

        let proof : Proof  = serde_json::from_str(&proof_json).unwrap();
        assert_eq!(1, proof.identifiers.len());

        let identifier = proof.identifiers[0].clone();

        // Verifier gets Schema from Ledger
        let (schema_id, schema_json) = pool.get_schema(Some(DID_MY1),  &identifier.schema_id );

        // Verifier gets CredentialDefinition from Ledger
        let (cred_def_id, cred_def_json) = pool.get_cred_def(Some(DID_MY1), &identifier.cred_def_id);

        // Verifier gets RevocationRegistryDefinition from Ledger
        let (rev_reg_id, revoc_reg_def_json) = pool.get_revoc_reg_def(Some(DID_MY1), &identifier.rev_reg_id.clone().unwrap());

        // Verifier gets RevocationRegistry from Ledger
        let (_, rev_reg_json, timestamp) =
            pool.get_revoc_reg_delta(Some(DID_MY1), &identifier.rev_reg_id.clone().unwrap(), None, identifier.timestamp.unwrap());

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

        let valid = anoncreds::verifier_verify_proof(&self.proof_request,
                                                     proof_json,
                                                     &schemas_json,
                                                     &cred_defs_json,
                                                     &rev_reg_defs_json,
                                                     &rev_regs_json).unwrap();

        valid
    }
}

#[cfg(feature = "revocation_tests")]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_demand() {
    anoncreds_revocation_interaction_test_one_prover("anoncreds_revocation_interaction_test_issuance_by_demand", r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);
}

#[cfg(feature = "revocation_tests")]
#[cfg(any(feature = "force_full_interaction_tests", not(target_os = "android")))]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_default()
{
    anoncreds_revocation_interaction_test_one_prover("anoncreds_revocation_interaction_test_issuance_by_default", r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_BY_DEFAULT"}"#);
}

// the common function for two previous tests
fn anoncreds_revocation_interaction_test_one_prover(pool_name: &str, revocation_registry_config: &str)
{
    utils::setup(pool_name);

    let pool = Pool::new(pool_name);

    let mut issuer = Issuer::new(&pool);

    let mut prover = Prover::new(None);

    // Issuer publish Prover DID
    pool.submit_nym(&issuer.issuer_did, issuer.issuer_wallet_handle, &prover.did,Some(&prover.verkey));


    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry
    issuer.create_initial_ledger_state(&pool,revocation_registry_config);


    ///////////////////////////////////////////////////////////////////////////////////////////////////////
    // Issuance Credential for Prover

    // Issuer creates Credential Offer
    let cred_offer_json = issuer.make_credential_offer();

    // Prover makes credential request
    let cred_req_json = prover.make_credential_request(&pool,&cred_offer_json);

    // Issuer issues credential
    let (cred_json, cred_rev_id, _revoc_reg_delta_json) = issuer.issue_credential(&pool, &cred_offer_json, &cred_req_json, &anoncreds::gvt_credential_values_json() );

    // Prover stores credentials
    prover.store_credentials(&pool, &cred_json, CREDENTIAL1_ID);


    // Basic check
    let credentials = anoncreds::prover_get_credentials(prover.wallet_handle, &json!({"schema_name": GVT_SCHEMA_NAME}).to_string()).unwrap();
    let credentials: Vec<serde_json::Value> = serde_json::from_str(&credentials).unwrap();
    assert_eq!(credentials.len(), 1);

    /////////////////////////////////////////////////////////////////////////////////////////////////
    // Verifying Prover's Credential
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

    let verifier = Verifier::new(&proof_request);

    let proof_json = prover.make_proof(&pool, &proof_request, "attr1_referent", None, to);


    // Verifier verifies revealed attribute
    verifier.verify_revealed(&proof_json,"attr1_referent","Alex");

    let valid = verifier.verify(&pool,&proof_json);
    assert!(valid);

    /////////////////////////////////////////////////////////////////////////////////////////
    // Issuer revokes cred_rev_id
    let _rev_reg_delta_json = issuer.revoke_credential(&pool, &cred_rev_id );

    // Verifying Prover Credential after Revocation
    thread::sleep(std::time::Duration::from_secs(3));

    let from = to;
    let to = time::get_time().sec as u64;

    let proof_json = prover.make_proof(&pool, &proof_request, "attr1_referent" , Some(from), to);

    let valid = verifier.verify(&pool, &proof_json);
    assert!(!valid);


    issuer.close();
    prover.close();

    pool.close();

    utils::tear_down(pool_name);
}


fn multi_steps_create_revocation_credential(pool : &Pool, issuer: &Issuer, prover : &mut Prover, cred_values_json : &str , cred_id: &str) -> (String, Option<String>)
{
    // Issuer creates Credential Offer
    let cred_offer_json = issuer.make_credential_offer();

    // Prover makes credential request
    let cred_req_json = prover.make_credential_request(&pool,&cred_offer_json);

    // Issuer issues credential
    let (cred_json, cred_rev_id, revoc_reg_delta_json) = issuer.issue_credential(&pool, &cred_offer_json, &cred_req_json, cred_values_json );

    // Prover stores credentials
    prover.store_credentials(&pool, &cred_json, cred_id);

    (cred_rev_id, revoc_reg_delta_json)
}


#[cfg(feature = "revocation_tests")]
#[cfg(any(feature = "force_full_interaction_tests", not(target_os = "android")))]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_demand_three_credentials_post_entry_three_times_proving_first() {
    utils::setup("anoncreds_4711");

    let pool = Pool::new("anoncreds_4711");

    let mut issuer = Issuer::new(&pool);

    let mut prover1 = Prover::new(Some("prover1_master_secret"));
    let mut prover2 = Prover::new(Some("prover2_master_secret"));
    let mut prover3 = Prover::new(Some("prover3_master_secret"));


    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry
    issuer.create_initial_ledger_state(&pool,r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);



    /*ISSUANCE CREDENTIAL FOR PROVER1*/

    let (_prover1_cred_rev_id, _prover1_revoc_reg_delta1_json) =
        multi_steps_create_revocation_credential(&pool, &issuer, &mut prover1, &anoncreds::gvt_credential_values_json(),CREDENTIAL1_ID);


    /*ISSUANCE CREDENTIAL FOR PROVER2*/

    let (_prover2_cred_rev_id, _prover2_revoc_reg_delta1_json) =
        multi_steps_create_revocation_credential(&pool, &issuer, &mut prover2, &anoncreds::gvt2_credential_values_json(),CREDENTIAL2_ID);


    /*ISSUANCE CREDENTIAL FOR PROVER3*/

    let (_prover3_cred_rev_id, _prover3_revoc_reg_delta1_json) =
        multi_steps_create_revocation_credential(&pool, &issuer, &mut prover3, &anoncreds::gvt3_credential_values_json(),CREDENTIAL3_ID);


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

    let verifier = Verifier::new(&proof_request);

    let proof_json = prover1.make_proof(&pool, &proof_request, "attr1_referent", None, to);


    // Verifier verifies revealed attribute
    verifier.verify_revealed(&proof_json,"attr1_referent","Alex");

    let valid = verifier.verify(&pool,&proof_json);
    assert!(valid);


    issuer.close();
    prover1.close();
    prover2.close();
    prover3.close();

    pool.close();

    utils::tear_down("anoncreds_4711");
}

#[cfg(feature = "revocation_tests")]
#[cfg(any(feature = "force_full_interaction_tests", not(target_os = "android")))]
#[test]
fn anoncreds_revocation_interaction_test_issuance_by_demand_three_credentials_post_common_entry_proving_all() {
    utils::setup("aritibdtcpcepa");

    let pool = Pool::new("aritibdtcpcepa");

    let mut issuer = Issuer::new(&pool);

    let mut prover1 = Prover::new(Some("prover1_master_secret"));
    let mut prover2 = Prover::new(Some("prover2_master_secret"));
    let mut prover3 = Prover::new(Some("prover3_master_secret"));


    // ISSUER post to Ledger Schema, CredentialDefinition, RevocationRegistry
    issuer.create_initial_ledger_state(&pool,r#"{"max_cred_num":5, "issuance_type":"ISSUANCE_ON_DEMAND"}"#);



    /*ISSUANCE CREDENTIAL FOR PROVER1*/

    let (_prover1_cred_rev_id, revoc_reg_delta1_json) =  multi_steps_create_revocation_credential(&pool, &issuer, &mut prover1, &anoncreds::gvt_credential_values_json(),CREDENTIAL1_ID);
    let revoc_reg_delta1_json = revoc_reg_delta1_json.unwrap();

    /*ISSUANCE CREDENTIAL FOR PROVER2*/

    let (_prover2_cred_rev_id, revoc_reg_delta2_json) =  multi_steps_create_revocation_credential(&pool, &issuer, &mut prover2, &anoncreds::gvt2_credential_values_json(),CREDENTIAL2_ID);
    let revoc_reg_delta2_json = revoc_reg_delta2_json.unwrap();

    // Issuer merge Revocation Registry Deltas
    let revoc_reg_delta_json = anoncreds::issuer_merge_revocation_registry_deltas(&revoc_reg_delta1_json, &revoc_reg_delta2_json).unwrap();

    /*ISSUANCE CREDENTIAL FOR PROVER3*/

    let (_prover3_cred_rev_id, revoc_reg_delta3_json) =  multi_steps_create_revocation_credential(&pool, &issuer, &mut prover3, &anoncreds::gvt3_credential_values_json(),CREDENTIAL3_ID);
    let revoc_reg_delta3_json = revoc_reg_delta3_json.unwrap();

    // Issuer merge Revocation Registry Deltas
    let _revoc_reg_delta_json = anoncreds::issuer_merge_revocation_registry_deltas(&revoc_reg_delta_json, &revoc_reg_delta3_json).unwrap();


    // TODO: test if the issuer can submit one delta instead of multiple deltas consequently
//    let rev_reg_entry_request =
//        ledger::build_revoc_reg_entry_request(&issuer_did, &rev_reg_id, REVOC_REG_TYPE, &revoc_reg_delta_json).unwrap();
//    ledger::sign_and_submit_request(pool_handle, issuer_wallet_handle, &issuer_did, &rev_reg_entry_request).unwrap();

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

    let verifier = Verifier::new(&proof_request);

    let proof_json = prover1.make_proof(&pool, &proof_request, "attr1_referent", None, to);

    verifier.verify_revealed(&proof_json,"attr1_referent", "Alex");
    let valid = verifier.verify(&pool, &proof_json);
    assert!(valid);

    // Verifying Prover2 Credential
    let proof_json = prover2.make_proof(&pool, &proof_request, "attr1_referent", None, to);

    // Verifier verifies proof from Prover2
    verifier.verify_revealed(&proof_json,"attr1_referent", "Alexander");
    let valid = verifier.verify(&pool, &proof_json);
    assert!(valid);


    // Verifying Prover3 Credential
    let proof_json = prover3.make_proof(&pool, &proof_request, "attr1_referent", None, to);

    // Verifier verifies proof from Prover2
    verifier.verify_revealed(&proof_json,"attr1_referent", "Artem");
    let valid = verifier.verify(&pool, &proof_json);
    assert!(valid);

    issuer.close();
    prover1.close();
    prover2.close();
    prover3.close();

    pool.close();


    utils::tear_down("aritibdtcpcepa");
}
