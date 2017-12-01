use super::{Command, CommandMetadata};

use std::collections::HashMap;
use std::rc::Rc;

pub struct CreateCommand {
    cnxt: Rc<String>,
}

impl CreateCommand {
    pub fn new(cnxt: Rc<String>) -> CreateCommand {
        CreateCommand {
            cnxt,
        }
    }
}

impl Command for CreateCommand {
    fn execute(&self, _params: &HashMap<String, String>) {
        println!("wallet create: >>> {:?}", self.cnxt);
    }

    fn metadata(&self) -> &CommandMetadata {
        lazy_static! {
            static ref METADATA: CommandMetadata =
                CommandMetadata::build("create", "Create new wallet with specified name")
                    .add_main_param("name", "The name of new wallet")
                    .finalize();
        }
        &METADATA
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn exec_works() {
        let cnxt = Rc::new("test".to_owned());
        let cmd = CreateCommand::new(cnxt);
        cmd.metadata().help();
        cmd.execute(&HashMap::new());
    }
}