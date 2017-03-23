use std::thread;
use std::time::Duration;
use std::sync::mpsc::{SyncSender, sync_channel};

/*
 * Command trait
 */

trait Command: Send {
    fn name(&self) -> &str;
    fn execute(&mut self);
}

/*
 * CommandExecutor Exit command implementation
 */

struct CommandExecutorExitCommand {
}

impl Command for CommandExecutorExitCommand {
    fn name(&self) -> &str {
        return "exit";
    }
    fn execute(&mut self) {
        println!("exit command executed!");
    }
}

/*
 * CommandExecutorService implementation
 */

pub struct CommandExecutorService {
    worker: Option<thread::JoinHandle<()>>,
    cmdqueue: SyncSender<Box<Command>>
}

impl CommandExecutorService {
    fn new() -> CommandExecutorService {
        let (tx, rx) = sync_channel(100);
        return CommandExecutorService {
            cmdqueue: tx,
            worker: Some(thread::spawn(move || {
                loop {
                    match rx.recv() {
                        Ok(ref mut cmd) if cmd.name() == "exit" => {
                            println!("thread exit!");
                            break;
                        },
                        Ok(mut cmd) => {
                            println!("execute command {}", cmd.name());
                            cmd.execute();
                        },
                        Err(_) => {
                            println!("failed to get command!");
                            break;
                        }
                    }
                }
            }))
        };
    }

    fn add_command(&mut self, cmd: Box<Command>) {
        self.cmdqueue.send(cmd);
    }

}

impl Drop for CommandExecutorService {
    fn drop(&mut self) {
        let cmd = CommandExecutorExitCommand{ };
        self.add_command(Box::new(cmd));
        // Option worker type and this kludge is workaround for rust
        self.worker.take().unwrap().join();
        println!("drop!!!!!");
    }
}

/*
 * Testing
 */

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCommand {
        dummy: i32
    }

    // unsafe static mut is ok for testing to check if command was executed
    static mut test_ret:i32 = 0;

    impl Command for TestCommand {
        fn name(&self) -> &str {
            return "test";
        }
        fn execute(&mut self) {
            println!("test command executed!");
            unsafe {
                test_ret = 12345;
            }
        }
    }

    #[test]
    fn add_command() {
        let mut commandExecutorService = CommandExecutorService::new();
        let cmd = TestCommand{ dummy: 0 };
        commandExecutorService.add_command(Box::new(cmd));
        thread::sleep(Duration::from_millis(1000));
        unsafe {
            assert_eq!(test_ret, 12345, "Command looks not executed within 1 second");
        }
    }
}
