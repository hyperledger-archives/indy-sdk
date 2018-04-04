mod query;

use std;
use std::collections::HashMap;

use rusqlite;
use serde_json;

use language;
use config::{StorageConfig,WalletRuntimeConfig,StorageCredentials};

use super::{StorageIterator,StorageType,Storage,StorageEntity,StorageValue,TagValue};
use super::error::StorageError;


struct DefaultStorageIterator {
    rows: Vec<StorageEntity>,
    index: usize,
}


impl DefaultStorageIterator {
    fn new(rows: Vec<StorageEntity>) -> DefaultStorageIterator {
        DefaultStorageIterator {
            rows: rows,
            index: 0,
        }
    }
}


impl StorageIterator for DefaultStorageIterator {
    fn next(&mut self) -> Result<Option<StorageEntity>, StorageError> {
        if self.rows.len() <= self.index {
            return Ok(None);
        }

        let item = self.rows[self.index].clone();
        self.index += 1;
        Ok(Some(item))
    }
}

#[derive(Deserialize)]
struct FetchOptions {
    fetch_value: bool,
    fetch_tags: bool,
}


#[derive(Debug)]
struct DefaultStorage {
    conn: Option<rusqlite::Connection>,
}

pub struct DefaultStorageType {}


impl DefaultStorageType {
    pub fn new() -> DefaultStorageType {
        DefaultStorageType {}
    }

    fn create_path(config: &StorageConfig,  name: &str) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::from(&config.base);
        path.push(name);
        path
    }
}


