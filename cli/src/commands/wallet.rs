use super::{Command, CommandMetadata, CommandMetadataBuilder};
use super::super::IndyContext;

use std::cell::RefCell;
use std::rc::Rc;

pub struct CreateCommand {
    ctx: Rc<RefCell<IndyContext>>,
    metadata: CommandMetadata,
}

impl CreateCommand {
    pub fn new(ctx: Rc<RefCell<IndyContext>>) -> CreateCommand {
        CreateCommand {
            ctx,
            metadata: CommandMetadataBuilder::new("create", "Create new wallet with specified name")
                .add_param("name", false, true, "The name of new wallet")
                .finalize()
        }
    }
}

impl Command for CreateCommand {
    fn execute(&self, line: &Vec<(&str, &str)>) {
        println!("wallet create: >>> execute {:?} while context {:?}", line, self.ctx);
    }

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}


pub struct OpenCommand {
    ctx: Rc<RefCell<IndyContext>>,
    metadata: CommandMetadata,
}

impl OpenCommand {
    pub fn new(ctx: Rc<RefCell<IndyContext>>) -> OpenCommand {
        OpenCommand {
            ctx,
            metadata: CommandMetadataBuilder::new("open", "Open wallet with specified name. Also close previously opened.")
                .add_param("name", false, true, "The name of wallet")
                .finalize()
        }
    }
}

impl Command for OpenCommand {
    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    fn execute(&self, line: &Vec<(&str, &str)>) {
        println!("wallet open: >>> execute {:?} while context {:?}", line, self.ctx);
        self.ctx.borrow_mut().cur_wallet = Some(line[0].1.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod create {
        use super::*;

        #[test]
        pub fn exec_works() {
            let ctx = Rc::new(RefCell::new(IndyContext { cur_wallet: None }));
            let cmd = CreateCommand::new(ctx);
            cmd.metadata().help();
            cmd.execute(&Vec::new());
        }
    }
}