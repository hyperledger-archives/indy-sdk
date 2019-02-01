Pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

println!("1. Creating a new local pool ledger configuration that can be used later to connect pool nodes");
let pool_config_file = create_genesis_txn_file_for_pool(pool_name);
let pool_config = json!({
        "genesis_txn" : &pool_config_file
    });
Pool::create_ledger_config(&pool_name, Some(&pool_config.to_string())).unwrap();

println!("2. Open pool ledger and get the pool handle from libindy");
let pool_handle: i32 = Pool::open_ledger(&pool_name, None).unwrap();

println!("3. Creates a new wallet");
let config = json!({ "id" : wallet_name.to_string() }).to_string();
Wallet::create(&config, USEFUL_CREDENTIALS).unwrap();

println!("4.  Open wallet and get the wallet handle from libindy");
let wallet_handle: i32 = Wallet::open(&config, USEFUL_CREDENTIALS).unwrap();

println!("5. Generating and storing steward DID and Verkey");
let first_json_seed = json!({
"seed":"000000000000000000000000Steward1"
}).to_string();
let (steward_did, _steward_verkey) = Did::new(wallet_handle, &first_json_seed).unwrap();

println!("6. Generating and storing Trust Anchor DID and Verkey");
let (trustee_did, trustee_verkey) = Did::new(wallet_handle, &"{}".to_string()).unwrap();

// 7. Build NYM request to add Trust Anchor to the ledger
println!("7. Build NYM request to add Trust Anchor to the ledger");
let build_nym_request: String = Ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).unwrap();

// 8. Sending the nym request to ledger
println!("8. Sending NYM request to ledger");
let _build_nym_sign_submit_result: String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).unwrap();
