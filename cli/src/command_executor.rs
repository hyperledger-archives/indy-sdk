use std::collections::HashMap;

#[derive(Debug)]
pub struct ParamMetadata {
    name: &'static str,
    is_optional: bool,
    help: &'static str,
}

impl ParamMetadata {
    pub fn new(name: &'static str, is_optional: bool, help: &'static str) -> ParamMetadata {
        ParamMetadata {
            name,
            is_optional,
            help
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_optional(&self) -> bool {
        self.is_optional
    }

    pub fn help(&self) -> &'static str {
        self.help
    }
}

#[derive(Debug)]
pub struct CommandMetadata {
    name: &'static str,
    help: &'static str,
    main_param: Option<ParamMetadata>,
    params: Vec<ParamMetadata>,
}

impl CommandMetadata {
    pub fn build(name: &'static str, help: &'static str) -> CommandMetadataBuilder {
        CommandMetadataBuilder {
            name,
            help,
            main_param: None,
            params: Vec::new(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn help(&self) -> &'static str {
        self.help
    }

    pub fn main_param(&self) -> Option<&ParamMetadata> {
        self.main_param.as_ref()
    }

    pub fn params(&self) -> &[ParamMetadata] {
        self.params.as_slice()
    }
}

pub struct CommandMetadataBuilder {
    help: &'static str,
    name: &'static str,
    main_param: Option<ParamMetadata>,
    params: Vec<ParamMetadata>,
}

impl CommandMetadataBuilder {
    pub fn add_main_param(mut self,
                          name: &'static str,
                          help: &'static str) -> CommandMetadataBuilder {
        self.main_param = Some(ParamMetadata::new(name, false, help));
        self
    }

    pub fn add_param(mut self,
                     name: &'static str,
                     is_optional: bool,
                     help: &'static str) -> CommandMetadataBuilder {
        self.params.push(ParamMetadata::new(name, is_optional, help));
        self
    }

    pub fn finalize(self) -> CommandMetadata {
        CommandMetadata {
            name: self.name,
            help: self.help,
            main_param: self.main_param,
            params: self.params,
        }
    }
}

pub trait Command {
    fn metadata(&self) -> &CommandMetadata;
    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()>;
}

#[derive(Debug)]
pub struct GroupMetadata {
    name: &'static str,
    help: &'static str,
}

impl GroupMetadata {
    pub fn new(name: &'static str, help: &'static str) -> GroupMetadata {
        GroupMetadata {
            name,
            help
        }
    }
}

impl GroupMetadata {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn help(&self) -> &'static str {
        self.help
    }
}

pub trait Group {
    fn metadata(&self) -> &GroupMetadata;
}

pub struct CommandExecutor {
    commands: HashMap<&'static str, Box<Command>>,
    grouped_commands: HashMap<&'static str, (Box<Group>, HashMap<&'static str, Box<Command>>)>,
}

impl CommandExecutor {
    pub fn build() -> CommandExecutorBuilder {
        CommandExecutorBuilder {
            commands: HashMap::new(),
            grouped_commands: HashMap::new(),
        }
    }

    pub fn execute(&self, line: &str) -> Result<(), ()> {
        let (cmd, params) = CommandExecutor::_split_first_word(line);

        if cmd == "help" {
            self._print_help();
            return Ok(())
        }

        if let Some(&(ref group, ref commands)) = self.grouped_commands.get(cmd) {
            return self._execute_group_command(group, commands, params);
        }

        if let Some(ref command) = self.commands.get(cmd) {
            return self._execute_command(None, command, params);
        }

        println!("Unknown group or command {}", cmd);
        println!();
        self._print_help();
        Err(())
    }

    fn _execute_group_command(&self, group: &Box<Group>, commands: &HashMap<&'static str, Box<Command>>, line: &str) -> Result<(), ()> {
        let (cmd, params) = CommandExecutor::_split_first_word(line);

        if cmd == "help" {
            self._print_group_help(group, commands);
            return Ok(());
        }

        if let Some(ref command) = commands.get(cmd) {
            return self._execute_command(Some(group), command, params);
        }

        println!("Unknown command {} {}", group.metadata().name(), cmd);
        println!();
        self._print_group_help(group, commands);
        Err(())
    }

