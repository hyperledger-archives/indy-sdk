extern crate owning_ref;

mod query;
mod transaction;

use std;

use rusqlite;
use serde_json;

use self::owning_ref::OwningHandle;
use std::rc::Rc;

use utils::environment::EnvironmentUtils;
use errors::wallet::WalletStorageError;
use errors::common::CommonError;
use services::wallet::language;

use super::{StorageIterator, WalletStorageType, WalletStorage, StorageEntity, EncryptedValue, Tag, TagName};
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

type TagRetrieverOwned = OwningHandle<Rc<rusqlite::Connection>, Box<TagRetriever<'static>>>
;

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
    fn next(&mut self) -> Result<Option<StorageEntity>, WalletStorageError> {
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
                Ok(Some(StorageEntity::new(name, value, type_, tags)))
            },
            Some(Err(err)) => Err(WalletStorageError::from(err)),
            None => Ok(None)
        }
    }

    fn get_total_count(&self) -> Result<Option<usize>, WalletStorageError> {
        Ok(self.total_count)
    }
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

    fn create_path(name: &str) -> std::path::PathBuf {
        let mut path = EnvironmentUtils::wallet_path(name);
        path.push(_SQLITE_DB );
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
    fn get(&self, type_: &Vec<u8>, name: &Vec<u8>, options: &str) -> Result<StorageEntity, WalletStorageError> {
        let options: RecordOptions = if options == "{}" {
            RecordOptions::default()
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

         Ok(StorageEntity::new(name.clone(), value, type_, tags))
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
    fn add(&self, type_: &Vec<u8>, name: &Vec<u8>, value: &EncryptedValue, tags: &[Tag]) -> Result<(), WalletStorageError> {
        let tx: transaction::Transaction = transaction::Transaction::new(&self.conn, rusqlite::TransactionBehavior::Deferred)?;
        let res = tx.prepare_cached("INSERT INTO items (type, name, value, key) VALUES (?1, ?2, ?3, ?4)")?
                            .insert(&[type_, name, &value.data, &value.key]);

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
    
    fn update(&self, type_: &Vec<u8>, name: &Vec<u8>, value: &EncryptedValue) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("UPDATE items SET value = ?1, key = ?2 WHERE type = ?3 AND name = ?4")?
            .execute(&[&value.data, &value.key, type_, name]);

        match res {
            Ok(1) => Ok(()),
            Ok(0) => Err(WalletStorageError::ItemNotFound),
            Ok(count) => Err(WalletStorageError::CommonError(CommonError::InvalidState(format!("SQLite returned update row count: {}", count)))),
            Err(err) => Err(WalletStorageError::from(err)),
        }
    }

    fn add_tags(&self, type_: &Vec<u8>, name: &Vec<u8>, tags: &[Tag]) -> Result<(), WalletStorageError> {
        let tx: transaction::Transaction = transaction::Transaction::new(&self.conn, rusqlite::TransactionBehavior::Deferred)?;

        let res = tx.prepare_cached("SELECT id FROM items WHERE type = ?1 AND name = ?2")?
            .query_row(&[type_, name], |row| row.get(0));

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

    fn update_tags(&self, type_: &Vec<u8>, name: &Vec<u8>, tags: &[Tag]) -> Result<(), WalletStorageError> {
        let tx: transaction::Transaction = transaction::Transaction::new(&self.conn, rusqlite::TransactionBehavior::Deferred)?;

        let res = tx.prepare_cached("SELECT id FROM items WHERE type = ?1 AND name = ?2")?
            .query_row(&[type_, name], |row| row.get(0));

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

    fn delete_tags(&self, type_: &Vec<u8>, name: &Vec<u8>, tag_names: &[TagName]) -> Result<(), WalletStorageError> {
        let res = self.conn.prepare_cached("SELECT id FROM items WHERE type =?1 AND name = ?2")?
            .query_row(&[type_, name], |row| row.get(0));

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

    fn set_storage_metadata(&self, metadata: &Vec<u8>) -> Result<(), WalletStorageError> {
        match self.conn.execute("UPDATE metadata SET value = ?1",&[metadata]) {
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

    fn search(&self, type_: &Vec<u8>, query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, WalletStorageError> {
        let search_options = match options {
            None => SearchOptions::default(),
            Some(option_str) => serde_json::from_str(option_str)?
        };

        let total_count: Option<usize> = if search_options.retrieve_total_count {
            let (query_string, query_arguments) = query::wql_to_sql_count(type_, query)?;

            self.conn.query_row(
                &query_string,
                &query_arguments,
                |row| { let x: i64 = row.get(0); Some(x as usize) }
            )?
        } else {None};


        if search_options.retrieve_records {
            let fetch_options = RecordOptions {
                retrieve_value: search_options.retrieve_value,
                retrieve_tags: search_options.retrieve_tags,
                retrieve_type: search_options.retrieve_type,
            };

            let (query_string, query_arguments) = query::wql_to_sql(type_, query, options)?;

            let statement = self._prepare_statement(&query_string)?;
            let tag_retriever = if fetch_options.retrieve_tags {
                Some(TagRetriever::new_owned(self.conn.clone())?)
            } else {
                None
            };
            let storage_iterator = SQLiteStorageIterator::new(Some(statement), &query_arguments, fetch_options, tag_retriever, total_count)?;
            Ok(Box::new(storage_iterator))
        }
        else {
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
    /// Creates the SQLite DB file with the provided name in the path specified in the config file,
    /// and initializes the encryption keys needed for encryption and decryption of data.
    ///
    /// # Arguments
    ///
    ///  * `name` - name of the SQLite DB file
    ///  * `storage_config` - config containing the location of SQLite DB files
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
    fn create_storage(&self, name: &str, config: Option<&str>, credentials: &str, metadata: &Vec<u8>) -> Result<(), WalletStorageError> {
        let db_file_path = SQLiteStorageType::create_path(name);
        if db_file_path.exists() {
            return Err(WalletStorageError::AlreadyExists);
        }

        let conn = rusqlite::Connection::open(db_file_path.as_path())?;

        match conn.execute_batch(_CREATE_SCHEMA) {
            Ok(_) => match conn.execute("INSERT OR REPLACE INTO metadata(value) VALUES(?1)", &[metadata]) {
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
    /// Establishes a connection to the SQLite DB with the provided name located in the path
    /// specified in the config. In case of a succesfull onection returns a Storage object
    /// embedding the connection and the encryption keys that will be used for encryption and
    /// decryption operations.
    ///
    ///
    /// # Arguments
    ///
    ///  * `name` - name of the SQLite DB file
    ///  * `config` - config containing the location of SQLite DB files
    ///  * `runtime_config` - #TODO
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
    ///  * `WalletStorageError::NotFound` - File with the provided name not found
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn open_storage(&self, name: &str, config: Option<&str>, credentials: &str) -> Result<Box<WalletStorage>, WalletStorageError> {
        let db_file_path = SQLiteStorageType::create_path(name);

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
    use std::collections::HashMap;
    use std::env;


    fn _create_and_open_test_storage() -> Box<WalletStorage> {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();
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
        db_file_path.push(_SQLITE_DB );
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
    fn sqlite_storage_type_create_check_metadata() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_metadata = _get_test_keys();

        storage_type.create_storage("test_wallet", None, &"", &test_metadata).unwrap();

        let storage = storage_type.open_storage("test_wallet", None, &"").unwrap();
        let storage_metadata = storage.get_storage_metadata().unwrap();

        assert_eq!(test_metadata, storage_metadata);
    }


    // no wallet with given name
    #[test]
    fn sqlite_wallet_storage_type_delete_for_nonexisting_wallet() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_metadata = _get_test_keys();

        storage_type.create_storage("test_wallet", None, &"", &test_metadata).unwrap();
        let res_wrong = storage_type.delete_storage("test_wallet_wrong", None, &"");
        let res_ok = storage_type.delete_storage("test_wallet", None, &"");

        assert_match!(Err(WalletStorageError::NotFound), res_wrong);
        assert_match!(Ok(()), res_ok);
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
        let keys = test_keys.clone(); // TODO: fix this

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();
        assert_eq!(keys, test_keys);

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        storage.add(&type_, &name, &value, &tags).unwrap();
        let entity = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value);
        assert_eq!(tags, entity.tags.unwrap());
    }

    // update a value
    #[test]
    fn sqlite_storage_cannot_add_twice_the_same_key() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();
        let keys = test_keys.clone(); // TODO: fix this

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();
        assert_eq!(keys, test_keys);

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let value2 = EncryptedValue{data: vec![100, 150, 200], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        storage.add(&type_, &name, &value, &tags).unwrap();
        let entity = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        assert_eq!(value, entity.value.unwrap());
        assert_eq!(tags, entity.tags.unwrap());

        let res = storage.add(&type_, &name, &value2, &tags);
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
        let value = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        {
            let storage = storage_type.open_storage("test_wallet", None, "").unwrap();
            storage.add(&type_, &name, &value, &tags).unwrap();
        }

        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();
        let entity = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value);
        assert_eq!(tags, entity.tags.unwrap());
    }

    // get for non-existing key
    #[test]
    fn sqlite_storage_get_works_for_wrong_key() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();
        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();
        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        storage.add(&type_, &name, &value, &tags).unwrap();
        let res = storage.get(&type_, &vec![5, 6, 6], r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##);

        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    // sql cmd inject
    #[test]
    fn sqlite_wallet_storage_sql_cmd_inject() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        storage.add(&type_, &name, &value, &tags).unwrap();

        let entity = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();
        assert_eq!(value, entity_value);
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
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        storage.add(&type_, &name, &value, &tags).unwrap();
        let entity = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value);
        assert_eq!(tags, entity.tags.unwrap());

        storage.delete(&type_, &name).unwrap();
        let res = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    /** negative tests */

    #[test]
    fn sqlite_storage_delete_returns_error_item_not_found_if_no_such_key() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        storage.add(&type_, &name, &value, &tags).unwrap();
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
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();

        let type_: Vec<u8> = vec![1, 2, 3];
        let name1: Vec<u8> = vec![4, 5, 6];
        let name2: Vec<u8> = vec![4, 5, 8];
        let value1 = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let value2 = EncryptedValue{data: vec![100, 150, 200], key: vec![10, 11, 12]};
        let mut tags: Vec<Tag> = Vec::new();
        tags.push(Tag::Encrypted(vec![1, 5, 8], vec![3, 5, 6]));
        tags.push(Tag::PlainText(vec![1, 5, 8, 1], "Plain value".to_string()));

        storage.add(&type_, &name1, &value1, &tags).unwrap();
        storage.add(&type_, &name2, &value2, &tags).unwrap();

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
        assert_eq!(retrieved_value1, value1);
        assert_eq!(item1.type_.clone().unwrap(), type_);
        let item2 = results.get(&name2).unwrap();
        let retrieved_value2 = item2.clone().value.unwrap();
        assert_eq!(retrieved_value2, value2);
        assert_eq!(item2.type_.clone().unwrap(), type_);
  }

    #[test]
    fn sqlite_storage_get_all_with_no_existing_keys() {
        _prepare_path();
        let storage_type = SQLiteStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create_storage("test_wallet", None, "", &test_keys).unwrap();
        let storage = storage_type.open_storage("test_wallet", None, "").unwrap();
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
        let value1 = EncryptedValue{data: vec![7, 8, 9], key: vec![10, 11, 12]};
        let value2 = EncryptedValue{data: vec![100, 150, 200], key: vec![10, 11, 34]};
        let tags = Vec::new();

        storage.add(&type_, &name, &value1, &tags).unwrap();
        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(item.value.unwrap(), value1);
        storage.update(&type_, &name, &value2).unwrap();
        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(item.value.unwrap(), value2);
    }

    #[test]
    fn sqlite_storage_update_returns_error_on_bad_item_name() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let wrong_name = vec![100, 100, 100];
        let value1 = EncryptedValue{data: vec![7,8,9], key:vec![10, 10, 10]};
        let value2 = EncryptedValue{data: vec![20, 20, 20], key: vec![30, 30, 30]};
        let tags = Vec::new();

        storage.add(&type_, &name, &value1, &tags).unwrap();
        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(item.value.unwrap(), value1);
        let res = storage.update(&type_, &wrong_name, &value2);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_update_returns_error_on_bad_type() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let wrong_type = vec![1,1,1];
        let name = vec![4,5,6];
        let value1 = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let value2 = EncryptedValue{data: vec![20, 20, 20], key: vec![30, 30, 30]};
        let tags = Vec::new();

        storage.add(&type_, &name, &value1, &tags).unwrap();
        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(item.value.unwrap(), value1);
        let res = storage.update(&wrong_type, &name, &value2);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }


    /**
     * Add tags tests
     */

    #[test]
    fn sqlite_storage_add_tags() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag1 = Tag::Encrypted(vec![0, 0, 0], vec![9, 9, 9]);
        tags.push(tag1.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let mut new_tags = Vec::new();
        let tag2 = Tag::Encrypted(vec![1, 1, 1], vec![2, 2, 2]);
        let tag3 = Tag::PlainText(vec![1, 1, 1], String::from("tag_value_3"));
        new_tags.push(tag2);
        new_tags.push(tag3);

        storage.add_tags(&type_, &name, &new_tags).unwrap();

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        assert_eq!(item.value.unwrap(), value);

        let mut expected_tags = new_tags.clone();
        expected_tags.push(tag1);
        expected_tags.sort();

        let mut item_tags = item.tags.unwrap();
        item_tags.sort();

        assert_eq!(item_tags, expected_tags);
    }

    #[test]
    fn sqlite_storage_add_tags_returns_proper_error_if_wrong_name() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let tags = Vec::new();

        storage.add(&type_, &name, &value, &tags).unwrap();

        let mut new_tags = Vec::new();
        new_tags.push(Tag::Encrypted(vec![1, 1, 1], vec![2, 2, 2]));

        let res = storage.add_tags(&type_, &vec![100,100,100], &new_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_add_tags_returns_proper_error_if_wrong_type() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let tags = Vec::new();

        storage.add(&vec![200,200], &name, &value, &tags).unwrap();

        let mut new_tags = Vec::new();
        new_tags.push(Tag::Encrypted(vec![1, 1, 1], vec![2, 2, 2]));

        let res = storage.add_tags(&type_, &vec![100,100,100], &new_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res)
    }

    #[test]
    fn sqlite_storage_add_tags_if_already_exists() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag1 = Tag::Encrypted(vec![0, 0, 0], vec![9, 9, 9]);
        let tag2 = Tag::Encrypted(vec![1, 1, 1], vec![8, 8, 8]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let mut new_tags = Vec::new();
        let tag3 = Tag::Encrypted(vec![2, 2, 2], vec![7, 7, 7]);
        new_tags.push(tag3);
        new_tags.push(tag1);

        storage.add_tags(&type_, &name, &new_tags).unwrap();

        let mut expected_tags = new_tags.clone();
        expected_tags.push(tag2);
        expected_tags.sort();

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let mut item_tags = item.tags.unwrap();
        item_tags.sort();

        assert_eq!(item_tags, expected_tags);
    }


    /**
     * Update tags tests
     */

    #[test]
    fn sqlite_storage_update_tags() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag1 = Tag::Encrypted(vec![0, 0, 0], vec![0, 0, 0]);
        let tag2 = Tag::PlainText(vec![1, 1, 1], "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(vec![2, 2, 2], vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let mut updated_tags = Vec::new();
        let new_tag1 = Tag::Encrypted(vec![0, 0, 0], vec![10, 10, 10, 10]);
        let new_tag2 = Tag::PlainText(vec![1, 1, 1], "new_tag_value_2".to_string());
        updated_tags.push(new_tag1.clone());
        updated_tags.push(new_tag2.clone());

        storage.update_tags(&type_, &name, &updated_tags).unwrap();

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(retrieved_tags, updated_tags);
    }

    #[test]
    fn sqlite_storage_update_tags_returns_error_if_wrong_name() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag1 = Tag::Encrypted(vec![0, 0, 0], vec![0, 0, 0]);
        let tag2 = Tag::PlainText(vec![1, 1, 1], "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(vec![2, 2, 2], vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let mut updated_tags = Vec::new();
        let new_tag1 = Tag::Encrypted(vec![0, 0, 0], vec![10, 10, 10, 10]);
        let new_tag2 = Tag::PlainText(vec![1, 1, 1], "new_tag_value_2".to_string());
        updated_tags.push(new_tag1.clone());
        updated_tags.push(new_tag2.clone());

        let res = storage.update_tags(&type_, &vec![100, 100], &updated_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_update_tags_returns_error_if_wrong_type() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag1 = Tag::Encrypted(vec![0, 0, 0], vec![0, 0, 0]);
        let tag2 = Tag::PlainText(vec![1, 1, 1], "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(vec![2, 2, 2], vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let mut updated_tags = Vec::new();
        let new_tag1 = Tag::Encrypted(vec![0, 0, 0], vec![10, 10, 10, 10]);
        updated_tags.push(new_tag1.clone());

        let res = storage.update_tags(&vec![100, 100], &name, &updated_tags);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_update_tags_succedes_for_nonexistant_tag_and_works_atomically() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag1 = Tag::Encrypted(vec![0, 0, 0], vec![0, 0, 0]);
        let tag2 = Tag::PlainText(vec![1, 1, 1], "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(vec![2, 2, 2], vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let mut updated_tags = Vec::new();
        let new_tag1 = Tag::Encrypted(vec![0, 0, 0], vec![10, 10, 10, 10]);
        let new_tag2 = Tag::PlainText(vec![100, 100, 100], "new_tag_value_2".to_string());
        updated_tags.push(new_tag1.clone());
        updated_tags.push(new_tag2.clone());

        let res = storage.update_tags(&type_, &name, &updated_tags);
        assert_match!(Ok(()), res);

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        assert_eq!(retrieved_tags, updated_tags);
    }


    /**
     * Delete tags tests
     */
    #[test]
    fn sqlite_storage_delete_tags() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        storage.delete_tags(&type_, &name, &tag_names).unwrap();

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let retrieved_tags = item.tags.unwrap();
        let mut expected_tags = Vec::new();
        expected_tags.push(tag3.clone());
        assert_eq!(retrieved_tags, expected_tags);
    }

    #[test]
    fn sqlite_storage_returns_error_if_wrong_type() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        let res = storage.delete_tags(&vec![0, 0, 0], &name, &tag_names);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let mut retrieved_tags = item.tags.unwrap();
        retrieved_tags.sort();

        tags.sort();
        assert_eq!(retrieved_tags, tags);
    }

    #[test]
    fn sqlite_storage_returns_error_if_wrong_name() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone())];
        let res = storage.delete_tags(&type_, &vec![0, 0, 0], &tag_names);
        assert_match!(Err(WalletStorageError::ItemNotFound), res);

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let mut retrieved_tags = item.tags.unwrap();
        retrieved_tags.sort();

        tags.sort();
        assert_eq!(retrieved_tags, tags);
    }

    #[test]
    fn sqlite_storage_delete_tags_works_atomically_and_no_error_if_one_tag_name_is_wrong() {
        let storage = _create_and_open_test_storage();
        let type_ = vec![1,2,3];
        let name = vec![4,5,6];
        let value = EncryptedValue{data: vec![7,8,9], key: vec![10, 10, 10]};
        let mut tags = Vec::new();
        let tag_name1 = vec![0, 0, 0];
        let tag_name2 = vec![1, 1, 1];
        let tag_name3 = vec![2, 2, 2];
        let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
        let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
        let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
        tags.push(tag1.clone());
        tags.push(tag2.clone());
        tags.push(tag3.clone());

        storage.add(&type_, &name, &value, &tags).unwrap();

        let tag_names = vec![TagName::OfEncrypted(tag_name1.clone()), TagName::OfPlain(tag_name2.clone()), TagName::OfEncrypted(vec![100, 100, 100])];
        let res = storage.delete_tags(&type_, &name, &tag_names).unwrap();

        let item = storage.get(&type_, &name, r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##).unwrap();
        let mut retrieved_tags = item.tags.unwrap();
        retrieved_tags.sort();

        let mut expected_tags = Vec::new();
        expected_tags.push(tag3.clone());
        expected_tags.sort();

        assert_eq!(retrieved_tags, expected_tags);
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
