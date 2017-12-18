use unescape::unescape;

use std::cell::RefCell;
use std::collections::BTreeMap;
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

#[derive(Debug)]
pub struct CommandContext {
    main_prompt: RefCell<String>,
    sub_prompts: RefCell<BTreeMap<usize, String>>,
    is_exit: RefCell<bool>,
    int_values: RefCell<HashMap<&'static str, i32>>,
    string_values: RefCell<HashMap<&'static str, String>>,
}

#[allow(dead_code)] //FIXME
impl CommandContext {
    pub fn new() -> CommandContext {
        CommandContext {
            main_prompt: RefCell::new("indy".to_owned()),
            sub_prompts: RefCell::new(BTreeMap::new()),
            is_exit: RefCell::new(false),
            int_values: RefCell::new(HashMap::new()),
            string_values: RefCell::new(HashMap::new()),
        }
    }

    pub fn set_main_prompt(&self, prompt: String) {
        *self.main_prompt.borrow_mut() = prompt;
    }

    pub fn set_sub_prompt(&self, pos: usize, value: Option<String>) {
        if let Some(value) = value {
            self.sub_prompts.borrow_mut().insert(pos, value);
        } else {
            self.sub_prompts.borrow_mut().remove(&pos);
        }
    }

    pub fn get_prompt(&self) -> String {
        let mut prompt = String::new();

        for (_key, value) in self.sub_prompts.borrow().iter() {
            prompt.push_str(value);
            prompt.push_str(":");
        }

        prompt.push_str(&self.main_prompt.borrow());
        prompt.push_str("> ");
        prompt
    }

    pub fn set_exit(&self) {
        *self.is_exit.borrow_mut() = true;
    }

    pub fn is_exit(&self) -> bool {
        *self.is_exit.borrow()
    }

    pub fn set_int_value(&self, key: &'static str, value: Option<i32>) {
        if let Some(value) = value {
            self.int_values.borrow_mut().insert(key, value);
        } else {
            self.int_values.borrow_mut().remove(key);
        }
    }

    pub fn get_int_value(&self, key: &'static str) -> Option<i32> {
        self.int_values.borrow().get(key).map(i32::to_owned)
    }

    pub fn set_string_value(&self, key: &'static str, value: Option<String>) {
        if let Some(value) = value {
            self.string_values.borrow_mut().insert(key, value);
        } else {
            self.string_values.borrow_mut().remove(key);
        }
    }

    pub fn get_string_value(&self, key: &'static str) -> Option<String> {
        self.string_values.borrow().get(key).map(String::to_owned)
    }
}

pub type CommandParams = HashMap<&'static str, String>;
pub type CommandResult = Result<(), ()>;
pub type CommandExecute = fn(&CommandContext, &CommandParams) -> CommandResult;
pub type CommandCleanup = fn(&CommandContext) -> ();

pub struct Command {
    metadata: CommandMetadata,
    executor: CommandExecute,
    cleaner: Option<CommandCleanup>,
}

impl Command {
    pub fn new(metadata: CommandMetadata,
               executor: CommandExecute,
               cleaner: Option<CommandCleanup>) -> Command {
        Command {
            metadata,
            executor,
            cleaner,
        }
    }

    pub fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }

    pub fn execute(&self, ctx: &CommandContext, params: &CommandParams) -> CommandResult {
        (self.executor)(ctx, params)
    }

    pub fn cleanup(&self, ctx: &CommandContext) {
        if let Some(cleaner) = self.cleaner {
            (cleaner)(ctx)
        }
    }
}

#[derive(Debug)]
pub struct CommandGroupMetadata {
    name: &'static str,
    help: &'static str,
}

impl CommandGroupMetadata {
    pub fn new(name: &'static str, help: &'static str) -> CommandGroupMetadata {
        CommandGroupMetadata {
            name,
            help
        }
    }
}

impl CommandGroupMetadata {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn help(&self) -> &'static str {
        self.help
    }
}

pub struct CommandGroup {
    metadata: CommandGroupMetadata,
}


impl CommandGroup {
    pub fn new(metadata: CommandGroupMetadata) -> CommandGroup {
        CommandGroup { metadata }
    }

    pub fn metadata(&self) -> &CommandGroupMetadata {
        &self.metadata
    }
}

