mod sqlite;

use errors::wallet::WalletError;
use services::wallet::sqlite::SqliteWallet;


pub trait Wallet {
    fn set(&self, keys: &[&String], value: &String) -> Result<(), WalletError>;
    fn get(&self, keys: &[&String]) -> Result<Option<String>, WalletError>;
}

pub trait AnoncredsWallet: Wallet {
    fn get_master_secret(&self, did: &String, schema: &String, pk: &String) -> Result<Option<String>, WalletError> {
        self.get((vec![did, schema, pk]).as_slice())
    }
}

pub trait IdentityWallet: Wallet {
    fn get_key_by_did(&self, did: &String) -> Result<Option<String>, WalletError> {
        self.get((vec![did]).as_slice())
    }
}

pub struct WalletService {
    wallet: Box<Wallet>
}

impl WalletService {
    pub fn new() -> WalletService {
        WalletService {
            wallet: Box::new(SqliteWallet::new().unwrap())
        }
    }

    pub fn new_inject_waller(wallet: Box<Wallet>) -> WalletService {
        WalletService {
            wallet: wallet
        }
    }
}

impl Wallet for WalletService {
    fn set(&self, keys: &[&String], value: &String) -> Result<(), WalletError> {
        self.wallet.set(keys, value)
    }

    fn get(&self, keys: &[&String]) -> Result<Option<String>, WalletError> {
        self.wallet.get(keys)
    }
}

impl AnoncredsWallet for WalletService {}

impl IdentityWallet for WalletService {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqlite_wallet_service_set_get_value_possible() {
        let wallet_service = WalletService::new();

        let (key, subkey, value) = ("key".to_string(), "subkey".to_string(), "value".to_string());

        assert!(wallet_service.set(&[&key, &subkey], &value).is_ok(), "Success set key in sqlite wallet");

        let result = wallet_service.get(&[&key, &subkey]);

        assert!(result.is_ok(), "Success get value from sqlite wallet");

        let received_value = result.unwrap();

        assert!(received_value.is_some(), "Success get value from sqlite wallet");

        assert_eq!(value, received_value.unwrap(), "Get correct value by key");
    }

    #[test]
    fn sqlite_wallet_service_get_master_secret_possible() {
        let wallet_service = WalletService::new();

        let (did, schema, pk, master) = ("did".to_string(), "schema".to_string(), "pk".to_string(), "master".to_string());

        assert!(wallet_service.set(&[&did, &schema, &pk], &master).is_ok(), "Success set key in sqlite wallet");

        let result = wallet_service.get_master_secret(&did, &schema, &pk);

        assert!(result.is_ok(), "Success get value from sqlite wallet");

        let received_value = result.unwrap();

        assert!(received_value.is_some(), "Success get value from sqlite wallet");

        assert_eq!(master, received_value.unwrap(), "Get correct value by key");
    }

    #[test]
    fn sqlite_wallet_service_get_key_by_did_possible() {
        let wallet_service = WalletService::new();

        let (did, master) = ("did".to_string(), "master".to_string());

        let result = wallet_service.get_key_by_did(&did);

        assert!(result.is_ok(), "Success set key in sqlite wallet");

        let received_value = result.unwrap();

        assert!(received_value.is_some(), "Success get value from sqlite wallet");

        assert_eq!(master, received_value.unwrap(), "Get correct value by key");
    }

    #[test]
    fn sqlite_wallet_service_get_not_found_error_in_case_wrong_key() {
        let wallet_service = WalletService::new();

        let wrong_key = "wrong_key".to_string();

        let result = wallet_service.get(&[&wrong_key]);

        assert!(result.is_ok(), "Success get value from sqlite wallet");

        let received_value = result.unwrap();

        assert!(received_value.is_none(), "Get None in case wrong key");
    }
}