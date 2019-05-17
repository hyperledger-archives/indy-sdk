// PART 1
indy::pool::set_protocol_version(PROTOCOL_VERSION).wait().unwrap();

println!("1. Creating a new local pool ledger configuration that can be used later to connect pool nodes");
let pool_config_file = create_genesis_txn_file_for_pool(pool_name);
let pool_config = json!({
    "genesis_txn" : &pool_config_file
});
pool::create_pool_ledger_config(&pool_name, Some(&pool_config.to_string())).wait().unwrap();

println!("2. Open pool ledger and get the pool handle from libindy");
let pool_handle: i32 = pool::open_pool_ledger(&pool_name, None).wait().unwrap();

println!("3. Creates a new wallet");
let config = json!({ "id" : wallet_name.to_string() }).to_string();
wallet::create_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

println!("4. Open wallet and get the wallet handle from libindy");
let wallet_handle: i32 = wallet::open_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

println!("5. Generating and storing steward DID and Verkey");
let first_json_seed = json!({
    "seed":"000000000000000000000000Steward1"
}).to_string();
let (steward_did, _steward_verkey) = did::create_and_store_my_did(wallet_handle, &first_json_seed).wait().unwrap();

println!("6. Generating and storing Trust Anchor DID and Verkey");
let (trustee_did, trustee_verkey) = did::create_and_store_my_did(wallet_handle, &"{}".to_string()).wait().unwrap();

// 7. Build NYM request to add Trust Anchor to the ledger
println!("7. Build NYM request to add Trust Anchor to the ledger");
let build_nym_request: String = ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).wait().unwrap();

// 8. Sending the nym request to ledger
println!("8. Sending NYM request to ledger");
let _build_nym_sign_submit_result: String = ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).wait().unwrap();