pub struct CommandExecutor {
    ctx: CommandContext,
    commands: HashMap<&'static str, Command>,
    grouped_commands: HashMap<&'static str, (CommandGroup, HashMap<&'static str, Command>)>,
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
            return Ok(());
        }

        if let Some(&(ref group, ref commands)) = self.grouped_commands.get(cmd) {
            return self._execute_group_command(group, commands, params);
        }

        if let Some(ref command) = self.commands.get(cmd) {
            return self._execute_command(None, command, params);
        }

        println_err!("Unknown group or command \"{}\"", cmd);
        println!("Type \"help\" to display the help");
        Err(())
    }

    pub fn ctx(&self) -> &CommandContext {
        &self.ctx
    }

    pub fn complete(&self, line: &str, word: &str, _start: usize, _end: usize) -> Vec<(String, char)> {
        let mut completes: Vec<(String, char)> = vec![];

        let (cmd, params) = CommandExecutor::_split_first_word(line);

        if cmd == "help" {
            // Top level help, no completion
        } else if let Some(ref command) = self.commands.get(cmd) {
            // Complete command params

            if CommandExecutor::_split_first_word(params).0 == "help" {
                // Command help, no completion
            } else {
                if "help".starts_with(word) {
                    completes.push(("help".to_owned(), ' '));
                }

                if let Some(main_param) = command.metadata().main_param() {
                    if main_param.name().starts_with(word) {
                        completes.push((main_param.name().to_owned(), '='));
                    }
                }

                let param_names: Vec<(String, char)> = command
                    .metadata()
                    .params()
                    .iter()
                    .filter(|param| param.name().starts_with(word))
                    .map(|param| (param.name().to_owned(), '='))
                    .collect();

                completes.extend(param_names);
            }
        } else if let Some(&(ref _group, ref commands)) = self.grouped_commands.get(cmd) {
            let (cmd, params) = CommandExecutor::_split_first_word(params);

            if cmd == "help" {
                // Group help, no completion
            } else if let Some(ref command) = commands.get(cmd) {
                // Complete command params

                if CommandExecutor::_split_first_word(params).0 == "help" {
                    // Command help, no completion
                } else {
                    if "help".starts_with(word) {
                        completes.push(("help".to_owned(), ' '));
                    }

                    if let Some(main_param) = command.metadata().main_param() {
                        if main_param.name().starts_with(word) {
                            completes.push((main_param.name().to_owned(), '='));
                        }
                    }

                    let param_names: Vec<(String, char)> = command
                        .metadata()
                        .params()
                        .iter()
                        .filter(|param| param.name().starts_with(word))
                        .map(|param| (param.name().to_owned(), '='))
                        .collect();

                    completes.extend(param_names);
                }
            } else {
                // Complete group commands

                if "help".starts_with(word) {
                    completes.push(("help".to_owned(), ' '));
                }

                let command_names: Vec<(String, char)> = commands
                    .iter()
                    .filter(|name_meta| name_meta.0.starts_with(word))
                    .map(|name_meta| ((*name_meta.0).to_owned(), ' '))
                    .collect();

                completes.extend(command_names);
            }
        } else {
            // Complete top level commands and groups

            if "help".starts_with(word) {
                completes.push(("help".to_owned(), ' '));
            }

            let command_names: Vec<(String, char)> = self.commands
                .iter()
                .filter(|name_meta| name_meta.0.starts_with(word))
                .map(|name_meta| ((*name_meta.0).to_owned(), ' '))
                .collect();

            let group_names: Vec<(String, char)> = self.grouped_commands
                .iter()
                .filter(|name_meta| name_meta.0.starts_with(word))
                .map(|name_meta| ((*name_meta.0).to_owned(), ' '))
                .collect();

            completes.extend(command_names);
            completes.extend(group_names);
        }

        completes
    }

    fn _execute_group_command(&self, group: &CommandGroup, commands: &HashMap<&'static str, Command>, line: &str) -> Result<(), ()> {
        let (cmd, params) = CommandExecutor::_split_first_word(line);

        if cmd == "help" {
            self._print_group_help(group, commands);
            return Ok(());
        }

        if let Some(ref command) = commands.get(cmd) {
            return self._execute_command(Some(group), command, params);
        }

        println_err!("Unknown command \"{} {}\"", group.metadata().name(), cmd);
        println!("Type \"{} help\" to display the help for \"{}\" group", group.metadata().name(), group.metadata().name());
        Err(())
    }

    fn _execute_command(&self, group: Option<&CommandGroup>, command: &Command, params: &str) -> Result<(), ()> {
        let (main_param, _) = CommandExecutor::_split_first_word(params);

        if main_param == "help" {
            self._print_command_help(group, command);
            return Ok(());
        }

        match CommandExecutor::_parse_params(command.metadata(), params) {
            Ok(ref params) => command.execute(&self.ctx, params),
            Err(ref err) => {
                println_err!("{}", err);
                if group.is_some() {
                    println!("Type \"{} {} help\" to display the help for \"{} {}\" command",
                             group.unwrap().metadata().name(), command.metadata().name(),
                             group.unwrap().metadata().name(), command.metadata().name());
                } else {
                    println!("Type \"{} help\" to display the help for \"{}\" command",
                             command.metadata().name(),
                             command.metadata().name());
                }
                Err(())
            }
        }
    }

    fn _print_help(&self) {
        println_acc!("Hyperledger Indy CLI");
        println!();
        println_acc!("Usage:");
        println!("\t[<command-group>] <command> [[<main-param-name>=]<main-param-value>] [<param_name-1>=<param_value-1>]...[<param_name-n>=<param_value-n>]");
        println!();
        println_acc!("Getting help:");
        println!("\thelp - Display this help");
        println!("\t<command-group> help - Display the help for the specific command group");
        println!("\t[<command-group>] <command> help - Display the help for the specific command");
        println!();
        println_acc!("Command groups are:");

        for (_, &(ref group, _)) in &self.grouped_commands {
            println!("\t{} - {}", group.metadata().name(), group.metadata().help())
        }

        println!();
        println_acc!("Top level commands are:");

        for (_, ref command) in &self.commands {
            println!("\t{} - {}", command.metadata().name(), command.metadata().help())
        }

        println!();
    }

    fn _print_group_help(&self, group: &CommandGroup, commands: &HashMap<&'static str, Command>) {
        println_acc!("Group:");
        println!("\t{} - {}", group.metadata().name(), group.metadata().help());
        println!();
        println_acc!("Usage:");
        println!("\t{} <command> [[<main-param-name>=]<main-param-value>] [<param_name-1>=<param_value-1>]...[<param_name-n>=<param_value-n>]", group.metadata().name());
        println!();
        println_acc!("Getting help:");
        println!("\t{} <command> help - Display the help for the specific command", group.metadata().name());
        println!();
        println_acc!("Group commands are:");

        for (_, ref command) in commands {
            println!("\t{} - {}", command.metadata().name(), command.metadata().help())
        }

        println!();
    }

    fn _print_command_help(&self, group: Option<&CommandGroup>, command: &Command) {
        println_acc!("Command:");

        if let Some(group) = group {
            println!("\t{} {} - {}", group.metadata().name(), command.metadata().name(), command.metadata().help());
        } else {
            println!("\t{} - {}", command.metadata().name(), command.metadata().help());
        }

        println!();
        println!();
        println_acc!("Usage:");

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
        println_acc!("Parameters are:");

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

    fn _parse_params(command: &CommandMetadata, params: &str) -> Result<CommandParams, String> {
        let mut res = CommandParams::new();
        let mut params = params;

        // Read main param
        if let Some(param_metadata) = command.main_param() {
            let (mut param_value, tail) = CommandExecutor::_split_first_word(params);
            params = tail;

            // Check for full param format
            let mut split = param_value.splitn(2, '=');
            if split.next().unwrap_or("") == param_metadata.name() {
                param_value = split.next().unwrap_or("")
            }

            if param_value.is_empty() {
                return Err(format!("No main \"{}\" parameter present", param_metadata.name()));
            }

            if let Some(param_value) = unescape(CommandExecutor::_trim_quotes(param_value)) {
                res.insert(param_metadata.name(), param_value);
            } else {
                return Err(format!("Invalid escape sequence for \"{}\" parameter present", param_metadata.name()));
            }
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

            if let Some(param_metadata) = param_metadata {
                if let Some(param_value) = param_value {
                    if res.contains_key(param_metadata.name()) {
                        return Err(format!("\"{}\" parameter presented multiple times", param_metadata.name()));
                    } else if let Some(param_value) = unescape(CommandExecutor::_trim_quotes(param_value)) {
                        res.insert(param_metadata.name(), param_value);
                    } else {
                        return Err(format!("Invalid escape sequence for \"{}\" parameter present", param_metadata.name()));
                    }
                } else {
                    return Err(format!("No value for \"{}\" parameter present", param_name));
                }
            } else {
                return Err(format!("Unknown \"{}\" parameter present", param_name));
            }
        }

        Ok(res)
    }

    fn _split_first_word(s: &str) -> (&str, &str) {
        let mut is_quote_escape = false;
        let mut is_whitespace_escape = false;
        let s = s.trim();

        for (pos, ch) in s.char_indices() {
            if ch.is_whitespace() && !is_whitespace_escape {
                return (&s[..pos], s[pos..].trim_left());
            }

            if !is_quote_escape && ch == '"' {
                is_whitespace_escape = !is_whitespace_escape;
            }

            is_quote_escape = ch == '\\';
        }

        (s, "")
    }

    fn _trim_quotes(s: &str) -> &str {
        if s.len() > 1 && s.starts_with("\"") && s.ends_with("\"") {
            &s[1..s.len() - 1]
        } else {
            s
        }
    }
}

