println!("20. Verifier is verifying proof from Prover");

let rev_reg_defs_json = json!({}).to_string();
let rev_regs_json = json!({}).to_string();
let valid = Verifier::verify_proof(&proof_req_json.to_string(),
&proof_json,
&schemas_json,
&credential_defs_json,
&rev_reg_defs_json,
&rev_regs_json
).unwrap();

assert!(valid);

// Clean UP
println!("21. Close and delete two wallets");
Wallet::close(prover_wallet_handle).unwrap();
Wallet::delete(&prover_wallet_config, USEFUL_CREDENTIALS).unwrap();
Wallet::close(wallet_handle).unwrap();
Wallet::delete(&config, USEFUL_CREDENTIALS).unwrap();

println!("22. Close pool and delete pool ledger config");
Pool::close(pool_handle).unwrap();
Pool::delete(&pool_name).unwrap();