use errors::sovrin::SovrinError;
use services::sovrin::SovrinService;

use std::rc::Rc;

pub enum SovrinCommand {
    SendNymTx(
        String, // issuer
        String, // dest
        Option<String>, // verkey
        Option<String>, // xref
        Option<String>, // data
        Option<String>, // role
        Box<Fn(Result<(), SovrinError>) + Send>)
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
            SovrinCommand::SendNymTx(issuer, did, verkey, xref, data, role, cb) => {
                info!(target: "sovrin_command_executor", "SendNymTx command received");
                (self.send_nym_tx(
                    &issuer,
                    &did,
                    verkey.as_ref().map(String::as_str),
                    xref.as_ref().map(String::as_str),
                    data.as_ref().map(String::as_str),
                    role.as_ref().map(String::as_str)), cb)
            }
        };

        cb(result);
    }

    fn send_nym_tx(&self, issuer: &str, did: &str, verkey: Option<&str>, xref: Option<&str>,
                   data: Option<&str>, role: Option<&str>) -> Result<(), SovrinError> {
        self.sovrin_service.send_nym_tx(issuer, did, verkey, xref, data, role)
    }
}