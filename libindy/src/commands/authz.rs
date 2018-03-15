extern crate indy_crypto;

use self::indy_crypto::bn::{BigNumber};
use self::indy_crypto::utils::json::{JsonDecodable, JsonEncodable};
use errors::common::CommonError;
use errors::indy::IndyError;
use services::ledger::types::{Reply, };
use services::pool::PoolService;
use services::wallet::WalletService;
use services::signus::SignusService;
use services::ledger::LedgerService;
use services::authz::AuthzService;
use services::authz::types::{Policy, PolicyAgentInfo};

use services::authz::constants::AUTHZ_ADDRESS_WALLET_KEY_PREFIX;
//use services::anoncreds::constants::MASTER_SECRET_WALLET_KEY_PREFIX;

use std::error::Error;
use std::rc::Rc;
use std::str;
use std::cell::RefCell;

use commands::crypto::CryptoCommandExecutor;


use std::collections::HashMap;
use utils::crypto::base58::Base58;


use super::utils::check_wallet_and_pool_handles_consistency;


pub enum AuthzCommand {
    CreateAndStorePolicy(
        i32, // wallet handle
        Box<Fn(Result<String, IndyError>) + Send>), // Return policy address as String
    AddAgentToStoredPolicy(
        i32, // wallet handle
        String, // policy address
        String, // verkey
        bool, // add commitment to the signing of the given verkey
        Box<Fn(Result<String, IndyError>) + Send>), // Return agent verkey as String
    UpdateAgentWitness(
        i32, // wallet handle
        String, // policy address
        String, // verkey
        String, // the new witness
        Box<Fn(Result<String, IndyError>) + Send>), // Return agent verkey as String
    GetPolicy(
        i32, // wallet handle
        String,  // policy address
        Box<Fn(Result<String, IndyError>) + Send>), // Return policy as json encoded String
}

pub struct AuthzCommandExecutor {
    pool_service: Rc<PoolService>,
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<SignusService>,
    ledger_service: Rc<LedgerService>,
    authz_service: Rc<AuthzService>,
    deferred_commands: RefCell<HashMap<i32, AuthzCommand>>,
}

