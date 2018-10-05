extern crate vcx;
extern crate serde;
extern crate rand;
extern crate rust_libindy_wrapper;

#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use vcx::utils::cstring::CStringUtils;
    use std::ffi::CString;
    use vcx::utils::libindy::return_types_u32;
    use std::fs;
    use std::path::PathBuf;
    use std::io::Write;
    use std::ptr;
    use std::time::Duration;
    use vcx::settings;
    use vcx::utils::constants::GENESIS_PATH;
    use vcx::api::utils::vcx_agent_provision_async;
    use vcx::api::vcx::{ vcx_init_with_config, vcx_shutdown };
    use vcx::utils::error;
    use vcx::api::wallet;

    pub fn get_details(agency:&str) -> serde_json::Value {
        match agency {
            "consumer" => json!({
                    "url": "https://agency-ea-sandbox.evernym.com",
                    "did": "HB7qFQyFxx4ptjKqioEtd8",
                    "verkey": "9pJkfHyfJMZjUjS7EZ2q2HX55CbFQPKpQ9eTjSAUMLU8",
                    "seed": "000000000000000000000000Trustee1"
            }),
            "enterprise" => json!({
                "url": "https://enym-eagency.pdev.evernym.com",
                "did": "dTLdJqRZLwMuWSogcKfBT",
                "verkey": "LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH",
                "seed": "000000000000000000000000Trustee1"
             }),
            &_ => json!({}),
        }
    }

    pub fn create_genesis_txn_file(test_pool_ip:&str) {
        let node_txns = vec![format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node1","blskey":"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba","blskey_pop":"RahHYiCvoNCtPTrVtP7nMC5eTYrsUA8WjXbdhNc8debh1agE9bGiJxWBXYNFbnJXoXhWFMvyqhqhRoq737YQemH5ik9oL7R4NTTCz2LEZhkgLJzB3QRQqJyBNyv7acbdHrAT8nQ9UkLbaVL9NBpnWXBTw4LEMePaSHEw66RzPNdAX1","client_ip":"{}","client_port":9702,"node_ip":"{}","node_port":9701,"services":["VALIDATOR"]}},"dest":"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv"}},"metadata":{{"from":"Th7MpTaRZVRYnPiabds81Y"}},"type":"0"}},"txnMetadata":{{"seqNo":1,"txnId":"fea82e10e894419fe2bea7d96296a6d46f50f93f9eeda954ec461b2ed2950b62"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
             format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node2","blskey":"37rAPpXVoxzKhz7d9gkUe52XuXryuLXoM6P6LbWDB7LSbG62Lsb33sfG7zqS8TK1MXwuCHj1FKNzVpsnafmqLG1vXN88rt38mNFs9TENzm4QHdBzsvCuoBnPH7rpYYDo9DZNJePaDvRvqJKByCabubJz3XXKbEeshzpz4Ma5QYpJqjk","blskey_pop":"Qr658mWZ2YC8JXGXwMDQTzuZCWF7NK9EwxphGmcBvCh6ybUuLxbG65nsX4JvD4SPNtkJ2w9ug1yLTj6fgmuDg41TgECXjLCij3RMsV8CwewBVgVN67wsA45DFWvqvLtu4rjNnE9JbdFTc1Z4WCPA3Xan44K1HoHAq9EVeaRYs8zoF5","client_ip":"{}","client_port":9704,"node_ip":"{}","node_port":9703,"services":["VALIDATOR"]}},"dest":"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb"}},"metadata":{{"from":"EbP4aYNeTHL6q385GuVpRV"}},"type":"0"}},"txnMetadata":{{"seqNo":2,"txnId":"1ac8aece2a18ced660fef8694b61aac3af08ba875ce3026a160acbc3a3af35fc"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
             format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node3","blskey":"3WFpdbg7C5cnLYZwFZevJqhubkFALBfCBBok15GdrKMUhUjGsk3jV6QKj6MZgEubF7oqCafxNdkm7eswgA4sdKTRc82tLGzZBd6vNqU8dupzup6uYUf32KTHTPQbuUM8Yk4QFXjEf2Usu2TJcNkdgpyeUSX42u5LqdDDpNSWUK5deC5","blskey_pop":"QwDeb2CkNSx6r8QC8vGQK3GRv7Yndn84TGNijX8YXHPiagXajyfTjoR87rXUu4G4QLk2cF8NNyqWiYMus1623dELWwx57rLCFqGh7N4ZRbGDRP4fnVcaKg1BcUxQ866Ven4gw8y4N56S5HzxXNBZtLYmhGHvDtk6PFkFwCvxYrNYjh","client_ip":"{}","client_port":9706,"node_ip":"{}","node_port":9705,"services":["VALIDATOR"]}},"dest":"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya"}},"metadata":{{"from":"4cU41vWW82ArfxJxHkzXPG"}},"type":"0"}},"txnMetadata":{{"seqNo":3,"txnId":"7e9f355dffa78ed24668f0e0e369fd8c224076571c51e2ea8be5f26479edebe4"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip),
             format!(r#"{{"reqSignature":{{}},"txn":{{"data":{{"data":{{"alias":"Node4","blskey":"2zN3bHM1m4rLz54MJHYSwvqzPchYp8jkHswveCLAEJVcX6Mm1wHQD1SkPYMzUDTZvWvhuE6VNAkK3KxVeEmsanSmvjVkReDeBEMxeDaayjcZjFGPydyey1qxBHmTvAnBKoPydvuTAqx5f7YNNRAdeLmUi99gERUU7TD8KfAa6MpQ9bw","blskey_pop":"RPLagxaR5xdimFzwmzYnz4ZhWtYQEj8iR5ZU53T2gitPCyCHQneUn2Huc4oeLd2B2HzkGnjAff4hWTJT6C7qHYB1Mv2wU5iHHGFWkhnTX9WsEAbunJCV2qcaXScKj4tTfvdDKfLiVuU2av6hbsMztirRze7LvYBkRHV3tGwyCptsrP","client_ip":"{}","client_port":9708,"node_ip":"{}","node_port":9707,"services":["VALIDATOR"]}},"dest":"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA"}},"metadata":{{"from":"TWwCRQRZ2ZHMJFn9TzLp7W"}},"type":"0"}},"txnMetadata":{{"seqNo":4,"txnId":"aa5e817d7cc626170eca175822029339a444eb0ee8f0bd20d3b0b76e566fb008"}},"ver":"1"}}"#, test_pool_ip, test_pool_ip)];

        let txn_file_data = node_txns[0..4].join("\n");

        let mut f = fs::File::create(GENESIS_PATH).unwrap();
        f.write_all(txn_file_data.as_bytes()).unwrap();
        f.flush().unwrap();
        f.sync_all().unwrap();
    }

    fn provision_agent() -> Result<String, u32> {
        use vcx::settings;
        let mut rng = rand::thread_rng();
        let settings = get_details("consumer");
        let config = json!({
            "wallet_name": settings::DEFAULT_WALLET_NAME,
            "agent_seed": format!("HANKHILL{}00000000001DIRECTION", rng.gen_range(1000,9999)),
            "enterprise_seed": settings["seed"],
            "wallet_key": settings::DEFAULT_WALLET_KEY,
            "agency_url": settings["url"],
            "agency_did": settings["did"],
            "agency_verkey": settings["verkey"],
        });
        let config = CStringUtils::string_to_cstring(config.to_string());
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        vcx_agent_provision_async(cb.command_handle, config.as_ptr(), Some(cb.get_callback()));
        let vcx_config = cb.receive(Some(Duration::from_secs(10)))?.unwrap();
        let mut vcx_config:serde_json::Value = serde_json::from_str(&vcx_config).unwrap();
        let vcx_config = vcx_config.as_object_mut().unwrap();
        vcx_config.insert( "institution_logo_url".to_string(), json!("https://robohash.org/hankhill"));
        vcx_config.insert( "institution_name".to_string(), json!("Harlan Gas"));
        vcx_config.insert( "genesis_path".to_string(), json!(GENESIS_PATH));
        match serde_json::to_string(&vcx_config) {
            Ok(s) => Ok(s),
            Err(_) => Err(1),
        }
    }

    fn delete_indy_client(){
        use std::fs::remove_dir_all;
        use std::env::home_dir;
        use std::path::PathBuf;
        let p = match home_dir() {
            Some(path) => path,
            None => panic!("Cannot find home directory"),
        };
        let mut path = PathBuf::new();
        path.push(p);
        path.push(".indy_client");
        path.push("wallet");
        remove_dir_all(path).unwrap_or(());
    }

    fn init_vcx(vcx_config: &str) -> Result<(), u32> {
        let cb = return_types_u32::Return_U32::new().unwrap();
        let err = vcx_init_with_config(cb.command_handle,
                                       CString::new(vcx_config).unwrap().as_ptr(),
                                       Some(cb.get_callback()));
        assert_eq!(err, error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10)))
    }

    fn get_token_info() -> String {
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        assert_eq!(wallet::vcx_wallet_get_token_info(cb.command_handle,
                                                     0,
                                                     Some(cb.get_callback())),
                   error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap().unwrap()
    }
    fn export_wallet(path: std::path::PathBuf) {
        let cb = return_types_u32::Return_U32::new().unwrap();
        assert_eq!(wallet::vcx_wallet_export(cb.command_handle,
                                             CString::new(path.to_str().unwrap()).unwrap().as_ptr(),
                                             CString::new(settings::DEFAULT_WALLET_BACKUP_KEY).unwrap().as_ptr(),
                                             Some(cb.get_callback())), error::SUCCESS.code_num);
        cb.receive(Some(Duration::from_secs(10))).unwrap();
    }

    fn send_tokens(amt:u32, addr: Option<&str>) -> Result<Option<String>, u32> {
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let dest_addr = match addr {
            Some(a) => a,
            None => "pay:sov:22UFLTPu1MagmX6b6g2ZXy9n98dA52Kd1VH8Hjjf82L7zZEnC5",
        };
        wallet::vcx_wallet_send_tokens(cb.command_handle,
                                       0,
                                       CString::new(amt.to_string()).unwrap().as_ptr(),
                                       CString::new(dest_addr).unwrap().as_ptr(),
                                       Some(cb.get_callback()));
        cb.receive(Some(Duration::from_secs(10)))
    }

    fn import_wallet(path: std::path::PathBuf) {
        let cb = return_types_u32::Return_U32::new().unwrap();
        let import_config = json!({
                settings::CONFIG_WALLET_NAME: settings::DEFAULT_WALLET_NAME,
                settings::CONFIG_WALLET_KEY: settings::DEFAULT_WALLET_KEY,
                settings::CONFIG_EXPORTED_WALLET_PATH: path.to_str().unwrap(),
                settings::CONFIG_WALLET_BACKUP_KEY: settings::DEFAULT_WALLET_BACKUP_KEY,
            }).to_string();

        let import_config_c = CString::new(import_config).unwrap();
        assert_eq!(wallet::vcx_wallet_import(cb.command_handle,
                                             import_config_c.as_ptr(),
                                             Some(cb.get_callback())), error::SUCCESS.code_num);

        match cb.receive(Some(Duration::from_secs(5))) {
            Ok(_) => (),
            Err(e) => println!("ERROR: {}", e),
        };
    }

    fn create_path_and_file_name() -> std::path::PathBuf {
        let mut path = PathBuf::new();
        let export_wallet_name = "backup.wallet";
        path.push("/tmp");
        path.push(export_wallet_name);
        fs::remove_file(path.clone()).unwrap_or(());
        path
    }

    fn get_fees() -> Result<Option<String>, u32> {
        use vcx::api::utils::vcx_ledger_get_fees;
        let cb = return_types_u32::Return_U32_STR::new().unwrap();
        let _err = vcx_ledger_get_fees(cb.command_handle, Some(cb.get_callback()));
        cb.receive(Some(Duration::from_secs(10)))
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_error_codes() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::DEFAULT_WALLET_KEY);
        delete_indy_client();
        create_genesis_txn_file("127.0.0.1");
        let vcx_config = provision_agent().unwrap();
        init_vcx(&vcx_config).unwrap();
        vcx_shutdown(false);
        assert_eq!(provision_agent().err(), Some(error::DID_ALREADY_EXISTS_IN_WALLET.code_num));
        vcx_shutdown(true);
    }

    #[cfg(feature = "agency")]
    #[cfg(feature = "sovtoken")]
    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_token_balance() {
        use vcx::api::vcx::vcx_mint_tokens;
        delete_indy_client();
        create_genesis_txn_file("127.0.0.1");
        let vcx_config = provision_agent().unwrap();
        init_vcx(&vcx_config).unwrap();
        vcx_mint_tokens(ptr::null_mut(),ptr::null_mut());
        send_tokens(1500, None).unwrap();
        let token_info2 = get_token_info();
        let path = create_path_and_file_name();
        export_wallet(path.clone());
        assert_eq!(vcx_shutdown(true), error::SUCCESS.code_num);
        settings::clear_config();
        import_wallet(path.clone());
        init_vcx(&vcx_config).unwrap();
        let token_info3 = get_token_info();
        let token_info2: serde_json::Value = serde_json::from_str(&token_info2).unwrap();
        let token_info3: serde_json::Value = serde_json::from_str(&token_info3).unwrap();
        assert_eq!(token_info2["balance_str"], token_info3["balance_str"]);
    }

