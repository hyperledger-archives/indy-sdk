use command_executor::{Command, CommandMetadata};
use commands::get_str_param;

use std::collections::HashMap;
use std::fs::{File, DirBuilder};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug)]
pub struct AboutCommand {
    metadata: CommandMetadata,
}

impl AboutCommand {
    pub fn new() -> AboutCommand {
        AboutCommand {
            metadata: CommandMetadata::build("about", "Show about information").finalize()
        }
    }
}

impl Command for AboutCommand {
    fn execute(&self, _params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("AboutCommand::execute >> self: {:?}, _params: {:?}", self, _params);

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

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}

#[derive(Debug)]
pub struct ShowCommand {
    metadata: CommandMetadata,
}

impl ShowCommand {
    pub fn new() -> ShowCommand {
        ShowCommand {
            metadata: CommandMetadata::build("show", "Print the content of text file")
                .add_main_param("file", "The path to file to show")
                .finalize()
        }
    }
}

impl Command for ShowCommand {
    fn execute(&self, params: &HashMap<&'static str, &str>) -> Result<(), ()> {
        trace!("ShowCommand::execute >> self: {:?}, params: {:?}", self, params);

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

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}