use commands::{Command, CommandExecutor};
use std::error;
use std::sync::Arc;

pub struct AnoncredsAPI {
    command_executor: Arc<CommandExecutor>,
}

impl AnoncredsAPI {
    pub fn new(command_executor: Arc<CommandExecutor>) -> AnoncredsAPI {
        AnoncredsAPI { command_executor: command_executor }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anoncreds_api_can_be_created() {
        let anoncreds_api = AnoncredsAPI::new(Arc::new(CommandExecutor::new()));
        assert! (true, "No crashes on AnoncredsAPI::new");
    }

    #[test]
    fn anoncredsn_api_can_be_dropped() {
        fn drop_test() {
            let anoncreds_api = AnoncredsAPI::new(Arc::new(CommandExecutor::new()));
        }

        drop_test();
        assert! (true, "No crashes on AnoncredsAPI::drop");
    }
}