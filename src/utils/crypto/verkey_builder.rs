extern crate rust_base58;

use self::rust_base58::base58::{FromBase58, FromBase58Error};

pub fn build_full_verkey(dest: &String, verkey: &Option<String>)
                         -> Result<Vec<u8>, FromBase58Error> {
    if let &Some(ref verkey) = verkey {
        if verkey.starts_with("~") {
            let mut result = dest.from_base58()?;
            let mut end = verkey[1..].from_base58()?;
            result.append(&mut end);
            return Ok(result);
        } else {
            return verkey.from_base58();
        }
    }
    dest.from_base58()
}
