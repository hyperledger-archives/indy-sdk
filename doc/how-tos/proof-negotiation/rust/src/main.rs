// ------------------------------------------
// crates.io
// ------------------------------------------
#[macro_use] extern crate serde_json;


// ------------------------------------------
// hyperledger crates
// ------------------------------------------
extern crate indy;                      // rust wrapper project

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use indy::did::Did;
use indy::anoncreds::Issuer;
use indy::ledger::Ledger;
use indy::pool::Pool;
use indy::anoncreds::Prover;
use indy::anoncreds::Verifier;
use indy::wallet::Wallet;

const PROTOCOL_VERSION: usize = 2;
static USEFUL_CREDENTIALS : &'static str = r#"
   {
       "key": "12345678901234567890123456789012"
   }
"#;


fn create_genesis_txn_file_for_pool(pool_name: &str) -> PathBuf {
    let nodes_count = 4;

    let test_pool_ip = env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string());

    let node_txns = vec![
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","blskey_pop":"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","blskey_pop":"Qr658mWZ2YC8JXGXwMDQTzuZCWF7NK9EwxphGmcBvCh6ybUuLxbG65nsX4JvD4SPNtkJ2w9ug1yLTj6fgmuDg41TgECXjLCij3RMsV8CwewBVgVN67wsA45DFWvqvLtu4rjNnE9JbdFTc1Z4WCPA3Xan44K1HoHAq9EVeaRYs8zoF5","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","blskey_pop":"QwDeb2CkNSx6r8QC8vGQK3GRv7Yndn84TGNijX8YXHPiagXajyfTjoR87rXUu4G4QLk2cF8NNyqWiYMus1623dELWwx57rLCFqGh7N4ZRbGDRP4fnVcaKg1BcUxQ866Ven4gw8y4N56S5HzxXNBZtLYmhGHvDtk6PFkFwCvxYrNYjh","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","blskey_pop":"RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip)];

    let txn_file_data = node_txns[0..(nodes_count as usize)].join("\n");

    write_genesis_txn_to_file(pool_name, txn_file_data.as_str())
}

fn write_genesis_txn_to_file(pool_name: &str,
                           txn_file_data: &str) -> PathBuf {

    let mut txn_file_path = env::temp_dir();
    txn_file_path.push("indy_client");
    txn_file_path.push(format!("{}.txn", pool_name));

    if !txn_file_path.parent().unwrap().exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(txn_file_path.parent().unwrap()).unwrap();
    }

    let mut f = fs::File::create(txn_file_path.as_path()).unwrap();
    f.write_all(txn_file_data.as_bytes()).unwrap();
    f.flush().unwrap();
    f.sync_all().unwrap();

    txn_file_path
}


