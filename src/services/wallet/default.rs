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
        _open_connection(name)?
            .execute("CREATE TABLE wallet (key TEXT, value TEXT)", &[])?;
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
    path.set_file_name("sqlite.db");
    path
}

fn _open_connection(name: &str) -> Result<Connection, WalletError> {
    Ok(Connection::open(_db_path(name))?)
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

    #[test]
    fn type_new_works() {
        DefaultWalletType::new();
    }

    #[test]
    #[ignore]
    fn type_create_works() {
        let wallet_type = DefaultWalletType::new();
        let res = wallet_type.create("wallet1", None, None);
        res.unwrap();
    }
}