use std::rc::Rc;

use errors::wallet::WalletError;

use super::WalletRecord;
use super::wallet::Keys;
use super::storage::StorageIterator;
use super::encryption::{decrypt_merged, decrypt_tags};


pub(super) struct WalletIterator {
    storage_iterator: Box<StorageIterator>,
    keys: Rc<Keys>,
}


impl WalletIterator {
    pub fn new(storage_iter: Box<StorageIterator>, keys: Rc<Keys>) -> Self {
        WalletIterator {
            storage_iterator: storage_iter,
            keys: keys,
        }
    }

    pub fn next(&mut self) -> Result<Option<WalletRecord>, WalletError> {
        let next_storage_entity = self.storage_iterator.next()?;
        if let Some(next_storage_entity) = next_storage_entity {
            let decrypted_name = decrypt_merged(&next_storage_entity.id, &self.keys.name_key)?;
            let name = String::from_utf8(decrypted_name)?;

            let type_ = match next_storage_entity.type_ {
                None => None,
                Some(encrypted_type) => Some(String::from_utf8(decrypt_merged(&encrypted_type, &self.keys.type_key)?)?)
            };

            let value = match next_storage_entity.value {
                None => None,
                Some(encrypted_value) => Some(encrypted_value.decrypt(&self.keys.value_key)?)
            };

            let tags = decrypt_tags(&next_storage_entity.tags, &self.keys.tag_name_key, &self.keys.tag_value_key)?;

            Ok(Some(WalletRecord::new(name, type_, value, tags)))
        } else { Ok(None) }
    }

    pub fn get_total_count(&self) -> Result<Option<usize>, WalletError> {
        let total_count = self.storage_iterator.get_total_count()?;
        Ok(total_count)
    }
}