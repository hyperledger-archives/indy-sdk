use super::{Command, CommandMetadata, CommandMetadataBuilder};
use super::super::IndyContext;

use libindy::wallet::Wallet;

use std::collections::HashMap;
use std::rc::Rc;

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
            metadata: CommandMetadataBuilder::new("create", "Create new wallet with specified name")
                .add_param("name", false, true, "The name of new wallet")
                .finalize()
        }
    }
}

impl Command for CreateCommand {
    fn execute(&self, line: &[(&str, &str)]) -> Result<(), ()> {
        println!("wallet create: >>> execute {:?} while context {:?}", line, self.ctx);
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
            metadata: CommandMetadataBuilder::new("open", "Open wallet with specified name. Also close previously opened.")
                .add_param("name", false, true, "The name of wallet")
                .finalize()
        }
    }

    fn parse_params<'a>(&self, params: &'a [(&str, &str)]) -> Result<HashMap<String, &'a str>, ()> {
        let mut params_map: HashMap<String, &str> = HashMap::new();
        for param in params {
            params_map.insert(param.0.to_string(), param.1);
        }
        for required_param in self.metadata.params.iter()
            .filter(|p| !p.is_optional()) {
            if !params_map.contains_key(required_param.name) {
                return Err(());
            }
        }
        Ok(params_map)
    }
}

impl Command for OpenCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, line: &[(&str, &str)]) -> Result<(), ()> {
        println!("wallet open: >>> execute {:?} while context {:?}", line, self.ctx);
        let params = self.parse_params(line)?;
        let wallet_name = params.get("name").unwrap();
        let config = params.get("config").map(|cfg| *cfg);
        let wallet_handle = Wallet::open_wallet(wallet_name, config)
            .map_err(|_| ())?;
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
            cmd.execute(&Vec::new()).unwrap();
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
            cmd.metadata().help();
            cmd.execute(&[("name", "wallet")]).unwrap_err(); //open not created wallet
            TestUtils::cleanup_storage();
        }
    }
}