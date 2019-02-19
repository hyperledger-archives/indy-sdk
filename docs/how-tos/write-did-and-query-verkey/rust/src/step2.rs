pool::set_protocol_version(PROTOCOL_VERSION).wait().unwrap();

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