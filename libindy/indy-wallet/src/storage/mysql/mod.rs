use std::{
    collections::{HashMap, VecDeque},
    iter::Iterator,
};

use async_trait::async_trait;
use futures::lock::Mutex;

use indy_api_types::errors::prelude::*;
use indy_utils::crypto::base64;

use serde::Deserialize;

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlRow},
    Done, MySqlPool, Row,
};

use crate::{
    language,
    storage::{StorageIterator, StorageRecord, Tag, TagName, WalletStorage, WalletStorageType},
    wallet::EncryptedValue,
    RecordOptions, SearchOptions,
};
use query::{wql_to_sql, wql_to_sql_count};

mod query;

struct MySQLStorageIterator {
    records: Option<VecDeque<IndyResult<StorageRecord>>>,
    total_count: Option<usize>,
}

impl MySQLStorageIterator {
    fn new(
        records: Option<VecDeque<IndyResult<StorageRecord>>>,
        total_count: Option<usize>,
    ) -> IndyResult<MySQLStorageIterator> {
        Ok(MySQLStorageIterator {
            records,
            total_count,
        })
    }
}

#[async_trait]
impl StorageIterator for MySQLStorageIterator {
    async fn next(&mut self) -> IndyResult<Option<StorageRecord>> {
        // TODO: Optimize!!!
        if let Some(ref mut records) = self.records {
            if let Some(record) = records.pop_front() {
                return Ok(Some(record?));
            } else {
                Ok(None)
            }
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
    pub read_host: String,
    pub write_host: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Deserialize)]
pub struct Credentials {
    pub user: String,
    pub pass: String,
}

#[derive(Debug)]
struct MySqlStorage {
    wallet_id: i64,
    read_pool: MySqlPool,
    write_pool: MySqlPool,
}

pub struct MySqlStorageType {
    connections: Mutex<HashMap<String, MySqlPool>>,
}

impl MySqlStorageType {
    pub fn new() -> MySqlStorageType {
        MySqlStorageType {
            connections: Mutex::new(HashMap::new()),
        }
    }

    pub async fn _connect(
        &self,
        read_only: bool,
        config: Option<&str>,
        credentials: Option<&str>,
    ) -> IndyResult<MySqlPool> {
        let config = config
            .map(serde_json::from_str::<Config>)
            .transpose()
            .to_indy(IndyErrorKind::InvalidStructure, "Malformed config json")?
            .ok_or(err_msg(
                IndyErrorKind::InvalidStructure,
                "Absent config json",
            ))?;

        let credentials = credentials
            .map(serde_json::from_str::<Credentials>)
            .transpose()
            .to_indy(
                IndyErrorKind::InvalidStructure,
                "Malformed credentials json",
            )?
            .ok_or(err_msg(
                IndyErrorKind::InvalidStructure,
                "Absent credentials json",
            ))?;

        let host_addr = if read_only {
            &config.read_host
        } else {
            &config.write_host
        };

        let connection_string = format!(
            "{}:{}@{}:{}/{}",
            credentials.user, credentials.pass, host_addr, config.port, config.db_name
        );

        let mut connref = self.connections.lock().await;

        if let Some(connection) = connref.get(&connection_string) {
            return Ok(connection.clone());
        }

        let connection = MySqlPoolOptions::default()
            .max_connections(4000)
            .test_before_acquire(false)
            .connect_with(
                MySqlConnectOptions::new()
                    .host(host_addr)
                    .database(&config.db_name)
                    .username(&credentials.user)
                    .password(&credentials.pass),
            )
            .await?;

        connref.insert(connection_string, connection.clone());
        Ok(connection)
    }
}

#[async_trait]
impl WalletStorage for MySqlStorage {
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

        let mut conn = self.read_pool.acquire().await?;

        let (value, tags): (Option<Vec<u8>>, Option<serde_json::Value>) = sqlx::query_as(&format!(
            r#"
            SELECT {}, {}
            FROM items
            WHERE
                wallet_id = ?
                    AND type = ?
                    AND name = ?
            "#,
            if options.retrieve_value {
                "value"
            } else {
                "NULL"
            },
            if options.retrieve_tags {
                "tags"
            } else {
                "NULL"
            },
        ))
        .bind(self.wallet_id)
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .fetch_one(&mut conn)
        .await?;

