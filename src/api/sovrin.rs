use commands::{Command, CommandExecutor};
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

    pub fn set_did(&self, did: String, cb: Box<Fn(Result<(), Box<error::Error>>) + Send>) {
        self.command_executor.send(Command::SetDidCommand(did, cb));
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