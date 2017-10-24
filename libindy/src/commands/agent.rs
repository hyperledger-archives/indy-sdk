extern crate rust_base58;
extern crate serde_json;
extern crate zmq_pw as zmq;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use base64;
use commands::ledger::LedgerCommand;
use commands::utils::check_wallet_and_pool_handles_consistency;
use errors::indy::IndyError;
use errors::common::CommonError;
use errors::wallet::WalletError;
use services::ledger::LedgerService;
use services::ledger::types::{Reply, GetNymResultData, GetNymReplyResult};
use services::pool::PoolService;
use services::signus::{SignusService, DEFAULT_CRYPTO_TYPE};
use services::signus::types::{Did, Key, Endpoint as SEndpoint};
use services::wallet::WalletService;
use utils::crypto::box_::CryptoBox;
use utils::json::JsonDecodable;
use utils::sequence::SequenceUtils;
use utils::crypto::verkey_builder::build_full_verkey;

pub type AgentConnectCB = Box<Fn(Result<i32, IndyError>) + Send>;
pub type AgentMessageCB = Box<Fn(Result<(i32, String), IndyError>) + Send>;
pub type AgentPrepMsgCB = Box<Fn(Result<Vec<u8>, IndyError>) + Send>;
pub type AgentParseMsgCB = Box<Fn(Result<(Option<String>, Vec<u8>), IndyError>) + Send>;

pub enum AgentCommand {
    PrepMsg(
        i32, // wallet handle
        String, // sender_vk
        String, // recipient_vk
        Vec<u8>, // msg
        AgentPrepMsgCB, // cb
    ),
    PrepAnonymousMsg(
        String, // recipient_vk
        Vec<u8>, // msg
        AgentPrepMsgCB, // cb
    ),
    ParseMsg(
        i32, // wallet handle
        String, // recipient_vk
        Vec<u8>, // msg
        AgentParseMsgCB, // cb
    ),
}

pub struct AgentCommandExecutor {
    ledger_service: Rc<LedgerService>,
    pool_service: Rc<PoolService>,
    signus_service: Rc<SignusService>,
    wallet_service: Rc<WalletService>,

    out_connections: RefCell<HashMap<i32, AgentMessageCB>>,
    listeners: RefCell<HashMap<i32, Listener>>,

    listen_callbacks: RefCell<HashMap<i32, (
        Box<Fn(Result<i32, IndyError>) + Send>, // listen cb
        Listener
    )>>,
    add_rm_identity_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
    connect_callbacks: RefCell<HashMap<i32, (AgentConnectCB, AgentMessageCB)>>,
    send_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
    close_callbacks: RefCell<HashMap<i32, Box<Fn(Result<(), IndyError>)>>>,
}

struct Listener {
    on_connect: Box<Fn(Result<(i32, i32, String, String), IndyError>) + Send>,
    on_msg: AgentMessageCB,
    conn_handles: HashSet<i32>,
}

impl AgentCommandExecutor {
    pub fn new(ledger_service: Rc<LedgerService>, pool_service: Rc<PoolService>, signus_service: Rc<SignusService>, wallet_service: Rc<WalletService>) -> AgentCommandExecutor {
        AgentCommandExecutor {
            ledger_service,
            pool_service,
            signus_service,
            wallet_service,
            out_connections: RefCell::new(HashMap::new()),
            listeners: RefCell::new(HashMap::new()),
            listen_callbacks: RefCell::new(HashMap::new()),
            add_rm_identity_callbacks: RefCell::new(HashMap::new()),
            connect_callbacks: RefCell::new(HashMap::new()),
            send_callbacks: RefCell::new(HashMap::new()),
            close_callbacks: RefCell::new(HashMap::new()),
        }
    }

    pub fn execute(&self, agent_cmd: AgentCommand) {
        match agent_cmd {
            AgentCommand::PrepMsg(wallet_handle, sender_vk, recipient_vk, msg, cb) => {
                info!(target: "agent_command_executor", "PrepMsg command received");
                self.prep_msg(wallet_handle, sender_vk, recipient_vk, msg, cb);
            }
            AgentCommand::PrepAnonymousMsg(recipient_vk, msg, cb) => {
                info!(target: "agent_command_executor", "PrepAnonymousMsg command received");
                self.prep_anonymous_msg(recipient_vk, msg, cb);
            }
            AgentCommand::ParseMsg(wallet_handle, recipient_vk, msg, cb) => {
                info!(target: "agent_command_executor", "ParseMsg command received");
                self.parse_msg(wallet_handle, recipient_vk, msg, cb);
            }
        }
    }

