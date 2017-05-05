use errors::wallet::WalletError;

use services::wallet::WalletService;

use std::rc::Rc;

pub enum WalletCommand {
    Create(String, // pool name
           String, // wallet name
           Option<String>, // wallet type
           Option<String>, // wallet config
           Option<String>, // wallet credentials
           Box<Fn(Result<(), WalletError>) + Send>),
    Open(String, // wallet name
         Option<String>, // wallet runtime config
         Option<String>, // wallet credentials
         Box<Fn(Result<i32, WalletError>) + Send>),
    Close(i32, // handle
          Box<Fn(Result<(), WalletError>) + Send>),
    Delete(String, // name
           Option<String>, // wallet credentials
           Box<Fn(Result<(), WalletError>) + Send>),
    SetSeqNoForValue(i32, // wallet handle
                     String, // wallet key
                     i32, // sequence number
                     Box<Fn(Result<(), WalletError>) + Send>)
}

pub struct WalletCommandExecutor {
    wallet_service: Rc<WalletService>
}

impl WalletCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>) -> WalletCommandExecutor {
        WalletCommandExecutor {
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
            WalletCommand::Open(name, runtime_config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Open command received");
                self.open(&name, runtime_config.as_ref().map(String::as_str),
                          credentials.as_ref().map(String::as_str), cb);
            }
            WalletCommand::Close(handle, cb) => {
                info!(target: "wallet_command_executor", "Close command received");
                self.close(handle, cb);
            }
            WalletCommand::Delete(name, credentials, cb) => {
                info!(target: "wallet_command_executor", "Delete command received");
                self.delete(&name, credentials.as_ref().map(String::as_str), cb);
            }
            WalletCommand::SetSeqNoForValue(handle, key, seq_no, cb) => {
                info!(target: "wallet_command_executor", "SetSeqNoForValue command received");
                self.set_seq_no_for_value(handle, &key, seq_no, cb);
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
            name: &str,
            runtime_config: Option<&str>,
            credentials: Option<&str>,
            cb: Box<Fn(Result<i32, WalletError>) + Send>) {
        cb(self.wallet_service.open(name, runtime_config, credentials));
    }

    fn close(&self,
             handle: i32,
             cb: Box<Fn(Result<(), WalletError>) + Send>) {
        cb(self.wallet_service.close(handle));
    }

    fn delete(&self,
              handle: &str,
              credentials: Option<&str>,
              cb: Box<Fn(Result<(), WalletError>) + Send>) {
        cb(self.wallet_service.delete(handle, credentials));
    }

    fn set_seq_no_for_value(&self,
                            handle: i32,
                            key: &str,
                            seq_no: i32,
                            cb: Box<Fn(Result<(), WalletError>) + Send>) {
        cb(self.wallet_service.set(handle, &format!("seq_no::{}", seq_no), key));
    }
}