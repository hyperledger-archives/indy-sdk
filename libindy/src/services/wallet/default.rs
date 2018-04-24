extern crate rusqlcipher;
extern crate time;
extern crate indy_crypto;
extern crate serde_json;

use super::{WalletStorage, WalletStorageType, WalletRecord, RecordOptions, WalletSearch, SearchOptions};

use errors::common::CommonError;
use errors::wallet::WalletError;
use utils::environment::EnvironmentUtils;

use self::rusqlcipher::Connection;

use std::error::Error;
use std::fs;
use std::path::PathBuf;


use self::indy_crypto::utils::json::JsonDecodable;

#[derive(Deserialize)]
struct DefaultWalletRuntimeConfig {}

impl Default for DefaultWalletRuntimeConfig {
    fn default() -> Self { DefaultWalletRuntimeConfig {} }
}

impl<'a> JsonDecodable<'a> for DefaultWalletRuntimeConfig {}

#[derive(Deserialize, Debug)]
struct DefaultWalletCredentials {
    key: String,
    rekey: Option<String>
}

impl<'a> JsonDecodable<'a> for DefaultWalletCredentials {}

impl Default for DefaultWalletCredentials {
    fn default() -> Self {
        DefaultWalletCredentials { key: String::new(), rekey: None }
    }
}

struct DefaultWallet {
    name: String,
    pool_name: String,
    config: DefaultWalletRuntimeConfig,
    credentials: DefaultWalletCredentials
}

impl DefaultWallet {
    fn new(name: &str,
           pool_name: &str,
           config: DefaultWalletRuntimeConfig,
           credentials: DefaultWalletCredentials) -> DefaultWallet {
        DefaultWallet {
            name: name.to_string(),
            pool_name: pool_name.to_string(),
            config,
            credentials
        }
    }
}

impl WalletStorage for DefaultWallet {
    fn add_record(&self, type_: &str, id: &str, value: &str, tags_json: &str) -> Result<(), WalletError> {
        if self.credentials.rekey.is_some() {
            return Err(WalletError::CommonError(CommonError::InvalidStructure(format!("Invalid wallet credentials json"))));
        }

        _open_connection(self.name.as_str(), &self.credentials)?
            .execute(
                "INSERT INTO wallet (id, type, value, tags) VALUES (?1, ?2, ?3, ?4)",
                &[&id.to_string(), &type_.to_string(), &value.to_string(), &tags_json.to_string()])?;

        Ok(())
    }

    fn close_wallet(&self) -> Result<(), WalletError> { Ok(()) }

