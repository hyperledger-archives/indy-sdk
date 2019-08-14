println!("20. Verifier is verifying proof from Prover");

let rev_reg_defs_json = json!({}).to_string();
let rev_regs_json = json!({}).to_string();
let valid = anoncreds::verifier_verify_proof(&proof_req_json.to_string(),
                                   &proof_json,
                                   &schemas_json,
                                   &credential_defs_json,
                                   &rev_reg_defs_json,
                                   &rev_regs_json
).wait().unwrap();

assert!(valid);

// Clean UP
println!("21. Close and delete two wallets");
wallet::close_wallet(prover_wallet_handle).wait().unwrap();
wallet::delete_wallet(&prover_wallet_config, USEFUL_CREDENTIALS).wait().unwrap();
wallet::close_wallet(wallet_handle).wait().unwrap();
wallet::delete_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

println!("22. Close pool and delete pool ledger config");
pool::close_pool_ledger(pool_handle).wait().unwrap();
pool::delete_pool_ledger(&pool_name).wait().unwrap();