    fn prep_msg(&self, wallet_handle: i32, sender_vk: String, recipient_vk: String, msg: Vec<u8>,
                cb: AgentPrepMsgCB) {
        let sender_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", sender_vk));
        if sender_key_json.is_err() {
            return cb(Err(IndyError::WalletError(WalletError::NotFound(format!("Key not found")))));
        }
        let sender_key_json = sender_key_json.unwrap();

        let sender_key = Key::from_json(&sender_key_json);
        if sender_key.is_err() {
            return cb(Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Key json")))));
        }
        let sender_key: Key = sender_key.unwrap();

        cb(self.signus_service.encrypt(&sender_key, &recipient_vk, msg.as_slice())
            .and_then(|(msg, nonce)| {
                let msg: serde_json::Value = json!({
                    "auth": true,
                    "nonce": base64::encode(nonce.as_slice()),
                    "msg": base64::encode(msg.as_slice()),
                    "sender": sender_vk,
                });
                let msg = serde_json::to_string(&msg).unwrap();
                self.signus_service.encrypt_sealed(&recipient_vk, msg.as_bytes())
            })
            .map_err(IndyError::SignusError))
    }

    fn prep_anonymous_msg(&self, recipient_vk: String, msg: Vec<u8>,
                          cb: AgentPrepMsgCB) {
        let msg: serde_json::Value = json!({
            "auth": false,
            "msg": base64::encode(msg.as_slice()),
        });
        let msg = serde_json::to_string(&msg).unwrap();
        cb(self.signus_service
            .encrypt_sealed(&recipient_vk, msg.as_bytes())
            .map_err(IndyError::SignusError))
    }

    fn parse_msg(&self, wallet_handle: i32, recipient_vk: String, msg: Vec<u8>,
                 cb: AgentParseMsgCB) {
        cb(self._parse_msg(wallet_handle, &recipient_vk, msg.as_slice()))
    }

    fn _parse_msg(&self, wallet_handle: i32, recipient_vk: &str, msg: &[u8]) -> Result<(Option<String>, Vec<u8>), IndyError> {
        let recipient_key_json = self.wallet_service.get(wallet_handle, &format!("key::{}", recipient_vk));
        if recipient_key_json.is_err() {
            return Err(IndyError::WalletError(WalletError::NotFound(format!("Key not found"))));
        }
        let recipient_key_json = recipient_key_json.unwrap();

        let recipient_key = Key::from_json(&recipient_key_json);
        if recipient_key.is_err() {
            return Err(IndyError::CommonError(CommonError::InvalidState(format!("Invalid Key json"))));
        }
        let recipient_key: Key = recipient_key.unwrap();


        let decrypted_msg = self.signus_service
            .decrypt_sealed(&recipient_key, msg)
            .map_err(map_err_trace!())?;

        #[derive(Deserialize)]
        struct DecryptedMsg {
            msg: String,
            authorized: bool,
            sender: Option<String>,
            nonce: Option<String>,
        }
        let parsed_msg: DecryptedMsg = serde_json::from_slice(decrypted_msg.as_slice())
            .map_err(|err| CommonError::InvalidStructure(format!("Can't determine internal msg format: {:?}", err)))
            .map_err(map_err_trace!())?;

        let internal_msg: Vec<u8> = base64::decode(&parsed_msg.msg)
            .map_err(|err| CommonError::InvalidStructure(format!("Can't decode internal msg filed from base64 {}", err)))
            .map_err(map_err_trace!())?;

        if !parsed_msg.authorized && parsed_msg.sender.is_none() && parsed_msg.nonce.is_none() {
            Ok((None, internal_msg))
        } else if let (&Some(ref sender_vk), &Some(ref nonce)) = (&parsed_msg.sender, &parsed_msg.nonce) {
            let nonce: Vec<u8> = base64::decode(nonce)
                .map_err(|err| CommonError::InvalidStructure(format!("Can't decode nonce from base64 {}", err)))
                .map_err(map_err_trace!())?;
            let decrypted_intenal_msg = self.signus_service
                .decrypt(&recipient_key, &sender_vk,
                                 internal_msg.as_slice(), nonce.as_slice())
                .map_err(map_err_trace!())?;
            Ok((parsed_msg.sender.clone(), decrypted_intenal_msg))
        } else {
            Err(IndyError::CommonError(CommonError::InvalidStructure(
                format!("Invalid internal msg format: authorized {}, sender is some {}, nonce is some {}",
                        parsed_msg.authorized, parsed_msg.sender.is_some(), parsed_msg.nonce.is_some()))))
        }
    }
}
