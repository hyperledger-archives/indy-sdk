use errors::pool::PoolError;

use services::pool::PoolService;

use std::rc::Rc;


pub enum PoolCommand {
    Create(String, // name
           Option<String>, // config
           Box<Fn(Result<(), PoolError>) + Send>),
    Delete(String, // name
           Box<Fn(Result<(), PoolError>) + Send>),
    Open(String, // name
         Option<String>, // config
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
                self.create(&name, config.as_ref().map(String::as_str), cb);
            },
            PoolCommand::Delete(name, cb) => {
                info!(target: "pool_command_executor", "Delete command received");
                self.delete(&name, cb);
            },
            PoolCommand::Open(name, config, cb) => {
                info!(target: "pool_command_executor", "Open command received");
                self.open(&name, config.as_ref().map(String::as_str), cb);
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

    fn create(&self, name: &str, config: Option<&str>, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }

    fn delete(&self, name: &str, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }

    fn open(&self, name: &str, config: Option<&str>, cb: Box<Fn(Result<i32, PoolError>) + Send>) {
        let config: String = match config {
            Some(s) => String::from(s),
            None => format!("{{\"genesis_txn\": \"{}.txn\"}}", name),
        };
        self.pool_service.open(name, config.as_str());
        // TODO: FIXME: Implement me!!!
        cb(Ok((1000)));
    }

    fn close(&self, handle: i32, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }

    fn refresh(&self, handle: i32, cb: Box<Fn(Result<(), PoolError>) + Send>) {
        // TODO: FIXME: Implement me!!!
        cb(Ok(()));
    }
}