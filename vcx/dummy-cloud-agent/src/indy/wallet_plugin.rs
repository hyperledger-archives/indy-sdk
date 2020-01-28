use std::ffi::CString;

use indyrs::ErrorCode;
use libc::c_char;
use serde_json::Value;

use crate::utils::dyn_lib::load_lib;

pub fn load_storage_library(library: &str, initializer: &str) -> Result<libloading::Library, String> {
    debug!("Loading storage plugin '{:}' as dynamic library.", library);
    match load_lib(library) {
        Ok(lib) => {
            unsafe {
                debug!("Storage library '{:}' loaded. Resolving its init function '{:}'.", library, initializer);
                let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = lib.get(initializer.as_bytes()).unwrap();
                debug!("Initializing library '{:}' by calling function '{:}'.", library, initializer);
                match init_func() {
                    ErrorCode::Success => debug!("Basic initialization for library '{:}' succeeded.", library),
                    err => return Err(format!("Failed to resolve init function '{:}' for storage library '{:}'. Details {:?}.", initializer, library, err))
                };
                Ok(lib)
            }
        }
        Err(err) => Err(format!("Storage library {:} failed to load. Details: {:?}", library, err))
    }
}
const POSTGRES_ADDITIONAL_INITIALIZER: &str = "init_storagetype";

pub fn finish_loading_postgres(storage_lib: libloading::Library, storage_config: &str, storage_credentials: &str) -> Result<(), String> {
    unsafe {
        debug!("Finishing initialization for postgre wallet plugin.");
        let init_storage_func: libloading::Symbol<unsafe extern fn(config: *const c_char, credentials: *const c_char) -> ErrorCode> = storage_lib.get(POSTGRES_ADDITIONAL_INITIALIZER.as_bytes()).unwrap();
        let init_config = CString::new(storage_config).expect("CString::new failed");
        let init_credentials = CString::new(storage_credentials).expect("CString::new failed");
        match init_storage_func(init_config.as_ptr(), init_credentials.as_ptr()) {
            ErrorCode::Success => {
                debug!("Successfully completed postgre library initialization.");
            }
            err => return Err(format!("Failed to complete postgre library initialization. Details {:?}.", err))
        }
    }
    Ok(())
}

fn get_plugin_library_path(storage_type: &str, plugin_library_path: &Option<String>) -> Result<String, String> {
    if storage_type == "postgres_storage" {
        Ok(plugin_library_path.clone().unwrap_or(DEFAULT_POSTGRES_PLUGIN_PATH.into()))
    } else {
        plugin_library_path.clone()
            .ok_or(format!("You have to specify 'storage.plugin_library_path' in config because storage of type {} does not have known default path.", storage_type))
    }
}

fn get_plugin_init_function(storage_type: &str, plugin_init_function: &Option<String>) -> Result<String, String> {
    if storage_type == "postgres_storage" {
        Ok(plugin_init_function.clone().unwrap_or(DEFAULT_POSTGRES_PLUGIN_INITIALIZER.into()))
    } else {
        plugin_init_function.clone()
            .ok_or(format!("You have to specify 'storage.plugin_init_function' in con_load_libfig because storage of type {} does not have known default path.", storage_type))
    }
}


const DEFAULT_POSTGRES_PLUGIN_INITIALIZER: &str = "postgresstorage_init";

#[cfg(target_os = "macos")]
static DEFAULT_POSTGRES_PLUGIN_PATH: &str = "/usr/local/lib/libindystrgpostgres.dylib";
#[cfg(target_os = "linux")]
static DEFAULT_POSTGRES_PLUGIN_PATH: &str = "/usr/lib/libindystrgpostgres.so";
#[cfg(target_os = "windows")]
static DEFAULT_POSTGRES_PLUGIN_PATH: &str = "c:\\windows\\system32\\libindystrgpostgres.dll";

pub fn serialize_storage_plugin_configuration(storage_type: &str,
                                              storage_config: &Option<Value>,
                                              storage_credentials: &Option<Value>,
                                              plugin_library_path: &Option<String>,
                                              plugin_init_function: &Option<String>)
                                              -> Result<(String, String, String, String), String> {
    let plugin_library_path_serialized = get_plugin_library_path(storage_type, plugin_library_path)?;
    let plugin_init_function_serialized = get_plugin_init_function(storage_type, plugin_init_function)?;
    let storage_config_serialized = serde_json::to_string(storage_config)
        .map_err(|err| format!("Failed to serialize 'storage_config'. {:?}", err))?;
    let storage_credentials_serialized = serde_json::to_string(storage_credentials)
        .map_err(|err| format!("Failed to serialize 'storage_credentials' {:?}", err))?;
    Ok((plugin_library_path_serialized,
        plugin_init_function_serialized,
        storage_config_serialized,
        storage_credentials_serialized)
    )
}


