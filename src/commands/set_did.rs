use commands::Command;
use services::sovrin::SovrinService;

use std::error;
use std::fmt;

struct SetDIDCommand<'a> {
    did: String,
    executor: &'a SetDIDCommandExecutor<'a>,
    cb: Box<Fn(Result<String, Box<error::Error>>)>
}

impl<'a> SetDIDCommand<'a> {
    fn new(did: String,
           cb: Box<Fn(Result<String, Box<error::Error>>)>,
           executor: &'a SetDIDCommandExecutor, ) -> SetDIDCommand<'a> {
        trace!(target: "SetDIDCommand", "new {:?}", did);
        SetDIDCommand {
            did: did,
            executor: executor,
            cb: cb
        }
    }
}

impl<'a> Command for SetDIDCommand<'a> {
    fn execute(&self) {
        trace!(target: "SetDIDCommand", "execute");
        self.executor.execute(self);
    }
}

impl<'a> fmt::Debug for SetDIDCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SetDIDCommand {{ did: {}, cb: cb }}", self.did)
    }
}

struct SetDIDCommandExecutor<'a> {
    sovrin_service: &'a SovrinService,
    dummy: String
}

impl<'a> SetDIDCommandExecutor<'a> {
    pub fn new(sovrin_service: &SovrinService) -> SetDIDCommandExecutor {
        trace!(target: "SetDIDCommandExecutor", "new");
        SetDIDCommandExecutor {
            sovrin_service: sovrin_service,
            dummy: "set_did_command_executor_dummy".to_string()
        }
    }

    pub fn new_command(&self,
                       did: String,
                       cb: Box<Fn(Result<String, Box<error::Error>>)>) -> SetDIDCommand {
        trace!(target: "SetDIDCommandExecutor", "new_command {:?}", did);
        SetDIDCommand::new(did, cb, self)
    }

    fn execute(&self, command: &SetDIDCommand) {
        trace!(target: "SetDIDCommandExecutor", "execute {:?}", "SetDIDCommand");
        let result = self.sovrin_service.set_did(command.did.clone());
        (command.cb)(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_can_be_created_and_executed() {
        use std::sync::Arc;
        use std::sync::Mutex;

        let sovrin_service = SovrinService::new();
        let executor = SetDIDCommandExecutor::new(&sovrin_service);

        let arc = Arc::new(Mutex::new("".to_string()));
        let arc2 = arc.clone();

        let command = executor.new_command("DID0".to_string(), Box::new(move |result| {
            let mut arc_value = arc2.lock().unwrap();

            match result {
                Result::Ok(val) => *arc_value = val,
                Result::Err(err) =>
                    panic!("SetDIDCommand finished with `Err` value: {:?}", err)
            }
        }));

        command.execute();

        assert_eq!("set_did_command_executor_dummy", command.executor.dummy, "Executor filled properly");
        assert_eq!("DID0", *arc.lock().unwrap(), "Command returns expected result");
    }
}