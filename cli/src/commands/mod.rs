mod wallet;

use super::IndyContext;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct ParamMetadata {
    name: &'static str,
    is_optional: bool,
    is_main: bool,
    #[allow(dead_code)] //TODO FIXME
    help: &'static str,
}

#[allow(dead_code)] //TODO FIXME
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

#[allow(dead_code)] //TODO FIXME
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

    pub fn main_param_name(&self) -> Option<&str> {
        self.params.iter()
            .find(|param| param.is_main)
            .map(|meta| meta.name)
    }
}

struct CommandMetadataBuilder {
    help: &'static str,
    name: &'static str,
    params: Vec<ParamMetadata>,
    already_has_main: bool,
}

impl CommandMetadataBuilder {
    pub fn new(name: &'static str, help: &'static str) -> CommandMetadataBuilder {
        let params = Vec::new();
        CommandMetadataBuilder {
            name,
            params,
            help,
            already_has_main: false,
        }
    }

    pub fn add_param(mut self,
                     name: &'static str,
                     is_optional: bool,
                     is_main: bool,
                     help: &'static str) -> CommandMetadataBuilder {
        self.params.push(ParamMetadata::new(name, is_optional, is_main, help));
        if is_main {
            assert!(!self.already_has_main);
            self.already_has_main = true;
        }
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
    fn execute(&self, params: &Vec<(&str, &str)>);
}

pub struct CommandExecutor {
    cmds: HashMap<String, HashMap<String, Box<Command>>>
}

impl CommandExecutor {
    pub fn new(indy_context: IndyContext) -> CommandExecutor {
        let ctx = Rc::new(RefCell::new(indy_context));

        let mut wallet_cmds: HashMap<String, Box<Command>> = HashMap::new();
        wallet_cmds.insert("create".to_owned(), Box::new(wallet::CreateCommand::new(ctx.clone())));
        wallet_cmds.insert("open".to_owned(), Box::new(wallet::OpenCommand::new(ctx.clone())));

        let mut cmds = HashMap::new();
        cmds.insert("wallet".to_owned(), wallet_cmds);

        CommandExecutor {
            cmds
        }
    }

    pub fn execute(&self, line: &str) {
        let tokens: Vec<&str> = line.splitn(3, " ").collect();
        assert_eq!(tokens.len(), 3); //TODO
        let cmd_group = if let Some(cmd_group) = self.cmds.get(tokens[0]) {
            cmd_group
        } else {
            println!("Commands group {} is not found.", tokens[0]);
            //TODO print common help
            return;
        };
        let cmd = if let Some(cmd) = cmd_group.get(tokens[1]) {
            cmd
        } else {
            println!("Command {} is not found in group {}.", tokens[1], tokens[0]);
            //TODO print group help
            return;
        };
        let mut params: Vec<(&str, &str)> = Vec::new();
        for param_with_value in tokens[2].split(" ") {
            let param_and_value: Vec<&str> = param_with_value.splitn(2, "=").collect();

            if param_and_value.len() == 1 {
                params.push((cmd.metadata().main_param_name().unwrap(), param_and_value[0]));
            } else {
                params.push((param_and_value[0], param_and_value[1]));
            }
        }

        cmd.execute(&params);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn execute_works() {
        let cmd_executor = CommandExecutor::new(IndyContext { cur_wallet: None });
        cmd_executor.execute("wallet create newWalletName");
    }
}

