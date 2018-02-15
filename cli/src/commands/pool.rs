extern crate serde_json;

use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;

use libindy::ErrorCode;
use libindy::pool::Pool;
use utils::table::print_list_table;

pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("pool", "Pool management commands"));
}

pub mod create_command {
    use super::*;

    command!(CommandMetadata::build("create", "Create new pool ledger config with specified name")
                .add_main_param("name", "The name of new pool ledger config")
                .add_required_param("gen_txn_file", "Path to file with genesis transactions")
                .add_example("pool create pool1 gen_txn_file=/home/pool_genesis_transactions")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;
        let gen_txn_file = get_str_param("gen_txn_file", params).map_err(error_err!())?;

        let config: String = json!({ "genesis_txn": gen_txn_file }).to_string();

        trace!(r#"Pool::create_pool_ledger_config try: name {}, config {:?}"#, name, config);

        let res = Pool::create_pool_ledger_config(name,
                                                  config.as_str());

        trace!(r#"Pool::create_pool_ledger_config return: {:?}"#, res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Pool config \"{}\" has been created", name)),
            Err(ErrorCode::CommonInvalidStructure) => Err(println_err!("Pool genesis file is invalid or does not exist.")),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Pool genesis file is invalid or does not exist.")),
            Err(ErrorCode::PoolLedgerConfigAlreadyExistsError) => Err(println_err!("Pool config \"{}\" already exists", name)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod connect_command {
    use super::*;

    command_with_cleanup!(CommandMetadata::build("connect", "Connect to pool with specified name. Also disconnect from previously connected.")
                .add_main_param("name", "The name of pool")
                .add_example("pool connect pool1")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;

        let res = Ok(())
            .and_then(|_| {
                if let Some((handle, name)) = get_connected_pool(ctx) {
                    match Pool::close(handle) {
                        Ok(()) => {
                            set_connected_pool(ctx, None);
                            Ok(println_succ!("Pool \"{}\" has been disconnected", name))
                        }
                        Err(ErrorCode::PoolLedgerTerminated) => Err(println_err!("Pool \"{}\" does not exist.", name)),
                        Err(ErrorCode::CommonInvalidStructure) => Err(println_err!("Invalid pool ledger config.")),
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
                    Err(ErrorCode::CommonIOError) => Err(println_err!("Pool \"{}\" does not exist.", name)),
                    Err(ErrorCode::PoolLedgerTerminated) => Err(println_err!("Pool \"{}\" does not exist.", name)),
                    Err(ErrorCode::PoolLedgerTimeout) => Err(println_err!("Pool \"{}\" has not been connected.", name)),
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                }
            });

        trace!("execute << {:?}", res);
        res
    }

    pub fn cleanup(ctx: &CommandContext) {
        trace!("cleanup >> ctx {:?}", ctx);

        if let Some((handle, name)) = get_connected_pool(ctx) {
            match Pool::close(handle) {
                Ok(()) => {
                    set_connected_pool(ctx, Some((handle, name.to_owned())));
                    println_succ!("Pool \"{}\" has been connected", name)
                }
                Err(err) => println_err!("Indy SDK error occurred {:?}", err),
            }
        }

        trace!("cleanup <<");
    }
}

pub mod list_command {
    use super::*;

    command!(CommandMetadata::build("list", "List existing pool configs.")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let res = match Pool::list() {
            Ok(pools) => {
                trace!("pools {:?}", pools);
                let pools: Vec<serde_json::Value> = serde_json::from_str(&pools)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                print_list_table(&pools, &vec![("pool", "Pool")], "There are no pool");

                if let Some((_, cur_pool)) = get_connected_pool(ctx) {
                    println_succ!("Current pool \"{}\"", cur_pool);
                }

                Ok(())
            }
            Err(ErrorCode::CommonIOError) => Err(println_succ!("There is no opened pool now")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod disconnect_command {
    use super::*;

    command!(CommandMetadata::build("disconnect", "Disconnect from current pool.")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

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

        trace!("execute << {:?}", res);
        res
    }
}

pub mod delete_command {
    use super::*;

    command!(CommandMetadata::build("delete", "Delete pool config with specified name")
                .add_main_param("name", "The name of deleted pool config")
                .add_example("pool delete pool1")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let name = get_str_param("name", params).map_err(error_err!())?;

        trace!(r#"Pool::delete try: name {}"#, name);

        let res = Pool::delete(name);

        trace!(r#"Pool::delete return: {:?}"#, res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Pool \"{}\" has been deleted.", name)),
            Err(ErrorCode::CommonInvalidState) => Err(println_err!("Connected pool cannot be deleted. Please disconnect first.")),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Pool \"{}\" does not exist.", name)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("execute << {:?}", res);
        res
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use libindy::pool::Pool;

    const POOL: &'static str = "pool";

    mod create {
        use super::*;

        #[test]
        pub fn create_works() {
            let ctx = CommandContext::new();

            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                params.insert("gen_txn_file", "docker_pool_transactions_genesis".to_string());
                cmd.execute(&ctx, &params).unwrap();
            }

            let pools = get_pools();
            assert_eq!(1, pools.len());
            assert_eq!(pools[0]["pool"].as_str().unwrap(), POOL);

            delete_pool(&ctx)
        }

        #[test]
        pub fn create_works_for_twice() {
            let ctx = CommandContext::new();

            create_pool(&ctx);
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                params.insert("gen_txn_file", "docker_pool_transactions_genesis".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_pool(&ctx)
        }

        #[test]
        pub fn create_works_for_missed_gen_txn_file() {
            let ctx = CommandContext::new();

            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
        }

        #[test]
        pub fn create_works_for_unknow_txn_file() {
            let ctx = CommandContext::new();

            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                params.insert("gen_txn_file", "unknow_pool_transactions_genesis".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
        }
    }

    mod connect {
        use super::*;

        #[test]
        pub fn connect_works() {
            let ctx = CommandContext::new();

            create_pool(&ctx);
            {
                let cmd = connect_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_connected_pool_handle(&ctx).unwrap();
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn connect_works_for_twice() {
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            {
                let cmd = connect_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            disconnect_and_delete_pool(&ctx);
        }

        #[test]
        pub fn connect_works_for_not_created() {
            let cmd = connect_command::new();
            let mut params = CommandParams::new();
            params.insert("name", POOL.to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap_err();
        }
    }

    mod list {
        use super::*;

        #[test]
        pub fn list_works() {
            let ctx = CommandContext::new();

            create_pool(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            delete_pool(&ctx);
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

    mod disconnect {
        use super::*;

        #[test]
        pub fn disconnect_works() {
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            {
                let cmd = disconnect_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            ensure_connected_pool_handle(&ctx).unwrap_err();
            delete_pool(&ctx);
        }

        #[test]
        pub fn disconnect_works_for_not_opened() {
            let ctx = CommandContext::new();

            create_pool(&ctx);
            {
                let cmd = disconnect_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_pool(&ctx);
        }

        #[test]
        pub fn disconnect_works_for_twice() {
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            {
                let cmd = disconnect_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            {
                let cmd = disconnect_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            delete_pool(&ctx);
        }
    }

    mod delete {
        use super::*;

        #[test]
        pub fn delete_works() {
            let ctx = CommandContext::new();

            create_pool(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let pools = get_pools();
            assert_eq!(0, pools.len());
        }

        #[test]
        pub fn delete_works_for_not_created() {
            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", POOL.to_string());
            cmd.execute(&CommandContext::new(), &params).unwrap_err();
        }


        #[test]
        pub fn delete_works_for_connected() {
            let ctx = CommandContext::new();

            create_and_connect_pool(&ctx);
            {
                let cmd = delete_command::new();
                let mut params = CommandParams::new();
                params.insert("name", POOL.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            disconnect_and_delete_pool(&ctx);
        }
    }

    pub fn create_pool(ctx: &CommandContext) {
        let cmd = create_command::new();
        let mut params = CommandParams::new();
        params.insert("name", POOL.to_string());
        params.insert("gen_txn_file", "docker_pool_transactions_genesis".to_string());
        cmd.execute(&ctx, &params).unwrap();
    }


    pub fn create_and_connect_pool(ctx: &CommandContext) {
        {
            let cmd = create_command::new();
            let mut params = CommandParams::new();
            params.insert("name", POOL.to_string());
            params.insert("gen_txn_file", "docker_pool_transactions_genesis".to_string());
            cmd.execute(&ctx, &params).unwrap();
        }

        {
            let cmd = connect_command::new();
            let mut params = CommandParams::new();
            params.insert("name", POOL.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    pub fn delete_pool(ctx: &CommandContext) {
        let cmd = delete_command::new();
        let mut params = CommandParams::new();
        params.insert("name", POOL.to_string());
        cmd.execute(&ctx, &params).unwrap();
    }

    pub fn disconnect_and_delete_pool(ctx: &CommandContext) {
        {
            let cmd = disconnect_command::new();
            let params = CommandParams::new();
            cmd.execute(&ctx, &params).unwrap();
        }

        {
            let cmd = delete_command::new();
            let mut params = CommandParams::new();
            params.insert("name", POOL.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }


    fn get_pools() -> Vec<serde_json::Value> {
        let pools = Pool::list().unwrap();
        serde_json::from_str(&pools).unwrap()
    }
}