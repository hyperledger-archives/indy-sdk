use std::rc::Rc;

use indy_api_types::errors::IndyError;

use super::{WalletRecord, wallet::Keys, storage::StorageIterator, encryption::decrypt_storage_record};

pub(super) struct WalletIterator {
    storage_iterator: Box<dyn StorageIterator>,
    keys: Rc<Keys>,
}

impl WalletIterator {
    pub fn new(storage_iter: Box<dyn StorageIterator>, keys: Rc<Keys>) -> Self {
        WalletIterator {
            storage_iterator: storage_iter,
            keys,
        }
    }

    pub async fn next(&mut self) -> Result<Option<WalletRecord>, IndyError> {
        let next_storage_entity = self.storage_iterator.next().await?;

        if let Some(next_storage_entity) = next_storage_entity {
            Ok(Some(decrypt_storage_record(
                &next_storage_entity,
                &self.keys,
            )?))
        } else {
            Ok(None)
        }
    }

    pub fn get_total_count(&self) -> Result<Option<usize>, IndyError> {
        Ok(self.storage_iterator.get_total_count()?)
    }
}
