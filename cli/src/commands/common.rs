extern crate libloading;

use libindy::ErrorCode;

use command_executor::{Command, CommandContext, CommandParams, CommandMetadata, CommandResult};
use commands::get_str_param;

pub mod about_command {
    use super::*;

    command!(CommandMetadata::build("about", "Show about information").finalize());

    fn execute(_ctx: &CommandContext, _params: &CommandParams) -> CommandResult {
        trace!("execute >> _ctx: params: {:?}", _params);

        println_succ!("Hyperledger Indy CLI (https://github.com/hyperledger/indy-sdk)");
        println!();
        println_succ!("This is the official CLI tool for Hyperledger Indy (https://www.hyperledger.org/projects),");
        println_succ!("which provides a distributed-ledger-based foundation for");
        println_succ!("self-sovereign identity (https://sovrin.org/).");
        println!();
        println_succ!("Version: {}", env!("CARGO_PKG_VERSION"));
        println_succ!("Apache License Version 2.0");
        println_succ!("Copyright 2017 Sovrin Foundation");
        println!();

        let res = Ok(());

        trace!("execute << {:?}", res);
        res
    }
}

pub mod show_command {
    use super::*;
    use std::io::Read;
    use std::fs::File;

    command!(CommandMetadata::build("show", "Print the content of text file")
                            .add_main_param("file", "The path to file to show")
                            .add_example("show /home/file.txt")
                            .finalize());

    fn execute(_ctx: &CommandContext, params: &CommandParams) -> CommandResult {
        trace!("execute >> params: {:?}", params);

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

        trace!("execute << {:?}", res);
        res
    }
}

pub mod prompt_command {
    use super::*;

    command!(CommandMetadata::build("prompt", "Change command prompt")
                            .add_main_param("prompt", "New prompt string")
                            .add_example("prompt new-prompt")
                            .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> CommandResult {
        trace!("execute >> ctx: {:?}, params: {:?}", ctx, params);

        let prompt = get_str_param("prompt", params).map_err(error_err!())?;

        ctx.set_main_prompt(prompt.to_owned());
        println_succ!("Command prompt has been set to \"{}\"", prompt);
        let res = Ok(());

        trace!("execute << {:?}", res);
        res
    }
}

pub mod load_command {
    use super::*;

    command!(CommandMetadata::build("load", "Load plugin")
                            .add_main_param("path", "The path to loading plugin")
                            .add_example("load test_plugin")
                            .finalize());

    fn execute(_ctx: &CommandContext, params: &CommandParams) -> CommandResult {
        trace!("execute >> params: {:?}", params);

        let path = get_str_param("path", params).map_err(error_err!())?;

        let lib = libloading::Library::new(path)
            .map_err(|_| println_err!("Plugin not found: \"{:?}\"", path))?;

        unsafe {
            let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = lib.get(b"init")
                .map_err(|_| println_err!("Init function not found"))?;

            match init_func() {
                ErrorCode::Success => println_succ!("Plugin has been loaded: \"{}\"", path),
                _ => println_err!("Plugin has not been loaded: \"{}\"", path)
            }
        }

        let res = _ctx.add_plugin(path, lib);

        trace!("execute << {:?}", res);

        Ok(res)
    }
}

pub mod exit_command {
    use super::*;

    command!(CommandMetadata::build("exit", "Exit Indy CLI").finalize());

    fn execute(ctx: &CommandContext, _params: &CommandParams) -> CommandResult {
        trace!("execute >> ctx: {:?}, params: {:?}", ctx, _params);

        ctx.set_exit();
        let res = Ok(());

        trace!("execute << {:?}", res);
        res
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const NULL_PAYMENT_METHOD: &'static str = "null_payment";
    pub const NULL_PAYMENT_PLUGIN: &'static str = "libnullpaymentplugin.so";

    mod load {
        use super::*;

        #[test]
        pub fn load_works() {
            let ctx = CommandContext::new();

            let cmd = load_command::new();
            let mut params = CommandParams::new();
            params.insert("path", NULL_PAYMENT_PLUGIN.to_string());
            cmd.execute(&ctx, &params).unwrap();
        }
    }

    pub fn load_null_payment_plugin(ctx: &CommandContext) -> () {
        let lib = libloading::Library::new(NULL_PAYMENT_PLUGIN).unwrap();
        unsafe {
            let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = lib.get(b"init").unwrap();
            init_func();
        }
        ctx.add_plugin(NULL_PAYMENT_PLUGIN, lib)
    }
}