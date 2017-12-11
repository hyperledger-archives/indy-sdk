extern crate serde_json;

use command_executor::{Command, CommandContext, CommandMetadata, CommandGroup, CommandGroupMetadata};
use commands::*;

use libindy::ErrorCode;
use libindy::pool::Pool;
use utils::table::print_table;

use std::collections::HashMap;

pub mod Group {
    use super::*;

    command_group!(CommandGroupMetadata::new("pool", "Pool management commands"));
}

pub mod CreateCommand {
    use super::*;

    command!(CommandMetadata::build("create", "Create new pool ledger config with specified name")
                .add_main_param("name", "The name of new pool ledger config")
                .add_param("gen_txn_file", true, "Path to file with genesis transactions")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("CreateCommand::execute >> params {:?}", params);

        let name = get_str_param("name", params).map_err(error_err!())?;
        let gen_txn_file = get_opt_str_param("gen_txn_file", params).map_err(error_err!())?
            .unwrap_or("pool_transactions_genesis");

        let config: String = json!({ "genesis_txn": gen_txn_file }).to_string();

        trace!(r#"Pool::create_pool_ledger_config try: name {}, config {:?}"#, name, config);

        let res = Pool::create_pool_ledger_config(name,
                                                  config.as_str());

        trace!(r#"Pool::create_pool_ledger_config return: {:?}"#, res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Pool config \"{}\" has been created", name)),
            Err(ErrorCode::PoolLedgerConfigAlreadyExistsError) => Err(println_err!("Pool config \"{}\" already exists", name)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("CreateCommand::execute << {:?}", res);
        res
    }
}

pub mod ConnectCommand {
    use super::*;

    command!(CommandMetadata::build("connect", "Connect to pool with specified name. Also disconnect from previously connected.")
                .add_main_param("name", "The name of pool")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("OpenCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;

        let res = Ok(())
            .and_then(|_| {
                if let Some((handle, name)) = get_connected_pool(ctx) {
                    match Pool::close(handle) {
                        Ok(()) => {
                            set_connected_pool(ctx, None);
                            Ok(println_succ!("Pool \"{}\" has been disconnected", name))
                        }
                        Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err))
                    }
                } else {
                    Ok(())
                }
            })
            .and_then(|_| {
                match Pool::open_pool_ledger(name, None) {
                    Ok(handle) => {
                        set_connected_pool(ctx, Some((handle, name.to_owned())));
                        Ok(println_succ!("Pool \"{}\" has been connected", name))
                    }
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                }
            });

        trace!("CreateCommand::execute << {:?}", res);
        res
    }
}

pub mod ListCommand {
    use super::*;

    command!(CommandMetadata::build("list", "List existing pool configs.")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("ListCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let res = match Pool::list() {
            Ok(pools) => {
                trace!("pools {:?}", pools);
                let pools: Vec<serde_json::Value> = serde_json::from_str(&pools)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                if pools.len() > 0 {
                    print_table(&pools, &vec![("pool", "Pool")]);
                } else {
                    println_succ!("There are no pool");
                }
                if let Some((_,cur_pool)) = get_connected_pool(ctx) {
                    println_succ!("Current pool \"{}\"", cur_pool);
                }

                Ok(())
            }
            Err(ErrorCode::CommonIOError) => Err(println_succ!("There are no pool")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("ListCommand::execute << {:?}", res);
        res
    }
}

pub mod DisconnectCommand {
    use super::*;

    command!(CommandMetadata::build("disconnect", "Disconnect from current pool.")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("DisconnectCommand::execute >> ctx {:?} params {:?}", ctx, params);

        let (handle, name) = if let Some(pool) = get_connected_pool(ctx) {
            pool
        } else {
            return Err(println_err!("There is no connected pool now"));
        };

        let res = match Pool::close(handle) {
            Ok(()) => {
                set_connected_pool(ctx, None);
                Ok(println_succ!("Pool \"{}\" has been disconnected", name))
            }
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("DisconnectCommand::execute << {:?}", res);
        res
    }
}

pub mod DeleteCommand {
    use super::*;

    command!(CommandMetadata::build("delete", "Delete pool config with specified name")
                .add_main_param("name", "The name of deleted pool config")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("DeleteCommand::execute >> params {:?}", params);

        let name = get_str_param("name", params).map_err(error_err!())?;

        trace!(r#"Pool::delete try: name {}"#, name);

        let res = Pool::delete(name);

        trace!(r#"Pool::delete return: {:?}"#, res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Pool \"{}\" has been deleted", name)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("DeleteCommand::execute << {:?}", res);
        res
    }
}