fn main() {
    let wallet_name = "wallet";
    let pool_name = "pool";

    // 1. Creating a new local pool ledger configuration that can be used later to connect pool nodes.
    let pool_config_pathbuf = create_genesis_txn_file_for_pool(pool_name);
    let pool_config_file = pool_config_pathbuf.as_os_str().to_str().unwrap().to_string();
    indy::pool::Pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

    println!("1. Creating a new local pool ledger configuration");
    println!("   pool: {} and file: {}", &pool_name, pool_config_file);
    let pool_config = json!({
        "genesis_txn" : &pool_config_file
    });
    Pool::create_ledger_config(&pool_name, Some(&pool_config.to_string())).unwrap();

    // 2. Open pool ledger and get the pool handle from libindy.
    println!("2. Open pool ledger");
    let pool_handle: i32 = Pool::open_ledger(&pool_name, None).unwrap();

    // 3. Creates a new wallet
    println!("3. Creates a new wallet");
    let config = json!({ "id" : wallet_name.to_string() }).to_string();
    Wallet::create(&config, USEFUL_CREDENTIALS).unwrap();

    // 4. Open wallet and get the wallet handle from libindy
    println!("4. Open wallet");
    let wallet_handle: i32 = Wallet::open(&config, USEFUL_CREDENTIALS).unwrap();

    // 5. Generating and storing steward DID and Verkey
    println!("5. Generating and storing steward DID and Verkey");
    let first_json_seed = json!({
        "seed":"000000000000000000000000Steward1"
    }).to_string();
    let (steward_did, _steward_verkey) = Did::new(wallet_handle, &first_json_seed).unwrap();

    // 6. Generating and storing Trust Anchor DID and Verkey
    println!("6. Generating and storing Trust Anchor DID and Verkey");
    let (trustee_did, trustee_verkey) = Did::new(wallet_handle, &"{}".to_string()).unwrap();

    // 7. Build NYM request to add Trust Anchor to the ledger
    println!("7. Build NYM request");
    let build_nym_request: String = Ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).unwrap();

    // 8. Sending the nym request to ledger
    println!("8. Sending the nym request to ledger");
    let _build_nym_sign_submit_result: String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).unwrap();

    // 9. build the schema definition request
    println!("9. build the schema definition request");
    let name = "gvt";
    let version = "1.0";
    let attributes = r#"["age", "sex", "height", "name"]"#;
    let (schema_id, schema_json) = Issuer::create_schema(&steward_did, name, version, attributes).unwrap();

    let build_schema_request: String = Ledger::build_schema_request(&steward_did, &schema_json).unwrap();

    // 10. Sending the SCHEMA request to the ledger
    println!("10. Sending the SCHEMA request to the ledger");
    let _signed_schema_request_response = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_schema_request).unwrap();

    // 11. Creating and storing CREDENTIAL DEFINITION using anoncreds as Trust Anchor, for the given Schema
    println!("11. Creating and storing CREDENTIAL DEFINITION using anoncreds as Trust Anchor, for the given Schema");
    let config_json = r#"{ "support_revocation": false }"#;
    let tag = r#"TAG1"#;

    let (cred_def_id, cred_def_json) = Issuer::create_and_store_credential_def(wallet_handle, &trustee_did, &schema_json, tag, None, config_json).unwrap();

    // 12. Creating Prover wallet and opening it to get the handle
    println!("12. Creating Prover wallet and opening it to get the handle");
    let prover_did = "VsKV7grR1BUE29mG2Fm2kX";
    let prover_wallet_name = "prover_wallet";
    let prover_wallet_config = json!({ "id" : prover_wallet_name.to_string() }).to_string();
    Wallet::create(&prover_wallet_config, USEFUL_CREDENTIALS).unwrap();
    let prover_wallet_handle: i32 = Wallet::open(&prover_wallet_config, USEFUL_CREDENTIALS).unwrap();

    // 13. Prover is creating Master Secret
    println!("13. Prover is creating Master Secret");
    let master_secret_name = "master_secret";
    Prover::create_master_secret(prover_wallet_handle, Some(master_secret_name)).unwrap();

    // 14. Issuer (Trust Anchor) is creating a Credential Offer for Prover
    println!("14. Issuer (Trust Anchor) is creating a Credential Offer for Prover");
    let cred_offer_json = Issuer::create_credential_offer(wallet_handle, &cred_def_id).unwrap();

    // 15. Prover creates Credential Request
    println!("15. Prover creates Credential Request");
    let (cred_req_json, cred_req_metadata_json) = Prover::create_credential_req(prover_wallet_handle, prover_did, &cred_offer_json, &cred_def_json, &master_secret_name).unwrap();

    // 16. Issuer (Trust Anchor) creates Credential for Credential Request
    println!("16. Issuer (Trust Anchor) creates Credential for Credential Request");

    let cred_values_json = json!({
        "sex": { "raw": "male", "encoded": "5944657099558967239210949258394887428692050081607692519917050011144233115103" },
        "name": { "raw": "Alex", "encoded": "99262857098057710338306967609588410025648622308394250666849665532448612202874" },
        "height": { "raw": "175", "encoded": "175" },
        "age": { "raw": "28", "encoded": "28" },
    });

    let (cred_json, _cred_revoc_id, _revoc_reg_delta_json) =
        Issuer::create_credential(wallet_handle, &cred_offer_json, &cred_req_json, &cred_values_json.to_string(), None, -1).unwrap();

    // 17. Prover processes and stores Credential
    println!("17. Prover processes and stores Credential");
    Prover::store_credential(prover_wallet_handle, None, &cred_req_metadata_json, &cred_json, &cred_def_json, None).unwrap();

    // 18. Prover gets Credentials for Proof Request
    println!("18. Prover gets Credentials for Proof Request");

    let proof_req_json = json!({
        "nonce": "123432421212",
        "name": "proof_req_1",
        "version": "0.1",
        "requested_attributes": {
            "attr1_referent": {
                "name": "name",
                "restrictions": {
                    "issuer_did": trustee_did,
                    "schema_id": schema_id
                }
            }
        },
        "requested_predicates": {
            "predicate1_referent": {
                "name": "age",
                "p_type": ">=",
                "p_value": 18,
                "restrictions": {
                    "issuer_did": trustee_did
                }
            }
        }
    });
    println!("Proof Request: {}", proof_req_json);

    let creds_for_proof_request_json = Prover::get_credentials_for_proof_req(prover_wallet_handle, &proof_req_json.to_string()).unwrap();
    println!("Credentials for Proof Request: {}", creds_for_proof_request_json);

    // 19. Prover creates Proof for Proof Request
    println!("19. Prover creates Proof for Proof Request");
    let creds_for_proof_request = serde_json::from_str::<serde_json::Value>(&creds_for_proof_request_json).unwrap();
    let creds_for_attr_1 = &creds_for_proof_request["attrs"]["attr1_referent"];
    let credential = &creds_for_attr_1[0]["cred_info"];

    let requested_credentials_json = json!({
        "self_attested_attributes": {},
        "requested_attributes": {
            "attr1_referent": {
                "cred_id": credential["referent"].as_str().unwrap(),
                "revealed": true
            }
        },
        "requested_predicates":{
            "predicate1_referent":{
                "cred_id": credential["referent"].as_str().unwrap(),
            }
        }
    });
    println!("Requested Credentials for Proving: {}", requested_credentials_json.to_string());

    let schemas_json = json!({
        schema_id.as_str(): serde_json::from_str::<serde_json::Value>(&schema_json).unwrap()
    }).to_string();
    let credential_defs_json = json!({
        cred_def_id.as_str(): serde_json::from_str::<serde_json::Value>(&cred_def_json).unwrap()
    }).to_string();
    let rev_states_json = json!({}).to_string();

    let proof_json = Prover::create_proof(prover_wallet_handle,
                                          &proof_req_json.to_string(),
                                          &requested_credentials_json.to_string(),
                                          &master_secret_name,
                                          &schemas_json,
                                          &credential_defs_json,
                                          &rev_states_json).unwrap();
    let proof = serde_json::from_str::<serde_json::Value>(&proof_json).unwrap();
    assert_eq!("Alex", proof["requested_proof"]["revealed_attrs"]["attr1_referent"]["raw"].as_str().unwrap());

    // 20. Verifier is verifying proof from Prover
    println!("20. Verifier is verifying proof from Prover");

    let rev_reg_defs_json = json!({}).to_string();
    let rev_regs_json = json!({}).to_string();
    let valid = Verifier::verify_proof(&proof_req_json.to_string(),
                                       &proof_json,
                                       &schemas_json,
                                       &credential_defs_json,
                                       &rev_reg_defs_json,
                                       &rev_regs_json
    ).unwrap();

    assert!(valid);

    // clean up
    // Close and delete wallet
    println!("Close and delete two wallets");
    Wallet::close(prover_wallet_handle).unwrap();
    Wallet::delete(&prover_wallet_config, USEFUL_CREDENTIALS).unwrap();
    Wallet::close(wallet_handle).unwrap();
    Wallet::delete(&config, USEFUL_CREDENTIALS).unwrap();

    // Close pool
    println!("Close pool");
    Pool::close(pool_handle).unwrap();

    // Delete pool ledger config
    println!("Delete pool ledger config");
    Pool::delete(&pool_name).unwrap();
    fs::remove_file(pool_config_pathbuf).unwrap();
}