use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    iter::Iterator,
    pin::Pin,
};

use async_trait::async_trait;
use futures::{Stream, TryStreamExt};
use indy_api_types::errors::prelude::*;
use indy_utils::{crypto::base64, environment};
use serde::Deserialize;

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlRow},
    ConnectOptions, Done, MySqlPool, Row,
};

use crate::{
    language,
    storage::{StorageIterator, StorageRecord, Tag, TagName, WalletStorage, WalletStorageType},
    wallet::EncryptedValue,
    RecordOptions, SearchOptions,
};
use async_std::sync::RwLock;

//mod query;

struct MySQLStorageIterator {
    records: VecDeque<IndyResult<StorageRecord>>,
    total_count: Option<usize>,
}

impl MySQLStorageIterator {
    fn new(records: VecDeque<IndyResult<StorageRecord>>) -> IndyResult<MySQLStorageIterator> {
        let total_count = Some(records.len());

        Ok(MySQLStorageIterator {
            records,
            total_count,
        })
    }
}

#[async_trait]
impl StorageIterator for MySQLStorageIterator {
    async fn next(&mut self) -> IndyResult<Option<StorageRecord>> {
        if let Some(record) = self.records.pop_front() {
            return Ok(Some(record?));
        }

        Ok(None)
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
    wallet_id: u64,
    read_pool: MySqlPool,
    write_pool: MySqlPool,
}

pub struct MySqlStorageType {
    connections: RwLock<HashMap<String, MySqlPool>>,
}

impl MySqlStorageType {
    pub fn new() -> MySqlStorageType {
        MySqlStorageType {
            connections: RwLock::new(HashMap::new()),
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
                "Absent config json",
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

        if let Some(connection) = self.connections.read().await.get(&connection_string) {
            return Ok(connection.clone());
        }

        let connection = MySqlPoolOptions::default()
            .connect_with(
                MySqlConnectOptions::new()
                    .host(host_addr)
                    .database(&config.db_name)
                    .username(&credentials.user)
                    .password(&credentials.pass),
            )
            .await?;

        self.connections
            .write()
            .await
            .insert(connection_string, connection.clone());
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

        let (value, tags): (Vec<u8>, String) = sqlx::query_as(&format!(
            r#"
            SELECT {}, {}
            FROM items
            WHERE
                wallet_id = ?1
                    AND type = ?2
                    AND name = ?3
            "#,
            if options.retrieve_value {
                "value"
            } else {
                "''"
            },
            if options.retrieve_tags { "tags" } else { "''" },
        ))
        .bind(&base64::encode(type_))
        .bind(&base64::encode(id))
        .fetch_one(&mut conn)
        .await?;

        let value = if options.retrieve_value {
            Some(EncryptedValue::from_bytes(&value)?)
        } else {
            None
        };

        let type_ = if options.retrieve_type {
            Some(type_.to_vec())
        } else {
            None
        };

        let tags = if options.retrieve_tags {
            Some(_tags_from_json(&tags)?)
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
            VALUE (?1, ?2, ?3, ?4, ?5)
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
            SET value = ?1
            WHERE type = ?2 AND name = ?3 AND wallet_id = 4
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
            .map(|(tag, val)| format!(r#"'$."{}"', {}"#, tag, val))
            .collect::<Vec<_>>()
            .join(",");

        let mut tx = self.write_pool.begin().await?;

        let row_updated = sqlx::query(&format!(
            r#"
            UPDATE items
                SET tags = JSON_SET(tags, {})
            WHERE type = ?1
                AND name = ?2
                AND wallet_id = ?3
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
            SET tags = ?1
            WHERE type = ?2
                AND name = ?3
                AND wallet_id = ?4
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
            WHERE type = ?1
                AND name = ?2
                AND wallet_id = ?3
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
            WHERE type = ?1
                AND name = ?2
                AND wallet_id = ?3"#,
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

        let (metadata,): (Vec<u8>,) = sqlx::query_as::<_, (Vec<u8>,)>(
            r#"
            SELECT metadata
            FROM wallets
            WHERE id = ?1
            "#,
        )
        .bind(&self.wallet_id)
        .fetch_one(&mut conn)
        .await?;

        Ok(metadata)
    }

    async fn set_storage_metadata(&self, metadata: &[u8]) -> IndyResult<()> {
        let mut tx = self.write_pool.begin().await?;

        sqlx::query(
            r#"
            UPDATE wallets
            SET metadata = ?1
            WHERE id = ?2
            "#,
        )
        .bind(metadata)
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
            WHERE wallet_id = ?1
            "#,
        )
        .bind(self.wallet_id)
        .map(|r: MySqlRow| -> IndyResult<StorageRecord> {
            let type_: String = r.get(0);
            let id: String = r.get(1);
            let value: Vec<u8> = r.get(2);
            let tags: String = r.get(3);

            let res = StorageRecord::new(
                base64::decode(&id)?,
                Some(EncryptedValue::from_bytes(&value)?),
                Some(base64::decode(&type_)?),
                Some(_tags_from_json(&tags)?),
            );

            Ok(res)
        })
        .fetch_all(&self.read_pool)
        .await?
        .into_iter()
        .collect();

        // FIXME: Fetch total count
        Ok(Box::new(MySQLStorageIterator::new(records)?))
    }

    async fn search(
        &self,
        type_: &[u8],
        query: &language::Operator,
        options: Option<&str>,
    ) -> IndyResult<Box<dyn StorageIterator>> {
        unimplemented!();
    }

    async fn close(&mut self) -> IndyResult<()> {
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

        let rows_affected = sqlx::query(
            r#"
            DELETE FROM wallets
            WHERE name = ?1
            "#,
        )
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

        sqlx::query(
            r#"
            INSERT INTO wallets (name, metadata)
            VALUES (?1, ?2)
            "#,
        )
        .bind(id)
        .bind(metadata)
        .execute(&mut tx)
        .await?; // FIXME: return wallet already exists on 1062 error code from MySQL

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

        let (wallet_id,) = sqlx::query_as::<_, (u64,)>(
            r#"
            SELECT id FROM wallets
            WHERE name = ?1
            "#,
        )
        .bind(id)
        .fetch_one(&read_pool)
        .await?;

        Ok(Box::new(MySqlStorage {
            read_pool,
            write_pool,
            wallet_id,
        }))
    }
}

#[cfg(test)]
mod tests {
    use indy_utils::{assert_kind, test};
    use serde_json::json;

    use super::super::Tag;
    use super::*;
    use std::path::Path;

    #[async_std::test]
    async fn smysql_storage_type_create_works() {
        _cleanup("smysql_storage_type_create_works");

        let storage_type = MySqlStorageType::new();

        storage_type
            .create_storage("smysql_storage_type_create_works", None, None, &_metadata())
            .await
            .unwrap();

        _cleanup("smysql_storage_type_create_works");
    }

    #[async_std::test]
    async fn smysql_storage_type_create_works_for_custom_path() {
        _cleanup("smysql_storage_type_create_works_for_custom_path");

        let config = json!({
            "path": _custom_path("smysql_storage_type_create_works_for_custom_path")
        })
        .to_string();

        _cleanup_custom_path("smysql_storage_type_create_works_for_custom_path");
        let storage_type = MySqlStorageType::new();

        storage_type
            .create_storage(
                "smysql_storage_type_create_works_for_custom_path",
                Some(&config),
                None,
                &_metadata(),
            )
            .await
            .unwrap();

        storage_type
            .delete_storage(
                "smysql_storage_type_create_works_for_custom_path",
                Some(&config),
                None,
            )
            .await
            .unwrap();

        _cleanup_custom_path("smysql_storage_type_create_works_for_custom_path");
        _cleanup("smysql_storage_type_create_works_for_custom_path");
    }

    fn _cleanup_custom_path(custom_path: &str) {
        let my_path = _custom_path(custom_path);
        let path = Path::new(&my_path);
        if path.exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    #[async_std::test]
    async fn smysql_storage_type_create_works_for_twice() {
        _cleanup("smysql_storage_type_create_works_for_twice");

        let storage_type = MySqlStorageType::new();
        storage_type
            .create_storage(
                "smysql_storage_type_create_works_for_twice",
                None,
                None,
                &_metadata(),
            )
            .await
            .unwrap();

        let res = storage_type
            .create_storage(
                "smysql_storage_type_create_works_for_twice",
                None,
                None,
                &_metadata(),
            )
            .await;

        assert_kind!(IndyErrorKind::WalletAlreadyExists, res);

        storage_type
            .delete_storage("smysql_storage_type_create_works_for_twice", None, None)
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn smysql_storage_get_storage_metadata_works() {
        _cleanup("smysql_storage_get_storage_metadata_works");

        {
            let storage = _storage("smysql_storage_get_storage_metadata_works").await;
            let metadata = storage.get_storage_metadata().await.unwrap();

            assert_eq!(metadata, _metadata());
        }

        _cleanup("smysql_storage_get_storage_metadata_works");
    }

    #[async_std::test]
    async fn smysql_storage_type_delete_works() {
        _cleanup("smysql_storage_type_delete_works");

        let storage_type = MySqlStorageType::new();
        storage_type
            .create_storage("smysql_storage_type_delete_works", None, None, &_metadata())
            .await
            .unwrap();

        storage_type
            .delete_storage("smysql_storage_type_delete_works", None, None)
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn smysql_storage_type_delete_works_for_non_existing() {
        _cleanup("smysql_storage_type_delete_works_for_non_existing");

        let storage_type = MySqlStorageType::new();

        storage_type
            .create_storage(
                "smysql_storage_type_delete_works_for_non_existing",
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
                "smysql_storage_type_delete_works_for_non_existing",
                None,
                None,
            )
            .await
            .unwrap();
    }

    #[async_std::test]
    async fn smysql_storage_type_open_works() {
        _cleanup("smysql_storage_type_open_works");
        _storage("smysql_storage_type_open_works").await;
        _cleanup("smysql_storage_type_open_works");
    }

    #[async_std::test]
    async fn smysql_storage_type_open_works_for_custom() {
        _cleanup("smysql_storage_type_open_works_for_custom");

        let my_path = _custom_path("smysql_storage_type_open_works_for_custom");
        let path = Path::new(&my_path);

        if path.exists() && path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        }

        _storage_custom("smysql_storage_type_open_works_for_custom").await;

        fs::remove_dir_all(path).unwrap();
    }

    #[async_std::test]
    async fn smysql_storage_type_open_works_for_not_created() {
        _cleanup("smysql_storage_type_open_works_for_not_created");

        let storage_type = MySqlStorageType::new();

        let res = storage_type
            .open_storage("unknown", Some("{}"), Some("{}"))
            .await;

        assert_kind!(IndyErrorKind::WalletNotFound, res);
    }

    #[async_std::test]
    async fn smysql_storage_add_works_for_is_802() {
        _cleanup("smysql_storage_add_works_for_is_802");

        {
            let storage = _storage("smysql_storage_add_works_for_is_802").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);

            let res = storage.add(&_type1(), &_id1(), &_value1(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
        }

        _cleanup("smysql_storage_add_works_for_is_802");
    }

    #[async_std::test]
    async fn smysql_storage_set_get_works() {
        _cleanup("smysql_storage_set_get_works");

        {
            let storage = _storage("smysql_storage_set_get_works").await;

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

        _cleanup("smysql_storage_set_get_works");
    }

    #[async_std::test]
    async fn smysql_storage_set_get_works_for_custom() {
        _cleanup("smysql_storage_set_get_works_for_custom");

        let path = _custom_path("smysql_storage_set_get_works_for_custom");
        let path = Path::new(&path);

        {
            let storage = _storage_custom("smysql_storage_set_get_works_for_custom").await;

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
    async fn smysql_storage_set_get_works_for_twice() {
        _cleanup("smysql_storage_set_get_works_for_twice");

        {
            let storage = _storage("smysql_storage_set_get_works_for_twice").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add(&_type1(), &_id1(), &_value2(), &_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemAlreadyExists, res);
        }

        _cleanup("smysql_storage_set_get_works_for_twice");
    }

    #[async_std::test]
    async fn smysql_storage_set_get_works_for_reopen() {
        _cleanup("smysql_storage_set_get_works_for_reopen");

        _storage("smysql_storage_set_get_works_for_reopen")
            .await
            .add(&_type1(), &_id1(), &_value1(), &_tags())
            .await
            .unwrap();

        let record = MySqlStorageType::new()
            .open_storage(
                "smysql_storage_set_get_works_for_reopen",
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

        _cleanup("smysql_storage_set_get_works_for_reopen");
    }

    #[async_std::test]
    async fn smysql_storage_get_works_for_wrong_key() {
        _cleanup("smysql_storage_get_works_for_wrong_key");

        {
            let storage = _storage("smysql_storage_get_works_for_wrong_key").await;

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

        _cleanup("smysql_storage_get_works_for_wrong_key");
    }

    #[async_std::test]
    async fn smysql_storage_delete_works() {
        _cleanup("smysql_storage_delete_works");

        {
            let storage = _storage("smysql_storage_delete_works").await;

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

        _cleanup("smysql_storage_delete_works");
    }

    #[async_std::test]
    async fn smysql_storage_delete_works_for_non_existing() {
        _cleanup("smysql_storage_delete_works_for_non_existing");

        {
            let storage = _storage("smysql_storage_delete_works_for_non_existing").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.delete(&_type1(), &_id2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("smysql_storage_delete_works_for_non_existing");
    }

    #[async_std::test]
    async fn smysql_storage_delete_returns_error_item_not_found_if_no_such_type() {
        _cleanup("smysql_storage_delete_returns_error_item_not_found_if_no_such_type");

        {
            let storage =
                _storage("smysql_storage_delete_returns_error_item_not_found_if_no_such_type")
                    .await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.delete(&_type2(), &_id2()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("smysql_storage_delete_returns_error_item_not_found_if_no_such_type");
    }

    #[async_std::test]
    async fn smysql_storage_get_all_works() {
        _cleanup("smysql_storage_get_all_works");

        {
            let storage = _storage("smysql_storage_get_all_works").await;

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

        _cleanup("smysql_storage_get_all_works");
    }

    #[async_std::test]
    async fn smysql_storage_get_all_works_for_empty() {
        _cleanup("smysql_storage_get_all_works_for_empty");

        {
            let storage = _storage("smysql_storage_get_all_works_for_empty").await;
            let mut storage_iterator = storage.get_all().await.unwrap();

            let record = storage_iterator.next().await.unwrap();
            assert!(record.is_none());
        }

        _cleanup("smysql_storage_get_all_works_for_empty");
    }

    #[async_std::test]
    async fn smysql_storage_update_works() {
        _cleanup("smysql_storage_update_works");

        {
            let storage = _storage("smysql_storage_update_works").await;

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

        _cleanup("smysql_storage_update_works");
    }

    #[async_std::test]
    async fn smysql_storage_update_works_for_non_existing_id() {
        _cleanup("smysql_storage_update_works_for_non_existing_id");

        {
            let storage = _storage("smysql_storage_update_works_for_non_existing_id").await;

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

        _cleanup("smysql_storage_update_works_for_non_existing_id");
    }

    #[async_std::test]
    async fn smysql_storage_update_works_for_non_existing_type() {
        _cleanup("smysql_storage_update_works_for_non_existing_type");

        {
            let storage = _storage("smysql_storage_update_works_for_non_existing_type").await;

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

        _cleanup("smysql_storage_update_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn smysql_storage_add_tags_works() {
        _cleanup("smysql_storage_add_tags_works");

        {
            let storage = _storage("smysql_storage_add_tags_works").await;

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

        _cleanup("smysql_storage_add_tags_works");
    }

    #[async_std::test]
    async fn smysql_storage_add_tags_works_for_non_existing_id() {
        _cleanup("smysql_storage_add_tags_works_for_non_existing_id");

        {
            let storage = _storage("smysql_storage_add_tags_works_for_non_existing_id").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("smysql_storage_add_tags_works_for_non_existing_id");
    }

    #[async_std::test]
    async fn smysql_storage_add_tags_works_for_non_existing_type() {
        _cleanup("smysql_storage_add_tags_works_for_non_existing_type");

        {
            let storage = _storage("smysql_storage_add_tags_works_for_non_existing_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.add_tags(&_type2(), &_id1(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("smysql_storage_add_tags_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn smysql_storage_add_tags_works_for_already_existing() {
        _cleanup("smysql_storage_add_tags_works_for_already_existing");

        {
            let storage = _storage("smysql_storage_add_tags_works_for_already_existing").await;

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

        _cleanup("smysql_storage_add_tags_works_for_already_existing");
    }

    #[async_std::test]
    async fn smysql_storage_update_tags_works() {
        _cleanup("smysql_storage_update_tags_works");

        {
            let storage = _storage("smysql_storage_update_tags_works").await;

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

        _cleanup("smysql_storage_update_tags_works");
    }

    #[async_std::test]
    async fn smysql_storage_update_tags_works_for_non_existing_id() {
        _cleanup("smysql_storage_update_tags_works_for_non_existing_id");

        {
            let storage = _storage("smysql_storage_update_tags_works_for_non_existing_id").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.update_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("smysql_storage_update_tags_works_for_non_existing_id");
    }

    #[async_std::test]
    async fn smysql_storage_update_tags_works_for_non_existing_type() {
        _cleanup("smysql_storage_update_tags_works_for_non_existing_type");

        {
            let storage = _storage("smysql_storage_update_tags_works_for_non_existing_type").await;

            storage
                .add(&_type1(), &_id1(), &_value1(), &_tags())
                .await
                .unwrap();

            let res = storage.update_tags(&_type1(), &_id2(), &_new_tags()).await;
            assert_kind!(IndyErrorKind::WalletItemNotFound, res);
        }

        _cleanup("smysql_storage_update_tags_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn smysql_storage_update_tags_works_for_already_existing() {
        _cleanup("smysql_storage_update_tags_works_for_already_existing");
        {
            let storage = _storage("smysql_storage_update_tags_works_for_already_existing").await;

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
        _cleanup("smysql_storage_update_tags_works_for_already_existing");
    }

    #[async_std::test]
    async fn smysql_storage_delete_tags_works() {
        _cleanup("smysql_storage_delete_tags_works");

        {
            let storage = _storage("smysql_storage_delete_tags_works").await;

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

        _cleanup("smysql_storage_delete_tags_works");
    }

    #[async_std::test]
    async fn smysql_storage_delete_tags_works_for_non_existing_type() {
        _cleanup("smysql_storage_delete_tags_works_for_non_existing_type");

        {
            let storage = _storage("smysql_storage_delete_tags_works_for_non_existing_type").await;

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

        _cleanup("smysql_storage_delete_tags_works_for_non_existing_type");
    }

    #[async_std::test]
    async fn smysql_storage_delete_tags_works_for_non_existing_id() {
        _cleanup("smysql_storage_delete_tags_works_for_non_existing_id");

        {
            let storage = _storage("smysql_storage_delete_tags_works_for_non_existing_id").await;

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

        _cleanup("smysql_storage_delete_tags_works_for_non_existing_id");
    }

    fn _cleanup(name: &str) {
        test::cleanup_storage(name)
    }

    async fn _storage(name: &str) -> Box<dyn WalletStorage> {
        let storage_type = MySqlStorageType::new();

        storage_type
            .create_storage(name, None, None, &_metadata())
            .await
            .unwrap();

        storage_type.open_storage(name, None, None).await.unwrap()
    }

    async fn _storage_custom(name: &str) -> Box<dyn WalletStorage> {
        let storage_type = MySqlStorageType::new();

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
fn _tags_from_json(json: &str) -> IndyResult<Vec<Tag>> {
    let string_tags: HashMap<String, String> = serde_json::from_str(json).to_indy(
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