        let value = if let Some(value) = value {
            Some(EncryptedValue::from_bytes(&value)?)
        } else {
            None
        };

        let type_ = if options.retrieve_type {
            Some(type_.to_vec())
        } else {
            None
        };

        let tags = if let Some(tags) = tags {
            Some(_tags_from_json(tags)?)
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
        let mut tx = self.write_pool.begin().await?;

        sqlx::query(
            r#"
            INSERT INTO items (type, name, value, tags, wallet_id)
            VALUE (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .bind(&value.to_bytes())
        .bind(&_tags_to_json(tags)?)
        .bind(&self.wallet_id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn update(&self, type_: &[u8], id: &[u8], value: &EncryptedValue) -> IndyResult<()> {
        let mut tx = self.write_pool.begin().await?;

        let row_updated = sqlx::query(
            r#"
            UPDATE items
            SET value = ?
            WHERE type = ?
                AND name = ?
                AND wallet_id = ?
            "#,
        )
        .bind(&value.to_bytes())
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .bind(&self.wallet_id)
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
        if tags.is_empty() {
            // FIXME: Think about checking item exists
            return Ok(());
        }

        let tag_paths = _tags_to_plain(&tags)
            .into_iter()
            .map(|(tag, val)| format!(r#"'$."{}"', "{}""#, tag, val))
            .collect::<Vec<_>>()
            .join(",");

        let mut tx = self.write_pool.begin().await?;

        let row_updated = sqlx::query(&format!(
            r#"
            UPDATE items
                SET tags = JSON_SET(tags, {})
            WHERE type = ?
                AND name = ?
                AND wallet_id = ?
            "#,
            tag_paths
        ))
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .bind(&self.wallet_id)
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

    async fn update_tags(&self, type_: &[u8], id: &[u8], tags: &[Tag]) -> IndyResult<()> {
        let mut tx = self.write_pool.begin().await?;

        let row_updated = sqlx::query(
            r#"
            UPDATE items
            SET tags = ?
            WHERE type = ?
                AND name = ?
                AND wallet_id = ?
            "#,
        )
        .bind(&_tags_to_json(tags)?)
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .bind(&self.wallet_id)
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

    async fn delete_tags(&self, type_: &[u8], id: &[u8], tag_names: &[TagName]) -> IndyResult<()> {
        if tag_names.is_empty() {
            // FIXME: Think about checking item exists
            return Ok(());
        }

        let mut tx = self.write_pool.begin().await?;

        let tag_name_paths = _tag_names_to_plain(&tag_names)
            .into_iter()
            .map(|tag_name| format!(r#"'$."{}"'"#, tag_name))
            .collect::<Vec<_>>()
            .join(",");

        let row_updated = sqlx::query(&format!(
            r#"
            UPDATE items
            SET tags = JSON_REMOVE(tags, {})
            WHERE type = ?
                AND name = ?
                AND wallet_id = ?
            "#,
            tag_name_paths
        ))
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .bind(&self.wallet_id)
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
        let mut tx = self.write_pool.begin().await?;

        let rows_affected = sqlx::query(
            r#"
            DELETE FROM items
            WHERE type = ?
                AND name = ?
                AND wallet_id = ?"#,
        )
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .bind(&self.wallet_id)
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
        let mut conn = self.read_pool.acquire().await?;

        let (metadata,): (String,) = sqlx::query_as::<_, (String,)>(
            r#"
            SELECT metadata
            FROM wallets
            WHERE id = ?
            "#,
        )
        .bind(&self.wallet_id)
        .fetch_one(&mut conn)
        .await?;

        base64::decode(&metadata)
    }

    async fn set_storage_metadata(&self, metadata: &[u8]) -> IndyResult<()> {
        let mut tx = self.write_pool.begin().await?;

        sqlx::query(
            r#"
            UPDATE wallets
            SET metadata = ?
            WHERE id = ?
            "#,
        )
        .bind(base64::encode(metadata))
        .bind(&self.wallet_id)
        .execute(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn get_all(&self) -> IndyResult<Box<dyn StorageIterator>> {
        let records: VecDeque<_> = sqlx::query(
            r#"
            SELECT type, name, value, tags
            FROM items
            WHERE wallet_id = ?
            ORDER BY id
            "#,
        )
        .bind(self.wallet_id)
        .map(|r: MySqlRow| -> IndyResult<StorageRecord> {
            let type_: String = r.get(0);
            let id: String = r.get(1);
            let value: Vec<u8> = r.get(2);
            let tags: serde_json::Value = r.get(3);

            let res = StorageRecord::new(
                base64::decode(&id)?,
                Some(EncryptedValue::from_bytes(&value)?),
                Some(base64::decode(&type_)?),
                Some(_tags_from_json(tags)?),
            );

            Ok(res)
        })
        .fetch_all(&self.read_pool)
        .await?
        .into_iter()
        .collect();

        let total_len = records.len();

        // FIXME: Fetch total count
        Ok(Box::new(MySQLStorageIterator::new(
            Some(records),
            Some(total_len),
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

        let mut conn = self.read_pool.acquire().await?;

        let total_count = if options.retrieve_total_count {
            let (query, args) = wql_to_sql_count(self.wallet_id, type_, query)?;
            let mut query = sqlx::query_as::<sqlx::MySql, (i64,)>(&query);

            for arg in args.iter() {
                query = if arg.is_i64() {
                    query.bind(arg.as_i64().unwrap())
                } else if arg.is_string() {
                    query.bind(arg.as_str().unwrap())
                } else {
                    return Err(err_msg(
                        IndyErrorKind::InvalidState,
                        "Unexpected sql parameter type.",
                    ));
                }
            }

            let (total_count,) = query.fetch_one(&mut conn).await?;
            Some(total_count as usize)
        } else {
            None
        };

        let records = if options.retrieve_records {
            let (query, args) = wql_to_sql(self.wallet_id, type_, query, &options)?;

            let mut query = sqlx::query::<sqlx::MySql>(&query);

            for arg in args.iter() {
                query = if arg.is_i64() {
                    query.bind(arg.as_i64().unwrap())
                } else if arg.is_string() {
                    query.bind(arg.as_str().unwrap())
                } else {
                    return Err(err_msg(
                        IndyErrorKind::InvalidState,
                        "Unexpected sql parameter type.",
                    ));
                }
            }

            let records: VecDeque<_> = query
                .map(|r: MySqlRow| -> IndyResult<StorageRecord> {
                    let type_ = if options.retrieve_type {
                        let type_: String = r.get(0);
                        Some(base64::decode(&type_)?)
                    } else {
                        None
                    };

                    let id = {
                        let id: String = r.get(1);
                        base64::decode(&id)?
                    };

                    let value = if options.retrieve_value {
                        let value: Vec<u8> = r.get(2);
                        Some(EncryptedValue::from_bytes(&value)?)
                    } else {
                        None
                    };

                    let tags = if options.retrieve_tags {
                        let tags: serde_json::Value = r.get(3);
                        Some(_tags_from_json(tags)?)
                    } else {
                        None
                    };

                    let res = StorageRecord::new(id, value, type_, tags);

                    Ok(res)
                })
                .fetch_all(&self.read_pool)
                .await?
                .into_iter()
                .collect();

            Some(records)
        } else {
            None
        };

        Ok(Box::new(MySQLStorageIterator::new(records, total_count)?))
    }

    fn close(&mut self) -> IndyResult<()> {
        Ok(())
    }
}

#[async_trait]
impl WalletStorageType for MySqlStorageType {
    ///
    /// Deletes the MySql database file with the provided id from the path specified in the
    /// config file.
    ///
    /// # Arguments
    ///
    ///  * `id` - id of the MySql DB file
    ///  * `storage_config` - config containing the location of MySql DB files
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
        credentials: Option<&str>,
    ) -> IndyResult<()> {
        let mut tx = self
            ._connect(false, config, credentials)
            .await?
            .begin()
            .await?;

         let res = sqlx::query(
            r#"
            DELETE FROM wallets
            WHERE name = ?
            "#,
        )
        .bind(id)
        .execute(&mut tx)
        .await;

        let rows_affected = res?.rows_affected();

        match rows_affected {
            1 => {
                tx.commit().await?;
                Ok(())
            }
            0 => {
                Err(err_msg(
                    IndyErrorKind::WalletNotFound,
                    "Item to delete not found",
                ))
            }
            _ => {
                Err(err_msg(
                    IndyErrorKind::InvalidState,
                    "More than one row deleted. Seems wallet structure is inconsistent",
                ))
            }
        }
    }

    ///
    /// Creates the MySql DB file with the provided name in the path specified in the config file,
    /// and initializes the encryption keys needed for encryption and decryption of data.
    ///
    /// # Arguments
    ///
    ///  * `id` - name of the MySql DB file
    ///  * `config` - config containing the location of MySql DB files
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
        credentials: Option<&str>,
        metadata: &[u8],
    ) -> IndyResult<()> {
        let mut tx = self
            ._connect(false, config, credentials)
            .await?
            .begin()
            .await?;

        let res = sqlx::query(
            r#"
            INSERT INTO wallets (name, metadata)
            VALUES (?, ?)
            "#,
        )
        .bind(id)
        .bind(base64::encode(metadata))
        .execute(&mut tx)
        .await;

        match res {
            Err(sqlx::Error::Database(e)) if e.code().is_some() && e.code().unwrap() == "23000" => {
                return Err(err_msg(
                    IndyErrorKind::WalletAlreadyExists,
                    "Wallet already exists",
                ))
            }
            e => e?,
        };

        // FIXME: return wallet already exists on 1062 error code from MySQL

        tx.commit().await?;
        Ok(())
    }

    ///
    /// Establishes a connection to the MySql DB with the provided id located in the path
    /// specified in the config. In case of a successful onection returns a Storage object
    /// embedding the connection and the encryption keys that will be used for encryption and
    /// decryption operations.
    ///
    ///
    /// # Arguments
    ///
    ///  * `id` - id of the MySql DB file
    ///  * `config` - config containing the location of MySql DB files
    ///  * `credentials` - DB credentials
    ///
    /// # Returns
    ///
    /// Result that can be either:
    ///
    ///  * `(Box<Storage>, Vec<u8>)` - Tuple of `MySqlStorage` and `encryption keys`
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
        credentials: Option<&str>,
    ) -> IndyResult<Box<dyn WalletStorage>> {
        let read_pool = self._connect(true, config, credentials).await?;
        let write_pool = self._connect(false, config, credentials).await?;

        let res = sqlx::query_as::<_, (i64,)>(
            r#"
            SELECT id FROM wallets
            WHERE name = ?
            "#,
        )
        .bind(id)
        .fetch_one(&read_pool)
        .await;

        let (wallet_id,) = match res {
            Err(sqlx::Error::RowNotFound) => {
                return Err(err_msg(IndyErrorKind::WalletNotFound, "Wallet not found"));
            }
            e => e?,
        };

        Ok(Box::new(MySqlStorage {
            read_pool,
            write_pool,
            wallet_id,
        }))
    }
}

#[cfg(test)]
mod tests {
    use indy_utils::{assert_kind, environment};

    use super::super::Tag;
    use super::*;

    // docker run --name indy-mysql -e MYSQL_ROOT_PASSWORD=pass@word1 -p 3306:3306 -d mysql:latest

    #[async_std::test]
    async fn mysql_storage_sync_send() {
        use futures::{channel::oneshot, executor::ThreadPool, future::join_all};
        use std::{sync::Arc, time::SystemTime};

        let count = 1000;
        let executor = ThreadPool::new().expect("Failed to new ThreadPool");
        let storage_type = Arc::new(Box::new(MySqlStorageType::new()));

        let waiters: Vec<_> = (0..count)
            .into_iter()
            .map(|id| {
                let st = storage_type.clone();
                let (tx, rx) = oneshot::channel::<IndyResult<()>>();

                let future = async move {
                    let res = st
                        .delete_storage(
                            &format!("mysql_storage_sync_send_{}", id),
                            _config(),
                            _credentials(),
                        )
                        .await;

                    tx.send(res).unwrap();
                };

                executor.spawn_ok(future);
                rx
            })
            .collect();

        let res = join_all(waiters).await;
        println!("------------> 1 {:?}", SystemTime::now());

        let waiters: Vec<_> = (0..count)
            .into_iter()
            .map(|id| {
                let st = storage_type.clone();
                let (tx, rx) = oneshot::channel::<IndyResult<()>>();

                let future = async move {
                    let res = st.create_storage(
                        &format!("mysql_storage_sync_send_{}", id),
                        _config(),
                        _credentials(),
                        &_metadata(),
                    )
                    .await;

                    tx.send(res).unwrap();
                };

                executor.spawn_ok(future);
                rx
            })
            .collect();

            let res = join_all(waiters).await;

        println!("------------> 3 {:?}", SystemTime::now());

        let waiters: Vec<_> = (0..count)
            .into_iter()
            .map(|id| {
                let st = storage_type.clone();
                let (tx, rx) = oneshot::channel::<IndyResult<()>>();

                let future = async move {
                    let res = st.delete_storage(
                        &format!("mysql_storage_sync_send_{}", id),
                        _config(),
                        _credentials(),
                    )
                    .await;

                    tx.send(res).unwrap();
                };

                executor.spawn_ok(future);
                rx
            })
            .collect();

            let res = join_all(waiters).await;

        println!("------------> 5 {:?}", SystemTime::now());
    }

    #[async_std::test]
    async fn mysql_storage_type_create_works() {
        _cleanup("mysql_storage_type_create_works").await;

        let storage_type = MySqlStorageType::new();

        storage_type
            .create_storage(
                "mysql_storage_type_create_works",
                _config(),
                _credentials(),
                &_metadata(),
            )
            .await
            .unwrap();

        _cleanup("mysql_storage_type_create_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_type_create_works_for_twice() {
        _cleanup("mysql_storage_type_create_works_for_twice").await;

        let storage_type = MySqlStorageType::new();
        storage_type
            .create_storage(
                "mysql_storage_type_create_works_for_twice",
                _config(),
                _credentials(),
                &_metadata(),
            )
            .await
            .unwrap();

        let res = storage_type
            .create_storage(
                "mysql_storage_type_create_works_for_twice",
                _config(),
                _credentials(),
                &_metadata(),
            )
            .await;

        assert_kind!(IndyErrorKind::WalletAlreadyExists, res);

        storage_type
            .delete_storage(
                "mysql_storage_type_create_works_for_twice",
                _config(),
                _credentials(),
            )
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn mysql_storage_get_storage_metadata_works() {
        _cleanup("mysql_storage_get_storage_metadata_works").await;

        {
            let storage = _storage("mysql_storage_get_storage_metadata_works").await;
            let metadata = storage.get_storage_metadata().await.unwrap();

            assert_eq!(metadata, _metadata());
        }

        _cleanup("mysql_storage_get_storage_metadata_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_type_delete_works() {
        _cleanup("mysql_storage_type_delete_works").await;

        let storage_type = MySqlStorageType::new();
        storage_type
            .create_storage(
                "mysql_storage_type_delete_works",
                _config(),
                _credentials(),
                &_metadata(),
            )
            .await
            .unwrap();

        storage_type
            .delete_storage("mysql_storage_type_delete_works", _config(), _credentials())
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn mysql_storage_type_delete_works_for_non_existing() {
        _cleanup("mysql_storage_type_delete_works_for_non_existing").await;

        let storage_type = MySqlStorageType::new();

        storage_type
            .create_storage(
                "mysql_storage_type_delete_works_for_non_existing",
                _config(),
                _credentials(),
                &_metadata(),
            )
            .await
            .unwrap();

        let res = storage_type
            .delete_storage("unknown", _config(), _credentials())
            .await;
        assert_kind!(IndyErrorKind::WalletNotFound, res);

        storage_type
            .delete_storage(
                "mysql_storage_type_delete_works_for_non_existing",
                _config(),
                _credentials(),
            )
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn mysql_storage_type_open_works() {
        _cleanup("mysql_storage_type_open_works").await;
        _storage("mysql_storage_type_open_works").await;
        _cleanup("mysql_storage_type_open_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_type_open_works_for_not_created() {
        _cleanup("mysql_storage_type_open_works_for_not_created").await;

        let storage_type = MySqlStorageType::new();

        let res = storage_type
            .open_storage("unknown", _config(), _credentials())
            .await;

        assert_kind!(IndyErrorKind::WalletNotFound, res);
    }

    #[async_std::test]
    async fn mysql_storage_add_works_for_is_802() {
        _cleanup("mysql_storage_add_works_for_is_802").await;

        {
            let storage = _storage("mysql_storage_add_works_for_is_802").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);

            let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
        }

        _cleanup("mysql_storage_add_works_for_is_802").await;
    }

    #[async_std::test]
    async fn mysql_storage_set_get_works() {
        _cleanup("mysql_storage_set_get_works").await;

        {
            let storage = _storage("mysql_storage_set_get_works").await;

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

        _cleanup("mysql_storage_set_get_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_set_get_works_for_twice() {
        _cleanup("mysql_storage_set_get_works_for_twice").await;

        {
            let storage = _storage("mysql_storage_set_get_works_for_twice").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add(&_type1(), &_id1(), &_value2(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
        }

        _cleanup("mysql_storage_set_get_works_for_twice").await;
    }

    #[async_std::test]
    async fn mysql_storage_set_get_works_for_reopen() {
        _cleanup("mysql_storage_set_get_works_for_reopen").await;

        _storage("mysql_storage_set_get_works_for_reopen")
            .await
            .add(&_type1(), &_id1(), &_value1(), &_tags())
            .await
            .unwrap();

        let record = MySqlStorageType::new()
            .open_storage(
                "mysql_storage_set_get_works_for_reopen",
                _config(),
                _credentials(),
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

        _cleanup("mysql_storage_set_get_works_for_reopen").await;
    }

    #[async_std::test]
    async fn mysql_storage_get_works_for_wrong_key() {
        _cleanup("mysql_storage_get_works_for_wrong_key").await;

        {
            let storage = _storage("mysql_storage_get_works_for_wrong_key").await;

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

        _cleanup("mysql_storage_get_works_for_wrong_key").await;
    }

    #[async_std::test]
    async fn mysql_storage_delete_works() {
        _cleanup("mysql_storage_delete_works").await;

        {
            let storage = _storage("mysql_storage_delete_works").await;

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

        _cleanup("mysql_storage_delete_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_delete_works_for_non_existing() {
        _cleanup("mysql_storage_delete_works_for_non_existing").await;

        {
            let storage = _storage("mysql_storage_delete_works_for_non_existing").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.delete(&_type1(), &_id2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("mysql_storage_delete_works_for_non_existing").await;
    }

    #[async_std::test]
    async fn mysql_storage_delete_returns_error_item_not_found_if_no_such_type() {
        _cleanup("mysql_storage_delete_returns_error_item_not_found_if_no_such_type").await;

        {
            let storage =
                _storage("mysql_storage_delete_returns_error_item_not_found_if_no_such_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.delete(&_type2(), &_id2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("mysql_storage_delete_returns_error_item_not_found_if_no_such_type").await;
    }

    #[async_std::test]
    async fn mysql_storage_get_all_works() {
        _cleanup("mysql_storage_get_all_works").await;

        {
            let storage = _storage("mysql_storage_get_all_works").await;

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

        _cleanup("mysql_storage_get_all_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_get_all_works_for_empty() {
        _cleanup("mysql_storage_get_all_works_for_empty").await;

        {
            let storage = _storage("mysql_storage_get_all_works_for_empty").await;
            let mut storage_iterator = storage.get_all().await.unwrap();

            let record = storage_iterator.next().await.unwrap();
            assert!(record.is_none());
        }

        _cleanup("mysql_storage_get_all_works_for_empty").await;
    }

    #[async_std::test]
    async fn mysql_storage_update_works() {
        _cleanup("mysql_storage_update_works").await;

        {
            let storage = _storage("mysql_storage_update_works").await;

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

        _cleanup("mysql_storage_update_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_update_works_for_non_existing_id() {
        _cleanup("mysql_storage_update_works_for_non_existing_id").await;

        {
            let storage = _storage("mysql_storage_update_works_for_non_existing_id").await;

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

        _cleanup("mysql_storage_update_works_for_non_existing_id").await;
    }

    #[async_std::test]
    async fn mysql_storage_update_works_for_non_existing_type() {
        _cleanup("mysql_storage_update_works_for_non_existing_type").await;

        {
            let storage = _storage("mysql_storage_update_works_for_non_existing_type").await;

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

        _cleanup("mysql_storage_update_works_for_non_existing_type").await;
    }

    #[async_std::test]
    async fn mysql_storage_add_tags_works() {
        _cleanup("mysql_storage_add_tags_works").await;

        {
            let storage = _storage("mysql_storage_add_tags_works").await;

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

        _cleanup("mysql_storage_add_tags_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_add_tags_works_for_non_existing_id() {
        _cleanup("mysql_storage_add_tags_works_for_non_existing_id").await;

        {
            let storage = _storage("mysql_storage_add_tags_works_for_non_existing_id").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("mysql_storage_add_tags_works_for_non_existing_id").await;
    }

    #[async_std::test]
    async fn mysql_storage_add_tags_works_for_non_existing_type() {
        _cleanup("mysql_storage_add_tags_works_for_non_existing_type").await;

        {
            let storage = _storage("mysql_storage_add_tags_works_for_non_existing_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add_tags(&_type2(), &_id1(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("mysql_storage_add_tags_works_for_non_existing_type").await;
    }

    #[async_std::test]
    async fn mysql_storage_add_tags_works_for_already_existing() {
        _cleanup("mysql_storage_add_tags_works_for_already_existing").await;

        {
            let storage = _storage("mysql_storage_add_tags_works_for_already_existing").await;

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

        _cleanup("mysql_storage_add_tags_works_for_already_existing").await;
    }

    #[async_std::test]
    async fn mysql_storage_update_tags_works() {
        _cleanup("mysql_storage_update_tags_works").await;

        {
            let storage = _storage("mysql_storage_update_tags_works").await;

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

        _cleanup("mysql_storage_update_tags_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_update_tags_works_for_non_existing_id() {
        _cleanup("mysql_storage_update_tags_works_for_non_existing_id").await;

        {
            let storage = _storage("mysql_storage_update_tags_works_for_non_existing_id").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.update_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("mysql_storage_update_tags_works_for_non_existing_id").await;
    }

    #[async_std::test]
    async fn mysql_storage_update_tags_works_for_non_existing_type() {
        _cleanup("mysql_storage_update_tags_works_for_non_existing_type").await;

        {
            let storage = _storage("mysql_storage_update_tags_works_for_non_existing_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.update_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("mysql_storage_update_tags_works_for_non_existing_type").await;
    }

    #[async_std::test]
    async fn mysql_storage_update_tags_works_for_already_existing() {
        _cleanup("mysql_storage_update_tags_works_for_already_existing").await;
        {
            let storage = _storage("mysql_storage_update_tags_works_for_already_existing").await;

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
        _cleanup("mysql_storage_update_tags_works_for_already_existing").await;
    }

    #[async_std::test]
    async fn mysql_storage_delete_tags_works() {
        _cleanup("mysql_storage_delete_tags_works").await;

        {
            let storage = _storage("mysql_storage_delete_tags_works").await;

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

        _cleanup("mysql_storage_delete_tags_works").await;
    }

    #[async_std::test]
    async fn mysql_storage_delete_tags_works_for_non_existing_type() {
        _cleanup("mysql_storage_delete_tags_works_for_non_existing_type").await;

        {
            let storage = _storage("mysql_storage_delete_tags_works_for_non_existing_type").await;

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

        _cleanup("mysql_storage_delete_tags_works_for_non_existing_type").await;
    }

    #[async_std::test]
    async fn mysql_storage_delete_tags_works_for_non_existing_id() {
        _cleanup("mysql_storage_delete_tags_works_for_non_existing_id").await;

        {
            let storage = _storage("mysql_storage_delete_tags_works_for_non_existing_id").await;

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

        _cleanup("mysql_storage_delete_tags_works_for_non_existing_id").await;
    }

    fn _config() -> Option<&'static str> {
        Some(
            r#"
            {
                "read_host": "127.0.0.1",
                "write_host": "127.0.0.1",
                "port": 3306,
                "db_name": "indy"
            }
            "#,
        )
    }

    fn _credentials() -> Option<&'static str> {
        Some(
            r#"
            {
                "user": "root",
                "pass": "pass@word1"
            }
            "#,
        )
    }

    async fn _cleanup(name: &str) {
        MySqlStorageType::new()
            .delete_storage(name, _config(), _credentials())
            .await
            .ok();
    }

    async fn _storage(name: &str) -> Box<dyn WalletStorage> {
        let storage_type = MySqlStorageType::new();

        storage_type
            .create_storage(name, _config(), _credentials(), &_metadata())
            .await
            .unwrap();

        storage_type
            .open_storage(name, _config(), _credentials())
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
            key: vec![
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
                9 + i,
                10 + i,
                11 + i,
            ],
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

// FIXME: copy/paste
fn _tags_to_plain(tags: &[Tag]) -> HashMap<String, String> {
    let mut map = HashMap::with_capacity(tags.len());

    for tag in tags {
        match *tag {
            Tag::Encrypted(ref name, ref value) => {
                map.insert(base64::encode(&name), base64::encode(&value))
            }
            Tag::PlainText(ref name, ref value) => {
                map.insert(format!("~{}", &base64::encode(&name)), value.to_string())
            }
        };
    }

    map
}

// FIXME: copy/paste
fn _tags_to_json(tags: &[Tag]) -> IndyResult<String> {
    serde_json::to_string(&_tags_to_plain(tags)).to_indy(
        IndyErrorKind::InvalidState,
        "Unable to serialize tags as json",
    )
}

// FIXME: copy/paste
fn _tags_from_json(json: serde_json::Value) -> IndyResult<Vec<Tag>> {
    let string_tags: HashMap<String, String> = serde_json::from_value(json).to_indy(
        IndyErrorKind::InvalidState,
        "Unable to deserialize tags from json",
    )?;

    let mut tags = Vec::with_capacity(string_tags.len());

    for (k, v) in string_tags {
        if k.starts_with('~') {
            let mut key = k;
            key.remove(0);
            tags.push(Tag::PlainText(
                base64::decode(&key).to_indy(
                    IndyErrorKind::InvalidState,
                    "Unable to decode tag key from base64",
                )?,
                v,
            ));
        } else {
            tags.push(Tag::Encrypted(
                base64::decode(&k).to_indy(
                    IndyErrorKind::InvalidState,
                    "Unable to decode tag key from base64",
                )?,
                base64::decode(&v).to_indy(
                    IndyErrorKind::InvalidState,
                    "Unable to decode tag value from base64",
                )?,
            ));
        }
    }
    Ok(tags)
}

// FIXME: copy/paste
fn _tag_names_to_plain(tag_names: &[TagName]) -> Vec<String> {
    tag_names
        .iter()
        .map(|tag_name| match *tag_name {
            TagName::OfEncrypted(ref tag_name) => base64::encode(tag_name),
            TagName::OfPlain(ref tag_name) => format!("~{}", base64::encode(tag_name)),
        })
        .collect()
}

// FIXME: copy/paste
fn _tag_names_to_json(tag_names: &[TagName]) -> IndyResult<String> {
    serde_json::to_string(&_tag_names_to_plain(tag_names)).to_indy(
        IndyErrorKind::InvalidState,
        "Unable to serialize tag names as json",
    )
}
