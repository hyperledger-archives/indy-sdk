extern crate rusqlite;

use super::{Wallet, WalletType};

use errors::wallet::WalletError;
use utils::environment::EnvironmentUtils;

use self::rusqlite::Connection;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct DefaultWallet {
    name: String
}

impl DefaultWallet {
    fn new(name: &str) -> DefaultWallet {
        DefaultWallet {
            name: name.to_string()
        }
    }
}

impl Wallet for DefaultWallet {
    fn set(&self, key: &str, value: &str) -> Result<(), WalletError> {
        _open_connection(self.name.as_str())?
            .execute(
                "INSERT OR REPLACE INTO wallet (key, value) VALUES (?1, ?2)",
                &[&key.to_string(), &value.to_string()])?;
        Ok(())
    }

    fn get(&self, key: &str) -> Result<String, WalletError> {
        Ok(_open_connection(self.name.as_str())?
            .query_row(
                "SELECT value FROM wallet WHERE key = ?1 LIMIT 1",
                &[&key.to_string()],
                |row| row.get(0))?)
    }
}

pub struct DefaultWalletType {}

impl DefaultWalletType {
    pub fn new() -> DefaultWalletType {
        DefaultWalletType {}
    }
}

impl WalletType for DefaultWalletType {
    fn create(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletError> {
        let path = _db_path(name);
        if path.exists() {
            return Err(WalletError::AlreadyExists(name.to_string()))
        }

        _open_connection(name)?
            .execute("CREATE TABLE wallet (key TEXT CONSTRAINT constraint_name PRIMARY KEY, value TEXT, timestamp DATETIME DEFAULT CURRENT_TIMESTAMP)", &[])?;
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), WalletError> {
        Ok(fs::remove_file(_db_path(name))?)
    }

    fn open(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<Box<Wallet>, WalletError> {
        Ok(Box::new(DefaultWallet::new(name)))
    }
}

fn _db_path(name: &str) -> PathBuf {
    let mut path = EnvironmentUtils::wallet_path(name);
    path.push("sqlite.db");
    path
}

fn _open_connection(name: &str) -> Result<Connection, WalletError> {
    let path = _db_path(name);
    if !path.parent().unwrap().exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(path.parent().unwrap())?;
    }

    Ok(Connection::open(path)?)
}

impl From<rusqlite::Error> for WalletError {
    fn from(err: rusqlite::Error) -> WalletError {
        match err {
            rusqlite::Error::QueryReturnedNoRows => WalletError::NotFound(err.description().to_string()),
            _ => WalletError::BackendError(err.description().to_string())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use errors::wallet::WalletError;
    use utils::test::TestUtils;

    #[test]
    fn type_new_works() {
        DefaultWalletType::new();
    }

    #[test]
    fn type_create_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn type_create_twice_fails() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();

        let res = wallet_type.create("wallet1", None, None);
        assert_match!(res, Err(WalletError::AlreadyExists(_)));

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn type_delete_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        wallet_type.delete("wallet1").unwrap();
        wallet_type.create("wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn type_open_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        wallet_type.open("wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_set_get_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", None, None).unwrap();

        wallet.set("key1", "value1").unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_set_get_works_for_reopen() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();

        {
            let wallet = wallet_type.open("wallet1", None, None).unwrap();
            wallet.set("key1", "value1").unwrap();
        }

        let wallet = wallet_type.open("wallet1", None, None).unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_get_works_for_unknown() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();

        let wallet = wallet_type.open("wallet1", None, None).unwrap();
        let value = wallet.get("key1");
        assert_match!(value,  Err(WalletError::NotFound(_)));

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_set_get_works_for_update() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", None, None).unwrap();

        wallet.set("key1", "value1").unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value1", value);

        wallet.set("key1", "value2").unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value2", value);

        TestUtils::cleanup_sovrin_home();
    }
}