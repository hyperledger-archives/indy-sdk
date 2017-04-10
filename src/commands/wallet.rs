use errors::wallet::WalletError;

use services::pool::PoolService;
use services::wallet::WalletService;

use std::rc::Rc;

pub enum WalletCommand {
    Create(String, // pool name
           String, // wallet name
           String, // wallet type
           String, // wallet config
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
            WalletCommand::Create(pool_name, name, xtype, config, cb) => {
                info!(target: "wallet_command_executor", "Create command received");
                self.create(&pool_name, &name, &xtype, &config, cb);
            }
        };
    }

    fn create(&self, pool_name: &str, name: &str, xtype: &str, config: &str,
              cb: Box<Fn(Result<(), WalletError>) + Send>) {
        unimplemented!()
    }
}