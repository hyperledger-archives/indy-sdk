mod query;

use std;
use std::collections::HashMap;
use std::cell::RefCell;

use rusqlite;
use serde_json;

use utils::environment::EnvironmentUtils;
use errors::wallet::WalletStorageError;
use services::wallet::wallet::{TagName, WalletRuntimeConfig};
use services::wallet::language;
use super::super::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};

use super::{StorageIterator, WalletStorageType, WalletStorage, StorageEntity, StorageValue, TagValue};


const _PLAIN_TAGS_QUERY: &str = "SELECT name, value from tags_plaintext where item_id = ?";
const _ENCRYPTED_TAGS_QUERY: &str = "SELECT name, value from tags_encrypted where item_id = ?";
const _CREATE_SCHEMA: &str = "
    PRAGMA locking_mode=EXCLUSIVE;
    PRAGMA foreign_keys=ON;

    BEGIN EXCLUSIVE TRANSACTION;

    /*** Keys Table ***/

    CREATE TABLE keys(
        id INTEGER NOT NULL,
        keys NOT NULL,
        PRIMARY KEY(id)
    );

    /*** Items Table ***/

    CREATE TABLE items(
        id INTEGER NOT NULL,
        type NOT NULL,
        name NOT NULL,
        value NOT NULL,
        key NOT NULL,
        PRIMARY KEY(id)
    );

    CREATE UNIQUE INDEX ux_items_type_name ON items(type, name);

    /*** Encrypted Tags Table ***/

    CREATE TABLE tags_encrypted(
        name NOT NULL,
        value NOT NULL,
        item_id INTEGER NOT NULL,
        PRIMARY KEY(name, item_id),
        FOREIGN KEY(item_id)
            REFERENCES items(id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );

    CREATE INDEX ix_tags_encrypted_name ON tags_encrypted(name);
    CREATE INDEX ix_tags_encrypted_value ON tags_encrypted(value);
    CREATE INDEX ix_tags_encrypted_item_id ON tags_encrypted(item_id);

    /*** PlainText Tags Table ***/

    CREATE TABLE tags_plaintext(
        name NOT NULL,
        value NOT NULL,
        item_id INTEGER NOT NULL,
        PRIMARY KEY(name, item_id),
        FOREIGN KEY(item_id)
            REFERENCES items(id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );

    CREATE INDEX ix_tags_plaintext_name ON tags_plaintext(name);
    CREATE INDEX ix_tags_plaintext_value ON tags_plaintext(value);
    CREATE INDEX ix_tags_plaintext_item_id ON tags_plaintext(item_id);

    END TRANSACTION;
";


#[derive(Debug)]
struct TagRetriever<'a> {
    plain_tags_stmt: rusqlite::Statement<'a>,
    encrypted_tags_stmt: rusqlite::Statement<'a>,
}


impl <'a> TagRetriever<'a> {
    fn new(conn: &'a rusqlite::Connection) -> Result<TagRetriever<'a>, WalletStorageError> {
        let plain_tags_stmt = conn.prepare(_PLAIN_TAGS_QUERY)?;
        let encrypted_tags_stmt = conn.prepare(_ENCRYPTED_TAGS_QUERY)?;
        Ok(TagRetriever {
            plain_tags_stmt: plain_tags_stmt,
            encrypted_tags_stmt: encrypted_tags_stmt,
        })
    }

    fn retrieve(&mut self, id: i64) -> Result<HashMap<Vec<u8>, TagValue>, WalletStorageError> {
        let mut tags = HashMap::new();

        let mut plain_results = self.plain_tags_stmt.query(&[&id])?;
        while let Some(res) = plain_results.next() {
            let row = res?;
            tags.insert(row.get(0), TagValue::Plain(row.get(1)));
        }

        let mut encrypted_results = self.encrypted_tags_stmt.query(&[&id])?;
        while let Some(res) = encrypted_results.next() {
            let row = res?;
            tags.insert(row.get(0), TagValue::Encrypted(row.get(1)));
        }

        Ok(tags)
    }
}


struct SQLiteStorageIterator<'a> {
    stmt: Box<rusqlite::Statement<'a>>,
    rows: Option<rusqlite::Rows<'a>>,
    tag_retriever: Option<TagRetriever<'a>>,
    options: FetchOptions,
    fetch_type_: bool,
}


impl<'a> SQLiteStorageIterator<'a> {
    fn new(stmt: rusqlite::Statement<'a>,
           args: &[&rusqlite::types::ToSql],
           options: FetchOptions,
           tag_retriver: Option<TagRetriever<'a>>,
           fetch_type_: bool) -> Result<SQLiteStorageIterator<'a>, WalletStorageError> {
            let mut iter = SQLiteStorageIterator {
                stmt: Box::new(stmt),
                rows: None,
                tag_retriever: tag_retriver,
                options: options,
                fetch_type_: fetch_type_
            };
            iter.rows = Some(
                unsafe {
                    (*(&mut *iter.stmt as *mut rusqlite::Statement)).query(args)?
                }
            );
            Ok(iter)
    }
}