    fn _execute_command(&self, group: Option<&Box<Group>>, command: &Box<Command>, params: &str) -> Result<(), ()> {
        let (main_param, _) = CommandExecutor::_split_first_word(params);

        if main_param == "help" {
            self._print_command_help(group, command);
            return Ok(());
        }

        match CommandExecutor::_parse_params(command.metadata(), params) {
            Ok(ref params) => command.execute(params),
            Err(ref err) => {
                println!("{}", err);
                println!();
                self._print_command_help(group, command);
                Err(())
            }
        }
    }

    fn _print_help(&self) {
        println!("Hyperledger Indy CLI");
        println!();
        println!("Usage:");
        println!("\t[<command-group>] <command> [[<main-param-name>=]<main-param-value>] [<param_name-1>=<param_value-1>]...[<param_name-n>=<param_value-n>]");
        println!();
        println!("Getting help:");
        println!("\thelp - Display this help");
        println!("\t<command-group> help - Display the help for the specific command group");
        println!("\t[<command-group>] <command> help - Display the help for the specific command");
        println!();
        println!("Command groups are:");

        for (_, &(ref group, _)) in &self.grouped_commands {
            println!("\t{} - {}", group.metadata().name(), group.metadata().help())
        }

        println!();
        println!("Top level commands are:");

        for (_, ref command) in &self.commands {
            println!("\t{} - {}", command.metadata().name(), command.metadata().help())
        }

        println!();
    }

    fn _print_group_help(&self, group: &Box<Group>, commands: &HashMap<&'static str, Box<Command>>) {
        println!("Group:");
        println!("\t{} - {}", group.metadata().name(), group.metadata().help());
        println!();
        println!("Usage:");
        println!("\t{} <command> [[<main-param-name>=]<main-param-value>] [<param_name-1>=<param_value-1>]...[<param_name-n>=<param_value-n>]", group.metadata().name());
        println!();
        println!("Getting help:");
        println!("\t{} <command> help - Display the help for the specific command", group.metadata().name());
        println!();
        println!("Group commands are:");

        for (_, ref command) in commands {
            println!("\t{} - {}", command.metadata().name(), command.metadata().help())
        }

        println!();
    }

    fn _print_command_help(&self, group: Option<&Box<Group>>, command: &Box<Command>) {
        println!("Command:");

        if let Some(group) = group {
            println!("\t{} {} - {}", group.metadata().name(), command.metadata().name(), command.metadata().help());
        } else {
            println!("\t{} - {}", command.metadata().name(), command.metadata().help());
        }

        println!();
        println!();
        println!("Usage:");

        if let Some(group) = group {
            print!("\t{} {}", group.metadata().name(), command.metadata().name());
        } else {
            print!("\t{}", command.metadata().name());
        }

        if let Some(ref main_param) = command.metadata().main_param() {
            print!(" [{}=]<{}-value>", main_param.name(), main_param.name())
        }

        for param in command.metadata().params() {
            if param.is_optional() {
                print!(" [{}=<{}-value>]", param.name(), param.name())
            } else {
                print!(" {}=<{}-value>", param.name(), param.name())
            }
        }

        println!();
        println!();
        println!("Parameters are:");

        if let Some(ref main_param) = command.metadata().main_param() {
            println!("\t{} - {}", main_param.name(), main_param.help())
        }

        for param in command.metadata().params() {
            print!("\t{} - ", param.name());

            if param.is_optional() {
                print!("(optional) ")
            }

            println!("{}", param.help());
        }

        println!();
    }

