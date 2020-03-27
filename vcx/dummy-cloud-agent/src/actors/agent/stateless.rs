use std::collections::HashMap;
use std::convert::Into;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use actix::prelude::*;
use failure::{Error, Fail};
use futures::*;
use serde_json;

use crate::actors::{HandleA2AMsg, HandleAdminMessage};
use crate::actors::admin::Admin;
use crate::actors::agent::agent::Agent;
use crate::actors::agent_connection::agent_connection::AgentConnection;
use crate::actors::router::Router;
use crate::domain::a2a::ConfigOption;
use crate::domain::admin_message::{ResAdminQuery, ResQueryAgent};
use crate::domain::config::WalletStorageConfig;
use crate::domain::invite::ForwardAgentDetail;
use crate::domain::key_derivation::KeyDerivationDirective;
use crate::indy::{did, ErrorCode, IndyError, wallet, WalletHandle};
use crate::utils::config_env::*;
use crate::utils::futures::*;
use crate::utils::rand;
use crate::utils::wallet::build_wallet_credentials;

impl Agent {

    pub fn load_configs(agent_wallet_handle: WalletHandle, agent_did: String) -> BoxedFuture<HashMap<String, String>, Error> {
        did::get_did_metadata(agent_wallet_handle, &agent_did)
            .then(|res| match res {
                Err(IndyError { error_code: ErrorCode::WalletItemNotFound, .. }) => Ok("{}".to_string()),
                r => r
            })
            .map(move |metadata| {
                let configs: HashMap<String, String> = serde_json::from_str(&metadata).expect("Can't restore Agent config.");
                configs
            })
            .map_err(|err| err.context("The provided DID was not found in provided wallet.").into())
            .into_box()
    }

    pub(crate) fn load_config(agent_wallet_handle: WalletHandle, agent_did: String, key: String) -> ResponseFuture<Option<String>, Error> {
        Self::load_configs(agent_wallet_handle, agent_did)
            .map(move |configs| {
                let m = configs.get(&key);
                match m {
                    None => None,
                    Some(m) => Some(m.clone())
                }
            })
            .into_box()
    }

    pub fn remove_configs(agent_wallet_handle: WalletHandle, agent_did: String, configs_to_remove: Vec<String>) -> BoxedFuture<(), Error> {
        Self::load_configs(agent_wallet_handle, agent_did.clone())
            .and_then(move |mut configs| {
                for config_to_remove in configs_to_remove {
                    configs.remove(&config_to_remove);
                }
                Self::set_configs(agent_wallet_handle, agent_did, configs)
            })
            .into_box()
    }

    pub fn insert_configs(agent_wallet_handle: WalletHandle, agent_did: String, config_options: Vec<ConfigOption>) -> BoxedFuture<(), Error> {
        let filtered_new: Vec<ConfigOption> = config_options
            .into_iter()
            .filter(|config_option| {
                match config_option.name.as_str() {
                    "name" | "logoUrl" | "notificationWebhookUrl" => true,
                    _ => {
                        warn!("Agent was trying to set up unsupported agent configuration option {}", config_option.name.as_str());
                        false
                    }
                }
            })
            .collect();
        Self::load_configs(agent_wallet_handle, agent_did.clone())
            .and_then(move |mut configs| {
                for new_config_option in filtered_new {
                    configs.insert(new_config_option.name, new_config_option.value);
                }
                Self::set_configs(agent_wallet_handle, agent_did, configs)
            })
            .into_box()
    }

    pub fn set_configs(agent_wallet_handle: WalletHandle, agent_did: String, new_config_options: HashMap<String, String>) -> BoxedFuture<(), Error> {
        let config_metadata = serde_json::to_string(&new_config_options).unwrap();
        did::set_did_metadata(agent_wallet_handle, &agent_did, &config_metadata)
            .map_err(|err| err.context("Can't store config data as DID metadata.").into())
            .into_box()
    }
}


#[cfg(test)]
mod tests {
    use failure::{Error, Fail};

