extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

extern crate rust_base58;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate log;

#[macro_use]
mod utils;

use utils::wallet::WalletUtils;
use utils::authz::AuthzUtils;
use utils::test::TestUtils;
use utils::pool::PoolUtils;
use utils::ledger::LedgerUtils;
use utils::constants::*;
use utils::types::ResponseType;

use indy::api::ErrorCode;

#[cfg(feature = "local_nodes_pool")]
use std::thread;

pub const ENCRYPTED_MESSAGE: &'static [u8; 45] = &[187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32];
pub const SIGNATURE: &'static [u8; 64] = &[20, 191, 100, 213, 101, 12, 197, 198, 203, 49, 89, 220, 205, 192, 224, 221, 97, 77, 220, 190, 90, 60, 142, 23, 16, 240, 189, 129, 45, 148, 245, 8, 102, 95, 95, 249, 100, 89, 41, 227, 213, 25, 100, 1, 232, 188, 245, 235, 186, 21, 52, 176, 236, 11, 99, 70, 155, 159, 89, 215, 197, 239, 138, 5];

mod high_cases {
    use super::*;

    mod creation_deletion_policy_address {
        use super::*;

        /*#[test]
        fn indy_creation_deletion_policy_address_works_for_creation() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();

            let address = AuthzUtils::create_and_store_my_did(wallet_handle, Some(MY1_SEED)).unwrap();

            let received_verkey = DidUtils::key_for_did(-1, wallet_handle, &did).unwrap();
            assert_eq!(verkey, received_verkey);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }*/
    }
}