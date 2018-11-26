println!("12. Creating Prover wallet and opening it to get the handle");
let prover_did = "VsKV7grR1BUE29mG2Fm2kX";
let prover_wallet_name = "prover_wallet";
let prover_wallet_config = json!({ "id" : prover_wallet_name.to_string() }).to_string();
Wallet::create(&prover_wallet_config, USEFUL_CREDENTIALS).unwrap();
let prover_wallet_handle: i32 = Wallet::open(&prover_wallet_config, USEFUL_CREDENTIALS).unwrap();

println!("13. Prover is creating Master Secret");
let master_secret_name = "master_secret";
Prover::create_master_secret(prover_wallet_handle, Some(master_secret_name)).unwrap();
