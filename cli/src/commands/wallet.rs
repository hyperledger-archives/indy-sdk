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
                .add_optional_deferred_param("key", "Auth key for the wallet")
                .add_example("wallet create wallet1 pool_name=pool1")
                .add_example("wallet create wallet1 pool_name=pool1 key")
                .add_example("wallet create wallet1 pool_name=pool1 key=AAAAB3NzaC1yc2EA")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_name = get_str_param("pool_name", params).map_err(error_err!())?;
        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_opt_str_param("key", params).map_err(error_err!())?;

        let credentials: Option<String> = key.map(|key| json!({ "key": key }).to_string());

        trace!("Wallet::create_wallet try: name {}, pool_name {}", name, pool_name);

        let res = Wallet::create_wallet(pool_name,
                                        name,
                                        None,
                                        None,
                                        credentials.as_ref().map(String::as_str),
        );

        trace!("Wallet::create_wallet return: {:?}", res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been created", name)),
            Err(ErrorCode::WalletAlreadyExistsError) => Err(println_err!("Wallet \"{}\" already exists", name)),
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
                            .add_optional_deferred_param("key", "Auth key for the wallet")
                            .add_optional_deferred_param("rekey", "New auth key for the wallet (will replace previous one).")
                            .add_example("wallet open wallet1")
                            .add_example("wallet open wallet1 key")
                            .add_example("wallet open wallet1 key rekey")
                            .add_example("wallet open wallet1 key=AAAAB3NzaC1yc2EA rekey=BBBAB3NzaC1AS4AC")
                            .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_opt_str_param("key", params).map_err(error_err!())?;
        let rekey = get_opt_str_param("rekey", params).map_err(error_err!())?;

        let credentials = {
            let mut json = JSONMap::new();

            update_json_map_opt_key!(json, "key", key);
            update_json_map_opt_key!(json, "rekey", rekey);

            if !json.is_empty() {
                Some(JSONValue::from(json).to_string())
            } else {
                None
            }
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
                match Wallet::open_wallet(name, None, credentials.as_ref().map(String::as_str)) {
                    Ok(handle) => {
                        set_opened_wallet(ctx, Some((handle, name.to_owned())));
                        Ok(println_succ!("Wallet \"{}\" has been opened", name))
                    }
                    Err(err) => {
                        set_opened_wallet(ctx, None);
                        match err {
                            ErrorCode::CommonInvalidStructure => Err(println_err!("Invalid wallet config")),
                            ErrorCode::WalletAlreadyOpenedError => Err(println_err!("Wallet \"{}\" already opened", name)),
                            ErrorCode::WalletAccessFailed => Err(println_err!("Cannot open encrypted wallet \"{}\"", name)),
                            ErrorCode::CommonIOError => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
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
                                       ("associated_pool_name", "Associated pool name"),
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
                .add_example("wallet delete wallet1")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx: {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;

        if let Some((_, opened_wallet_name)) = get_opened_wallet(&ctx) {
            // TODO: Indy-Sdk allows delete opened wallet
            if name == opened_wallet_name {
                return Err(println_err!("Wallet {:?} is opened", name));
            }
        }

        let res = match Wallet::delete_wallet(name) {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been deleted", name)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use libindy::wallet::Wallet;

    const WALLET: &'static str = "wallet";
    const POOL: &'static str = "pool";

    mod create {
        use super::*;

        #[test]
        pub fn create_works() {
            let ctx = CommandContext::new();

            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("pool_name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let wallets = get_wallets();
            assert_eq!(1, wallets.len());
            assert_eq!(wallets[0]["name"].as_str().unwrap(), WALLET);
            assert_eq!(wallets[0]["associated_pool_name"].as_str().unwrap(), POOL);

            delete_wallet(&ctx);
        }

        #[test]
        pub fn create_works_for_twice() {
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                params.insert("pool_name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
        }

        #[test]
        pub fn create_works_for_missed_pool_name() {
            let ctx = CommandContext::new();

            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
        }
    }

    mod open {
        use super::*;

        #[test]
        pub fn open_works() {
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_opened_wallet_handle(&ctx).unwrap();
            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn open_works_for_twice() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap(); //TODO: we close and open same wallet
            }
            close_and_delete_wallet(&ctx);
        }

        #[test]
        pub fn open_works_for_not_created() {
            let cmd = open_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap_err();
        }
    }

    mod list {
        use super::*;

        #[test]
        pub fn list_works() {
            let ctx = CommandContext::new();
            create_wallet(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            delete_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_empty_list() {
            let ctx = CommandContext::new();
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
        }
    }

    mod close {
        use super::*;

        #[test]
        pub fn close_works() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_opened_wallet_handle(&ctx).unwrap_err();
            delete_wallet(&ctx);
        }

        #[test]
        pub fn close_works_for_not_opened() {
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_wallet(&ctx);
        }

        #[test]
        pub fn close_works_for_twice() {
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
        }
    }

    mod delete {
        use super::*;

        #[test]
        pub fn delete_works() {
            let ctx = CommandContext::new();

            create_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&CommandContext::new(), &params).unwrap();
            }
            let wallets = get_wallets();
            assert_eq!(0, wallets.len());
        }

        #[test]
        pub fn delete_works_for_not_created() {
            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap_err();
        }

        #[test]
        pub fn delete_works_for_opened() {
            let ctx = CommandContext::new();

            create_and_open_wallet(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", WALLET.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            close_and_delete_wallet(&ctx);
        }
    }

    pub fn create_wallet(ctx: &CommandContext) {
        let create_cmd = create_command::new();
        let mut params = CommandParams::new();
        params.insert("name", WALLET.to_string());
        params.insert("pool_name", POOL.to_string());

        create_cmd.execute(&ctx, &params).unwrap();
    }

    pub fn create_and_open_wallet(ctx: &CommandContext) -> i32 {
        {
            let create_cmd = create_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            params.insert("pool_name", POOL.to_string());
            create_cmd.execute(&ctx, &params).unwrap();
        }
        {
            let cmd = open_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
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
            cmd.execute(&CommandContext::new(), &params).unwrap();
        }
    }

    pub fn delete_wallet(ctx: &CommandContext) {
        {
            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", WALLET.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    fn get_wallets() -> Vec<serde_json::Value> {
        let wallets = Wallet::list_wallets().unwrap();
        serde_json::from_str(&wallets).unwrap()
    }
}