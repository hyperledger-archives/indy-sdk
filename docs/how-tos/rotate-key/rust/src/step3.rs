println!("9. Generating new Verkey of Trust Anchor in the wallet");
let trustee_temp_verkey = Did::replace_keys_start(wallet_handle, &trustee_did, &"{}").unwrap();

println!("10. Building NYM request to update new verkey to ledger");
let replace_key_nym_request: String = Ledger::build_nym_request(&trustee_did, &trustee_did, Some(&trustee_temp_verkey), None, Some("TRUST_ANCHOR")).unwrap();

println!("11. Sending NYM request to the ledger");
let _replace_key_nym_sign_submit_result: String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &replace_key_nym_request).unwrap();

println!("12. Applying new Trust Anchor's Verkey in wallet");
Did::replace_keys_apply(wallet_handle, &trustee_did).unwrap();