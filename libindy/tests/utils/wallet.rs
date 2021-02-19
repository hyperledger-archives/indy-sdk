use std::{
    collections::HashSet,
    ffi::CString,
    path::{Path, PathBuf},
    sync::Mutex,
};

use indyrs::{future::Future, wallet, CommandHandle, ErrorCode, IndyError, WalletHandle};
use lazy_static::lazy_static;
use libc::c_char;
use serde_json;

use crate::utils::{
    callback,
    constants::{INMEM_TYPE, TYPE, WALLET_CREDENTIALS},
    environment,
    inmem_wallet::InmemWallet,
    sequence,
};

pub fn register_wallet_storage(xtype: &str, force_create: bool) -> Result<(), ErrorCode> {
    lazy_static! {
        static ref REGISERED_WALLETS: Mutex<HashSet<String>> = Default::default();
    }

    let mut wallets = REGISERED_WALLETS.lock().unwrap();

    if wallets.contains(xtype) & !force_create {
        // as registering of plugged wallet with
        return Ok(());
    }

    let (receiver, command_handle, cb) = callback::_closure_to_cb_ec();

    let xxtype = CString::new(xtype).unwrap();

    let err = unsafe {
        indy_register_wallet_storage(
            command_handle,
            xxtype.as_ptr(),
            Some(InmemWallet::create),
            Some(InmemWallet::open),
            Some(InmemWallet::close),
            Some(InmemWallet::delete),
            Some(InmemWallet::add_record),
            Some(InmemWallet::update_record_value),
            Some(InmemWallet::update_record_tags),
            Some(InmemWallet::add_record_tags),
            Some(InmemWallet::delete_record_tags),
            Some(InmemWallet::delete_record),
            Some(InmemWallet::get_record),
            Some(InmemWallet::get_record_id),
            Some(InmemWallet::get_record_type),
            Some(InmemWallet::get_record_value),
            Some(InmemWallet::get_record_tags),
            Some(InmemWallet::free_record),
            Some(InmemWallet::get_storage_metadata),
            Some(InmemWallet::set_storage_metadata),
            Some(InmemWallet::free_storage_metadata),
            Some(InmemWallet::search_records),
            Some(InmemWallet::search_all_records),
            Some(InmemWallet::get_search_total_count),
            Some(InmemWallet::fetch_search_next_record),
            Some(InmemWallet::free_search),
            cb,
        )
    };

    wallets.insert(xtype.to_string());

    super::results::result_to_empty(err as i32, receiver)
}

pub fn create_wallet(config: &str, credentials: &str) -> Result<(), IndyError> {
    wallet::create_wallet(config, credentials).wait()
}

pub fn open_wallet(config: &str, credentials: &str) -> Result<WalletHandle, IndyError> {
    wallet::open_wallet(config, credentials).wait()
}

pub fn create_and_open_default_wallet(
    wallet_name: &str,
) -> Result<(WalletHandle, String), IndyError> {
    let config = json!({
        "id": format!("default-wallet_id-{}-{}", wallet_name, sequence::get_next_id()),
        "storage_type": TYPE
    })
    .to_string();

    create_wallet(&config, WALLET_CREDENTIALS)?;
    let wallet_handle = open_wallet(&config, WALLET_CREDENTIALS).unwrap();
    Ok((wallet_handle, config))
}

pub fn create_and_open_mysql_wallet(
    wallet_name: &str,
) -> Result<(WalletHandle, String, String), IndyError> {
    let storage_config = json!({
        "read_host": "127.0.0.1",
        "write_host": "127.0.0.1",
        "port": 3306,
        "db_name": "indy"
    });

    let storage_creds = json!({
        "user": "root",
        "pass": "pass@word1"
    });

    let wallet_config = json!({
        "id": format!("default-wallet_id-{}-{}", wallet_name, sequence::get_next_id()),
        "storage_type": "mysql",
        "storage_config": storage_config
    })
    .to_string();

    let wallet_credentials = json!({
        "key": "8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY",
        "key_derivation_method": "RAW",
        "storage_credentials": storage_creds
    })
    .to_string();

    create_wallet(&wallet_config, &wallet_credentials)?;
    let wallet_handle = open_wallet(&wallet_config, &wallet_credentials).unwrap();

    Ok((wallet_handle, wallet_config, wallet_credentials))
}

