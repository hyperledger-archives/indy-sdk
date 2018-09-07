#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
extern crate rust_libindy_wrapper as indy;
#[macro_use]
mod utils;

use indy::did::Did;
use indy::ErrorCode;
use utils::b58::{FromBase58, IntoBase58};
use utils::constants::{DID_1, SEED_1, VERKEY_1};
use utils::wallet::Wallet;

#[cfg(test)]
mod create_new_did {
    use super::*;

    fn seed_config(seed: &str) -> String {
        json!({
            "seed": seed
        }).to_string()
    }

    #[test]
    fn create_did_with_empty_json() {
        let wallet = Wallet::new();

        let (did, verkey) = Did::new(wallet.handle, "{}").unwrap();

        assert_eq!(16, did.from_base58().unwrap().len());
        assert_eq!(32, verkey.from_base58().unwrap().len());
    }

    #[test]
    fn create_did_with_seed() {
        let wallet = Wallet::new();

        let config = seed_config(SEED_1);
        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

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

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        assert_eq!(VERKEY_1, did);
        assert_eq!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_did() {
        let wallet = Wallet::new();

        let config = json!({
            "did": DID_1
        }).to_string();

        let (did, verkey) = Did::new(wallet.handle, &config).unwrap();

        assert_eq!(DID_1, did);
        assert_ne!(VERKEY_1, verkey);
    }

    #[test]
    fn create_did_with_crypto_type() {
        let wallet = Wallet::new();

        let config = json!({
            "crypto_type": "ed25519"
        }).to_string();

        let result = Did::new(wallet.handle, &config);

        assert!(result.is_ok());

    }

    #[test]
    fn create_did_with_invalid_wallet_handle() {
        let result = Did::new(583741, "{}");
        assert_eq!(ErrorCode::WalletInvalidHandle, result.unwrap_err());
    }

    #[test]
    fn create_wallet_empty_config() {
        let wallet = Wallet::new();
        
        let result = Did::new(wallet.handle, "");

        assert!(result.is_err());
    }
}