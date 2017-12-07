extern crate serde_json;

use indy_context::IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::{get_opt_int_param, get_str_param, get_opt_str_param};

use libindy::ErrorCode;
use libindy::wallet::Wallet;

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

#[derive(Debug)]
pub struct CreateCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct OpenCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct ListCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct CloseCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

#[derive(Debug)]
pub struct DeleteCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}


impl CreateCommand {
    pub fn new(ctx: Rc<IndyContext>) -> CreateCommand {
        CreateCommand {
            ctx,
            metadata: CommandMetadata::build("create", "Create new wallet with specified name")
                .add_main_param("name", "The name of new wallet")
                .add_param("pool_name", false, "The name of associated Indy pool")
                .add_param("key", true, "Auth key for the wallet")
                .finalize()
        }
    }
}

impl Command for CreateCommand {
    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("CreateCommand::execute >> self {:?} params {:?}", self, params);

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

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}

impl OpenCommand {
    pub fn new(ctx: Rc<IndyContext>) -> OpenCommand {
        OpenCommand {
            ctx,
            metadata: CommandMetadata::build("open", "Open wallet with specified name. Also close previously opened.")
                .add_main_param("name", "The name of wallet")
                .add_param("key", true, "Auth key for the wallet")
                .add_param("rekey", true, "New auth key for the wallet (will replace previous one).")
                .add_param("freshness_time", true, "Freshness time for entities in the wallet")
                .finalize()
        }
    }
}

impl Command for OpenCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("OpenCommand::execute >> self {:?} params {:?}", self, params);

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
                if let Some((name, handle)) = self.ctx.get_opened_wallet() {
                    match Wallet::close_wallet(handle) {
                        Ok(()) => {
                            self.ctx.unset_opened_wallet();
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
                        self.ctx.set_opened_wallet(name, handle);
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

impl ListCommand {
    pub fn new(ctx: Rc<IndyContext>) -> ListCommand {
        ListCommand {
            ctx,
            metadata: CommandMetadata::build("list", "List existing wallets.")
                .finalize()
        }
    }
}

impl Command for ListCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("ListCommand::execute >> self {:?} params {:?}", self, params);

        let res = match Wallet::list_wallets() {
            Ok(wallets) => {
                let wallets: Vec<serde_json::Value> = serde_json::from_str(&wallets)
                    .map_err(|_| println_err!("Wrong data has been received"))?;
                if wallets.len() > 0 {
                    println_acc!("{0: <25} | {1: <25} | {2}", "name", "associated pool name", "type");
                    for wallet in wallets {
                        println!("{0: <25} | {1: <25} | {2}", wallet["name"].as_str().unwrap_or("-"),
                                 wallet["associated_pool_name"].as_str().unwrap_or("-"), &wallet["type"].as_str().unwrap_or("-"));
                    }
                } else {
                    println_succ!("There are no wallets");
                }
                if let Some(cur_wallet) = self.ctx.get_opened_wallet_name() {
                    println_succ!("Current wallet \"{}\"", cur_wallet);
                }
                Ok(())
            }
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("ListCommand::execute << {:?}", res);
        res
    }
}

impl CloseCommand {
    pub fn new(ctx: Rc<IndyContext>) -> CloseCommand {
        CloseCommand {
            ctx,
            metadata: CommandMetadata::build("close", "Close opened wallet.")
                .finalize()
        }
    }
}

impl Command for CloseCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("OpenCommand::execute >> self {:?} params {:?}", self, params);

        let res = Ok(())
            .and_then(|_| {
                if let Some(wallet) = self.ctx.get_opened_wallet() {
                    Ok(wallet)
                } else {
                    Err(println_err!("There is no opened wallet now"))
                }
            })
            .and_then(|wallet| {
                let (name, handle) = wallet;
                match Wallet::close_wallet(handle) {
                    Ok(()) => {
                        self.ctx.unset_opened_wallet();
                        Ok(println_succ!("Wallet \"{}\" has been closed", name))
                    }
                    Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
                }
            });

        trace!("CloseCommand::execute << {:?}", res);
        res
    }
}

impl DeleteCommand {
    pub fn new(ctx: Rc<IndyContext>) -> DeleteCommand {
        DeleteCommand {
            ctx,
            metadata: CommandMetadata::build("delete", "Delete wallet with specified name")
                .add_main_param("name", "The name of deleted wallet")
                .finalize()
        }
    }
}

impl Command for DeleteCommand {
    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("DeleteCommand::execute >> self {:?} params {:?}", self, params);

        let name = get_str_param("name", params).map_err(error_err!())?;

        let res = match Wallet::delete_wallet(name) {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been deleted", name)),
            Err(ErrorCode::CommonIOError) => Err(println_err!("Wallet \"{}\" not found or unavailable", name)),
            Err(err) => Err(println_err!("Indy SDK error occurred {:?}", err)),
        };

        trace!("DeleteCommand::execute << {:?}", res);
        res
    }

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
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
            let ctx = Rc::new(IndyContext::new());
            let cmd = OpenCommand::new(ctx);
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
            let ctx = Rc::new(IndyContext::new());

            {
                let cmd = CreateCommand::new(ctx.clone());
                let mut params = HashMap::new();
                params.insert("name", "wallet");
                params.insert("pool_name", "pool");
                cmd.execute(&params).unwrap();
            }
            {
                let cmd = OpenCommand::new(ctx.clone());
                let mut params = HashMap::new();
                params.insert("name", "wallet");
                cmd.execute(&params).unwrap();
            }

            let cmd = CloseCommand::new(ctx.clone());
            let params = HashMap::new();
            cmd.execute(&params).unwrap();

            TestUtils::cleanup_storage();
        }
    }
}