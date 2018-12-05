println!("11. Creating and storing CREDENTIAL DEFINITION using anoncreds as Trust Anchor, for the given Schema");
let config_json = r#"{ "support_revocation": false }"#;
let tag = r#"TAG1"#;

let (_cred_def_id, _cred_def_json) = Issuer::create_and_store_credential_def(wallet_handle, &trustee_did, &schema_json, tag, None, config_json).unwrap();

// CLEAN UP
println!("12. Close and delete wallet");
indy::wallet::Wallet::close(wallet_handle).unwrap();
indy::wallet::Wallet::delete(&config, USEFUL_CREDENTIALS).unwrap();

println!("13. Close pool and delete pool ledger config");
Pool::close(pool_handle).unwrap();
Pool::delete(&pool_name).unwrap();