extern crate indy_crypto;

use self::indy_crypto::bn::{BigNumber};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::authz::AuthzError;
use errors::wallet::WalletError;
use errors::indy::IndyError;
use services::crypto::types::{KeyInfo, Key};
use services::ledger::types::{Reply, };
use services::pool::PoolService;
use services::wallet::WalletService;
use services::crypto::CryptoService;
use services::ledger::LedgerService;
use services::authz::AuthzService;
use services::authz::constants::AUTHZ_ADDRESS_WALLET_KEY_PREFIX;
use services::authz::types::Policy;

use std::error::Error;
use std::rc::Rc;
use std::str;
use std::cell::RefCell;

use commands::ledger::LedgerCommand;
use commands::{Command, CommandExecutor};
use std::collections::HashMap;
use utils::sequence::SequenceUtils;
use utils::crypto::base58::Base58;


use super::utils::check_wallet_and_pool_handles_consistency;


pub enum AuthzCommand {
    CreateAndStorePolicy(
        i32, // wallet handle
        Box<Fn(Result<String, IndyError>) + Send>), // Return policy address as String
    /*GetPolicy(
        i32, // wallet handle
        String,  // policy address
        Box<Fn(Result<String, IndyError>) + Send>), // Return policy as json encoded String*/
}

pub struct AuthzCommandExecutor {
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    ledger_service: Rc<LedgerService>,
    authz_service: Rc<AuthzService>,
    deferred_commands: RefCell<HashMap<i32, AuthzCommand>>,
}

impl AuthzCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>,
               crypto_service: Rc<CryptoService>,
               ledger_service: Rc<LedgerService>,
               authz_service: Rc<AuthzService>) -> AuthzCommandExecutor {
        AuthzCommandExecutor {
            pool_service,
            wallet_service,
            crypto_service,
            ledger_service,
            authz_service,
            deferred_commands: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, command: AuthzCommand) {
        match command {
            AuthzCommand::CreateAndStorePolicy(wallet_handle, cb) => {
                info!("CreateAndStorePolicyAddress command received");
                cb(self.create_and_store_policy(wallet_handle));
            }
            /*AuthzCommand::GetPolicy(wallet_handle, policy_addr, cb) => {
                info!("CreateAndStorePolicyAddress command received");
                cb(self.get_policy_from_wallet(wallet_handle, policy_addr));
            }*/
        }
    }

    fn create_and_store_policy(&self, wallet_handle: i32) -> Result<String, IndyError> {
        /*if let Some(ref did) = my_did_info.did.as_ref() {
            if self.wallet_service.get(wallet_handle, &format!("my_did::{}", did)).is_ok() {
                return Err(IndyError::DidError(DidError::AlreadyExistsError(format!("Did already exists"))));
            };
        }*/
        // TODO: Maybe some validation
        let policy = self.authz_service.generate_new_policy()?;
        let s = self._set_policy_in_wallet(wallet_handle, policy)?;
        Ok(s)
    }

    /*fn get_policy_from_wallet(&self, wallet_handle: i32, policy_addr: String) -> Result<String, AuthzError> {
        let res = self._get_policy_from_wallet(wallet_handle, BigNumber::from_dec(&policy_addr)?)?;
        let policy_json = Policy::to_json(res).map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Can't serialize policy: {}", err.description())))?;
        Ok(policy_json)
    }*/

    /*fn _add_new_policy_in_wallet(&self, wallet_handle: i32) -> Result<String, AuthzError> {
//        let address = generate_policy_address()?;
//        let policy = Policy::new(address, None);
        // TODO: Should have a check of duplicate policy address

        let s = self._set_policy_in_wallet(wallet_handle, policy)?;
        Ok(s)
    }*/

    fn _set_policy_in_wallet(&self, wallet_handle: i32, policy: Policy) -> Result<String, IndyError> {
        let policy_json = Policy::to_json(&policy)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Can't serialize policy: {}", err.description())))?;
        let addr_str = BigNumber::to_dec(&policy.address)?;
        self.wallet_service.set(wallet_handle, &format!("{}::{}",
                                                        AUTHZ_ADDRESS_WALLET_KEY_PREFIX,
                                                        addr_str),&policy_json)?;
        Ok(policy_json)
    }

    /*fn _get_policy_from_wallet(&self, wallet_handle: i32, policy_addr: BigNumber) -> Result<Policy, AuthzError> {
        let value = self.wallet_service.get(wallet_handle, &format!("{}::{}",
                                                        AUTHZ_ADDRESS_WALLET_KEY_PREFIX,
                                                        policy_addr))?;
        let policy = Policy::from_json(value)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Can't deserialize policy: {}", err.description())))?;
        Ok(policy)
    }*/
}


#[cfg(test)]
mod tests {
    use super::*;
    use errors::wallet::WalletError;
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;

    use commands::{Command, CommandExecutor};

    #[test]
    fn create_new_policy_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", Some("default"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let result = CommandExecutor::instance()
            .send(Command::Authz(AuthzCommand::CreateAndStorePolicy(
                wallet_handle,
                Box::new(move |result| {
                    println!("result is {:?}", result);
                })
            )));

        TestUtils::cleanup_indy_home();
    }

    /*#[test]
    fn get_newly_created_policy_works() {
        TestUtils::cleanup_indy_home();

        // TODO: Remove duplicated code
        let wallet_service = WalletService::new();
        wallet_service.create("pool1", Some("default"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let result = CommandExecutor::instance()
            .send(Command::AuthzCommand(AuthzCommandExecutor::CreateAndStorePolicyAddress(
                wallet_handle,
                Box::new(move |result| {
                    println!("result is {:?}", result);
                })
            )));

        let result = CommandExecutor::instance()
            .send(Command::AuthzCommand(AuthzCommandExecutor::CreateAndStorePolicyAddress(
                wallet_handle,
                Box::new(move |result| {
                    println!("result is {:?}", result);
                })
            )));

        TestUtils::cleanup_indy_home();
    }*/
}
