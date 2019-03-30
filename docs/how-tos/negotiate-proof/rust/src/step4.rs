println!("19. Prover creates Proof for Proof Request");
let creds_for_proof_request = serde_json::from_str::<serde_json::Value>(&creds_for_proof_request_json).unwrap();
let creds_for_attr_1 = &creds_for_proof_request["attrs"]["attr1_referent"];
let credential = &creds_for_attr_1[0]["cred_info"];

let requested_credentials_json = json!({
    "self_attested_attributes": {},
    "requested_attributes": {
        "attr1_referent": {
            "cred_id": credential["referent"].as_str().unwrap(),
            "revealed": true
        }
    },
    "requested_predicates":{
        "predicate1_referent":{
            "cred_id": credential["referent"].as_str().unwrap(),
        }
    }
});
println!("Requested Credentials for Proving: {}", requested_credentials_json.to_string());

let schemas_json = json!({
    schema_id.as_str(): serde_json::from_str::<serde_json::Value>(&schema_json).unwrap()
}).to_string();
let credential_defs_json = json!({
    cred_def_id.as_str(): serde_json::from_str::<serde_json::Value>(&cred_def_json).unwrap()
}).to_string();
let rev_states_json = json!({}).to_string();

let proof_json = anoncreds::prover_create_proof(prover_wallet_handle,
                                      &proof_req_json.to_string(),
                                      &requested_credentials_json.to_string(),
                                      &master_secret_name,
                                      &schemas_json,
                                      &credential_defs_json,
                                      &rev_states_json).wait().unwrap();
let proof = serde_json::from_str::<serde_json::Value>(&proof_json).unwrap();
assert_eq!("Alex", proof["requested_proof"]["revealed_attrs"]["attr1_referent"]["raw"].as_str().unwrap());
