use actix::prelude::*;
use actors::forward_agent::ForwardAgent;
use dirs;
use domain::a2a::*;
use domain::config::*;
use env_logger;
use failure::{err_msg, Error, Fail};
use futures::*;
use indy::{self, did, IndyError, wallet};
use std::env;
use std::fs;
use std::path::PathBuf;
use tokio_core::reactor::Core;
use utils::futures::*;

pub const EDGE_AGENT_WALLET_ID: &'static str = "edge_agent_wallet_id";
pub const EDGE_AGENT_WALLET_CONFIG: &'static str = "{\"id\": \"edge_agent_wallet_id\"}";
pub const EDGE_AGENT_WALLET_PASSPHRASE: &'static str = "edge_agent_wallet_passphrase";
pub const EDGE_AGENT_WALLET_CREDENTIALS: &'static str = "{\"key\": \"edge_agent_wallet_passphrase\"}";
pub const EDGE_AGENT_DID: &'static str = "NcYxiDXkpYi6ov5FcYDi1e";
pub const EDGE_AGENT_DID_INFO: &'static str = "{\"did\": \"NcYxiDXkpYi6ov5FcYDi1e\", \"seed\": \"0000000000000000000000000000Edge\"}";
pub const EDGE_AGENT_DID_VERKEY: &'static str = "B4aUxMQdPFkwBtcNUgs4fAbJhLSbXjQmrXByzL6gfDEq";
pub const FORWARD_AGENT_ENDPOINT: &'static str = "http://localhost:8080/forward_agent";

pub const FORWARD_AGENT_WALLET_ID: &'static str = "forward_agent_wallet_id";
pub const FORWARD_AGENT_WALLET_CONFIG: &'static str = "{\"id\": \"forward_agent_wallet_id\"}";
pub const FORWARD_AGENT_WALLET_PASSPHRASE: &'static str = "forward_agent_wallet_passphrase";
pub const FORWARD_AGENT_WALLET_CREDENTIALS: &'static str = "{\"key\": \"forward_agent_wallet_passphrase\"}";
pub const FORWARD_AGENT_DID: &'static str = "VsKV7grR1BUE29mG2Fm2kX";
pub const FORWARD_AGENT_DID_SEED: &'static str = "0000000000000000000000000Forward";
pub const FORWARD_AGENT_DID_INFO: &'static str = "{\"did\": \"VsKV7grR1BUE29mG2Fm2kX\", \"seed\": \"0000000000000000000000000Forward\"}";
pub const FORWARD_AGENT_DID_VERKEY: &'static str = "Hezce2UWMZ3wUhVkh2LfKSs8nDzWwzs2Win7EzNN3YaR";

pub fn indy_home_path() -> PathBuf {
    // TODO: FIXME: Provide better handling for the unknown home path case!!!
    let mut path = dirs::home_dir().unwrap_or(PathBuf::from("/home/indy"));
    path.push(if cfg!(target_os = "ios") { "Documents/.indy_client" } else { ".indy_client" });
    path
}

pub fn wallet_home_path() -> PathBuf {
    let mut path = indy_home_path();
    path.push("wallet");
    path
}

pub fn wallet_path(wallet_name: &str) -> PathBuf {
    let mut path = wallet_home_path();
    path.push(wallet_name);
    path
}

pub fn tmp_path() -> PathBuf {
    let mut path = env::temp_dir();
    path.push("indy_client");
    path
}

pub fn tmp_file_path(file_name: &str) -> PathBuf {
    let mut path = tmp_path();
    path.push(file_name);
    path
}

