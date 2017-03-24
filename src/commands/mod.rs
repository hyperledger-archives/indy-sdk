mod set_did;

use commands::set_did::SetDidCommandExecutor;
use services::sovrin::SovrinService;

use std::error;
use std::sync::mpsc::{Sender, channel};
use std::rc::Rc;
use std::thread;

pub enum Command {
    Exit,
    SetDidCommand(String, Box<Fn(Result<(), Box<error::Error>>) + Send>)
}

pub struct CommandExecutor {
    worker: Option<thread::JoinHandle<()>>,
    sender: Sender<Command>
}

impl CommandExecutor {
    pub fn new() -> CommandExecutor {
        let (sender, receiver) = channel();

        CommandExecutor {
            sender: sender,
            worker: Some(thread::spawn(move || {
                loop {
                    info!(target: "CommandExecutor", "Worker thread started");

                    let sovrin_service = Rc::new(SovrinService::new());
                    let set_did_executor = SetDidCommandExecutor::new(sovrin_service);

                    match receiver.recv() {
                        Ok(Command::SetDidCommand(did, cb)) => {
                            info!(target: "CommandExecutor", "SetDidCommand command received");
                            set_did_executor.execute(did, cb);
                        }
                        Ok(Command::Exit) => {
                            info!(target: "CommandExecutor", "Exit command received");
                            break
                        },
                        Err(err) => {
                            error!(target: "CommandExecutor", "Failed to get command!");
                            panic!("CommandExecutor: Failed to get command! {:?}", err)
                        }
                    }
                }
            }))
        }
    }

    pub fn send(&self, cmd: Command) {
        self.sender.send(cmd);
    }
}

impl Drop for CommandExecutor {
    fn drop(&mut self) {
        info!(target: "CommandExecutor", "Drop started");
        self.send(Command::Exit);
        // Option worker type and this kludge is workaround for rust
        self.worker.take().unwrap().join();
        info!(target: "CommandExecutor", "Drop finished");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_executor_can_be_created() {
        let command_executor = CommandExecutor::new();
        assert!(true, "No crashes on CommandExecutor::new");
    }

    #[test]
    fn command_executor_can_be_dropped() {
        fn drop_test() {
            let command_executor = CommandExecutor::new();
        }

        drop_test();
        assert!(true, "No crashes on CommandExecutor::drop");
    }

    #[test]
    fn set_did_command_can_be_sent() {
        let (sender, receiver) = channel();

        let cb = Box::new(move |result| {
            match result {
                Ok(val) => sender.send("OK"),
                Err(err) => sender.send("ERR")
            };
        });

        let cmd_executor = CommandExecutor::new();
        cmd_executor.send(Command::SetDidCommand("DID0".to_string(), cb));

        match receiver.recv() {
            Ok(result) => {
                assert_eq!("OK", result);
            }
            Err(err) => {
                panic!("Error on result recv: {:?}", err);
            }
        }
    }
}