use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use indy_api_types::errors::prelude::*;
use indy_utils::environment;
use serde::Deserialize;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
    ConnectOptions, Done, SqlitePool,
};

use async_trait::async_trait;

use crate::{
    language,
    storage::{StorageIterator, StorageRecord, Tag, TagName, WalletStorage, WalletStorageType},
    wallet::EncryptedValue,
    RecordOptions, SearchOptions,
};

mod query;

const _SQLITE_DB: &str = "sqlite.db";

struct SQLiteStorageIterator {
    records: Option<VecDeque<StorageRecord>>,
    total_count: Option<usize>,
}

impl SQLiteStorageIterator {
    fn new(
        records: Option<VecDeque<StorageRecord>>,
        total_count: Option<usize>,
    ) -> IndyResult<SQLiteStorageIterator> {
        Ok(SQLiteStorageIterator {
            records,
            total_count,
        })
    }
}

#[async_trait]
impl StorageIterator for SQLiteStorageIterator {
    async fn next(&mut self) -> IndyResult<Option<StorageRecord>> {
        if let Some(ref mut records) = self.records {
            Ok(records.pop_front())
        } else {
            Ok(None)
        }
    }

    fn get_total_count(&self) -> IndyResult<Option<usize>> {
        Ok(self.total_count.to_owned())
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    path: Option<String>,
}

#[derive(Debug)]
struct SQLiteStorage {
    pool: SqlitePool,
}

pub struct SQLiteStorageType {}

impl SQLiteStorageType {
    pub fn new() -> SQLiteStorageType {
        SQLiteStorageType {}
    }

    fn _db_path(id: &str, config: Option<&Config>) -> std::path::PathBuf {
        let mut path = match config {
            Some(Config {
                path: Some(ref path),
            }) => std::path::PathBuf::from(path),
            _ => environment::wallet_home_path(),
        };

        path.push(id);
        path.push(_SQLITE_DB);
        path
    }
}

#[async_trait]
impl WalletStorage for SQLiteStorage {
    ///
    /// Tries to fetch values and/or tags from the storage.
    /// Returns Result with StorageEntity object which holds requested data in case of success or
    /// Result with IndyError in case of failure.
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
    ///  * `IndyError`
    ///
    /// # Errors
    ///
    /// Any of the following `IndyError` type_ of errors can be throw by this method:
    ///
    ///  * `IndyError::Closed` - Storage is closed
    ///  * `IndyError::ItemNotFound` - Item is not found in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    async fn get(&self, type_: &[u8], id: &[u8], options: &str) -> IndyResult<StorageRecord> {
        let options: RecordOptions = serde_json::from_str(options).to_indy(
            IndyErrorKind::InvalidStructure,
            "RecordOptions is malformed json",
        )?;

        let mut conn = self.pool.acquire().await?;

        let (item_id, value, key): (i64, Vec<u8>, Vec<u8>) =
            sqlx::query_as("SELECT id, value, key FROM items where type = ?1 AND name = ?2")
                .bind(type_)
                .bind(id)
                .fetch_one(&mut conn)
                .await?;

        let value = if options.retrieve_value {
            Some(EncryptedValue::new(value, key))
        } else {
            None
        };

        let type_ = if options.retrieve_type {
            Some(type_.to_vec())
        } else {
            None
        };

        let tags = if options.retrieve_tags {
            let mut tags = Vec::new();

            tags.extend(
                sqlx::query_as::<_, (Vec<u8>, String)>(
                    "SELECT name, value from tags_plaintext where item_id = ?",
                )
                .bind(item_id)
                .fetch_all(&mut conn)
                .await?
                .drain(..)
                .map(|r| Tag::PlainText(r.0, r.1)),
            );

            tags.extend(
                sqlx::query_as::<_, (Vec<u8>, Vec<u8>)>(
                    "SELECT name, value from tags_encrypted where item_id = ?",
                )
                .bind(item_id)
                .fetch_all(&mut conn)
                .await?
                .drain(..)
                .map(|r| Tag::Encrypted(r.0, r.1)),
            );

            Some(tags)
        } else {
            None
        };

        Ok(StorageRecord::new(id.to_vec(), value, type_, tags))
    }

