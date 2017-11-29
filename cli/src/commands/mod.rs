mod wallet;

use std::collections::HashMap;
use std::rc::Rc;

struct ParamMetadata {
    name: &'static str,
    is_optional: bool,
    is_main: bool,
    help: &'static str,
}

impl ParamMetadata {
    pub fn new(name: &'static str, is_optional: bool, is_main: bool, help: &'static str) -> ParamMetadata {
        ParamMetadata {
            name,
            is_optional,
            is_main,
            help
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn is_optional(&self) -> bool {
        self.is_optional
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }
}

struct CommandMetadata {
    name: &'static str,
    help: &'static str,
    params: Vec<ParamMetadata>,
}

impl CommandMetadata {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn help(&self) -> &str {
        self.help
    }

    pub fn params(&self) -> &[ParamMetadata] {
        self.params.as_slice()
    }
}

struct CommandMetadataBuilder {
    help: &'static str,
    name: &'static str,
    params: Vec<ParamMetadata>,
}

impl CommandMetadataBuilder {
    pub fn new(name: &'static str, help: &'static str) -> CommandMetadataBuilder {
        let params = Vec::new();
        CommandMetadataBuilder {
            name,
            params,
            help,
        }
    }

    pub fn add_param(mut self,
                    name: &'static str,
                    is_optional: bool,
                    is_main: bool,
                    help: &'static str) -> CommandMetadataBuilder {
        self.params.push(ParamMetadata::new(name, is_optional, is_main, help));
        self
    }

    pub fn finalize(self) -> CommandMetadata {
        CommandMetadata {
            name: self.name,
            help: self.help,
            params: self.params,
        }
    }
}

trait Command {
   fn metadata(&self) -> &CommandMetadata;
   fn execute(&self);
}

pub struct CommandExecutor {
    cmds: HashMap<String, Box<Command>>
}

impl CommandExecutor {

    pub fn new() -> CommandExecutor {
        let cnxt = Rc::new("test".to_owned());

        let mut cmds: HashMap<String, Box<Command>> = HashMap::new();
        cmds.insert("test".to_owned(), Box::new(wallet::CreateCommand::new(cnxt.clone())));

        CommandExecutor {
            cmds
        }
    }

    pub fn execute(&self, line: &str) {
        self.cmds.get("test").unwrap().execute();
    }
}

mod tests {
    use super::*;

    #[test]
    pub fn execute_works() {
        let cmd_executor = CommandExecutor::new();
        cmd_executor.execute("test");
    }
}