pub fn create_and_open_plugged_wallet() -> Result<(WalletHandle, String), IndyError> {
    let config = json!({
        "id": format!("default-wallet_id-{}", sequence::get_next_id()),
        "storage_type": INMEM_TYPE
    })
    .to_string();

    register_wallet_storage("inmem", false).unwrap();
    create_wallet(&config, WALLET_CREDENTIALS)?;
    let wallet_handle = open_wallet(&config, WALLET_CREDENTIALS).unwrap();

    Ok((wallet_handle, config))
}

pub fn delete_wallet(config: &str, credentials: &str) -> Result<(), IndyError> {
    wallet::delete_wallet(config, credentials).wait()
}

pub fn close_wallet(wallet_handle: WalletHandle) -> Result<(), IndyError> {
    wallet::close_wallet(wallet_handle).wait()
}

pub fn close_and_delete_wallet(
    wallet_handle: WalletHandle,
    wallet_config: &str,
) -> Result<(), IndyError> {
    close_wallet(wallet_handle)?;
    delete_wallet(wallet_config, WALLET_CREDENTIALS)
}

pub fn export_wallet(
    wallet_handle: WalletHandle,
    export_config_json: &str,
) -> Result<(), IndyError> {
    wallet::export_wallet(wallet_handle, export_config_json).wait()
}

pub fn import_wallet(
    config: &str,
    credentials: &str,
    import_config: &str,
) -> Result<(), IndyError> {
    wallet::import_wallet(config, credentials, import_config).wait()
}

pub fn export_wallet_path(name: &str) -> PathBuf {
    environment::tmp_file_path(name)
}

pub fn prepare_export_wallet_config(path: &Path) -> String {
    let json = json!({
        "path": path.to_str().unwrap(),
        "key": "export_key",
    });
    serde_json::to_string(&json).unwrap()
}

pub fn generate_wallet_key(config: Option<&str>) -> Result<String, IndyError> {
    wallet::generate_wallet_key(config).wait()
}

extern "C" {
    pub fn indy_register_wallet_storage(
        command_handle: CommandHandle,
        type_: *const c_char,
        create: Option<WalletCreate>,
        open: Option<WalletOpen>,
        close: Option<WalletClose>,
        delete: Option<WalletDelete>,
        add_record: Option<WalletAddRecord>,
        update_record_value: Option<WalletUpdateRecordValue>,
        update_record_tags: Option<WalletUpdateRecordTags>,
        add_record_tags: Option<WalletAddRecordTags>,
        delete_record_tags: Option<WalletDeleteRecordTags>,
        delete_record: Option<WalletDeleteRecord>,
        get_record: Option<WalletGetRecord>,
        get_record_id: Option<WalletGetRecordId>,
        get_record_type: Option<WalletGetRecordType>,
        get_record_value: Option<WalletGetRecordValue>,
        get_record_tags: Option<WalletGetRecordTags>,
        free_record: Option<WalletFreeRecord>,
        get_storage_metadata: Option<WalletGetStorageMetadata>,
        set_storage_metadata: Option<WalletSetStorageMetadata>,
        free_storage_metadata: Option<WalletFreeStorageMetadata>,
        search_records: Option<WalletSearchRecords>,
        search_all_records: Option<WalletSearchAllRecords>,
        get_search_total_count: Option<WalletGetSearchTotalCount>,
        fetch_search_next_record: Option<WalletFetchSearchNextRecord>,
        free_search: Option<WalletFreeSearch>,
        cb: Option<ResponseEmptyCB>,
    ) -> ErrorCode;
}

pub type WalletCreate = extern "C" fn(
    name: *const c_char,
    config: *const c_char,
    credentials_json: *const c_char,
    metadata: *const c_char,
) -> ErrorCode;

pub type WalletOpen = extern "C" fn(
    name: *const c_char,
    config: *const c_char,
    credentials_json: *const c_char,
    storage_handle_p: *mut i32,
) -> ErrorCode;

