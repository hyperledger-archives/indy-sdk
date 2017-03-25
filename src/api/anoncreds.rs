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

    pub fn dummy() {}
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