use domain::ledger::constants;
use errors::common::CommonError;
use errors::pool::PoolError;
use self::indy_crypto::utils::json::JsonEncodable;
use serde_json;
use serde_json::Value as SJsonValue;
use services::ledger::merkletree::merkletree::MerkleTree;
use services::pool::types::*;
use std::error::Error;

extern crate indy_crypto;


pub const REQUESTS_FOR_STATE_PROOFS: [&'static str; 7] = [
    constants::GET_NYM,
    constants::GET_SCHEMA,
    constants::GET_CRED_DEF,
    constants::GET_ATTR,
    constants::GET_REVOC_REG,
    constants::GET_REVOC_REG_DEF,
    constants::GET_REVOC_REG_DELTA,
];

const REQUEST_FOR_FULL: [&'static str; 2] = [
    constants::POOL_RESTART,
    constants::GET_VALIDATOR_INFO,
];

#[derive(Debug, Clone)]
pub enum NetworkerEvent {
    SendOneRequest(
        String, //msg
        String, //req_id
    ),
    SendAllRequest(
        String, //msg
        String, //req_id
    ),
    Resend(String),
    NodesStateUpdated(Vec<RemoteNode>),
    ExtendTimeout(
        String, //req_id
        String, //node_alias
    ),
    CleanTimeout(
        String, //req_id
        Option<String>, //node_alias
    ),
    Timeout,
}

#[derive(Clone, Debug)]
pub enum PoolEvent {
    CheckCache(i32),
    NodeReply(
        String, // reply
        String, // node alias
    ),
    Close(
        i32, //cmd_id
    ),
    Refresh(
        i32, //cmd_id
    ),
    CatchupTargetFound(
        Vec<u8>, //target_mt_root
        usize, //target_mt_size
        MerkleTree,
    ),
    CatchupTargetNotFound(PoolError),
    #[allow(dead_code)] //FIXME
    PoolOutdated,
    Synced(
        MerkleTree
    ),
    #[allow(dead_code)] //FIXME
    NodesBlacklisted,
    SendRequest(
        i32, // cmd_id
        String, // request
    ),
    Timeout(
        String, //req_id
        String, //node alias
    ),
}

#[derive(Clone, Debug)]
pub enum RequestEvent {
    LedgerStatus(
        LedgerStatus,
        Option<String>, //node alias
        Option<MerkleTree>,
    ),
    CatchupReq(
        MerkleTree,
        usize, // target mt size
        Vec<u8>, // target mt root
    ),
    Timeout(
        String, //req_id
        String, //node_alias
    ),
    CatchupRep(
        CatchupRep,
        String, // node_alias
    ),
    CustomSingleRequest(
        String, // message
        String, // req_id
    ),
    CustomConsensusRequest(
        String, // message
        String, // req_id
    ),
    CustomFullRequest(
        String, // message
        String, // req_id
    ),
    ConsistencyProof(
        ConsistencyProof,
        String, //node alias
    ),
    Reply(
        Reply,
        String, //raw_msg
        String, //node alias
        String, //req_id
    ),
    ReqACK(
        Response,
        String, //raw_msg
        String, //node alias
        String, //req_id
    ),
    ReqNACK(
        Response,
        String, //raw_msg
        String, //node alias
        String, //req_id
    ),
    Reject(
        Response,
        String, //raw_msg
        String, //node alias
        String, //req_id
    ),
    PoolLedgerTxns,
    Ping,
    Pong,
    Terminate,
}

impl RequestEvent {
    pub fn get_req_id(&self) -> String {
        match self {
            &RequestEvent::CustomSingleRequest(_, ref id) => id.to_string(),
            &RequestEvent::CustomConsensusRequest(_, ref id) => id.to_string(),
            &RequestEvent::CustomFullRequest(_, ref id) => id.to_string(),
            &RequestEvent::Reply(_, _, _, ref id) => id.to_string(),
            &RequestEvent::ReqACK(_, _, _, ref id) => id.to_string(),
            &RequestEvent::ReqNACK(_, _, _, ref id) => id.to_string(),
            &RequestEvent::Reject(_, _, _, ref id) => id.to_string(),
            _ => "".to_string()
        }
    }
}