impl<'a> StorageIterator for SQLiteStorageIterator<'a> {
    fn next(&mut self) -> Result<Option<StorageEntity>, WalletStorageError> {
        match self.rows.as_mut().unwrap().next() {
            Some(Ok(row)) => {
                let name = row.get(1);
                let value = if self.options.fetch_value {
                    Some(StorageValue::new(row.get(2), row.get(3)))
                } else {
                    None
                };
                let tags = if self.options.fetch_tags {
                    Some(self.tag_retriever.as_mut().unwrap().retrieve(row.get(0))?)
                } else {
                    None
                };
                let type_ = if self.fetch_type_ {
                    Some(row.get(4))
                } else {
                    None
                };
                Ok(Some(StorageEntity::new(name, value, type_, tags)))
            },
            Some(Err(err)) => Err(WalletStorageError::from(err)),
            None => Ok(None)
        }
    }
}


#[derive(Debug,Deserialize,Serialize)]
struct FetchOptions {
    fetch_type: bool,
    fetch_value: bool,
    fetch_tags: bool,
}

impl FetchOptions {
    fn new(fetch_type: bool, fetch_value: bool, fetch_tags: bool) -> FetchOptions {
        FetchOptions {
            fetch_type: fetch_type,
            fetch_value: fetch_value,
            fetch_tags: fetch_tags,
        }
    }
}

impl Default for FetchOptions {
    fn default() -> FetchOptions {
        FetchOptions {
            fetch_type: false,
            fetch_value: true,
            fetch_tags: false,
        }
    }
}

impl JsonEncodable for FetchOptions {}

impl<'a> JsonDecodable<'a> for FetchOptions {}



#[derive(Debug)]
struct SQLiteStorage {
    conn: rusqlite::Connection,
}

pub struct SQLiteStorageType {}


impl SQLiteStorageType {
    pub fn new() -> SQLiteStorageType {
        SQLiteStorageType {}
    }

    fn create_path(name: &str) -> std::path::PathBuf {
        let mut path = EnvironmentUtils::wallet_path(name);
        path.push("sqlite.db");
        path
    }
}