#[warn(dead_code)]
impl Storage for DefaultStorage {
    ///
    /// Tries to fetch values and/or tags from the storage.
    /// Returns Result with StorageEntity object which holds requested data in case of success or
    /// Result with StorageError in case of failure.
    ///
    ///
    /// # Arguments
    ///
    ///  * `class` - class of the item in storag
    ///  * `name` - name of the item in storage
    ///  * `options` - JSon containing what needs to be fetched.
    ///  Example: {"fetch_value": true, "fetch_tags": true}
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `StorageEntity` - Contains name, optional value and optional tags
    ///  * `StorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `StorageError` class of errors can be throw by this method:
    ///
    ///  * `StorageError::Closed` - Storage is closed
    ///  * `StorageError::ItemNotFound` - Item is not found in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn get(&self, class: &Vec<u8>, name: &Vec<u8>, options: &str) -> Result<StorageEntity, StorageError> {
        let options: FetchOptions = serde_json::from_str(options)?;
        match self.conn {
            None => Err(StorageError::Closed),
            Some(ref conn) => {
                let res: Result<(i64, Vec<u8>, Vec<u8>), rusqlite::Error> = conn.query_row(
                    "SELECT id, value, key FROM items where type = ?1 AND name = ?2",
                    &[class, name],
                    |row| {
                        (row.get(0), row.get(1), row.get(2))
                    }
                );
                let item = match res {
                    Ok(entity) => entity,
                    Err(rusqlite::Error::QueryReturnedNoRows) => return Err(StorageError::ItemNotFound),
                    Err(err) => return Err(StorageError::from(err))
                };

                if options.fetch_tags {
                    let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();

                    // get all encrypted.
                    let mut stmt = conn.prepare_cached("SELECT name, value FROM tags_encrypted WHERE item_id = ?1")?;
                    let mut rows = stmt.query(&[&item.0])?;

                    while let Some(row) = rows.next() {
                        let row = row?;
                        tags.insert(row.get(0), TagValue::Encrypted(row.get(1)));
                    }

                    // get all plain
                    let mut stmt = conn.prepare_cached("SELECT name, value FROM tags_plaintext WHERE item_id = ?1")?;
                    let mut rows = stmt.query(&[&item.0])?;

                    while let Some(row) = rows.next() {
                        let row = row?;
                        tags.insert(row.get(0), TagValue::Plain(row.get(1)));
                    }

                    // get all meta
                    let mut stmt = conn.prepare_cached("SELECT name, value FROM tags_metadata WHERE item_id = ?1")?;
                    let mut rows = stmt.query(&[&item.0])?;

                    while let Some(row) = rows.next() {
                        let row = row?;
                        tags.insert(row.get(0), TagValue::Meta(row.get(1)));
                    }

                    Ok(StorageEntity::new(name.clone(),
                                          if options.fetch_value
                                              {Some(StorageValue::new(item.1, item.2))}
                                              else {None},
                                          Some(tags))
                    )
                }
                else {
                    Ok(StorageEntity::new(name.clone(),
                                          if options.fetch_value
                                              {Some(StorageValue::new(item.1, item.2))}
                                              else {None},
                                          None)
                    )
                }
            }
        }
    }

    ///
    /// inserts value and tags into storage.
    /// Returns Result with () on success or
    /// Result with StorageError in case of failure.
    ///
    ///
    /// # Arguments
    ///
    ///  * `class` - class of the item in storag
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
    ///  * `StorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `StorageError` class of errors can be throw by this method:
    ///
    ///  * `StorageError::Closed` - Storage is closed
    ///  * `StorageError::ItemAlreadyExists` - Item is already present in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn add(&self, class: &Vec<u8>, name: &Vec<u8>, value: &Vec<u8>, value_key: &Vec<u8>, tags: &HashMap<Vec<u8>, TagValue>) -> Result<(), StorageError> {
        match self.conn {
            None => Err(StorageError::Closed),
            Some(ref conn) => {
                let res = conn.prepare_cached("INSERT INTO items (type, name, value, key) VALUES (?1, ?2, ?3, ?4)")?
                    .insert(&[class, name, value, value_key]);

                let id = match res {
                    Ok(entity) => entity,
                    Err(rusqlite::Error::SqliteFailure(_, _)) => return Err(StorageError::ItemAlreadyExists),
                    Err(err) => return Err(StorageError::from(err))
                };

                let mut stmt_e = conn.prepare_cached("INSERT INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)")?;
                let mut stmt_p = conn.prepare_cached("INSERT INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)")?;
                let mut stmt_m = conn.prepare_cached("INSERT INTO tags_metadata (item_id, name, value) VALUES (?1, ?2, ?3)")?;

                for (tag_name, tag_value) in tags {
                    match tag_value {
                        &TagValue::Encrypted(ref tag_data) => stmt_e.execute(&[&id, tag_name, tag_data])?,
                        &TagValue::Plain(ref tag_data) => stmt_p.execute(&[&id, tag_name, tag_data])?,
                        &TagValue::Meta(ref tag_data) => stmt_m.execute(&[&id, tag_name, tag_data])?,
                    };
                }

                Ok(())
            }
        }
    }

    ///
    /// deletes value and tags into storage.
    /// Returns Result with () on success or
    /// Result with StorageError in case of failure.
    ///
    ///
    /// # Arguments
    ///
    ///  * `class` - class of the item in storag
    ///  * `name` - name of the item in storage
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `()`
    ///  * `StorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `StorageError` class of errors can be throw by this method:
    ///
    ///  * `StorageError::Closed` - Storage is closed
    ///  * `StorageError::ItemNotFound` - Item is not found in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn delete(&self, class: &Vec<u8>, name: &Vec<u8>) -> Result<(), StorageError> {
        match self.conn {
            None => Err(StorageError::Closed),
            Some(ref conn) => {
                let row_count = conn.execute(
                    "DELETE FROM items where type = ?1 AND name = ?2",
                    &[class, name]
                )?;
                if row_count == 1 {
                    Ok(())
                } else {
                    Err(StorageError::ItemNotFound)
                }
            }
        }
    }

    fn get_all(&self) -> Result<Box<StorageIterator>, StorageError> {
        match self.conn {
            None => Err(StorageError::Closed),
            Some(ref conn) => {
                let mut stmt = conn.prepare("SELECT i.id, i.type, i.name, i.value, i.key, te.name, te.value, tp.name, tp.value, tm.name, tm.value \
                                                            FROM items i \
                                                                LEFT JOIN tags_encrypted te ON te.item_id = i.id \
                                                                LEFT JOIN tags_plaintext tp ON tp.item_id = i.id \
                                                                LEFT JOIN tags_metadata tm ON tm.item_id = i.id \
                                                            ORDER BY i.id")?;
                let mut rows = stmt.query(&[])?;

                let mut entities = Vec::new();

                let mut old_id = -1;
                let mut entity: Option<StorageEntity> = None;
                while let Some(row) = rows.next() {
                    let row = row?;
                    let id: i64 = row.get(0);

                    let mut new_tags = HashMap::new();

                    if let Some(tag_name) = row.get(5) {
                        new_tags.insert(tag_name, TagValue::Encrypted(row.get(6)));
                    }
                    else if let Some(tag_name) = row.get(7) {
                        new_tags.insert(tag_name, TagValue::Plain(row.get(8)));
                    }
                    else if let Some(tag_name) = row.get(9) {
                        new_tags.insert(tag_name, TagValue::Meta(row.get(10)));
                    }

                    if id != old_id {
                        if let Some(x) = entity {
                            entities.push(x);
                        }

                        let value = StorageValue::new(row.get(3), row.get(4));

                        entity = Some(StorageEntity::new(row.get(2), Some(value), if new_tags.len() > 0 {Some(new_tags)} else {None}));
                        old_id = id;
                    }
                    else {
                        match entity {
                            None => {
                                // This should never happen.
                                return Err(StorageError::IOError("Current entity not exists".to_string()));
                            },
                            Some(ref mut entity) => {
                                match entity.tags {
                                    None => {
                                        // This should never happen.
                                        entity.tags = Some(new_tags);
                                    },
                                    Some(ref mut tags) => {
                                        tags.extend(new_tags);
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(Box::new(DefaultStorageIterator::new(entities)))
            }
        }
    }
    
    fn search(&self, class: &Vec<u8>, query: &language::Operator, options: Option<&str>) -> Result<Box<StorageIterator>, StorageError> {
        match self.conn {
            None => Err(StorageError::Closed),
            Some(ref conn) => {
                let (query_string, query_arguments) = query::wql_to_sql(class, query, options);
                println!("Query: {}", &query_string);
                let mut search_stmt = conn.prepare(&query_string)?;
                let results = search_stmt.query_map(
                    &query_arguments,
                    |row| {
                        let value = StorageValue::new(row.get(1), row.get(2));
                        // TODO - tags
                        println!("Value: {:?}", value);
                        StorageEntity::new(row.get(0), Some(value), None)
                    }
                )?;
                let mut entities = Vec::new();
                for res in results {
                    entities.push(res.unwrap());
                }
                Ok(Box::new(DefaultStorageIterator::new(entities)))
            }
        }
    }

    ///
    /// deletes all values and tags from storage.
    /// Returns Result with () on success or
    /// Result with StorageError in case of failure.
    ///
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `()`
    ///  * `StorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `StorageError` class of errors can be throw by this method:
    ///
    ///  * `StorageError::Closed` - Storage is closed
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn clear(&self) -> Result<(), StorageError> {
        match self.conn {
            None => Err(StorageError::Closed),
            Some(ref conn) => {
                // Default do not have TRUNCATE TABLE command
                conn.execute("DELETE FROM tags_encrypted", &[])?;
                conn.execute("DELETE FROM tags_plaintext", &[])?;
                conn.execute("DELETE FROM tags_metadata", &[])?;
                conn.execute("DELETE FROM items", &[])?;
                Ok(())
            }
        }
    }

    fn close(&mut self) -> Result<(), StorageError> {
        match self.conn {
            None => Err(StorageError::Closed),
            Some(_) => {
                let tmp_conn = std::mem::replace(&mut self.conn, None).unwrap();

                match tmp_conn.close() {
                    Ok(()) => Ok(()),
                    Err((conn, err)) => {
                        std::mem::replace(&mut self.conn, Some(conn));
                        Err(StorageError::from(err))
                    }
                }
            }
        }
    }
}


impl StorageType for DefaultStorageType {
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
    ///  * `StorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `StorageError` class of errors can be throw by this method:
    ///
    ///  * `AlreadyExists` - File with a given name already exists on the path
    ///  * `IOError("IO error during storage operation:...")` - Connection to the DB failed
    ///  * `IOError("Error occurred while creating wallet file:..)"` - Creation of schema failed
    ///  * `IOError("Error occurred while inserting the keys...")` - Insertion of keys failed
    ///  * `IOError(..)` - Deletion of the file form the file-system failed
    ///
    fn create(&self, name: &str, storage_config: &StorageConfig, storage_credentials: &StorageCredentials, keys: &Vec<u8>) -> Result<(), StorageError> {

        let db_file_path = DefaultStorageType::create_path(storage_config, name);

        if db_file_path.exists() {
            return Err(StorageError::AlreadyExists);
        }

        let conn = rusqlite::Connection::open(db_file_path.as_path())?;

        let schema_creation_stmt = "

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

            /*** MetaData Tags Table ***/

            CREATE TABLE tags_metadata(
                name NOT NULL,
                value NOT NULL,
                item_id INTEGER NOT NULL,
                PRIMARY KEY(name, item_id),
                FOREIGN KEY(item_id)
                    REFERENCES items(id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE
            );

            CREATE INDEX ix_tags_metadata_item_id ON tags_metadata(item_id);

            END TRANSACTION;
        ";

        match conn.execute_batch(schema_creation_stmt) {
            Ok(_) => match conn.execute("INSERT INTO keys(keys) VALUES(?1)", &[keys]) {
                Ok(_) => Ok(()),
                Err(error) => {
                    std::fs::remove_file(db_file_path)?;
                    Err(StorageError::IOError(format!("Error occurred while inserting the keys: {}", error)))
                }
            },
            Err(error) => {
                std::fs::remove_file(db_file_path)?;
                Err(StorageError::IOError(format!("Error occurred while creating wallet file: {}", error)))
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
    ///  * `StorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `StorageError` class of errors can be throw by this method:
    ///
    ///  * `StorageError::NotFound` - File with the provided name not found
    ///  * `IOError(..)` - Deletion of the file form the file-system failed
    ///
    fn delete(&self, name: &str, storage_config: &StorageConfig, storage_credentials: &StorageCredentials) -> Result<(), StorageError> {
        let db_file_path = DefaultStorageType::create_path(storage_config, name);

        if db_file_path.exists() {
            std::fs::remove_file(db_file_path)?;
            Ok(())
        } else {
            Err(StorageError::NotFound)
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
    ///  * `(Box<Storage>, Vec<u8>)` - Tuple of `DefaultStorage` and `encryption keys`
    ///  * `StorageError`
    ///
    /// # Errors
    ///
    /// Any of the following `StorageError` class of errors can be throw by this method:
    ///
    ///  * `StorageError::NotFound` - File with the provided name not found
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    fn open(&self, name: &str, storage_config: &StorageConfig, runtime_config: &WalletRuntimeConfig, storage_credentials: &StorageCredentials) -> Result<(Box<Storage>, Vec<u8>), StorageError> {
        let db_file_path = DefaultStorageType::create_path(storage_config, name);

        if !db_file_path.exists() {
            return Err(StorageError::NotFound);
        }

        let conn = rusqlite::Connection::open(db_file_path.as_path())?;
        let keys: Vec<u8> = conn.query_row("SELECT keys FROM keys LIMIT 1", &[], |row| row.get(0))?;

        Ok(
            (
                Box::new(DefaultStorage { conn: Some(conn) }),
                keys
            )
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::TagValue;
    use config::{StorageConfig, WalletRuntimeConfig};
    use std::fs::File;
    use std::io::prelude::*;
    use std::collections::HashMap;

    fn _remove_test_file(file: &str) {
        if std::path::Path::new(&file).exists() {
            std::fs::remove_file(&file).unwrap();
        }
    }

    fn _remove_test_wallet_file() {
        _remove_test_file("/tmp/test_wallet");
    }

    fn _make_test_storage_config() -> StorageConfig {
        let json_str = r##"{"base": "/tmp"}"##;
        let storage_config: StorageConfig = serde_json::from_str(json_str).unwrap();
        storage_config
    }


    /**
     * Storage config tests
     */

    fn _make_test_storage_credentials() -> StorageCredentials {
        let json_str = r##"{"username": "user1", "password": "pass123"}"##;
        let storage_credentials: StorageCredentials = serde_json::from_str(json_str).unwrap();
        storage_credentials
    }

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

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: ErrorImpl { code: EofWhileParsingString, line: 1, column: 16 }")]
    fn sqlite_wallet_storage_type_create_returns_error_for_not_a_json_path() {
        let config_str = r##"{"base": "/root}"##;
        let config: StorageConfig = serde_json::from_str(config_str).unwrap();
    }


    /**
     *  Create tests
     */
    #[test]
    fn sqlite_storage_type_create() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let storage_type = DefaultStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
    }

    /** negative tests */

    /** Invalid name and config tests - not in scope, wallet should pass only valid names */

    // no access
    #[test]
    fn sqlite_wallet_storage_type_create_returns_error_for_no_access_path() {
        let config_str = "{\"base\": \"/root\"}";
        let config: StorageConfig = serde_json::from_str(config_str).unwrap();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();

        let res = storage_type.create("test_wallet", &config, &test_storage_credentials, &test_keys);

        assert_match!(Err(StorageError::IOError(_)), res);
    }

    // no folder
    #[test]
    fn sqlite_wallet_storage_type_create_returns_error_for_no_folder() {
        let config_str = "{\"base\": \"/some/non/existing/folder\"}";
        let config: StorageConfig = serde_json::from_str(config_str).unwrap();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();

        let res = storage_type.create("test_wallet", &config, &test_storage_credentials, &test_keys);

        assert_match!(Err(StorageError::IOError(_)), res);
    }

    // not a folder (it's a file actually)
    #[test]
    fn sqlite_wallet_storage_type_create_returns_error_for_not_a_folder_path() {

        // prepare test file
        _remove_test_file("/tmp/foo.db");
        let mut file = File::create("/tmp/foo.db").unwrap();
        file.write_all(b"Hello, world!").unwrap();
        file.sync_all().unwrap();

        let config_str = "{\"base\": \"/tmp/foo.db\"}";
        let config: StorageConfig = serde_json::from_str(config_str).unwrap();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let storage_type = DefaultStorageType::new();

        let res = storage_type.create("test_wallet", &config, &test_storage_credentials, &test_keys);

        _remove_test_file("/tmp/foo.db");

        assert_match!(Err(StorageError::IOError(_)), res);
    }


    // Already existing wallet
    #[test]
    fn sqlite_storage_type_create_works_for_twice() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let storage_type = DefaultStorageType::new();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();

        let res = storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys);
        assert_match!(Err(StorageError::AlreadyExists), res);
    }

    /**
     * DefaultWalletStorageType Delete tests
     */

    /** postitive tests */
    #[test]
    fn sqlite_storage_type_create_check_keys() {
        // Prepare
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_wallet_path = DefaultStorageType::create_path(&test_storage_config, "test_wallet");
        let storage_type = DefaultStorageType::new();
        let test_keys = _get_test_keys();

        // Test
        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();

        // Assert
        let conn = rusqlite::Connection::open(&test_wallet_path).unwrap();
        let db_keys: Vec<u8> = conn.query_row("SELECT keys from keys LIMIT 1", &[], |row| row.get(0)).unwrap();
        assert_eq!(test_keys, db_keys);
    }

    // path traversal
    #[test]
    fn sqlite_wallet_storage_type_delete_returns_error_if_path_traversal_config() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let config_str = "{\"base\": \"/tmp/../tmp\"}";
        let config: StorageConfig = serde_json::from_str(config_str).unwrap();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &config, &test_storage_credentials, &test_keys).unwrap();
        storage_type.delete("test_wallet", &config, &test_storage_credentials).unwrap();
        storage_type.create("test_wallet", &config, &test_storage_credentials, &test_keys).unwrap();

        _remove_test_wallet_file();
    }


    /** negative tests */

    // no wallet with given name
    #[test]
    fn sqlite_wallet_storage_type_delete_for_nonexisting_wallet() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let storage_type = DefaultStorageType::new();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();

        let res = storage_type.delete("wrong_test_wallet", &test_storage_config, &test_storage_credentials);
        _remove_test_wallet_file();

        assert_match!(Err(StorageError::NotFound), res);
    }

    // no access
    #[test]
    fn sqlite_wallet_storage_type_delete_returns_error_if_no_access_config() {
        _remove_test_wallet_file();
        let bad_config_str = "{\"base\": \"/root\"}";
        let storage_type = DefaultStorageType::new();
        let bad_config: StorageConfig = serde_json::from_str(bad_config_str).unwrap();
        let test_wallet_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &test_wallet_config, &test_storage_credentials, &test_keys).unwrap();

        let res = storage_type.delete("test_wallet", &bad_config, &test_storage_credentials);
        _remove_test_wallet_file();

        assert_match!(Err(StorageError::NotFound), res);
    }

    //    // no folder
    #[test]
    fn sqlite_wallet_storage_type_delete_returns_error_if_no_folder_config() {
        _remove_test_wallet_file();
        let bad_config_str = "{\"base\": \"/some/non/existing/folder\"}";
        let storage_type = DefaultStorageType::new();
        let bad_config: StorageConfig = serde_json::from_str(bad_config_str).unwrap();
        let test_wallet_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &test_wallet_config, &test_storage_credentials, &test_keys).unwrap();

        let res = storage_type.delete("test_wallet", &bad_config, &test_storage_credentials);
        _remove_test_wallet_file();

        assert_match!(Err(StorageError::NotFound), res);
    }
    // not path
    #[test]
    fn sqlite_wallet_storage_type_delete_returns_error_if_not_path_config() {
        _remove_test_wallet_file();
        let bad_config_str = "{\"base\": \"@#@%#$^%$&^\"}";
        let storage_type = DefaultStorageType::new();
        let bad_config: StorageConfig = serde_json::from_str(bad_config_str).unwrap();
        let test_wallet_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", & test_wallet_config, &test_storage_credentials, &test_keys).unwrap();

        let res = storage_type.delete("test_wallet", &bad_config, &test_storage_credentials);
        _remove_test_wallet_file();

        assert_match!(Err(StorageError::NotFound), res);
    }

    // not a folder (file already)
    /** Not sure this is a valid test **/
    #[test]
    #[ignore]
    fn sqlite_wallet_storage_type_delete_returns_error_if_config_is_already_a_file() {
        _remove_test_wallet_file();

        // prepare test file
        _remove_test_file("/tmp/foo.db");
        let mut file = File::create("/tmp/foo.db").unwrap();
        file.write_all(b"Hello, world!").unwrap(); file.sync_all().unwrap();

        let bad_config_str = "{\"base\": \"/tmp/foo.db\"}";
        let bad_config: StorageConfig = serde_json::from_str(bad_config_str).unwrap();
        let test_wallet_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_wallet_config, &test_storage_credentials, &test_keys).unwrap();

        let res = storage_type.delete("wrong_test_wallet", &bad_config, &test_storage_credentials);

        _remove_test_file("/tmp/foo.db");

        assert_match!(Err(StorageError::ConfigError), res);
    }


    /**
     * DefaultWalletStorageType Open tests
     */

    #[test]
    fn sqlite_storage_type_open_works() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        storage_type.open("test_wallet", &test_storage_config, &WalletRuntimeConfig::default(), &test_storage_credentials).unwrap();
    }

    /** negative tests */
    // wallet not created
    #[test]
    fn sqlite_storage_type_open_returns_error_if_wallet_not_created() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let storage_type = DefaultStorageType::new();
        let res = storage_type.open("test_wallet", &test_storage_config, &WalletRuntimeConfig::default(), &test_storage_credentials);
        assert_match!(Err(StorageError::NotFound), res);
    }


    /**
     * Get/Set tests
     */

    #[test]
    fn sqlite_storage_set_get_works() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8, 1], TagValue::Encrypted(vec![3, 5, 6, 1]));
        tags.insert(vec![1, 5, 8, 2], TagValue::Encrypted(vec![3, 5, 6, 2]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &WalletRuntimeConfig::default(), &test_storage_credentials).unwrap();
        assert_eq!(keys, test_keys);

        storage.add(&class, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());
    }

    #[test]
    fn sqlite_storage_set_get_works_with_no_tags() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let tags: HashMap<Vec<u8>, TagValue> = HashMap::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &WalletRuntimeConfig::default(), &test_storage_credentials).unwrap();
        assert_eq!(keys, test_keys);

        storage.add(&class, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());
    }

    // update a value
    #[test]
    fn sqlite_storage_cannot_add_twice_the_same_key() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (wallet, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        wallet.add(&class, &name, &value, &value_key, &tags).unwrap();
        let entity = wallet.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": false}"##).unwrap();
        assert_eq!(value, entity.value.unwrap().data);

        let res = wallet.add(&class, &name, &value2, &value_key, &tags);
        assert_match!(Err(StorageError::ItemAlreadyExists), res);
    }

    // get set for reopen
    #[test]
    fn sqlite_storage_set_get_works_for_reopen() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let storage_type = DefaultStorageType::new();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();

        {
            let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();
            storage.add(&class, &name, &value, &value_key, &tags).unwrap();
        }

        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##).unwrap();
        let entity_value = entity.value.unwrap();

        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());
    }

    // get for non-existing key
    #[test]
    fn sqlite_storage_get_works_for_unknown() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();

        let (wallet, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();
        let entity = wallet.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##);
        assert_match!(Err(StorageError::ItemNotFound), entity);
    }

    /** negative tests */
    #[test]
    fn sqlite_storage_returns_error_closed_if_set_called_after_closing() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (mut storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();
        storage.close().unwrap();

        let res = storage.add(&class, &name, &value, &value_key, &tags);

        assert_match!(Err(StorageError::Closed), res);
    }

    #[test]
    fn sqlite_storage_returns_error_closed_if_get_called_after_closing() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (mut storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        storage.add(&class, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();
        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());

        storage.close().unwrap();

        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##);
        assert_match!(Err(StorageError::Closed), entity);
    }


    // sql cmd inject
    #[test]
    fn sqlite_wallet_storage_sql_cmd_inject() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let storage_type = DefaultStorageType::new();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Plain("value'); DROP TABLE items; --".to_string()));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        storage.add(&class, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();
        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());
    }

    /** Get/Set tests end */

    /**
     * Unset tests
     */

    #[test]
    fn sqlite_storage_unset() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();
        storage.add(&class, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##).unwrap();

        let entity_value = entity.value.unwrap();
        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());

        storage.delete(&class, &name).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##);
        assert_match!(Err(StorageError::ItemNotFound), entity);
    }

    /** negative tests */

    #[test]
    fn sqlite_storage_unset_returns_error_item_not_found_if_no_such_key() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        let res = storage.delete(&class, &name);

        assert_match!(Err(StorageError::ItemNotFound), res);
    }

    #[test]
    fn sqlite_storage_unset_returns_error_closed_if_wallet_closed() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (mut storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        storage.close().unwrap();
        let res = storage.delete(&class, &name);

        assert_match!(Err(StorageError::Closed), res);
    }

    /** Unset tests - END **/


    // TODO: fix when implementing get_all()
    /**
     * Get all tests
     */

    #[test]
    fn sqlite_storage_get_all_with_3_existing_keys() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name1: Vec<u8> = vec![4, 5, 6];
        let name2: Vec<u8> = vec![4, 5, 8];
        let name3: Vec<u8> = vec![4, 5, 9];
        let value1: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value3: Vec<u8> = vec![7, 8, 9, 11];
        let value_key1: Vec<u8> = vec![10, 11, 12, 1];
        let value_key2: Vec<u8> = vec![10, 11, 12, 2];
        let value_key3: Vec<u8> = vec![10, 11, 12, 3];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8, 1], TagValue::Encrypted(vec![3, 5, 6]));
        tags.insert(vec![1, 5, 8, 2], TagValue::Plain("Plain".to_string()));
        tags.insert(vec![1, 5, 8, 3], TagValue::Meta(vec![3, 5, 6, 11, 2]));
        tags.insert(vec![1, 5, 8, 3, 2], TagValue::Meta(vec![3, 5, 6, 11, 2, 3]));

        let mut expected_entities = HashMap::new();
        expected_entities.insert(name1.clone(), StorageEntity::new(name1.clone(), Some(StorageValue::new(value1.clone(), value_key1.clone())), Some(tags.clone())));
        expected_entities.insert(name2.clone(), StorageEntity::new(name2.clone(), Some(StorageValue::new(value2.clone(), value_key2.clone())), Some(tags.clone())));
        expected_entities.insert(name3.clone(), StorageEntity::new(name3.clone(), Some(StorageValue::new(value3.clone(), value_key3.clone())), None));

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        storage.add(&class, &name1, &value1, &value_key1, &tags).unwrap();
        storage.add(&class, &name2, &value2, &value_key2, &tags).unwrap();
        storage.add(&class, &name3, &value3, &value_key3, &HashMap::new()).unwrap();

        let mut storage_iterator = storage.get_all().unwrap();

        let mut entities = HashMap::new();
        loop {
            let entity = storage_iterator.next().unwrap();
            match entity {
                None => break,
                Some(entity) => entities.insert(entity.name.clone(), entity)
            };
        }

        assert_eq!(entities, expected_entities);
  }

    #[test]
    fn sqlite_storage_get_all_with_no_existing_keys() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (wallet, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        let mut storage_iterator = wallet.get_all().unwrap();
        let res = storage_iterator.next().unwrap();

        assert!(res.is_none());
    }

    /** negative tests */

    #[test]
    fn sqlite_storage_get_all_returns_error_if_wallet_closed() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (mut wallet, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();
        wallet.close().unwrap();

        let res = wallet.get_all();

        assert_match!(res, StorageError::Closed)
    }


    /**
     * Clear tests
     */

    #[test]
    fn sqlite_storage_clear() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name1: Vec<u8> = vec![4, 5, 6];
        let name2: Vec<u8> = vec![4, 5, 8];
        let value1: Vec<u8> = vec![7, 8, 9];
        let value2: Vec<u8> = vec![7, 8, 9, 10];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        storage.add(&class, &name1, &value1, &value_key, &tags).unwrap();
        storage.add(&class, &name2, &value2, &value_key, &tags).unwrap();

        let mut storage_iterator = storage.get_all().unwrap();

        // TODO: fix when implementing get_all()
