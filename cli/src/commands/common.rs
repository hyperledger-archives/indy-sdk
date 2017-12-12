use command_executor::{Command, CommandContext, CommandExecParams, CommandMetadata, CommandResult};
use commands::get_str_param;

use std::io::Read;
use std::fs::File;

pub mod AboutCommand {
    use super::*;

    command!(CommandMetadata::build("about", "Show about information").finalize());

    fn execute(_ctx: &CommandContext, _params: &CommandExecParams) -> CommandResult {
        trace!("AboutCommand::execute >> self: params: {:?}", _params);

        println_succ!("Hyperledger Indy CLI (https://github.com/hyperledger/indy-sdk)");
        println!();
        println_succ!("This is the official CLI tool for Hyperledger Indy (https://www.hyperledger.org/projects),");
        println_succ!("which provides a distributed-ledger-based foundation for");
        println_succ!("self-sovereign identity (https://sovrin.org/).");
        println!();
        println_succ!("Apache License Version 2.0");
        println_succ!("Copyright 2017 Sovrin Foundation");
        println!();

        let res = Ok(());

        trace!("AboutCommand::execute << {:?}", res);
        res
    }
}

pub mod ShowCommand {
    use super::*;

    command!(CommandMetadata::build("show", "Print the content of text file")
                            .add_main_param("file", "The path to file to show")
                            .finalize());

    fn execute(_ctx: &CommandContext, params: &CommandExecParams) -> CommandResult {
        trace!("ShowCommand::execute >> params: {:?}", params);

        let file = get_str_param("file", params).map_err(error_err!())?;

        let mut file = File::open(file)
            .map_err(error_err!())
            .map_err(map_println_err!("Can't read the file"))?;

        let content = {
            let mut s = String::new();
            file.read_to_string(&mut s)
                .map_err(error_err!())
                .map_err(|err| println_err!("Can't read the file: {}", err))?;
            s
        };

        println!("{}", content);
        let res = Ok(());

        trace!("ShowCommand::execute << {:?}", res);
        res
    }
}

pub mod PromptCommand {
    use super::*;

    command!(CommandMetadata::build("prompt", "Change command prompt")
                            .add_main_param("prompt", "New prompt string")
                            .finalize());

    fn execute(ctx: &CommandContext, params: &CommandExecParams) -> CommandResult {
        trace!("PromptCommand::execute >> ctx: {:?}, params: {:?}", ctx, params);

        let prompt = get_str_param("prompt", params).map_err(error_err!())?;

        ctx.set_main_prompt(prompt.to_owned());
        println_succ!("Command prompt has been set to \"{}\"", prompt);
        let res = Ok(());

        trace!("PromptCommand::execute << {:?}", res);
        res
    }
}

pub mod ExitCommand {
    use super::*;

    command!(CommandMetadata::build("exit", "Exit Indy CLI").finalize());

    fn execute(ctx: &CommandContext, _params: &CommandExecParams) -> CommandResult {
        trace!("ExitCommand::execute >> ctx: {:?}, params: {:?}", ctx, _params);

        ctx.set_exit();
        println_succ!("Goodbye...");
        let res = Ok(());

        trace!("ExitCommand::execute << {:?}", res);
        res
    }
}