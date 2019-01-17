println!("13. Reading new Verkey from wallet");
let trustee_verkey_from_wallet = Did::get_ver_key_local(wallet_handle, &trustee_did).unwrap();

println!("14. Building GET_NYM request to get Trust Anchor from Verkey");
let refresh_build_nym_request: String = Ledger::build_get_nym_request(None, &trustee_did).unwrap();

println!("15. Sending GET_NYM request to ledger");
let refresh_build_nym_response: String = Ledger::submit_request(pool_handle, &refresh_build_nym_request).unwrap();

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
Wallet::close(wallet_handle).unwrap();
Wallet::delete(&config, USEFUL_CREDENTIALS).unwrap();

// Close pool
println!("    Close pool and delete pool ledger config");
Pool::close(pool_handle).unwrap();
Pool::delete(&pool_name).unwrap();