    fn get_pool_name(&self) -> String {
        self.pool_name.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn update_record_value(&self, type_: &str, id: &str, value: &str) -> Result<(), WalletError> {
        _open_connection(self.name.as_str(), &self.credentials)?
            .execute(
                "UPDATE wallet SET value = ?1 WHERE id = ?2 AND type = ?3",
                &[&value.to_string(), &id.to_string(), &type_.to_string()])?;
        Ok(())
    }

    fn update_record_tags(&self, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError> {
        _open_connection(self.name.as_str(), &self.credentials)?
            .execute(
                "UPDATE wallet SET tags = ?1 WHERE id = ?2 AND type = ?3",
                &[&tags_json.to_string(), &id.to_string(), &type_.to_string()])?;
        Ok(())
    }

    fn add_record_tags(&self, type_: &str, id: &str, tags_json: &str) -> Result<(), WalletError> {
        _open_connection(self.name.as_str(), &self.credentials)?
            .execute(
                "UPDATE wallet SET tags = ?1 WHERE id = ?2 AND type = ?3",
                &[&tags_json.to_string(), &id.to_string(), &type_.to_string()])?;
        Ok(())
    }

    fn delete_record_tags(&self, type_: &str, id: &str, tag_names_json: &str) -> Result<(), WalletError> {
        _open_connection(self.name.as_str(), &self.credentials)?
            .execute(
                "UPDATE wallet SET tags = null WHERE id = ?1 AND type = ?2",
                &[&id.to_string(), &type_.to_string()])?;
        Ok(())
    }

    fn delete_record(&self, type_: &str, id: &str) -> Result<(), WalletError> {
        _open_connection(self.name.as_str(), &self.credentials)?
            .execute(
                "DELETE FROM wallet WHERE id = ?1 AND type = ?2",
                &[&id.to_string(), &type_.to_string()])?;
        Ok(())
    }

    fn get_record(&self, type_: &str, id: &str, options_json: &str) -> Result<WalletRecord, WalletError> {
        let options = RecordOptions::from_json(options_json)
            .map_err(|err|
                WalletError::CommonError(
                    CommonError::InvalidStructure(format!("Cannot deserialize RecordRetrieveOptions: {:?}", err))))?;

        let mut columns: Vec<&'static str> = vec!["id"];

        if Some(true) == options.retrieve_type {
            columns.push("type")
        }
        if Some(false) != options.retrieve_value {
            columns.push("value")
        }
        if Some(false) != options.retrieve_tags {
            columns.push("tags")
        }

        let columns_str = columns.join(", ").to_string();

        let record = _open_connection(self.name.as_str(), &self.credentials)?
            .query_row(
                &format!("SELECT {} FROM wallet WHERE id = ?1 AND type = ?2 LIMIT 1", columns_str),
                &[&id.to_string(), &type_.to_string()], |row| {
                    WalletRecord {
                        id: row.get("id"),
                        type_: if columns.contains(&"type") { row.get("type") } else { None },
                        value: if columns.contains(&"value") { row.get("value") } else { None },
                        tags: if columns.contains(&"tags") { row.get("tags") } else { None },
                    }
                })?;
        Ok(record)
    }

    fn search_records(&self, type_: &str, query_json: &str, options_json: &str) -> Result<WalletSearch, WalletError> {
        let options = SearchOptions::from_json(options_json)
            .map_err(|err|
                WalletError::CommonError(
                    CommonError::InvalidStructure(format!("Cannot deserialize RecordRetrieveOptions: {:?}", err))))?;

        let mut columns: Vec<&'static str> = vec!["id"];

        if Some(true) == options.retrieve_type {
            columns.push("type")
        }
        if Some(false) != options.retrieve_value {
            columns.push("value")
        }
        if Some(false) != options.retrieve_tags {
            columns.push("tags")
        }

        let columns_str = columns.join(", ").to_string();

        let connection = _open_connection(self.name.as_str(), &self.credentials)?;
        let mut stmt = connection.prepare(&format!("SELECT {} FROM wallet WHERE type = ?1", columns_str))?;
        let records = stmt.query_map(&[&type_.to_string()], |row| {
            WalletRecord {
                id: row.get("id"),
                type_: if columns.contains(&"type") { row.get("type") } else { None },
                value: if columns.contains(&"value") { row.get("value") } else { None },
                tags: if columns.contains(&"tags") { row.get("tags") } else { None }
            }
        })?;

        let mut wallet_records: Vec<WalletRecord> = Vec::new();

        for record in records {
            wallet_records.push(record?);
        }

        let wallet_search = WalletSearch {
            total_count: Some(wallet_records.len()),
            iter: Some(Box::new(wallet_records.into_iter()))
        };

        Ok(wallet_search)
    }

    fn search_all_records(&self) -> Result<WalletSearch, WalletError> {
        let connection = _open_connection(self.name.as_str(), &self.credentials)?;
        let mut stmt = connection.prepare("SELECT id, type, value, tags FROM wallet")?;
        let records = stmt.query_map(&[], |row| {
            WalletRecord {
                id: row.get(0),
                type_: row.get(1),
                value: row.get(2),
                tags: row.get(3)
            }
        })?;

        let mut wallet_records: Vec<WalletRecord> = Vec::new();

        for record in records {
            wallet_records.push(record?);
        }

        let wallet_search = WalletSearch {
            total_count: Some(wallet_records.len()),
            iter: Some(Box::new(wallet_records.into_iter()))
        };

        Ok(wallet_search)
    }

    fn close_search(&self, search_handle: u32) -> Result<(), WalletError> { Ok(()) }

}

pub struct DefaultWalletType {}

impl DefaultWalletType {
    pub fn new() -> DefaultWalletType {
        DefaultWalletType {}
    }
}

impl WalletStorageType for DefaultWalletType {
    fn create_wallet(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<(), WalletError> {
        trace!("DefaultWalletType.create >> {}, with config {:?} and credentials {:?}", name, config, credentials);
        let path = _db_path(name);
        if path.exists() {
            trace!("DefaultWalletType.create << path exists");
            return Err(WalletError::AlreadyExists(name.to_string()));
        }

        let runtime_auth = DefaultWalletCredentials::from_json(credentials)?;

        if runtime_auth.rekey.is_some() {
            return Err(WalletError::CommonError(CommonError::InvalidStructure(format!("Invalid wallet credentials json"))));
        }

        _open_connection(name, &runtime_auth).map_err(map_err_trace!())?
            .execute("CREATE TABLE wallet (id TEXT NOT NULL, type TEXT NOT NULL, value TEXT NOT NULL, tags TEXT, PRIMARY KEY (id, type))", &[])
            .map_err(map_err_trace!())?;

        trace!("DefaultWalletType.create <<");
        Ok(())
    }

    fn delete_wallet(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<(), WalletError> {
        trace!("DefaultWalletType.delete {}, with config {:?} and credentials {:?}", name, config, credentials);
        // FIXME: parse and implement credentials!!!
        Ok(fs::remove_file(_db_path(name)).map_err(map_err_trace!())?)
    }

    fn open_wallet(&self, name: &str, pool_name: &str, _config: Option<&str>, runtime_config: Option<&str>, credentials: &str) -> Result<Box<WalletStorage>, WalletError> {
        let runtime_config = match runtime_config {
            Some(config) => DefaultWalletRuntimeConfig::from_json(config)?,
            None => DefaultWalletRuntimeConfig::default()
        };

        let mut runtime_auth = DefaultWalletCredentials::from_json(credentials)?;

        _open_connection(name, &runtime_auth).map_err(map_err_trace!())?
            .query_row("SELECT sql FROM sqlite_master", &[], |_| {})
            .map_err(map_err_trace!())?;

        if let Some(rekey) = runtime_auth.rekey {
            runtime_auth.key = rekey;
            runtime_auth.rekey = None;
        }

        Ok(Box::new(
            DefaultWallet::new(
                name,
                pool_name,
                runtime_config,
                runtime_auth)))
    }
}

fn _db_path(name: &str) -> PathBuf {
    let mut path = EnvironmentUtils::wallet_path(name);
    path.push("sqlite.db");
    path
}

fn _open_connection(name: &str, credentials: &DefaultWalletCredentials) -> Result<Connection, WalletError> {
    let path = _db_path(name);
    if !path.parent().unwrap().exists() {
        fs::DirBuilder::new()
            .recursive(true)
            .create(path.parent().unwrap())?;
    }

    let conn = Connection::open(path)?;
    conn.execute(&format!("PRAGMA key='{}'", credentials.key), &[])?;

    match credentials.rekey {
        None => Ok(conn),
        Some(ref rk) => {
            if credentials.key.len() == 0 && rk.len() > 0 {
                _export_unencrypted_to_encrypted(conn, name, &rk)
            } else if rk.len() > 0 {
                conn.execute(&format!("PRAGMA rekey='{}'", rk), &[])?;
                Ok(conn)
            } else {
                _export_encrypted_to_unencrypted(conn, name)
            }
        }
    }
}

fn _export_encrypted_to_unencrypted(conn: Connection, name: &str) -> Result<Connection, WalletError> {
    let mut path = EnvironmentUtils::wallet_path(name);
    path.push("plaintext.db");

    conn.execute(&format!("ATTACH DATABASE {:?} AS plaintext KEY ''", path), &[])?;
    conn.query_row(&"SELECT sqlcipher_export('plaintext')", &[], |_| {})?;
    conn.execute(&"DETACH DATABASE plaintext", &[])?;
    let r = conn.close();
    if let Err((_, w)) = r {
        Err(WalletError::from(w))
    } else {
        let wallet = _db_path(name);
        fs::remove_file(&wallet)?;
        fs::rename(&path, &wallet)?;

        Ok(Connection::open(wallet)?)
    }
}

fn _export_unencrypted_to_encrypted(conn: Connection, name: &str, key: &str) -> Result<Connection, WalletError> {
    let mut path = EnvironmentUtils::wallet_path(name);
    path.push("encrypted.db");

    let sql = format!("ATTACH DATABASE {:?} AS encrypted KEY '{}'", path, key);
    conn.execute(&sql, &[])?;
    conn.query_row(&"SELECT sqlcipher_export('encrypted')", &[], |_| {})?;
    conn.execute(&"DETACH DATABASE encrypted", &[])?;
    let r = conn.close();
    if let Err((_, w)) = r {
        Err(WalletError::from(w))
    } else {
        let wallet = _db_path(name);
        fs::remove_file(&wallet)?;
        fs::rename(&path, &wallet)?;

        let new = Connection::open(wallet)?;
        new.execute(&format!("PRAGMA key='{}'", key), &[])?;
        Ok(new)
    }
}

impl From<rusqlcipher::Error> for WalletError {
    fn from(err: rusqlcipher::Error) -> WalletError {
        match err {
            rusqlcipher::Error::QueryReturnedNoRows => WalletError::NotFound(format!("Wallet record is not found: {}", err.description())),
            rusqlcipher::Error::SqliteFailure(err, _) if err.code == rusqlcipher::ErrorCode::NotADatabase =>
                WalletError::AccessFailed(format!("Wallet security error: {}", err.description())),
            _ => WalletError::CommonError(CommonError::InvalidState(format!("Unexpected SQLite error: {}", err.description())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use errors::wallet::WalletError;
    use utils::test::TestUtils;

    use serde_json;
    use self::serde_json::Error as JsonError;

    use std::time::Duration;
    use std::thread;

    #[test]
    fn default_wallet_type_new_works() {
        DefaultWalletType::new();
    }

    #[test]
    fn default_wallet_type_create_works() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_type_create_works_for_twice() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();

        let res = wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#);
        assert_match!(Err(WalletError::AlreadyExists(_)), res);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_type_delete_works() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        wallet_type.delete_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_type_open_works() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_set_get_works() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        let wallet = wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();

        wallet.add_record("type1", "key1", "value1", "{}").unwrap();
        let value = wallet.get_record("type1", "key1", "{}").unwrap();
        assert_eq!("value1", value.get_value().unwrap());

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_set_get_works_for_reopen() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();

        {
            let wallet = wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();
            wallet.add_record("type1", "key1", "value1", "{}").unwrap();
        }

        let wallet = wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();
        let value = wallet.get_record("type1", "key1", "{}").unwrap();
        assert_eq!("value1", value.get_value().unwrap());

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_get_works_for_unknown() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();

        let wallet = wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();
        let search = wallet.get_record("type1", "key1", "{}");
        assert_match!(Err(WalletError::NotFound(_)), search);

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_update_record_works() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        let wallet = wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();

        wallet.add_record("type1", "key1", "value1", "{}").unwrap();
        let value = wallet.get_record("type1", "key1", "{}").unwrap();
        assert_eq!("value1", value.get_value().unwrap());

        wallet.update_record_value("type1", "key1", "value2").unwrap();
        let value = wallet.get_record("type1", "key1", "{}").unwrap();
        assert_eq!("value2", value.get_value().unwrap());

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_list_works() {
        TestUtils::cleanup_indy_home();

        let wallet_type = DefaultWalletType::new();
        wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        let wallet = wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();

        wallet.add_record("type1", "key1", "value1", "{}").unwrap();
        wallet.add_record("type1", "key2", "value2", "{}").unwrap();

        let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
        assert_eq!(2, search.get_total_count().unwrap().unwrap());

        let record = search.fetch_next_record().unwrap().unwrap();
        assert_eq!("key1", record.get_id());
        assert_eq!("value1", record.get_value().unwrap());

        let record = search.fetch_next_record().unwrap().unwrap();
        assert_eq!("key2", record.get_id());
        assert_eq!("value2", record.get_value().unwrap());

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_get_pool_name_works() {
        TestUtils::cleanup_indy_home();

        let default_wallet_type = DefaultWalletType::new();
        default_wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        let wallet = default_wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();

        assert_eq!(wallet.get_pool_name(), "pool1");

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_get_name_works() {
        TestUtils::cleanup_indy_home();

        let default_wallet_type = DefaultWalletType::new();
        default_wallet_type.create_wallet("wallet1", None, r#"{"key":"key"}"#).unwrap();
        let wallet = default_wallet_type.open_wallet("wallet1", "pool1", None, None, r#"{"key":"key"}"#).unwrap();

        assert_eq!(wallet.get_name(), "wallet1");

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_credentials_deserialize() {
        let empty: Result<DefaultWalletCredentials, JsonError> = serde_json::from_str(r#"{}"#);
        assert!(empty.is_err());

        let one: Result<DefaultWalletCredentials, JsonError> = serde_json::from_str(r#"{"key":""}"#);
        assert!(one.is_ok());
        let rone = one.unwrap();
        assert_eq!(rone.key, "");
        assert_eq!(rone.rekey, None);

        let two: Result<DefaultWalletCredentials, JsonError> = serde_json::from_str(r#"{"key":"thisisatest","rekey":null}"#);
        assert!(two.is_ok());
        let rtwo = two.unwrap();
        assert_eq!(rtwo.key, "thisisatest");
        assert_eq!(rtwo.rekey, None);

        let three: Result<DefaultWalletCredentials, JsonError> = serde_json::from_str(r#"{"key":"","rekey":"thisismynewpassword"}"#);
        assert!(three.is_ok());
        let rthree = three.unwrap();
        assert_eq!(rthree.key, "");
        assert_eq!(rthree.rekey, Some("thisismynewpassword".to_string()));

        let four: Result<DefaultWalletCredentials, JsonError> = serde_json::from_str(r#"{"key": "", "rekey": ""}"#);
        assert!(four.is_ok());
        let rfour = four.unwrap();
        assert_eq!(rfour.key, "");
        assert_eq!(rfour.rekey, Some("".to_string()));
    }

    #[test]
    fn default_wallet_convert_nonencrypted_to_encrypted() {
        TestUtils::cleanup_indy_home();
        {
            let default_wallet_type = DefaultWalletType::new();
            default_wallet_type.create_wallet("mywallet", None, r#"{"key":""}"#).unwrap();
            let wallet = default_wallet_type.open_wallet("mywallet", "pool1", None, None, r#"{"key":""}"#).unwrap();

            wallet.add_record("type1", "key1", "value1", "{}").unwrap();
        }
        {
            let default_wallet_type = DefaultWalletType::new();
            let wallet = default_wallet_type.open_wallet("mywallet", "pool1", None, None, r#"{"key":"", "rekey":"thisisatest"}"#).unwrap();
            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }
        {
            let default_wallet_type = DefaultWalletType::new();
            let wallet = default_wallet_type.open_wallet("mywallet", "pool1", None, None, r#"{"key":"thisisatest"}"#).unwrap();

            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_convert_encrypted_to_nonencrypted() {
        TestUtils::cleanup_indy_home();
        {
            let default_wallet_type = DefaultWalletType::new();
            default_wallet_type.create_wallet("mywallet", None, r#"{"key":"thisisatest"}"#).unwrap();
            let wallet = default_wallet_type.open_wallet("mywallet", "pool1", None, None, r#"{"key":"thisisatest"}"#).unwrap();

            wallet.add_record("type1", "key1", "value1", "{}").unwrap();
        }
        {
            let default_wallet_type = DefaultWalletType::new();
            let wallet = default_wallet_type.open_wallet("mywallet", "pool1", None, None, r#"{"key":"thisisatest", "rekey":""}"#).unwrap();

            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }
        {
            let default_wallet_type = DefaultWalletType::new();
            let wallet = default_wallet_type.open_wallet("mywallet", "pool1", None, None, r#"{"key":""}"#).unwrap();

            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_create_encrypted() {
        TestUtils::cleanup_indy_home();

        {
            let default_wallet_type = DefaultWalletType::new();
            default_wallet_type.create_wallet("encrypted_wallet", None, r#"{"key":"test"}"#).unwrap();
            let wallet = default_wallet_type.open_wallet("encrypted_wallet", "pool1", None, None, r#"{"key":"test"}"#).unwrap();

            wallet.add_record("type1", "key1", "value1", "{}").unwrap();

            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }
        {
            let default_wallet_type = DefaultWalletType::new();
            let wallet_error = default_wallet_type.open_wallet("encrypted_wallet", "pool1", None, None, r#"{"key":"key"}"#);

            match wallet_error {
                Ok(_) => assert!(false),
                Err(error) => assert_eq!(error.description(), String::from("Wallet security error: File opened that is not a database file"))
            };
        }

        TestUtils::cleanup_indy_home();
    }

    #[test]
    fn default_wallet_change_key() {
        TestUtils::cleanup_indy_home();

        {
            let default_wallet_type = DefaultWalletType::new();
            default_wallet_type.create_wallet("encrypted_wallet", None, r#"{"key":"test"}"#).unwrap();
            let wallet = default_wallet_type.open_wallet("encrypted_wallet", "pool1", None, None, r#"{"key":"test"}"#).unwrap();

            wallet.add_record("type1", "key1", "value1", "{}").unwrap();

            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }

        {
            let default_wallet_type = DefaultWalletType::new();
            let wallet = default_wallet_type.open_wallet("encrypted_wallet", "pool1", None, None, r#"{"key":"test","rekey":"newtest"}"#).unwrap();

            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }

        {
            let default_wallet_type = DefaultWalletType::new();
            let wallet = default_wallet_type.open_wallet("encrypted_wallet", "pool1", None, None, r#"{"key":"newtest"}"#).unwrap();

            let mut search = wallet.search_records("type1", "{}", "{}").unwrap();
            assert_eq!(1, search.get_total_count().unwrap().unwrap());

            let record = search.fetch_next_record().unwrap().unwrap();
            assert_eq!("key1", record.get_id());
            assert_eq!("value1", record.get_value().unwrap());
        }

        TestUtils::cleanup_indy_home();
    }
}
