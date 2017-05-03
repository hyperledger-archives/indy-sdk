extern crate rusqlite;
extern crate time;

use super::{Wallet, WalletType};

use errors::wallet::WalletError;
use utils::environment::EnvironmentUtils;

use self::rusqlite::Connection;
use self::time::Timespec;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::ops::Sub;

struct DefaultWallet {
    name: String,
    freshness_time: i64
}

impl DefaultWallet {
    fn new(name: &str, freshness_time: i64) -> DefaultWallet {
        DefaultWallet {
            name: name.to_string(),
            freshness_time: freshness_time
        }
    }
}

struct WalletRecord {
    key: String,
    value: String,
    time_created: Timespec
}

struct WalletConfig {
    freshness_time: i64
}

impl Wallet for DefaultWallet {
    fn set(&self, key: &str, value: &str) -> Result<(), WalletError> {
        _open_connection(self.name.as_str())?
            .execute(
                "INSERT OR REPLACE INTO wallet (key, value, time_created) VALUES (?1, ?2, ?3)",
                &[&key.to_string(), &value.to_string(), &time::get_time()])?;
        Ok(())
    }

    fn get(&self, key: &str) -> Result<String, WalletError> {
        let record = _open_connection(self.name.as_str())?
            .query_row(
                "SELECT key, value, time_created FROM wallet WHERE key = ?1 LIMIT 1",
                &[&key.to_string()], |row| {
                    WalletRecord {
                        key: row.get(0),
                        value: row.get(1),
                        time_created: row.get(2)
                    }
                })?;
        Ok(record.value)
    }

    fn get_not_expired(&self, key: &str) -> Result<String, WalletError> {
        let record = _open_connection(self.name.as_str())?
            .query_row(
                "SELECT key, value, time_created FROM wallet WHERE key = ?1 LIMIT 1",
                &[&key.to_string()], |row| {
                    WalletRecord {
                        key: row.get(0),
                        value: row.get(1),
                        time_created: row.get(2)
                    }
                })?;

        if self.freshness_time != 0
            && time::get_time().sub(record.time_created).num_seconds() > self.freshness_time {
            return Err(WalletError::NotFound(key.to_string()))
        }

        return Ok(record.value)
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
            .execute("CREATE TABLE wallet (key TEXT CONSTRAINT constraint_name PRIMARY KEY, value TEXT NOT NULL, time_created TEXT NOT_NULL)", &[])?;
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), WalletError> {
        Ok(fs::remove_file(_db_path(name))?)
    }

    fn open(&self, name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<Box<Wallet>, WalletError> {
        // FIXME: parse config for freshness_time!!!
        Ok(Box::new(DefaultWallet::new(name, 10000)))
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

    use std::time::{Duration};
    use std::thread;

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
    fn type_create_works_for_twice() {
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

    #[test]
    fn wallet_set_get_not_expired_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", None, None).unwrap();
        wallet.set("key1", "value1").unwrap();

        // Wait until value expires
        thread::sleep(Duration::new(3, 0));

        let value = wallet.get_not_expired("key1");
        assert_match!(value,  Err(WalletError::NotFound(_)));

        TestUtils::cleanup_sovrin_home();
    }
}