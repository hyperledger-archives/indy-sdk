// PART 2
println!("9. Generating new Verkey of Trust Anchor in the wallet");
let trustee_temp_verkey = did::replace_keys_start(wallet_handle, &trustee_did, &"{}").wait().unwrap();

println!("10. Building NYM request to update new verkey to ledger");
let replace_key_nym_request: String = ledger::build_nym_request(&trustee_did, &trustee_did, Some(&trustee_temp_verkey), None, Some("TRUST_ANCHOR")).wait().unwrap();

println!("11. Sending NYM request to the ledger");
let _replace_key_nym_sign_submit_result: String = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &replace_key_nym_request).wait().unwrap();

println!("12. Applying new Trust Anchor's Verkey in wallet");
did::replace_keys_apply(wallet_handle, &trustee_did).wait().unwrap();
