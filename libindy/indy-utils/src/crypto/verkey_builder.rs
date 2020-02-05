use indy_api_types::errors::prelude::*;
use rust_base58::{FromBase58, ToBase58};


pub fn build_full_verkey(dest: &str, verkey: Option<&str>) -> Result<String, IndyError> {
    if let Some(verkey) = verkey {
        let verkey = if verkey.starts_with('~') {
            let mut result = dest.from_base58()?;
            let mut end = verkey[1..].from_base58()?;
            result.append(&mut end );
            result.to_base58()
        } else {
            verkey.to_owned()
        };

        Ok(verkey)
    } else {
        // Cryptonym
        Ok(dest.to_owned())
    }
}