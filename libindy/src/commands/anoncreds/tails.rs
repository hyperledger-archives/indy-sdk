extern crate digest;
extern crate indy_crypto;
extern crate sha2;
extern crate rust_base58;

use errors::common::CommonError;
use services::blob_storage::BlobStorageService;
use domain::revocation_registry_definition::RevocationRegistryDefinitionV1;

use self::indy_crypto::cl::{Tail, RevocationTailsAccessor, RevocationTailsGenerator};
use self::indy_crypto::errors::IndyCryptoError;
use self::digest::Input;

use self::rust_base58::{ToBase58, FromBase58};

use std::rc::Rc;

const _TAILS_BLOB_TAG_SZ: usize = 2;
const TAIL_SIZE: usize = Tail::BYTES_REPR_SIZE;

pub struct SDKTailsAccessor {
    tails_service: Rc<BlobStorageService>,
    tails_reader_handle: i32,
}

impl SDKTailsAccessor {
    pub fn new(tails_service: Rc<BlobStorageService>,
               tails_reader_handle: i32,
               rev_reg_def: &RevocationRegistryDefinitionV1) -> Result<SDKTailsAccessor, CommonError> {
        let tails_hash = rev_reg_def.value.tails_hash.from_base58()
            .map_err(|_| CommonError::InvalidState(format!("Invalid base58 for Tails hash")))?;

        let tails_reader_handle = tails_service.open_blob(tails_reader_handle,
                                                          &rev_reg_def.value.tails_location,
                                                          tails_hash.as_slice())?;
        Ok(SDKTailsAccessor {
            tails_service,
            tails_reader_handle
        })
    }
}

impl Drop for SDKTailsAccessor {
    fn drop(&mut self) {
        #[allow(unused_must_use)] //TODO
            {
                self.tails_service.close(self.tails_reader_handle)
                    .map_err(map_err_err!());
            }
    }
}

impl RevocationTailsAccessor for SDKTailsAccessor {
    fn access_tail(&self, tail_id: u32, accessor: &mut FnMut(&Tail)) -> Result<(), IndyCryptoError> {
        let tail_bytes = self.tails_service
            .read(self.tails_reader_handle,
                  TAIL_SIZE,
                  TAIL_SIZE * tail_id as usize)  // + _TAILS_BLOB_TAG_SZ
            .map_err(|_|
                IndyCryptoError::InvalidState("Can't read tail bytes from blob storage".to_owned()))?; //TODO
        let tail = Tail::from_bytes(tail_bytes.as_slice())?;
        accessor(&tail);
        Ok(())
    }
}

pub fn store_tails_from_generator(service: Rc<BlobStorageService>,
                                  writer_handle: i32,
                                  rtg: &mut RevocationTailsGenerator) -> Result<(String, String), CommonError> {
    trace!("store_tails_from_generator ---> start");

    let blob_handle = service.create_blob(writer_handle)?;

    let mut hasher = sha2::Sha256::default();

    //FIXME store version/tag/meta at start of the Tail's BLOB

    while let Some(tail) = rtg.next()? {
        let tail_bytes = tail.to_bytes()?;
        hasher.process(tail_bytes.as_slice());
        service.append(blob_handle, tail_bytes.as_slice())?;
    }

    let res = service.finalize(blob_handle).map(|(location, hash)| (location, hash.to_base58()))?;

    trace!("finalize ---> end");
    Ok(res)
}
