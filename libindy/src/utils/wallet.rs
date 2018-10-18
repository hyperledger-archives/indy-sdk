
#[cfg(test)]
pub mod test_utils {
    extern crate sharedlib;
    extern crate base64;
    extern crate libc;
    extern crate os_type;

    use self::sharedlib::{Lib, Func, Symbol};

    use api::wallet::*;
    use errors;

    use domain::wallet::{Config, Credentials};
    use services::wallet::WalletService;

    use std::collections::HashMap;
    use std::env;
    use std::sync::Mutex;

    use std::path::Path;

    use serde_json;
    use serde_json::Value;


    /*
    * Update wallet config based on supplied configuration,
    *     *only if* "storage_type" is not already provided.
    */
    pub fn override_wallet_config_creds(config: &Config, credentials: &Credentials, wallet_service: &WalletService, load_dynalib: bool) -> (Config, Credentials) {
        // if storge_type is explicit then bail
        //if let Some(_) = config.storage_type {
        //    return ((*config).clone(), (*credentials).clone());
        //}

        // if no config is provided at all then bail
        let storage_config = wallet_storage_overrides();
        if !any_overrides(&storage_config) {
            return ((*config).clone(), (*credentials).clone());
        }

        // load dynamic library if requested
        if load_dynalib {
            load_storage_library_config(&storage_config, wallet_service).unwrap();
        }

        // update config and credentials
        let config = override_wallet_configuration(config, &storage_config);
        let credentials = override_wallet_credentials(credentials, &storage_config);

        return (config, credentials);
    }

    /*
    * Dynamically loads the specified library and registers storage
    */
    pub fn load_storage_library(wallet_service: &WalletService, stg_type: &str, library_path: &str, fn_pfx: &str) -> Result<(), errors::wallet::WalletError> {
        println!("Loading {} {} {}", stg_type, library_path, fn_pfx);
        lazy_static! {
            static ref STG_REGISERED_WALLETS: Mutex<HashMap<String, Lib>> = Default::default();
        }

        let mut wallets = STG_REGISERED_WALLETS.lock().unwrap();

        if !wallets.contains_key(stg_type) {
            let lib_path = Path::new(library_path);
            unsafe {
                let lib = match Lib::new(lib_path) {
                    Ok(rlib) => {
                        rlib
                    },
                    Err(err) => {
                        panic!("Load error {:?}", err)
                    }
                };
            wallets.insert(stg_type.to_string(), lib);
            }
        }

        let lib_ref = wallets.get(stg_type).unwrap();

        let err;
        unsafe {
            let fn_create_handler: Func<WalletCreate> = lib_ref.find_func(&format!("{}create", fn_pfx)).unwrap();
            let fn_open_handler: Func<WalletOpen> = lib_ref.find_func(&format!("{}open", fn_pfx)).unwrap();
            let fn_close_handler: Func<WalletClose> = lib_ref.find_func(&format!("{}close", fn_pfx)).unwrap();
            let fn_delete_handler: Func<WalletDelete> = lib_ref.find_func(&format!("{}delete", fn_pfx)).unwrap();
            let fn_add_record_handler: Func<WalletAddRecord> = lib_ref.find_func(&format!("{}add_record", fn_pfx)).unwrap();
            let fn_update_record_value_handler: Func<WalletUpdateRecordValue> = lib_ref.find_func(&format!("{}update_record_value", fn_pfx)).unwrap();
            let fn_update_record_tags_handler: Func<WalletUpdateRecordTags> = lib_ref.find_func(&format!("{}update_record_tags", fn_pfx)).unwrap();
            let fn_add_record_tags_handler: Func<WalletAddRecordTags> = lib_ref.find_func(&format!("{}add_record_tags", fn_pfx)).unwrap();
            let fn_delete_record_tags_handler: Func<WalletDeleteRecordTags> = lib_ref.find_func(&format!("{}delete_record_tags", fn_pfx)).unwrap();
            let fn_delete_record_handler: Func<WalletDeleteRecord> = lib_ref.find_func(&format!("{}delete_record", fn_pfx)).unwrap();
            let fn_get_record_handler: Func<WalletGetRecord> = lib_ref.find_func(&format!("{}get_record", fn_pfx)).unwrap();
            let fn_get_record_id_handler: Func<WalletGetRecordId> = lib_ref.find_func(&format!("{}get_record_id", fn_pfx)).unwrap();
            let fn_get_record_type_handler: Func<WalletGetRecordType> = lib_ref.find_func(&format!("{}get_record_type", fn_pfx)).unwrap();
            let fn_get_record_value_handler: Func<WalletGetRecordValue> = lib_ref.find_func(&format!("{}get_record_value", fn_pfx)).unwrap();
            let fn_get_record_tags_handler: Func<WalletGetRecordTags> = lib_ref.find_func(&format!("{}get_record_tags", fn_pfx)).unwrap();
            let fn_free_record_handler: Func<WalletFreeRecord> = lib_ref.find_func(&format!("{}free_record", fn_pfx)).unwrap();
            let fn_get_storage_metadata_handler: Func<WalletGetStorageMetadata> = lib_ref.find_func(&format!("{}get_storage_metadata", fn_pfx)).unwrap();
            let fn_set_storage_metadata_handler: Func<WalletSetStorageMetadata> = lib_ref.find_func(&format!("{}set_storage_metadata", fn_pfx)).unwrap();
            let fn_free_storage_metadata_handler: Func<WalletFreeStorageMetadata> = lib_ref.find_func(&format!("{}free_storage_metadata", fn_pfx)).unwrap();
            let fn_search_records_handler: Func<WalletSearchRecords> = lib_ref.find_func(&format!("{}search_records", fn_pfx)).unwrap();
            let fn_search_all_records_handler: Func<WalletSearchAllRecords> = lib_ref.find_func(&format!("{}search_all_records", fn_pfx)).unwrap();
            let fn_get_search_total_count_handler: Func<WalletGetSearchTotalCount> = lib_ref.find_func(&format!("{}get_search_total_count", fn_pfx)).unwrap();
            let fn_fetch_search_next_record_handler: Func<WalletFetchSearchNextRecord> = lib_ref.find_func(&format!("{}fetch_search_next_record", fn_pfx)).unwrap();
            let fn_free_search_handler: Func<WalletFreeSearch> = lib_ref.find_func(&format!("{}free_search", fn_pfx)).unwrap();

            err = wallet_service.register_wallet_storage(
                stg_type,
                fn_create_handler.get(),
                fn_open_handler.get(),
                fn_close_handler.get(),
                fn_delete_handler.get(),
                fn_add_record_handler.get(),
                fn_update_record_value_handler.get(),
                fn_update_record_tags_handler.get(),
                fn_add_record_tags_handler.get(),
                fn_delete_record_tags_handler.get(),
                fn_delete_record_handler.get(),
                fn_get_record_handler.get(),
                fn_get_record_id_handler.get(),
                fn_get_record_type_handler.get(),
                fn_get_record_value_handler.get(),
                fn_get_record_tags_handler.get(),
                fn_free_record_handler.get(),
                fn_get_storage_metadata_handler.get(),
                fn_set_storage_metadata_handler.get(),
                fn_free_storage_metadata_handler.get(),
                fn_search_records_handler.get(),
                fn_search_all_records_handler.get(),
                fn_get_search_total_count_handler.get(),
                fn_fetch_search_next_record_handler.get(),
                fn_free_search_handler.get()
            );
        }

        match err {
            Err(errors::wallet::WalletError::TypeAlreadyRegistered(_)) => Ok(()),
            _ => err
        }
    }

