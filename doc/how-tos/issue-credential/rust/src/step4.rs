println!("14. Issuer (Trust Anchor) is creating a Credential Offer for Prover");
let cred_offer_json = Issuer::create_credential_offer(wallet_handle, &cred_def_id).unwrap();

println!("15. Prover creates Credential Request");
let (cred_req_json, cred_req_metadata_json) = Prover::create_credential_req(prover_wallet_handle, prover_did, &cred_offer_json, &cred_def_json, &master_secret_name).unwrap();

println!("16. Issuer (Trust Anchor) creates Credential for Credential Request");

let cred_values_json = json!({
        "sex": { "raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103" },
        "name": { "raw": "Alex", "encoded": "99262857098057710338306967609588410025648622308394250666849665532448612202874" },
        "height": { "raw": "175", "encoded": "175" },
        "age": { "raw": "28", "encoded": "28" },
    });

println!("cred_values_json = '{}'", &cred_values_json.to_string());

let (cred_json, _cred_revoc_id, _revoc_reg_delta_json) =
Issuer::create_credential(wallet_handle, &cred_offer_json, &cred_req_json, &cred_values_json.to_string(), None, -1).unwrap();

println!("17. Prover processes and stores Credential");
let out_cred_id = Prover::store_credential(prover_wallet_handle, None, &cred_req_metadata_json, &cred_json, &cred_def_json, None).unwrap();

println!("Stored Credential ID is {}", &out_cred_id);

// Clean UP
println!("17. Close and delete two wallets");
Wallet::close(prover_wallet_handle).unwrap();
Wallet::delete(&prover_wallet_config, USEFUL_CREDENTIALS).unwrap();
Wallet::close(wallet_handle).unwrap();
Wallet::delete(&config, USEFUL_CREDENTIALS).unwrap();

println!("18. Close pool and delete pool ledger config");
Pool::close(pool_handle).unwrap();
Pool::delete(&pool_name).unwrap();