// PART 3
println!("13. Reading new Verkey from wallet");
let trustee_verkey_from_wallet = did::key_for_local_did(wallet_handle, &trustee_did).wait().unwrap();

println!("14. Building GET_NYM request to get Trust Anchor from Verkey");
let refresh_build_nym_request: String = ledger::build_get_nym_request(None, &trustee_did).wait().unwrap();

println!("15. Sending GET_NYM request to ledger");
let refresh_build_nym_response: String = ledger::submit_request(pool_handle, &refresh_build_nym_request).wait().unwrap();

println!("16. Comparing Trust Anchor verkeys");
let refresh_json: Value = serde_json::from_str(&refresh_build_nym_response).unwrap();
let refresh_data: Value = serde_json::from_str(refresh_json["result"]["data"].as_str().unwrap()).unwrap();
let trustee_verkey_from_ledger = refresh_data["verkey"].as_str().unwrap();
println!("    Written by Steward: {}", &trustee_verkey);
println!("    Current from wallet: {}", &trustee_verkey_from_wallet);
println!("    Current from ledger: {}", &trustee_verkey_from_ledger);
assert_ne!(trustee_verkey, trustee_verkey_from_ledger, "Verkey's are matched");
assert_eq!(trustee_verkey_from_wallet, trustee_verkey_from_ledger, "Verkey's did not match as expected");

// CLEAN UP
println!("17. Close and delete wallet");
wallet::close_wallet(wallet_handle).wait().unwrap();
wallet::delete_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

// Close pool
println!("    Close pool and delete pool ledger config");
pool::close_pool_ledger(pool_handle).wait().unwrap();
pool::delete_pool_ledger(&pool_name).wait().unwrap();
