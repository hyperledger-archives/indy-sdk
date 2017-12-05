use IndyContext;
use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use commands::{get_opt_i64_param, get_str_param, get_opt_str_param};

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

pub struct OpenCommand {
    ctx: Rc<IndyContext>,
    metadata: CommandMetadata,
}

pub struct CloseCommand {
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

        let pool_name = get_str_param("pool_name", params).map_err(log_err!())?;
        let name = get_str_param("name", params).map_err(log_err!())?;
        let key = get_opt_str_param("key", params).map_err(log_err!())?;

        let config: Option<String> = key.map(|key| json!({ "key": key }).to_string());

        trace!(r#"Wallet::create_wallet try: name {}, pool_name {}, config {:?}"#, name, pool_name, config);

        let res = Wallet::create_wallet(pool_name,
                                        name,
                                        None,
                                        config.as_ref().map(String::as_str));

        trace!(r#"Wallet::create_wallet return: {:?}"#, res);

        let res = match res {
            Ok(()) => Ok(println_succ!("Wallet \"{}\" has been created", name)),
            Err(ErrorCode::WalletAlreadyExistsError) => Err(println_err!("Wallet \"{}\" already exists", name)),
            Err(err) => Err(println_err!("Wallet \"{}\" create failed with unexpected Indy SDK error {:?}", name, err)),
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
                .finalize()
        }
    }
}

impl Command for OpenCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        println!("wallet open: >>> execute {:?} while context {:?}", params, self.ctx);

        let name = get_str_param("name", params)?;
        let key = get_opt_str_param("key", params)?;
        let rekey = get_opt_str_param("rekey", params)?;
        let freshness_time = get_opt_i64_param("freshness_time", params)?;

        let config = {
            let mut json = JSONMap::new();

            if let Some(key) = key {
                json.insert("key".to_string(), JSONValue::from(key));
            }

            if let Some(rekey) = rekey {
                json.insert("rekey".to_string(), JSONValue::from(rekey));
            }

            if let Some(freshness_time) = freshness_time {
                json.insert("freshness_time".to_string(), JSONValue::from(freshness_time));
            }

            if !json.is_empty() {
                Some(JSONValue::from(json).to_string())
            } else {
                None
            }
        };

        let wallet_handle = Wallet::open_wallet(name, config.as_ref().map(String::as_str))
            .map_err(|_| ())?;
        //TODO close previously opened wallet
        self.ctx.set_opened_wallet(name, wallet_handle);

        Ok(())
    }
}

impl CloseCommand {
    pub fn new(ctx: Rc<IndyContext>) -> CloseCommand {
        CloseCommand {
            ctx,
            metadata: CommandMetadata::build("close", "Close wallet with specified handle.")
                .add_main_param("handle", "The handle of wallet")
                .finalize()
        }
    }
}

impl Command for CloseCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        println!("wallet close: >>> execute {:?} while context {:?}", params, self.ctx);
        let wallet_handle = self.ctx.get_current_wallet_handle().ok_or((/* TODO error */))?;
        Wallet::close_wallet(wallet_handle).map_err(|_| ())?;
        self.ctx.unset_opened_wallet();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::test::TestUtils;

    use std::cell::RefCell;

    mod create {
        use super::*;

        #[test]
        pub fn exec_works() {
            TestUtils::cleanup_storage();
            let ctx = Rc::new((IndyContext { cur_wallet: RefCell::new(None) }));
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
            let ctx = Rc::new((IndyContext { cur_wallet: RefCell::new(None) }));
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
            let ctx = Rc::new((IndyContext { cur_wallet: RefCell::new(None) }));

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