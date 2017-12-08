use application_context::ApplicationContext;
use indy_context::IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::{get_opt_int_param, get_str_param, get_opt_str_param};
use utils::table::print_table;
use libindy::ErrorCode;
use libindy::wallet::Wallet;

use serde_json;
use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use std::collections::HashMap;
use std::rc::Rc;

pub struct Group {
    metadata: GroupMetadata
}

impl Group {
    pub fn new() -> Group {
        Group {
            metadata: GroupMetadata::new("wallet", "Wallet management commands")
        }
    }
}

impl GroupTrait for Group {
    fn metadata(&self) -> &GroupMetadata {
        &self.metadata
    }
}

pub mod CreateCommand {
    use super::*;

    command_without_ctx!(CommandMetadata::build("create", "Create new wallet with specified name")
                .add_main_param("name", "The name of new wallet")
                .add_param("pool_name", false, "The name of associated Indy pool")
                .add_param("key", true, "Auth key for the wallet")
                .finalize()
    );

    fn execute(params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("CreateCommand::execute >> params {:?}", params);

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

        trace!("CreateCommand::execute << {:?}", res);
        res
    }
}

pub mod OpenCommand {
    use super::*;

    command_with_indy_and_indy_ctx!(CommandMetadata::build("open", "Open wallet with specified name. Also close previously opened.")
                .add_main_param("name", "The name of wallet")
                .add_param("key", true, "Auth key for the wallet")
                .add_param("rekey", true, "New auth key for the wallet (will replace previous one).")
                .add_param("freshness_time", true, "Freshness time for entities in the wallet")
                .finalize()
    );

    fn execute(app_ctx: Rc<ApplicationContext>, indy_ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("OpenCommand::execute >> app_ctx {:?} indy_ctx {:?} params {:?}", app_ctx, indy_ctx, params);

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
                if let Some((name, handle)) = indy_ctx.get_opened_wallet() {
                    match Wallet::close_wallet(handle) {
                        Ok(()) => {
                            app_ctx.unset_sub_prompt(2);
                            indy_ctx.unset_opened_wallet();
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
                        app_ctx.set_sub_prompt(2, &format!("wallet({})", name));
                        indy_ctx.set_opened_wallet(name, handle);
                        Ok(println_succ!("Wallet \"{}\" has been opened", name))
                    }
                    Err(ErrorCode::WalletAlreadyOpenedError) => Err(println_err!("Wallet \"{}\" already opened", name)),
                    Err(ErrorCode::CommonIOError) => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                }
            });

        trace!("CreateCommand::execute << {:?}", res);
        Ok(())
    }
}

pub mod ListCommand {
    use super::*;

    command_with_indy_ctx!(CommandMetadata::build("list", "List existing wallets.")
                .finalize()
    );

    fn execute(indy_ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("ListCommand::execute >> indy_ctx {:?} params {:?}", indy_ctx, params);

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

                if let Some(cur_wallet) = indy_ctx.get_opened_wallet_name() {
                    println_succ!("Current wallet \"{}\"", cur_wallet);
                }
                Ok(())
            }
            Err(ErrorCode::CommonIOError) => Err(println_succ!("There are no wallets")),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("ListCommand::execute << {:?}", res);
        res
    }
}

pub mod CloseCommand {
    use super::*;

    command_with_indy_and_indy_ctx!(CommandMetadata::build("close", "Close opened wallet.")
                .finalize()
    );

    fn execute(app_ctx: Rc<ApplicationContext>, indy_ctx: Rc<IndyContext>, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("OpenCommand::execute >> app_ctx {:?} indy_ctx {:?} params {:?}", app_ctx, indy_ctx, params);

        let res = Ok(())
            .and_then(|_| {
                if let Some(wallet) = indy_ctx.get_opened_wallet() {
                    Ok(wallet)
                } else {
                    Err(println_err!("There is no opened wallet now"))
                }
            })
            .and_then(|wallet| {
                let (name, handle) = wallet;
                match Wallet::close_wallet(handle) {
                    Ok(()) => {
                        app_ctx.unset_sub_prompt(2);
                        indy_ctx.unset_opened_wallet();
                        Ok(println_succ!("Wallet \"{}\" has been closed", name))
                    }
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                }
            });

        trace!("CloseCommand::execute << {:?}", res);
        res
    }
}

pub mod DeleteCommand {
    use super::*;

    command_without_ctx!(CommandMetadata::build("delete", "Delete wallet with specified name")
                .add_main_param("name", "The name of deleted wallet")
                .finalize()
    );

    fn execute(params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("DeleteCommand::execute >> params {:?}", params);

        let name = get_str_param("name", params).map_err(error_err!())?;

        let res = match Wallet::delete_wallet(name) {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been deleted", name)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("DeleteCommand::execute << {:?}", res);
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
            let ctx = Rc::new(IndyContext::new());
            let cmd = CreateCommand::new(ctx);
            cmd.metadata().help();
            let mut params = HashMap::new();
            params.insert("name", "wallet");
            params.insert("pool_name", "pool");
            cmd.execute(&params).unwrap();
            TestUtils::cleanup_storage();
        }
    }

    mod open {
        use super::*;

        #[test]
        pub fn exec_works() {
            TestUtils::cleanup_storage();
            let app_ctx = Rc::new(ApplicationContext::new());
            let indy_ctx = Rc::new(IndyContext::new());

            let cmd = OpenCommand::new(app_ctx, indy_ctx);
            let mut params = HashMap::new();
            cmd.metadata().help();
            params.insert("name", "wallet");

            cmd.execute(&params).unwrap_err(); //open not created wallet

            TestUtils::cleanup_storage();
        }

        //TODO add open_for_created_works
    }

    mod close {
        use super::*;

        #[test]
        pub fn exec_for_opened_works() {
            TestUtils::cleanup_storage();
            let app_ctx = Rc::new(ApplicationContext::new());
            let indy_ctx = Rc::new(IndyContext::new());

            {
                let cmd = CreateCommand::new(indy_ctx.clone());
                let mut params = HashMap::new();
                params.insert("name", "wallet");
                params.insert("pool_name", "pool");
                cmd.execute(&params).unwrap();
            }
            {
                let cmd = OpenCommand::new(app_ctx.clone(), indy_ctx.clone());
                let mut params = HashMap::new();
                params.insert("name", "wallet");
                cmd.execute(&params).unwrap();
            }

            let cmd = CloseCommand::new(app_ctx, indy_ctx);
            let params = HashMap::new();
            cmd.execute(&params).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}