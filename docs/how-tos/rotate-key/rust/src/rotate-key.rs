/*
Example demonstrating how to do the key rotation on the ledger.

Steward already exists on the ledger and its DID/Verkey are obtained using seed.
Trust Anchor's DID/Verkey pair is generated and stored into wallet.
Stewards builds NYM request in order to add Trust Anchor to the ledger.
Once NYM transaction is done, Trust Anchor wants to change its Verkey.
First, temporary key is created in the wallet.
Second, Trust Anchor builds NYM request to replace the Verkey on the ledger.
Third, when NYM transaction succeeds, Trust Anchor makes new Verkey permanent in wallet
(it was only temporary before).

To assert the changes, Trust Anchor reads both the Verkey from the wallet and the Verkey from the ledger
using GET_NYM request, to make sure they are equal to the new Verkey, not the original one
added by Steward
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
use indy::future::Future;
use indy::ledger;
use indy::pool;
use indy::wallet;

const PROTOCOL_VERSION: usize = 2;
static USEFUL_CREDENTIALS: &'static str = r#"{"key": "12345678901234567890123456789012"}"#;

fn main() {
    let wallet_name = "wallet";
    let pool_name = "pool";

    // PART 1
    indy::pool::set_protocol_version(PROTOCOL_VERSION).wait().unwrap();

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

    println!("5. Generating and storing steward DID and Verkey");
    let first_json_seed = json!({
        "seed":"000000000000000000000000Steward1"
    }).to_string();
    let (steward_did, _steward_verkey) = did::create_and_store_my_did(wallet_handle, &first_json_seed).wait().unwrap();

    println!("6. Generating and storing Trust Anchor DID and Verkey");
    let (trustee_did, trustee_verkey) = did::create_and_store_my_did(wallet_handle, &"{}".to_string()).wait().unwrap();

    // 7. Build NYM request to add Trust Anchor to the ledger
    println!("7. Build NYM request to add Trust Anchor to the ledger");
    let build_nym_request: String = ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).wait().unwrap();

    // 8. Sending the nym request to ledger
    println!("8. Sending NYM request to ledger");
    let _build_nym_sign_submit_result: String = ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).wait().unwrap();

    // PART 2
    println!("9. Generating new Verkey of Trust Anchor in the wallet");
    let trustee_temp_verkey = did::replace_keys_start(wallet_handle, &trustee_did, &"{}").wait().unwrap();

    println!("10. Building NYM request to update new verkey to ledger");
    let replace_key_nym_request: String = ledger::build_nym_request(&trustee_did, &trustee_did, Some(&trustee_temp_verkey), None, Some("TRUST_ANCHOR")).wait().unwrap();

    println!("11. Sending NYM request to the ledger");
    let _replace_key_nym_sign_submit_result: String = ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &replace_key_nym_request).wait().unwrap();

    println!("12. Applying new Trust Anchor's Verkey in wallet");
    did::replace_keys_apply(wallet_handle, &trustee_did).wait().unwrap();

    // PART 3
    println!("13. Reading new Verkey from wallet");
    let trustee_verkey_from_wallet = did::key_for_local_did(wallet_handle, &trustee_did).wait().unwrap();

    println!("14. Building GET_NYM request to get Trust Anchor from Verkey");
    let refresh_build_nym_request: String = ledger::build_get_nym_request(None, &trustee_did).wait().unwrap();

    println!("15. Sending GET_NYM request to ledger");
    let refresh_build_nym_response: String = ledger::submit_request(pool_handle, &refresh_build_nym_request).wait().unwrap();

    println!("16. Comparing Trust Anchor verkeys");
    let refresh_json: Value = serde_json::from_str(&refresh_build_nym_response).unwrap();
    let refresh_data: Value = serde_json::from_str(refresh_json["result"]["data"].as_str().unwrap()).unwrap();
    let trustee_verkey_from_ledger = refresh_data["verkey"].as_str().unwrap();
    println!("    Written by Steward: {}", &trustee_verkey);
    println!("    Current from wallet: {}", &trustee_verkey_from_wallet);
    println!("    Current from ledger: {}", &trustee_verkey_from_ledger);
    assert_ne!(trustee_verkey, trustee_verkey_from_ledger, "Verkey's are matched");
    assert_eq!(trustee_verkey_from_wallet, trustee_verkey_from_ledger, "Verkey's did not match as expected");

    // CLEAN UP
    println!("17. Close and delete wallet");
    wallet::close_wallet(wallet_handle).wait().unwrap();
    wallet::delete_wallet(&config, USEFUL_CREDENTIALS).wait().unwrap();

    // Close pool
    println!("    Close pool and delete pool ledger config");
    pool::close_pool_ledger(pool_handle).wait().unwrap();
    pool::delete_pool_ledger(&pool_name).wait().unwrap();
}

fn create_genesis_txn_file_for_pool(pool_name: &str) -> String {
    let test_pool_ip = env::var("TEST_POOL_IP").unwrap_or("127.0.0.1".to_string());

    let node_txns = format!(
        r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","blskey_pop":"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1","client_ip":"{0}","client_port":9702,"node_ip":"{0}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}
           {{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","blskey_pop":"Qr658mWZ2YC8JXGXwMDQTzuZCWF7NK9EwxphGmcBvCh6ybUuLxbG65nsX4JvD4SPNtkJ2w9ug1yLTj6fgmuDg41TgECXjLCij3RMsV8CwewBVgVN67wsA45DFWvqvLtu4rjNnE9JbdFTc1Z4WCPA3Xan44K1HoHAq9EVeaRYs8zoF5","client_ip":"{0}","client_port":9704,"node_ip":"{0}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}
           {{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","blskey_pop":"QwDeb2CkNSx6r8QC8vGQK3GRv7Yndn84TGNijX8YXHPiagXajyfTjoR87rXUu4G4QLk2cF8NNyqWiYMus1623dELWwx57rLCFqGh7N4ZRbGDRP4fnVcaKg1BcUxQ866Ven4gw8y4N56S5HzxXNBZtLYmhGHvDtk6PFkFwCvxYrNYjh","client_ip":"{0}","client_port":9706,"node_ip":"{0}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}
           {{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","blskey_pop":"RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP","client_ip":"{0}","client_port":9708,"node_ip":"{0}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip);

    let pool_config_pathbuf = write_genesis_txn_to_file(pool_name, node_txns.as_str());
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
