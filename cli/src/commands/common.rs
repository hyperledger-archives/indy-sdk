use command_executor::{Command, CommandMetadata};

use std::collections::HashMap;

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

        println!("Hyperledger Indy CLI");
        println!();
        println!("This is the official CLI tool for Hyperledger Indy (https://www.hyperledger.org/projects),");
        println!("which provides a distributed-ledger-based foundation for");
        println!("self-sovereign identity (https://sovrin.org/).");
        println!();
        println!("Apache License Version 2.0");
        println!("Copyright 2017 Sovrin Foundation");
        println!();

        let res = Ok(());

        trace!("AboutCommand::execute << {:?}", res);
        res
    }

    fn metadata(&self) -> &CommandMetadata {
        &self.metadata
    }
}