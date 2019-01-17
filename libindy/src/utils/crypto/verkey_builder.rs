use errors::prelude::*;
use utils::crypto::base58;


pub fn build_full_verkey(dest: &str, verkey: Option<&str>) -> Result<String, IndyError> {
    if let Some(verkey) = verkey {
        let (verkey, crypto_type) = if verkey.contains(':') {
            let splits: Vec<&str> = verkey.split(':').collect();
            (splits[0], Some(splits[1]))
        } else {
            (verkey, None)
        };

        let verkey = if verkey.starts_with('~') {
            let mut result = base58::decode(dest)?;
            let mut end = base58::decode(&verkey[1..])?;
            result.append(&mut end);
            base58::encode(&result)
        } else {
            verkey.to_owned()
        };

        let verkey = if let Some(crypto_type) = crypto_type {
            format!("{}:{}", verkey, crypto_type)
        } else {
            verkey
        };

        Ok(verkey)
    } else {
        // Cryptonym
        Ok(dest.to_owned())
    }
}
