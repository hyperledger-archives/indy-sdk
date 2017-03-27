use commands::{Command, CommandExecutor};
use std::error;
use std::sync::Arc;
use errors::anoncreds::AnoncredsError;

pub struct AnoncredsAPI {
    command_executor: Arc<CommandExecutor>,
}

impl AnoncredsAPI {
    pub fn new(command_executor: Arc<CommandExecutor>) -> AnoncredsAPI {
        AnoncredsAPI { command_executor: command_executor }
    }

    /// Creates master secret key for prover.
    ///
    /// #Params
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// Master secret key as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn create_master_secret(cb: Box<Fn(Result<(String), AnoncredsError>) + Send>) {
        unimplemented!();
    }

    /// Creates public/private keys pair for issuer.
    ///
    /// #Params
    /// schema: claim definition schema as a byte array.
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// Tuple of
    /// (Public primary key, Secret Key, Public non-revocation key, Secret non-revocation key)
    /// as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn create_key_pair(schema: &[&u8],
                       cb: Box<Fn(
                           Result<((String, String, String, String)), AnoncredsError>) + Send>) {
        unimplemented!();
    }


    pub fn create_context_attribute(i_a: String, user_id: String,
                                    cb: Box<Fn(Result<(String), AnoncredsError>) + Send>) {

    }

    /// Issues accumulator.
    ///
    /// #Params
    /// schema: claim definition schema as a byte array.
    /// accumulator_id: accumulator id as a byte array.
    /// max_claims: maximum number of claims within accumulator as a byte array.
    /// public_key_non_revocation: non-revocation public key as a byte array.
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// Tuple of (Accumulator, Tails, Accumulator public key, Accumulator secret key) as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn issue_accumulator(schema: &[&u8], accumulator_id: &[&u8], max_claims: &[&u8],
                             public_key_non_revocation: &[&u8],
                             cb: Box<Fn(
                                 Result<((String, String, String, String)),
                                     AnoncredsError>
                             ) + Send>) {
        unimplemented!();
    }

    pub fn issue_claim(attributes: String, accumulator: String,i_a: String, i: String,
                       claim_request: String, context_attribute: String, public_key: String,
                       secret_key: String, public_key_revocation: String,
                       secret_key_revocation: String, tails: String,
                       secret_key_accumulator: String,
                       cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {

    }

    /// Creates claim request.
    ///
    /// #Params
    /// master_secret: prover master secret as a byte array.
    /// public_key: issuer public_key as a byte array.
    /// public_key_non_revocation: issuer non-revocation public key as a byte array.
    /// request_non_revocation: whether to request non-revocation claim as a byte array.
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// Claim request as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn create_claim_request(master_secret: &[&u8], public_key: &[&u8],
                                public_key_non_revocation: &[&u8], request_non_revocation: &[&u8],
                                cb: Box<Fn(Result<(String), AnoncredsError>) + Send>) {
        unimplemented!();
    }

    pub fn create_proof(proof_input: String, nonce: String, claims: String,
                        public_key_revocation: String, accum: String, public_key: String,
                        master_secret: String,
                        cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {

    }

    pub fn verify_proof(proof_input: String, proof: String, revealed_attributes: String,
                        nonce: String, public_key_revocation: String,
                        public_key_accumulator: String, accumulator: String,
                        public_key: String, attributes: String,
                        cb: Box<Fn(Result<(String), AnoncredsError>) + Send>) {

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