impl Drop for CommandExecutor {
    fn drop(&mut self) {
        for (_, command) in &self.commands {
            command.cleanup(&self.ctx);
        }

        for (_, commands) in &self.grouped_commands {
            for (_, command) in &commands.1 {
                command.cleanup(&self.ctx);
            }
        }

        println_succ!("Goodbye...");
    }
}

pub struct CommandExecutorBuilder {
    commands: HashMap<&'static str, Command>,
    grouped_commands: HashMap<&'static str, (CommandGroup, HashMap<&'static str, Command>)>,
}

impl CommandExecutorBuilder {
    pub fn add_group(self, group: CommandGroup) -> CommandExecutorGroupBuilder {
        CommandExecutorGroupBuilder {
            commands: self.commands,
            grouped_commands: self.grouped_commands,
            group,
            group_commands: HashMap::new(),
        }
    }

    pub fn add_command(mut self, command: Command) -> CommandExecutorBuilder {
        self.commands.insert(command.metadata().name, command);
        self
    }

    pub fn finalize(self) -> CommandExecutor {
        CommandExecutor {
            ctx: CommandContext::new(),
            commands: self.commands,
            grouped_commands: self.grouped_commands,
        }
    }
}

pub struct CommandExecutorGroupBuilder {
    commands: HashMap<&'static str, Command>,
    grouped_commands: HashMap<&'static str, (CommandGroup, HashMap<&'static str, Command>)>,
    group: CommandGroup,
    group_commands: HashMap<&'static str, Command>,
}

