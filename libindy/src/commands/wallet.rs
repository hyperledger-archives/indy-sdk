extern crate libc;
extern crate serde_json;

use api::ErrorCode;
use errors::indy::IndyError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use services::wallet::WalletService;
use std::rc::Rc;

use self::libc::c_char;

pub enum WalletCommand {
    RegisterWalletType(String, // xtype
                       extern fn(name: *const c_char,
                                 config: *const c_char,
                                 credentials: *const c_char) -> ErrorCode, // create
                       extern fn(name: *const c_char,
                                 config: *const c_char,
                                 runtime_config: *const c_char,
                                 credentials: *const c_char,
                                 handle: *mut i32) -> ErrorCode, // open
                       extern fn(handle: i32,
                                 key: *const c_char,
                                 value: *const c_char) -> ErrorCode, // set
                       extern fn(handle: i32,
                                 key: *const c_char,
                                 value_ptr: *mut *const c_char) -> ErrorCode, // get
                       extern fn(handle: i32,
                                 key: *const c_char,
                                 value_ptr: *mut *const c_char) -> ErrorCode, // get_not_expired
                       extern fn(handle: i32,
                                 key_prefix: *const c_char,
                                 values_json_ptr: *mut *const c_char) -> ErrorCode, // list
                       extern fn(handle: i32) -> ErrorCode, // close
                       extern fn(name: *const c_char,
                                 config: *const c_char,
                                 credentials: *const c_char) -> ErrorCode, // delete
                       extern fn(wallet_handle: i32, str: *const c_char) -> ErrorCode, // free
                       Box<Fn(Result<(), IndyError>) + Send>),
    Create(String, // pool name
           String, // wallet name
           Option<String>, // wallet type
           Option<String>, // wallet config
           Option<String>, // wallet credentials
           Box<Fn(Result<(), IndyError>) + Send>),
    Open(String, // wallet name
         Option<String>, // wallet runtime config
         Option<String>, // wallet credentials
         Box<Fn(Result<i32, IndyError>) + Send>),
    Close(i32, // handle
          Box<Fn(Result<(), IndyError>) + Send>),
    ListWallets(Box<Fn(Result<String, IndyError>) + Send>),
    Delete(String, // name
           Option<String>, // wallet credentials
           Box<Fn(Result<(), IndyError>) + Send>)
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
            WalletCommand::RegisterWalletType(xtype, create, open, set, get,
                                              get_not_expired, list, close, delete, free, cb) => {
                info!(target: "wallet_command_executor", "RegisterWalletType command received");
                self.register_type(&xtype, create, open, set,
                                   get, get_not_expired, list, close, delete, free, cb);
            }
            WalletCommand::Create(pool_name, name, xtype, config, credentials, cb) => {
                info!(target: "wallet_command_executor", "Create command received");
                self.create(&pool_name, &name, xtype.as_ref().map(String::as_str),
                            config.as_ref().map(String::as_str),
                            credentials.as_ref().map(String::as_str), cb);
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
            WalletCommand::ListWallets(cb) => {
                info!(target: "wallet_command_executor", "ListWallets command received");
                self.list_wallets(cb);
            }
            WalletCommand::Delete(name, credentials, cb) => {
                info!(target: "wallet_command_executor", "Delete command received");
                self.delete(&name, credentials.as_ref().map(String::as_str), cb);
            }
        };
    }

    fn register_type(&self,
                     xtype: &str,
                     create: extern fn(name: *const c_char,
                                       config: *const c_char,
                                       credentials: *const c_char) -> ErrorCode,
                     open: extern fn(name: *const c_char,
                                     config: *const c_char,
                                     runtime_config: *const c_char,
                                     credentials: *const c_char,
                                     handle: *mut i32) -> ErrorCode,
                     set: extern fn(handle: i32,
                                    key: *const c_char,
                                    value: *const c_char) -> ErrorCode,
                     get: extern fn(handle: i32,
                                    key: *const c_char,
                                    value_ptr: *mut *const c_char) -> ErrorCode,
                     get_not_expired: extern fn(handle: i32,
                                                key: *const c_char,
                                                value_ptr: *mut *const c_char) -> ErrorCode,
                     list: extern fn(handle: i32,
                                     key_prefix: *const c_char,
                                     values_json_ptr: *mut *const c_char) -> ErrorCode,
                     close: extern fn(handle: i32) -> ErrorCode,
                     delete: extern fn(name: *const c_char,
                                       config: *const c_char,
                                       credentials: *const c_char) -> ErrorCode,
                     free: extern fn(wallet_handle: i32,
                                     value: *const c_char) -> ErrorCode,
                     cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self
            .wallet_service
            .register_type(
                xtype, create, open, set,
                get, get_not_expired,
                list, close, delete, free)
            .map_err(IndyError::from));
    }

    fn create(&self,
              pool_name: &str,
              name: &str,
              xtype: Option<&str>,
              config: Option<&str>,
              credentials: Option<&str>,
              cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self.wallet_service.create(pool_name, xtype, name, config, credentials)
            .map_err(|err| IndyError::WalletError(err)));
    }

    fn open(&self,
            name: &str,
            runtime_config: Option<&str>,
            credentials: Option<&str>,
            cb: Box<Fn(Result<i32, IndyError>) + Send>) {
        cb(self.wallet_service.open(name, runtime_config, credentials)
            .map_err(|err| IndyError::WalletError(err)));
    }

    fn close(&self,
             handle: i32,
             cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self.wallet_service.close(handle)
            .map_err(|err| IndyError::WalletError(err)));
    }

    fn list_wallets(&self, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let result = self.wallet_service.list_wallets()
            .and_then(|wallets|
                serde_json::to_string(&wallets)
                    .map_err(|err|
                        WalletError::CommonError(CommonError::InvalidState(format!("Can't serialize wallets list {}", err)))))
            .map_err(IndyError::from);
        cb(result)
    }

    fn delete(&self,
              handle: &str,
              credentials: Option<&str>,
              cb: Box<Fn(Result<(), IndyError>) + Send>) {
        cb(self.wallet_service.delete(handle, credentials)
            .map_err(|err| IndyError::WalletError(err)));
    }
}