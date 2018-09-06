extern crate rust_libindy_wrapper as indy;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde;
extern crate byteorder;
#[macro_use]
mod utils;

use indy::wallet::Wallet;
use utils::constants::DEFAULT_CREDENTIALS;

mod low_tests {
    use super::*;

    #[test]
    fn create_payment_address_works () {
        let wallet_name = r#"{"id":"create_payment_address_works"}"#;
        safe_wallet_create!(wallet_name);
        let handle = Wallet::open(wallet_name, DEFAULT_CREDENTIALS).unwrap();

        wallet_cleanup!(handle, wallet_name);
    }
}