impl CommandExecutorGroupBuilder {
    pub fn add_command(mut self, command: Command) -> CommandExecutorGroupBuilder {
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

    pub mod test_group {
        use super::*;
        command_group!(CommandGroupMetadata::new("test_group", "Test group help"));
    }

    pub mod test_command {
        use super::*;

        command!(CommandMetadata::build("test_command", "Test command help")
                    .add_main_param("main_param", "Main param help")
                    .add_param("param1", false, "Param1 help")
                    .add_param("param2", true, "Param2 help")
                    .finalize());

        fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
            println!("Test comamnd params: ctx {:?} params {:?}", ctx, params);
            Ok(())
        }
    }

    #[test]
    pub fn execute_works() {
        let cmd_executor = CommandExecutor::build()
            .add_group(test_group::new())
            .add_command(test_command::new())
            .finalize_group()
            .add_command(test_command::new())
            .finalize();
        cmd_executor.execute("test_group test_command \"main param\" param1=\"param1 value\" param2=param2-value").unwrap();
    }

    #[test]
    pub fn _trim_quites_works() {
        assert_eq!(CommandExecutor::_trim_quotes(""), "");
        assert_eq!(CommandExecutor::_trim_quotes("\""), "\"");
        assert_eq!(CommandExecutor::_trim_quotes("\"\""), "");
        assert_eq!(CommandExecutor::_trim_quotes("\"123 456\""), "123 456");
    }

    #[test]
    pub fn _unescape_works() {
        assert_eq!(unescape("123\\\"456"), Some("123\"456".to_owned()));
    }
}

