#![cfg_attr(feature = "fatal_warnings", deny(warnings))]

extern crate base64;

#[macro_use]
extern crate log;

extern crate serde;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

// Note that to use macroses from indy_common::util inside of other modules it must me loaded first!
extern crate indy_crypto;
extern crate libc;
extern crate rand;
extern crate postgres;

mod libindy;

// Note that to use macroses from util inside of other modules it must me loaded first!
#[macro_use]
pub mod utils;
pub mod errors;
pub mod api;
pub mod wallet_storage;

pub mod postgres_wallet;
pub mod postgres_storage;

use postgres_wallet::PostgresWallet;

use std::ffi::CString;


#[no_mangle]
pub extern fn postgresstorage_init() -> api::ErrorCode {
    //if let Err(err) = utils::logger::LibnullpayLogger::init() {
    //    return err;
    //}

    let postgres_storage_name = CString::new(postgres_wallet::POSTGRES_STORAGE_NAME).unwrap();

    libindy::wallet::register_wallet_storage(
        postgres_storage_name.as_ptr(),
        PostgresWallet::postgreswallet_fn_create,
        PostgresWallet::postgreswallet_fn_open,
        PostgresWallet::postgreswallet_fn_close,
        PostgresWallet::postgreswallet_fn_delete,
        PostgresWallet::postgreswallet_fn_add_record,
        PostgresWallet::postgreswallet_fn_update_record_value,
        PostgresWallet::postgreswallet_fn_update_record_tags,
        PostgresWallet::postgreswallet_fn_add_record_tags,
        PostgresWallet::postgreswallet_fn_delete_record_tags,
        PostgresWallet::postgreswallet_fn_delete_record,
        PostgresWallet::postgreswallet_fn_get_record,
        PostgresWallet::postgreswallet_fn_get_record_id,
        PostgresWallet::postgreswallet_fn_get_record_type,
        PostgresWallet::postgreswallet_fn_get_record_value,
        PostgresWallet::postgreswallet_fn_get_record_tags,
        PostgresWallet::postgreswallet_fn_free_record,
        PostgresWallet::postgreswallet_fn_get_storage_metadata,
        PostgresWallet::postgreswallet_fn_set_storage_metadata,
        PostgresWallet::postgreswallet_fn_free_storage_metadata,
        PostgresWallet::postgreswallet_fn_search_records,
        PostgresWallet::postgreswallet_fn_search_all_records,
        PostgresWallet::postgreswallet_fn_get_search_total_count,
        PostgresWallet::postgreswallet_fn_fetch_search_next_record,
        PostgresWallet::postgreswallet_fn_free_search,
    )
}

