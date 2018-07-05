use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;
use utils::table::print_list_table;
use libindy::ErrorCode;
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

    command!(CommandMetadata::build("create", "Create new wallet with specified name")
                .add_main_param("name", "Identifier of the wallet")
                .add_required_deferred_param("key", "Auth key for the wallet")
                .add_optional_param("storage_type", "Type of the wallet storage.")
                .add_optional_param("storage_config", "The list of key:value pairs defined by storage type.")
                .add_example("wallet create wallet1 key")
                .add_example("wallet create wallet1 key=key")
                .add_example("wallet create wallet1 key=key storage_type=default")
                .add_example(r#"wallet create wallet1 key=key storage_type=default storage_config={"key1":"value1","key2":"value2"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let storage_type = get_opt_str_param("storage_type", params).map_err(error_err!())?.unwrap_or("default");
        let storage_config = get_opt_object_param("storage_config", params).map_err(error_err!())?;

        let config: String = json!({ "id": id.clone(), "storage_type": storage_type, "storage_config": storage_config }).to_string();
        let credentials: String = json!({ "key": key.clone() }).to_string();

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
            Err(ErrorCode::WalletAlreadyExistsError) => Err(println_err!("Wallet \"{}\" already exists", id)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Invalid wallet name  \"{}\"", id)),
            Err(err) => return Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod open_command {
    use super::*;

    command_with_cleanup!(CommandMetadata::build("open", "Open wallet with specified name. Also close previously opened.")
                            .add_main_param("name", "The name of wallet")
                            .add_required_deferred_param("key", "Auth key for the wallet")
                            .add_optional_deferred_param("rekey", "New auth key for the wallet (will replace previous one).")
                            .add_example("wallet open wallet1 key")
                            .add_example("wallet open wallet1 key rekey")
                            .add_example("wallet open wallet1 key=key rekey=other_key")
                            .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let rekey = get_opt_str_param("rekey", params).map_err(error_err!())?;

        let config = _read_wallet_config(id)
            .map_err(|_| println_err!("Wallet \"{}\" not found or unavailable", id))?;

        let credentials = {
            let mut json = JSONMap::new();

            json.insert("key".to_string(), serde_json::Value::String(key.to_string()));

            update_json_map_opt_key!(json, "rekey", rekey);

            JSONValue::from(json).to_string()
        };

        let res = Ok(())
            .and_then(|_| {
                set_active_did(ctx, None);
                if let Some((handle, id)) = get_opened_wallet(ctx) {
                    match Wallet::close_wallet(handle) {
                        Ok(()) => Ok(println_succ!("Wallet \"{}\" has been closed", id)),
                        Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
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
                        match err {
                            ErrorCode::CommonInvalidStructure => Err(println_err!("Invalid wallet config")),
                            ErrorCode::WalletAlreadyOpenedError => Err(println_err!("Wallet \"{}\" already opened", id)),
                            ErrorCode::WalletAccessFailed => Err(println_err!("Cannot open wallet \"{}\". Invalid key \"{}\" has been provided", id, key)),
                            ErrorCode::WalletNotFoundError => Err(println_err!("Wallet \"{}\" not found or unavailable", id)),
                            err => Err(println_err!("Indy SDK error occurred {:?}", err)),
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
                Err(err) => println_err!("Indy SDK error occurred {:?}", err),
            }
        }

        trace!("cleanup <<");
    }
}

pub mod list_command {
    use super::*;

    command!(CommandMetadata::build("list", "List existing wallets.")
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
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                }
            });

        trace!("CloseCommand::execute << {:?}", res);
        res
    }
}

pub mod delete_command {
    use super::*;

    command!(CommandMetadata::build("delete", "Delete wallet with specified name")
                .add_main_param("name", "The name of deleted wallet")
                .add_required_deferred_param("key", "Auth key for the wallet")
                .add_example("wallet delete wallet1 key")
                .add_example("wallet delete wallet1 key=key")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx: {:?} params {:?}", ctx, params);

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;

        let config = _read_wallet_config(id)
            .map_err(|_| println_err!("Wallet \"{}\" not found or unavailable", id))?;

        let credentials: String = json!({ "key": key }).to_string();

        if let Some((_, opened_wallet_id)) = get_opened_wallet(&ctx) {
            // TODO: Indy-Sdk allows delete opened wallet
            if id == opened_wallet_id {
                return Err(println_err!("Wallet {:?} is opened", id));
            }
        }

        let res = match Wallet::delete_wallet(config.as_str(), credentials.as_str()) {
            Ok(()) => {
                _delete_wallet_config(id)
                    .map_err(|err| println_err!("Cannot delete \"{}\" config file: {:?}", id, err))?;

                Ok(println_succ!("Wallet \"{}\" has been deleted", id))
            }
            Err(ErrorCode::CommonIOError) => Err(println_err!("Wallet \"{}\" not found or unavailable", id)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Wallet \"{}\" not found or unavailable", id)),
            Err(ErrorCode::WalletAccessFailed) => Err(println_err!("Cannot delete wallet \"{}\". Invalid key \"{}\" has been provided ", id, key)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod export_command {
    use super::*;

    command!(CommandMetadata::build("export", "Export opened wallet to the file")
                .add_required_param("export_path", "Path to the export file")
                .add_required_deferred_param("export_key", "Passphrase used to derive export key")
                .add_example("wallet export export_path=/home/indy/export_wallet export_key=key")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let export_path = get_str_param("export_path", params).map_err(error_err!())?;
        let export_key = get_str_param("export_key", params).map_err(error_err!())?;

        let export_config: String = json!({ "path": export_path.clone(), "key": export_key.clone() }).to_string();

        trace!("Wallet::export_wallet try: wallet_name {}, export_path {}", wallet_name, export_path);

        let res = Wallet::export_wallet(wallet_handle,
                                        export_config.as_str());

        trace!("Wallet::export_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been exported to the file \"{}\"", wallet_name, export_path)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Can not export Wallet: Path \"{}\" is invalid or file already exists", export_path)),
            Err(err) => return Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}


pub mod import_command {
    use super::*;

    command!(CommandMetadata::build("import", "Create new wallet and then import content from the specified file")
                .add_main_param("name", "The name of new wallet")
                .add_required_deferred_param("key", "Auth key for the wallet")
                .add_optional_param("storage_type", "Type of the wallet storage.")
                .add_optional_param("storage_config", "The list of key:value pairs defined by storage type.")
                .add_required_param("export_path", "Path to the file that contains exported wallet content")
                .add_required_deferred_param("export_key", "Passphrase used to derive export key")
                .add_example("wallet import wallet1 key export_path=/home/indy/export_wallet export_key")
                .add_example("wallet import wallet1 key=key export_path=/home/indy/export_wallet export_key=export_key")
                .add_example(r#"wallet import wallet1 key export_path=/home/indy/export_wallet export_key=export_key storage_type=default storage_config={"key1":"value1","key2":"value2"}"#)
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let id = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let export_path = get_str_param("export_path", params).map_err(error_err!())?;
        let export_key = get_str_param("export_key", params).map_err(error_err!())?;
        let storage_type = get_opt_str_param("storage_type", params).map_err(error_err!())?;
        let storage_config = get_opt_object_param("storage_config", params).map_err(error_err!())?;

        let config: String = json!({ "id": id.clone(), "storage_type": storage_type, "storage_config": storage_config }).to_string();
        let credentials: String = json!({ "key": key.clone() }).to_string();
        let import_config: String = json!({ "path": export_path.clone(), "key": export_key.clone() }).to_string();

        trace!("Wallet::import_wallet try: config {}, import_config {}", config, import_config);

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
            Err(ErrorCode::WalletAlreadyExistsError) => Err(println_err!("Wallet \"{}\" already exists", id)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Can not import Wallet from file: \"{}\"", export_path)),
            Err(ErrorCode::CommonInvalidStructure) => Err(println_err!("Can not import Wallet: Invalid file format or encryption key")),
            Err(err) => return Err(println_err!("Indy SDK error occurred {:?}", err)),
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::test::TestUtils;
    use utils::environment::EnvironmentUtils;
    use std::path::PathBuf;

    const WALLET: &'static str = "wallet";
    const POOL: &'static str = "pool";
    const WALLET_KEY: &'static str = "wallet_key";
    const EXPORT_KEY: &'static str = "export_key";

    mod create {
        use super::*;

        #[test]
        pub fn create_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);

            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn create_works_for_twice() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn create_works_for_missed_credentials() {
            let ctx = CommandContext::new();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn create_works_for_type() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            let storage_type = "default";
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("storage_type", storage_type.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["storage_type"].as_str().unwrap(), storage_type);

            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn create_works_for_config() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();
            let config = r#"{"key":"value","key2":"value2"}"#;
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("storage_config", config.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = _list_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["id"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["storage_config"].as_object().unwrap(),
                       serde_json::from_str::<serde_json::Value>(config).unwrap().as_object().unwrap());

            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        pub fn open_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_opened_wallet_handle(&ctx).unwrap();
            close_and_delete_wallet(&ctx);

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn open_works_for_twice() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap(); //TODO: we close and open same wallet
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn open_works_for_not_created() {
            TestUtils::cleanup_storage();

            let cmd = open_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY.to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap_err();

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn open_works_for_missed_key() {
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
        }

        #[test]
        pub fn open_works_for_wrong_key() {
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", "other_key".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
        }
    }

    mod list {
        use super::*;

        #[test]
        pub fn list_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn list_works_for_empty_list() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            TestUtils::cleanup_storage();
        }
    }

    mod close {
        use super::*;

        #[test]
        pub fn close_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_opened_wallet_handle(&ctx).unwrap_err();
            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn close_works_for_not_opened() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn close_works_for_twice() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();
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
            TestUtils::cleanup_storage();
        }
    }

    mod delete {
        use super::*;

        #[test]
        pub fn delete_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&CommandContext::new(), &params).unwrap();
            }
            let wallets = _list_wallets();
            assert_eq!(0, wallets.len());

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn delete_works_for_not_created() {
            TestUtils::cleanup_storage();

            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY.to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap_err();

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn delete_works_for_opened() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn delete_works_for_wrong_key() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", "other_key".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            TestUtils::cleanup_storage();
        }
    }

    mod export {
        use super::*;

        #[test]
        pub fn export_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            let (path, path_str) = export_wallet_path();
            {
                let cmd = export_command::new();
                let mut params = CommandParams::new();
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            assert!(path.exists());

            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn export_works_for_file_already_exists() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            let (_, path_str) = export_wallet_path();

            export_wallet(&ctx, &path_str);
            {
                let cmd = export_command::new();
                let mut params = CommandParams::new();
                params.insert("export_path", path_str);
                params.insert("export_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    mod import {
        use super::*;
        use super::did::tests::{new_did, use_did, SEED_MY1, DID_MY1};

        #[test]
        pub fn import_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

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

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn import_works_for_not_found_file() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

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

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn import_works_for_other_key() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

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
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn import_works_for_duplicate_name() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

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
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn import_works_for_config() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

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

            TestUtils::cleanup_storage();
        }
    }

    pub fn create_wallet(ctx: &CommandContext) {
        let create_cmd = create_command::new();
        let mut params = CommandParams::new();
        params.insert("name", WALLET.to_string());
        params.insert("key", WALLET_KEY.to_string());
        create_cmd.execute(&ctx, &params).unwrap();
    }

    pub fn create_and_open_wallet(ctx: &CommandContext) -> i32 {
        {
            let create_cmd = create_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY.to_string());
            create_cmd.execute(&ctx, &params).unwrap();
        }
        {
            let cmd = open_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY.to_string());
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
            params.insert("key", WALLET_KEY.to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap();
        }
    }

    pub fn delete_wallet(ctx: &CommandContext) {
        {
            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("key", WALLET_KEY.to_string());
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