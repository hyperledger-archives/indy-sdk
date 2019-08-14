extern crate libloading;

use indy::ErrorCode;

use command_executor::{Command, CommandContext, CommandParams, CommandMetadata, CommandResult};
use commands::get_str_param;

use utils::logger;
use utils::file::read_file;

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

    command!(CommandMetadata::build("show", "Print the content of text file")
                            .add_main_param("file", "The path to file to show")
                            .add_example("show /home/file.txt")
                            .finalize());

    fn execute(_ctx: &CommandContext, params: &CommandParams) -> CommandResult {
        trace!("execute >> params: {:?}", params);

        let file = get_str_param("file", params).map_err(error_err!())?;

        let content = read_file(file)
            .map_err(|err| println_err!("{}", err))?;

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

pub mod load_plugin_command {
    use super::*;

    command!(CommandMetadata::build("load-plugin", "Load plugin in Libindy")
                            .add_required_param("library", "Name of plugin (can be absolute or relative path)")
                            .add_required_param("initializer", "Name of plugin init function")
                            .add_example("load-plugin library=libnullpay initializer=libnullpay_init")
                            .finalize());

    fn execute(_ctx: &CommandContext, params: &CommandParams) -> CommandResult {
        trace!("execute >> params: {:?}", params);

        let library = get_str_param("library", params).map_err(error_err!())?;
        let initializer = get_str_param("initializer", params).map_err(error_err!())?;

        load_plugin(_ctx, library, initializer)?;

        trace!("execute << ");

        Ok(())
    }
}

pub mod init_logger_command {
    use super::*;

    command!(CommandMetadata::build("init-logger", "Init logger according to a config file. \n\tIndy Cli uses `log4rs` logging framework: https://crates.io/crates/log4rs")
                            .add_main_param("file", "The path to the logger config file")
                            .add_example("init-logger /home/logger.yml")
                            .finalize());

    fn execute(_ctx: &CommandContext, params: &CommandParams) -> CommandResult {
        trace!("execute >> params: {:?}", params);

        let file = get_str_param("file", params).map_err(error_err!())?;

        match logger::IndyCliLogger::init(&file){
            Ok(()) => println_succ!("Logger has been initialized according to the config file: \"{}\"", file),
            Err(err) => println_err!("{}", err)
        };

        trace!("execute << ");

        Ok(())
    }
}

pub fn load_plugin(ctx: &CommandContext, library: &str, initializer: &str) -> Result<(), ()> {
    let lib = _load_lib(library)
        .map_err(|_| println_err!("Plugin not found: {:?}", library))?;

    unsafe {
        let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = lib.get(initializer.as_bytes())
            .map_err(|_| println_err!("Init function not found"))?;

        match init_func() {
            ErrorCode::Success => println_succ!("Plugin has been loaded: \"{}\"", library),
            _ => {
                println_err!("Plugin has not been loaded: \"{}\"", library);
                return Err(())
            }
        }
    }

    //TODO think more about behaviour in case of init_func failed
    ctx.add_plugin(library, lib);

    Ok(())
}

#[cfg(all(unix, test))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::os::unix::Library::open(Some(library), ::libc::RTLD_NOW | ::libc::RTLD_NODELETE)
        .map(libloading::Library::from)
}

#[cfg(any(not(unix), not(test)))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::Library::new(library)
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
#[cfg(feature = "nullpay_plugin")]
pub mod tests {
    use super::*;

    pub const NULL_PAYMENT_METHOD: &'static str = "null";
    pub const NULL_PAYMENT_PLUGIN_INIT_FUNCTION: &'static str = "nullpay_init";

    #[cfg(any(unix))]
    pub const NULL_PAYMENT_PLUGIN: &'static str = "libnullpay.so";
    #[cfg(any(windows))]
    pub const NULL_PAYMENT_PLUGIN: &'static str = "nullpay.dll";

    mod load {
        use super::*;
        use utils::test::TestUtils;

        #[test]
        pub fn load_works() {
            TestUtils::cleanup_storage();
            let ctx = CommandContext::new();

            let cmd = load_plugin_command::new();
            let mut params = CommandParams::new();
            params.insert("library", NULL_PAYMENT_PLUGIN.to_string());
            params.insert("initializer", NULL_PAYMENT_PLUGIN_INIT_FUNCTION.to_string());
            cmd.execute(&ctx, &params).unwrap();

            TestUtils::cleanup_storage();
        }

        #[test]
        pub fn load_works_for_unknown_plugin() {
            let ctx = CommandContext::new();

            let cmd = load_plugin_command::new();
            let mut params = CommandParams::new();
            params.insert("library", "unknown_payment_plugin".to_string());
            params.insert("initializer", NULL_PAYMENT_PLUGIN_INIT_FUNCTION.to_string());
            cmd.execute(&ctx, &params).unwrap_err();
        }

        #[test]
        pub fn load_works_for_unknown_init_function() {
            let ctx = CommandContext::new();

            let cmd = load_plugin_command::new();
            let mut params = CommandParams::new();
            params.insert("library", NULL_PAYMENT_PLUGIN.to_string());
            params.insert("initializer", "unknown_init_function".to_string());
            cmd.execute(&ctx, &params).unwrap_err();
        }
    }

    pub fn load_null_payment_plugin(ctx: &CommandContext) -> () {
        let lib = _load_lib(NULL_PAYMENT_PLUGIN).unwrap();
        unsafe {
            let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = lib.get(NULL_PAYMENT_PLUGIN_INIT_FUNCTION.as_bytes()).unwrap();
            init_func();
        }
        ctx.add_plugin(NULL_PAYMENT_PLUGIN, lib)
    }
}