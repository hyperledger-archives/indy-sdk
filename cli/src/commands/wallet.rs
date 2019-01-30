use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;
use utils::table::print_list_table;
use indy::ErrorCode;
use libindy::wallet::Wallet;

use serde_json;
use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::fs;
use std::fs::File;
use utils::environment::EnvironmentUtils;
use std::io::{Read, Write};
use std::path::PathBuf;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("wallet", "Wallet management commands"));
}

pub mod create_command {
    use super::*;

    command!(CommandMetadata::build("create", "Create new wallet and attach to Indy CLI")
                .add_main_param("name", "Identifier of the wallet")
                .add_required_deferred_param("key", "Key or passphrase used for wallet key derivation.
                                               Look to key_derivation_method param for information about supported key derivation methods.")
                .add_optional_param("key_derivation_method", "Algorithm to use for wallet key derivation. One of:
                                    argon2m - derive secured wallet key (used by default)
                                    argon2i - derive secured wallet key (less secured but faster)
                                    raw - raw wallet key provided (skip derivation)")
                .add_optional_param("storage_type", "Type of the wallet storage.")
                .add_optional_param("storage_config", "The list of key:value pairs defined by storage type.")
                .add_optional_param("storage_credentials", "The list of key:value pairs defined by storage type.")
                .add_example("wallet create wallet1 key")
                .add_example("wallet create wallet1 key storage_type=default")
                .add_example(r#"wallet create wallet1 key storage_type=default storage_config={"key1":"value1","key2":"value2"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, secret!(params));

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let key_derivation_method = get_opt_str_param("key_derivation_method", params).map_err(error_err!())?;
        let storage_type = get_opt_str_param("storage_type", params).map_err(error_err!())?.unwrap_or("default");
        let storage_config = get_opt_object_param("storage_config", params).map_err(error_err!())?;
        let storage_credentials = get_opt_object_param("storage_credentials", params).map_err(error_err!())?;

        let config: String = json!({ "id": id.clone(), "storage_type": storage_type, "storage_config": storage_config }).to_string();

        let credentials = {
            let mut json = JSONMap::new();

            json.insert("key".to_string(), serde_json::Value::String(key.to_string()));
            json.insert("key_derivation_method".to_string(), serde_json::Value::String(map_key_derivation_method(key_derivation_method)?.to_string()));
            update_json_map_opt_key!(json, "storage_credentials", storage_credentials);

            JSONValue::from(json).to_string()
        };

        if _wallet_config_path(id).exists() {
            return Err(println_err!("Wallet \"{}\" is already attached to CLI", id));
        }

        trace!("Wallet::create_wallet try: config {}", config);

        let res = Wallet::create_wallet(config.as_str(),
                                        credentials.as_str(),
        );

        trace!("Wallet::create_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => {
                _store_wallet_config(id, &config)
                    .map_err(|err| println_err!("Cannot store wallet \"{}\" config file: {:?}", id, err))?;

                Ok(println_succ!("Wallet \"{}\" has been created", id))
            }
            Err(err) => {
                match err.error_code {
                    ErrorCode::WalletAlreadyExistsError => Err(println_err!("Wallet \"{}\" already exists", id)),
                    ErrorCode::CommonIOError => Err(println_err!("Invalid wallet name \"{}\"", id)),
                    _ => Err(handle_indy_error(err, None, None, Some(&id)))
                }
            }
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod attach_command {
    use super::*;

    command!(CommandMetadata::build("attach", "Attach existing wallet to Indy CLI")
                .add_main_param("name", "Identifier of the wallet")
                .add_optional_param("storage_type", "Type of the wallet storage.")
                .add_optional_param("storage_config", "The list of key:value pairs defined by storage type.")
                .add_example("wallet attach wallet1")
                .add_example("wallet attach wallet1 storage_type=default")
                .add_example(r#"wallet attach wallet1 storage_type=default storage_config={"key1":"value1","key2":"value2"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, secret!(params));

        let id = get_str_param("name", params).map_err(error_err!())?;
        let storage_type = get_opt_str_param("storage_type", params).map_err(error_err!())?.unwrap_or("default");
        let storage_config = get_opt_object_param("storage_config", params).map_err(error_err!())?;

        if _wallet_config_path(id).exists() {
            return Err(println_err!("Wallet \"{}\" is already attached to CLI", id));
        }

        let config: String = json!({ "id": id.clone(), "storage_type": storage_type, "storage_config": storage_config }).to_string();

        _store_wallet_config(id, &config)
            .map_err(|err| println_err!("Cannot store wallet \"{}\" config file: {:?}", id, err))?;

        let res = Ok(println_succ!("Wallet \"{}\" has been attached", id));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod open_command {
    use super::*;

    command_with_cleanup!(CommandMetadata::build("open", "Open wallet. Also close previously opened.")
                            .add_main_param("name", "Identifier of the wallet")
                            .add_required_deferred_param("key", "Key or passphrase used for wallet key derivation.
                                               Look to key_derivation_method param for information about supported key derivation methods.")
                            .add_optional_param("key_derivation_method", "Algorithm to use for wallet key derivation. One of:
                                                argon2m - derive secured wallet key (used by default)
                                                argon2i - derive secured wallet key (less secured but faster)
                                                raw - raw key provided (skip derivation)")
                            .add_optional_deferred_param("rekey", "New key or passphrase used for wallet key derivation (will replace previous one).")
                            .add_optional_param("rekey_derivation_method", "Algorithm to use for wallet rekey derivation. One of:
                                                argon2m - derive secured wallet key (used by default)
                                                argon2i - derive secured wallet key (less secured but faster)
                                                raw - raw key provided (skip derivation)")
                            .add_optional_param("storage_credentials", "The list of key:value pairs defined by storage type.")
                            .add_example("wallet open wallet1 key")
                            .add_example("wallet open wallet1 key rekey")
                            .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, secret!(params));

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let rekey = get_opt_str_param("rekey", params).map_err(error_err!())?;
        let key_derivation_method = get_opt_str_param("key_derivation_method", params).map_err(error_err!())?;
        let rekey_derivation_method = get_opt_str_param("rekey_derivation_method", params).map_err(error_err!())?;
        let storage_credentials = get_opt_object_param("storage_credentials", params).map_err(error_err!())?;

        let config = _read_wallet_config(id)
            .map_err(|_| println_err!("Wallet \"{}\" isn't attached to CLI", id))?;

        let credentials = {
            let mut json = JSONMap::new();

            json.insert("key".to_string(), serde_json::Value::String(key.to_string()));
            json.insert("key_derivation_method".to_string(), serde_json::Value::String(map_key_derivation_method(key_derivation_method)?.to_string()));

            update_json_map_opt_key!(json, "rekey", rekey);
            json.insert("rekey_derivation_method".to_string(), serde_json::Value::String(map_key_derivation_method(rekey_derivation_method)?.to_string()));
            update_json_map_opt_key!(json, "storage_credentials", storage_credentials);

            JSONValue::from(json).to_string()
        };

        let res = Ok(())
            .and_then(|_| {
                set_active_did(ctx, None);
                if let Some((handle, id)) = get_opened_wallet(ctx) {
                    match Wallet::close_wallet(handle) {
                        Ok(()) => Ok(println_succ!("Wallet \"{}\" has been closed", id)),
                        Err(err) => Err(handle_indy_error(err, None, None, None))
                    }
                } else {
                    Ok(())
                }
            })
            .and_then(|_| {
                match Wallet::open_wallet(config.as_str(), &credentials.as_str()) {
                    Ok(handle) => {
                        set_opened_wallet(ctx, Some((handle, id.to_owned())));
                        Ok(println_succ!("Wallet \"{}\" has been opened", id))
                    }
                    Err(err) => {
                        set_opened_wallet(ctx, None);
                        match err.error_code {
                            ErrorCode::WalletAlreadyOpenedError => Err(println_err!("Wallet \"{}\" already opened", id)),
                            ErrorCode::WalletAccessFailed => Err(println_err!("Cannot open wallet \"{}\". Invalid key has been provided", id)),
                            ErrorCode::WalletNotFoundError => Err(println_err!("Wallet \"{}\" not found or unavailable", id)),
                            _ => Err(handle_indy_error(err, None, None, Some(&id))),
                        }
                    }
                }
            });

        trace!("execute << {:?}", res);
        res
    }

    pub fn cleanup(ctx: &CommandContext) {
        trace!("cleanup >> ctx {:?}", ctx);

        if let Some((handle, name)) = get_opened_wallet(ctx) {
            match Wallet::close_wallet(handle) {
                Ok(()) => {
                    set_opened_wallet(ctx, Some((handle, name.clone())));
                    println_succ!("Wallet \"{}\" has been closed", name)
                }
                Err(err) => handle_indy_error(err, None, None, None)
            }
        }

        trace!("cleanup <<");
    }
}

pub mod list_command {
    use super::*;

    command!(CommandMetadata::build("list", "List attached wallets.")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallets: Vec<serde_json::Value> = _list_wallets();

        print_list_table(&wallets,
                         &vec![("id", "Name"),
                               ("storage_type", "Type")],
                         "There are no wallets");

        if let Some((_, cur_wallet)) = get_opened_wallet(ctx) {
            println_succ!("Current wallet \"{}\"", cur_wallet);
        }

        let res = Ok(());

        trace!("execute << {:?}", res);
        res
    }
}

pub mod close_command {
    use super::*;

    command!(CommandMetadata::build("close", "Close opened wallet.")
                    .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let res = Ok(())
            .and_then(|_| {
                if let Some(wallet) = get_opened_wallet(ctx) {
                    Ok(wallet)
                } else {
                    Err(println_err!("There is no opened wallet now"))
                }
            })
            .and_then(|wallet| {
                let (handle, name) = wallet;
                match Wallet::close_wallet(handle) {
                    Ok(()) => {
                        set_opened_wallet(ctx, None);
                        set_active_did(ctx, None);
                        Ok(println_succ!("Wallet \"{}\" has been closed", name))
                    }
                    Err(err) => Err(handle_indy_error(err, None, None, None))
                }
            });

        trace!("CloseCommand::execute << {:?}", res);
        res
    }
}

pub mod delete_command {
    use super::*;

    command!(CommandMetadata::build("delete", "Delete wallet.")
                .add_main_param("name", "Identifier of the wallet")
                .add_required_deferred_param("key", "Key or passphrase used for wallet key derivation.
                                               Look to key_derivation_method param for information about supported key derivation methods.")
                .add_optional_param("key_derivation_method", "Algorithm to use for wallet key derivation. One of:
                                    argon2m - derive secured wallet key (used by default)
                                    argon2i - derive secured wallet key (less secured but faster)
                                    raw - raw key provided (skip derivation)")
                .add_optional_param("storage_credentials", "The list of key:value pairs defined by storage type.")
                .add_example("wallet delete wallet1 key")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx: {:?} params {:?}", ctx, secret!(params));

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let key_derivation_method = get_opt_str_param("key_derivation_method", params).map_err(error_err!())?;
        let storage_credentials = get_opt_object_param("storage_credentials", params).map_err(error_err!())?;

        let config = _read_wallet_config(id)
            .map_err(|_| println_err!("Wallet \"{}\" isn't attached to CLI", id))?;

        let credentials = {
            let mut json = JSONMap::new();

            json.insert("key".to_string(), serde_json::Value::String(key.to_string()));
            json.insert("key_derivation_method".to_string(), serde_json::Value::String(map_key_derivation_method(key_derivation_method)?.to_string()));
            update_json_map_opt_key!(json, "storage_credentials", storage_credentials);

            JSONValue::from(json).to_string()
        };

        let res = match Wallet::delete_wallet(config.as_str(), credentials.as_str()) {
            Ok(()) => {
                _delete_wallet_config(id)
                    .map_err(|err| println_err!("Cannot delete \"{}\" config file: {:?}", id, err))?;

                Ok(println_succ!("Wallet \"{}\" has been deleted", id))
            }
            Err(err) => {
                match err.error_code {
                    ErrorCode::WalletNotFoundError => Err(println_err!("Wallet \"{}\" not found or unavailable", id)),
                    ErrorCode::WalletAccessFailed => Err(println_err!("Cannot delete wallet \"{}\". Invalid key has been provided ", id)),
                    _ => Err(handle_indy_error(err, None, None, Some(&id)))
                }
            }
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod detach_command {
    use super::*;

    command!(CommandMetadata::build("detach", "Detach wallet from Indy CLI")
                .add_main_param("name", "Identifier of the wallet")
                .add_example("wallet detach wallet1")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx: {:?} params {:?}", ctx, secret!(params));

        let id = get_str_param("name", params).map_err(error_err!())?;

        if !_wallet_config_path(id).exists() {
            return Err(println_err!("Wallet \"{}\" isn't attached to CLI", id));
        }

        if let Some((_, name)) = get_opened_wallet(ctx) {
            if id == name { return Err(println_err!("Wallet \"{}\" is opened", id)); }
        }

        _delete_wallet_config(id)
            .map_err(|err| println_err!("Cannot delete \"{}\" config file: {:?}", id, err))?;

        let res = Ok(println_succ!("Wallet \"{}\" has been detached", id));
        trace!("execute << {:?}", res);
        res
    }
}

pub mod export_command {
    use super::*;

    command!(CommandMetadata::build("export", "Export opened wallet to the file")
                .add_required_param("export_path", "Path to the export file")
                .add_required_deferred_param("export_key", "Key or passphrase used for export wallet key derivation.
                                               Look to key_derivation_method param for information about supported key derivation methods.")
                .add_optional_param("export_key_derivation_method", "Algorithm to use for export key derivation. One of:
                                    argon2m - derive secured export key (used by default)
                                    argon2i - derive secured export key (less secured but faster)
                                    raw - raw export key provided (skip derivation)")
                .add_example("wallet export export_path=/home/indy/export_wallet export_key")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, secret!(params));

        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let export_path = get_str_param("export_path", params).map_err(error_err!())?;
        let export_key = get_str_param("export_key", params).map_err(error_err!())?;
        let export_key_derivation_method = get_opt_str_param("export_key_derivation_method", params).map_err(error_err!())?;
        let export_config: String = json!({ "path": export_path.clone(), "key": export_key.clone(), "key_derivation_method": map_key_derivation_method(export_key_derivation_method)? }).to_string();

        trace!("Wallet::export_wallet try: wallet_name {}, export_path {}", wallet_name, export_path);

        let res = Wallet::export_wallet(wallet_handle,
                                        export_config.as_str());

        trace!("Wallet::export_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been exported to the file \"{}\"", wallet_name, export_path)),
            Err(err) => Err(handle_indy_error(err, None, None, Some(wallet_name.as_ref())))
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod import_command {
    use super::*;

    command!(CommandMetadata::build("import", "Create new wallet, attach to Indy CLI and then import content from the specified file")
                .add_main_param("name", "The name of new wallet")
                .add_required_deferred_param("key", "Key or passphrase used for wallet key derivation.
                                               Look to key_derivation_method param for information about supported key derivation methods.")
                .add_optional_param("key_derivation_method", "Algorithm to use for wallet key derivation. One of:
                                    argon2m - derive secured wallet key (used by default)
                                    argon2i - derive secured wallet key (less secured but faster)
                                    raw - raw key provided (skip derivation)")
                .add_optional_param("storage_type", "Type of the wallet storage.")
                .add_optional_param("storage_config", "The list of key:value pairs defined by storage type.")
                .add_optional_param("storage_credentials", "The list of key:value pairs defined by storage type.")
                .add_required_param("export_path", "Path to the file that contains exported wallet content")
                .add_required_deferred_param("export_key", "Key used for export of the wallet")
                .add_example("wallet import wallet1 key export_path=/home/indy/export_wallet export_key")
                .add_example(r#"wallet import wallet1 key export_path=/home/indy/export_wallet export_key storage_type=default storage_config={"key1":"value1","key2":"value2"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, secret!(params));

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let key_derivation_method = get_opt_str_param("key_derivation_method", params).map_err(error_err!())?;
        let export_path = get_str_param("export_path", params).map_err(error_err!())?;
        let export_key = get_str_param("export_key", params).map_err(error_err!())?;
        let storage_type = get_opt_str_param("storage_type", params).map_err(error_err!())?;
        let storage_config = get_opt_object_param("storage_config", params).map_err(error_err!())?;
        let storage_credentials = get_opt_object_param("storage_credentials", params).map_err(error_err!())?;

        let config: String = json!({ "id": id.clone(), "storage_type": storage_type, "storage_config": storage_config }).to_string();
        let import_config: String = json!({ "path": export_path.clone(), "key": export_key.clone()}).to_string();

        let credentials = {
            let mut json = JSONMap::new();

            json.insert("key".to_string(), serde_json::Value::String(key.to_string()));
            json.insert("key_derivation_method".to_string(), serde_json::Value::String(map_key_derivation_method(key_derivation_method)?.to_string()));
            update_json_map_opt_key!(json, "storage_credentials", storage_credentials);

            JSONValue::from(json).to_string()
        };

        if _wallet_config_path(id).exists() {
            return Err(println_err!("Wallet \"{}\" is already attached to CLI", id));
        }

        trace!("Wallet::import_wallet try: config {}, import_config {}", config, secret!(&import_config));

        let res = Wallet::import_wallet(config.as_str(),
                                        credentials.as_str(),
                                        import_config.as_str(),
        );

        trace!("Wallet::import_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => {
                _store_wallet_config(id, &config)
                    .map_err(|err| println_err!("Cannot store \"{}\" config file: {:?}", id, err))?;
                Ok(println_succ!("Wallet \"{}\" has been created", id))
            }
            Err(err) => Err(handle_indy_error(err, None, None, Some(id)))
        };

        trace!("execute << {:?}", res);
        res
    }
}

fn _wallets_path() -> PathBuf {
    let mut path = EnvironmentUtils::indy_home_path();
    path.push("wallets");
    path
}

fn _wallet_config_path(id: &str) -> PathBuf {
    let mut path = _wallets_path();
    path.push(id);
    path.set_extension("json");
    path
}

fn _init_wallets_dir(path: &PathBuf) -> Result<(), std::io::Error> {
    fs::DirBuilder::new()
        .recursive(true)
        .create(path)
}

fn _store_wallet_config(id: &str, config: &str) -> Result<(), std::io::Error> {
    _init_wallets_dir(&_wallets_path())?;

    let path = _wallet_config_path(id);

    let mut config_file = File::create(path)?;
    config_file.write_all(config.as_bytes())?;
    config_file.sync_all()?;

    Ok(())
}

fn _read_wallet_config(id: &str) -> Result<String, std::io::Error> {
    let path = _wallet_config_path(id);

    let mut config_json = String::new();

    let mut file = File::open(path)?;
    file.read_to_string(&mut config_json)?;

    Ok(config_json)
}

fn _delete_wallet_config(id: &str) -> Result<(), std::io::Error> {
    let path = _wallet_config_path(id);
    fs::remove_file(path)
}

fn _list_wallets() -> Vec<serde_json::Value> {
    let mut configs: Vec<serde_json::Value> = Vec::new();

    if let Ok(entries) = fs::read_dir(_wallets_path()) {
        for entry in entries {
            let file = if let Ok(dir_entry) = entry { dir_entry } else { continue };

            let mut config_json = String::new();

            File::open(file.path()).ok()
                .and_then(|mut f| f.read_to_string(&mut config_json).ok())
                .and_then(|_| serde_json::from_str::<serde_json::Value>(config_json.as_str()).ok())
                .map(|config| configs.push(config));
        }
    }

    configs
}

fn map_key_derivation_method(key: Option<&str>) -> Result<&'static str, ()> {
    // argon2m, argon2i, raw
    match key {
        None | Some("argon2m") => Ok("ARGON2I_MOD"),
        Some("argon2i") => Ok("ARGON2I_INT"),
        Some("raw") => Ok("RAW"),
        val @ _ => Err(println_err!("Unsupported Wallet Key type has been specified \"{}\"", val.unwrap())),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::environment::EnvironmentUtils;
    use std::path::PathBuf;

    const WALLET: &'static str = "wallet";
    const WALLET_KEY: &'static str = "wallet_key";
    const WALLET_KEY_RAW: &'static str = "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw";
    const EXPORT_KEY: &'static str = "export_key";

    mod create {
        use super::*;

        #[test]
        pub fn create_works() {
            let ctx = setup();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY_RAW.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);

            delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn create_works_for_twice() {
            let ctx = setup_with_wallet();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY_RAW.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn create_works_for_missed_credentials() {
            let ctx = setup();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }

        #[test]
        pub fn create_works_for_type() {
            let ctx = setup();
            let storage_type = "default";
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY_RAW.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                params.insert("storage_type", storage_type.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["storage_type"].as_str().unwrap(), storage_type);

            delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn create_works_for_config() {
            let ctx = setup();
            let config = r#"{"key":"value","key2":"value2"}"#;
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY_RAW.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                params.insert("storage_config", config.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["storage_config"].as_object().unwrap(),
                       serde_json::from_str::<serde_json::Value>(config).unwrap().as_object().unwrap());
            tear_down();
        }

        #[test]
        pub fn create_works_for_key_derivation_method() {
            let ctx = setup();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("key_derivation_method", "argon2m".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);

            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("key_derivation_method", "argon2m".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down();
        }

        #[test]
        pub fn create_works_for_wrong_export_key_derivation_method() {
            let ctx = setup();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("key_derivation_method", "some_type".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }
    }

    mod attach {
        use super::*;

        #[test]
        pub fn attach_works() {
            let ctx = setup();
            {
                let cmd = attach_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());
            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);

            tear_down();
        }

        #[test]
        pub fn attach_works_for_twice() {
            let ctx = setup();
            attach_wallet(&ctx);
            {
                let cmd = attach_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }

        #[test]
        pub fn attach_works_for_type() {
            let ctx = setup();
            let storage_type = "default";
            {
                let cmd = attach_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("storage_type", storage_type.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["storage_type"].as_str().unwrap(), storage_type);

            tear_down();
        }

        #[test]
        pub fn attach_for_config() {
            let ctx = setup();
            let config = r#"{"key":"value","key2":"value2"}"#;
            {
                let cmd = attach_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("storage_config", config.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["storage_config"].as_object().unwrap(),
                       serde_json::from_str::<serde_json::Value>(config).unwrap().as_object().unwrap());

            tear_down();
        }
    }

    mod open {
        use super::*;

        #[test]
        pub fn open_works() {
            let ctx = setup();
            create_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY_RAW.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_opened_wallet_handle(&ctx).unwrap();
            close_and_delete_wallet(&ctx);

            tear_down();
        }

        #[test]
        pub fn open_works_for_twice() {
            let ctx = setup_with_wallet();
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY_RAW.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                cmd.execute(&ctx, &params).unwrap(); //TODO: we close and open same wallet
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn open_works_for_not_created() {
            let ctx = setup();
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }

        #[test]
        pub fn open_works_for_missed_key() {
            let ctx = setup();
            create_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn open_works_for_wrong_key() {
            let ctx = setup();
            create_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", "other_key".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
            tear_down();
        }
    }

    mod list {
        use super::*;

        #[test]
        pub fn list_works() {
            let ctx = setup_with_wallet();
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_empty_list() {
            let ctx = setup();
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            tear_down();
        }
    }

    mod close {
        use super::*;

        #[test]
        pub fn close_works() {
            let ctx = setup_with_wallet();
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_opened_wallet_handle(&ctx).unwrap_err();
            delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn close_works_for_not_opened() {
            let ctx = setup();
            create_wallet(&ctx);
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn close_works_for_twice() {
            let ctx = setup();
            create_and_open_wallet(&ctx);
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
            tear_down();
        }
    }

    mod delete {
        use super::*;

        #[test]
        pub fn delete_works() {
            let ctx = setup();
            create_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY_RAW.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                cmd.execute(&CommandContext::new(), &params).unwrap();
            }
            let wallets = _list_wallets();
            assert_eq!(0, wallets.len());

            tear_down();
        }

        #[test]
        pub fn delete_works_for_not_created() {
            let ctx = setup();

            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY.to_string());
            cmd.execute(&ctx, &params).unwrap_err();

            tear_down();
        }

        #[test]
        pub fn delete_works_for_opened() {
            let ctx = setup();
            create_and_open_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn delete_works_for_wrong_key() {
            let ctx = setup();
            create_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", "other_key".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }
    }

    mod detach {
        use super::*;

        #[test]
        pub fn detach_works() {
            let ctx = setup();
            create_wallet(&ctx);
            {
                let cmd = detach_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&CommandContext::new(), &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(0, wallets.len());

            tear_down();
        }

        #[test]
        pub fn detach_works_for_not_attached() {
            let ctx = setup();

            let cmd = detach_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            cmd.execute(&ctx, &params).unwrap_err();

            tear_down();
        }

        #[test]
        pub fn detach_works_for_opened() {
            let ctx = setup();

            create_and_open_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            tear_down();
        }
    }

    mod export {
        use super::*;

        #[test]
        pub fn export_works() {
            let ctx = setup_with_wallet();

            let (path, path_str) = export_wallet_path();
            {
                let cmd = export_command::new();
                let mut params = CommandParams::new();
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            assert!(path.exists());
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn export_works_for_file_already_exists() {
            let ctx = setup_with_wallet();

            let (_, path_str) = export_wallet_path();

            export_wallet(&ctx, &path_str);
            {
                let cmd = export_command::new();
                let mut params = CommandParams::new();
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet(&ctx);
        }
    }

    mod import {
        use super::*;
        use super::did::tests::{new_did, use_did, SEED_MY1, DID_MY1};

        #[test]
        pub fn import_works() {
            let ctx = setup();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_MY1);

            let (_, path_str) = export_wallet_path();
            export_wallet(&ctx, &path_str);

            let wallet_name = "imported_wallet";
            // import wallet
            {
                let cmd = import_command::new();
                let mut params = CommandParams::new();
                params.insert("name", wallet_name.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            // open exported wallet
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", wallet_name.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            use_did(&ctx, DID_MY1);

            close_and_delete_wallet(&ctx);

            // delete first wallet
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", wallet_name.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&CommandContext::new(), &params).unwrap();
            }

            tear_down();
        }

        #[test]
        pub fn import_works_for_not_found_file() {
            let ctx = setup();
            let (_, path_str) = export_wallet_path();
            // import wallet
            {
                let cmd = import_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }

        #[test]
        pub fn import_works_for_other_key() {
            let ctx = setup();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_MY1);

            let (_, path_str) = export_wallet_path();
            export_wallet(&ctx, &path_str);

            let wallet_name = "imported_wallet";
            // import wallet
            {
                let cmd = import_command::new();
                let mut params = CommandParams::new();
                params.insert("name", wallet_name.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("export_path", path_str);
                params.insert("export_key", "other_key".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn import_works_for_duplicate_name() {
            let ctx = setup();

            create_and_open_wallet(&ctx);

            let (_, path_str) = export_wallet_path();
            export_wallet(&ctx, &path_str);

            // import wallet
            {
                let cmd = import_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }

            close_and_delete_wallet(&ctx);
            tear_down();
        }

        #[test]
        pub fn import_works_for_config() {
            let ctx = setup();

            create_and_open_wallet(&ctx);
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_MY1);

            let (_, path_str) = export_wallet_path();
            export_wallet(&ctx, &path_str);
            close_and_delete_wallet(&ctx);

            let config = r#"{"key":"value"}"#;

            // import wallet
            {
                let cmd = import_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                params.insert("export_key", EXPORT_KEY.to_string());
                params.insert("storage_config", config.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["storage_config"].as_object().unwrap(),
                       serde_json::from_str::<serde_json::Value>(config).unwrap().as_object().unwrap());

            tear_down();
        }

        #[test]
        pub fn import_works_for_different_key_derivation_methods() {
            let ctx = setup();

            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("key_derivation_method", "argon2i".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("key_derivation_method", "argon2i".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            new_did(&ctx, SEED_MY1);
            use_did(&ctx, DID_MY1);

            let (_, path_str) = export_wallet_path();

            {
                let cmd = export_command::new();
                let mut params = CommandParams::new();
                params.insert("export_path", path_str.clone());
                params.insert("export_key", EXPORT_KEY.to_string());
                params.insert("key_derivation_method", "argon2m".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallet_name = "imported_wallet";
            let key = "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw";
            // import wallet
            {
                let cmd = import_command::new();
                let mut params = CommandParams::new();
                params.insert("name", wallet_name.to_string());
                params.insert("key", key.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            // open exported wallet
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", wallet_name.to_string());
                params.insert("key", key.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            use_did(&ctx, DID_MY1);

            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }

            // delete first wallet
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("key_derivation_method", "argon2i".to_string());
                cmd.execute(&CommandContext::new(), &params).unwrap();
            }

            // delete second wallet
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", wallet_name.to_string());
                params.insert("key", key.to_string());
                params.insert("key_derivation_method", "raw".to_string());
                cmd.execute(&CommandContext::new(), &params).unwrap();
            }

            tear_down();
        }
    }

    pub fn create_wallet(ctx: &CommandContext) {
        let create_cmd = create_command::new();
        let mut params = CommandParams::new();
        params.insert("name", WALLET.to_string());
        params.insert("key", WALLET_KEY_RAW.to_string());
        params.insert("key_derivation_method", "raw".to_string());
        create_cmd.execute(&ctx, &params).unwrap();
    }

    pub fn attach_wallet(ctx: &CommandContext) {
        let create_cmd = attach_command::new();
        let mut params = CommandParams::new();
        params.insert("name", WALLET.to_string());
        create_cmd.execute(&ctx, &params).unwrap();
    }

    pub fn create_and_open_wallet(ctx: &CommandContext) -> i32 {
        {
            let create_cmd = create_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY_RAW.to_string());
            params.insert("key_derivation_method", "raw".to_string());
            create_cmd.execute(&ctx, &params).unwrap();
        }
        {
            let cmd = open_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY_RAW.to_string());
            params.insert("key_derivation_method", "raw".to_string());
            cmd.execute(&ctx, &params).unwrap();
        }

        ensure_opened_wallet_handle(&ctx).unwrap()
    }

    pub fn open_wallet(ctx: &CommandContext) -> i32 {
        {
            let cmd = open_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY_RAW.to_string());
            params.insert("key_derivation_method", "raw".to_string());
            cmd.execute(&ctx, &params).unwrap();
        }

        ensure_opened_wallet_handle(&ctx).unwrap()
    }

    pub fn close_and_delete_wallet(ctx: &CommandContext) {
        {
            let cmd = close_command::new();
            let params = CommandParams::new();
            cmd.execute(&ctx, &params).unwrap();
        }

        {
            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY_RAW.to_string());
            params.insert("key_derivation_method", "raw".to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap();
        }
    }

    pub fn close_wallet(ctx: &CommandContext) {
        {
            let cmd = close_command::new();
            let params = CommandParams::new();
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    pub fn delete_wallet(ctx: &CommandContext) {
        {
            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY_RAW.to_string());
            params.insert("key_derivation_method", "raw".to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    pub fn export_wallet_path() -> (PathBuf, String) {
        let path = EnvironmentUtils::tmp_file_path("export_file");
        (path.clone(), path.to_str().unwrap().to_string())
    }

    pub fn export_wallet(ctx: &CommandContext, path: &str) {
        let cmd = export_command::new();
        let mut params = CommandParams::new();
        params.insert("export_path", path.to_string());
        params.insert("export_key", EXPORT_KEY.to_string());
        cmd.execute(&ctx, &params).unwrap()
    }
}
