println!("9. Create Schema and Build the SCHEMA request to add new schema to the ledger as a Steward");
let name = "gvt";
let version = "1.0";
let attributes = r#"["age", "sex", "height", "name"]"#;
let (_schema_id, schema_json) = Issuer::create_schema(&steward_did, name, version, attributes).unwrap();

let build_schema_request: String = Ledger::build_schema_request(&steward_did, &schema_json).unwrap();

println!("10. Sending the SCHEMA request to the ledger");
let _signed_schema_request_response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_schema_request).unwrap();