#[warn(dead_code)]
impl WalletStorage for SQLiteStorage {
    ///
    /// Tries to fetch values and/or tags from the storage.
    /// Returns Result with StorageEntity object which holds requested data in case of success or
    /// Result with WalletStorageError in case of failure.
    ///
    ///
    /// # Arguments
    ///
    ///  * `type_` - type_ of the item in storag
    ///  * `name` - name of the item in storage
    ///  * `options` - JSon containing what needs to be fetched.
    ///  Example: {"fetch_value": true, "fetch_tags": true}
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `StorageEntity` - Contains name, optional value and optional tags
    ///  * `WalletStorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `WalletStorageError` type_ of errors can be throw by this method:
    ///
    ///  * `WalletStorageError::Closed` - Storage is closed
    ///  * `WalletStorageError::ItemNotFound` - Item is not found in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn get(&self, type_: &Vec<u8>, name: &Vec<u8>, options: &str) -> Result<StorageEntity, WalletStorageError> {
        let options: FetchOptions = if options == "{}" {
            FetchOptions::default()
        } else {
            serde_json::from_str(options)?
        };
        let res: Result<(i64, Vec<u8>, Vec<u8>), rusqlite::Error> = self.conn.query_row(
            "SELECT id, value, key FROM items where type = ?1 AND name = ?2",
            &[type_, name],
            |row| {
                (row.get(0), row.get(1), row.get(2))
            }
        );
        let item = match res {
            Ok(entity) => entity,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err))
        };

        if options.fetch_tags {
            let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();

            // get all encrypted.
            let mut stmt = self.conn.prepare_cached("SELECT name, value FROM tags_encrypted WHERE item_id = ?1")?;
            let mut rows = stmt.query(&[&item.0])?;

            while let Some(row) = rows.next() {
                let row = row?;
                tags.insert(row.get(0), TagValue::Encrypted(row.get(1)));
            }

            // get all plain
            let mut stmt = self.conn.prepare_cached("SELECT name, value FROM tags_plaintext WHERE item_id = ?1")?;
            let mut rows = stmt.query(&[&item.0])?;

            while let Some(row) = rows.next() {
                let row = row?;
                tags.insert(row.get(0), TagValue::Plain(row.get(1)));
            }

            Ok(StorageEntity::new(name.clone(),
                                  if options.fetch_value
                                      {Some(StorageValue::new(item.1, item.2))}
                                      else {None},
                                  None,
                                  Some(tags))
            )
        }
        else {
            Ok(StorageEntity::new(name.clone(),
                                  if options.fetch_value
                                      {Some(StorageValue::new(item.1, item.2))}
                                      else {None},
                                  None,
                                  None)
            )
        }
    }

    ///
    /// inserts value and tags into storage.
    /// Returns Result with () on success or
    /// Result with WalletStorageError in case of failure.
    ///
    ///
    /// # Arguments
    ///
    ///  * `type_` - type of the item in storage
    ///  * `name` - name of the item in storage
    ///  * `value` - value of the item in storage
    ///  * `value_key` - key used to encrypt the value
    ///  * `tags` - tags assigned to the value
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `()`
    ///  * `WalletStorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `WalletStorageError` class of errors can be throw by this method:
    ///
    ///  * `WalletStorageError::Closed` - Storage is closed
    ///  * `WalletStorageError::ItemAlreadyExists` - Item is already present in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn add(&self, type_: &Vec<u8>, name: &Vec<u8>, value: &Vec<u8>, value_key: &Vec<u8>, tags: &HashMap<Vec<u8>, TagValue>) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("INSERT INTO items (type, name, value, key) VALUES (?1, ?2, ?3, ?4)")?
                            .insert(&[type_, name, value, value_key]);

        let id = match res {
            Ok(entity) => entity,
            Err(rusqlite::Error::SqliteFailure(_, _)) => return Err(WalletStorageError::ItemAlreadyExists),
            Err(err) => return Err(WalletStorageError::from(err))
        };

        let mut stmt_e = self.conn.prepare_cached("INSERT INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)")?;
        let mut stmt_p = self.conn.prepare_cached("INSERT INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)")?;

        for (tag_name, tag_value) in tags {
            match tag_value {
                &TagValue::Encrypted(ref tag_data) => stmt_e.execute(&[&id, tag_name, tag_data])?,
                &TagValue::Plain(ref tag_data) => stmt_p.execute(&[&id, tag_name, tag_data])?
            };
        }

        Ok(())
    }
    
    fn update(&self, type_: &Vec<u8>, name: &Vec<u8>, value: &Vec<u8>, value_key: &Vec<u8>) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("UPDATE items SET value = ?1, key = ?2 WHERE type = ?3 AND name = ?4")?
            .execute(&[value, value_key, type_, name]);

        match res {
            Ok(1) => Ok(()),
            Ok(0) => Err(WalletStorageError::ItemNotFound),
            Ok(_) => unreachable!(),
            Err(err) => Err(WalletStorageError::from(err)),
        }
    }

    fn add_tags(&mut self, type_: &Vec<u8>, name: &Vec<u8>, tags: &HashMap<Vec<u8>, TagValue>) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("SELECT id FROM items WHERE type = ?1 AND name = ?2")?
            .query_row(&[type_, name], |row| row.get(0));

        let item_id: i64 = match res {
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err)),
            Ok(id) => id
        };

        let tx = self.conn.transaction()?;
        {
            let mut enc_tag_insert_stmt = tx.prepare_cached("INSERT INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)")?;
            let mut plain_tag_insert_stmt = tx.prepare_cached("INSERT INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)")?;

            for (tag_name, tag_value) in tags {
                match tag_value {
                    &TagValue::Encrypted(ref tag_data) => enc_tag_insert_stmt.execute(&[&item_id, tag_name, tag_data])?,
                    &TagValue::Plain(ref tag_data) => plain_tag_insert_stmt.execute(&[&item_id, tag_name, tag_data])?
                };
            }
        }
        tx.commit()?;

        Ok(())
    }

    fn update_tags(&mut self, type_: &Vec<u8>, name: &Vec<u8>, tags: &HashMap<Vec<u8>, TagValue>) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("SELECT id FROM items WHERE type = ?1 AND name = ?2")?
            .query_row(&[type_, name], |row| row.get(0));

        let item_id: i64 = match res {
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err)),
            Ok(id) => id
        };

        let tx = self.conn.transaction()?;
        {
            let mut enc_tag_insert_stmt = tx.prepare_cached("UPDATE tags_encrypted SET value = ?1 WHERE item_id = ?2 AND name = ?3")?;
            let mut plain_tag_insert_stmt = tx.prepare_cached("UPDATE tags_plaintext SET value = ?1 WHERE item_id = ?2 AND name = ?3")?;

            for (tag_name, tag_value) in tags {
                let res = match tag_value {
                    &TagValue::Encrypted(ref tag_data) => enc_tag_insert_stmt.execute(&[tag_data, &item_id, tag_name])?,
                    &TagValue::Plain(ref tag_data) => plain_tag_insert_stmt.execute(&[tag_data, &item_id, tag_name])?
                };

                if res == 0 {
                    return Err(WalletStorageError::ItemNotFound);
                }
            }
        }
        tx.commit()?;

        Ok(())
    }

    fn delete_tags(&mut self, type_: &Vec<u8>, name: &Vec<u8>, tag_names: &[TagName]) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("SELECT id FROM items WHERE type =?1 AND name = ?2")?
            .query_row(&[type_, name], |row| row.get(0));

        let item_id: i64 = match res {
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err)),
            Ok(id) => id
        };

        let tx = self.conn.transaction()?;
        {
            let mut enc_tag_delete_stmt = tx.prepare_cached("DELETE FROM tags_encrypted WHERE item_id = ?1 AND name = ?2")?;
            let mut plain_tag_delete_stmt = tx.prepare_cached("DELETE FROM tags_plaintext WHERE item_id = ?1 AND name = ?2")?;

            for tag_name in tag_names {
                let res = match tag_name {
                    &TagName::OfEncrypted(ref tag_name) => enc_tag_delete_stmt.execute(&[&item_id, tag_name])?,
                    &TagName::OfPlain(ref tag_name) => plain_tag_delete_stmt.execute(&[&item_id, tag_name])?,
                };

                if res == 0 {
                    return Err(WalletStorageError::ItemNotFound);
                }
            }
        }
        tx.commit()?;

        Ok(())
    }

    ///
    /// deletes value and tags into storage.
    /// Returns Result with () on success or
    /// Result with WalletStorageError in case of failure.
    ///
    ///
    /// # Arguments
    ///
    ///  * `type_` - type of the item in storag
    ///  * `name` - name of the item in storage
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `()`
    ///  * `WalletStorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `WalletStorageError` type_ of errors can be throw by this method:
    ///
    ///  * `WalletStorageError::Closed` - Storage is closed
    ///  * `WalletStorageError::ItemNotFound` - Item is not found in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn delete(&self, type_: &Vec<u8>, name: &Vec<u8>) -> Result<(), WalletStorageError> {
        let row_count = self.conn.execute(
            "DELETE FROM items where type = ?1 AND name = ?2",
            &[type_, name]
        )?;
        if row_count == 1 {
            Ok(())
        } else {
            Err(WalletStorageError::ItemNotFound)
        }
    }

    fn get_all<'a>(&'a self) -> Result<Box<StorageIterator + 'a>, WalletStorageError> {
        let statement = self.conn.prepare("SELECT id, name, value, key, type FROM items;")?;
        let fetch_options = FetchOptions {
            fetch_type: false,
            fetch_value: true,
            fetch_tags: true,
        };
        let tag_retriever = Some(TagRetriever::new(&self.conn)?);

        let storage_iterator = SQLiteStorageIterator::new(statement, &[], fetch_options, tag_retriever, true)?;
        Ok(Box::new(storage_iterator))
    }

    fn search<'a>(&'a self, type_: &Vec<u8>, query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator + 'a>, WalletStorageError> {
        let fetch_options = match options {
            None => FetchOptions::default(),
            Some(option_str) => serde_json::from_str(option_str)?
        };
        let (query_string, query_arguments) = query::wql_to_sql(type_, query, options);

        let statement = self.conn.prepare(&query_string)?;
        let tag_retriever = if fetch_options.fetch_tags {
            Some(TagRetriever::new(&self.conn)?)
        } else {
            None
        };
        let storage_iterator = SQLiteStorageIterator::new(statement, &query_arguments, fetch_options, tag_retriever, false)?;
        Ok(Box::new(storage_iterator))
    }

    ///
    /// deletes all values and tags from storage.
    /// Returns Result with () on success or
    /// Result with WalletStorageError in case of failure.
    ///
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `()`
    ///  * `WalletStorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `WalletStorageError` type_ of errors can be throw by this method:
    ///
    ///  * `WalletStorageError::Closed` - Storage is closed
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn clear(&self) -> Result<(), WalletStorageError> {
        // SQLite do not have TRUNCATE TABLE command
        self.conn.execute("DELETE FROM tags_encrypted", &[])?;
        self.conn.execute("DELETE FROM tags_plaintext", &[])?;
        self.conn.execute("DELETE FROM items", &[])?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), WalletStorageError> {
        Ok(())
    }
}


