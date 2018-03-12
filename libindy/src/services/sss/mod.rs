// Use later when supporting multiple sharding schemes

/*
extern crate serde_json;

extern crate indy_crypto;
use self::indy_crypto::sss::{Share, shard_secret, recover_secret};

use errors::sss::SSSError;
use services::signus::SignusService;

pub mod types;
pub mod constants;

use std::rc::Rc;
use std::collections::HashMap;


pub struct SSSService {
    crypto_service: Rc<SignusService>
}

// Should it be singleton
impl SSSService {
    pub fn new(crypto_service: Rc<SignusService>) -> SSSService { SSSService { crypto_service } }

    pub fn shard_json(msg: &str, m: u8, n: u8, sign_shares: Option<bool>) -> Result<Vec<Share>, SSSError> {
        unimplemented!()
    }

    pub fn recover_secret(shards: Vec<Share>, verify_signatures: Option<bool>) -> Result<String, SSSError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_new_sss_service() -> SSSService {
        let crypto_service: Rc<SignusService> = Rc::new(SignusService::new());
        SSSService::new(crypto_service.clone())
    }

    #[test]
    fn test_shard_and_recover_from_json_msg() {
        let sss_service = get_new_sss_service();
        let msg = json!({
          "key1": "Value1",
          "key2": 1,
          "key3": [ "a", "b"],
          "secret": "some secret i need",
        }).to_string();
        let shards = SSSService::shard_json(&msg, 3, 5, None).unwrap();
        assert_eq!(shards.len(), 5);
        assert_eq!(SSSService::recover_secret(shards, None).unwrap(), msg);
    }
}*/
