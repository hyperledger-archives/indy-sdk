extern crate sharedlib;

use super::ErrorCode;
use self::sharedlib::{Lib, Func, Symbol};

use libc::c_char;
use std::ffi::CString;

use indy::ErrorCode;
use indy::wallet;
use indy::future::Future;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;


pub struct Wallet {}

impl Wallet {
    pub fn create_wallet(config: &str, credentials: &str) -> Result<(), ErrorCode> {
        wallet::create_wallet(config, credentials).wait()
    }

    pub fn open_wallet(config: &str, credentials: &str) -> Result<i32, ErrorCode> {
        wallet::open_wallet(config, credentials).wait()
    }

    pub fn delete_wallet(wallet_name: &str, credentials: &str) -> Result<(), ErrorCode> {
        wallet::delete_wallet(wallet_name, credentials).wait()
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        wallet::close_wallet(wallet_handle).wait()
    }

    pub fn export_wallet(wallet_handle: i32, export_config_json: &str) -> Result<(), ErrorCode> {
        wallet::export_wallet(wallet_handle, export_config_json).wait()
    }

    pub fn import_wallet(config: &str, credentials: &str, import_config_json: &str) -> Result<(), ErrorCode> {
        wallet::import_wallet(config, credentials, import_config_json).wait()
    }

    pub fn register_wallet_storage(stg_type: &str, library_path: &str, fn_pfx: &str) -> Result<(), ErrorCode> {
        println!("Loading {} {} {}", stg_type, library_path, fn_pfx);
        lazy_static! {
                static ref STG_REGISERED_WALLETS: Mutex<HashMap<String, Lib>> = Default::default();
            }

        let mut wallets = STG_REGISERED_WALLETS.lock().unwrap();

        if wallets.contains_key(stg_type) {
            // as registering of plugged wallet with
            // already registered so just return
            return Ok(());
        }

        let (receiver, command_handle, cb) = super::callbacks::_closure_to_cb_ec();

        let xxtype = CString::new(stg_type).unwrap();

        let err;
        let lib;
        let lib_path = Path::new(library_path);
        unsafe {
            println!("Loading {:?}", lib_path);
            lib = match Lib::new(lib_path) {
                Ok(rlib) => {
                    println!("Loaded lib");
                    rlib
                },
                Err(err) => {
                    println!("Load error {:?}", err);
                    panic!("Load error {:?}", err)
                }
            };

            println!("Get fn pointers ...");
            let fn_create_handler: Func<WalletCreate> = lib.find_func(&format!("{}create", fn_pfx)).unwrap();
            let fn_open_handler: Func<WalletOpen> = lib.find_func(&format!("{}open", fn_pfx)).unwrap();
            let fn_close_handler: Func<WalletClose> = lib.find_func(&format!("{}close", fn_pfx)).unwrap();
            let fn_delete_handler: Func<WalletDelete> = lib.find_func(&format!("{}delete", fn_pfx)).unwrap();
            let fn_add_record_handler: Func<WalletAddRecord> = lib.find_func(&format!("{}add_record", fn_pfx)).unwrap();
            let fn_update_record_value_handler: Func<WalletUpdateRecordValue> = lib.find_func(&format!("{}update_record_value", fn_pfx)).unwrap();
            let fn_update_record_tags_handler: Func<WalletUpdateRecordTags> = lib.find_func(&format!("{}update_record_tags", fn_pfx)).unwrap();
            let fn_add_record_tags_handler: Func<WalletAddRecordTags> = lib.find_func(&format!("{}add_record_tags", fn_pfx)).unwrap();
            let fn_delete_record_tags_handler: Func<WalletDeleteRecordTags> = lib.find_func(&format!("{}delete_record_tags", fn_pfx)).unwrap();
            let fn_delete_record_handler: Func<WalletDeleteRecord> = lib.find_func(&format!("{}delete_record", fn_pfx)).unwrap();
            let fn_get_record_handler: Func<WalletGetRecord> = lib.find_func(&format!("{}get_record", fn_pfx)).unwrap();
            let fn_get_record_id_handler: Func<WalletGetRecordId> = lib.find_func(&format!("{}get_record_id", fn_pfx)).unwrap();
            let fn_get_record_type_handler: Func<WalletGetRecordType> = lib.find_func(&format!("{}get_record_type", fn_pfx)).unwrap();
            let fn_get_record_value_handler: Func<WalletGetRecordValue> = lib.find_func(&format!("{}get_record_value", fn_pfx)).unwrap();
            let fn_get_record_tags_handler: Func<WalletGetRecordTags> = lib.find_func(&format!("{}get_record_tags", fn_pfx)).unwrap();
            let fn_free_record_handler: Func<WalletFreeRecord> = lib.find_func(&format!("{}free_record", fn_pfx)).unwrap();
            let fn_get_storage_metadata_handler: Func<WalletGetStorageMetadata> = lib.find_func(&format!("{}get_storage_metadata", fn_pfx)).unwrap();
            let fn_set_storage_metadata_handler: Func<WalletSetStorageMetadata> = lib.find_func(&format!("{}set_storage_metadata", fn_pfx)).unwrap();
            let fn_free_storage_metadata_handler: Func<WalletFreeStorageMetadata> = lib.find_func(&format!("{}free_storage_metadata", fn_pfx)).unwrap();
            let fn_search_records_handler: Func<WalletSearchRecords> = lib.find_func(&format!("{}search_records", fn_pfx)).unwrap();
            let fn_search_all_records_handler: Func<WalletSearchAllRecords> = lib.find_func(&format!("{}search_all_records", fn_pfx)).unwrap();
            let fn_get_search_total_count_handler: Func<WalletGetSearchTotalCount> = lib.find_func(&format!("{}get_search_total_count", fn_pfx)).unwrap();
            let fn_fetch_search_next_record_handler: Func<WalletFetchSearchNextRecord> = lib.find_func(&format!("{}fetch_search_next_record", fn_pfx)).unwrap();
            let fn_free_search_handler: Func<WalletFreeSearch> = lib.find_func(&format!("{}free_search", fn_pfx)).unwrap();

            println!("Register wallet ...");
            err = indy_register_wallet_storage(
                command_handle,
                xxtype.as_ptr(),
                Some(fn_create_handler.get()),
                Some(fn_open_handler.get()),
                Some(fn_close_handler.get()),
                Some(fn_delete_handler.get()),
                Some(fn_add_record_handler.get()),
                Some(fn_update_record_value_handler.get()),
                Some(fn_update_record_tags_handler.get()),
                Some(fn_add_record_tags_handler.get()),
                Some(fn_delete_record_tags_handler.get()),
                Some(fn_delete_record_handler.get()),
                Some(fn_get_record_handler.get()),
                Some(fn_get_record_id_handler.get()),
                Some(fn_get_record_type_handler.get()),
                Some(fn_get_record_value_handler.get()),
                Some(fn_get_record_tags_handler.get()),
                Some(fn_free_record_handler.get()),
                Some(fn_get_storage_metadata_handler.get()),
                Some(fn_set_storage_metadata_handler.get()),
                Some(fn_free_storage_metadata_handler.get()),
                Some(fn_search_records_handler.get()),
                Some(fn_search_all_records_handler.get()),
                Some(fn_get_search_total_count_handler.get()),
                Some(fn_fetch_search_next_record_handler.get()),
                Some(fn_free_search_handler.get()),
                cb
            );
        }

        println!("Finish up ...");
        wallets.insert(stg_type.to_string(), lib);

        super::results::result_to_empty(err, receiver)
    }
}

