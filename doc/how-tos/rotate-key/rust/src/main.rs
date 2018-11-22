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

use serde_json::{Value};

use indy::did::Did;
use indy::ledger::Ledger;
use indy::pool::Pool;

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
    Pool::set_protocol_version(PROTOCOL_VERSION).unwrap();

    println!("1. Creating a new local pool ledger configuration");
    println!("   pool: {} and file: {}", &pool_name, pool_config_file);
    let pool_config = json!({
        "genesis_txn" : &pool_config_file
    });
    Pool::create_ledger_config(&pool_name, Some(&pool_config.to_string())).unwrap();

    // 2. Open pool ledger and get the pool handle from libindy.
    println!("2. Open pool ledger");
    let pool_handle : i32 = indy::pool::Pool::open_ledger(&pool_name, None).unwrap();

    // 3. Creates a new wallet
    println!("3. Creates a new wallet");
    let config = json!({ "id" : wallet_name.to_string() }).to_string();
    indy::wallet::Wallet::create(&config, USEFUL_CREDENTIALS).unwrap();

    // 4. Open wallet and get the wallet handle from libindy
    println!("4. Open wallet");
    let wallet_handle : i32 = indy::wallet::Wallet::open(&config, USEFUL_CREDENTIALS).unwrap();

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
    let build_nym_request : String = Ledger::build_nym_request(&steward_did, &trustee_did, Some(&trustee_verkey), None, Some("TRUST_ANCHOR")).unwrap();

    // 8. Sending the nym request to ledger
    println!("8. Sending the nym request to ledger");
    let _build_nym_sign_submit_result : String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &steward_did, &build_nym_request).unwrap();

    // 9. Generating new Verkey of Trust Anchor in the wallet
    println!("9. Generating new Verkey of Trust Anchor in the wallet");
    let trustee_temp_verkey = Did::replace_keys_start(wallet_handle, &trustee_did, &"{}").unwrap();

    // 10. Building NYM request to update new verkey to ledger
    println!("10. Building NYM request to update new verkey to ledger");
    let replace_key_nym_request : String = Ledger::build_nym_request(&trustee_did, &trustee_did, Some(&trustee_temp_verkey), None, Some("TRUST_ANCHOR")).unwrap();

    // 11. Sending NYM request to the ledger
    println!("11. Sending NYM request to the ledger");
    let _replace_key_nym_sign_submit_result : String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &replace_key_nym_request).unwrap();

    // 12. Applying new Trust Anchor's Verkey in wallet
    println!("12. Applying new Trust Anchor's Verkey in wallet");
    Did::replace_keys_apply(wallet_handle, &trustee_did).unwrap();

    // 13. Reading new Verkey from wallet
    println!("13. Reading new Verkey from wallet");
    let trustee_verkey_from_wallet = Did::get_ver_key_local(wallet_handle, &trustee_did).unwrap();

    // 14. Building GET_NYM request to get Trust Anchor from Verkey
    println!("14. Building GET_NYM request to get Trust Anchor from Verkey");
    let refresh_build_nym_request : String = Ledger::build_nym_request(&trustee_did, &trustee_did, None, None, None).unwrap();

    // 15. Sending GET_NYM request to ledger
    println!("15. Sending GET_NYM request to ledger");
    let refresh_build_nym_response : String = Ledger::sign_and_submit_request(pool_handle, wallet_handle, &trustee_did, &refresh_build_nym_request).unwrap();

    // 16. Comparing Trust Anchor verkeys
    println!("16. output results");
    println!("    Trustee Did {}", &trustee_did);
    println!("    Trustee VerkKey from wallet {}", &trustee_verkey_from_wallet);
    println!("    nym response {}", refresh_build_nym_response);
    let refresh_json : Value = serde_json::from_str(&refresh_build_nym_response).unwrap();
    let refresh_signature_values = &refresh_json["result"]["reqSignature"]["values"];
    let trustee_did_from_ledger = refresh_signature_values[0]["from"].as_str().unwrap();

    // clean up
    // Close and delete wallet
    println!("16. Cleanup");
    println!("    Close and delete wallet");
    indy::wallet::Wallet::close(wallet_handle).unwrap();
    indy::wallet::Wallet::delete(&config, USEFUL_CREDENTIALS).unwrap();

    // Close pool
    println!("    Close pool");
    indy::pool::Pool::close(pool_handle).unwrap();

    // Delete pool ledger config
    println!("    Delete pool ledger config");
    indy::pool::Pool::delete(&pool_name).unwrap();
    fs::remove_file(pool_config_pathbuf).unwrap();

    // Perform assertions last, so that none of the demo data remains
    println!("17. Tests to confirm results");
    assert_eq!(trustee_did, trustee_did_from_ledger, "DIDs did not match as expected");

}
