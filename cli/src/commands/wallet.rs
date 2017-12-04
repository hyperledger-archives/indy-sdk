use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use super::super::{IndyContext, serde_json};

use libindy::wallet::Wallet;

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
                .finalize()
        }
    }
}

impl Command for CreateCommand {
    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        println!("wallet create: >>> execute {:?} while context {:?}", params, self.ctx);
        let pool_name = params.get("pool_name").ok_or((/* TODO error */))?;
        let wallet_name = params.get("name").ok_or((/* TODO error */))?;
        let config: Option<String> = params.get("key").map(|key|
            json!({
                "key": key
            }).to_string());
        Wallet::create_wallet(pool_name,
                              wallet_name,
                              None,
                              config.as_ref().map(String::as_str))
            .map_err(|_| ())?;
        Ok(())
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

        let wallet_name = params.get("name").ok_or((/* TODO error */))?;

        let is_config_required = params.contains_key("key")
            || params.contains_key("rekey")
            || params.contains_key("freshness_time");

        let config: Option<String> = if is_config_required {
            let mut config = serde_json::Map::new();

            for str_config_option in &["key", "rekey"] {
                if let Some(config_option) = params.get(str_config_option) {
                    config.insert(config_option.to_string(), serde_json::Value::String(config_option.to_string()));
                }
            }

            if let Some(freshness_time) = params.get("freshness_time") {
                let freshness_time: i64 = freshness_time.parse().map_err(|_| (/* TODO error */))?;
                config.insert("freshness_time".to_string(), serde_json::Value::from(freshness_time));
            }

            Some(serde_json::to_string(&serde_json::Value::from(config)).unwrap())
        } else {
            None
        };

        let wallet_handle = Wallet::open_wallet(wallet_name, config.as_ref().map(String::as_str))
            .map_err(|_| ())?;
        //TODO close previously opened wallet
        self.ctx.set_current_wallet(wallet_name, wallet_handle);

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
        self.ctx.reset_current_wallet();
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