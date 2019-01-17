use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use domain::ledger::request::ProtocolVersion;
use domain::pool::{PoolConfig, PoolOpenConfig};
use errors::prelude::*;
use services::pool::PoolService;

pub enum PoolCommand {
    Create(
        String, // name
        Option<PoolConfig>, // config
        Box<Fn(IndyResult<()>) + Send>),
    Delete(
        String, // name
        Box<Fn(IndyResult<()>) + Send>),
    Open(
        String, // name
        Option<PoolOpenConfig>, // config
        Box<Fn(IndyResult<i32>) + Send>),
    OpenAck(
        i32, // cmd id
        i32, // pool handle
        IndyResult<()>),
    List(Box<Fn(IndyResult<String>) + Send>),
    Close(
        i32, // pool handle
        Box<Fn(IndyResult<()>) + Send>),
    CloseAck(i32,
             IndyResult<()>),
    Refresh(
        i32, // pool handle
        Box<Fn(IndyResult<()>) + Send>),
    RefreshAck(i32,
               IndyResult<()>),
    SetProtocolVersion(
        usize, // protocol version
        Box<Fn(IndyResult<()>) + Send>),
}

pub struct PoolCommandExecutor {
    pool_service: Rc<PoolService>,
    close_callbacks: RefCell<HashMap<i32, Box<Fn(IndyResult<()>)>>>,
    refresh_callbacks: RefCell<HashMap<i32, Box<Fn(IndyResult<()>)>>>,
    open_callbacks: RefCell<HashMap<i32, Box<Fn(IndyResult<i32>)>>>,
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
                cb(self.create(&name, config));
            }
            PoolCommand::Delete(name, cb) => {
                info!(target: "pool_command_executor", "Delete command received");
                cb(self.delete(&name));
            }
            PoolCommand::Open(name, config, cb) => {
                info!(target: "pool_command_executor", "Open command received");
                self.open(&name, config, cb);
            }
            PoolCommand::OpenAck(handle, pool_id, result) => {
                info!("OpenAck handle {:?}, pool_id {:?}, result {:?}", handle, pool_id, result);
                match self.open_callbacks.try_borrow_mut() {
                    Ok(mut cbs) => {
                        match cbs.remove(&handle) {
                            Some(cb) => {
                                cb(result.and_then(|_| self.pool_service.add_open_pool(pool_id)))
                            }
                            None => {
                                error!("Can't process PoolCommand::OpenAck for handle {} with result {:?} - appropriate callback not found!", handle, result);
                            }
                        }
                    }
                    Err(err) => { error!("{:?}", err); }
                }
            }
            PoolCommand::List(cb) => {
                info!(target: "pool_command_executor", "List command received");
                cb(self.list());
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
                                error!("Can't process PoolCommand::CloseAck for handle {} with result {:?} - appropriate callback not found!", handle, result);
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
                            Some(cb) => cb(result),
                            None => {
                                error!("Can't process PoolCommand::RefreshAck for handle {} with result {:?} - appropriate callback not found!",
                                       handle, result);
                            }
                        }
                    }
                    Err(err) => { error!("{:?}", err); }
                }
            }
            PoolCommand::SetProtocolVersion(protocol_version, cb) => {
                info!(target: "pool_command_executor", "SetProtocolVersion command received");
                cb(self.set_protocol_version(protocol_version));
            }
        };
    }

    fn create(&self, name: &str, config: Option<PoolConfig>) -> IndyResult<()> {
        debug!("create >>> name: {:?}, config: {:?}", name, config);

        let res = self.pool_service.create(name, config)?;

        debug!("create << res: {:?}", res);

        Ok(res)
    }

    fn delete(&self, name: &str) -> IndyResult<()> {
        debug!("delete >>> name: {:?}", name);

        let res = self.pool_service.delete(name)?;

        debug!("delete << res: {:?}", res);

        Ok(res)
    }

    fn open(&self, name: &str, config: Option<PoolOpenConfig>, cb: Box<Fn(IndyResult<i32>) + Send>) {
        debug!("open >>> name: {:?}, config: {:?}", name, config);

        let result = self.pool_service.open(name, config)
            .and_then(|handle| {
                match self.open_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, handle)),
                    Err(err) => Err(err.into()),
                }
            });
        match result {
            Err(err) => { cb(Err(err)); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, cb); /* TODO check if map contains same key */ }
        };

        debug!("open <<<");
    }

    fn list(&self) -> IndyResult<String> {
        debug!("list >>> ");

        let res = self.pool_service
            .list()
            .and_then(|pools| ::serde_json::to_string(&pools)
                .to_indy(IndyErrorKind::InvalidState, "Can't serialize pools list"))?;

        debug!("list << res: {:?}", res);
        Ok(res)
    }

    fn close(&self, handle: i32, cb: Box<Fn(IndyResult<()>) + Send>) {
        debug!("close >>> handle: {:?}", handle);

        let result = self.pool_service.close(handle)
            .and_then(|handle| {
                match self.close_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, handle)),
                    Err(err) => Err(err.into())
                }
            });
        match result {
            Err(err) => { cb(Err(err)); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, cb); /* TODO check if map contains same key */ }
        };

        debug!("close <<<");
    }

    fn refresh(&self, handle: i32, cb: Box<Fn(IndyResult<()>) + Send>) {
        debug!("refresh >>> handle: {:?}", handle);

        let result = self.pool_service.refresh(handle)
            .and_then(|handle| {
                match self.refresh_callbacks.try_borrow_mut() {
                    Ok(cbs) => Ok((cbs, handle)),
                    Err(err) => Err(err.into())
                }
            });
        match result {
            Err(err) => { cb(Err(err)); }
            Ok((mut cbs, handle)) => { cbs.insert(handle, cb); /* TODO check if map contains same key */ }
        };

        debug!("refresh <<<");
    }

    fn set_protocol_version(&self, version: usize) -> IndyResult<()> {
        debug!("set_protocol_version >>> version: {:?}", version);

        if version != 1 && version != 2 {
            return Err(err_msg(IndyErrorKind::PoolIncompatibleProtocolVersion, format!("Unsupported Protocol version: {}", version)));
        }

        ProtocolVersion::set(version);

        debug!("set_protocol_version <<<");

        Ok(())
    }
}