/// This will mint the standard amount of sovatoms into your wallet (500000000) and
/// then you can send those to an address.
/// Provide the ip address of your ledger (for the generic genesis txn file), the
/// payment address to receive the sovatoms, and the amout of sovatoms you wish
/// to send to the address.
    #[ignore]
    #[cfg(feature = "agency")]
    #[cfg(feature = "sovtoken")]
    #[test]
    fn test_sandbox_token_balance() {
        use vcx::api::vcx::vcx_mint_tokens;
        let sovatoms = 1234567890;
        let receiving_address = Some("pay:sov:jsPfjNn9GULzrhSqDWC3swx1uFjUgutSHwr32GTSMZ8kwA7VT");
        let ip = "34.212.206.9";
        create_genesis_txn_file(ip);
        let vcx_config = provision_agent().unwrap();
        init_vcx(&vcx_config).unwrap();
        vcx_mint_tokens(ptr::null_mut(),ptr::null_mut());
        send_tokens(sovatoms, receiving_address).unwrap();
    }

    /// this will simply get fees from a ledger at the given ip address
    #[ignore]
    #[cfg(feature = "sovtoken")]
    #[cfg(feature = "agency")]
    #[test]
    fn test_get_fees() {
        let ip="127.0.0.1";
        create_genesis_txn_file(ip);
        let vcx_config = provision_agent().unwrap();
        init_vcx(&vcx_config).unwrap();
        println!("{:?}", get_fees().unwrap());
    }
}
