indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

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

println!("4. Open wallet and get the wallet handle from libindy");
let wallet_handle: i32 = Wallet::open(&config, USEFUL_CREDENTIALS).unwrap();