use commands::Command;
use services::sovrin::SovrinService;

use std::rc::Rc;
use std::error;

pub struct SetDidCommandExecutor {
    sovrin_service: Rc<SovrinService>
}

impl SetDidCommandExecutor {
    pub fn new(sovrin_service: Rc<SovrinService>) -> SetDidCommandExecutor {
        SetDidCommandExecutor { sovrin_service: sovrin_service }
    }
    pub fn execute(&self, did: String, cb: Box<Fn(Result<(), Box<error::Error>>)>) {
        let result = self.sovrin_service.set_did(did);
        (cb)(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_can_be_created_and_executed() {
        use std::rc::Rc;
        use std::sync::Mutex;

        let arc = Rc::new(Mutex::new(""));
        let arc2 = arc.clone();

        let cb = Box::new(move |result| {
            let mut arc_value = arc2.lock().unwrap();

            match result {
                Result::Ok(_) => *arc_value = "OK",
                Result::Err(err) => *arc_value = "ERR"
            }
        });

        let executor = SetDidCommandExecutor::new(Rc::new(SovrinService::new()));
        executor.execute("DID0".to_string(), cb);

        assert_eq!("OK", *arc.lock().unwrap(), "Command returns expected result");
    }
}