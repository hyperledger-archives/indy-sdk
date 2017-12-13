use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;
use utils::table::print_table;
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
                .add_param("pool_name", false, "The name of associated Indy pool")
                .add_param("key", true, "Auth key for the wallet")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let pool_name = get_str_param("pool_name", params).map_err(error_err!())?;
        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_opt_str_param("key", params).map_err(error_err!())?;

        let config: Option<String> = key.map(|key| json!({ "key": key }).to_string());

        trace!("Wallet::create_wallet try: name {}, pool_name {}, config {:?}", name, pool_name, config);

        let res = Wallet::create_wallet(pool_name,
                                        name,
                                        None,
                                        config.as_ref().map(String::as_str));

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
                            .add_param("key", true, "Auth key for the wallet")
                            .add_param("rekey", true, "New auth key for the wallet (will replace previous one).")
                            .add_param("freshness_time", true, "Freshness time for entities in the wallet")
                            .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;
        let key = get_opt_str_param("key", params).map_err(error_err!())?;
        let rekey = get_opt_str_param("rekey", params).map_err(error_err!())?;
        let freshness_time = get_opt_int_param::<i64>("freshness_time", params).map_err(error_err!())?;

        let config = {
            let mut json = JSONMap::new();

            update_json_map_opt_key!(json, "key", key);
            update_json_map_opt_key!(json, "rekey", rekey);
            update_json_map_opt_key!(json, "freshness_time", freshness_time);

            if !json.is_empty() {
                Some(JSONValue::from(json).to_string())
            } else {
                None
            }
        };

        let res = Ok(())
            .and_then(|_| {
                if let Some((handle, name)) = get_opened_wallet(ctx) {
                    match Wallet::close_wallet(handle) {
                        Ok(()) => {
                            set_opened_wallet(ctx, Some((handle, name.clone())));
                            Ok(println_succ!("Wallet \"{}\" has been closed", name))
                        }
                        Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                    }
                } else {
                    Ok(())
                }
            })
            .and_then(|_| {
                match Wallet::open_wallet(name, config.as_ref().map(String::as_str)) {
                    Ok(handle) => {
                        set_opened_wallet(ctx, Some((handle, name.to_owned())));
                        Ok(println_succ!("Wallet \"{}\" has been opened", name))
                    }
                    Err(ErrorCode::WalletAlreadyOpenedError) => Err(println_err!("Wallet \"{}\" already opened", name)),
                    Err(ErrorCode::CommonIOError) => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
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

                if wallets.len() > 0 {
                    print_table(&wallets,
                                &vec![("name", "Name"),
                                      ("associated_pool_name", "Associated pool name"),
                                      ("type", "Type")]);
                } else {
                    println_succ!("There are no wallets");
                }

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

    command!(CommandMetadata::build("close", "Close opened wallet.").finalize());

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
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx: {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;

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
mod tests {
    use super::*;
    use utils::test::TestUtils;

    mod create {
        use super::*;

        #[test]
        pub fn exec_works() {
            TestUtils::cleanup_storage();
            let cmd = create_command::new();
            cmd.metadata().help();
            let mut params = CommandParams::new();
            params.insert("name", "wallet".to_owned());
            params.insert("pool_name", "pool".to_owned());
            cmd.execute(&CommandContext::new(), &params).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        pub fn exec_works() {
            TestUtils::cleanup_storage();

            let cmd = open_command::new();
            let mut params = CommandParams::new();
            cmd.metadata().help();
            params.insert("name", "wallet".to_owned());

            cmd.execute(&CommandContext::new(), &params).unwrap_err(); //open not created wallet

            TestUtils::cleanup_storage();
        }

        //TODO add open_for_created_works
    }

    mod close {
        use super::*;

        #[test]
        pub fn exec_for_opened_works() {
            TestUtils::cleanup_storage();

            let ctx = CommandContext::new();

            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "wallet".to_owned());
                params.insert("pool_name", "pool".to_owned());
                cmd.execute(&ctx, &params).unwrap();
            }

            {
                let cmd = open_command::new();
                let mut params = CommandParams::new();
                params.insert("name", "wallet".to_owned());
                cmd.execute(&ctx, &params).unwrap();
            }

            {
                let cmd = close_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }

            TestUtils::cleanup_storage();
        }
    }
}