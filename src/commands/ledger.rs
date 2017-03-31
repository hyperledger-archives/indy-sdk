use errors::ledger::LedgerError;
use services::ledger::LedgerService;

use std::rc::Rc;

pub enum LedgerCommand {
    SendNymTx(
        String, // issuer
        String, // dest
        Option<String>, // verkey
        Option<String>, // xref
        Option<String>, // data
        Option<String>, // role
        Box<Fn(Result<(), LedgerError>) + Send>)
}

pub struct LedgerCommandExecutor {
    ledger_service: Rc<LedgerService>
}

impl LedgerCommandExecutor {
    pub fn new(ledger_service: Rc<LedgerService>) -> LedgerCommandExecutor {
        LedgerCommandExecutor {
            ledger_service: ledger_service
        }
    }

    pub fn execute(&self, command: LedgerCommand) {
        let (result, cb) = match command {
            LedgerCommand::SendNymTx(issuer, did, verkey, xref, data, role, cb) => {
                info!(target: "ledger_command_executor", "SendNymTx command received");
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
                   data: Option<&str>, role: Option<&str>) -> Result<(), LedgerError> {
        self.ledger_service.send_nym_tx(issuer, did, verkey, xref, data, role)
    }
}