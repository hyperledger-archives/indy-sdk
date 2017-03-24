#[macro_use]
extern crate log;

mod commands;
mod services;

use commands::{Command, CommandExecutor};
use std::error;
use std::sync::Arc;

struct AnoncredsAPI {
    command_executor: Arc<CommandExecutor>,
}

impl AnoncredsAPI {
    pub fn new(command_executor: Arc<CommandExecutor>) -> AnoncredsAPI {
        AnoncredsAPI { command_executor: command_executor }
    }

    fn dummy() {}
}

struct SovrinAPI {
    command_executor: Arc<CommandExecutor>,
}

impl SovrinAPI {
    pub fn new(command_executor: Arc<CommandExecutor>) -> SovrinAPI {
        SovrinAPI {
            command_executor: command_executor
        }
    }

    pub fn set_did(&self, did: String, cb: Box<Fn(Result<(), Box<error::Error>>) + Send>) {
        self.command_executor.send(Command::SetDidCommand(did, cb));
    }

    pub fn create_master_secret(cb: Box<Fn(Result<(String), Box<error::Error>>) + Send>) {

    }

    pub fn create_keys(schema: String,
                       cb: Box<Fn(Result<((String, String)), Box<error::Error>>) + Send>) {

    }

    pub fn create_context_attribute(i_a: String, user_id: String,
                                    cb: Box<Fn(Result<(String), Box<error::Error>>) + Send>) {

    }

    pub fn issue_accumulator(schema: String, i_a: String, l: String,
                             public_key_revocation: String,
                             cb: Box<Fn(
                                 Result<((String, String, String, String)),
                                 Box<error::Error>>
                             ) + Send>) {

    }

    pub fn issue_claim(attributes: String, accumulator: String,i_a: String, i: String,
                       claim_request: String, context_attribute: String, public_key: String,
                       secret_key: String, public_key_revocation: String,
                       secret_key_revocation: String, tails: String,
                       secret_key_accumulator: String,
                       cb: Box<Fn(Result<(String, String), Box<error::Error>>) + Send>) {

    }

    pub fn create_claim_request(master_secret: String, public_key: String,
                                public_key_revocation: String, request_non_revocation: String,
                                cb: Box<Fn(Result<(String), Box<error::Error>>) + Send>) {

    }

    pub fn create_proof(proof_input: String, nonce: String, claims: String,
                        public_key_revocation: String, accum: String, public_key: String,
                        master_secret: String,
                        cb: Box<Fn(Result<(String, String), Box<error::Error>>) + Send>) {

    }

    pub fn verify_proof(proof_input: String, proof: String, revealed_attributes: String,
                        nonce: String, public_key_revocation: String,
                        public_key_accumulator: String, accumulator: String,
                        public_key: String, attributes: String,
                        cb: Box<Fn(Result<(String), Box<error::Error>>) + Send>) {

    }
}


pub struct SovrinClient {
    sovrin: SovrinAPI,
    anoncreds: AnoncredsAPI
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
    fn set_did_method_can_be_called() {
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