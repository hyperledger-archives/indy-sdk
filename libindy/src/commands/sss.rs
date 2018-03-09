extern crate indy_crypto;

use self::indy_crypto::bn::{BigNumber};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::authz::AuthzError;
use errors::wallet::WalletError;
use errors::indy::IndyError;
use services::signus::types::{KeyInfo, Key};
use services::ledger::types::{Reply, };
use services::pool::PoolService;
use services::wallet::WalletService;
use services::signus::SignusService;
use services::ledger::LedgerService;
use services::sss::SSSService;

use services::anoncreds::constants::MASTER_SECRET_WALLET_KEY_PREFIX;

use std::error::Error;
use std::rc::Rc;
use std::str;
use std::cell::RefCell;

use commands::ledger::LedgerCommand;
use commands::{Command, CommandExecutor};
use commands::crypto::CryptoCommandExecutor;


use std::collections::HashMap;
use utils::sequence::SequenceUtils;
use utils::crypto::base58::Base58;


use super::utils::check_wallet_and_pool_handles_consistency;

pub enum SSSCommand {

}

pub struct SSSCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<SignusService>,
    sss_service: Rc<SSSService>,
    deferred_commands: RefCell<HashMap<i32, SSSCommand>>,
}

impl SSSCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
               crypto_service: Rc<SignusService>,
               sss_service: Rc<SSSService>) -> SSSCommandExecutor {
        SSSCommandExecutor {
            wallet_service,
            crypto_service,
            sss_service,
            deferred_commands: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: SSSCommand) {

    }
}