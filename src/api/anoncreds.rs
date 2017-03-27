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
    pub fn create_master_secret(cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
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

    /// Issues accumulator.
    ///
    /// #Params
    /// schema: claim definition schema as a byte array.
    /// accumulator_id: accumulator id as a byte array.
    /// max_claims: maximum number of claims within accumulator as a byte array.
    /// public_key_non_revocation: issuer's non-revocation public key as a byte array.
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


    /// Issues claim.
    ///
    /// #Params
    /// attributes:  all names of attributes as a byte array.
    /// accumulator: accumulator as a byte array.
    /// sequence_number: claim's sequence number within accumulator as a byte array.
    /// claim_request:
    ///     A claim request containing prover ID and prover-generated values as a byte array.
    /// public_key: issuer's public_key as a byte array.
    /// secret_key: issuer's secret_key as a byte array.
    /// public_key_non_revocation: issuer's non-revocation public key as a byte array.
    /// secret_key_non_revocation: issuer's non-revocation secret key as a byte array.
    /// tails: tails as a byte array.
    /// secret_key_accumulator: accumulator's secret key as a byte array.
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// The claim (both primary and non-revocation) as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn issue_claim(attributes: &[&u8], accumulator: &[&u8], sequence_number: &[&u8],
                       claim_request: &[&u8], public_key: &[&u8],
                       secret_key: &[&u8], public_key_non_revocation: &[&u8],
                       secret_key_non_revocation: &[&u8], tails: &[&u8],
                       secret_key_accumulator: &[&u8],
                       cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        unimplemented!();
    }

    /// Creates claim request.
    ///
    /// #Params
    /// master_secret: prover's master secret as a byte array.
    /// public_key: issuer's public_key as a byte array.
    /// public_key_non_revocation: issuer's non-revocation public key as a byte array.
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
                                cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        unimplemented!();
    }


    /// Creates proof.
    ///
    /// #Params
    /// proof_input: description of a proof to be presented
    ///     (revealed attributes, predicates, timestamps for non-revocation) as a byte array.
    /// nonce: verifier's nonce as a byte array.
    /// claims: necessary claims for proof as a byte array.
    /// master_secret: prover's master secret key as a byte array.
    /// public_key: issuer's public key as a byte array.
    /// public_key_non_revocation: issuer's non-revocation public key as a byte array.
    /// accumulator: accumulator as a byte array.
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// Typle of (
    ///         Proof (both primary and non-revocation),
    ///         Revealed attributes (initial non-encoded values)
    ///     ) as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn create_proof(proof_input: &[&u8], nonce: &[&u8], claims: &[&u8],
                        public_key_non_revocation: &[&u8], accumulator: &[&u8], public_key: &[&u8],
                        master_secret: &[&u8],
                        cb: Box<Fn(Result<(String, String), AnoncredsError>) + Send>) {
        unimplemented!();
    }

    /// Verifies proof.
    ///
    /// #Params
    /// proof_input: description of a proof to be presented
    ///     (revealed attributes, predicates, timestamps for non-revocation) as a byte array.
    /// proof: a proof as a byte array.
    /// revealed_attributes:
    ///     values of revealed attributes (initial values, non-encoded) as a byte array.
    /// nonce: verifier's nonce as a byte array.
    /// public_key_non_revocation: issuer's non-revocation public key as a byte array.
    /// accumulator: accumulator as a byte array.
    /// public_key_accumulator: accumulator's public key as a byte array.
    /// public_key: issuer's public key as a byte array.
    /// attributes:  all names of attributes as a byte array.
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// True if verified successfully and false otherwise as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn verify_proof(proof_input: &[&u8], proof: &[&u8], revealed_attributes: &[&u8],
                        nonce: &[&u8], public_key_non_revocation: &[&u8],
                        accumulator: &[&u8], public_key_accumulator: &[&u8],
                        public_key: &[&u8], attributes: &[&u8],
                        cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        unimplemented!();
    }

    /// Creates verifier's nonce.
    ///
    /// #Params
    /// cb: Callback that takes command result as a parameter.
    ///
    /// #Returns
    /// Nonce as a String.
    ///
    /// #Errors
    /// No method specific errors.
    /// See `AnoncredsError` docs for common errors description.
    pub fn create_nonce(cb: Box<Fn(Result<String, AnoncredsError>) + Send>) {
        unimplemented!();
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