println!("14. Issuer (Trust Anchor) is creating a Credential Offer for Prover");
let cred_offer_json = anoncreds::issuer_create_credential_offer(wallet_handle, &cred_def_id).wait().unwrap();

println!("15. Prover creates Credential Request");
let (cred_req_json, cred_req_metadata_json) = anoncreds::prover_create_credential_req(prover_wallet_handle, prover_did, &cred_offer_json, &cred_def_json, &master_secret_name).wait().unwrap();

println!("16. Issuer (Trust Anchor) creates Credential for Credential Request");
let cred_values_json = json!({
    "sex": { "raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103" },
    "name": { "raw": "Alex", "encoded": "99262857098057710338306967609588410025648622308394250666849665532448612202874" },
    "height": { "raw": "175", "encoded": "175" },
    "age": { "raw": "28", "encoded": "28" },
});

println!("cred_values_json = '{}'", &cred_values_json.to_string());
let (cred_json, _cred_revoc_id, _revoc_reg_delta_json) =
    anoncreds::issuer_create_credential(wallet_handle, &cred_offer_json, &cred_req_json, &cred_values_json.to_string(), None, -1).wait().unwrap();

println!("17. Prover processes and stores Credential");
let out_cred_id = anoncreds::prover_store_credential(prover_wallet_handle, None, &cred_req_metadata_json, &cred_json, &cred_def_json, None).wait().unwrap();

println!("Stored Credential ID is {}", &out_cred_id);

// Clean UP
println!("17. Close and delete two wallets");
wallet::close_wallet(prover_wallet_handle).wait().unwrap();
wallet::delete_wallet(&prover_wallet_config, USEFUL_CREDENTIALS).wait().unwrap();
wallet::close_wallet(wallet_handle).wait().unwrap();
wallet::delete_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

println!("18. Close pool and delete pool ledger config");
pool::close_pool_ledger(pool_handle).wait().unwrap();
pool::delete_pool_ledger(&pool_name).wait().unwrap();
