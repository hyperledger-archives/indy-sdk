use errors::prelude::*;
use services::blob_storage::BlobStorageService;
use domain::anoncreds::revocation_registry_definition::RevocationRegistryDefinitionV1;

use ursa::cl::{Tail, RevocationTailsAccessor, RevocationTailsGenerator};
use ursa::errors::prelude::{UrsaCryptoError, UrsaCryptoErrorKind};

use rust_base58::{ToBase58, FromBase58};

use std::rc::Rc;

const TAILS_BLOB_TAG_SZ: u8 = 2;
const TAIL_SIZE: usize = Tail::BYTES_REPR_SIZE;

pub struct SDKTailsAccessor {
    tails_service: Rc<BlobStorageService>,
    tails_reader_handle: i32,
}

impl SDKTailsAccessor {
    pub fn new(tails_service: Rc<BlobStorageService>,
               tails_reader_handle: i32,
               rev_reg_def: &RevocationRegistryDefinitionV1) -> IndyResult<SDKTailsAccessor> {
        let tails_hash = rev_reg_def.value.tails_hash.from_base58()
            .map_err(|_| err_msg(IndyErrorKind::InvalidState, "Invalid base58 for Tails hash"))?;

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
    fn access_tail(&self, tail_id: u32, accessor: &mut FnMut(&Tail)) -> Result<(), UrsaCryptoError> {
        debug!("access_tail >>> tail_id: {:?}", tail_id);

        let tail_bytes = self.tails_service
            .read(self.tails_reader_handle,
                  TAIL_SIZE,
                  TAIL_SIZE * tail_id as usize + TAILS_BLOB_TAG_SZ as usize)
            .map_err(|_|
                UrsaCryptoError::from_msg(UrsaCryptoErrorKind::InvalidState, "Can't read tail bytes from blob storage"))?; // FIXME: IO error should be returned

        let tail = Tail::from_bytes(tail_bytes.as_slice())?;
        accessor(&tail);

        let res = ();
        debug!("access_tail <<< res: {:?}", res);
        Ok(res)
    }
}

pub fn store_tails_from_generator(service: Rc<BlobStorageService>,
                                  writer_handle: i32,
                                  rtg: &mut RevocationTailsGenerator) -> IndyResult<(String, String)> {
    debug!("store_tails_from_generator >>> writer_handle: {:?}", writer_handle);

    let blob_handle = service.create_blob(writer_handle)?;

    let version = vec![0u8, TAILS_BLOB_TAG_SZ];
    service.append(blob_handle, version.as_slice())?;

    while let Some(tail) = rtg.try_next()? {
        let tail_bytes = tail.to_bytes()?;
        service.append(blob_handle, tail_bytes.as_slice())?;
    }

    let res = service.finalize(blob_handle).map(|(location, hash)| (location, hash.to_base58()))?;

    debug!("store_tails_from_generator <<< res: {:?}", res);
    Ok(res)
}
