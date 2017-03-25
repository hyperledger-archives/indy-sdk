use errors::sovrin::SovrinError;
use services::sovrin::SovrinService;

use std::rc::Rc;

pub enum SovrinCommand {
    SendNymTx(String, Box<Fn(Result<(), SovrinError>) + Send>)
}

pub struct SovrinCommandExecutor {
    sovrin_service: Rc<SovrinService>
}

impl SovrinCommandExecutor {
    pub fn new(sovrin_service: Rc<SovrinService>) -> SovrinCommandExecutor {
        SovrinCommandExecutor {
            sovrin_service: sovrin_service
        }
    }

    pub fn execute(&self, command: SovrinCommand) {
        let (result, cb) = match command {
            SovrinCommand::SendNymTx(did, cb) => {
                info!(target: "sovrin_command_executor", "SendNymTx command received");
                (self.send_nym_tx(&did), cb)
            }
        };

        cb(result);
    }

    fn send_nym_tx(&self, did: &str) -> Result<(), SovrinError> {
        self.sovrin_service.send_nym_tx(did)
    }
}