impl WalletStorageType for SQLiteStorageType {
    ///
    /// Creates the SQLite DB file with the provided name in the path specified in the config file,
    /// and initializes the encryption keys needed for encryption and decryption of data.
    ///
    /// # Arguments
    ///
    ///  * `name` - name of the SQLite DB file
    ///  * `storage_config` - config containing the location of SQLite DB files
    ///  * `keys` - encryption keys that need to be stored in the newly created DB
    ///  * `storage_credentials` - DB credentials
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `()`
    ///  * `WalletStorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `WalletStorageError` type_ of errors can be throw by this method:
    ///
    ///  * `AlreadyExists` - File with a given name already exists on the path
    ///  * `IOError("IO error during storage operation:...")` - Connection to the DB failed
    ///  * `IOError("Error occurred while creating wallet file:..)"` - Creation of schema failed
    ///  * `IOError("Error occurred while inserting the keys...")` - Insertion of keys failed
    ///  * `IOError(..)` - Deletion of the file form the file-system failed
    ///
    fn create_storage(&self, name: &str, config: Option<&str>, credentials: &str, keys: &Vec<u8>) -> Result<(), WalletStorageError> {
        let db_file_path = SQLiteStorageType::create_path(name);
        if db_file_path.exists() {
            return Err(WalletStorageError::AlreadyExists);
        }

        let conn = rusqlite::Connection::open(db_file_path.as_path())?;

        match conn.execute_batch(_CREATE_SCHEMA) {
            Ok(_) => match conn.execute("INSERT INTO keys(keys) VALUES(?1)", &[keys]) {
                Ok(_) => Ok(()),
                Err(error) => {
                    std::fs::remove_file(db_file_path)?;
                    Err(WalletStorageError::IOError(format!("Error occurred while inserting the keys: {}", error)))
                }
            },
            Err(error) => {
                std::fs::remove_file(db_file_path)?;
                Err(WalletStorageError::IOError(format!("Error occurred while creating wallet file: {}", error)))
            }
        }
    }