impl Into<Option<RequestEvent>> for PoolEvent {
    fn into(self) -> Option<RequestEvent> {
        match self {
            PoolEvent::NodeReply(msg, node_alias) => {
                _parse_msg(&msg).map(|parsed|
                    match parsed {
                        //TODO change mapping for CatchupReq. May be return None
                        //TODO: REMOVE UNWRAP!!!!!
                        Message::CatchupReq(_) => RequestEvent::CatchupReq(MerkleTree::from_vec(Vec::new()).unwrap(), 0, vec![]),
                        Message::CatchupRep(rep) => RequestEvent::CatchupRep(rep, node_alias),
                        Message::LedgerStatus(ls) => RequestEvent::LedgerStatus(ls, Some(node_alias), None),
                        Message::ConsistencyProof(cp) => RequestEvent::ConsistencyProof(cp, node_alias),
                        Message::Reply(rep) => {
                            let req_id = rep.req_id();
                            RequestEvent::Reply(rep, msg, node_alias, req_id.to_string())
                        }
                        Message::ReqACK(rep) => {
                            let req_id = rep.req_id();
                            RequestEvent::ReqACK(rep, msg, node_alias, req_id.to_string())
                        }
                        Message::ReqNACK(rep) => {
                            let req_id = rep.req_id();
                            RequestEvent::ReqNACK(rep, msg, node_alias, req_id.to_string())
                        }
                        Message::Reject(rep) => {
                            let req_id = rep.req_id();
                            RequestEvent::Reject(rep, msg, node_alias, req_id.to_string())
                        }
                        Message::PoolLedgerTxns(_) => RequestEvent::PoolLedgerTxns,
                        Message::Ping => RequestEvent::Ping,
                        Message::Pong => RequestEvent::Pong,
                    })
            }
            PoolEvent::SendRequest(_, msg) => {
                let req_id = _parse_req_id_and_op(&msg);
                match req_id {
                    Ok((ref req_id, ref op)) if REQUESTS_FOR_STATE_PROOFS.contains(&op.as_str()) => Some(RequestEvent::CustomSingleRequest(msg, req_id.clone())), //FIXME check plugged also
                    Ok((ref req_id, ref op)) if REQUEST_FOR_FULL.contains(&op.as_str()) => Some(RequestEvent::CustomFullRequest(msg, req_id.clone())),
                    Ok((ref req_id, _)) => Some(RequestEvent::CustomConsensusRequest(msg, req_id.clone())),
                    Err(_) => None,
                }
            }
            PoolEvent::Timeout(req_id, node_alias) => Some(RequestEvent::Timeout(req_id, node_alias)),
            _ => None
        }
    }
}


impl Into<Option<NetworkerEvent>> for RequestEvent {
    fn into(self) -> Option<NetworkerEvent> {
        match self {
            RequestEvent::LedgerStatus(ls, _, _) => {
                let req_id = ls.merkleRoot.clone();
                Some(NetworkerEvent::SendAllRequest(Message::LedgerStatus(ls).to_json().expect("FIXME"), req_id))
            }
            _ => None
        }
    }
}

fn _parse_msg(msg: &str) -> Option<Message> {
    Message::from_raw_str(msg).map_err(map_err_trace!()).ok()
}

fn _parse_req_id_and_op(msg: &str) -> Result<(String, String), CommonError> {
    let req_json = _get_req_json(msg)?;

    let req_id: u64 = req_json["reqId"]
        .as_u64()
        .ok_or(CommonError::InvalidStructure("No reqId in request".to_string()))?;

    let op = req_json["operation"]["type"]
        .as_str()
        .ok_or(CommonError::InvalidStructure("No operation type in request".to_string()))?;

    Ok((req_id.to_string(), op.to_string()))
}

fn _get_req_json(msg: &str) -> Result<SJsonValue, CommonError> {
    serde_json::from_str(msg)
        .map_err(|err|
            CommonError::InvalidStructure(
                format!("Invalid request json: {}", err.description())))
}