mod sqlite;

use errors::wallet::WalletError;
use services::wallet::sqlite::SqliteWallet;


trait Wallet {
    fn set(&self, keys: &[&String], value: &String) -> Result<(), WalletError>;
    fn get(&self, keys: &[&String]) -> Result<String, WalletError>;
}

trait AnoncredsWallet: Wallet {
    fn get_master_secret(&self, did: &String, schema: &String, pk: &String) -> Result<String, WalletError> {
        self.get(&[did, schema, pk])
    }
}

trait IdentityWallet: Wallet {
    fn get_key_by_did(&self, did: &String) -> Result<String, WalletError> {
        self.get(&[did])
    }
}

struct WallerService {
    wallet: Box<Wallet>
}

impl WallerService {
    fn new() -> WallerService {
        WallerService {
            wallet: Box::new(SqliteWallet::new().unwrap())
        }
    }

    fn new_inject_waller(wallet: Box<Wallet>) -> WallerService {
        WallerService {
            wallet: wallet
        }
    }
}

impl Wallet for WallerService {
    fn set(&self, keys: &[&String], value: &String) -> Result<(), WalletError> {
        self.wallet.set(keys, value)
    }

    fn get(&self, keys: &[&String]) -> Result<String, WalletError> {
        self.wallet.get(keys)
    }
}

impl AnoncredsWallet for WallerService {}

impl IdentityWallet for WallerService {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqlite_wallet_service_set_get_value_possible() {
        let wallet_service = WallerService::new();

        let (key, subkey, value) = ("key".to_string(), "subkey".to_string(), "value".to_string());

        assert!(wallet_service.set(&[&key, &subkey], &value).is_ok(), "Success set key in sqlite wallet");

        let result = wallet_service.get(&[&key, &subkey]);

        assert!(result.is_ok(), "Success get value from sqlite wallet");

        assert_eq!(value, result.unwrap(), "Get correct value by key");
    }

    #[test]
    fn sqlite_wallet_service_get_master_secret_possible() {
        let wallet_service = WallerService::new();

        let (did, schema, pk, master) = ("did".to_string(), "schema".to_string(), "pk".to_string(), "master".to_string());

        assert!(wallet_service.set(&[&did, &schema, &pk], &master).is_ok(), "Success set key in sqlite wallet");

        let result = wallet_service.get_master_secret(&did, &schema, &pk);

        assert!(result.is_ok(), "Success get value from sqlite wallet");

        assert_eq!(master, result.unwrap(), "Get correct value by key");
    }

    #[test]
    fn sqlite_wallet_service_get_key_by_did_possible() {
        let wallet_service = WallerService::new();

        let (did, master) = ("did".to_string(), "master".to_string());

        assert!(wallet_service.set(&[&did], &master).is_ok(), "Success set key in sqlite wallet");

        let result = wallet_service.get_key_by_did(&did);

        assert!(result.is_ok(), "Success get value from sqlite wallet");

        assert_eq!(master, result.unwrap(), "Get correct value by key");
    }

    #[test]
    fn sqlite_wallet_service_get_not_found_error_in_case_wrong_key() {
        let wallet_service = WallerService::new();

        let wrong_key = "wrong_key".to_string();

        assert!(wallet_service.get(&[&wrong_key]).is_err(), "Get NotFoundError in case wrong key");
    }
}