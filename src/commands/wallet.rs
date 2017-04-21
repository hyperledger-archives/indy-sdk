use errors::wallet::WalletError;

use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum WalletCommand {
    Create(String, // pool name
           String, // wallet name
           Option<String>, // wallet type
           Option<String>, // wallet config
           Option<String>, // wallet credentials
           Box<Fn(Result<(), WalletError>) + Send>),
    Open(i32, // pool handle
         String, // wallet name
         Option<String>, // wallet config
         Option<String>, // wallet credentials
         Box<Fn(Result<i32, WalletError>) + Send>),
    Close(i32, // handle
          Box<Fn(Result<(), WalletError>) + Send>),
    Delete(String, // name
           Box<Fn(Result<(), WalletError>) + Send>),
    SetSeqNoForValue(i32, // wallet handle
                     String, // wallet key
                     Box<Fn(Result<(), WalletError>) + Send>)
}

pub struct WalletCommandExecutor {
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>
}

impl WalletCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>, wallet_service: Rc<WalletService>) -> WalletCommandExecutor {
        WalletCommandExecutor {
            pool_service: pool_service,
            wallet_service: wallet_service
        }
    }

    pub fn execute(&self, command: WalletCommand) {
        match command {
            WalletCommand::Create(pool_name, name, xtype, config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Create command received");
                self.create(&pool_name, &name, xtype.as_ref().map(String::as_str),
                            config.as_ref().map(String::as_str), credentials.as_ref().map(String::as_str), cb);
            }
            WalletCommand::Open(pool_handle, name, config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Open command received");
                self.open(pool_handle, &name, config.as_ref().map(String::as_str),
                          credentials.as_ref().map(String::as_str), cb);
            }
            WalletCommand::Close(handle, cb) => {
                info!(target: "wallet_command_executor", "Close command received");
                self.close(handle, cb);
            }
            WalletCommand::Delete(name, cb) => {
                info!(target: "wallet_command_executor", "Delete command received");
                self.delete(&name, cb);
            }
            WalletCommand::SetSeqNoForValue(wallet_handle, wallet_key, cb) => {
                info!(target: "wallet_command_executor", "SetSeqNoForValue command received");
                self.set_seq_no_for_value(wallet_handle, &wallet_key, cb);
            }
        };
    }

    fn create(&self,
              pool_name: &str,
              name: &str,
              xtype: Option<&str>,
              config: Option<&str>,
              credentials: Option<&str>,
              cb: Box<Fn(Result<(), WalletError>) + Send>) {
        cb(self.wallet_service.create(pool_name, xtype, name, config, credentials));
    }

    fn open(&self,
            pool_handle: i32,
            name: &str,
            config: Option<&str>,
            credentials: Option<&str>,
            cb: Box<Fn(Result<i32, WalletError>) + Send>) {
        //let pool_name = "sandbox"; // TODO: FIXME: Change to pool_service.get_name(handle);
        //cb(self.wallet_service.open(pool_name, name, config, credentials));
        cb(Ok(0))
    }

    fn close(&self,
             handle: i32,
             cb: Box<Fn(Result<(), WalletError>) + Send>) {
        cb(Ok(()));
    }

    fn delete(&self,
              handle: &str,
              cb: Box<Fn(Result<(), WalletError>) + Send>) {
        cb(Ok(()));
    }

    fn set_seq_no_for_value(&self,
                            wallet_handle: i32,
                            wallet_key: &str,
                            cb: Box<Fn(Result<(), WalletError>) + Send>) {
        cb(Ok(()));
    }
}