pub fn cleanup_indy_home() {
    let path = indy_home_path();
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

pub fn cleanup_temp() {
    let path = tmp_path();
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

pub fn cleanup_storage() {
    cleanup_indy_home();
    cleanup_temp();
}

pub fn run_test<F, B>(f: F)
    where
        F: FnOnce(Addr<ForwardAgent>) -> B + 'static,
        B: IntoFuture<Item=(), Error=Error> + 'static {
    indy::logger::set_default_logger(None).ok();
    env_logger::try_init().ok();
    cleanup_storage();

    System::run(|| {
        Arbiter::spawn_fn(move || {
            future::ok(())
                .and_then(move |_| {
                    ForwardAgent::create_or_restore(forward_agent_config(), wallet_storage_config())
                })
                .and_then(f)
                .map(move |_| {
                    System::current().stop()
                })
                .map_err(|err| panic!("Test error: {}!", err))
        })
    });
}

pub fn forward_agent_config() -> ForwardAgentConfig {
    ForwardAgentConfig {
        wallet_id: FORWARD_AGENT_WALLET_ID.into(),
        wallet_passphrase: FORWARD_AGENT_WALLET_PASSPHRASE.into(),
        did: FORWARD_AGENT_DID.into(),
        did_seed: Some(FORWARD_AGENT_DID_SEED.into()),
        endpoint: FORWARD_AGENT_ENDPOINT.into(),
    }
}

pub fn wallet_storage_config() -> WalletStorageConfig {
    WalletStorageConfig {
        xtype: None,
        config: None,
        credentials: None,
    }
}

pub fn edge_wallet_setup() -> BoxedFuture<i32, Error> {
    future::ok(())
        .and_then(|_| {
            wallet::create_wallet(EDGE_AGENT_WALLET_CONFIG, EDGE_AGENT_WALLET_CREDENTIALS)
                .then(|res| {
                    match res {
                        Err(IndyError::WalletAlreadyExistsError) => Ok(()),
                        r => r
                    }
                })
                .map_err(|err| err.context("Can't create edge agent wallet.").into())
        })
        .and_then(|_| {
            wallet::open_wallet(EDGE_AGENT_WALLET_CONFIG, EDGE_AGENT_WALLET_CREDENTIALS)
                .map_err(|err| err.context("Can't open edge agent wallet.").into())
        })
        .and_then(|wallet_handle| {
            did::create_and_store_my_did(wallet_handle, EDGE_AGENT_DID_INFO)
                .then(|res| match res {
                    Ok(_) => Ok(()),
                    Err(IndyError::DidAlreadyExistsError) => Ok(()), // Already exists
                    Err(err) => Err(err),
                })
                .map(move |_| wallet_handle)
                .map_err(|err| err.context("Can't create edge agent did.").into())
        })
        .into_box()
}

pub fn compose_connect(wallet_handle: i32) -> BoxedFuture<Vec<u8>, Error> {
    let msgs = [A2AMessage::Connect(
        Connect {
            from_did: EDGE_AGENT_DID.into(),
            from_did_verkey: EDGE_AGENT_DID_VERKEY.into(),
        })];

    future::ok(())
        .and_then(move |_| {
            A2AMessage::bundle_authcrypted(wallet_handle,
                                           EDGE_AGENT_DID_VERKEY,
                                           FORWARD_AGENT_DID_VERKEY,
                                           &msgs)
                .map(move |msg| (wallet_handle, msg))
        })
        .and_then(|(wallet_handle, msg)| {
            compose_forward(FORWARD_AGENT_DID, FORWARD_AGENT_DID_VERKEY, msg)
        })
        .into_box()
}

pub fn decompose_connected(wallet_handle: i32, msg: &[u8]) -> BoxedFuture<(String, String, String), Error> {
    A2AMessage::unbundle_authcrypted(wallet_handle, EDGE_AGENT_DID_VERKEY, &msg)
        .and_then(|(sender_verkey, mut msgs)| {
            if let Some(A2AMessage::Connected(msg)) = msgs.pop() {
                let Connected { with_pairwise_did: pairwise_did, with_pairwise_did_verkey: pairwise_verkey } = msg;
                Ok((sender_verkey, pairwise_did, pairwise_verkey))
            } else {
                Err(err_msg("Invalid message"))
            }
        })
        .into_box()
}

pub fn compose_signup(wallet_handle: i32, pairwise_did: &str,  pairwise_verkey: &str) -> BoxedFuture<Vec<u8>, Error> {
    let msgs = [A2AMessage::SignUp(SignUp {})];
    let pairwise_did = pairwise_did.to_owned();

    A2AMessage::bundle_authcrypted(wallet_handle,
                                   EDGE_AGENT_DID_VERKEY,
                                   pairwise_verkey,
                                   &msgs)
        .map(move |msg| (wallet_handle, msg))
        .and_then(move |(wallet_handle, msg)| {
            compose_forward(&pairwise_did, FORWARD_AGENT_DID_VERKEY, msg)
        })
        .into_box()
}

pub fn decompose_signedup(wallet_handle: i32, msg: &[u8]) -> BoxedFuture<String, Error> {
    A2AMessage::unbundle_authcrypted(wallet_handle, EDGE_AGENT_DID_VERKEY, &msg)
        .and_then(|(sender_verkey, mut msgs)| {
            if let Some(A2AMessage::SignedUp(_)) = msgs.pop() {
                Ok(sender_verkey)
            } else {
                Err(err_msg("Invalid message"))
            }
        })
        .into_box()
}

pub fn compose_create_agent(wallet_handle: i32, pairwise_did: &str, pairwise_verkey: &str) -> BoxedFuture<Vec<u8>, Error> {
    let msgs = [A2AMessage::CreateAgent(CreateAgent {})];
    let pairwise_did = pairwise_did.to_owned();

    A2AMessage::bundle_authcrypted(wallet_handle,
                                   EDGE_AGENT_DID_VERKEY,
                                   pairwise_verkey,
                                   &msgs)
        .map(move |msg| (wallet_handle, msg))
        .and_then(move |(wallet_handle, msg)| {
            compose_forward(&pairwise_did, FORWARD_AGENT_DID_VERKEY, msg)
        })
        .into_box()
}

pub fn decompose_agent_created(wallet_handle: i32, msg: &[u8]) -> BoxedFuture<(String, String, String), Error> {
    A2AMessage::unbundle_authcrypted(wallet_handle, EDGE_AGENT_DID_VERKEY, &msg)
        .and_then(|(sender_verkey, mut msgs)| {
            if let Some(A2AMessage::AgentCreated(agent_created)) = msgs.pop() {
                let AgentCreated { with_pairwise_did: pw_did, with_pairwise_did_verkey: pw_vk } = agent_created;
                Ok((sender_verkey, pw_did, pw_vk))
            } else {
                Err(err_msg("Invalid message"))
            }
        })
        .into_box()
}

pub fn compose_forward(recipient_did: &str, recipient_vk: &str, msg: Vec<u8>) -> BoxedFuture<Vec<u8>, Error> {
    let msgs = [A2AMessage::Forward(
        Forward {
            fwd: recipient_did.into(),
            msg,
        })];

    A2AMessage::bundle_anoncrypted(recipient_vk, &msgs)
}