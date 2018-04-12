extern crate indy;

// Workaround to share some utils code based on indy sdk types between tests and indy sdk
use indy::api as api;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod utils;

use utils::inmem_wallet::InmemWallet;
use utils::wallet::WalletUtils;
use utils::test::TestUtils;
use utils::constants::*;

use indy::api::ErrorCode;

pub const CONFIG: &'static str = r#"{"freshness_time":1000}"#;
pub const CREDENTIALS: &'static str = r#"{"key":"testkey"}"#;

mod high_cases {
    use super::*;

    mod register_wallet_type {
        use super::*;

        #[test]
        fn indy_register_wallet_type_works() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_type(INMEM_TYPE, false).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, Some(TYPE), None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_type(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_create_wallet_works_for_unknown_type() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::create_wallet(POOL, WALLET, Some("unknown_type"), None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletUnknownTypeError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_type() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_config() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, Some(TYPE), Some(CONFIG), None).unwrap();

            TestUtils::cleanup_storage();
        }


        #[test]
        fn indy_create_wallet_works_for_credentials() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, Some(TYPE), None, Some(CREDENTIALS)).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            WalletUtils::delete_wallet(WALLET).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_closed() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            WalletUtils::delete_wallet(WALLET).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        #[ignore] //TODO FUX BUG. We can delete only closed wallet
        fn indy_delete_wallet_works_for_opened() {
            TestUtils::cleanup_storage();

            let wallet_handle = WalletUtils::create_and_open_wallet(POOL, None).unwrap();
            let res = WalletUtils::delete_wallet(WALLET);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_type(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();
            WalletUtils::delete_wallet(WALLET).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works() {
            TestUtils::cleanup_storage();

            let wallet_name = "indy_open_wallet_works";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, None).unwrap();
            WalletUtils::open_wallet(wallet_name, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let wallet_name = "indy_open_wallet_works_for_plugged";

            WalletUtils::register_wallet_type(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, wallet_name, Some(INMEM_TYPE), None, None).unwrap();
            WalletUtils::open_wallet(wallet_name, None, None).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_open_wallet_works_for_config() {
            TestUtils::cleanup_storage();

            let wallet_name = "indy_open_wallet_works_for_config";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, None).unwrap();
            WalletUtils::open_wallet(wallet_name, Some(CONFIG), None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_encrypted_wallet_with_correct_credentials() {
            TestUtils::cleanup_storage();

            let wallet_name = "indy_open_wallet_works_for_encrypted_wallet_with_correct_credentials";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, Some(CREDENTIALS)).unwrap();
            WalletUtils::open_wallet(wallet_name, None, Some(CREDENTIALS)).unwrap();

            TestUtils::cleanup_storage();
        }
    }

    mod close_wallet {
        use super::*;

        #[test]
        fn indy_close_wallet_works() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_close_wallet_works_for_plugged() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_type(INMEM_TYPE, false).unwrap();
            WalletUtils::create_wallet(POOL, WALLET, Some(INMEM_TYPE), None, None).unwrap();

            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();
            let wallet_handle = WalletUtils::open_wallet(WALLET, None, None).unwrap();
            WalletUtils::close_wallet(wallet_handle).unwrap();

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }
}

mod medium_cases {
    extern crate libc;

    use super::*;
    use std::ffi::CString;
    use self::libc::c_char;

    mod register_wallet_type {
        use super::*;
        use indy::api::wallet::indy_register_wallet_type;

        #[test]
        fn indy_register_wallet_type_does_not_work_twice_with_same_name() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            WalletUtils::register_wallet_type(INMEM_TYPE, false).unwrap();
            let res = WalletUtils::register_wallet_type(INMEM_TYPE, true);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletTypeAlreadyRegisteredError);

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }

        #[test]
        fn indy_register_wallet_type_does_not_work_with_null_params() {
            TestUtils::cleanup_storage();
            InmemWallet::cleanup();

            let xtype = CString::new(INMEM_TYPE).unwrap();
            let res = indy_register_wallet_type(1, xtype.as_ptr(), None, None, None, None, None,
                                                None, None, None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam3);

            extern "C" fn callback(_: *const c_char, _: *const c_char,
                                   _: *const c_char) -> ErrorCode {
                ErrorCode::Success
            }

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), None, None, None,
                                                None, None, None, None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam4);