    use crate::actors::ForwardA2AMsg;
    use crate::domain::a2a::{ConfigOption, GetMessagesDetailResponse, MessageDetailPayload, RemoteMessageType};
    use crate::domain::a2connection::MessagesByConnection;
    use crate::domain::status::MessageStatusCode;
    use crate::indy::{did, ErrorCode, IndyError, wallet, WalletHandle};
    use crate::utils::tests::*;
    use crate::utils::to_i8;

    use super::*;

    pub const ANONCREDS_WALLET_CONFIG: &'static str = r#"{"id": "anoncreds_wallet"}"#;
    pub const WALLET_CREDENTIALS: &'static str = r#"{"key":"8dvfYSt5d1taSd6yJdpjq4emkwsPDDLYxkNFysFD2cZY", "key_derivation_method":"RAW"}"#;

    #[test]
    fn update_configs_works() {
        let (loaded_configs, agent_did, agent_verkey): (HashMap<String, String>, String, String) = future::ok(())
            .and_then(move |_| {
                wallet::create_wallet(ANONCREDS_WALLET_CONFIG, WALLET_CREDENTIALS)
                    .then(|res| match res {
                        Err(IndyError { error_code: ErrorCode::WalletAlreadyExistsError, .. }) => Ok(()),
                        r => r
                    })
            })
            .and_then(|_| {
                wallet::open_wallet(ANONCREDS_WALLET_CONFIG, WALLET_CREDENTIALS)
            })
            .and_then(|wallet_handle| {
                did::create_and_store_my_did(wallet_handle, "{}")
                    .map(move |(agent_did, agent_verkey)| (wallet_handle, agent_did, agent_verkey))
            })
            .and_then(|(wallet_handle, agent_did, agent_verkey)| {
                let config_options = vec![ConfigOption { name: "notificationWebhookUrl".into(), value: "http://example.org".into() }];
                Agent::insert_configs(wallet_handle, agent_did.clone(), config_options)
                    .map(move |_| (wallet_handle, agent_did, agent_verkey))
                    .map_err(|err| IndyError { message: "".into(), indy_backtrace: None, error_code: ErrorCode::AnoncredsProofRejected })
            })
            .and_then(|(wallet_handle, agent_did, agent_verkey)| {
                let config_options = vec![
                    ConfigOption{name: "logoUrl".into(), value: "http://logo.url".into()},
                    ConfigOption{name: "name".into(), value: "Foobar".into()}
                ];
                Agent::insert_configs(wallet_handle, agent_did.clone(), config_options)
                    .map(move |_| (wallet_handle, agent_did, agent_verkey))
                    .map_err(|err| IndyError { message: "".into(), indy_backtrace: None, error_code: ErrorCode::AnoncredsProofRejected })
            })
            .and_then(|(wallet_handle, agent_did, agent_verkey)| {
                Agent::remove_configs(wallet_handle, agent_did.clone(), vec![String::from("name")])
                    .map(move |loaded_configs| (wallet_handle, agent_did, agent_verkey))
                    .map_err(|err| IndyError { message: "".into(), indy_backtrace: None, error_code: ErrorCode::AnoncredsProofRejected })
            })
            .and_then(|(wallet_handle, agent_did, agent_verkey)| {
                Agent::load_configs(wallet_handle, agent_did.clone())
                    .map(move |loaded_configs| (loaded_configs, wallet_handle, agent_did, agent_verkey))
                    .map_err(|err| IndyError { message: "".into(), indy_backtrace: None, error_code: ErrorCode::AnoncredsProofRejected })
            })
            .and_then(|(loaded_configs, wallet_handle, agent_did, agent_verkey)| {
                wallet::close_wallet(wallet_handle.clone())
                    .map(|_| (loaded_configs, agent_did, agent_verkey))
            })
            .wait().expect("FAILED");
        assert_eq!(loaded_configs.get("notificationWebhookUrl").expect("Not found"), "http://example.org");
        assert_eq!(loaded_configs.get("logoUrl").expect("Not found"), "http://logo.url");
    }
}