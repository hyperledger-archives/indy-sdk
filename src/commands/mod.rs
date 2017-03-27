pub mod anoncreds;
pub mod crypto;
pub mod sovrin;
pub mod wallet;

use commands::sovrin::{SovrinCommand, SovrinCommandExecutor};
use services::sovrin::SovrinService;

use std::error;
use std::sync::mpsc::{Sender, channel};
use std::rc::Rc;
use std::thread;

pub enum Command {
    Exit,
    Sovrin(SovrinCommand)
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
                    info!(target: "command_executor", "Worker thread started");

                    let sovrin_service = Rc::new(SovrinService::new());
                    let sovrin_command_executor = SovrinCommandExecutor::new(sovrin_service.clone());

                    match receiver.recv() {
                        Ok(Command::Sovrin(cmd)) => {
                            info!(target: "command_executor", "SovrinCommand command received");
                            sovrin_command_executor.execute(cmd);
                        }
                        Ok(Command::Exit) => {
                            info!(target: "command_executor", "Exit command received");
                            break
                        },
                        Err(err) => {
                            error!(target: "command_executor", "Failed to get command!");
                            panic!("Failed to get command! {:?}", err)
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
        info!(target: "command_executor", "Drop started");
        self.send(Command::Exit);
        // Option worker type and this kludge is workaround for rust
        self.worker.take().unwrap().join();
        info!(target: "command_executor", "Drop finished");
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
        cmd_executor.send(
            Command::Sovrin
                (SovrinCommand::SendNymTx(
                    "DID0".to_string(),
                    None,
                    None,
                    None,
                    None,
                    cb)));

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