    /*
    * Dynamically loads the specified library and registers storage, based on provided config
    */
    pub fn load_storage_library_config(storage_config: &HashMap<String, Option<String>>, wallet_service: &WalletService) -> Result<(), errors::wallet::WalletError> {
        match storage_config.get("STG_LIB") {
            Some(slibrary) => match slibrary {
                Some(library) => {
                    let stg_type: String = match storage_config.get("STG_TYPE") {
                        Some(styp) => match styp {
                            Some(typ) => typ.clone(),
                            None => "".to_string()
                        },
                        None => "".to_string()
                    };
                    let fn_pfx: String = match storage_config.get("STG_FN_PREFIX") {
                        Some(spfx) => match spfx {
                            Some(pfx) => pfx.clone(),
                            None => "".to_string()
                        },
                        None => "".to_string()
                    };
                    load_storage_library(wallet_service, &stg_type[..], &library[..], &fn_pfx[..])
                },
                None => Ok(())
            },
            None => Ok(())
        }
    }

    /*
    * Update the given configuration string based on supplied overrides
    */
    pub fn override_wallet_configuration(config: &Config, overrides: &HashMap<String, Option<String>>) -> Config {
        let mut config: Config = (*config).clone();

        match overrides.get("STG_TYPE") {
            Some(stype) => match stype {
                Some(wtype) => {
                    config.storage_type = Some(wtype.clone());
                },
                None => ()
            },
            None => ()
        }
        match overrides.get("STG_CONFIG") {
            Some(sconfig) => match sconfig {
                Some(wconfig) => {
                    let v: Value = serde_json::from_str(&wconfig[..]).unwrap();
                    config.storage_config = Some(v.clone());
                },
                None => ()
            },
            None => ()
        }

        config
    }

