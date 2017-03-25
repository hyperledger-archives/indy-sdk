#[macro_use]
extern crate log;

mod api;
mod commands;
mod services;

use api::anoncreds::AnoncredsAPI;
use api::sovrin::SovrinAPI;
use commands::{Command, CommandExecutor};
use std::error;
use std::sync::Arc;

pub struct SovrinClient {
    anoncreds: AnoncredsAPI,
    sovrin: SovrinAPI
}

impl SovrinClient {
    pub fn new() -> SovrinClient {
        let command_executor = Arc::new(CommandExecutor::new());
        SovrinClient {
            anoncreds: AnoncredsAPI::new(command_executor.clone()),
            sovrin: SovrinAPI::new(command_executor.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn sovrin_client_can_be_created() {
        let sovrin_client = SovrinClient::new();
        assert! (true, "No crashes on SovrinClient::new");
    }

    #[test]
    fn sovrin_client_can_be_dropped() {
        fn drop_test() {
            let sovrin_client = SovrinClient::new();
        }

        drop_test();
        assert! (true, "No crashes on SovrinClient::drop");
    }

    #[test]
    fn sovrin_set_did_method_can_be_called() {
        let (sender, receiver) = channel();

        let cb = Box::new(move |result| {
            match result {
                Ok(val) => sender.send("OK"),
                Err(err) => sender.send("ERROR")
            };
        });

        let sovrin_client = SovrinClient::new();
        sovrin_client.sovrin.set_did("DID0".to_string(), cb);

        match receiver.recv() {
            Ok(result) => {
                assert_eq! ("OK", result);
            }
            Err(err) => {
                panic!("Error on result recv: {:?}", err);
            }
        }
    }
}