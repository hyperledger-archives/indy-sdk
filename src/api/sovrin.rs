use errors::sovrin::SovrinError;
use commands::{Command, CommandExecutor};
use commands::sovrin::SovrinCommand;

use std::error;
use std::sync::Arc;

pub struct SovrinAPI {
    command_executor: Arc<CommandExecutor>
}

impl SovrinAPI {
    pub fn new(command_executor: Arc<CommandExecutor>) -> SovrinAPI {
        SovrinAPI {
            command_executor: command_executor
        }
    }

    pub fn send_nym_tx(&self, did: &str, cb: Box<Fn(Result<(), SovrinError>) + Send>) {
        self.command_executor.send(Command::Sovrin(SovrinCommand::SendNymTx(did.to_string(), cb)));
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