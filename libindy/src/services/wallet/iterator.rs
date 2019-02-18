use std::rc::Rc;

use errors::IndyError;

use super::WalletRecord;
use super::wallet::Keys;
use super::storage::StorageIterator;
use super::encryption::{decrypt_storage_record};

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

    pub fn next(&mut self) -> Result<Option<WalletRecord>, IndyError> {
        let next_storage_entity = self.storage_iterator.next()?;
        if let Some(next_storage_entity) = next_storage_entity {
            let record = decrypt_storage_record(&next_storage_entity, &self.keys)?;
            Ok(Some(record))
        } else { Ok(None) }
    }

    pub fn get_total_count(&self) -> Result<Option<usize>, IndyError> {
        let total_count = self.storage_iterator.get_total_count()?;
        Ok(total_count)
    }
}