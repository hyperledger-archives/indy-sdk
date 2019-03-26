println!("9. Create Schema and Build the SCHEMA request to add new schema to the ledger as a Steward");
let name = "gvt";
let version = "1.0";
let attributes = r#"["age", "sex", "height", "name"]"#;
let (_schema_id, schema_json) = anoncreds::issuer_create_schema(&steward_did, name, version, attributes).wait().unwrap();

let build_schema_request: String = ledger::build_schema_request(&steward_did, &schema_json).wait().unwrap();

println!("10. Sending the SCHEMA request to the ledger");
let _signed_schema_request_response = ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_schema_request).wait().unwrap();
