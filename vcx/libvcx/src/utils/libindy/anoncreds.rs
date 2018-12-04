extern crate libc;

use futures::Future;
use serde_json;
use serde_json::{ map::Map, Value};
use settings;
use utils::constants::{ LIBINDY_CRED_OFFER, REQUESTED_ATTRIBUTES, ATTRS, REV_STATE_JSON};
use utils::error::{ INVALID_PROOF_REQUEST, INVALID_ATTRIBUTES_STRUCTURE, INVALID_CONFIGURATION } ;
use utils::libindy::{ error_codes::map_rust_indy_sdk_error_code, mock_libindy_rc, wallet::get_wallet_handle };
use indy::anoncreds;
use indy::blob_storage;

pub fn libindy_verifier_verify_proof(proof_req_json: &str,
                                     proof_json: &str,
                                     schemas_json: &str,
                                     credential_defs_json: &str,
                                     rev_reg_defs_json: &str,
                                     rev_regs_json: &str)  -> Result<bool, u32> {

    //TODO there was timeout here (before future-based Rust wrapper)
    anoncreds::verifier_verify_proof(proof_req_json,
                                     proof_json,
                                     schemas_json,
                                     credential_defs_json,
                                     rev_reg_defs_json,
                                     rev_regs_json)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_create_and_store_revoc_reg(issuer_did: &str, cred_def_id: &str, tails_path: &str, max_creds: u32) -> Result<(String, String, String), u32> {
    trace!("creating revocation: {}, {}, {}", cred_def_id, tails_path, max_creds);
    let tails_config = json!({"base_dir": tails_path,"uri_pattern": ""}).to_string();
    let writer = blob_storage::open_writer("default", &tails_config.to_string())
        .wait()
        .map_err(|ec|map_rust_indy_sdk_error_code(ec))?;
    let revoc_config = json!({"max_cred_num": max_creds,"issuance_type": "ISSUANCE_BY_DEFAULT"}).to_string();

    anoncreds::issuer_create_and_store_revoc_reg(get_wallet_handle(), issuer_did, None, "tag1", cred_def_id, &revoc_config, writer)
        .wait()
        .map_err(|ec|map_rust_indy_sdk_error_code(ec))
}

pub fn libindy_create_and_store_credential_def(issuer_did: &str,
                                               schema_json: &str,
                                               tag: &str,
                                               sig_type: Option<&str>,
                                               config_json: &str)  -> Result<(String, String), u32>  {

    anoncreds::issuer_create_and_store_credential_def(get_wallet_handle(),
                                                      issuer_did,
                                                      schema_json,
                                                      tag,
                                                      sig_type,
                                                      config_json)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_issuer_create_credential_offer(cred_def_id: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(LIBINDY_CRED_OFFER.to_string());
    }
    anoncreds::issuer_create_credential_offer(get_wallet_handle(),
                                    cred_def_id)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_issuer_create_credential(cred_offer_json: &str,
                                        cred_req_json: &str,
                                        cred_values_json: &str,
                                        rev_reg_id: Option<String>,
                                        tails_file: Option<String>) -> Result<(String, Option<String>, Option<String>), u32>{

    let revocation = rev_reg_id.as_ref().map(String::as_str);

    let blob_handle = match tails_file {
        Some(x) => {
            let tails_config = json!({"base_dir": x,"uri_pattern": ""}).to_string();
            blob_storage::open_reader("default", &tails_config.to_string())
                .wait()
                .map_err(map_rust_indy_sdk_error_code)?
        },
        None => -1,
    };

    anoncreds::issuer_create_credential(get_wallet_handle(),
                                        cred_offer_json,
                                        cred_req_json,
                                        cred_values_json,
                                        revocation,
                                        blob_handle)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_create_proof(proof_req_json: &str,
                                   requested_credentials_json: &str,
                                   master_secret_id: &str,
                                   schemas_json: &str,
                                   credential_defs_json: &str,
                                   revoc_states_json: Option<&str>) -> Result<String, u32> {
    let revoc_states_json = revoc_states_json.unwrap_or("{}");
    anoncreds::prover_create_proof(get_wallet_handle(),
                                   proof_req_json,
                                   requested_credentials_json,
                                   master_secret_id,
                                   schemas_json,
                                   credential_defs_json,
                                   revoc_states_json)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

fn fetch_credentials(search_handle: i32, requested_attributes: Map<String, Value>) -> Result<String, u32> {
    let mut v: Value = json!({});
    for item_referent in requested_attributes.keys().into_iter() {
        v[ATTRS][item_referent] = serde_json::from_str(&anoncreds::prover_fetch_credentials_for_proof_req(search_handle, item_referent, 100).wait()
            .map_err(map_rust_indy_sdk_error_code)?)
            .map_err(|_| {
                error!("Invalid Json Parsing of Object Returned from Libindy. Did Libindy change its structure?");
                INVALID_CONFIGURATION.code_num
            })?
    }
    Ok(v.to_string())
}

fn close_search_handle(search_handle: i32) -> Result<(), u32> {
    anoncreds::prover_close_credentials_search_for_proof_req(search_handle).wait().map_err(|ec| {
        error!("Error closing search handle");
        map_rust_indy_sdk_error_code(ec)
    })
}

pub fn libindy_prover_get_credentials_for_proof_req(proof_req: &str) -> Result<String, u32> {
    let wallet_handle = get_wallet_handle();

    // this may be too redundant since Prover::search_credentials will validate the proof reqeuest already.
    let proof_request_json:Map<String, Value> = serde_json::from_str(proof_req).map_err(|_| INVALID_PROOF_REQUEST.code_num)?;

    // since the search_credentials_for_proof request validates that the proof_req is properly structured, this get()
    // fn should never fail, unless libindy changes their formats.
    let requested_attributes:Option<Map<String,Value>> = proof_request_json.get(REQUESTED_ATTRIBUTES).and_then(|v| {
        serde_json::from_value(v.clone()).map_err(|_| {
            error!("Invalid Json Parsing of Requested Attributes Retrieved From Libindy. Did Libindy change its structure?");
        }).ok()
    });

    match requested_attributes {
        Some(attrs) => {
            let search_handle = anoncreds::prover_search_credentials_for_proof_req(wallet_handle, proof_req, None)
                .wait()
                .map_err(|ec| {
                error!("Opening Indy Search for Credentials Failed");
                map_rust_indy_sdk_error_code(ec)
            })?;
            let creds: String = fetch_credentials(search_handle, attrs)?;
            // should an error on closing a search handle throw an error, or just a warning?
            // for now we're are just outputting to the user that there is an issue, and continuing on.
            let _ = close_search_handle(search_handle);
            Ok(creds)
        },
        None => {
            Err(INVALID_ATTRIBUTES_STRUCTURE.code_num)
        }
    }

}

pub fn libindy_prover_create_credential_req(prover_did: &str,
                                            credential_offer_json: &str,
                                            credential_def_json: &str) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() { return Ok((::utils::constants::CREDENTIAL_REQ_STRING.to_owned(), String::new())); }

    let master_secret_name = settings::DEFAULT_LINK_SECRET_ALIAS;
    anoncreds::prover_create_credential_req(get_wallet_handle(),
                                  prover_did,
                                  credential_offer_json,
                                  credential_def_json,
                                  master_secret_name)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_create_revocation_state(rev_reg_def_json: &str, rev_reg_delta_json: &str, cred_rev_id: &str, tails_file: &str) ->  Result<String,  u32> {
    if settings::test_indy_mode_enabled() { return Ok(REV_STATE_JSON.to_string()); }
    let tails_config = json!({"base_dir": tails_file,"uri_pattern": ""}).to_string();
    let blob_handle = blob_storage::open_reader("default", &tails_config.to_string())
        .wait()
        .map_err(|ec|map_rust_indy_sdk_error_code(ec))?;

    anoncreds::create_revocation_state(blob_handle, rev_reg_def_json,  rev_reg_delta_json, 100, cred_rev_id)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_update_revocation_state(rev_reg_def_json: &str, rev_state_json: &str, rev_reg_delta_json: &str, cred_rev_id: &str, tails_file: &str) ->  Result<String,  u32> {
    if settings::test_indy_mode_enabled() { return Ok(REV_STATE_JSON.to_string()); }
    let tails_config = json!({"base_dir": tails_file,"uri_pattern": ""}).to_string();
    let blob_handle = blob_storage::open_reader("default", &tails_config.to_string())
        .wait()
        .map_err(|ec|map_rust_indy_sdk_error_code(ec))?;

    anoncreds::update_revocation_state(blob_handle, rev_state_json, rev_reg_def_json,  rev_reg_delta_json, 100, cred_rev_id)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_store_credential(cred_id: Option<&str>,
                                       cred_req_meta: &str,
                                       cred_json: &str,
                                       cred_def_json: &str,
                                       rev_reg_def_json: Option<String>) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok("cred_id".to_string()); }

    let revocation = rev_reg_def_json.as_ref().map(String::as_str);

    anoncreds::prover_store_credential(get_wallet_handle(),
                             cred_id,
                             cred_req_meta,
                             cred_json,
                             cred_def_json,
                             revocation)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_create_master_secret(master_secret_id: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(settings::DEFAULT_LINK_SECRET_ALIAS.to_string()); }

    anoncreds::prover_create_master_secret(get_wallet_handle(),
                                 Some(master_secret_id))
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_issuer_create_schema(issuer_did: &str,
                                    name: &str,
                                    version: &str,
                                    attrs: &str) -> Result<(String, String), u32>{

    anoncreds::issuer_create_schema(issuer_did,
                          name,
                          version,
                          attrs)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_issuer_revoke_credential(tails_file: &str, rev_reg_id: &str, cred_rev_id: &str) -> Result<String, u32> {

    let tails_config = json!({"base_dir": tails_file,"uri_pattern": ""}).to_string();
    let blob_handle = blob_storage::open_reader("default", &tails_config)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)?;

    anoncreds::issuer_revoke_credential(get_wallet_handle(), blob_handle, rev_reg_id, cred_rev_id)
        .wait()
        .map_err(map_rust_indy_sdk_error_code)
}


#[cfg(test)]
pub mod tests {
    use super::*;
    extern crate serde_json;
    extern crate rand;
    use rand::Rng;
    use settings;
    use utils::constants::*;
    use std::thread;
    use std::time::Duration;

    pub fn create_schema(attr_list: &str) -> (String, String) {
        let data = attr_list.to_string();
        let schema_name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let schema_version: String = format!("{}.{}", rand::thread_rng().gen::<u32>().to_string(),
                                             rand::thread_rng().gen::<u32>().to_string());
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        libindy_issuer_create_schema(&institution_did, &schema_name, &schema_version, &data).unwrap()
    }

    pub fn create_schema_req(schema_json: &str) -> String {
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        ::utils::libindy::ledger::libindy_build_schema_request(&institution_did, schema_json).unwrap()
    }

    pub fn write_schema(request: &str) {
        let (payment_info, response) = ::utils::libindy::payments::pay_for_txn(&request, SCHEMA_TXN_TYPE).unwrap();
    }

    pub fn create_and_write_test_schema(attr_list: &str) -> (String, String) {
        let (schema_id, schema_json) = create_schema(attr_list);
        let req = create_schema_req(&schema_json);
        write_schema(&req);
        thread::sleep(Duration::from_millis(1000));
        (schema_id, schema_json)
    }

    pub fn create_and_store_credential_def(attr_list: &str, support_rev: bool) -> (String, String, String, String, u32, Option<String>) {
        /* create schema */
        let (schema_id, schema_json) = create_and_write_test_schema(attr_list);

        let name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        /* create cred-def */
        let mut revocation_details = json!({"support_revocation":support_rev});
        if support_rev {
            revocation_details["tails_file"] = json!(TEST_TAILS_FILE);
            revocation_details["max_creds"] = json!(10);
        }
        let handle = ::credential_def::create_new_credentialdef("1".to_string(),
                                                                name,
                                                                institution_did.clone(),
                                                                schema_id.clone(),
                                                                "tag1".to_string(),
                                                                revocation_details.to_string()).unwrap();

        thread::sleep(Duration::from_millis(1000));
        let cred_def_id = ::credential_def::get_cred_def_id(handle).unwrap();
        thread::sleep(Duration::from_millis(1000));
        let (_, cred_def_json) = ::utils::libindy::ledger::get_cred_def_json(&cred_def_id).unwrap();
        let rev_reg_id = ::credential_def::get_rev_reg_id(handle).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, handle, rev_reg_id)
    }

    pub fn create_credential_offer(attr_list: &str, revocation: bool) -> (String, String, String, String, String, Option<String>) {
        let (schema_id, schema_json, cred_def_id, cred_def_json, _, rev_reg_id) = create_and_store_credential_def(attr_list, revocation);

        let offer = ::utils::libindy::anoncreds::libindy_issuer_create_credential_offer(&cred_def_id).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer, rev_reg_id)
    }

    pub fn create_credential_req(attr_list: &str, revocation: bool) -> (String, String, String, String, String, String, String, Option<String>) {
        let (schema_id, schema_json, cred_def_id, cred_def_json, offer, rev_reg_id) = create_credential_offer(attr_list, revocation);
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (req, req_meta) = ::utils::libindy::anoncreds::libindy_prover_create_credential_req(&institution_did, &offer, &cred_def_json).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, rev_reg_id)
    }

    pub fn create_and_store_credential(attr_list: &str, revocation: bool) -> (String, String, String, String, String, String, String, String, Option<String>, Option<String>) {

        let (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, rev_reg_id) = create_credential_req(attr_list, revocation);

        /* create cred */
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let encoded_attributes = ::issuer_credential::encode_attributes(&credential_data).unwrap();
        let (rev_def_json, tails_file) = if revocation {
            let (id, json) = ::utils::libindy::ledger::get_rev_reg_def_json(&rev_reg_id.clone().unwrap()).unwrap();
            (Some(json), Some(TEST_TAILS_FILE.to_string()))

        } else { (None, None) };
        let (cred, cred_rev_id, _) = ::utils::libindy::anoncreds::libindy_issuer_create_credential(&offer, &req, &encoded_attributes, rev_reg_id.clone(), tails_file).unwrap();
        /* store cred */
        let cred_id = ::utils::libindy::anoncreds::libindy_prover_store_credential(None, &req_meta, &cred, &cred_def_json, rev_def_json).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, cred_id, rev_reg_id, cred_rev_id)
    }

    pub fn create_proof() -> (String, String, String, String) {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, cred_id, _, _)
        = create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS, false);

        let proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "zip_2": json!({
                   "name":"zip",
                   "restrictions": [json!({ "issuer_did": did })]
               }),
               "self_attest_3": json!({
                   "name":"self_attest",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();

        let requested_credentials_json = json!({
              "self_attested_attributes":{
                 "self_attest_3": "my_self_attested_val"
              },
              "requested_attributes":{
                 "address1_1": {"cred_id": cred_id, "revealed": true},
                 "zip_2": {"cred_id": cred_id, "revealed": true}
                },
              "requested_predicates":{}
        }).to_string();

        let schema_json: serde_json::Value = serde_json::from_str(&schema_json).unwrap();
        let schemas = json!({
            schema_id: schema_json,
        }).to_string();

        let cred_def_json: serde_json::Value = serde_json::from_str(&cred_def_json).unwrap();
        let cred_defs = json!({
            cred_def_id: cred_def_json,
        }).to_string();

       libindy_prover_get_credentials_for_proof_req(&proof_req).unwrap();

        let proof = libindy_prover_create_proof(
            &proof_req,
            &requested_credentials_json,
            "main",
            &schemas,
            &cred_defs,
            None).unwrap();
        (schemas, cred_defs, proof_req, proof)
    }

    pub fn create_self_attested_proof() -> (String, String) {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        let proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
               }),
               "zip_2": json!({
                   "name":"zip",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();

        let requested_credentials_json = json!({
              "self_attested_attributes":{
                 "address1_1": "my_self_attested_address",
                 "zip_2": "my_self_attested_zip"
              },
              "requested_attributes":{},
              "requested_predicates":{}
        }).to_string();

        let schemas = json!({}).to_string();
        let cred_defs = json!({}).to_string();

       libindy_prover_get_credentials_for_proof_req(&proof_req).unwrap();

        let proof = libindy_prover_create_proof(
            &proof_req,
            &requested_credentials_json,
            "main",
            &schemas,
            &cred_defs,
            None).unwrap();
        (proof_req, proof)
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_prover_verify_proof() {
        init!("ledger");
        let (schemas, cred_defs, proof_req, proof) = create_proof();

        let result = libindy_verifier_verify_proof(
            &proof_req,
            &proof,
            &schemas,
            &cred_defs,
            "{}",
            "{}",
        );

        assert!(result.is_ok());
        let proof_validation = result.unwrap();
        assert!(proof_validation, true);
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn tests_libindy_prover_get_credentials() {
        init!("ledger");
        let proof_req = "{";
        let result = libindy_prover_get_credentials_for_proof_req(&proof_req);
        assert_eq!(result.err(), Some(INVALID_PROOF_REQUEST.code_num));
        let proof_req = json!({
           "nonce":"123432421212",
           "name":"proof_req_1",
           "version":"0.1",
           "requested_attributes": json!({
               "address1_1": json!({
                   "name":"address1",
               }),
               "zip_2": json!({
                   "name":"zip",
               }),
           }),
           "requested_predicates": json!({}),
        }).to_string();
        let result = libindy_prover_get_credentials_for_proof_req(&proof_req);
        let result_malformed_json = libindy_prover_get_credentials_for_proof_req("{}");
        let wallet_handle = get_wallet_handle();
        let proof_req_str:String = serde_json::to_string(&proof_req).unwrap();
        assert!(result.is_ok());
        assert_eq!(result_malformed_json.err(), Some(INVALID_ATTRIBUTES_STRUCTURE.code_num));
    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_revoke_credential(){
        init!("ledger");
        let rc = libindy_issuer_revoke_credential(TEST_TAILS_FILE, "", "");
        assert!(rc.is_err());

        let (_, _, cred_def_id, _, _, _, _, cred_id, rev_reg_id, cred_rev_id)
        = create_and_store_credential(::utils::constants::DEFAULT_SCHEMA_ATTRS, true);

        let rc = ::utils::libindy::anoncreds::libindy_issuer_revoke_credential(TEST_TAILS_FILE, &rev_reg_id.unwrap(), &cred_rev_id.unwrap());
        assert!(rc.is_ok());
    }
}
