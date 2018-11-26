println!("5. Generating and storing steward DID and Verkey");
let first_json_seed = json!({
"seed":"000000000000000000000000Steward1"
}).to_string();
let (steward_did, _steward_verkey) = Did::new(wallet_handle, &first_json_seed).unwrap();

println!("6. Generating and storing Trust Anchor DID and Verkey");
let (trustee_did, trustee_verkey) = Did::new(wallet_handle, &"{}".to_string()).unwrap();