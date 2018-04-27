use serde_json;

use errors::wallet::WalletError;

use super::WalletRecord;
use super::wallet::Keys;
use super::storage::StorageIterator;
use super::encryption::{decrypt,decrypt_tags};


pub(super) struct WalletIterator<'a> {
    storage_iterator: Box<StorageIterator + 'a>,
    keys: &'a Keys
}


impl<'a> WalletIterator<'a> {
    pub fn new(storage_iter: Box<StorageIterator + 'a>, keys: &'a Keys) -> Self {
        WalletIterator {
            storage_iterator: storage_iter,
            keys: keys
        }
    }

    pub fn next(&mut self) -> Result<Option<WalletRecord>, WalletError> {
        let next_storage_entity = self.storage_iterator.next()?;
        if let Some(next_storage_entity) = next_storage_entity {
            let decrypted_name = decrypt(&next_storage_entity.name, self.keys.name_key)?;
            let name = String::from_utf8(decrypted_name)?;

            let value = match next_storage_entity.value {
                None => None,
                Some(storage_value) => {
                    let value_key = decrypt(&storage_value.key, self.keys.value_key)?;
                    if value_key.len() != 32 {
                        return Err(WalletError::EncryptionError("Value key is not right size".to_string()));
                    }
                    let mut vkey: [u8; 32] = Default::default();
                    vkey.copy_from_slice(&value_key);
                    Some(String::from_utf8(decrypt(&storage_value.data, vkey)?)?)
                }
            };

            let tags = match decrypt_tags(&next_storage_entity.tags, self.keys.tag_name_key, self.keys.tag_value_key)? {
                None => None,
                Some(tags) => Some(serde_json::to_string(&tags)?)
            };

            Ok(Some(WalletRecord::new(name, None, value, tags)))
        } else { Ok(None) }
    }
}