println!("11. Creating and storing CREDENTAIL DEFINITION using anoncreds as Trust Anchor, for the given Schema");
let config_json = r#"{ "support_revocation": false }"#;
let tag = r#"TAG1"#;

let (_cred_def_id, _cred_def_json) = anoncreds::issuer_create_and_store_credential_def(wallet_handle, &trustee_did, &schema_json, tag, None, config_json).wait().unwrap();

// CLEAN UP
println!("12. Close and delete wallet");
indy::wallet::close_wallet(wallet_handle).wait().unwrap();
indy::wallet::delete_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

println!("13. Close pool and delete pool ledger config");
pool::close_pool_ledger(pool_handle).wait().unwrap();
pool::delete_pool_ledger(&pool_name).wait().unwrap();
