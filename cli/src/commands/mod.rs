mod wallet;

use super::IndyContext;

use std::cell::RefCell;
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
    fn execute(&self, line: &str);
}

pub struct CommandExecutor {
    cmds: HashMap<String, HashMap<String, Box<Command>>>
}

impl CommandExecutor {
    pub fn new(indy_context: IndyContext) -> CommandExecutor {
        let cnxt = Rc::new(RefCell::new(indy_context));

        let mut wallet_cmds: HashMap<String, Box<Command>> = HashMap::new();
        wallet_cmds.insert("create".to_owned(), Box::new(wallet::CreateCommand::new(cnxt.clone())));
        wallet_cmds.insert("open".to_owned(), Box::new(wallet::OpenCommand::new(cnxt.clone())));

        let mut cmds = HashMap::new();
        cmds.insert("wallet".to_owned(), wallet_cmds);

        CommandExecutor {
            cmds
        }
    }

    pub fn execute(&self, line: &str) {
        let tokens: Vec<&str> = line.splitn(3, " ").collect();
        self.cmds.get(tokens[0]).unwrap().get(tokens[1]).unwrap().execute(tokens[2]);
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

