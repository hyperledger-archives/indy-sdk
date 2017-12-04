use command_executor::{Command, CommandMetadata, Group as GroupTrait, GroupMetadata};
use super::super::IndyContext;

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
        let config = params.get("config").map(|cfg| *cfg);
        let wallet_handle = Wallet::open_wallet(wallet_name, config)
            .map_err(|_| ())?;
        //TODO close previously opened wallet
        self.ctx.set_current_wallet(wallet_name, wallet_handle);
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
    }
}