    fn _parse_params<'a>(command: &CommandMetadata, params: &'a str) -> Result<HashMap<&'static str, &'a str>, String> {
        let mut res: HashMap<&'static str, &str> = HashMap::new();
        let mut params = params;

        // Read main param
        if let Some(param_metadata) = command.main_param() {
            let (param_value, tail) = CommandExecutor::_split_first_word(params);
            params = tail;

            if param_value.is_empty() {
                return Err(format!("No main {} parameter present", param_metadata.name()));
            }

            res.insert(param_metadata.name(), param_value);
        }

        // Read rest params
        loop {
            let (param, tail) = CommandExecutor::_split_first_word(params);
            params = tail;

            if param.is_empty() {
                break;
            }

            let mut split = param.splitn(2, '=');
            let param_name = split.next().unwrap();
            let param_value = split.next();
            let param_metadata = command.params().iter().find(|p| p.name() == param_name);

            if let Some(param_metadata) = param_metadata  {
                if let Some(param_value) = param_value {
                    res.insert(param_metadata.name(), param_value);

                } else {
                    return Err(format!("No value for {} parameter present", param_name));
                }

            } else {
                return Err(format!("Unknown {} parameter present", param_name));
            }
        }

        Ok(res)
    }

    fn _split_first_word(s: &str) -> (&str, &str) {
        let mut is_whitespace_escape = false;
        let s = s.trim();

        for (pos, ch) in s.char_indices() {
            if ch.is_whitespace() && !is_whitespace_escape {
                return (&s[..pos], s[pos..].trim_left());
            }

            if ch == '"' {
                is_whitespace_escape = !is_whitespace_escape;
            }
        }

        (s, "")
    }
}

pub struct CommandExecutorBuilder {
    commands: HashMap<&'static str, Box<Command>>,
    grouped_commands: HashMap<&'static str, (Box<Group>, HashMap<&'static str, Box<Command>>)>,
}

impl CommandExecutorBuilder {
    pub fn add_group(self, group: Box<Group>) -> CommandExecutorGroupBuilder {
        CommandExecutorGroupBuilder {
            commands: self.commands,
            grouped_commands: self.grouped_commands,
            group,
            group_commands: HashMap::new(),
        }
    }

    pub fn add_command(mut self, command: Box<Command>) -> CommandExecutorBuilder {
        self.commands.insert(command.metadata().name, command);
        self
    }

    pub fn finalize(self) -> CommandExecutor {
        CommandExecutor {
            commands: self.commands,
            grouped_commands: self.grouped_commands,
        }
    }
}

pub struct CommandExecutorGroupBuilder {
    commands: HashMap<&'static str, Box<Command>>,
    grouped_commands: HashMap<&'static str, (Box<Group>, HashMap<&'static str, Box<Command>>)>,
    group: Box<Group>,
    group_commands: HashMap<&'static str, Box<Command>>,
}

impl CommandExecutorGroupBuilder {
    pub fn add_command(mut self, command: Box<Command>) -> CommandExecutorGroupBuilder {
        self.group_commands.insert(command.metadata().name(), command);
        self
    }

    pub fn finalize_group(mut self) -> CommandExecutorBuilder {
        self.grouped_commands.insert(self.group.metadata().name(), (self.group, self.group_commands));
        CommandExecutorBuilder {
            commands: self.commands,
            grouped_commands: self.grouped_commands,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestGroup {
        metadata: GroupMetadata
    }

    impl TestGroup {
        pub fn new() -> TestGroup {
            TestGroup {
                metadata: GroupMetadata::new("test_group", "Test group help")
            }
        }
    }

    impl Group for TestGroup {
        fn metadata(&self) -> &GroupMetadata {
            &self.metadata
        }
    }

    struct TestCommand {
        metadata: CommandMetadata
    }

    impl TestCommand {
        pub fn new() -> TestCommand {
            TestCommand {
                metadata: CommandMetadata::build("test_command", "Test command help")
                    .add_main_param("main_param", "Main param help")
                    .add_param("param1", false, "Param1 help")
                    .add_param("param2", true, "Param2 help")
                    .finalize()
            }
        }
    }

    impl Command for TestCommand {
        fn metadata(&self) -> &CommandMetadata {
            &self.metadata
        }

        fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
            println!("Test comamnd params: {:?}", params);
            Ok(())
        }
    }

    #[test]
    pub fn execute_works() {
        let cmd_executor = CommandExecutor::build()
            .add_group(Box::new(TestGroup::new()))
            .add_command(Box::new(TestCommand::new()))
            .finalize_group()
            .add_command(Box::new(TestCommand::new()))
            .finalize();
        cmd_executor.execute("test_group test_command \"main param\" param1=\"param1 value\" param2=param2-value").unwrap();
    }
}

