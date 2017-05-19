extern crate rusqlite;
extern crate time;

use super::{Wallet, WalletType};

use errors::wallet::WalletError;
use utils::environment::EnvironmentUtils;
use utils::json::JsonDecodable;

use self::rusqlite::Connection;
use self::time::Timespec;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::ops::Sub;

#[derive(Deserialize)]
struct DefaultWalletRuntimeConfig {
    freshness_time: i64
}

impl<'a> JsonDecodable<'a> for DefaultWalletRuntimeConfig {}

impl Default for DefaultWalletRuntimeConfig {
    fn default() -> Self {
        DefaultWalletRuntimeConfig { freshness_time: 1000 }
    }
}

#[derive(Deserialize)]
struct DefaultWalletCredentials {}

impl<'a> JsonDecodable<'a> for DefaultWalletCredentials {}

struct DefaultWalletRecord {
    key: String,
    value: String,
    time_created: Timespec
}

struct DefaultWallet {
    name: String,
    pool_name: String,
    config: DefaultWalletRuntimeConfig
}

impl DefaultWallet {
    fn new(name: &str,
           pool_name: &str,
           config: DefaultWalletRuntimeConfig,
           credentials: DefaultWalletCredentials) -> DefaultWallet {
        DefaultWallet {
            name: name.to_string(),
            pool_name: pool_name.to_string(),
            config: config
        }
    }
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
                    DefaultWalletRecord {
                        key: row.get(0),
                        value: row.get(1),
                        time_created: row.get(2)
                    }
                })?;
        Ok(record.value)
    }

    fn list(&self, key_prefix: &str) -> Result<Vec<(String, String)>, WalletError> {
        let connection = _open_connection(self.name.as_str())?;
        let mut stmt = connection.prepare("SELECT key, value, time_created FROM wallet WHERE key like ?1 order by key")?;
        let records = stmt.query_map(&[&format!("{}%", key_prefix)], |row| {
            DefaultWalletRecord {
                key: row.get(0),
                value: row.get(1),
                time_created: row.get(2)
            }
        })?;

        let mut key_values = Vec::new();

        for record in records {
            let key_value = record?;
            key_values.push((key_value.key, key_value.value));
        }

        Ok(key_values)
    }

    fn get_not_expired(&self, key: &str) -> Result<String, WalletError> {
        let record = _open_connection(self.name.as_str())?
            .query_row(
                "SELECT key, value, time_created FROM wallet WHERE key = ?1 LIMIT 1",
                &[&key.to_string()], |row| {
                    DefaultWalletRecord {
                        key: row.get(0),
                        value: row.get(1),
                        time_created: row.get(2)
                    }
                })?;

        if self.config.freshness_time != 0
            && time::get_time().sub(record.time_created).num_seconds() > self.config.freshness_time {
            return Err(WalletError::NotFound(key.to_string()))
        }

        return Ok(record.value)
    }

    fn get_pool_name(&self) -> String {
        self.pool_name.clone()
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

    fn delete(&self, name: &str, credentials: Option<&str>) -> Result<(), WalletError> {
        // FIXME: parse and implement credentials!!!
        Ok(fs::remove_file(_db_path(name))?)
    }

    fn open(&self, name: &str, pool_name: &str, config: Option<&str>, runtime_config: Option<&str>, credentials: Option<&str>) -> Result<Box<Wallet>, WalletError> {
        let runtime_config = match runtime_config {
            Some(config) => DefaultWalletRuntimeConfig::from_json(config)?,
            None => DefaultWalletRuntimeConfig::default()
        };

        // FIXME: parse and implement credentials!!!
        Ok(Box::new(
            DefaultWallet::new(
                name,
                pool_name,
                runtime_config,
                DefaultWalletCredentials {})))
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
        assert_match!(Err(WalletError::AlreadyExists(_)), res);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn type_delete_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        wallet_type.delete("wallet1", None).unwrap();
        wallet_type.create("wallet1", None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn type_open_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_set_get_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

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
            let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
            wallet.set("key1", "value1").unwrap();
        }

        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
        let value = wallet.get("key1").unwrap();
        assert_eq!("value1", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_get_works_for_unknown() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();

        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();
        let value = wallet.get("key1");
        assert_match!(Err(WalletError::NotFound(_)), value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_set_get_works_for_update() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

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
        let wallet = wallet_type.open("wallet1", "pool1", None, Some("{\"freshness_time\": 1}"), None).unwrap();
        wallet.set("key1", "value1").unwrap();

        // Wait until value expires
        thread::sleep(Duration::new(2, 0));

        let value = wallet.get_not_expired("key1");
        assert_match!(Err(WalletError::NotFound(_)), value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn wallet_list_works() {
        TestUtils::cleanup_sovrin_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create("wallet1", None, None).unwrap();
        let wallet = wallet_type.open("wallet1", "pool1", None, None, None).unwrap();

        wallet.set("key1::subkey1", "value1").unwrap();
        wallet.set("key1::subkey2", "value2").unwrap();

        let mut key_values = wallet.list("key1::").unwrap();
        assert_eq!(2, key_values.len());

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey2", key);
        assert_eq!("value2", value);

        let (key, value) = key_values.pop().unwrap();
        assert_eq!("key1::subkey1", key);
        assert_eq!("value1", value);

        TestUtils::cleanup_sovrin_home();
    }

    #[test]
    fn default_wallet_get_pool_name_works() {
        TestUtils::cleanup_sovrin_home();

        let pool_name = "pool1";
        let wallet_name = "wallet1";
        let default_wallet_type = DefaultWalletType::new();
        default_wallet_type.create(wallet_name, None, None).unwrap();
        let wallet = default_wallet_type.open(wallet_name, pool_name, None, None, None).unwrap();

        assert_eq!(wallet.get_pool_name(), pool_name);

        TestUtils::cleanup_sovrin_home();
    }
}