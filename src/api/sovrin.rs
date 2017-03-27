use errors::sovrin::SovrinError;
use commands::{Command, CommandExecutor};
use commands::sovrin::SovrinCommand;
use constants::SovrinRole;

use std::error;
use std::sync::Arc;

pub struct SovrinAPI {
    command_executor: Arc<CommandExecutor>
}

impl SovrinAPI {
    /// Constructs a new `SovrinAPI`.
    ///
    /// #Params
    /// command_executor: Reference to `CommandExecutor` instance.
    ///
    pub fn new(command_executor: Arc<CommandExecutor>) -> SovrinAPI {
        SovrinAPI {
            command_executor: command_executor
        }
    }

    /// Sends NYM transaction to Identity Ledger.
    ///
    /// Creates a new NYM records for specific user, trust anchor, steward or trustee.
    /// Note that only trustees and stewards can create new sponsors and trustee can be created only by other trusties.
    ///
    /// #Params
    /// dest: Id of a target NYM record or an alias.
    /// verkey: Optional(defaults to dest). Verification key.
    /// xref: Optional (Required if dest is an alias). Id of a NYM record.
    /// data: Optional. Alias.
    /// role: Optional (defaults to None). Role of a user NYM record being created for.
    ///     One of USER, TRUST_ANCHOR, STEWARD, TRUSTEE.
    ///     Also a TRUSTEE can change any Nym's role to None, this stopping it from making any writes.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// No result
    ///
    /// #Errors
    /// No method specific errors.
    /// See `SovrinError` docs for common errors description.
    pub fn send_nym_tx(&self, dest: &str, verkey: Option<&str>, xref: Option<&str>,
                       data: Option<&str>, role: Option<SovrinRole>,
                       cb: Box<Fn(Result<(), SovrinError>) + Send>) {
        self.command_executor.send(
            Command::Sovrin(
                SovrinCommand::SendNymTx(
                    dest.to_string(),
                    verkey.map(String::from),
                    xref.map(String::from),
                    data.map(String::from),
                    role,
                    cb)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sovrin_api_can_be_created() {
        let sovrin_api = SovrinAPI::new(Arc::new(CommandExecutor::new()));
        assert! (true, "No crashes on SovrinAPI::new");
    }

    #[test]
    fn sovrin_api_can_be_dropped() {
        fn drop_test() {
            let sovrin_api = SovrinAPI::new(Arc::new(CommandExecutor::new()));
        }

        drop_test();
        assert! (true, "No crashes on SovrinAPI::drop");
    }
}