    /*
    * Update the given credentials string based on supplied overrides
    */
    pub fn override_wallet_credentials(creds: &Credentials, overrides: &HashMap<String, Option<String>>) -> Credentials {
        let mut creds: Credentials = (*creds).clone();

        match overrides.get("STG_CREDS") {
            Some(screds) => match screds {
                Some(wcreds) => {
                    let v: Value = serde_json::from_str(&wcreds[..]).unwrap();
                    creds.storage_credentials = Some(v.clone());
                },
                None => ()
            },
            None => ()
        }

        creds
    }

    /*
    * Returns wallet storage configuation dynamically configured via environment variables:
    * STG_CONFIG - json configuration string to pass to the wallet on creation and open
    * STG_CREDS - json credentials string to pass to the wallet on creation and open
    * STG_TYPE - storage type to create
    * STG_LIB - c-callable library to load (contains a plug-in storage)
    *             - if specified will dynamically load and register a wallet storage
    * STG_FN_PREFIX - prefix for all plug-in functions (allows standard function naming)
    */
    pub fn wallet_storage_overrides() -> HashMap<String, Option<String>> {
        // check for default configs for inmem or postgres plugins
        let env_var = "STG_USE";
        match env::var(env_var) {
            Ok(var) => {
                match var.to_lowercase().as_ref() {
                    "inmem" => {
                        return inmem_lib_test_overrides();
                    },
                    "postgres" => {
                        return postgres_lib_test_overrides();
                    },
                    _ => ()
                }
            },
            Err(_) => ()
        };

        let mut storage_config = HashMap::new();
        let env_vars = vec!["STG_CONFIG", "STG_CREDS", "STG_TYPE", "STG_LIB", "STG_FN_PREFIX"];

        for env_var in env_vars.iter() {
            match env::var(env_var) {
                Ok(var) => storage_config.insert(env_var.to_string(), Some(var.to_string())),
                Err(_) => storage_config.insert(env_var.to_string(), None)
            };
        }

        storage_config
    }

    pub fn any_overrides(storage_config: &HashMap<String, Option<String>>) -> bool {
        for (_key, val) in storage_config {
            if let Some(_) = val {
                return true;
            }
        }
        return false;
    }

    pub fn inmem_lib_test_overrides() -> HashMap<String, Option<String>> {
        // Note - on OS/X we specify the fully qualified path to the shared lib
        //      - on UNIX systems we have to include the directories in LD_LIBRARY_PATH, e.g.:
        //      export LD_LIBRARY_PATH=../samples/storage/storage-inmem/target/debug/:./target/debug/
        let os = os_type::current_platform();
        let osfile = match os.os_type {
            os_type::OSType::OSX => "libindystrginmem.dylib",
            _ => "libindystrginmem.so"
        };

        let mut storage_config = HashMap::new();
        let env_vars = vec!["STG_CONFIG", "STG_CREDS", "STG_TYPE", "STG_LIB", "STG_FN_PREFIX"];
        storage_config.insert(env_vars[0].to_string(), None);
        storage_config.insert(env_vars[1].to_string(), None);
        storage_config.insert(env_vars[2].to_string(), Some("inmem_custom".to_string()));
        storage_config.insert(env_vars[3].to_string(), Some(osfile.to_string()));
        storage_config.insert(env_vars[4].to_string(), Some("inmemwallet_fn_".to_string()));
        storage_config
    }

    pub fn postgres_lib_test_overrides() -> HashMap<String, Option<String>> {
        // Note - on OS/X we specify the fully qualified path to the shared lib
        //      - on UNIX systems we have to include the directories in LD_LIBRARY_PATH, e.g.:
        //      export LD_LIBRARY_PATH=../samples/storage/storage-inmem/target/debug/:./target/debug/
        let os = os_type::current_platform();
        let osfile = match os.os_type {
            os_type::OSType::OSX => "libindystrgpostgres.dylib",
            _ => "libindystrgpostgres.so"
        };

        let mut storage_config = HashMap::new();
        let env_vars = vec!["STG_CONFIG", "STG_CREDS", "STG_TYPE", "STG_LIB", "STG_FN_PREFIX"];
        storage_config.insert(env_vars[0].to_string(), Some(r#"{"url":"localhost:5432"}"#.to_string()));
        storage_config.insert(env_vars[1].to_string(), Some(r#"{"account":"postgres","password":"mysecretpassword","admin_account":"postgres","admin_password":"mysecretpassword"}"#.to_string()));
        storage_config.insert(env_vars[2].to_string(), Some("postgres_custom".to_string()));
        storage_config.insert(env_vars[3].to_string(), Some(osfile.to_string()));
        storage_config.insert(env_vars[4].to_string(), Some("postgreswallet_fn_".to_string()));
        storage_config
    }
}
