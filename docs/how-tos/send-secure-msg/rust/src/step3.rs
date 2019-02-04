fn init() -> (i32, String, String) {
    let mut cmd = String::new();

    println!("Who are you? ");
    io::stdin().read_line(&mut cmd).unwrap();

    let config = json!({ "id" : format!("{}-wallet", cmd) }).to_string();
    Wallet::create(&config, USEFUL_CREDENTIALS).unwrap();
    let wallet_handle: i32 = Wallet::open(&config, USEFUL_CREDENTIALS).unwrap();

    let (did, verkey) = Did::new(wallet_handle, "{}").unwrap();
    println!("DID and Verkey: {} {}", did, verkey);

    println!("Other party's DID and Verkey? ");
    let mut other = String::new();
    io::stdin().read_line(&mut other).unwrap();
    let other_verkey = other.trim().split(" ").collect::<Vec<&str>>()[1].trim().to_string();

    (wallet_handle, verkey, other_verkey)
}