    ///
    /// inserts value and tags into storage.
    /// Returns Result with () on success or
    /// Result with IndyError in case of failure.
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
    ///  * `IndyError`
    ///
    /// # Errors
    ///
    /// Any of the following `IndyError` class of errors can be throw by this method:
    ///
    ///  * `IndyError::Closed` - Storage is closed
    ///  * `IndyError::ItemAlreadyExists` - Item is already present in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    async fn add(
        &self,
        type_: &[u8],
        id: &[u8],
        value: &EncryptedValue,
        tags: &[Tag],
    ) -> IndyResult<()> {
        let mut tx = self.pool.begin().await?;

        let id = sqlx::query("INSERT INTO items (type, name, value, key) VALUES (?1, ?2, ?3, ?4)")
            .bind(type_)
            .bind(id)
            .bind(&value.data)
            .bind(&value.key)
            .execute(&mut tx)
            .await?
            .last_insert_rowid();

        for tag in tags {
            match *tag {
                Tag::Encrypted(ref tag_name, ref tag_data) => {
                    sqlx::query(
                        "INSERT INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)",
                    )
                    .bind(id)
                    .bind(tag_name)
                    .bind(tag_data)
                    .execute(&mut tx)
                    .await?
                }
                Tag::PlainText(ref tag_name, ref tag_data) => {
                    sqlx::query(
                        "INSERT INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)",
                    )
                    .bind(id)
                    .bind(tag_name)
                    .bind(tag_data)
                    .execute(&mut tx)
                    .await?
                }
            };
        }

        tx.commit().await?;
        Ok(())
    }

    async fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> IndyResult<()> {
        let mut tx = self.pool.begin().await?;

        let row_updated =
            sqlx::query("UPDATE items SET value = ?1, key = ?2 WHERE type = ?3 AND name = ?4")
                .bind(&value.data)
                .bind(&value.key)
                .bind(&type_)
                .bind(&id)
                .execute(&mut tx)
                .await?
                .rows_affected();

        match row_updated {
            1 => {
                tx.commit().await?;
                Ok(())
            }
            0 => Err(err_msg(
                IndyErrorKind::WalletItemNotFound,
                "Item to update not found",
            )),
            _ => Err(err_msg(
                IndyErrorKind::InvalidState,
                "More than one row update. Seems wallet structure is inconsistent",
            )),
        }
    }

    async fn add_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> IndyResult<()> {
        let mut tx = self.pool.begin().await?;

        let (item_id,): (i64,) =
            sqlx::query_as("SELECT id FROM items WHERE type = ?1 AND name = ?2")
                .bind(type_)
                .bind(id)
                .fetch_one(&mut tx)
                .await?;

        for tag in tags {
            match *tag {
                Tag::Encrypted(ref tag_name, ref tag_data) => {
                    sqlx::query(
                        "INSERT OR REPLACE INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)",
                    )
                    .bind(item_id)
                    .bind(tag_name)
                    .bind(tag_data)
                    .execute(&mut tx)
                    .await?
                }
                Tag::PlainText(ref tag_name, ref tag_data) => {
                    sqlx::query(
                        "INSERT OR REPLACE INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)",
                    )
                    .bind(item_id)
                    .bind(tag_name)
                    .bind(tag_data)
                    .execute(&mut tx)
                    .await?
                }
            };
        }

        tx.commit().await?;
        Ok(())
    }

    async fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> IndyResult<()> {
        let mut tx = self.pool.begin().await?;

        let (item_id,): (i64,) =
            sqlx::query_as("SELECT id FROM items WHERE type = ?1 AND name = ?2")
                .bind(type_)
                .bind(&id)
                .fetch_one(&mut tx)
                .await?;

        sqlx::query("DELETE FROM tags_encrypted WHERE item_id = ?1")
            .bind(item_id)
            .execute(&mut tx)
            .await?;

        sqlx::query("DELETE FROM tags_plaintext WHERE item_id = ?1")
            .bind(item_id)
            .execute(&mut tx)
            .await?;

        for tag in tags {
            match *tag {
                Tag::Encrypted(ref tag_name, ref tag_data) => {
                    sqlx::query(
                        "INSERT INTO tags_encrypted (item_id, name, value) VALUES (?1, ?2, ?3)",
                    )
                    .bind(item_id)
                    .bind(tag_name)
                    .bind(tag_data)
                    .execute(&mut tx)
                    .await?
                }
                Tag::PlainText(ref tag_name, ref tag_data) => {
                    sqlx::query(
                        "INSERT INTO tags_plaintext (item_id, name, value) VALUES (?1, ?2, ?3)",
                    )
                    .bind(item_id)
                    .bind(tag_name)
                    .bind(tag_data)
                    .execute(&mut tx)
                    .await?
                }
            };
        }

        tx.commit().await?;

        Ok(())
    }

    async fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> IndyResult<()> {
        let mut tx = self.pool.begin().await?;

        let (item_id,): (i64,) =
            sqlx::query_as("SELECT id FROM items WHERE type = ?1 AND name = ?2")
                .bind(type_)
                .bind(id)
                .fetch_one(&mut tx)
                .await?;

        for tag_name in tag_names {
            match *tag_name {
                TagName::OfEncrypted(ref tag_name) => {
                    sqlx::query("DELETE FROM tags_encrypted WHERE item_id = ?1 AND name = ?2")
                        .bind(item_id)
                        .bind(tag_name)
                        .execute(&mut tx)
                        .await?
                }
                TagName::OfPlain(ref tag_name) => {
                    sqlx::query("DELETE FROM tags_plaintext WHERE item_id = ?1 AND name = ?2")
                        .bind(item_id)
                        .bind(tag_name)
                        .execute(&mut tx)
                        .await?
                }
            };
        }

        tx.commit().await?;
        Ok(())
    }

    ///
    /// deletes value and tags into storage.
    /// Returns Result with () on success or
    /// Result with IndyError in case of failure.
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
    ///  * `IndyError`
    ///
    /// # Errors
    ///
    /// Any of the following `IndyError` type_ of errors can be throw by this method:
    ///
    ///  * `IndyError::Closed` - Storage is closed
    ///  * `IndyError::ItemNotFound` - Item is not found in database
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    async fn delete(&self, type_: &[u8], id: &[u8]) -> IndyResult<()> {
        let mut tx = self.pool.begin().await?;

        let rows_affected = sqlx::query("DELETE FROM items where type = ?1 AND name = ?2")
            .bind(type_)
            .bind(id)
            .execute(&mut tx)
            .await?
            .rows_affected();

        match rows_affected {
            1 => {
                tx.commit().await?;
                Ok(())
            }
            0 => Err(err_msg(
                IndyErrorKind::WalletItemNotFound,
                "Item to delete not found",
            )),
            _ => Err(err_msg(
                IndyErrorKind::InvalidState,
                "More than one row deleted. Seems wallet structure is inconsistent",
            )),
        }
    }

    async fn get_storage_metadata(&self) -> IndyResult<Vec<u8>> {
        let mut conn = self.pool.acquire().await?;

        let (metadata,): (Vec<u8>,) = sqlx::query_as::<_, (Vec<u8>,)>("SELECT value FROM metadata")
            .fetch_one(&mut conn)
            .await?;

        Ok(metadata)
    }

    async fn set_storage_metadata(&self, metadata: &[u8]) -> IndyResult<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("UPDATE metadata SET value = ?1")
            .bind(metadata)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn get_all(&self) -> IndyResult<Box<dyn StorageIterator>> {
        let mut conn = self.pool.acquire().await?;
        let mut tags: Vec<(i64, Tag)> = Vec::new();

        tags.extend(
            sqlx::query_as::<_, (i64, Vec<u8>, String)>(
                "SELECT item_id, name, value from tags_plaintext",
            )
            .fetch_all(&mut conn)
            .await?
            .drain(..)
            .map(|r| (r.0, Tag::PlainText(r.1, r.2))),
        );

        tags.extend(
            sqlx::query_as::<_, (i64, Vec<u8>, Vec<u8>)>(
                "SELECT item_id, name, value from tags_encrypted",
            )
            .fetch_all(&mut conn)
            .await?
            .drain(..)
            .map(|r| (r.0, Tag::Encrypted(r.1, r.2))),
        );

        let mut mtags = HashMap::new();

        for (k, v) in tags {
            mtags.entry(k).or_insert_with(Vec::new).push(v)
        }

        let records: VecDeque<_> = sqlx::query_as::<_, (i64, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>(
            "SELECT id, name, value, key, type FROM items",
        )
        .fetch_all(&mut conn)
        .await?
        .drain(..)
        .map(|r| {
            StorageRecord::new(
                r.1,
                Some(EncryptedValue::new(r.2, r.3)),
                Some(r.4),
                mtags.remove(&r.0).or_else(|| Some(Vec::new())),
            )
        })
        .collect();

        let total_count = records.len();

        Ok(Box::new(SQLiteStorageIterator::new(
            Some(records),
            Some(total_count),
        )?))
    }

    async fn search(
        &self,
        type_: &[u8],
        query: &language::Operator,
        options: Option<&str>,
    ) -> IndyResult<Box<dyn StorageIterator>> {
        let options = if let Some(options) = options {
            serde_json::from_str(options).to_indy(
                IndyErrorKind::InvalidStructure,
                "Search options is malformed json",
            )?
        } else {
            SearchOptions::default()
        };

        let mut conn = self.pool.acquire().await?;

        let records = if options.retrieve_records {
            let (query, args) = query::wql_to_sql(type_, query, None)?;

            // "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i WHERE i.type = ?"

            let mut query =
                sqlx::query_as::<sqlx::Sqlite, (i64, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>)>(&query);

            for arg in args.iter() {
                query = match arg {
                    query::ToSQL::ByteSlice(a) => query.bind(a),
                    query::ToSQL::CharSlice(a) => query.bind(a),
                }
            }

            let mut records = query.fetch_all(&mut conn).await?;

            let mut mtags = if options.retrieve_tags && records.len() > 0 {
                let mut tags: Vec<(i64, Tag)> = Vec::new();

                let in_binings = std::iter::repeat("?").take(records.len()).collect::<Vec<_>>().join(",");

                let query = format!(
                    r#"
                    SELECT item_id, name, value
                    FROM tags_plaintext
                    WHERE item_id IN ({})
                    "#,
                    in_binings
                );

                let mut query = sqlx::query_as::<sqlx::Sqlite, (i64, Vec<u8>, String)>(&query);

                for record in records.iter() {
                    query = query.bind(record.0);
                }

                tags.extend(
                    query
                        .fetch_all(&mut conn)
                        .await?
                        .drain(..)
                        .map(|r| (r.0, Tag::PlainText(r.1, r.2))),
                );

                let query = format!(
                    r#"
                    SELECT item_id, name, value
                    FROM tags_encrypted
                    WHERE item_id IN ({})
                    "#,
                    in_binings
                );

                let mut query = sqlx::query_as::<sqlx::Sqlite, (i64, Vec<u8>, Vec<u8>)>(&query);

                for record in records.iter() {
                    query = query.bind(record.0);
                }

                tags.extend(
                    query
                        .fetch_all(&mut conn)
                        .await?
                        .drain(..)
                        .map(|r| (r.0, Tag::Encrypted(r.1, r.2))),
                );

                let mut mtags = HashMap::new();

                for (k, v) in tags {
                    mtags.entry(k).or_insert_with(Vec::new).push(v)
                }

                mtags
            } else {
                HashMap::new()
            };

            let records = records
                .drain(..)
                .map(|r| {
                    StorageRecord::new(
                        r.1,
                        if options.retrieve_value {
                            Some(EncryptedValue::new(r.2, r.3))
                        } else {
                            None
                        },
                        if options.retrieve_type {
                            Some(r.4)
                        } else {
                            None
                        },
                        if options.retrieve_tags {
                            mtags.remove(&r.0).or_else(|| Some(Vec::new()))
                        } else {
                            None
                        },
                    )
                })
                .collect();

            Some(records)
        } else {
            None
        };

        let total_count = if options.retrieve_total_count {
            let (query, mut args) = query::wql_to_sql_count(&type_, query)?;

            let mut query = sqlx::query_as::<sqlx::Sqlite, (i64,)>(&query);

            while let Some(arg) = args.pop() {
                query = match arg {
                    query::ToSQL::ByteSlice(a) => query.bind(a),
                    query::ToSQL::CharSlice(a) => query.bind(a),
                }
            }

            let (total_count,) = query.fetch_one(&mut conn).await?;
            Some(total_count as usize)
        } else {
            None
        };

        Ok(Box::new(SQLiteStorageIterator::new(records, total_count)?))
    }

    fn close(&mut self) -> IndyResult<()> {
        Ok(())
    }
}