            extern "C" fn callback1(_: *const c_char, _: *const c_char, _: *const c_char,
                                    _: *const c_char, _: *mut i32) -> ErrorCode {
                ErrorCode::Success
            }

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                None, None, None, None, None, None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam5);

            extern "C" fn callback2(_: i32, _: *const c_char, _: *const c_char) -> ErrorCode {
                ErrorCode::Success
            }

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                Some(callback2), None, None, None, None, None,
                                                None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam6);

            extern "C" fn callback3(_: i32, _: *const c_char, _: *mut *const c_char) -> ErrorCode {
                ErrorCode::Success
            }

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                Some(callback2), Some(callback3), None, None, None,
                                                None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam7);

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                Some(callback2), Some(callback3), Some(callback3),
                                                None, None, None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam8);

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                Some(callback2), Some(callback3), Some(callback3),
                                                Some(callback3), None, None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam9);

            extern "C" fn callback4(_: i32) -> ErrorCode {
                ErrorCode::Success
            }

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                Some(callback2), Some(callback3), Some(callback3),
                                                Some(callback3), Some(callback4), None, None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam10);

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                Some(callback2), Some(callback3), Some(callback3),
                                                Some(callback3), Some(callback4), Some(callback),
                                                None, None);
            assert_eq!(res, ErrorCode::CommonInvalidParam11);

            extern "C" fn callback5(_: i32, _: *const c_char) -> ErrorCode {
                ErrorCode::Success
            }

            let res = indy_register_wallet_type(1, xtype.as_ptr(), Some(callback), Some(callback1),
                                                Some(callback2), Some(callback3), Some(callback3),
                                                Some(callback3), Some(callback4), Some(callback),
                                                Some(callback5), None);
            assert_eq!(res, ErrorCode::CommonInvalidParam12);

            TestUtils::cleanup_storage();
            InmemWallet::cleanup();
        }
    }

    mod create_wallet {
        use super::*;

        #[test]
        fn indy_create_wallet_works_for_duplicate_name() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let res = WalletUtils::create_wallet(POOL, WALLET, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyExistsError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_create_wallet_works_for_empty_name() {
            TestUtils::cleanup_storage();

            let wallet_name = "";
            let res = WalletUtils::create_wallet(POOL, wallet_name, None, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonInvalidParam3);

            TestUtils::cleanup_storage();
        }
    }

    mod delete_wallet {
        use super::*;

        #[test]
        fn indy_delete_wallet_works_for_not_created() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::delete_wallet(WALLET);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_delete_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            WalletUtils::delete_wallet(WALLET).unwrap();
            let res = WalletUtils::delete_wallet(WALLET);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }
    }

    mod open_wallet {
        use super::*;

        #[test]
        fn indy_open_wallet_works_for_not_created_wallet() {
            TestUtils::cleanup_storage();

            let res = WalletUtils::open_wallet(WALLET, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::CommonIOError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_twice() {
            TestUtils::cleanup_storage();

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();

            WalletUtils::open_wallet(WALLET, None, None).unwrap();
            let res = WalletUtils::open_wallet(WALLET, None, None);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletAlreadyOpenedError);

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_two_wallets() {
            TestUtils::cleanup_storage();

            let wallet_name_1 = "indy_open_wallet_works_for_two_wallets1";
            let wallet_name_2 = "indy_open_wallet_works_for_two_wallets2";

            WalletUtils::create_wallet(POOL, wallet_name_1, None, None, None).unwrap();
            WalletUtils::create_wallet(POOL, wallet_name_2, None, None, None).unwrap();
            WalletUtils::open_wallet(wallet_name_1, None, None).unwrap();
            WalletUtils::open_wallet(wallet_name_2, None, None).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_encrypted_wallet_with_invalid_credentials() {
            TestUtils::cleanup_storage();

            let wallet_name = "indy_open_wallet_works_for_encrypted_wallet_with_invalid_credentials";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, Some(CREDENTIALS)).unwrap();
            let res = WalletUtils::open_wallet(wallet_name, None, Some(r#"{"key":"otherkey"}"#));
            assert_eq!(ErrorCode::WalletAccessFailed, res.unwrap_err());

            TestUtils::cleanup_storage();
        }


        #[test]
        fn indy_open_wallet_works_for_changing_credentials() {
            TestUtils::cleanup_storage();

            let wallet_name = "indy_open_wallet_works_for_encrypted_wallet_with_changing_credentials";
            WalletUtils::create_wallet(POOL, wallet_name, None, None, Some(CREDENTIALS)).unwrap();
            WalletUtils::open_wallet(wallet_name, None, Some(r#"{"key":"testkey", "rekey":"newkey"}"#)).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        fn indy_open_wallet_works_for_invalid_config() {
            TestUtils::cleanup_storage();

            let config = r#"{"field":"value"}"#;

            WalletUtils::create_wallet(POOL, WALLET, None, None, None).unwrap();
            let res = WalletUtils::open_wallet(WALLET, Some(config), None);
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

            let wallet_handle = WalletUtils::create_and_open_wallet(WALLET, None).unwrap();

            WalletUtils::close_wallet(wallet_handle).unwrap();
            let res = WalletUtils::close_wallet(wallet_handle);
            assert_eq!(res.unwrap_err(), ErrorCode::WalletInvalidHandle);

            TestUtils::cleanup_storage();
        }
    }
}
