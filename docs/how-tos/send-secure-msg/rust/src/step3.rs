fn init() -> (i32, String, String) {
    let mut cmd = String::new();

    println!("Who are you? ");
    io::stdin().read_line(&mut cmd).unwrap();

    let config = json!({ "id" : format!("{}-wallet", cmd) }).to_string();
    wallet::create_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();
    let wallet_handle: i32 = wallet::open_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

    let (did, verkey) = did::create_and_store_my_did(wallet_handle, "{}").wait().unwrap();
    println!("My DID and Verkey: {} {}", did, verkey);

    println!("Other party's DID and Verkey? ");
    let mut other = String::new();
    io::stdin().read_line(&mut other).unwrap();
    let other_verkey = other.trim().split(" ").collect::<Vec<&str>>()[1].trim().to_string();

    (wallet_handle, verkey, other_verkey)
}
