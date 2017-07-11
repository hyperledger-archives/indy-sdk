extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[macro_use]
mod utils;

use utils::inmem_wallet::InmemWallet;
use utils::wallet::WalletUtils;
use utils::signus::SignusUtils;
use utils::test::TestUtils;

use indy::api::ErrorCode;

mod high_cases {
    use super::*;

    mod register_wallet_type {
        use super::*;

        #[test]
        fn indy_register_wallet_type_works() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_type("inmem").unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_create_wallet_works";
            let wallet_name = "indy_create_wallet_works";
            let xtype = "default";

            WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let pool_name = "indy_create_wallet_works";
            let wallet_name = "indy_create_wallet_works";
            let xtype = "inmem";

            WalletUtils::register_wallet_type("inmem").unwrap();
            WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_create_wallet_works_for_unknown_type() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_create_wallet_works_for_unknown_type";
            let wallet_name = "indy_create_wallet_works_for_unknown_type";
            let xtype = "type";

            let res = WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletUnknownTypeError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_type() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_create_wallet_works_for_empty_type";
            let wallet_name = "indy_create_wallet_works_for_empty_type";

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_config() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_create_wallet_works";
            let wallet_name = "indy_create_wallet_works";
            let xtype = "default";
            let config = r#"{"freshness_time":1000}"#;

            WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), Some(config)).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_delete_wallet_works";
            let wallet_name = "indy_delete_wallet_works";

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();
            WalletUtils::delete_wallet(wallet_name).unwrap();
            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore] //There is bug
        fn sovrin_delete_wallet_works_for_opened() {
            TestUtils::cleanup_storage();

            let pool_name = "sovrin_delete_wallet_works_for_opened";
            let wallet_name = "wallet1";

            WalletUtils::create_and_open_wallet(pool_name, wallet_name, "default").unwrap();
            let res = WalletUtils::delete_wallet(wallet_name);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn sovrin_delete_wallet_works_for_closed() {
            TestUtils::cleanup_storage();

            let pool_name = "sovrin_delete_wallet_works_for_closed";
            let wallet_name = "wallet1";

            let wallet_handle = WalletUtils::create_and_open_wallet(pool_name, wallet_name, "default").unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::delete_wallet(wallet_name).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let pool_name = "indy_delete_wallet_works_for_plugged";
            let wallet_name = "indy_delete_wallet_works_for_plugged";
            let xtype = "inmem";

            WalletUtils::register_wallet_type(xtype).unwrap();
            WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), None).unwrap();
            WalletUtils::delete_wallet(wallet_name).unwrap();
            WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_open_wallet_works";
            let wallet_name = "indy_open_wallet_works";

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();
            WalletUtils::open_wallet(wallet_name, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let pool_name = "indy_open_wallet_works_for_plugged";
            let wallet_name = "indy_open_wallet_works_for_plugged";
            let xtype = "inmem";

            WalletUtils::register_wallet_type(xtype).unwrap();
            WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), None).unwrap();
            WalletUtils::open_wallet(wallet_name, None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_open_wallet_works_for_config() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_open_wallet_works_for_config";
            let wallet_name = "indy_open_wallet_works_for_config";
            let config = r#"{"freshness_time":1000}"#;

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();
            WalletUtils::open_wallet(wallet_name, Some(config)).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_close_wallet_works";
            let wallet_name = "indy_close_wallet_works";

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();

            let wallet_handle = WalletUtils::open_wallet(wallet_name, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::open_wallet(wallet_name, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_close_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let pool_name = "indy_close_wallet_works_for_plugged";
            let wallet_name = "indy_close_wallet_works_for_plugged";
            let xtype = "inmem";

            WalletUtils::register_wallet_type(xtype).unwrap();
            WalletUtils::create_wallet(pool_name, wallet_name, Some(xtype), None).unwrap();

            let wallet_handle = WalletUtils::open_wallet(wallet_name, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::open_wallet(wallet_name, None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod set_seqno_wallet {
        use super::*;

        #[test]
        fn indy_wallet_set_seqno_works() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("indy_wallet_set_seqno_works", None).unwrap();

            let (did, _, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            WalletUtils::wallet_set_seq_no_for_value(wallet_handle, &did, 1).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_wallet_set_seqno_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let xtype = "inmem";

            WalletUtils::register_wallet_type(xtype).unwrap();
            let wallet_handle = WalletUtils::create_and_open_wallet("indy_wallet_set_seqno_works_for_plugged", Some(xtype)).unwrap();

            let (did, _, _) = SignusUtils::create_my_did(wallet_handle, "{}").unwrap();

            WalletUtils::wallet_set_seq_no_for_value(wallet_handle, &did, 1).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }
}

mod medium_cases {
    use super::*;

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works_for_duplicate_name() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_create_wallet_works_for_duplicate_name";
            let wallet_name = "indy_create_wallet_works_for_duplicate_name";

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();
            let res = WalletUtils::create_wallet(pool_name, wallet_name, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyExistsError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_name() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_create_wallet_works_for_empty_name";
            let wallet_name = "";

            let res = WalletUtils::create_wallet(pool_name, wallet_name, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidParam3);

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works_for_invalid_wallet_name() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::delete_wallet("indy_delete_wallet_works_for_invalid_wallet_name");
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_delete_wallet_works_for_deleted_wallet";
            let wallet_name = "indy_delete_wallet_works_for_deleted_wallet";

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();
            WalletUtils::delete_wallet(wallet_name).unwrap();
            let res = WalletUtils::delete_wallet(wallet_name);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works_for_not_created_wallet() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::open_wallet("indy_open_wallet_works_for_not_created_wallet", None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore] //TODO Check is not implemented
        fn indy_open_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_create_wallet_works";
            let wallet_name = "indy_open_wallet_works_for_twice";

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();

            WalletUtils::open_wallet(wallet_name, None).unwrap();
            let res = WalletUtils::open_wallet(wallet_name, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_open_wallet_works_for_two_wallets";
            let wallet_name_1 = "indy_open_wallet_works_for_two_wallets1";
            let wallet_name_2 = "indy_open_wallet_works_for_two_wallets2";

            WalletUtils::create_wallet(pool_name, wallet_name_1, None, None).unwrap();
            WalletUtils::create_wallet(pool_name, wallet_name_2, None, None).unwrap();
            WalletUtils::open_wallet(wallet_name_1, None).unwrap();
            WalletUtils::open_wallet(wallet_name_2, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_config() {
            TestUtils::cleanup_storage();

            let pool_name = "indy_open_wallet_works_for_invalid_config";
            let wallet_name = "indy_open_wallet_works_for_invalid_config";
            let config = r#"{"field":"value"}"#;

            WalletUtils::create_wallet(pool_name, wallet_name, None, None).unwrap();
            let res = WalletUtils::open_wallet(wallet_name, Some(config));
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidStructure);

            TestUtils::cleanup_storage();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works_for_invalid_handle() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::close_wallet(1);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_close_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("indy_close_wallet_works_for_twice", None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            let res = WalletUtils::close_wallet(wallet_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }

    mod set_seqno {
        use super::*;

        #[test]
        fn indy_wallet_set_seqno_works_for_not_exists_key() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("indy_wallet_set_seqno_works_for_not_exists_key", None).unwrap();

            //TODO may be we must return WalletNotFound in case if key not exists in wallet
            WalletUtils::wallet_set_seq_no_for_value(wallet_handle, "key", 1).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_wallet_set_seqno_works_for_invalid_wallet() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet("indy_wallet_set_seqno_works_for_invalid_wallet", None).unwrap();


            let invalid_wallet_handle = wallet_handle + 1;
            let res = WalletUtils::wallet_set_seq_no_for_value(invalid_wallet_handle, "key", 1);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }
}
