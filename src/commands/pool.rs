use errors::pool::PoolError;

use services::pool::PoolService;

use std::rc::Rc;


pub enum PoolCommand {
    Create(String, // name
           String, // config
           Box<Fn(Result<(), PoolError>) + Send>),
    Delete(String, // name
           Box<Fn(Result<(), PoolError>) + Send>),
    Open(String, // name
         String, // config
         Box<Fn(Result<i32, PoolError>) + Send>),
    Close(i32, // pool handle
          Box<Fn(Result<(), PoolError>) + Send>),
    Refresh(i32, // pool handle
            Box<Fn(Result<(), PoolError>) + Send>)
}

pub struct PoolCommandExecutor {
    pool_service: Rc<PoolService>
}

impl PoolCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>) -> PoolCommandExecutor {
        PoolCommandExecutor {
            pool_service: pool_service
        }
    }

    pub fn execute(&self, command: PoolCommand) {
        match command {
            PoolCommand::Create(name, config, cb) => {
                info!(target: "pool_command_executor", "Create command received");
                self.create(&name, &config, cb);
            },
            PoolCommand::Delete(name, cb) => {
                info!(target: "pool_command_executor", "Delete command received");
                self.delete(&name, cb);
            },
            PoolCommand::Open(name, config, cb) => {
                info!(target: "pool_command_executor", "Open command received");
                self.open(&name, &config, cb);
            },
            PoolCommand::Close(handle, cb) => {
                info!(target: "pool_command_executor", "Close command received");
                self.close(handle, cb);
            },
            PoolCommand::Refresh(handle, cb) => {
                info!(target: "pool_command_executor", "Refresh command received");
                self.close(handle, cb);
            }
        };
    }

    fn create(&self, name: &str, config: &str, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        unimplemented!()
    }

    fn delete(&self, name: &str, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        unimplemented!()
    }

    fn open(&self, name: &str, config: &str, cb: Box<Fn(Result<i32, PoolError>) + Send>) {
        unimplemented!()
    }

    fn close(&self, handle: i32, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        unimplemented!()
    }

    fn refresh(&self, handle: i32, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        unimplemented!()
    }
}