    ///
    /// Deletes the SQLite database file with the provided name from the path specified in the
    /// config file.
    ///
    /// # Arguments
    ///
    ///  * `name` - name of the SQLite DB file
    ///  * `storage_config` - config containing the location of SQLite DB files
    ///  * `storage_credentials` - DB credentials
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `()`
    ///  * `WalletStorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `WalletStorageError` type_ of errors can be throw by this method:
    ///
    ///  * `WalletStorageError::NotFound` - File with the provided name not found
    ///  * `IOError(..)` - Deletion of the file form the file-system failed
    ///
    fn delete_storage(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<(), WalletStorageError> {
        let db_file_path = SQLiteStorageType::create_path(name);

        if db_file_path.exists() {
            std::fs::remove_file(db_file_path)?;
            Ok(())
        } else {
            Err(WalletStorageError::NotFound)
        }
    }

    ///
    /// Establishes a connection to the SQLite DB with the provided name located in the path
    /// specified in the config. In case of a succesfull onection returns a Storage object
    /// embedding the connection and the encryption keys that will be used for encryption and
    /// decryption operations.
    ///
    ///
    /// # Arguments
    ///
    ///  * `name` - name of the SQLite DB file
    ///  * `storage_config` - config containing the location of SQLite DB files
    ///  * `runtime_config` - #TODO
    ///  * `storage_credentials` - DB credentials
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `(Box<Storage>, Vec<u8>)` - Tuple of `SQLiteStorage` and `encryption keys`
    ///  * `WalletStorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `WalletStorageError` type_ of errors can be throw by this method:
    ///
    ///  * `WalletStorageError::NotFound` - File with the provided name not found
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn open_storage(&self, name: &str, config: Option<&str>, credentials: &str)
        -> Result<(Box<WalletStorage>, Vec<u8>), WalletStorageError> {
        let db_file_path = SQLiteStorageType::create_path(name);

        if !db_file_path.exists() {
            return Err(WalletStorageError::NotFound);
        }

        let conn = rusqlite::Connection::open(db_file_path.as_path())?;
        let keys: Vec<u8> = conn.query_row("SELECT keys FROM keys LIMIT 1", &[], |row| row.get(0))?;

        Ok(
            (
                Box::new(SQLiteStorage { conn: conn }),
                keys
            )
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::TagValue;
    use std::fs::File;
    use std::io::prelude::*;
    use std::collections::HashMap;
    use std::env;


    fn _create_and_open_test_storage() -> Box<WalletStorage> {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, _) = storage_type.open_storage("test_wallet", None, "").unwrap();
        storage
    }


    fn _remove_test_file(file: &str) {
        if std::path::Path::new(&file).exists() {
            std::fs::remove_file(&file).unwrap();
        }
    }

    fn _wallet_base_path() -> std::path::PathBuf {
        let mut directory_path = env::home_dir().unwrap();
        directory_path.push(".indy_client");
        directory_path.push("wallet");
        directory_path.push("test_wallet");
        directory_path
    }

    fn _db_file_path() -> std::path::PathBuf {
        let mut db_file_path = _wallet_base_path();
        db_file_path.push("sqlite.db");
        db_file_path
    }

    fn _remove_test_wallet_file() {
        let db_file_path = _db_file_path();
        if db_file_path.exists() {
            std::fs::remove_file(db_file_path).unwrap();
        }

        let mut desc_file_path = _wallet_base_path();
        desc_file_path.push("wallet.json");
        if desc_file_path.exists() {
            std::fs::remove_file(desc_file_path).unwrap();
        }
    }

    fn _prepare_path() {
        let mut directory_path = env::home_dir().unwrap();
        directory_path.push(".indy_client");
        directory_path.push("wallet");
        directory_path.push("test_wallet");
        if !directory_path.exists() {
            std::fs::DirBuilder::new()
                .recursive(true)
                .create(directory_path).unwrap();
        }

        _remove_test_wallet_file();
    }


    /**
     * Storage config tests
     */

    fn _get_test_keys() -> Vec<u8> {
        return vec![
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8,
            1, 2, 3, 4, 5, 6, 7, 8
        ];
    }


    /**
     *  Create tests
     */
    #[test]
    fn sqlite_storage_type_create() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, &"", &test_keys).unwrap();
    }

