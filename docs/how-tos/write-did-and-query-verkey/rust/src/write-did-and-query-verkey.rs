/*
Example demonstrating how to add DID with the role of Trust Anchor as Steward.
Uses seed to obtain Steward's DID which already exists on the ledger.
Then it generates new DID/Verkey pair for Trust Anchor.
Using Steward's DID, NYM transaction request is built to add Trust Anchor's DID and Verkey
on the ledger with the role of Trust Anchor.
Once the NYM is successfully written on the ledger, it generates new DID/Verkey pair that represents
a client, which are used to create GET_NYM request to query the ledger and confirm Trust Anchor's Verkey.
For the sake of simplicity, a single wallet is used. In the real world scenario, three different wallets
would be used and DIDs would be exchanged using some channel of communication
*/

// ------------------------------------------
// crates.io
// ------------------------------------------
#[macro_use]
extern crate serde_json;


// ------------------------------------------
// hyperledger crates
// ------------------------------------------
extern crate indyrs as indy;                      // rust wrapper project

use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use serde_json::Value;

use indy::did;
use indy::ledger;
use indy::pool;
use indy::wallet;
use indy::future::Future;

const PROTOCOL_VERSION: usize = 2;
static USEFUL_CREDENTIALS: &'static str = r#"{"key": "12345678901234567890123456789012"}"#;

fn main() {
    let wallet_name = "wallet";
    let pool_name = "pool";

    // PART 1
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

    // PART 2
    println!("5. Generating and storing steward DID and Verkey");
    let first_json_seed = json!({
        "seed":"000000000000000000000000Steward1"
    }).to_string();
    let (steward_did, _steward_verkey) = did::create_and_store_my_did(wallet_handle, &first_json_seed).wait().unwrap();

    println!("6. Generating and storing Trust Anchor DID and Verkey");
    let (trustee_did, trustee_verkey) = did::create_and_store_my_did(wallet_handle, &"{}".to_string()).wait().unwrap();

    // PART 3
    println!("7. Build NYM request to add Trust Anchor to the ledger");
    let build_nym_request: String = ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).wait().unwrap();

    println!("8. Sending the nym request to ledger");
    let _build_nym_sign_submit_result: String = ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).wait().unwrap();

    // PART 4
    println!("9. Generating and storing client DID and Verkey");
    let (client_did, _client_verkey) = did::create_and_store_my_did(wallet_handle, &"{}".to_string()).wait().unwrap();

    println!("10. Building the GET_NYM request to query Trust Anchor's Verkey as the Client");
    let build_get_nym_request: String = ledger::build_get_nym_request(Some(&client_did), &trustee_did).wait().unwrap();

    println!("11. Sending the GET_NYM request to the ledger");
    let build_get_nym_submit_result: String = ledger::submit_request(pool_handle, &build_get_nym_request).wait().unwrap();

    println!("12. Comparing Trust Anchor Verkey as written by Steward and as retrieved in Client's query");
    let refresh_json: Value = serde_json::from_str(&build_get_nym_submit_result).unwrap();
    let refresh_data: Value = serde_json::from_str(refresh_json["result"]["data"].as_str().unwrap()).unwrap();
    let trustee_verkey_from_ledger = refresh_data["verkey"].as_str().unwrap();
    println!("    Written by Steward: {}", &trustee_verkey);
    println!("    Queried from ledger: {}", trustee_verkey_from_ledger);
    assert_eq!(trustee_verkey, trustee_verkey_from_ledger, "verkeys did not match as expected");

    // CLEAN UP
    println!("13. Close and delete wallet");
    wallet::close_wallet(wallet_handle);
    wallet::delete_wallet(&config, USEFUL_CREDENTIALS);

    println!("14. Close pool and delete pool ledger config");
    pool::close_pool_ledger(pool_handle);
    pool::delete_pool_ledger(&pool_name);
}

fn create_genesis_txn_file_for_pool(pool_name: &str) -> String {
    #[allow(clippy::or_fun_call)]
    let test_pool_ip = env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string());

    let node_txns = vec![
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","blskey_pop":"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","blskey_pop":"Qr658mWZ2YC8JXGXwMDQTzuZCWF7NK9EwxphGmcBvCh6ybUuLxbG65nsX4JvD4SPNtkJ2w9ug1yLTj6fgmuDg41TgECXjLCij3RMsV8CwewBVgVN67wsA45DFWvqvLtu4rjNnE9JbdFTc1Z4WCPA3Xan44K1HoHAq9EVeaRYs8zoF5","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","blskey_pop":"QwDeb2CkNSx6r8QC8vGQK3GRv7Yndn84TGNijX8YXHPiagXajyfTjoR87rXUu4G4QLk2cF8NNyqWiYMus1623dELWwx57rLCFqGh7N4ZRbGDRP4fnVcaKg1BcUxQ866Ven4gw8y4N56S5HzxXNBZtLYmhGHvDtk6PFkFwCvxYrNYjh","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
        format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","blskey_pop":"RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip)
    ];

    let txn_file_data = node_txns.join("\n");

    let pool_config_pathbuf = write_genesis_txn_to_file(pool_name, txn_file_data.as_str());
    pool_config_pathbuf.as_os_str().to_str().unwrap().to_string()
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