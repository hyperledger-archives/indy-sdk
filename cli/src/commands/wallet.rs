use super::{Command, CommandMetadata, CommandMetadataBuilder};
use super::super::IndyContext;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct CreateCommand {
    cnxt: Rc<RefCell<IndyContext>>,
    metadata: CommandMetadata,
}

impl CreateCommand {
    pub fn new(cnxt: Rc<RefCell<IndyContext>>) -> CreateCommand {
        CreateCommand {
            cnxt,
            metadata: CommandMetadataBuilder::new("create", "Create new wallet with specified name")
                .add_param("name", false, true, "The name of new wallet")
                .finalize()
        }
    }
}

impl Command for CreateCommand {
    fn execute(&self, line: &str) {
        println!("wallet create: >>> execute {} while context {:?}", line, self.cnxt);
    }

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}


pub struct OpenCommand {
    cnxt: Rc<RefCell<IndyContext>>,
    metadata: CommandMetadata,
}

impl OpenCommand {
    pub fn new(cnxt: Rc<RefCell<IndyContext>>) -> OpenCommand {
        OpenCommand {
            cnxt,
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

    fn execute(&self, line: &str) {
        println!("wallet open: >>> execute {} while context {:?}", line, self.cnxt);
        self.cnxt.borrow_mut().cur_wallet = Some(line.to_string());
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn exec_works() {
        let cnxt = Rc::new("test".to_owned());
        let cmd = CreateCommand::new(cnxt);
        cmd.metadata().help();
        cmd.execute();
    }
}