#[cfg(test)]
mod tests {
    use crate::domain::config::WalletStorageConfig;

    use super::*;

    #[test]
    fn should_load_custom_storage_settings_from_config() {
        let sample_config = r#"
        {
            "config": {
                "foo": "bar"
            },
            "credentials": {
                "password": "baz"
            },
            "type": "foo_storage_plugin",
            "plugin_library_path": "/some/custom/path",
            "plugin_init_function": "init_foo"
        }
        "#;

        let config: WalletStorageConfig = serde_json::from_str(sample_config).expect("Cant deserialize test data");
        let result = serialize_storage_plugin_configuration(&config.xtype.expect(""),
                                                            &config.config,
                                                            &config.credentials,
                                                            &config.plugin_library_path,
                                                            &config.plugin_init_function);
        match result {
            Err(err) => panic!(format!("Failed to process configuration. Details: {}", err)),
            Ok((path, initializer, config, credentials)) => {
                assert_eq!(path, "/some/custom/path");
                assert_eq!(initializer, "init_foo");
                assert_eq!(config, r#"{"foo":"bar"}"#);
                assert_eq!(credentials, r#"{"password":"baz"}"#);
            }
        };
    }

    #[test]
    #[should_panic(expected = "You have to specify 'storage.plugin_library_path' in config")]
    fn should_fail_if_plugin_config_is_not_provided_for_unknown_plugin() {
        let sample_config = r#"
        {
            "config": {
                "foo": "bar"
            },
            "credentials": {
                "password": "baz"
            },
            "type": "foo_storage_plugin",
            "plugin_library_path": null,
            "plugin_init_function": null
        }
        "#;

        let config: WalletStorageConfig = serde_json::from_str(sample_config).expect("Cant deserialize test data");
        let result = serialize_storage_plugin_configuration(&config.xtype.expect(""),
                                                            &config.config,
                                                            &config.credentials,
                                                            &config.plugin_library_path,
                                                            &config.plugin_init_function);
        match result {
            Err(err) => panic!(format!("Failed to process configuration. Details: {}", err)),
            Ok(_) => {}
        };
    }

    #[test]
    fn should_use_default_values_for_storage_of_type_postgres_storage() {
        let sample_config = r#"
        {
            "config": {
                "foo": "bar"
            },
            "credentials": {
                "password": "baz"
            },
            "type": "postgres_storage",
            "plugin_library_path": null,
            "plugin_init_function": null
        }
        "#;

        let config: WalletStorageConfig = serde_json::from_str(sample_config).expect("Cant deserialize test data");
        let result = serialize_storage_plugin_configuration(&config.xtype.expect(""),
                                                            &config.config,
                                                            &config.credentials,
                                                            &config.plugin_library_path,
                                                            &config.plugin_init_function);
        match result {
            Err(err) => panic!(format!("Failed to process configuration. Details: {}", err)),
            Ok((path, initializer, config, credentials)) => {
                assert_eq!(path, DEFAULT_POSTGRES_PLUGIN_PATH);
                assert_eq!(initializer, DEFAULT_POSTGRES_PLUGIN_INITIALIZER);
                assert_eq!(config, r#"{"foo":"bar"}"#);
                assert_eq!(credentials, r#"{"password":"baz"}"#);
            }
        };
    }

    #[test]
    fn should_be_possible_to_override_postgres_storage_config_defaults() {
        let sample_config = r#"
        {
            "config": {
                "foo": "bar"
            },
            "credentials": {
                "password": "baz"
            },
            "type": "postgres_storage",
            "plugin_library_path": "OVERRIDE1",
            "plugin_init_function": "OVERRIDE2"
        }
        "#;

        let config: WalletStorageConfig = serde_json::from_str(sample_config).expect("Cant deserialize test data");
        let result = serialize_storage_plugin_configuration(&config.xtype.expect(""),
                                                            &config.config,
                                                            &config.credentials,
                                                            &config.plugin_library_path,
                                                            &config.plugin_init_function);
        match result {
            Err(err) => panic!(format!("Failed to process configuration. Details: {}", err)),
            Ok((path, initializer, config, credentials)) => {
                assert_eq!(path, "OVERRIDE1");
                assert_eq!(initializer, "OVERRIDE2");
                assert_eq!(config, r#"{"foo":"bar"}"#);
                assert_eq!(credentials, r#"{"password":"baz"}"#);
            }
        };
    }
}