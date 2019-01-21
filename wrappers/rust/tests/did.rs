#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate indyrs as indy;
extern crate futures;
#[macro_use]
mod utils;

use indy::did;
use indy::ErrorCode;
use utils::b58::{FromBase58};
use utils::constants::{
    DID_1,
    SEED_1,
    VERKEY_1,
    METADATA,
    VERKEY_ABV_1,
    INVALID_HANDLE
};
use utils::setup::{Setup, SetupConfig};
use utils::wallet::Wallet;

#[allow(unused_imports)]
use futures::Future;

#[inline]
fn assert_verkey_len(verkey: &str) {
    assert_eq!(32, verkey.from_base58().unwrap().len());
}

#[cfg(test)]
mod create_new_did {
    use super::*;

    #[inline]
    fn assert_did_length(did: &str) {
        assert_eq!(16, did.from_base58().unwrap().len());
    }

    #[test]
    fn create_did_with_empty_json() {
        let wallet = Wallet::new();

        let (did, verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        assert_did_length(&did);
        assert_verkey_len(&verkey);
    }

    #[test]
    fn create_did_with_seed() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        assert_eq!(DID_1, did);
        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_cid() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1,
            "cid": true,
        }).to_string();

        let (did, verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        assert_eq!(VERKEY_1, did);
        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_did() {
        let wallet = Wallet::new();

        let config = json!({
            "did": DID_1
        }).to_string();

        let (did, verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        assert_eq!(DID_1, did);
        assert_ne!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_crypto_type() {
        let wallet = Wallet::new();

        let config = json!({
            "crypto_type": "ed25519"
        }).to_string();

        let result = did::create_and_store_my_did(wallet.handle, &config).wait();

        assert!(result.is_ok());

    }

    #[test]
    fn create_did_with_invalid_wallet_handle() {
        let result = did::create_and_store_my_did(INVALID_HANDLE, "{}").wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }

    #[test]
    fn create_wallet_empty_config() {
        let wallet = Wallet::new();

        let result = did::create_and_store_my_did(wallet.handle, "").wait();

        assert!(result.is_err());
    }

}

#[cfg(test)]
mod replace_keys_start {
    use super::*;

    #[test]
    fn replace_keys_start() {
        let wallet = Wallet::new();
        let (did, verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        let new_verkey = did::replace_keys_start(wallet.handle, &did, "{}").wait().unwrap();

        assert_verkey_len(&new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_invalid_wallet() {
        let result = did::replace_keys_start(INVALID_HANDLE, DID_1, "{}").wait();

        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }

    #[test]
    fn replace_keys_start_with_seed() {
        let wallet = Wallet::new();
        let (did, verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();
        let config = json!({"seed": SEED_1}).to_string();

        let new_verkey = did::replace_keys_start(wallet.handle, &did, &config).wait().unwrap();

        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_valid_crypto_type() {
        let wallet = Wallet::new();
        let (did, verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();
        let config = json!({"crypto_type": "ed25519"}).to_string();

        let new_verkey = did::replace_keys_start(wallet.handle, &did, &config).wait().unwrap();

        assert_verkey_len(&new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_start_invalid_crypto_type() {
        let wallet = Wallet::new();
        let (did, _verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();
        let config = json!({"crypto_type": "ed25518"}).to_string();

        let result = did::replace_keys_start(wallet.handle, &did, &config).wait();

        assert_eq!(ErrorCode::UnknownCryptoTypeError, result.unwrap_err().error_code);
    }

    #[test]
    fn replace_keys_start_invalid_did() {
        let wallet = Wallet::new();
        let result = did::replace_keys_start(wallet.handle, DID_1, "{}").wait();

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod replace_keys_apply {
    use super::*;

    fn setup() -> (Wallet, String, String) {
        let wallet = Wallet::new();
        let (did, verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        (wallet, did, verkey)
    }

    #[inline]
    fn start_key_replacement(wallet: &Wallet, did: &str) {
        let config = json!({"seed": SEED_1}).to_string();
        did::replace_keys_start(wallet.handle, did, &config).wait().unwrap();
    }

    #[test]
    fn replace_keys_apply() {
        let (wallet, did, verkey) = setup();
        start_key_replacement(&wallet, &did);

        let result = did::replace_keys_apply(wallet.handle, &did).wait();

        assert_eq!((), result.unwrap());

        let new_verkey = did::key_for_local_did(wallet.handle, &did).wait().unwrap();

        assert_eq!(VERKEY_1, new_verkey);
        assert_ne!(verkey, new_verkey);
    }

    #[test]
    fn replace_keys_apply_without_replace_keys_start() {
        let (wallet, did, _) = setup();

        let result = did::replace_keys_apply(wallet.handle, &did).wait();

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err().error_code);
    }

    #[test]
    fn replace_keys_apply_invalid_did() {
        let wallet = Wallet::new();

        let result = did::replace_keys_apply(wallet.handle, DID_1).wait();

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err().error_code);
    }

    #[test]
    fn replace_keys_apply_invalid_wallet() {
        let result = did::replace_keys_apply(INVALID_HANDLE, DID_1).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_store_their_did {
    use super::*;

    #[test]
    fn store_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": VERKEY_1}).to_string();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!((), result.unwrap());

        let verkey = did::key_for_local_did(wallet.handle, VERKEY_1).wait().unwrap();

        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn store_their_did_with_verkey() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!((), result.unwrap());

        let verkey = did::key_for_local_did(wallet.handle, DID_1).wait().unwrap();

        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn store_their_did_with_crypto_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": DID_1,
            "verkey": format!("{}:ed25519", VERKEY_1)
        }).to_string();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!((), result.unwrap());

        let verkey = did::key_for_local_did(wallet.handle, DID_1).wait().unwrap();

        assert_eq!(format!("{}:ed25519", VERKEY_1), verkey);
    }

    #[test]
    fn store_their_did_empty_identify_json() {
        let wallet = Wallet::new();

        let result = did::store_their_did(wallet.handle, "{}").wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn store_their_did_invalid_handle() {
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        let result = did::store_their_did(INVALID_HANDLE, &config).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }

    #[test]
    fn store_their_did_abbreviated_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": "8wZcEriaNLNKtteJvx7f8i",
            "verkey": "~NcYxiDXkpYi6ov5FcYDi1e"
        }).to_string();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!((), result.unwrap());
    }

    #[test]
    fn store_their_did_invalid_did() {
        let wallet = Wallet::new();
        let config = json!({"did": "InvalidDid"}).to_string();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn store_their_did_with_invalid_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": DID_1,
            "verkey": "InvalidVerkey"
        }).to_string();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn store_their_did_with_invalid_crypto_verkey() {
        let wallet = Wallet::new();
        let config = json!({
            "did": DID_1,
            "verkey": format!("{}:bad_crypto_type", VERKEY_1)
        }).to_string();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!(ErrorCode::UnknownCryptoTypeError, result.unwrap_err().error_code);
    }

    #[test]
    fn store_their_did_duplicate() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        did::store_their_did(wallet.handle, &config).wait().unwrap();

        let result = did::store_their_did(wallet.handle, &config).wait();

        assert_eq!(ErrorCode::WalletItemAlreadyExists, result.unwrap_err().error_code);
    }

    #[test]
    /*
    This test resulted from the ticket https://jira.hyperledger.org/browse/IS-802
    Previously, an error was being thrown because rollback wasn't happening.
    This test ensures the error is no longer occuring.
    */
    fn store_their_did_multiple_error_fixed() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();

        did::store_their_did(wallet.handle, &config).wait().unwrap();

        let result = did::store_their_did(wallet.handle, &config).wait();
        assert_eq!(ErrorCode::WalletItemAlreadyExists, result.unwrap_err().error_code);

        let result = did::store_their_did(wallet.handle, &config).wait();
        assert_eq!(ErrorCode::WalletItemAlreadyExists, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_get_verkey_local {
    use super::*;

    #[test]
    fn get_verkey_local_my_did() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        let (did, verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        let stored_verkey = did::key_for_local_did(wallet.handle, &did).wait().unwrap();

        assert_eq!(verkey, stored_verkey);
    }

    #[test]
    fn get_verkey_local_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        did::store_their_did(wallet.handle, &config).wait().unwrap();

        let stored_verkey = did::key_for_local_did(wallet.handle, DID_1).wait().unwrap();

        assert_eq!(VERKEY_1, stored_verkey);
    }

    #[test]
    fn get_verkey_local_invalid_did() {
        let wallet = Wallet::new();
        let result = did::key_for_local_did(wallet.handle, DID_1).wait();

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err().error_code);
    }

    #[test]
    fn get_verkey_local_invalid_wallet() {
        let result = did::key_for_local_did(INVALID_HANDLE, DID_1).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_get_verkey_ledger {
    use super::*;

    #[test]
    fn get_verkey_my_did() {
        let wallet = Wallet::new();
        let (did, verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        let stored_verkey = did::key_for_did(
            -1,
            wallet.handle,
            &did
        ).wait().unwrap();

        assert_eq!(verkey, stored_verkey);
    }

    #[test]
    fn get_verkey_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        did::store_their_did(wallet.handle, &config).wait().unwrap();

        let stored_verkey = did::key_for_did(
            -1,
            wallet.handle,
            DID_1,
        ).wait().unwrap();

        assert_eq!(VERKEY_1, stored_verkey);
    }

    #[test]
    fn get_verkey_not_on_ledger() {
        let wallet = Wallet::new();
        let wallet2 = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: true,
            num_trustees: 0,
            num_users: 0,
            num_nodes: 4
        });
        let pool_handle = setup.pool_handle.unwrap();

        let (did, _verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        let result = did::key_for_did(
            pool_handle,
            wallet2.handle,
            &did
        ).wait();

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err().error_code);
    }

    #[test]
    fn get_verkey_on_ledger() {
        let wallet = Wallet::new();
        let wallet2 = Wallet::new();
        let setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: true,
            num_trustees: 1,
            num_users: 1,
            num_nodes: 4
        });
        let pool_handle = setup.pool_handle.unwrap();
        let user = &setup.users.as_ref().unwrap()[0];

        let ledger_verkey = did::key_for_did(
            pool_handle,
            wallet2.handle,
            &user.did
        ).wait().unwrap();

        assert_eq!(ledger_verkey, user.verkey);
    }

    #[test]
    fn get_verkey_invalid_pool() {
        let wallet = Wallet::new();

        let result = did::key_for_did(-1, wallet.handle, DID_1).wait();

        assert_eq!(ErrorCode::PoolLedgerInvalidPoolHandle, result.unwrap_err().error_code);
    }

    #[test]
    fn get_verkey_invalid_wallet() {
        let result = did::key_for_did(-1, INVALID_HANDLE, DID_1).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_set_metadata {
    use super::*;

    #[inline]
    fn setup() -> (Wallet, String) {
        let wallet = Wallet::new();
        let (did, _) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        (wallet, did)
    }

    #[test]
    fn set_metadata_my_did() {
        let (wallet, did) = setup();

        let result = did::set_did_metadata(wallet.handle, &did, METADATA).wait();
        let metadata = did::get_did_metadata(wallet.handle, &did).wait().unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        did::store_their_did(wallet.handle, &config).wait().unwrap();

        let result = did::set_did_metadata(wallet.handle, DID_1, METADATA).wait();
        let metadata = did::get_did_metadata(wallet.handle, DID_1).wait().unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_replace_metadata() {
        let (wallet, did) = setup();

        did::set_did_metadata(wallet.handle, &did, METADATA).wait().unwrap();
        let metadata = did::get_did_metadata(wallet.handle, &did).wait().unwrap();

        assert_eq!(METADATA, metadata);

        let next_metadata = "replacement metadata";
        did::set_did_metadata(wallet.handle, &did, next_metadata).wait().unwrap();
        let metadata = did::get_did_metadata(wallet.handle, &did).wait().unwrap();

        assert_eq!(next_metadata, metadata);
    }

    #[test]
    fn set_metadata_empty_string() {
        let (wallet, did) = setup();

        let result = did::set_did_metadata(wallet.handle, &did, "").wait();
        let metadata = did::get_did_metadata(wallet.handle, &did).wait().unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!("", metadata);
    }

    #[test]
    fn set_metadata_invalid_did() {
        let wallet = Wallet::new();

        let result = did::set_did_metadata(wallet.handle, "InvalidDid", METADATA).wait();

        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn set_metadata_unknown_did() {
        let wallet = Wallet::new();

        let result = did::set_did_metadata(wallet.handle, DID_1, METADATA).wait();
        let metadata = did::get_did_metadata(wallet.handle, DID_1).wait().unwrap();

        assert_eq!((), result.unwrap());
        assert_eq!(METADATA, metadata);
    }

    #[test]
    fn set_metadata_invalid_wallet() {
        let result = did::set_did_metadata(INVALID_HANDLE, DID_1, METADATA).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_get_metadata {
    use super::*;

    #[inline]
    fn setup() -> (Wallet, String) {
        let wallet = Wallet::new();
        let (did, _) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        (wallet, did)
    }

    #[test]
    fn get_metadata_my_did() {
        let (wallet, did) = setup();
        did::set_did_metadata(wallet.handle, &did, METADATA).wait().unwrap();

        let result = did::get_did_metadata(wallet.handle, &did).wait();

        assert_eq!(METADATA, result.unwrap());
    }

    #[test]
    fn get_metadata_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        did::store_their_did(wallet.handle, &config).wait().unwrap();
        did::set_did_metadata(wallet.handle, DID_1, METADATA).wait().unwrap();

        let result = did::get_did_metadata(wallet.handle, DID_1).wait();

        assert_eq!(METADATA, result.unwrap());
    }

    #[test]
    fn get_metadata_empty_string() {
        let (wallet, did) = setup();
        did::set_did_metadata(wallet.handle, &did, "").wait().unwrap();

        let result = did::get_did_metadata(wallet.handle, &did).wait();

        assert_eq!(String::from(""), result.unwrap());
    }

    #[test]
    fn get_metadata_no_metadata_set() {
        let (wallet, did) = setup();

        let result = did::get_did_metadata(wallet.handle, &did).wait();

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err().error_code);
    }

    #[test]
    fn get_metadata_unknown_did() {
        let wallet = Wallet::new();

        let result = did::get_did_metadata(wallet.handle, DID_1).wait();

        assert_eq!(ErrorCode::WalletItemNotFound, result.unwrap_err().error_code);
    }

    #[test]
    fn get_metadata_invalid_wallet() {
        let result = did::get_did_metadata(INVALID_HANDLE, DID_1).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_set_endpoint {
    use super::*;

    #[test]
    pub fn set_endpoint_succeeds() {
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        indy::did::set_endpoint_for_did(wallet.handle, &did, "192.168.1.10", &verkey).wait().unwrap();

    }
}

#[cfg(test)]
mod test_get_endpoint {
    use super::*;

    #[test]
    pub fn get_endpoint_succeeds() {
        let end_point_address = "192.168.1.10";
        let wallet = Wallet::new();

        let config = json!({
            "seed": SEED_1
        }).to_string();

        let (did, verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        let pool_setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        indy::did::set_endpoint_for_did(wallet.handle, &did, end_point_address, &verkey).wait().unwrap();

        let pool_handle = indy::pool::open_pool_ledger(&pool_setup.pool_name, None).wait().unwrap();
        let mut test_succeeded : bool = false;
        let mut error_code: indy::ErrorCode = indy::ErrorCode::Success;

        match indy::did::get_endpoint_for_did(wallet.handle, pool_handle, &did).wait() {
            Ok(ret_address) => {

                let (address, _) = Some(ret_address).unwrap();

                if end_point_address.to_string() == address {
                    test_succeeded = true;
                }
            },
            Err(ec) => {
                error_code = ec.error_code;
            }
        }

        indy::pool::close_pool_ledger(pool_handle).wait().unwrap();

        if indy::ErrorCode::Success != error_code {
            assert!(false, "get_endpoint_works failed error code {:?}", error_code);
        }

        if false == test_succeeded {
            assert!(false, "get_endpoint_works failed to successfully compare end_point address");
        }
    }

    /// ----------------------------------------------------------------------------------------
    /// get_endpoint_fails_no_set doesnt call set_endpoint before calling get_endpoint.
    /// get_endpoint should return error code since the endpoint has not been set
    /// ----------------------------------------------------------------------------------------
    #[test]
    pub fn get_endpoint_fails_no_set() {
        let wallet = Wallet::new();

        let config = json!({}).to_string();

        let (did, _verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        let pool_setup = Setup::new(&wallet, SetupConfig {
            connect_to_pool: false,
            num_trustees: 0,
            num_nodes: 4,
            num_users: 0,
        });

        let pool_handle = indy::pool::open_pool_ledger(&pool_setup.pool_name, None).wait().unwrap();
        let mut error_code: indy::ErrorCode = indy::ErrorCode::Success;

        match indy::did::get_endpoint_for_did(wallet.handle, pool_handle, &did).wait() {
            Ok(_) => { },
            Err(ec) => {
                error_code = ec.error_code;
            }
        }

        indy::pool::close_pool_ledger(pool_handle).wait().unwrap();

        assert_eq!(error_code, indy::ErrorCode::CommonInvalidState);
    }
}

#[cfg(test)]
mod test_abbreviate_verkey {
    use super::*;

    #[test]
    fn abbreviate_verkey_abbreviated() {
        let result = did::abbreviate_verkey(DID_1, VERKEY_1).wait();
        assert_eq!(VERKEY_ABV_1, result.unwrap());
    }

    #[test]
    fn abbreviate_verkey_full_verkey() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1}).to_string();

        let (did, verkey) = did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        let result = did::abbreviate_verkey(&did, &verkey).wait();

        assert_eq!(verkey, result.unwrap());
    }

    #[test]
    fn abbreviate_verkey_invalid_did() {
        let result = did::abbreviate_verkey("InvalidDid", VERKEY_1).wait();
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }

    #[test]
    fn abbreviate_verkey_invalid_verkey() {
        let result = did::abbreviate_verkey(DID_1, "InvalidVerkey").wait();
        assert_eq!(ErrorCode::CommonInvalidStructure, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_list_with_metadata {
    use super::*;

    fn setup_multiple(wallet: &Wallet) -> Vec<serde_json::Value> {
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        did::store_their_did(wallet.handle, &config).wait().unwrap();
        let (did1, verkey1) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();
        let (did2, verkey2) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();
        did::set_did_metadata(wallet.handle, &did1, METADATA).wait().unwrap();

        let expected = vec![
            json!({
                "did": did1,
                "verkey": verkey1,
                "tempVerkey": null,
                "metadata": Some(METADATA.to_string()),
            }),
            json!({
                "did": did2,
                "verkey": verkey2,
                "tempVerkey": null,
                "metadata": null
            })
        ];

        expected
    }

    fn assert_multiple(json: String, expected: Vec<serde_json::Value>) {
        let dids: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();

        assert_eq!(expected.len(), dids.len());

        for did in expected {
            assert!(dids.contains(&did));
        }
    }

    #[test]
    fn list_with_metadata_no_dids() {
        let wallet = Wallet::new();

        let result = did::list_my_dids_with_metadata(wallet.handle).wait();

        assert_eq!("[]", result.unwrap());
    }

    #[test]
    fn list_with_metadata_their_did() {
        let wallet = Wallet::new();
        let config = json!({"did": DID_1, "verkey": VERKEY_1}).to_string();
        did::store_their_did(wallet.handle, &config).wait().unwrap();

        let result = did::list_my_dids_with_metadata(wallet.handle).wait();

        assert_eq!("[]", result.unwrap());
    }

    #[test]
    fn list_with_metadata_cryptonym() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1, "cid": true}).to_string();
        did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();

        let json = did::list_my_dids_with_metadata(wallet.handle).wait().unwrap();
        let dids: serde_json::Value = serde_json::from_str(&json).unwrap();

        let expected = json!([{
            "did": VERKEY_1,
            "verkey": VERKEY_1,
            "tempVerkey": null,
            "metadata": null
        }]);

        assert_eq!(expected, dids);
    }

    #[test]
    fn list_with_metadata_did_with_metadata() {
        let wallet = Wallet::new();
        let config = json!({"seed": SEED_1}).to_string();
        did::create_and_store_my_did(wallet.handle, &config).wait().unwrap();
        did::set_did_metadata(wallet.handle, DID_1, METADATA).wait().unwrap();

        let json = did::list_my_dids_with_metadata(wallet.handle).wait().unwrap();
        let dids: serde_json::Value = serde_json::from_str(&json).unwrap();

        let expected = json!([{
            "did": DID_1,
            "verkey": VERKEY_1,
            "tempVerkey": null,
            "metadata": METADATA
        }]);

        assert_eq!(expected, dids);
    }

    #[test]
    fn list_with_metadata_multiple_dids() {
        let wallet = Wallet::new();
        let expected = setup_multiple(&wallet);

        let dids = did::list_my_dids_with_metadata(wallet.handle).wait().unwrap();

        assert_multiple(dids, expected);
    }

    #[test]
    fn list_with_metadata_invalid_wallet() {
        let result = did::list_my_dids_with_metadata(INVALID_HANDLE).wait();
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err().error_code);
    }
}

#[cfg(test)]
mod test_get_my_metadata {
    use super::*;

    #[test]
    pub fn get_my_metadata_success() {
        let wallet = Wallet::new();

        let (did, _verkey) = did::create_and_store_my_did(wallet.handle, "{}").wait().unwrap();

        match did::get_my_did_with_metadata(wallet.handle, &did).wait() {
            Ok(_) => {},
            Err(ec) => {
                assert!(false, "get_my_metadata_success failed with error code {:?}", ec);
            }
        }
    }
}