    // Already existing wallet
    #[test]
    fn sqlite_storage_type_create_works_for_twice() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, &"", &test_keys).unwrap();
        let res = storage_type.create_storage("test_wallet", None, &"", &test_keys);

        assert_match!(Err(WalletStorageError::AlreadyExists), res);
    }

    /**
     * SQLiteWalletStorageType Delete tests
     */

    /** postitive tests */
    #[test]
    fn sqlite_storage_type_create_check_keys() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, &"", &test_keys).unwrap();

        // Assert
        let conn = rusqlite::Connection::open(_db_file_path()).unwrap();
        let db_keys: Vec<u8> = conn.query_row("SELECT keys from keys LIMIT 1", &[], |row| row.get(0)).unwrap();
        assert_eq!(test_keys, db_keys);
    }


    // no wallet with given name
    #[test]
    fn sqlite_wallet_storage_type_delete_for_nonexisting_wallet() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, &"", &test_keys).unwrap();
        let res = storage_type.delete_storage("test_wallet_wrong", None, &"");

        assert_match!(Err(WalletStorageError::NotFound), res);
    }
//
//    /**
//     * SQLiteWalletStorageType Open tests
//     */
//
    #[test]
    fn sqlite_storage_type_open_works() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        storage_type.open_storage("test_wallet", None, "").unwrap();
    }

    /** negative tests */
    // wallet not created
    #[test]
    fn sqlite_storage_type_open_returns_error_if_wallet_not_created() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();

        let res = storage_type.open_storage("test_wallet", None, "");

        assert_match!(Err(WalletStorageError::NotFound), res);
    }


    /**
     * Get/Set tests
     */

    #[test]
    fn sqlite_storage_set_get_works() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();
        assert_eq!(keys, test_keys);

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());
    }

    // update a value
    #[test]
    fn sqlite_storage_cannot_add_twice_the_same_key() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();
        assert_eq!(keys, test_keys);

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![100, 150, 200];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());

        let res = storage.add(&type_, &name, &value2, &value_key, &tags);
        assert_match!(Err(WalletStorageError::ItemAlreadyExists), res);
    }

    // get set for reopen
    #[test]
    fn sqlite_storage_set_get_works_for_reopen() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();
        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![100, 150, 200];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        {
            let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();
            storage.add(&type_, &name, &value, &value_key, &tags).unwrap();
        }

        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();
        let entity = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());
    }

    // get for non-existing key
    #[test]
    fn sqlite_storage_get_works_for_wrong_key() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();
        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();
        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![9, 8, 9];
        let value2: Vec<u8> = vec![100, 150, 200];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();
        let res = storage.get(&type_, &vec![5, 6, 6], r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##);

        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    // sql cmd inject
    #[test]
    fn sqlite_wallet_storage_sql_cmd_inject() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Plain("value'); DROP TABLE items; --".to_string()));

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let entity = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();
        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());
    }

    /** Get/Set tests end */

    /**
     * delete tests
     */

    #[test]
    fn sqlite_storage_delete() {
         _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());

        storage.delete(&type_, &name).unwrap();
        let res = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    /** negative tests */

    #[test]
    fn sqlite_storage_delete_returns_error_item_not_found_if_no_such_key() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();
        let res = storage.delete(&type_, &vec![5, 5, 6]);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }
