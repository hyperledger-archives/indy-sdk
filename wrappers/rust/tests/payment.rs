extern crate indyrs as indy;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;

#[macro_use]
mod utils;
use utils::wallet::Wallet;

mod low_tests {
    use super::*;

    #[test]
    fn create_payment_address_works () {
        let _handle = Wallet::new();
    }
}