impl AuthzCommandExecutor {
    pub fn new(pool_service: Rc<PoolService>,
               wallet_service: Rc<WalletService>,
               crypto_service: Rc<SignusService>,
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
            AuthzCommand::AddAgentToStoredPolicy(wallet_handle, policy_addr, verkey, add_commitment, cb) => {
                info!("AddAgentToPolicy command received");
                cb(self.add_add_agent_to_policy(wallet_handle, &policy_addr,
                                                &verkey,
                                                add_commitment));
            }
            AuthzCommand::UpdateAgentWitness(wallet_handle, policy_addr, verkey, witness, cb) => {
                info!("AddAgentToPolicy command received");
                cb(self.update_agent_witness(wallet_handle, &policy_addr,
                                                &verkey, &witness));
            }
            AuthzCommand::GetPolicy(wallet_handle, policy_addr, cb) => {
                info!("GetPolicy command received");
                cb(self.get_policy_from_wallet(wallet_handle, policy_addr));
            }
        }
    }

    fn create_and_store_policy(&self, wallet_handle: i32) -> Result<String, IndyError> {
        // TODO: Maybe some validation for unique policy address
        let policy = self.authz_service.generate_new_policy()?;
        let s = self._set_policy_in_wallet(wallet_handle, policy)?;
        Ok(s)
    }

    /*fn add_add_agent_to_policy(&self, wallet_handle: i32,
                               policy_addr: &str,
                               key_json: Option<&str>,
                               master_secret_name: Option<&str>,) -> Result<String, IndyError> {
        let mut policy = self._get_policy_from_wallet(wallet_handle,
                                                      policy_addr.to_string())?;
        let verkey = if key_json.is_none() && master_secret_name.is_none() {
            let key = self.authz_service.add_new_agent_to_policy(&mut policy, None)?;
            key.verkey
        } else {
            let (seed, crypto_type) = match key_json {
                Some(info) => {
                    let key_info = SignusService::get_key_info_from_json(info.to_string())?;
                    (key_info.seed, key_info.crypto_type)
                }
                None => (None, None)
            };
            let master_secret = match master_secret_name {
                Some(name) => {
                    let master_secret = self.wallet_service.get(wallet_handle, &format!("{}::{}", MASTER_SECRET_WALLET_KEY_PREFIX, name))?;
                    Some(BigNumber::from_dec(&master_secret)?)
                }
                None => None
            };
            let agent_info = PolicyAgentInfo::new(seed, crypto_type, master_secret);
            let key = self.authz_service.add_new_agent_to_policy(&mut policy, Some(&agent_info))?;
            key.verkey
        };
        self._set_policy_in_wallet(wallet_handle, policy)?;
        Ok(verkey)
    }*/

    fn add_add_agent_to_policy(&self, wallet_handle: i32,
                               policy_addr: &str,
                               verkey: &str,
                               add_commitment: bool,) -> Result<String, IndyError> {
        let secret = if add_commitment {
            let k = CryptoCommandExecutor::__wallet_get_key(self.wallet_service.clone(),
                                                            wallet_handle, verkey)?;
            let sk = k.signkey;
            let sk_raw = Base58::decode(&sk)?;
            let sk_num = BigNumber::from_bytes(sk_raw.as_slice())?;
            Some(sk_num)
        } else {
            None
        };

        let mut policy = self._get_policy_from_wallet(wallet_handle,
                                                      policy_addr.to_string())?;
        self.authz_service.add_new_agent_to_policy_with_verkey(&mut policy, verkey.to_string(), secret)?;
        self._set_policy_in_wallet(wallet_handle, policy)?;
        Ok(verkey.to_string())
    }

    fn update_agent_witness(&self, wallet_handle: i32,
                               policy_addr: &str,
                               verkey: &str,
                               witness: &str,) -> Result<String, IndyError> {
        let mut policy = self._get_policy_from_wallet(wallet_handle,
                                                      policy_addr.to_string())?;
        let witness = BigNumber::from_dec(witness)?;
        self.authz_service.update_agent_witness(&mut policy, verkey.to_string(), &witness)?;
        self._set_policy_in_wallet(wallet_handle, policy)?;
        Ok(verkey.to_string())
    }

    fn get_policy_from_wallet(&self, wallet_handle: i32, policy_addr: String) -> Result<String, IndyError> {
        let res = self._get_policy_from_wallet(wallet_handle, policy_addr)?;
        let policy_json = Policy::to_json(&res).map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Can't serialize policy: {}", err.description())))?;
        Ok(policy_json)
    }

    fn _set_policy_in_wallet(&self, wallet_handle: i32, policy: Policy) -> Result<String, IndyError> {
        let policy_json = Policy::to_json(&policy)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Can't serialize policy: {}", err.description())))?;
        let addr_str = BigNumber::to_dec(&policy.address)?;
        let key = AuthzCommandExecutor::_policy_addr_to_wallet_key(addr_str);
        println!("Setting key {:?} value {:?}", &key, &policy_json);
        self.wallet_service.set(wallet_handle, &key, &policy_json)?;
        Ok(policy_json)
    }

    fn _get_policy_from_wallet(&self, wallet_handle: i32, policy_addr: String) -> Result<Policy, IndyError> {
        let key = AuthzCommandExecutor::_policy_addr_to_wallet_key(policy_addr);
        println!("Getting key {:?}", &key);
        let value = self.wallet_service.get(wallet_handle, &key)?;
        let policy = Policy::from_json(&value)
            .map_err(map_err_trace!())
            .map_err(|err|
                CommonError::InvalidStructure(
                    format!("Can't deserialize policy: {}", err.description())))?;
        Ok(policy)
    }

    pub fn _policy_addr_to_wallet_key(policy_address: String) -> String {
        format!("{}::{:}", AUTHZ_ADDRESS_WALLET_KEY_PREFIX, policy_address)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use errors::wallet::WalletError;
    use utils::inmem_wallet::InmemWallet;
    use utils::test::TestUtils;

    use commands::{Command, CommandExecutor};


    // TODO: Fix tests

    #[test]
    fn create_new_policy_works() {
        TestUtils::cleanup_indy_home();

        let wallet_service = WalletService::new();
        wallet_service.create("pool1", Some("default"), "wallet1", None, None).unwrap();
        let wallet_handle = wallet_service.open("wallet1", None, None).unwrap();

        let result1 = CommandExecutor::instance()
            .send(Command::Authz(AuthzCommand::CreateAndStorePolicy(
                wallet_handle,
                Box::new(move |result| {
                    println!("result is {:?}", result);
                })
            )));

        println!("{:?}", result1);
        let result2 = CommandExecutor::instance()
            .send(Command::Authz(AuthzCommand::GetPolicy(
                wallet_handle,
                "a".to_string(),
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