pub type WalletClose = extern "C" fn(storage_handle: i32) -> ErrorCode;

pub type WalletDelete = extern "C" fn(
    name: *const c_char,
    config: *const c_char,
    credentials_json: *const c_char,
) -> ErrorCode;

pub type WalletAddRecord = extern "C" fn(
    storage_handle: i32,
    type_: *const c_char,
    id: *const c_char,
    value: *const u8,
    value_len: usize,
    tags_json: *const c_char,
) -> ErrorCode;

pub type WalletUpdateRecordValue = extern "C" fn(
    storage_handle: i32,
    type_: *const c_char,
    id: *const c_char,
    value: *const u8,
    value_len: usize,
) -> ErrorCode;

pub type WalletUpdateRecordTags = extern "C" fn(
    storage_handle: i32,
    type_: *const c_char,
    id: *const c_char,
    tags_json: *const c_char,
) -> ErrorCode;

pub type WalletAddRecordTags = extern "C" fn(
    storage_handle: i32,
    type_: *const c_char,
    id: *const c_char,
    tags_json: *const c_char,
) -> ErrorCode;

pub type WalletDeleteRecordTags = extern "C" fn(
    storage_handle: i32,
    type_: *const c_char,
    id: *const c_char,
    tag_names_json: *const c_char,
) -> ErrorCode;

pub type WalletDeleteRecord =
    extern "C" fn(storage_handle: i32, type_: *const c_char, id: *const c_char) -> ErrorCode;

pub type WalletGetRecord = extern "C" fn(
    storage_handle: i32,
    type_: *const c_char,
    id: *const c_char,
    options_json: *const c_char,
    record_handle_p: *mut i32,
) -> ErrorCode;

pub type WalletGetRecordId = extern "C" fn(
    storage_handle: i32,
    record_handle: i32,
    record_id_p: *mut *const c_char,
) -> ErrorCode;

pub type WalletGetRecordType = extern "C" fn(
    storage_handle: i32,
    record_handle: i32,
    record_type_p: *mut *const c_char,
) -> ErrorCode;

pub type WalletGetRecordValue = extern "C" fn(
    storage_handle: i32,
    record_handle: i32,
    record_value_p: *mut *const u8,
    record_value_len_p: *mut usize,
) -> ErrorCode;

pub type WalletGetRecordTags = extern "C" fn(
    storage_handle: i32,
    record_handle: i32,
    record_tags_p: *mut *const c_char,
) -> ErrorCode;

pub type WalletFreeRecord = extern "C" fn(storage_handle: i32, record_handle: i32) -> ErrorCode;

pub type WalletGetStorageMetadata = extern "C" fn(
    storage_handle: i32,
    metadata_p: *mut *const c_char,
    metadata_handle: *mut i32,
) -> ErrorCode;

pub type WalletSetStorageMetadata =
    extern "C" fn(storage_handle: i32, metadata_p: *const c_char) -> ErrorCode;

pub type WalletFreeStorageMetadata =
    extern "C" fn(storage_handle: i32, metadata_handle: i32) -> ErrorCode;

pub type WalletSearchRecords = extern "C" fn(
    storage_handle: i32,
    type_: *const c_char,
    query_json: *const c_char,
    options_json: *const c_char,
    search_handle_p: *mut i32,
) -> ErrorCode;

pub type WalletSearchAllRecords =
    extern "C" fn(storage_handle: i32, search_handle_p: *mut i32) -> ErrorCode;

pub type WalletGetSearchTotalCount =
    extern "C" fn(storage_handle: i32, search_handle: i32, total_count_p: *mut usize) -> ErrorCode;

pub type WalletFetchSearchNextRecord =
    extern "C" fn(storage_handle: i32, search_handle: i32, record_handle_p: *mut i32) -> ErrorCode;

pub type WalletFreeSearch = extern "C" fn(storage_handle: i32, search_handle: i32) -> ErrorCode;

pub type ResponseEmptyCB = extern "C" fn(xcommand_handle: i32, err: i32);
