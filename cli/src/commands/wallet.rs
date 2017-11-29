use super::{Command, CommandMetadata, CommandMetadataBuilder};

use std::collections::HashMap;
use std::rc::Rc;

pub struct CreateCommand {
    cnxt: Rc<String>,
    metadata: CommandMetadata,
}

impl CreateCommand {
    pub fn new(cnxt: Rc<String>) -> CreateCommand {
        CreateCommand {
            cnxt,
            metadata: CommandMetadataBuilder::new("create", "Create new wallet with specified name")
                .add_param("name", false, true, "The name of new wallet")
                .finalize()
        }
    }
}

impl Command for CreateCommand {
    fn execute(&self) {
        println!("wallet create: >>> {:?}", self.cnxt);
    }

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
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