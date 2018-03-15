extern crate indy_crypto;
extern crate serde_json;

use self::indy_crypto::sss::{shard_secret, recover_secret, Share};
use errors::indy::IndyError;
use services::wallet::WalletService;
use services::signus::SignusService;

use std::error::Error;
use std::rc::Rc;
use std::str;
use std::cell::RefCell;

use serde_json::{Value, Map};

use commands::crypto::CryptoCommandExecutor;

use utils::crypto::base58::Base58;
use utils::crypto::box_::CryptoBox;

use super::utils::check_wallet_and_pool_handles_consistency;

pub const SSS_WALLET_KEY_PREFIX: &'static str = "sss";
pub const SSS_SECRET_NAME_IN_SHARD: &'static str = "__key__";


pub enum SSSCommand {
    ShardMsgWithSecretAndStoreShards(
        i32, // wallet handle,
        usize, // m (threshold)
        usize,  // n (total shards)
        Option<String>, // msg as JSON
        String, // verkey for which secret key has to be sharded
        Box<Fn(Result<String, IndyError>) + Send>), // Return the id as String by which all shards can be retrieved
    GetShardsOfVerkey(
        i32, // wallet handle,
        String, // verkey for which secret key has to be sharded
        Box<Fn(Result<String, IndyError>) + Send>), // Return the list of shards as JSON
    RecoverSecretFromShards(
        String, // shards as JSON array with each shard as an element
        Box<Fn(Result<String, IndyError>) + Send>), // Return the secret in base58 format
}

pub struct SSSCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<SignusService>
}

impl SSSCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
               crypto_service: Rc<SignusService>) -> SSSCommandExecutor {
        SSSCommandExecutor {
            wallet_service,
            crypto_service
        }
    }

    pub fn execute(&self, command: SSSCommand) {
        match command {
            SSSCommand::ShardMsgWithSecretAndStoreShards(wallet_handle, m, n, msg, verkey, cb) => {
                info!("ShardVerkeyAndStoreShards command received");
                cb(self.shard_msg_secret_and_store_shards(wallet_handle, m, n, msg.as_ref().map(String::as_str), &verkey));
            }
            SSSCommand::GetShardsOfVerkey(wallet_handle, verkey, cb) => {
                info!("GetShardsOfVerkey command received");
                cb(self.get_shards_of_verkey(wallet_handle, &verkey));
            }
            SSSCommand::RecoverSecretFromShards(shards_json, cb) => {
                info!("RecoverSecretFromShards command received");
                cb(self.recover_secret_from_shards(&shards_json));
            }
        }
    }

    // Computes the seed corresponding to the given verkey, updates the `msg` JSON (empty JSON) if `msg` is None
    fn shard_msg_secret_and_store_shards(&self, wallet_handle: i32, m: usize, n: usize, msg: Option<&str>, verkey: &str) -> Result<String, IndyError> {
        let mut msg: Map<String, Value> = match msg {
            Some(s) => {
                let mut v: Value = serde_json::from_str(s)?;
                v.as_object_mut().unwrap().clone()
            }
            None => Map::new()
        };

        self.update_msg_with_secret_key(wallet_handle, &mut msg, verkey)?;

        let updated_json = json!(msg).to_string();
        let shares = shard_secret(m, n, &updated_json.as_bytes().to_vec(), false)?;
        let shares_json = json!(shares).to_string();
        let wallet_key = SSSCommandExecutor::_verkey_to_wallet_key(&verkey);
        self.wallet_service.set(wallet_handle, &wallet_key, &shares_json)?;
        Ok(verkey.to_string())
    }

    fn get_shards_of_verkey(&self, wallet_handle: i32, verkey: &str) -> Result<String, IndyError> {
        let wallet_key = SSSCommandExecutor::_verkey_to_wallet_key(&verkey);
        Ok(self.wallet_service.get(wallet_handle, &wallet_key)?)
    }

    fn recover_secret_from_shards(&self, shards_json: &str) -> Result<String, IndyError> {
        let shards: Vec<Share> = serde_json::from_str(shards_json)?;
        let recovered_secret = recover_secret(shards, false)?;
        Ok(str::from_utf8(&recovered_secret)?.to_string())
    }

    fn update_msg_with_secret_key(&self, wallet_handle: i32, msg: &mut Map<String, Value>, verkey: &str) -> Result<(), IndyError> {
        let k = CryptoCommandExecutor::__wallet_get_key(self.wallet_service.clone(),
                                                        wallet_handle, verkey)?;
        let sk = Base58::decode(&k.signkey)?;
        let seed = CryptoBox::ed25519_sk_to_seed(&Vec::from(&sk as &[u8]))?;
        msg.insert(SSSCommandExecutor::_secret_key_in_msg(verkey), serde_json::Value::String(Base58::encode(&seed)));
        Ok(())
    }

    fn _secret_key_in_msg(secret_name: &str) -> String {
        format!("{}::{}", SSS_SECRET_NAME_IN_SHARD, secret_name)
    }

    fn _verkey_to_wallet_key(verkey: &str) -> String {
        format!("{}::{}", SSS_WALLET_KEY_PREFIX, verkey)
    }
}