//
//    /** delete tests - END **/
//
//
//    /**
//     * Get all tests
//     */
//
    #[test]
    fn sqlite_storage_get_all_with_2_existing_keys() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name1: Vec<u8> = vec![4, 5, 6];
        let name2: Vec<u8> = vec![4, 5, 8];
        let value1: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage.add(&type_, &name1, &value1, &value_key, &tags).unwrap();
        storage.add(&type_, &name2, &value2, &value_key, &tags).unwrap();

        let mut storage_iterator = storage.get_all().unwrap();

        let mut results = HashMap::new();
        let res1 = storage_iterator.next().unwrap().unwrap();
        results.insert(res1.name.clone(), res1);
        let res2 = storage_iterator.next().unwrap().unwrap();
        results.insert(res2.name.clone(), res2);
        let res3 = storage_iterator.next().unwrap();
        assert!(res3.is_none());

        let item1 = results.get(&name1).unwrap();
        let retrieved_value1 = item1.clone().value.unwrap();
        assert_eq!(retrieved_value1.data, value1);
        assert_eq!(retrieved_value1.key, value_key);
        assert_eq!(item1.type_.clone().unwrap(), type_);
        let item2 = results.get(&name2).unwrap();
        let retrieved_value2 = item2.clone().value.unwrap();
        assert_eq!(retrieved_value2.data, value2);
        assert_eq!(retrieved_value2.key, value_key);
        assert_eq!(item2.type_.clone().unwrap(), type_);
  }

    #[test]
    fn sqlite_storage_get_all_with_no_existing_keys() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();
        let mut storage_iterator = storage.get_all().unwrap();
        let res = storage_iterator.next().unwrap();

        assert!(res.is_none());
    }

    /**
     * Update tests
     */

    #[test]
    fn sqlite_storage_update() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value1 = vec![7,8,9];
        let value_key1 = vec![10, 10, 10];
        let value2 = vec![20, 20, 20];
        let value_key2 = vec![30, 30, 30];
        let tags = HashMap::new();

        storage.add(&type_, &name, &value1, &value_key1, &tags).unwrap();
        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        assert_eq!(item.value.unwrap().data, value1);
        storage.update(&type_, &name, &value2, &value_key2).unwrap();
        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        assert_eq!(item.value.unwrap().data, value2);
    }

    #[test]
    fn sqlite_storage_update_returns_error_on_bad_item_name() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let wrong_name = vec![100, 100, 100];
        let value1 = vec![7,8,9];
        let value_key1 = vec![10, 10, 10];
        let value2 = vec![20, 20, 20];
        let value_key2 = vec![30, 30, 30];
        let tags = HashMap::new();

        storage.add(&type_, &name, &value1, &value_key1, &tags).unwrap();
        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        assert_eq!(item.value.unwrap().data, value1);
        let res = storage.update(&type_, &wrong_name, &value2, &value_key2);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_update_returns_error_on_bad_type() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let wrong_type = vec![1,1,1];
        let name = vec![4,5,6];
        let value1 = vec![7,8,9];
        let value_key1 = vec![10, 10, 10];
        let value2 = vec![20, 20, 20];
        let value_key2 = vec![30, 30, 30];
        let tags = HashMap::new();

        storage.add(&type_, &name, &value1, &value_key1, &tags).unwrap();
        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        assert_eq!(item.value.unwrap().data, value1);
        let res = storage.update(&wrong_type, &name, &value2, &value_key2);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }


    /**
     * Add tags tests
     */

    #[test]
    fn sqlite_storage_add_tags() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![9, 9, 9]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let mut new_tags = HashMap::new();
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Encrypted(vec![2, 2, 2]);
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Plain(String::from("tag_value_3"));
        new_tags.insert(tag_name_2, tag_value_2);
        new_tags.insert(tag_name_3, tag_value_3);

        storage.add_tags(&type_, &name, &new_tags).unwrap();

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        assert_eq!(item.value.unwrap().data, value);

        let mut expected_tags = new_tags.clone();
        expected_tags.insert(tag_name_1, tag_value_1);

        assert_eq!(item.tags.unwrap(), expected_tags);
    }

    #[test]
    fn sqlite_storage_add_tags_returns_proper_error_if_wrong_name() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let tags = HashMap::new();

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let mut new_tags = HashMap::new();
        let tag_name = vec![1, 1, 1];
        let tag_value = TagValue::Encrypted(vec![2, 2, 2]);
        new_tags.insert(tag_name, tag_value);

        let res = storage.add_tags(&type_, &vec![100,100,100], &new_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_add_tags_returns_proper_error_if_wrong_type() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let tags = HashMap::new();

        storage.add(&vec![200,200], &name, &value, &value_key, &tags).unwrap();

        let mut new_tags = HashMap::new();
        let tag_name = vec![1, 1, 1];
        let tag_value = TagValue::Encrypted(vec![2, 2, 2]);
        new_tags.insert(tag_name, tag_value);

        let res = storage.add_tags(&type_, &vec![100,100,100], &new_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_add_tags_if_already_exists() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![9, 9, 9]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let mut new_tags = HashMap::new();
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Encrypted(vec![2, 2, 2]);
        new_tags.insert(tag_name_2, tag_value_2);
        new_tags.insert(tag_name_1, tag_value_1);

        let res = storage.add_tags(&type_, &name, &new_tags);
        assert_match!(Err(WalletStorageError::ItemAlreadyExists), res);

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        assert_eq!(item.tags.unwrap(), tags);
    }


    /**
     * Update tags tests
     */

    #[test]
    fn sqlite_storage_update_tags() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let mut updated_tags = HashMap::new();
        let new_tag_value_1 = TagValue::Encrypted(vec![10, 10, 10, 10]);
        let new_tag_value_2 = TagValue::Plain("new_tag_value_2".to_string());
        updated_tags.insert(tag_name_1.clone(), new_tag_value_1.clone());
        updated_tags.insert(tag_name_2.clone(), new_tag_value_2.clone());

        storage.update_tags(&type_, &name, &updated_tags).unwrap();

        let mut expected_tags = updated_tags.clone();
        expected_tags.insert(tag_name_3.clone(), tag_value_3.clone());

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(retrieved_tags, expected_tags);
    }

    #[test]
    fn sqlite_storage_update_tags_returns_error_if_wrong_name() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let mut updated_tags = HashMap::new();
        let new_tag_value_1 = TagValue::Encrypted(vec![10, 10, 10, 10]);
        updated_tags.insert(tag_name_1.clone(), new_tag_value_1.clone());

        let res = storage.update_tags(&type_, &vec![100, 100], &updated_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_update_tags_returns_error_if_wrong_type() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let mut updated_tags = HashMap::new();
        let new_tag_value_1 = TagValue::Encrypted(vec![10, 10, 10, 10]);
        updated_tags.insert(tag_name_1.clone(), new_tag_value_1.clone());

        let res = storage.update_tags(&vec![100, 100], &name, &updated_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_update_tags_returns_error_if_nonexistant_tag_and_works_atomically() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let mut updated_tags = HashMap::new();
        let new_tag_value_1 = TagValue::Encrypted(vec![10, 10, 10, 10]);
        let new_tag_value_2 = TagValue::Plain("new_tag_value_2".to_string());
        let nonexistant_tag_name = vec![100, 100];
        updated_tags.insert(tag_name_1.clone(), new_tag_value_1.clone());
        updated_tags.insert(nonexistant_tag_name, new_tag_value_2.clone());

        let res = storage.update_tags(&type_, &name, &updated_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(retrieved_tags, tags);
    }


    /**
     * Delete tags tests
     */
    #[test]
    fn sqlite_storage_delete_tags() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name_1.clone()), TagName::OfPlain(tag_name_2.clone())];
        storage.delete_tags(&type_, &name, &tag_names).unwrap();

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        let mut expected_tags = HashMap::new();
        expected_tags.insert(tag_name_3.clone(), tag_value_3.clone());
        assert_eq!(retrieved_tags, expected_tags);
    }

    #[test]
    fn sqlite_storage_returns_error_if_wrong_type() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name_1.clone()), TagName::OfPlain(tag_name_2.clone())];
        let res = storage.delete_tags(&vec![0, 0, 0], &name, &tag_names);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(retrieved_tags, tags);
    }

    #[test]
    fn sqlite_storage_returns_error_if_wrong_name() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name_1.clone()), TagName::OfPlain(tag_name_2.clone())];
        let res = storage.delete_tags(&type_, &vec![0, 0, 0], &tag_names);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(retrieved_tags, tags);
    }

    #[test]
    fn sqlite_storage_delete_tags_works_atomically_and_returns_error_if_one_tag_name_is_wrong() {
        let mut storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = vec![7,8,9];
        let value_key = vec![10, 10, 10];
        let mut tags = HashMap::new();
        let tag_name_1 = vec![0, 0, 0];
        let tag_value_1 = TagValue::Encrypted(vec![0, 0, 0]);
        let tag_name_2 = vec![1, 1, 1];
        let tag_value_2 = TagValue::Plain("tag_value_2".to_string());
        let tag_name_3 = vec![2, 2, 2];
        let tag_value_3 = TagValue::Encrypted(vec![2, 2, 2]);
        tags.insert(tag_name_1.clone(), tag_value_1.clone());
        tags.insert(tag_name_2.clone(), tag_value_2.clone());
        tags.insert(tag_name_3.clone(), tag_value_3.clone());

        storage.add(&type_, &name, &value, &value_key, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name_1.clone()), TagName::OfPlain(tag_name_2.clone()), TagName::OfEncrypted(vec![100, 100, 100])];
        let res = storage.delete_tags(&type_, &name, &tag_names);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);

        let item = storage.get(&type_, &name, r##"{"fetch_type": false, "fetch_value": true, "fetch_tags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(retrieved_tags, tags);
    }

    /**
     * Clear tests
     */

    #[test]
    fn sqlite_storage_clear() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let (storage, keys) = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name1: Vec<u8> = vec![4, 5, 6];
        let name2: Vec<u8> = vec![4, 5, 8];
        let value1: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage.add(&type_, &name1, &value1, &value_key, &tags).unwrap();
        storage.add(&type_, &name2, &value2, &value_key, &tags).unwrap();

        let mut storage_iterator = storage.get_all().unwrap();

        let mut results = HashMap::new();
        let res1 = storage_iterator.next().unwrap().unwrap();
        results.insert(res1.name.clone(), res1);
        let res2 = storage_iterator.next().unwrap().unwrap();
        results.insert(res2.name.clone(), res2);
        let res3 = storage_iterator.next().unwrap();
        assert!(res3.is_none());

        let item1 = results.get(&name1).unwrap();
        let retrieved_value1 = item1.clone().value.unwrap();
        assert_eq!(retrieved_value1.data, value1);
        assert_eq!(retrieved_value1.key, value_key);
        assert_eq!(item1.type_.clone().unwrap(), type_);
        let item2 = results.get(&name2).unwrap();
        let retrieved_value2 = item2.clone().value.unwrap();
        assert_eq!(retrieved_value2.data, value2);
        assert_eq!(retrieved_value2.key, value_key);
        assert_eq!(item2.type_.clone().unwrap(), type_);

        storage.clear().unwrap();
        let mut storage_iterator = storage.get_all().unwrap();
        let res = storage_iterator.next().unwrap();
        assert!(res.is_none());
    }

    /**
     * Delete tests
     */

    #[test]
    fn sqlite_storage_type_delete_works() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        storage_type.delete_storage("test_wallet", None, "").unwrap();
        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();

    }

    /** Delete negative tests */
    #[test]
    fn sqlite_storage_type_delete_for_nonexisting_wallet() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let res = storage_type.delete_storage("wrong_test_wallet", None, "");

        assert_match!(Err(WalletStorageError::NotFound), res);
    }
}
