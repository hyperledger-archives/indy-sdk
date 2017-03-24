use std::error::Error;

mod sqllite;

pub struct WalletService<T: WalletServiceImpl + AnnonCredsWalletServiceImpl + IdentityWalletServiceImpl> {
    service_impl: T
}

impl<T: WalletServiceImpl + AnnonCredsWalletServiceImpl + IdentityWalletServiceImpl> WalletService<T> {
    pub fn new(service_impl: T) -> WalletService<T> {
        trace!(target: "WalletService", "new");
        WalletService { service_impl: service_impl }
    }
}

pub trait WalletServiceImpl {
    fn set(&self, key: &[String], value: String) -> Result<(), Box<Error>>;
    fn get(&self, key: &[String]) -> Option<String>;
}

pub trait AnnonCredsWalletServiceImpl {
    fn get_master_secret(&self, did: String, schema: String, pk: String) -> Option<String>;
}

pub trait IdentityWalletServiceImpl {
    fn get_key_by_did(&self, did: String) -> Option<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_service_set_get_value_possible() {
        let sqlite_service = sqllite::SqliteWalletService::new();
        let wallet_service = WalletService::new(sqlite_service);

        let (key, subkey, value) = ("key", "subkey", "value");

        wallet_service.service_impl.set(
            &[format!("{}", &key), format!("{}", &subkey)],
            format!("{}", &value)
        );

        let mut result =
            wallet_service.service_impl.get(
                &[format!("{}", &key),
                    format!("{}", &subkey)]
            );

        assert_eq!(Some(format!("{}", &value)), result, "Get value by key");
    }

    #[test]
    fn wallet_service_get_master_secret_possible() {
        let sqlite_service = sqllite::SqliteWalletService::new();
        let wallet_service = WalletService::new(sqlite_service);

        let (did, schema, pk, master) = ("did", "schema", "pk", "master");

        wallet_service.service_impl.set(
            &[format!("{}", &did), format!("{}", &schema), format!("{}", &pk)],
            format!("{}", &master)
        );

        let result = wallet_service.service_impl.get_master_secret(
            format!("{}", &did),
            format!("{}", &schema),
            format!("{}", &pk)
        );

        assert_eq!(Some(format!("{}", &master)), result, "Get master secret");
    }

    #[test]
    fn wallet_service_get_key_by_did_possible() {
        let sqlite_service = sqllite::SqliteWalletService::new();
        let wallet_service = WalletService::new(sqlite_service);

        let (key, did) = ("key", "did");

        wallet_service.service_impl.set(
            &[format!("{}", &did)],
            format!("{}", &key)
        );

        let result = wallet_service.service_impl.get_key_by_did(format!("{}", &did));

        assert_eq!(Some(format!("{}", &key)), result, "Get key by did");
    }

    #[test]
    fn wallet_service_get_none_in_case_wrong_key() {
        let sqlite_service = sqllite::SqliteWalletService::new();
        let wallet_service = WalletService::new(sqlite_service);

        let result = wallet_service.service_impl.get_key_by_did("wrong".to_string());

        assert_eq!(None, result, "Get None in case wrong key");
    }
}