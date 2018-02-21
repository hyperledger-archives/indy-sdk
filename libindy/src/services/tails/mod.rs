pub mod default_writer;

extern crate digest;
extern crate indy_crypto;
extern crate sha2;

use self::digest::{FixedOutput, Input};
use self::indy_crypto::cl::RevocationTailsGenerator;

use errors::common::CommonError;

use std::cell::RefCell;
use std::collections::HashMap;


trait TailsWriterType {
    fn create(&self, config: &str) -> Box<TailsWriter>;
}

trait TailsWriter {
    fn append(&mut self, bytes: &[u8]) -> ();
    fn finalize(&mut self, hash: &[u8]) -> String;
}

struct TailsWriterService {
    types: RefCell<HashMap<String, Box<TailsWriterType>>>
}

impl TailsWriterService {
    pub fn new() -> TailsWriterService {
        let mut types: HashMap<String, Box<TailsWriterType>> = HashMap::new();
        types.insert("default".to_owned(), Box::new(default_writer::DefaultTailsWriterType::new()));

        TailsWriterService {
            types: RefCell::new(types)
        }
    }

    pub fn store_tails_from_generator(&self,
                                      type_: &str,
                                      config: &str,
                                      rtg: &mut RevocationTailsGenerator)
                                      -> Result<(), CommonError> {
        let mut tails_writer = self.types.borrow_mut().get(type_).unwrap().create(config);
        let mut hasher = sha2::Sha256::default();

        //FIXME store version/tag/meta at start of the Tail's BLOB

        while let Some(tail) = rtg.next()? {
            let tail_bytes = tail.to_bytes()?;
            hasher.process(tail_bytes.as_slice());
            tails_writer.append(tail_bytes.as_slice())
        }

        tails_writer.finalize(hasher.fixed_result().as_slice());

        Ok(())
    }
}