//        let res1 = storage_iterator.next().unwrap().unwrap();
//        assert_eq!(res1.name, "key1");
//        assert_eq!(res1.value, "value1");
//
//        let res2 = storage_iterator.next().unwrap().unwrap();
//        assert_eq!(res2.name, "key2");
//        assert_eq!(res2.value, "value2");

        storage.clear().unwrap();
        let mut storage_iterator = storage.get_all().unwrap();
        let res = storage_iterator.next().unwrap();
        assert!(res.is_none());
    }

    /** negative tests */

    #[test]
    fn sqlite_storage_clear_returns_error_if_wallet_closed() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (wallet, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        let res = wallet.get_all();

        assert_match!(res, StorageError::Closed);
    }

    /**
     * Close tests
     */

    #[test]
    fn sqlite_storage_close() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let class: Vec<u8> = vec![1, 2, 3];
        let name: Vec<u8> = vec![4, 5, 6];
        let value: Vec<u8> = vec![7, 8, 9];
        let value_key: Vec<u8> = vec![10, 11, 12];
        let mut tags: HashMap<Vec<u8>, TagValue> = HashMap::new();
        tags.insert(vec![1, 5, 8], TagValue::Encrypted(vec![3, 5, 6]));

        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (mut storage, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();
        storage.add(&class, &name, &value, &value_key, &tags).unwrap();
        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##).unwrap();
        let entity_value = entity.value.unwrap();
        assert_eq!(value, entity_value.data);
        assert_eq!(value_key, entity_value.key);
        assert_eq!(tags, entity.tags.unwrap());


        let res = storage.close();
        assert_match!(Ok(()), res);

        let entity = storage.get(&class, &name, r##"{"fetch_value": true, "fetch_tags": true}"##);
        assert_match!(Err(StorageError::Closed), entity);
    }


    #[test]
    fn sqlite_storage_close_returns_error_closed_if_already_closed() {
        _remove_test_wallet_file();
        let test_storage_config = _make_test_storage_config();
        let runtime_config = WalletRuntimeConfig::default();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();
        let storage_type = DefaultStorageType::new();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        let (mut wallet, keys) = storage_type.open("test_wallet", &test_storage_config, &runtime_config, &test_storage_credentials).unwrap();

        let res = wallet.close();
        assert_match!(Ok(()), res);

        let res = wallet.close();
        assert_match!(Err(StorageError::Closed), res);
    }

    /**
     * Delete tests
     */

    #[test]
    fn sqlite_storage_type_delete_works() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
        storage_type.delete("test_wallet", &test_storage_config, &test_storage_credentials).unwrap();
        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();
    }

    /** Delete negative tests */
    #[test]
    fn sqlite_storage_type_delete_for_nonexisting_wallet() {
        _remove_test_wallet_file();
        let storage_type = DefaultStorageType::new();
        let test_storage_config = _make_test_storage_config();
        let test_storage_credentials = _make_test_storage_credentials();
        let test_keys = _get_test_keys();

        storage_type.create("test_wallet", &test_storage_config, &test_storage_credentials, &test_keys).unwrap();

        let res = storage_type.delete("wrong_test_wallet", &test_storage_config, &test_storage_credentials);

        assert_match!(Err(StorageError::NotFound), res);
    }


}
