println!("9. Generating and storing client DID and Verkey");
let (client_did, _client_verkey) = did::create_and_store_my_did(wallet_handle, &"{}".to_string()).wait().unwrap();

println!("10. Building the GET_NYM request to query Trust Anchor's Verkey as the Client");
let build_get_nym_request: String = ledger::build_get_nym_request(Some(&client_did), &trustee_did).wait().unwrap();

println!("11. Sending the GET_NYM request to the ledger");
let build_get_nym_submit_result: String = ledger::submit_request(pool_handle, &build_get_nym_request).wait().unwrap();

println!("12. Comparing Trust Anchor Verkey as written by Steward and as retrieved in Client's query");
let refresh_json: Value = serde_json::from_str(&build_get_nym_submit_result).unwrap();
let refresh_data: Value = serde_json::from_str(refresh_json["result"]["data"].as_str().unwrap()).unwrap();
let trustee_verkey_from_ledger = refresh_data["verkey"].as_str().unwrap();
println!("    Written by Steward: {}", &trustee_verkey);
println!("    Queried from ledger: {}", trustee_verkey_from_ledger);
assert_eq!(trustee_verkey, trustee_verkey_from_ledger, "verkeys did not match as expected");

// CLEAN UP
println!("13. Close and delete wallet");
wallet::close_wallet(wallet_handle);
wallet::delete_wallet(&config, USEFUL_CREDENTIALS);

println!("14. Close pool and delete pool ledger config");
pool::close_pool_ledger(pool_handle);
pool::delete_pool_ledger(&pool_name);