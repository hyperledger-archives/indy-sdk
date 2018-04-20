use errors::common::CommonError;
use errors::indy::IndyError;
use errors::pool::PoolError;

use services::pool::PoolService;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;


pub enum PoolCommand {
    Create(String, // name
           Option<String>, // config
           Box<Fn(Result<(), IndyError>) + Send>),
    Delete(String, // name
           Box<Fn(Result<(), IndyError>) + Send>),
    Open(String, // name
         Option<String>, // config
         Box<Fn(Result<i32, IndyError>) + Send>),
    OpenAck(i32, // cmd id
            i32, // pool handle
            Result<() /* pool handle */, PoolError>),
    List(Box<Fn(Result<String, IndyError>) + Send>),
    Close(i32, // pool handle
          Box<Fn(Result<(), IndyError>) + Send>),
    CloseAck(i32,
             Result<(), PoolError>),
    Refresh(i32, // pool handle
            Box<Fn(Result<(), IndyError>) + Send>),
    RefreshAck(i32,
               Result<(), PoolError>),
}

pub struct PoolCommandExecutor {
    pool_service: Rc<PoolService>,
    close_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
    refresh_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
    open_callbacks: RefCell<HashMap<i32, Box<Fn(Result<i32, IndyError>)>>>,
}

impl PoolCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>) -> PoolCommandExecutor {
        PoolCommandExecutor {
            pool_service,
            close_callbacks: RefCell::new(HashMap::new()),
            refresh_callbacks: RefCell::new(HashMap::new()),
            open_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: PoolCommand) {
        match command {
            PoolCommand::Create(name, config, cb) => {
                info!(target: "pool_command_executor", "Create command received");
                self.create(&name, config.as_ref().map(String::as_str), cb);
            }
            PoolCommand::Delete(name, cb) => {
                info!(target: "pool_command_executor", "Delete command received");
                self.delete(&name, cb);
            }
            PoolCommand::Open(name, config, cb) => {
                info!(target: "pool_command_executor", "Open command received");
                self.open(&name, config.as_ref().map(String::as_str), cb);
            }
            PoolCommand::OpenAck(handle, pool_id, result) => {
                info!("OpenAck handle {:?}, pool_id {:?}, result {:?}", handle, pool_id, result);
                match self.open_callbacks.try_borrow_mut() {
                    Ok(mut cbs) => {
                        match cbs.remove(&handle) {
                            Some(cb) => {
                                cb(result
                                    .and_then(|_|
                                        self.pool_service.add_open_pool(pool_id)
                                            .map_err(PoolError::from))
                                    .map_err(IndyError::from))
                            }
                            None => {
                                error!("Can't process PoolCommand::OpenAck for handle {} with result {:?} - appropriate callback not found!",
                                       handle, result);
                            }
                        }
                    }
                    Err(err) => { error!("{:?}", err); }
                }
            }
            PoolCommand::List(cb) => {
                info!(target: "pool_command_executor", "List command received");
                self.list(cb);
            }
            PoolCommand::Close(handle, cb) => {
                info!(target: "pool_command_executor", "Close command received");
                self.close(handle, cb);
            }
            PoolCommand::CloseAck(handle, result) => {
                info!(target: "pool_command_executor", "CloseAck command received");
                match self.close_callbacks.try_borrow_mut() {
                    Ok(mut cbs) => {
                        match cbs.remove(&handle) {
                            Some(cb) => cb(result.map_err(IndyError::from)),
                            None => {
                                error!("Can't process PoolCommand::CloseAck for handle {} with result {:?} - appropriate callback not found!",
                                       handle, result);
                            }
                        }
                    }
                    Err(err) => { error!("{:?}", err); }
                }
            }
            PoolCommand::Refresh(handle, cb) => {
                info!(target: "pool_command_executor", "Refresh command received");
                self.refresh(handle, cb);
            }
            PoolCommand::RefreshAck(handle, result) => {
                info!(target: "pool_command_executor", "RefreshAck command received");
                match self.refresh_callbacks.try_borrow_mut() {
                    Ok(mut cbs) => {
                        match cbs.remove(&handle) {
                            Some(cb) => cb(result.map_err(IndyError::from)),
                            None => {
                                error!("Can't process PoolCommand::RefreshAck for handle {} with result {:?} - appropriate callback not found!",
                                       handle, result);
                            }
                        }
                    }
                    Err(err) => { error!("{:?}", err); }
                }
            }
        };
    }

    fn create(&self, name: &str, config: Option<&str>, cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self.pool_service.create(name, config).map_err(IndyError::from))
    }

    fn delete(&self, name: &str, cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self.pool_service.delete(name).map_err(IndyError::from));
    }

    fn open(&self, name: &str, config: Option<&str>, cb: Box<Fn(Result<i32, IndyError>) + Send>) {
        let result = self.pool_service.open(name, config)
            .map_err(|err| IndyError::PoolError(err))
            .and_then(|handle| {
                match self.open_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, handle)),
                    Err(err) => Err(IndyError::PoolError(PoolError::from(CommonError::from(err)))),
                }
            });
        match result {
            Err(err) => { cb(Err(err)); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, cb); /* TODO check if map contains same key */ }
        };
    }

    fn list(&self, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let res = self.pool_service.list()
            .and_then(|pools| ::serde_json::to_string(&pools).map_err(|err|
                PoolError::CommonError(CommonError::InvalidState(format!("Can't serialize pools list {}", err)))))
            .map_err(IndyError::from);
        cb(res)
    }

    fn close(&self, handle: i32, cb: Box<Fn(Result<(), IndyError>) + Send>) {
        let result = self.pool_service.close(handle)
            .map_err(From::from)
            .and_then(|handle| {
                match self.close_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, handle)),
                    Err(err) => Err(IndyError::PoolError(PoolError::from(CommonError::from(err))))
                }
            });
        match result {
            Err(err) => { cb(Err(err)); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, cb); /* TODO check if map contains same key */ }
        };
    }

    fn refresh(&self, handle: i32, cb: Box<Fn(Result<(), IndyError>) + Send>) {
        let result = self.pool_service.refresh(handle)
            .map_err(From::from)
            .and_then(|handle| {
                match self.refresh_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, handle)),
                    Err(err) => Err(IndyError::PoolError(PoolError::from(CommonError::from(err))))
                }
            });
        match result {
            Err(err) => { cb(Err(err)); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, cb); /* TODO check if map contains same key */ }
        };
    }
}