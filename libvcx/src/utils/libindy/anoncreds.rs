extern crate libc;

use settings;
use utils::constants::{ LIBINDY_CRED_OFFER };
use utils::libindy::mock_libindy_rc;
use utils::libindy::wallet::get_wallet_handle;
use utils::timeout::TimeoutUtils;
use utils::libindy::error_codes::map_rust_indy_sdk_error_code;
use indy::anoncreds::{ Verifier, Prover, Issuer };

pub fn libindy_verifier_verify_proof(proof_req_json: &str,
                                     proof_json: &str,
                                     schemas_json: &str,
                                     credential_defs_json: &str,
                                     rev_reg_defs_json: &str,
                                     rev_regs_json: &str)  -> Result<bool, u32> {

    Verifier::verify_proof_timeout(proof_req_json,
                                   proof_json,
                                   schemas_json,
                                   credential_defs_json,
                                   rev_reg_defs_json,
                                   rev_regs_json,
                                   TimeoutUtils::long_timeout())
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_create_and_store_credential_def(issuer_did: &str,
                                               schema_json: &str,
                                               tag: &str,
                                               sig_type: Option<&str>,
                                               config_json: &str)  -> Result<(String, String), u32>  {

    Issuer::create_and_store_credential_def(get_wallet_handle(),
                                            issuer_did,
                                            schema_json,
                                            tag,
                                            sig_type,
                                            config_json)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_issuer_create_credential_offer(cred_def_id: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(LIBINDY_CRED_OFFER.to_string());
    }
    Issuer::create_credential_offer(get_wallet_handle(),
                                    cred_def_id)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_issuer_create_credential(cred_offer_json: &str,
                                        cred_req_json: &str,
                                        cred_values_json: &str,
                                        rev_reg_id: Option<&str>,
                                        blob_storage_reader_handle: Option<i32>) -> Result<(String, Option<String>, Option<String>), u32>{

    let blob_storage_reader_handle = blob_storage_reader_handle.unwrap_or(-1);

    Issuer::create_credential(get_wallet_handle(),
                              cred_offer_json,
                              cred_req_json,
                              cred_values_json,
                              rev_reg_id,
                              blob_storage_reader_handle)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_create_proof(proof_req_json: &str,
                                   requested_credentials_json: &str,
                                   master_secret_id: &str,
                                   schemas_json: &str,
                                   credential_defs_json: &str,
                                   revoc_states_json: Option<&str>) -> Result<String, u32> {
    let revoc_states_json = revoc_states_json.unwrap_or("{}");
    Prover::create_proof(get_wallet_handle(),
                         proof_req_json,
                         requested_credentials_json,
                         master_secret_id,
                         schemas_json,
                         credential_defs_json,
                         revoc_states_json)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_get_credentials_for_proof_req(proof_req: &str) -> Result<String, u32> {

    Prover::get_credentials_for_proof_req(get_wallet_handle(),
                                          proof_req)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_create_credential_req(prover_did: &str,
                                            credential_offer_json: &str,
                                            credential_def_json: &str) -> Result<(String, String), u32> {
    if settings::test_indy_mode_enabled() { return Ok((::utils::constants::CREDENTIAL_REQ_STRING.to_owned(), String::new())); }

    let master_secret_name = settings::DEFAULT_LINK_SECRET_ALIAS;
    Prover::create_credential_req(get_wallet_handle(),
                                  prover_did,
                                  credential_offer_json,
                                  credential_def_json,
                                  master_secret_name)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_store_credential(cred_id: Option<&str>,
                                       cred_req_meta: &str,
                                       cred_json: &str,
                                       cred_def_json: &str,
                                       rev_reg_def_json: Option<&str>) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok("cred_id".to_string()); }

    Prover::store_credential(get_wallet_handle(),
                             cred_id,
                             cred_req_meta,
                             cred_json,
                             cred_def_json,
                             rev_reg_def_json)
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_prover_create_master_secret(master_secret_id: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() { return Ok(settings::DEFAULT_LINK_SECRET_ALIAS.to_string()); }

    Prover::create_master_secret(get_wallet_handle(),
                                 Some(master_secret_id))
        .map_err(map_rust_indy_sdk_error_code)
}

pub fn libindy_issuer_create_schema(issuer_did: &str,
                                    name: &str,
                                    version: &str,
                                    attrs: &str) -> Result<(String, String), u32>{

    Issuer::create_schema(issuer_did,
                          name,
                          version,
                          attrs)
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

    pub fn create_schema() -> (String, String) {
        let data = r#"["address1","address2","zip","city","state"]"#.to_string();
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

    pub fn create_and_write_test_schema() -> (String, String) {
        let (schema_id, schema_json) = create_schema();
        let req = create_schema_req(&schema_json);
        write_schema(&req);
        thread::sleep(Duration::from_millis(1000));
        (schema_id, schema_json)
    }

    pub fn create_and_store_credential_def() -> (String, String, String, String) {
        /* create schema */
        let (schema_id, schema_json) = create_and_write_test_schema();

        let name: String = rand::thread_rng().gen_ascii_chars().take(25).collect::<String>();
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();

        /* create cred-def */
        let handle = ::credential_def::create_new_credentialdef("1".to_string(),
                                                                name,
                                                                institution_did.clone(),
                                                                schema_id.clone(),
                                                                "tag1".to_string(),
                                                                r#"{"support_revocation":false}"#.to_string()).unwrap();

        thread::sleep(Duration::from_millis(1000));
        let cred_def_id = ::credential_def::get_cred_def_id(handle).unwrap();
        thread::sleep(Duration::from_millis(1000));
        let (_, cred_def_json) = ::credential_def::retrieve_credential_def(&cred_def_id).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json)
    }

    pub fn create_credential_offer() -> (String, String, String, String, String) {
        let (schema_id, schema_json, cred_def_id, cred_def_json) = create_and_store_credential_def();

        let offer = ::utils::libindy::anoncreds::libindy_issuer_create_credential_offer(&cred_def_id).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer)
    }

    pub fn create_credential_req() -> (String, String, String, String, String, String, String) {
        let (schema_id, schema_json, cred_def_id, cred_def_json, offer) = create_credential_offer();
        let institution_did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (req, req_meta) = ::utils::libindy::anoncreds::libindy_prover_create_credential_req(&institution_did, &offer, &cred_def_json).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta)
    }

    pub fn create_and_store_credential() -> (String, String, String, String, String, String, String, String) {

        let (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta) = create_credential_req();

        /* create cred */
        let credential_data = r#"{"address1": ["123 Main St"], "address2": ["Suite 3"], "city": ["Draper"], "state": ["UT"], "zip": ["84000"]}"#;
        let encoded_attributes = ::issuer_credential::encode_attributes(&credential_data).unwrap();
        let (cred, _, _) = ::utils::libindy::anoncreds::libindy_issuer_create_credential(&offer, &req, &encoded_attributes, None, None).unwrap();
        /* store cred */
        let cred_id = ::utils::libindy::anoncreds::libindy_prover_store_credential(None, &req_meta, &cred, &cred_def_json, None).unwrap();
        (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, cred_id)
    }

    pub fn create_proof() -> (String, String, String, String) {
        let did = settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap();
        let (schema_id, schema_json, cred_def_id, cred_def_json, offer, req, req_meta, cred_id) = create_and_store_credential();

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

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_prover_verify_proof() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_verify_proof";
        ::utils::devsetup::tests::setup_ledger_env(wallet_name);

        let (schemas, cred_defs, proof_req, proof) = create_proof();

        let result = libindy_verifier_verify_proof(
            &proof_req,
            &proof,
            &schemas,
            &cred_defs,
            "{}",
            "{}",
        );

        ::utils::devsetup::tests::cleanup_dev_env(wallet_name);
        assert!(result.is_ok());
        let proof_validation = result.unwrap();
        assert!(proof_validation, true);
    }
}
