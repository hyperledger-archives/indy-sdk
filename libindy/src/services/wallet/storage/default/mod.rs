extern crate owning_ref;
extern crate sodiumoxide;

mod query;
mod transaction;

use std;
use std::fs;

use rusqlite;
use serde_json;

use self::owning_ref::OwningHandle;
use std::rc::Rc;

use utils::environment::EnvironmentUtils;
use errors::wallet::WalletStorageError;
use errors::common::CommonError;
use services::wallet::language;

use super::{StorageIterator, WalletStorageType, WalletStorage, StorageRecord, EncryptedValue, Tag, TagName};
use super::super::{RecordOptions, SearchOptions};


const _SQLITE_DB: &str = "sqlite.db";
const _PLAIN_TAGS_QUERY: &str = "SELECT name, value from tags_plaintext where item_id = ?";
const _ENCRYPTED_TAGS_QUERY: &str = "SELECT name, value from tags_encrypted where item_id = ?";
const _CREATE_SCHEMA: &str = "
    PRAGMA locking_mode=EXCLUSIVE;
    PRAGMA foreign_keys=ON;

    BEGIN EXCLUSIVE TRANSACTION;

    /*** Keys Table ***/

    CREATE TABLE metadata (
        id INTEGER NOT NULL,
        value NOT NULL,
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

type TagRetrieverOwned = OwningHandle<Rc<rusqlite::Connection>, Box<TagRetriever<'static>>>;

impl<'a> TagRetriever<'a> {
    fn new_owned(conn: Rc<rusqlite::Connection>) -> Result<TagRetrieverOwned, WalletStorageError> {
        OwningHandle::try_new(conn.clone(), |conn| -> Result<_, rusqlite::Error> {
            let (plain_tags_stmt, encrypted_tags_stmt) = unsafe {
                ((*conn).prepare(_PLAIN_TAGS_QUERY)?,
                 (*conn).prepare(_ENCRYPTED_TAGS_QUERY)?)
            };
            let tr = TagRetriever {
                plain_tags_stmt,
                encrypted_tags_stmt,
            };
            Ok(Box::new(tr))
        }).map_err(WalletStorageError::from)
    }

    fn retrieve(&mut self, id: i64) -> Result<Vec<Tag>, WalletStorageError> {
        let mut tags = Vec::new();

        let mut plain_results = self.plain_tags_stmt.query(&[&id])?;
        while let Some(res) = plain_results.next() {
            let row = res?;
            tags.push(Tag::PlainText(row.get(0), row.get(1)));
        }

        let mut encrypted_results = self.encrypted_tags_stmt.query(&[&id])?;
        while let Some(res) = encrypted_results.next() {
            let row = res?;
            tags.push(Tag::Encrypted(row.get(0), row.get(1)));
        }

        Ok(tags)
    }
}


struct SQLiteStorageIterator {
    rows: Option<
        OwningHandle<
            OwningHandle<
                Rc<rusqlite::Connection>,
                Box<rusqlite::Statement<'static>>>,
            Box<rusqlite::Rows<'static>>>>,
    tag_retriever: Option<TagRetrieverOwned>,
    options: RecordOptions,
    total_count: Option<usize>,
}


impl SQLiteStorageIterator {
    fn new(stmt: Option<OwningHandle<Rc<rusqlite::Connection>, Box<rusqlite::Statement<'static>>>>,
           args: &[&rusqlite::types::ToSql],
           options: RecordOptions,
           tag_retriever: Option<TagRetrieverOwned>,
           total_count: Option<usize>) -> Result<SQLiteStorageIterator, WalletStorageError> {
        let mut iter = SQLiteStorageIterator {
            rows: None,
            tag_retriever,
            options,
            total_count
        };

        if let Some(stmt) = stmt {
            iter.rows = Some(OwningHandle::try_new(
                stmt, |stmt|
                    unsafe {
                        (*(stmt as *mut rusqlite::Statement)).query(args).map(Box::new)
                    },
            )?);
        }
        Ok(iter)
    }
}


impl StorageIterator for SQLiteStorageIterator {
    fn next(&mut self) -> Result<Option<StorageRecord>, WalletStorageError> {
        // if records are not requested.
        if self.rows.is_none() {
            return Ok(None);
        }

        match self.rows.as_mut().unwrap().next() {
            Some(Ok(row)) => {
                let name = row.get(1);
                let value = if self.options.retrieve_value {
                    Some(EncryptedValue::new(row.get(2), row.get(3)))
                } else {
                    None
                };
                let tags = if self.options.retrieve_tags {
                    match self.tag_retriever {
                        Some(ref mut tag_retriever) => Some(tag_retriever.retrieve(row.get(0))?),
                        None => return Err(WalletStorageError::CommonError(
                            CommonError::InvalidState("Fetch tags option set and tag retriever is None".to_string())
                        ))
                    }
                } else {
                    None
                };
                let type_ = if self.options.retrieve_type {
                    Some(row.get(4))
                } else {
                    None
                };
                Ok(Some(StorageRecord::new(name, value, type_, tags)))
            }
            Some(Err(err)) => Err(WalletStorageError::from(err)),
            None => Ok(None)
        }
    }

    fn get_total_count(&self) -> Result<Option<usize>, WalletStorageError> {
        Ok(self.total_count)
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    path: Option<String>,
}

#[derive(Debug)]
struct SQLiteStorage {
    conn: Rc<rusqlite::Connection>,
}

pub struct SQLiteStorageType {}


impl SQLiteStorageType {
    pub fn new() -> SQLiteStorageType {
        SQLiteStorageType {}
    }

    fn _db_path(id: &str, config: Option<&Config>) -> std::path::PathBuf {

        let mut path = match config {
            Some(Config {path: Some(ref path)}) => std::path::PathBuf::from(path),
            _ => EnvironmentUtils::wallet_home_path()
        };

        path.push(id);
        path.push(_SQLITE_DB);
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
    ///  * `type_` - type_ of the item in storage
    ///  * `id` - id of the item in storage
    ///  * `options` - JSon containing what needs to be fetched.
    ///  Example: {"retrieveValue": true, "retrieveTags": true}
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
    fn get(&self, type_: &[u8], id: &[u8], options: &str) -> Result<StorageRecord, WalletStorageError> {
        let options: RecordOptions = if options == "{}" { // FIXME:
            RecordOptions::default()
        } else {
            serde_json::from_str(options)?
        };
        let res: Result<(i64, Vec<u8>, Vec<u8>), rusqlite::Error> = self.conn.query_row(
            "SELECT id, value, key FROM items where type = ?1 AND name = ?2",
            &[&type_.to_vec(), &id.to_vec()],
            |row| {
                (row.get(0), row.get(1), row.get(2))
            }
        );
        let item = match res {
            Ok(entity) => entity,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err))
        };
        let value = if options.retrieve_value
            { Some(EncryptedValue::new(item.1, item.2)) } else { None };
        let type_ = if options.retrieve_type { Some(type_.clone()) } else { None };
        let tags = if options.retrieve_tags {
            let mut tags = Vec::new();

            // get all encrypted.
            let mut stmt = self.conn.prepare_cached("SELECT name, value FROM tags_encrypted WHERE item_id = ?1")?;
            let mut rows = stmt.query(&[&item.0])?;

            while let Some(row) = rows.next() {
                let row = row?;
                tags.push(Tag::Encrypted(row.get(0), row.get(1)));
            }

            // get all plain
            let mut stmt = self.conn.prepare_cached("SELECT name, value FROM tags_plaintext WHERE item_id = ?1")?;
            let mut rows = stmt.query(&[&item.0])?;

            while let Some(row) = rows.next() {
                let row = row?;
                tags.push(Tag::PlainText(row.get(0), row.get(1)));
            }
            Some(tags)
        } else { None };

        Ok(StorageRecord::new(id.to_vec(), value, type_.map(|val| val.to_vec()), tags))
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
    ///  * `id` - id of the item in storage
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
    fn add(&self, type_: &[u8], id: &[u8], value: &EncryptedValue, tags: &[Tag]) -> Result<(), WalletStorageError> {
        let tx: transaction::Transaction = transaction::Transaction::new(&self.conn, rusqlite::TransactionBehavior::Deferred)?;
        let res = tx.prepare_cached("INSERT INTO items (type, name, value, key) VALUES (?1, ?2, ?3, ?4)")?
            .insert(&[&type_.to_vec(), &id.to_vec(), &value.data, &value.key]);

        let id = match res {
            Ok(entity) => entity,
            Err(rusqlite::Error::SqliteFailure(_, _)) => return Err(WalletStorageError::ItemAlreadyExists),
            Err(err) => return Err(WalletStorageError::from(err))
        };

        {
            let mut stmt_e = tx.prepare_cached("INSERT INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)")?;
            let mut stmt_p = tx.prepare_cached("INSERT INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)")?;

            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => stmt_e.execute(&[&id, tag_name, tag_data])?,
                    &Tag::PlainText(ref tag_name, ref tag_data) => stmt_p.execute(&[&id, tag_name, tag_data])?
                };
            }
        }

        tx.commit()?;

        Ok(())
    }

    fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("UPDATE items SET value = ?1, key = ?2 WHERE type = ?3 AND name = ?4")?
            .execute(&[&value.data, &value.key, &type_.to_vec(), &id.to_vec()]);

        match res {
            Ok(1) => Ok(()),
            Ok(0) => Err(WalletStorageError::ItemNotFound),
            Ok(count) => Err(WalletStorageError::CommonError(CommonError::InvalidState(format!("SQLite returned update row count: {}", count)))),
            Err(err) => Err(WalletStorageError::from(err)),
        }
    }

    fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), WalletStorageError> {
        let tx: transaction::Transaction = transaction::Transaction::new(&self.conn, rusqlite::TransactionBehavior::Deferred)?;

        let res = tx.prepare_cached("SELECT id FROM items WHERE type = ?1 AND name = ?2")?
            .query_row(&[&type_.to_vec(), &id.to_vec()], |row| row.get(0));

        let item_id: i64 = match res {
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err)),
            Ok(id) => id
        };

        {
            let mut enc_tag_insert_stmt = tx.prepare_cached("INSERT OR REPLACE INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)")?;
            let mut plain_tag_insert_stmt = tx.prepare_cached("INSERT OR REPLACE INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)")?;

            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => enc_tag_insert_stmt.execute(&[&item_id, tag_name, tag_data])?,
                    &Tag::PlainText(ref tag_name, ref tag_data) => plain_tag_insert_stmt.execute(&[&item_id, tag_name, tag_data])?
                };
            }
        }
        tx.commit()?;

        Ok(())
    }

    fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> Result<(), WalletStorageError> {
        let tx: transaction::Transaction = transaction::Transaction::new(&self.conn, rusqlite::TransactionBehavior::Deferred)?;

        let res = tx.prepare_cached("SELECT id FROM items WHERE type = ?1 AND name = ?2")?
            .query_row(&[&type_.to_vec(), &id.to_vec()], |row| row.get(0));

        let item_id: i64 = match res {
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err)),
            Ok(id) => id
        };

        tx.execute("DELETE FROM tags_encrypted WHERE item_id = ?1", &[&item_id])?;
        tx.execute("DELETE FROM tags_plaintext WHERE item_id = ?1", &[&item_id])?;

        {
            let mut enc_tag_insert_stmt = tx.prepare_cached("INSERT INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)")?;
            let mut plain_tag_insert_stmt = tx.prepare_cached("INSERT INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)")?;

            for tag in tags {
                match tag {
                    &Tag::Encrypted(ref tag_name, ref tag_data) => enc_tag_insert_stmt.execute(&[&item_id, tag_name, tag_data])?,
                    &Tag::PlainText(ref tag_name, ref tag_data) => plain_tag_insert_stmt.execute(&[&item_id, tag_name, tag_data])?
                };
            }
        }
        tx.commit()?;

        Ok(())
    }

    fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("SELECT id FROM items WHERE type =?1 AND name = ?2")?
            .query_row(&[&type_.to_vec(), &id.to_vec()], |row| row.get(0));

        let item_id: i64 = match res {
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err)),
            Ok(id) => id
        };

        let tx: transaction::Transaction = transaction::Transaction::new(&self.conn, rusqlite::TransactionBehavior::Deferred)?;
        {
            let mut enc_tag_delete_stmt = tx.prepare_cached("DELETE FROM tags_encrypted WHERE item_id = ?1 AND name = ?2")?;
            let mut plain_tag_delete_stmt = tx.prepare_cached("DELETE FROM tags_plaintext WHERE item_id = ?1 AND name = ?2")?;

            for tag_name in tag_names {
                match tag_name {
                    &TagName::OfEncrypted(ref tag_name) => enc_tag_delete_stmt.execute(&[&item_id, tag_name])?,
                    &TagName::OfPlain(ref tag_name) => plain_tag_delete_stmt.execute(&[&item_id, tag_name])?,
                };
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
    ///  * `type_` - type of the item in storage
    ///  * `id` - id of the item in storage
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
    fn delete(&self, type_: &[u8], id: &[u8]) -> Result<(), WalletStorageError> {
        let row_count = self.conn.execute(
            "DELETE FROM items where type = ?1 AND name = ?2",
            &[&type_.to_vec(), &id.to_vec()]
        )?;
        if row_count == 1 {
            Ok(())
        } else {
            Err(WalletStorageError::ItemNotFound)
        }
    }

    fn get_storage_metadata(&self) -> Result<Vec<u8>, WalletStorageError> {
        let res: Result<Vec<u8>, rusqlite::Error> = self.conn.query_row(
            "SELECT value FROM metadata",
            &[],
            |row| { row.get(0) }
        );

        match res {
            Ok(entity) => Ok(entity),
            Err(rusqlite::Error::QueryReturnedNoRows) => return Err(WalletStorageError::ItemNotFound),
            Err(err) => return Err(WalletStorageError::from(err))
        }
    }

    fn set_storage_metadata(&self, metadata: &[u8]) -> Result<(), WalletStorageError> {
        match self.conn.execute("UPDATE metadata SET value = ?1", &[&metadata.to_vec()]) {
            Ok(_) => Ok(()),
            Err(error) => {
                Err(WalletStorageError::IOError(format!("Error occurred while inserting the keys: {}", error)))
            }
        }
    }

    fn get_all(&self) -> Result<Box<StorageIterator>, WalletStorageError> {
        let statement = self._prepare_statement("SELECT id, name, value, key, type FROM items;")?;
        let fetch_options = RecordOptions {
            retrieve_type: true,
            retrieve_value: true,
            retrieve_tags: true,
        };
        let tag_retriever = Some(TagRetriever::new_owned(self.conn.clone())?);

        let storage_iterator = SQLiteStorageIterator::new(Some(statement), &[], fetch_options, tag_retriever, None)?;
        Ok(Box::new(storage_iterator))
    }

    fn search(&self, type_: &[u8], query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, WalletStorageError> {
        let type_ = type_.to_vec(); // FIXME

        let search_options = match options {
            None => SearchOptions::default(),
            Some(option_str) => serde_json::from_str(option_str)?
        };

        let total_count: Option<usize> = if search_options.retrieve_total_count {
            let (query_string, query_arguments) = query::wql_to_sql_count(&type_, query)?;

            self.conn.query_row(
                &query_string,
                &query_arguments,
                |row| {
                    let x: i64 = row.get(0);
                    Some(x as usize)
                }
            )?
        } else { None };


        if search_options.retrieve_records {
            let fetch_options = RecordOptions {
                retrieve_value: search_options.retrieve_value,
                retrieve_tags: search_options.retrieve_tags,
                retrieve_type: search_options.retrieve_type,
            };

            let (query_string, query_arguments) = query::wql_to_sql(&type_, query, options)?;

            let statement = self._prepare_statement(&query_string)?;
            let tag_retriever = if fetch_options.retrieve_tags {
                Some(TagRetriever::new_owned(self.conn.clone())?)
            } else {
                None
            };
            let storage_iterator = SQLiteStorageIterator::new(Some(statement), &query_arguments, fetch_options, tag_retriever, total_count)?;
            Ok(Box::new(storage_iterator))
        } else {
            let storage_iterator = SQLiteStorageIterator::new(None, &[], RecordOptions::default(), None, total_count)?;
            Ok(Box::new(storage_iterator))
        }
    }

    fn close(&mut self) -> Result<(), WalletStorageError> {
        Ok(())
    }
}

impl SQLiteStorage {
    fn _prepare_statement(&self, sql: &str) -> Result<
        OwningHandle<Rc<rusqlite::Connection>, Box<rusqlite::Statement<'static>>>,
        WalletStorageError> {
        OwningHandle::try_new(self.conn.clone(), |conn| {
            unsafe { (*conn).prepare(sql) }.map(Box::new).map_err(WalletStorageError::from)
        })
    }
}


impl WalletStorageType for SQLiteStorageType {
    ///
    /// Deletes the SQLite database file with the provided id from the path specified in the
    /// config file.
    ///
    /// # Arguments
    ///
    ///  * `id` - id of the SQLite DB file
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
    ///  * `WalletStorageError::NotFound` - File with the provided id not found
    ///  * `IOError(..)` - Deletion of the file form the file-system failed
    ///
    fn delete_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>) -> Result<(), WalletStorageError> {
        let config = config
            .map(serde_json::from_str::<Config>)
            .map_or(Ok(None), |v| v.map(Some))
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize config: {:?}", err)))?;

        let db_file_path = SQLiteStorageType::_db_path(id, config.as_ref());

        if db_file_path.exists() {
            std::fs::remove_dir_all(db_file_path.parent().unwrap())?;
            Ok(())
        } else {
            Err(WalletStorageError::NotFound)
        }
    }

    ///
    /// Creates the SQLite DB file with the provided name in the path specified in the config file,
    /// and initializes the encryption keys needed for encryption and decryption of data.
    ///
    /// # Arguments
    ///
    ///  * `id` - name of the SQLite DB file
    ///  * `config` - config containing the location of SQLite DB files
    ///  * `credentials` - DB credentials
    ///  * `metadata` - encryption keys that need to be stored in the newly created DB
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
    fn create_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>, metadata: &[u8]) -> Result<(), WalletStorageError> {

        let config = config
            .map(serde_json::from_str::<Config>)
            .map_or(Ok(None), |v| v.map(Some))
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize config: {:?}", err)))?;

        let db_path = SQLiteStorageType::_db_path(id, config.as_ref());

        if db_path.exists() {
            return Err(WalletStorageError::AlreadyExists);
        }

        fs::DirBuilder::new()
            .recursive(true)
            .create(db_path.parent().unwrap())?;

        let conn = rusqlite::Connection::open(db_path.as_path())?;

        match conn.execute_batch(_CREATE_SCHEMA) {
            Ok(_) => match conn.execute("INSERT OR REPLACE INTO metadata(value) VALUES(?1)", &[&metadata.to_vec()]) {
                Ok(_) => Ok(()),
                Err(error) => {
                    std::fs::remove_file(db_path)?;
                    Err(WalletStorageError::IOError(format!("Error occurred while inserting the keys: {}", error)))
                }
            },
            Err(error) => {
                std::fs::remove_file(db_path)?;
                Err(WalletStorageError::IOError(format!("Error occurred while creating wallet file: {}", error)))
            }
        }
    }

    ///
    /// Establishes a connection to the SQLite DB with the provided id located in the path
    /// specified in the config. In case of a successful onection returns a Storage object
    /// embedding the connection and the encryption keys that will be used for encryption and
    /// decryption operations.
    ///
    ///
    /// # Arguments
    ///
    ///  * `id` - id of the SQLite DB file
    ///  * `config` - config containing the location of SQLite DB files
    ///  * `credentials` - DB credentials
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
    ///  * `WalletStorageError::NotFound` - File with the provided id not found
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn open_storage(&self, id: &str, config: Option<&str>, credentials: Option<&str>) -> Result<Box<WalletStorage>, WalletStorageError> {

        let config = config
            .map(serde_json::from_str::<Config>)
            .map_or(Ok(None), |v| v.map(Some))
            .map_err(|err| CommonError::InvalidStructure(format!("Cannot deserialize config: {:?}", err)))?;

        let db_file_path = SQLiteStorageType::_db_path(id, config.as_ref());

        if !db_file_path.exists() {
            return Err(WalletStorageError::NotFound);
        }

        let conn = rusqlite::Connection::open(db_file_path.as_path())?;

        Ok(Box::new(SQLiteStorage { conn: Rc::new(conn) }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::Tag;

    use utils::test::TestUtils;

    #[test]
    fn sqlite_storage_type_create_works() {
        _cleanup();

        let storage_type = SQLiteStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();
    }

    #[test]
    fn sqlite_storage_type_create_works_for_custom_path() {
        _cleanup();

        let config = json!({
            "path": _custom_path()
        }).to_string();

        let storage_type = SQLiteStorageType::new();
        storage_type.create_storage(_wallet_id(), Some(&config), None, &_metadata()).unwrap();
    }

    #[test]
    fn sqlite_storage_type_create_works_for_twice() {
        _cleanup();

        let storage_type = SQLiteStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();

        let res = storage_type.create_storage(_wallet_id(), None, None, &_metadata());
        assert_match!(Err(WalletStorageError::AlreadyExists), res);
    }

    #[test]
    fn sqlite_storage_get_storage_metadata_works() {
        _cleanup();

        let storage = _storage();
        let metadata = storage.get_storage_metadata().unwrap();

        assert_eq!(metadata, _metadata());
    }

    #[test]
    fn sqlite_storage_type_delete_works() {
        _cleanup();

        let storage_type = SQLiteStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();

        storage_type.delete_storage(_wallet_id(), None, None).unwrap();
    }


    #[test]
    fn sqlite_storage_type_delete_works_for_non_existing() {
        _cleanup();

        let storage_type = SQLiteStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();

        let res = storage_type.delete_storage("unknown", None, None);
        assert_match!(Err(WalletStorageError::NotFound), res);

        storage_type.delete_storage(_wallet_id(), None, None).unwrap();
    }

    #[test]
    fn sqlite_storage_type_open_works() {
        _cleanup();
        _storage();
    }

    #[test]
    fn sqlite_storage_type_open_works_for_custom() {
        _cleanup();
        _storage_custom();
    }

    #[test]
    fn sqlite_storage_type_open_works_for_not_created() {
        _cleanup();

        let storage_type = SQLiteStorageType::new();

        let res = storage_type.open_storage("unknown", Some("{}"), Some("{}"));
        assert_match!(Err(WalletStorageError::NotFound), res);
    }

    #[test]
    fn sqlite_storage_add_works_for_is_802() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags());
        assert_match!(Err(WalletStorageError::ItemAlreadyExists), res);

        let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags());
        assert_match!(Err(WalletStorageError::ItemAlreadyExists), res);
    }

    #[test]
    fn sqlite_storage_set_get_works() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));
    }

    #[test]
    fn sqlite_storage_set_get_works_for_custom() {
        _cleanup();

        let storage = _storage_custom();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));
    }

    #[test]
    fn sqlite_storage_set_get_works_for_twice() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add(&_type1(), &_id1(), &_value2(), &_tags());
        assert_match!(Err(WalletStorageError::ItemAlreadyExists), res);
    }

    #[test]
    fn sqlite_storage_set_get_works_for_reopen() {
        _cleanup();

        {
            _storage().add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        }

        let storage_type = SQLiteStorageType::new();
        let storage = storage_type.open_storage(_wallet_id(), Some("{}"), Some("{}")).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));
    }

    #[test]
    fn sqlite_storage_get_works_for_wrong_key() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.get(&_type1(), &_id2(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_delete_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

        storage.delete(&_type1(), &_id1()).unwrap();
        let res = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_delete_works_for_non_existing() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.delete(&_type1(), &_id2());
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }


    #[test]
    fn sqlite_storage_get_all_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        storage.add(&_type2(), &_id2(), &_value2(), &_tags()).unwrap();

        let mut storage_iterator = storage.get_all().unwrap();

        let record = storage_iterator.next().unwrap().unwrap();
        assert_eq!(record.type_.unwrap(), _type1());
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

        let record = storage_iterator.next().unwrap().unwrap();
        assert_eq!(record.type_.unwrap(), _type2());
        assert_eq!(record.value.unwrap(), _value2());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());
    }

    #[test]
    fn sqlite_storage_get_all_works_for_empty() {
        _cleanup();

        let storage = _storage();
        let mut storage_iterator = storage.get_all().unwrap();

        let record = storage_iterator.next().unwrap();
        assert!(record.is_none());
    }

    #[test]
    fn sqlite_storage_update_works() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        storage.update(&_type1(), &_id1(), &_value2()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value2());
    }

    #[test]
    fn sqlite_storage_update_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let res = storage.update(&_type1(), &_id2(), &_value2());
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_update_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();

        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();
        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let res = storage.update(&_type2(), &_id1(), &_value2());
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_add_tags_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        storage.add_tags(&_type1(), &_id1(), &_new_tags()).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let expected_tags = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            _sort(tags)
        };

        assert_eq!(_sort(record.tags.unwrap()), expected_tags);
    }

    #[test]
    fn sqlite_storage_add_tags_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add_tags(&_type1(), &_id2(), &_new_tags());
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_add_tags_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.add_tags(&_type2(), &_id1(), &_new_tags());
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_add_tags_works_for_already_existing() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let tags_with_existing = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            tags
        };

        storage.add_tags(&_type1(), &_id1(), &tags_with_existing).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let expected_tags = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            _sort(tags)
        };

        assert_eq!(_sort(record.tags.unwrap()), expected_tags);
    }

    #[test]
    fn sqlite_storage_update_tags_works() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let tags_with_existing = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            tags
        };

        storage.update_tags(&_type1(), &_id1(), &_new_tags()).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_new_tags()));
    }

    #[test]
    fn sqlite_storage_update_tags_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.update_tags(&_type1(), &_id2(), &_new_tags());
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_update_tags_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let res = storage.update_tags(&_type1(), &_id2(), &_new_tags());
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_update_tags_works_for_already_existing() {
        _cleanup();

        let storage = _storage();
        storage.add(&_type1(), &_id1(), &_value1(), &_tags()).unwrap();

        let tags_with_existing = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            tags
        };

        storage.update_tags(&_type1(), &_id1(), &tags_with_existing).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.value.unwrap(), _value1());

        let expected_tags = {
            let mut tags = _tags();
            tags.extend(_new_tags());
            _sort(tags)
        };

        assert_eq!(_sort(record.tags.unwrap()), expected_tags);
    }

    #[test]
    fn sqlite_storage_delete_tags_works() {
        _cleanup();

        let storage = _storage();

        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

        storage.add(&_type1(), &_id1(), &_value1(), &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        storage.delete_tags(&_type1(), &_id1(), &tag_names).unwrap();

        let record = storage.get(&_type1(), &_id1(), r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(record.tags.unwrap(), vec![tag3]);
    }

    #[test]
    fn sqlite_storage_delete_tags_works_for_non_existing_type() {
        _cleanup();

        let storage = _storage();

        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

        storage.add(&_type1(), &_id1(), &_value1(), &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        let res = storage.delete_tags(&_type2(), &_id1(), &tag_names);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_delete_tags_works_for_non_existing_id() {
        _cleanup();

        let storage = _storage();

        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

        storage.add(&_type1(), &_id1(), &_value1(), &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        let res = storage.delete_tags(&_type1(), &_id2(), &tag_names);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    fn _cleanup() {
        TestUtils::cleanup_storage()
    }

    fn _wallet_id() -> &'static str {
        "w1"
    }

    fn _storage() -> Box<WalletStorage> {
        let storage_type = SQLiteStorageType::new();
        storage_type.create_storage(_wallet_id(), None, None, &_metadata()).unwrap();
        storage_type.open_storage(_wallet_id(), None, None).unwrap()
    }

    fn _storage_custom() -> Box<WalletStorage> {
        let storage_type = SQLiteStorageType::new();

        let config = json!({
            "path": _custom_path()
        }).to_string();

        storage_type.create_storage(_wallet_id(), Some(&config), None, &_metadata()).unwrap();
        storage_type.open_storage(_wallet_id(), Some(&config), None).unwrap()
    }

    fn _metadata() -> Vec<u8> {
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

    fn _type(i: u8) -> Vec<u8> {
        vec![i, 1 + i, 2 + i]
    }

    fn _type1() -> Vec<u8> {
        _type(1)
    }

    fn _type2() -> Vec<u8> {
        _type(2)
    }

    fn _id(i: u8) -> Vec<u8> {
        vec![3 + i, 4 + i, 5 + i]
    }

    fn _id1() -> Vec<u8> {
        _id(1)
    }

    fn _id2() -> Vec<u8> {
        _id(2)
    }

    fn _value(i: u8) -> EncryptedValue {
        EncryptedValue { data: vec![6 + i, 7 + i, 8 + i], key: vec![9 + i, 10 + i, 11 + i] }
    }

    fn _value1() -> EncryptedValue {
        _value(1)
    }

    fn _value2() -> EncryptedValue {
        _value(2)
    }

    fn _tags() -> Vec<Tag> {
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));
        tags
    }

    fn _new_tags() -> Vec<Tag> {
        vec![
            Tag::Encrypted(vec![1, 1, 1], vec![2, 2, 2]),
            Tag::PlainText(vec![1, 1, 1], String::from("tag_value_3"))
        ]
    }

    fn _sort(mut v: Vec<Tag>) -> Vec<Tag> {
        v.sort();
        v
    }

    fn _custom_path() -> String {
        let mut path = EnvironmentUtils::tmp_path();
        path.push("custom_wallet_path");
        path.to_str().unwrap().to_owned()
    }
}
