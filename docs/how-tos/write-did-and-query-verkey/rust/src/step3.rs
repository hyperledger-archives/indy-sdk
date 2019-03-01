 println!("5. Generating and storing steward DID and Verkey");
let first_json_seed = json!({
    "seed":"000000000000000000000000Steward1"
}).to_string();
let (steward_did, _steward_verkey) = did::create_and_store_my_did(wallet_handle, &first_json_seed).wait().unwrap();

println!("6. Generating and storing Trust Anchor DID and Verkey");
let (trustee_did, trustee_verkey) = did::create_and_store_my_did(wallet_handle, &"{}".to_string()).wait().unwrap();