#[async_trait]
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
    ///  * `IndyError`
    ///
    /// # Errors
    ///
    /// Any of the following `IndyError` type_ of errors can be throw by this method:
    ///
    ///  * `IndyError::NotFound` - File with the provided id not found
    ///  * `IOError(..)` - Deletion of the file form the file-system failed
    ///
    async fn delete_storage(
        &self,
        id: &str,
        config: Option<&str>,
        _credentials: Option<&str>,
    ) -> IndyResult<()> {
        let config = config
            .map(serde_json::from_str::<Config>)
            .map_or(Ok(None), |v| v.map(Some))
            .to_indy(IndyErrorKind::InvalidStructure, "Malformed config json")?;

        let db_file_path = SQLiteStorageType::_db_path(id, config.as_ref());

        if !db_file_path.exists() {
            return Err(err_msg(
                IndyErrorKind::WalletNotFound,
                format!("Wallet storage file isn't found: {:?}", db_file_path),
            ));
        }

        std::fs::remove_dir_all(db_file_path.parent().unwrap())?;
        Ok(())
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
    ///  * `IndyError`
    ///
    /// # Errors
    ///
    /// Any of the following `IndyError` type_ of errors can be throw by this method:
    ///
    ///  * `AlreadyExists` - File with a given name already exists on the path
    ///  * `IOError("IO error during storage operation:...")` - Connection to the DB failed
    ///  * `IOError("Error occurred while creating wallet file:..)"` - Creation of schema failed
    ///  * `IOError("Error occurred while inserting the keys...")` - Insertion of keys failed
    ///  * `IOError(..)` - Deletion of the file form the file-system failed
    ///
    async fn create_storage(
        &self,
        id: &str,
        config: Option<&str>,
        _credentials: Option<&str>,
        metadata: &[u8],
    ) -> IndyResult<()> {
        let config = config
            .map(serde_json::from_str::<Config>)
            .map_or(Ok(None), |v| v.map(Some))
            .to_indy(IndyErrorKind::InvalidStructure, "Malformed config json")?;

        let db_path = SQLiteStorageType::_db_path(id, config.as_ref());

        if db_path.exists() {
            return Err(err_msg(
                IndyErrorKind::WalletAlreadyExists,
                format!("Wallet database file already exists: {:?}", db_path),
            ));
        }

        fs::DirBuilder::new()
            .recursive(true)
            .create(db_path.parent().unwrap())?;

        let mut conn = SqliteConnectOptions::default()
            .filename(db_path.as_path())
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .connect()
            .await?;

        let res = sqlx::query(
            r#"
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
            
            /*** Insert metadata ***/
            INSERT INTO metadata(value) VALUES (?1);

            COMMIT;
        "#,
        )
        .persistent(false)
        .bind(metadata)
        .execute(&mut conn)
        .await;

        // TODO: I am not sure force cleanup here is a good idea.
        if let Err(err) = res {
            std::fs::remove_file(db_path)?;
            Err(err)?;
        }

        Ok(())
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
    ///  * `IndyError`
    ///
    /// # Errors
    ///
    /// Any of the following `IndyError` type_ of errors can be throw by this method:
    ///
    ///  * `IndyError::NotFound` - File with the provided id not found
    ///  * `IOError("IO error during storage operation:...")` - Failed connection or SQL query
    ///
    async fn open_storage(
        &self,
        id: &str,
        config: Option<&str>,
        _credentials: Option<&str>,
    ) -> IndyResult<Box<dyn WalletStorage>> {
        let config: Option<Config> = config
            .map(serde_json::from_str)
            .map_or(Ok(None), |v| v.map(Some))
            .to_indy(IndyErrorKind::InvalidStructure, "Malformed config json")?;

        let db_path = SQLiteStorageType::_db_path(id, config.as_ref());

        if !db_path.exists() {
            return Err(err_msg(
                IndyErrorKind::WalletNotFound,
                "No wallet database exists",
            ));
        }

        Ok(Box::new(SQLiteStorage {
            pool: SqlitePoolOptions::default()
                .min_connections(1)
                .max_connections(1)
                .connect_with(
                    SqliteConnectOptions::new()
                        .filename(db_path.as_path())
                        .journal_mode(SqliteJournalMode::Wal),
                )
                .await?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use indy_utils::{assert_kind, test};
    use serde_json::json;

    use super::super::Tag;
    use super::*;

    #[async_std::test]
    async fn sqlite_storage_type_create_works() {
        _cleanup("sqlite_storage_type_create_works");

        let storage_type = SQLiteStorageType::new();

        storage_type
            .create_storage("sqlite_storage_type_create_works", None, None, &_metadata())
            .await
            .unwrap();

        _cleanup("sqlite_storage_type_create_works");
    }

    #[async_std::test]
    async fn sqlite_storage_type_create_works_for_custom_path() {
        _cleanup("sqlite_storage_type_create_works_for_custom_path");

        let config = json!({
            "path": _custom_path("sqlite_storage_type_create_works_for_custom_path")
        })
        .to_string();

        _cleanup_custom_path("sqlite_storage_type_create_works_for_custom_path");
        let storage_type = SQLiteStorageType::new();

        storage_type
            .create_storage(
                "sqlite_storage_type_create_works_for_custom_path",
                Some(&config),
                None,
                &_metadata(),
            )
            .await
            .unwrap();

        storage_type
            .delete_storage(
                "sqlite_storage_type_create_works_for_custom_path",
                Some(&config),
                None,
            )
            .await
            .unwrap();

        _cleanup_custom_path("sqlite_storage_type_create_works_for_custom_path");
        _cleanup("sqlite_storage_type_create_works_for_custom_path");
    }

    fn _cleanup_custom_path(custom_path: &str) {
        let my_path = _custom_path(custom_path);
        let path = Path::new(&my_path);
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    #[async_std::test]
    async fn sqlite_storage_type_create_works_for_twice() {
        _cleanup("sqlite_storage_type_create_works_for_twice");

        let storage_type = SQLiteStorageType::new();
        storage_type
            .create_storage(
                "sqlite_storage_type_create_works_for_twice",
                None,
                None,
                &_metadata(),
            )
            .await
            .unwrap();

        let res = storage_type
            .create_storage(
                "sqlite_storage_type_create_works_for_twice",
                None,
                None,
                &_metadata(),
            )
            .await;

        assert_kind!(IndyErrorKind::WalletAlreadyExists, res);

        storage_type
            .delete_storage("sqlite_storage_type_create_works_for_twice", None, None)
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn sqlite_storage_get_storage_metadata_works() {
        _cleanup("sqlite_storage_get_storage_metadata_works");

        {
            let storage = _storage("sqlite_storage_get_storage_metadata_works").await;
            let metadata = storage.get_storage_metadata().await.unwrap();

            assert_eq!(metadata, _metadata());
        }

        _cleanup("sqlite_storage_get_storage_metadata_works");
    }

    #[async_std::test]
    async fn sqlite_storage_type_delete_works() {
        _cleanup("sqlite_storage_type_delete_works");

        let storage_type = SQLiteStorageType::new();
        storage_type
            .create_storage("sqlite_storage_type_delete_works", None, None, &_metadata())
            .await
            .unwrap();

        storage_type
            .delete_storage("sqlite_storage_type_delete_works", None, None)
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn sqlite_storage_type_delete_works_for_non_existing() {
        _cleanup("sqlite_storage_type_delete_works_for_non_existing");

        let storage_type = SQLiteStorageType::new();

        storage_type
            .create_storage(
                "sqlite_storage_type_delete_works_for_non_existing",
                None,
                None,
                &_metadata(),
            )
            .await
            .unwrap();

        let res = storage_type.delete_storage("unknown", None, None).await;
        assert_kind!(IndyErrorKind::WalletNotFound, res);

        storage_type
            .delete_storage(
                "sqlite_storage_type_delete_works_for_non_existing",
                None,
                None,
            )
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn sqlite_storage_type_open_works() {
        _cleanup("sqlite_storage_type_open_works");
        _storage("sqlite_storage_type_open_works").await;
        _cleanup("sqlite_storage_type_open_works");
    }

    #[async_std::test]
    async fn sqlite_storage_type_open_works_for_custom() {
        _cleanup("sqlite_storage_type_open_works_for_custom");

        let my_path = _custom_path("sqlite_storage_type_open_works_for_custom");
        let path = Path::new(&my_path);

        if path.exists() && path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        }

        _storage_custom("sqlite_storage_type_open_works_for_custom").await;

        fs::remove_dir_all(path).unwrap();
    }

    #[async_std::test]
    async fn sqlite_storage_type_open_works_for_not_created() {
        _cleanup("sqlite_storage_type_open_works_for_not_created");

        let storage_type = SQLiteStorageType::new();

        let res = storage_type
            .open_storage("unknown", Some("{}"), Some("{}"))
            .await;

        assert_kind!(IndyErrorKind::WalletNotFound, res);
    }

    #[async_std::test]
    async fn sqlite_storage_add_works_for_is_802() {
        _cleanup("sqlite_storage_add_works_for_is_802");

        {
            let storage = _storage("sqlite_storage_add_works_for_is_802").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);

            let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
        }

        _cleanup("sqlite_storage_add_works_for_is_802");
    }

    #[async_std::test]
    async fn sqlite_storage_set_get_works() {
        _cleanup("sqlite_storage_set_get_works");

        {
            let storage = _storage("sqlite_storage_set_get_works").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());
            assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));
        }

        _cleanup("sqlite_storage_set_get_works");
    }

    #[async_std::test]
    async fn sqlite_storage_set_get_works_for_custom() {
        _cleanup("sqlite_storage_set_get_works_for_custom");

        let path = _custom_path("sqlite_storage_set_get_works_for_custom");
        let path = Path::new(&path);

        {
            let storage = _storage_custom("sqlite_storage_set_get_works_for_custom").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.id, _id1());
            assert_eq!(record.value.unwrap(), _value1());
            assert_eq!(record.type_, None);
            assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));
        }

        fs::remove_dir_all(path).unwrap();
    }

    #[async_std::test]
    async fn sqlite_storage_set_get_works_for_twice() {
        _cleanup("sqlite_storage_set_get_works_for_twice");

        {
            let storage = _storage("sqlite_storage_set_get_works_for_twice").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add(&_type1(), &_id1(), &_value2(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
        }

        _cleanup("sqlite_storage_set_get_works_for_twice");
    }

    #[async_std::test]
    async fn sqlite_storage_set_get_works_for_reopen() {
        _cleanup("sqlite_storage_set_get_works_for_reopen");

        _storage("sqlite_storage_set_get_works_for_reopen")
            .await
            .add(&_type1(), &_id1(), &_value1(), &_tags())
            .await
            .unwrap();

        let record = SQLiteStorageType::new()
            .open_storage(
                "sqlite_storage_set_get_works_for_reopen",
                Some("{}"),
                Some("{}"),
            )
            .await
            .unwrap()
            .get(
                &_type1(),
                &_id1(),
                r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
            )
            .await
            .unwrap();

        assert_eq!(record.value.unwrap(), _value1());
        assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

        _cleanup("sqlite_storage_set_get_works_for_reopen");
    }

    #[async_std::test]
    async fn sqlite_storage_get_works_for_wrong_key() {
        _cleanup("sqlite_storage_get_works_for_wrong_key");

        {
            let storage = _storage("sqlite_storage_get_works_for_wrong_key").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage
                .get(
                    &_type1(),
                    &_id2(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await;

            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_get_works_for_wrong_key");
    }

    #[async_std::test]
    async fn sqlite_storage_delete_works() {
        _cleanup("sqlite_storage_delete_works");

        {
            let storage = _storage("sqlite_storage_delete_works").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());
            assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

            storage.delete(&_type1(), &_id1()).await.unwrap();

            let res = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await;

            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_delete_works");
    }

    #[async_std::test]
    async fn sqlite_storage_delete_works_for_non_existing() {
        _cleanup("sqlite_storage_delete_works_for_non_existing");

        {
            let storage = _storage("sqlite_storage_delete_works_for_non_existing").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.delete(&_type1(), &_id2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_delete_works_for_non_existing");
    }

    #[async_std::test]
    async fn sqlite_storage_delete_returns_error_item_not_found_if_no_such_type() {
        _cleanup("sqlite_storage_delete_returns_error_item_not_found_if_no_such_type");

        {
            let storage =
                _storage("sqlite_storage_delete_returns_error_item_not_found_if_no_such_type")
                    .await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.delete(&_type2(), &_id2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_delete_returns_error_item_not_found_if_no_such_type");
    }

    #[async_std::test]
    async fn sqlite_storage_get_all_works() {
        _cleanup("sqlite_storage_get_all_works");

        {
            let storage = _storage("sqlite_storage_get_all_works").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            storage
                .add(&_type2(), &_id2(), &_value2(), &_tags())
                .await
                .unwrap();

            let mut storage_iterator = storage.get_all().await.unwrap();

            let record = storage_iterator.next().await.unwrap().unwrap();
            assert_eq!(record.type_.unwrap(), _type1());
            assert_eq!(record.value.unwrap(), _value1());
            assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

            let record = storage_iterator.next().await.unwrap().unwrap();
            assert_eq!(record.type_.unwrap(), _type2());
            assert_eq!(record.value.unwrap(), _value2());
            assert_eq!(_sort(record.tags.unwrap()), _sort(_tags()));

            let record = storage_iterator.next().await.unwrap();
            assert!(record.is_none());
        }

        _cleanup("sqlite_storage_get_all_works");
    }

    #[async_std::test]
    async fn sqlite_storage_get_all_works_for_empty() {
        _cleanup("sqlite_storage_get_all_works_for_empty");

        {
            let storage = _storage("sqlite_storage_get_all_works_for_empty").await;
            let mut storage_iterator = storage.get_all().await.unwrap();

            let record = storage_iterator.next().await.unwrap();
            assert!(record.is_none());
        }

        _cleanup("sqlite_storage_get_all_works_for_empty");
    }

    #[async_std::test]
    async fn sqlite_storage_update_works() {
        _cleanup("sqlite_storage_update_works");

        {
            let storage = _storage("sqlite_storage_update_works").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());

            storage
                .update(&_type1(), &_id1(), &_value2())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value2());
        }

        _cleanup("sqlite_storage_update_works");
    }

    #[async_std::test]
    async fn sqlite_storage_update_works_for_non_existing_id() {
        _cleanup("sqlite_storage_update_works_for_non_existing_id");

        {
            let storage = _storage("sqlite_storage_update_works_for_non_existing_id").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());

            let res = storage.update(&_type1(), &_id2(), &_value2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_update_works_for_non_existing_id");
    }

    #[async_std::test]
    async fn sqlite_storage_update_works_for_non_existing_type() {
        _cleanup("sqlite_storage_update_works_for_non_existing_type");

        {
            let storage = _storage("sqlite_storage_update_works_for_non_existing_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());

            let res = storage.update(&_type2(), &_id1(), &_value2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_update_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn sqlite_storage_add_tags_works() {
        _cleanup("sqlite_storage_add_tags_works");

        {
            let storage = _storage("sqlite_storage_add_tags_works").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            storage
                .add_tags(&_type1(), &_id1(), &_new_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());

            let expected_tags = {
                let mut tags = _tags();
                tags.extend(_new_tags());
                _sort(tags)
            };

            assert_eq!(_sort(record.tags.unwrap()), expected_tags);
        }

        _cleanup("sqlite_storage_add_tags_works");
    }

    #[async_std::test]
    async fn sqlite_storage_add_tags_works_for_non_existing_id() {
        _cleanup("sqlite_storage_add_tags_works_for_non_existing_id");

        {
            let storage = _storage("sqlite_storage_add_tags_works_for_non_existing_id").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_add_tags_works_for_non_existing_id");
    }

    #[async_std::test]
    async fn sqlite_storage_add_tags_works_for_non_existing_type() {
        _cleanup("sqlite_storage_add_tags_works_for_non_existing_type");

        {
            let storage = _storage("sqlite_storage_add_tags_works_for_non_existing_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add_tags(&_type2(), &_id1(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_add_tags_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn sqlite_storage_add_tags_works_for_already_existing() {
        _cleanup("sqlite_storage_add_tags_works_for_already_existing");

        {
            let storage = _storage("sqlite_storage_add_tags_works_for_already_existing").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let tags_with_existing = {
                let mut tags = _tags();
                tags.extend(_new_tags());
                tags
            };

            storage
                .add_tags(&_type1(), &_id1(), &tags_with_existing)
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());

            let expected_tags = {
                let mut tags = _tags();
                tags.extend(_new_tags());
                _sort(tags)
            };

            assert_eq!(_sort(record.tags.unwrap()), expected_tags);
        }

        _cleanup("sqlite_storage_add_tags_works_for_already_existing");
    }

    #[async_std::test]
    async fn sqlite_storage_update_tags_works() {
        _cleanup("sqlite_storage_update_tags_works");

        {
            let storage = _storage("sqlite_storage_update_tags_works").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            storage
                .update_tags(&_type1(), &_id1(), &_new_tags())
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());
            assert_eq!(_sort(record.tags.unwrap()), _sort(_new_tags()));
        }

        _cleanup("sqlite_storage_update_tags_works");
    }

    #[async_std::test]
    async fn sqlite_storage_update_tags_works_for_non_existing_id() {
        _cleanup("sqlite_storage_update_tags_works_for_non_existing_id");

        {
            let storage = _storage("sqlite_storage_update_tags_works_for_non_existing_id").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.update_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_update_tags_works_for_non_existing_id");
    }

    #[async_std::test]
    async fn sqlite_storage_update_tags_works_for_non_existing_type() {
        _cleanup("sqlite_storage_update_tags_works_for_non_existing_type");

        {
            let storage = _storage("sqlite_storage_update_tags_works_for_non_existing_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.update_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_update_tags_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn sqlite_storage_update_tags_works_for_already_existing() {
        _cleanup("sqlite_storage_update_tags_works_for_already_existing");
        {
            let storage = _storage("sqlite_storage_update_tags_works_for_already_existing").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let tags_with_existing = {
                let mut tags = _tags();
                tags.extend(_new_tags());
                tags
            };

            storage
                .update_tags(&_type1(), &_id1(), &tags_with_existing)
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.value.unwrap(), _value1());

            let expected_tags = {
                let mut tags = _tags();
                tags.extend(_new_tags());
                _sort(tags)
            };

            assert_eq!(_sort(record.tags.unwrap()), expected_tags);
        }
        _cleanup("sqlite_storage_update_tags_works_for_already_existing");
    }

    #[async_std::test]
    async fn sqlite_storage_delete_tags_works() {
        _cleanup("sqlite_storage_delete_tags_works");

        {
            let storage = _storage("sqlite_storage_delete_tags_works").await;

            let tag_name1 = vec![0, 0, 0];
            let tag_name2 = vec![1, 1, 1];
            let tag_name3 = vec![2, 2, 2];
            let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
            let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
            let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
            let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

            storage
                .add(&_type1(), &_id1(), &_value1(), &tags)
                .await
                .unwrap();

            let tag_names = vec![
                TagName::OfEncrypted(tag_name1.clone()),
                TagName::OfPlain(tag_name2.clone()),
            ];

            storage
                .delete_tags(&_type1(), &_id1(), &tag_names)
                .await
                .unwrap();

            let record = storage
                .get(
                    &_type1(),
                    &_id1(),
                    r##"{"retrieveType": false, "retrieveValue": true, "retrieveTags": true}"##,
                )
                .await
                .unwrap();

            assert_eq!(record.tags.unwrap(), vec![tag3]);
        }

        _cleanup("sqlite_storage_delete_tags_works");
    }

    #[async_std::test]
    async fn sqlite_storage_delete_tags_works_for_non_existing_type() {
        _cleanup("sqlite_storage_delete_tags_works_for_non_existing_type");

        {
            let storage = _storage("sqlite_storage_delete_tags_works_for_non_existing_type").await;

            let tag_name1 = vec![0, 0, 0];
            let tag_name2 = vec![1, 1, 1];
            let tag_name3 = vec![2, 2, 2];
            let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
            let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
            let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
            let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

            storage
                .add(&_type1(), &_id1(), &_value1(), &tags)
                .await
                .unwrap();

            let tag_names = vec![
                TagName::OfEncrypted(tag_name1.clone()),
                TagName::OfPlain(tag_name2.clone()),
            ];

            let res = storage.delete_tags(&_type2(), &_id1(), &tag_names).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_delete_tags_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn sqlite_storage_delete_tags_works_for_non_existing_id() {
        _cleanup("sqlite_storage_delete_tags_works_for_non_existing_id");

        {
            let storage = _storage("sqlite_storage_delete_tags_works_for_non_existing_id").await;

            let tag_name1 = vec![0, 0, 0];
            let tag_name2 = vec![1, 1, 1];
            let tag_name3 = vec![2, 2, 2];
            let tag1 = Tag::Encrypted(tag_name1.clone(), vec![0, 0, 0]);
            let tag2 = Tag::PlainText(tag_name2.clone(), "tag_value_2".to_string());
            let tag3 = Tag::Encrypted(tag_name3.clone(), vec![2, 2, 2]);
            let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

            storage
                .add(&_type1(), &_id1(), &_value1(), &tags)
                .await
                .unwrap();

            let tag_names = vec![
                TagName::OfEncrypted(tag_name1.clone()),
                TagName::OfPlain(tag_name2.clone()),
            ];

            let res = storage.delete_tags(&_type1(), &_id2(), &tag_names).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("sqlite_storage_delete_tags_works_for_non_existing_id");
    }

    fn _cleanup(name: &str) {
        test::cleanup_storage(name)
    }

    async fn _storage(name: &str) -> Box<dyn WalletStorage> {
        let storage_type = SQLiteStorageType::new();

        storage_type
            .create_storage(name, None, None, &_metadata())
            .await
            .unwrap();

        storage_type.open_storage(name, None, None).await.unwrap()
    }

    async fn _storage_custom(name: &str) -> Box<dyn WalletStorage> {
        let storage_type = SQLiteStorageType::new();

        let config = json!({ "path": _custom_path(name) }).to_string();

        storage_type
            .create_storage(name, Some(&config), None, &_metadata())
            .await
            .unwrap();

        storage_type
            .open_storage(name, Some(&config), None)
            .await
            .unwrap()
    }

    fn _metadata() -> Vec<u8> {
        return vec![
            1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5,
            6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2,
            3, 4, 5, 6, 7, 8,
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
        EncryptedValue {
            data: vec![6 + i, 7 + i, 8 + i],
            key: vec![9 + i, 10 + i, 11 + i],
        }
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
            Tag::PlainText(vec![1, 1, 1], String::from("tag_value_3")),
        ]
    }

    fn _sort(mut v: Vec<Tag>) -> Vec<Tag> {
        v.sort();
        v
    }

    fn _custom_path(name: &str) -> String {
        let mut path = environment::tmp_path();
        path.push(name);
        path.to_str().unwrap().to_owned()
    }
}
