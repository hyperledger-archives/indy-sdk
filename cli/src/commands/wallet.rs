use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;
use utils::table::print_list_table;
use libindy::ErrorCode;
use libindy::wallet::Wallet;

use serde_json;
use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("wallet", "Wallet management commands"));
}

pub mod create_command {
    use super::*;

    command!(CommandMetadata::build("create", "Create new wallet with specified name")
                .add_main_param("name", "The name of new wallet")
                .add_required_param("pool_name", "The name of associated Indy pool")
                .add_required_deferred_param("key", "Auth key for the wallet")
                .add_example("wallet create wallet1 pool_name=pool1 key")
                .add_example("wallet create wallet1 pool_name=pool1 key=key")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_name = get_str_param("pool_name", params).map_err(error_err!())?;
        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;

        let credentials: String = json!({ "key": key.clone() }).to_string();

        trace!("Wallet::create_wallet try: name {}, pool_name {}", name, pool_name);

        let res = Wallet::create_wallet(pool_name,
                                        name,
                                        None,
                                        None,
                                        credentials.as_str(),
        );

        trace!("Wallet::create_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been created", name)),
            Err(ErrorCode::WalletAlreadyExistsError) => Err(println_err!("Wallet \"{}\" already exists", name)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Invalid wallet name  \"{}\"", name)),
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

        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let rekey = get_opt_str_param("rekey", params).map_err(error_err!())?;

        let credentials = {
            let mut json = JSONMap::new();

            json.insert("key".to_string(), serde_json::Value::String(key.to_string()));

            update_json_map_opt_key!(json, "rekey", rekey);

            JSONValue::from(json).to_string()
        };

        let res = Ok(())
            .and_then(|_| {
                set_active_did(ctx, None);
                if let Some((handle, name)) = get_opened_wallet(ctx) {
                    match Wallet::close_wallet(handle) {
                        Ok(()) => Ok(println_succ!("Wallet \"{}\" has been closed", name)),
                        Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                    }
                } else {
                    Ok(())
                }
            })
            .and_then(|_| {
                match Wallet::open_wallet(name, None, &credentials) {
                    Ok(handle) => {
                        set_opened_wallet(ctx, Some((handle, name.to_owned())));
                        Ok(println_succ!("Wallet \"{}\" has been opened", name))
                    }
                    Err(err) => {
                        set_opened_wallet(ctx, None);
                        match err {
                            ErrorCode::CommonInvalidStructure => Err(println_err!("Invalid wallet config")),
                            ErrorCode::WalletAlreadyOpenedError => Err(println_err!("Wallet \"{}\" already opened", name)),
                            ErrorCode::WalletAccessFailed => Err(println_err!("Cannot open wallet \"{}\". Invalid key \"{}\" has been provided", name, key)),
                            ErrorCode::WalletNotFoundError => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
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

        let res = match Wallet::list_wallets() {
            Ok(wallets) => {
                let wallets: Vec<serde_json::Value> = serde_json::from_str(&wallets)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                print_list_table(&wallets,
                                 &vec![("name", "Name"),
                                       ("pool_name", "Associated pool name"),
                                       ("type", "Type")],
                                 "There are no wallets");

                if let Some((_, cur_wallet)) = get_opened_wallet(ctx) {
                    println_succ!("Current wallet \"{}\"", cur_wallet);
                }
                Ok(())
            }
            Err(ErrorCode::CommonIOError) => Err(println_succ!("There are no wallets")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

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

        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;

        let credentials: String = json!({ "key": key }).to_string();

        if let Some((_, opened_wallet_name)) = get_opened_wallet(&ctx) {
            // TODO: Indy-Sdk allows delete opened wallet
            if name == opened_wallet_name {
                return Err(println_err!("Wallet {:?} is opened", name));
            }
        }

        let res = match Wallet::delete_wallet(name, credentials.as_str()) {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been deleted", name)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
            Err(ErrorCode::WalletNotFoundError) => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
            Err(ErrorCode::WalletAccessFailed) => Err(println_err!("Cannot delete wallet \"{}\". Invalid key \"{}\" has been provided ", name, key)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod export_command {
    use super::*;

    command!(CommandMetadata::build("export", "Export opened wallet to the file")
                .add_required_param("path", "Path to the export file")
                .add_required_deferred_param("key", "Passphrase used to export key")
                .add_example("wallet export path=/home/indy/export_wallet key=key")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let (wallet_handle, wallet_name) = ensure_opened_wallet(&ctx)?;

        let path = get_str_param("path", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;

        let export_config: String = json!({ "path": path.clone(), "key": key.clone() }).to_string();

        trace!("Wallet::export_wallet try: wallet_name {}, path {}, key {}", wallet_name, path, key);

        let res = Wallet::export_wallet(wallet_handle,
                                        export_config.as_str());

        trace!("Wallet::export_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been exported to the file \"{}\"", wallet_name, path)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Can not export Wallet: Path \"{}\" is invalid or file already exists", path)),
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
                .add_required_param("pool_name", "The name of associated Indy pool")
                .add_required_deferred_param("key", "Auth key for the wallet")
                .add_required_param("file", "Path to the file that contains exported wallet content")
                .add_required_deferred_param("import_key", "Passphrase used to export key")
                .add_example("wallet create wallet1 pool_name=pool1 key file=/home/indy/export_wallet import_key")
                .add_example("wallet create wallet1 pool_name=pool1 key=key file=/home/indy/export_wallet import_key=import_key")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_name = get_str_param("pool_name", params).map_err(error_err!())?;
        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_str_param("key", params).map_err(error_err!())?;
        let file = get_str_param("file", params).map_err(error_err!())?;
        let import_key = get_str_param("import_key", params).map_err(error_err!())?;

        let credentials: String = json!({ "key": key.clone() }).to_string();
        let import_config: String = json!({ "path": file.clone(), "key": import_key.clone() }).to_string();

        trace!("Wallet::import_wallet try: name {}, pool_name {}, file {}", name, pool_name, file);

        let res = Wallet::import_wallet(pool_name,
                                        name,
                                        None,
                                        None,
                                        credentials.as_str(),
                                        import_config.as_str()
        );

        trace!("Wallet::import_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been created", name)),
            Err(ErrorCode::WalletAlreadyExistsError) => Err(println_err!("Wallet \"{}\" already exists", name)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Can not import Wallet from file: \"{}\"", file)),
            Err(ErrorCode::CommonInvalidStructure) => Err(println_err!("Can not import Wallet: Invalid file format or encryption key")),
            Err(err) => return Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use utils::test::TestUtils;
    use utils::environment::EnvironmentUtils;
    use libindy::wallet::Wallet;
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
                params.insert("pool_name", POOL.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = get_wallets();
            assert_eq!(1, wallets.len());

            assert_eq!(wallets[0]["name"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["pool_name"].as_str().unwrap(), POOL);

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
                params.insert("pool_name", POOL.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn create_works_for_missed_pool_name() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("key", WALLET_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
        }

        #[test]
        pub fn create_works_for_missed_credentials() {
            let ctx = CommandContext::new();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("pool_name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
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
            let wallets = get_wallets();
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
                params.insert("path", path_str);
                params.insert("key", EXPORT_KEY.to_string());
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
                params.insert("path", path_str);
                params.insert("key", EXPORT_KEY.to_string());
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
                params.insert("pool_name", POOL.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("file", path_str);
                params.insert("import_key", EXPORT_KEY.to_string());
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
                params.insert("pool_name", POOL.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("file", path_str);
                params.insert("import_key", EXPORT_KEY.to_string());
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
                params.insert("pool_name", POOL.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("file", path_str);
                params.insert("import_key", "other_key".to_string());
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
                params.insert("pool_name", POOL.to_string());
                params.insert("key", WALLET_KEY.to_string());
                params.insert("file", path_str);
                params.insert("import_key", EXPORT_KEY.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }

            close_and_delete_wallet(&ctx);
            TestUtils::cleanup_storage();
        }
    }

    pub fn create_wallet(ctx: &CommandContext) {
        let create_cmd = create_command::new();
        let mut params = CommandParams::new();
        params.insert("name", WALLET.to_string());
        params.insert("pool_name", POOL.to_string());
        params.insert("key", WALLET_KEY.to_string());
        create_cmd.execute(&ctx, &params).unwrap();
    }

    pub fn create_and_open_wallet(ctx: &CommandContext) -> i32 {
        {
            let create_cmd = create_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("pool_name", POOL.to_string());
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

    fn get_wallets() -> Vec<serde_json::Value> {
        let wallets = Wallet::list_wallets().unwrap();
        serde_json::from_str(&wallets).unwrap()
    }

    pub fn export_wallet_path() -> (PathBuf, String) {
        let path = EnvironmentUtils::tmp_file_path("export_file");
        (path.clone(), path.to_str().unwrap().to_string())
    }

    pub fn export_wallet(ctx: &CommandContext, path: &str) {
        let cmd = export_command::new();
        let mut params = CommandParams::new();
        params.insert("path", path.to_string());
        params.insert("key", EXPORT_KEY.to_string());
        cmd.execute(&ctx, &params).unwrap()
    }
}