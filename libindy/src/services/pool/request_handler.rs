extern crate rust_base58;

use commands::CommandExecutor;
use commands::Command;
use commands::ledger::LedgerCommand;
use errors::common::CommonError;
use errors::pool::PoolError;
use services::pool::events::RequestEvent;
use services::pool::events::PoolEvent;
use services::pool::events::NetworkerEvent;
use services::pool::networker::Networker;
use services::pool::state_proof;
use services::pool::types::HashableValue;

use self::rust_base58::FromBase58;
use base64;
use serde_json;
use serde_json::Value as SJsonValue;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;
use super::indy_crypto::bls::Generator;
use super::indy_crypto::bls::VerKey;

trait RequestState {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct StartState<T: Networker> {
    networker: Rc<RefCell<T>>
}

impl<T: Networker> RequestState for StartState<T> {}

struct ConsensusState<T: Networker> {
    nack_cnt: HashSet<String>,
    replies: HashMap<HashableValue, HashSet<String>>,
    networker: Rc<RefCell<T>>
}

impl<T: Networker> RequestState for ConsensusState<T> {}

struct SingleState<T: Networker> {
    nack_cnt: HashSet<String>,
    replies: HashMap<HashableValue, HashSet<String>>,
    networker: Rc<RefCell<T>>
}

impl<T: Networker> RequestState for SingleState<T> {}

struct FinishState {}

impl RequestState for FinishState {
    fn is_terminal(&self) -> bool {
        true
    }
}

struct FullState<T: Networker> {
    nack_cnt: HashSet<String>,
    accum_reply: Option<HashableValue>,
    networker: Rc<RefCell<T>>
}

impl<T: Networker> RequestState for FullState<T> {}

struct RequestSM<T: RequestState> {
    f: usize,
    cmd_ids: Vec<i32>,
    nodes: HashMap<String, Option<VerKey>>,
    generator: Generator,
    state: T
}

impl<T: Networker> RequestSM<StartState<T>> {
    pub fn new(networker: Rc<RefCell<T>>, f: usize, cmd_ids: &Vec<i32>, nodes: HashMap<String, Option<VerKey>>, generator: Option<Generator>) -> Self {
        RequestSM {
            f,
            cmd_ids: cmd_ids.clone(),
            nodes,
            generator: generator.unwrap_or(Generator::from_bytes(&"3LHpUjiyFC2q2hD7MnwwNmVXiuaFbQx2XkAFJWzswCjgN1utjsCeLzHsKk1nJvFEaS4fcrUmVAkdhtPCYbrVyATZcmzwJReTcJqwqBCPTmTQ9uWPwz6rEncKb2pYYYFcdHa8N17HzVyTqKfgPi4X9pMetfT3A5xCHq54R2pDNYWVLDX".from_base58().unwrap()).unwrap()),
            state: StartState {
                networker
            },
        }
    }
}

impl<T: Networker> From<RequestSM<StartState<T>>> for RequestSM<SingleState<T>> {
    fn from(sm: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            f: sm.f,
            cmd_ids: sm.cmd_ids,
            nodes: sm.nodes,
            generator: sm.generator,
            state: SingleState {
                nack_cnt: HashSet::new(),
                replies: HashMap::new(),
                networker: sm.state.networker,
            }
        }
    }
}

impl<T: Networker> From<RequestSM<StartState<T>>> for RequestSM<ConsensusState<T>> {
    fn from(val: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            f: val.f,
            cmd_ids: val.cmd_ids,
            nodes: val.nodes,
            generator: val.generator,
            state: ConsensusState {
                nack_cnt: HashSet::new(),
                replies: HashMap::new(),
                networker: val.state.networker.clone(),
            }
        }
    }
}

impl<T: Networker> From<RequestSM<StartState<T>>> for RequestSM<FullState<T>> {
    fn from(val: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            f: val.f,
            cmd_ids: val.cmd_ids,
            nodes: val.nodes,
            generator: val.generator,
            state: FullState {
                nack_cnt: HashSet::new(),
                accum_reply: None,
                networker: val.state.networker.clone(),
            }
        }
    }
}

impl<T: Networker> From<RequestSM<SingleState<T>>> for RequestSM<FinishState> {
    fn from(val: RequestSM<SingleState<T>>) -> Self {
        //TODO: close connections in networker
        RequestSM {
            f: val.f,
            cmd_ids: val.cmd_ids,
            nodes: val.nodes,
            generator: val.generator,
            state: FinishState {}
        }
    }
}

impl<T: Networker> From<RequestSM<ConsensusState<T>>> for RequestSM<FinishState> {
    fn from(val: RequestSM<ConsensusState<T>>) -> Self {
        //TODO: close connections in networker
        RequestSM {
            f: val.f,
            cmd_ids: val.cmd_ids,
            nodes: val.nodes,
            generator: val.generator,
            state: FinishState {}
        }
    }
}

impl<T: Networker> From<RequestSM<FullState<T>>> for RequestSM<FinishState> {
    fn from(sm: RequestSM<FullState<T>>) -> Self {
        //TODO: close connections in networker
        RequestSM {
            f: sm.f,
            cmd_ids: sm.cmd_ids,
            nodes: sm.nodes,
            generator: sm.generator,
            state: FinishState {}
        }
    }
}

impl<T: Networker> From<RequestSM<StartState<T>>> for RequestSM<FinishState> {
    fn from(sm: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            f: sm.f,
            cmd_ids: sm.cmd_ids,
            nodes: sm.nodes,
            generator: sm.generator,
            state: FinishState {}
        }
    }
}

enum RequestSMWrapper<T: Networker> {
    Start(RequestSM<StartState<T>>),
    Single(RequestSM<SingleState<T>>),
    Consensus(RequestSM<ConsensusState<T>>),
    CatchupSingle(RequestSM<SingleState<T>>),
    CatchupConsensus(RequestSM<ConsensusState<T>>),
    Full(RequestSM<FullState<T>>),
    Finish(RequestSM<FinishState>)
}

impl<T: Networker> RequestSMWrapper<T> {
    fn handle_event(self, re: RequestEvent) -> (Self, Option<PoolEvent>) {
        match self {
            RequestSMWrapper::Start(request) => {
                match re {
                    RequestEvent::LedgerStatus(ls) => {
                        (RequestSMWrapper::CatchupConsensus(request.into()), None)
                    },
                    RequestEvent::CatchupReq(cr) => {
                        (RequestSMWrapper::CatchupSingle(request.into()), None)
                    },
                    RequestEvent::CustomSingleRequest(msg, req_id) => {
                        match req_id {
                            Ok(req_id) => {
                                request.state.networker.borrow_mut().send_request(Some(NetworkerEvent::SendOneRequest));
                                (RequestSMWrapper::Consensus(request.into()), None)
                            }
                            Err(e) => {
                                _send_replies(&request.cmd_ids, Err(PoolError::CommonError(e)));
                                (RequestSMWrapper::Finish(request.into()), None)
                            }
                        }
                    }
                    RequestEvent::CustomFullRequest(msg, req_id) => {
                        match req_id {
                            Ok(req_id) => {
                                request.state.networker.borrow_mut().send_request(Some(NetworkerEvent::SendAllRequest));
                                (RequestSMWrapper::Full(request.into()), None)
                            }
                            Err(e) => {
                                _send_replies(&request.cmd_ids, Err(PoolError::CommonError(e)));
                                (RequestSMWrapper::Finish(request.into()), None)
                            }
                        }
                    }
                    RequestEvent::CustomConsensusRequest(msg, req_id) => {
                        match req_id {
                            Ok(req_id) => {
                                request.state.networker.borrow_mut().send_request(Some(NetworkerEvent::SendAllRequest));
                                (RequestSMWrapper::Consensus(request.into()), None)
                            }
                            Err(e) => {
                                _send_replies(&request.cmd_ids, Err(PoolError::CommonError(e)));
                                (RequestSMWrapper::Finish(request.into()), None)
                            }
                        }
                    }
                    _ => (RequestSMWrapper::Start(request), None)
                }
            }
            RequestSMWrapper::Consensus(mut request) => {
                match re {
                    RequestEvent::Reply(rep, raw_msg, node_alias, req_id) => {
                        if let Ok((_, result_without_proof)) = _get_msg_result_without_state_proof(&raw_msg) {
                            let hashable = HashableValue {inner: result_without_proof};

                            let cnt = if let Some(set) = request.state.replies.get_mut(&hashable) {
                                set.insert(node_alias.clone());
                                set.len()
                            } else {
                                1usize
                            };

                            if cnt == 1usize {
                                request.state.replies.insert(hashable, HashSet::from_iter(vec![node_alias]));
                            }

                            if cnt > request.f {
                                _send_ok_replies(&request.cmd_ids, &raw_msg);
                                (RequestSMWrapper::Finish(request.into()), None)
                            } else {
                                let rep_no: usize = request.state.replies.values().map(|set| set.len()).sum();
                                let max_no = request.state.replies.values().map(|set| set.len()).max().unwrap_or(0);

                                if max_no + request.nodes.len() - rep_no - request.state.nack_cnt.len() > request.f {
                                    (RequestSMWrapper::Consensus(request), None)
                                } else {
                                    //TODO: maybe we should change the error, but it was made to escape changing of ErrorCode returned to client
                                    _send_replies(&request.cmd_ids, Err(PoolError::Timeout));
                                    (RequestSMWrapper::Finish(request.into()), None)
                                }
                            }
                        } else {
                            (RequestSMWrapper::Consensus(request), None)
                        }
                    },
                    RequestEvent::ReqACK(rep, raw_msg, node_alias, req_id) => {
                        //TODO: extend timeout
                        (RequestSMWrapper::Consensus(request), None)
                    }
                    RequestEvent::ReqNACK(rep, raw_msg, node_alias, req_id) | RequestEvent::Reject(rep, raw_msg, node_alias, req_id) => {
                        if _parse_nack(&mut request.state.nack_cnt, request.f, &raw_msg, &request.cmd_ids, &node_alias) {
                            (RequestSMWrapper::Finish(request.into()), None)
                        } else {
                            (RequestSMWrapper::Consensus(request), None)
                        }

                    }
                    _ => (RequestSMWrapper::Consensus(request), None)
                }
            }
            RequestSMWrapper::Single(mut request) => {
                match re {
                    RequestEvent::Reply(rep, raw_msg, node_alias, req_id) => {
                        if let Ok((result, result_without_proof)) = _get_msg_result_without_state_proof(&raw_msg) {
                            let hashable = HashableValue {inner: result_without_proof};

                            let cnt = if let Some(set) = request.state.replies.get_mut(&hashable) {
                                set.insert(node_alias.clone());
                                set.len()
                            } else {
                                1usize
                            };

                            if cnt == 1usize {
                                request.state.replies.insert(hashable, HashSet::from_iter(vec![node_alias]));
                            }

                            if cnt > request.f || _check_state_proof(&result, request.f, &request.generator, &request.nodes, &raw_msg) {
                                _send_ok_replies(&request.cmd_ids, &raw_msg);
                                (RequestSMWrapper::Finish(request.into()), None)
                            } else {
                                //TODO: resend to next node
                                (RequestSMWrapper::Single(request), None)
                            }
                        } else {
                            //TODO: resend to next node
                            (RequestSMWrapper::Single(request), None)
                        }
                    },
                    RequestEvent::ReqACK(rep, raw_msg, node_alias, req_id) => {
                        //TODO: extend timeout
                        (RequestSMWrapper::Single(request), None)
                    }
                    RequestEvent::ReqNACK(rep, raw_msg, node_alias, req_id) | RequestEvent::Reject(rep, raw_msg, node_alias, req_id) => {
                        if _parse_nack(&mut request.state.nack_cnt, request.f, &raw_msg, &request.cmd_ids, &node_alias) {
                            (RequestSMWrapper::Finish(request.into()), None)
                        } else {
                            //TODO: remap on RESEND
                            request.state.networker.borrow_mut().send_request(Some(NetworkerEvent::SendOneRequest));
                            (RequestSMWrapper::Single(request), None)
                        }
                    }
                    _ => (RequestSMWrapper::Single(request), None)
                }
            },
            RequestSMWrapper::CatchupConsensus(request) => {
                match re {
                    RequestEvent::LedgerStatus(ls) => {
                        //if consensus reached and catchup is needed
                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::CatchupTargetFound))
                        //if consensus reached and we are up to date
//                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::Synced))
                        //if failed
//                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::ConsensusFailed))
                        //if still waiting
//                        (RequestSMWrapper::CatchupConsensus(request), None)
                    }
                    RequestEvent::ConsistencyProof(cp) => {
                        (RequestSMWrapper::CatchupConsensus(request), None)
                    }
                    _ => (RequestSMWrapper::CatchupConsensus(request), None)
                }
            },
            RequestSMWrapper::CatchupSingle(request) => {
                match re {
                    RequestEvent::CatchupRep(cr) => {
                        //if round-robin successful
                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::Synced))
                        //if failed
//                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::NodesBlacklisted))
                        //if still waiting
//                        (RequestSMWrapper::CatchupSingle(request), None)
                    }
                    _ => (RequestSMWrapper::CatchupSingle(request), None)
                }
            },
            RequestSMWrapper::Full(mut request) => {
                match re {
                    RequestEvent::Reply(rep, raw_msg, node_alias, req_id) => {
                        let req_id = rep.req_id();
                        let first_resp = request.state.accum_reply.is_none();
                        if first_resp {
                            request.state.accum_reply = Some(HashableValue{
                                inner: json!({node_alias: raw_msg})
                            })
                        } else {
                            request.state.accum_reply.as_mut().unwrap()
                                .inner.as_object_mut().unwrap()
                                .insert(node_alias, SJsonValue::from(raw_msg));
                        }

                        let reply_cnt = request.state.accum_reply.as_ref().unwrap()
                            .inner.as_object().unwrap().len();

                        if reply_cnt == request.nodes.len() {
                            let reply = request.state.accum_reply.as_ref().unwrap().inner.to_string();
                            _send_ok_replies(&request.cmd_ids, &reply);
                            (RequestSMWrapper::Finish(request.into()), None)
                        } else {
                            (RequestSMWrapper::Full(request), None)
                        }

                    }
                    RequestEvent::ReqACK(rep, raw_msg, node_alias, req_id) => {
                        //TODO: extend timeout
                        (RequestSMWrapper::Full(request), None)
                    }
                    RequestEvent::ReqNACK(rep, raw_msg, node_alias, req_id) | RequestEvent::Reject(rep, raw_msg, node_alias, req_id) => {
                        if _parse_nack(&mut request.state.nack_cnt, request.f, &raw_msg, &request.cmd_ids, &node_alias) {
                            (RequestSMWrapper::Finish(request.into()), None)
                        } else {
                            (RequestSMWrapper::Full(request), None)
                        }
                    }
                    _ => (RequestSMWrapper::Full(request), None)
                }
            }
            RequestSMWrapper::Finish(request) => (RequestSMWrapper::Finish(request), None)
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            &RequestSMWrapper::Start(ref request) => request.state.is_terminal(),
            &RequestSMWrapper::Consensus(ref request) => request.state.is_terminal(),
            &RequestSMWrapper::Single(ref request) => request.state.is_terminal(),
            &RequestSMWrapper::Finish(ref request) => request.state.is_terminal(),
            &RequestSMWrapper::CatchupSingle(ref request) => request.state.is_terminal(),
            &RequestSMWrapper::CatchupConsensus(ref request) => request.state.is_terminal(),
            &RequestSMWrapper::Full(ref request) => request.state.is_terminal(),
        }
    }
}

pub trait RequestHandler<T: Networker> {
    fn new(networker: Rc<RefCell<T>>, f: usize, cmd_ids: &Vec<i32>, nodes: HashMap<String, Option<VerKey>>, generator: Option<Generator>) -> Self;
    fn process_event(&mut self, ore: Option<RequestEvent>) -> Option<PoolEvent>;
    fn is_terminal(&self) -> bool;
}

pub struct RequestHandlerImpl<T: Networker> {
    request_wrapper: Option<RequestSMWrapper<T>>
}

impl<T: Networker> RequestHandler<T> for RequestHandlerImpl<T> {
    fn new(networker: Rc<RefCell<T>>, f: usize, cmd_ids: &Vec<i32>, nodes: HashMap<String, Option<VerKey>>, generator: Option<Generator>) -> Self {
        RequestHandlerImpl {
            request_wrapper: Some(RequestSMWrapper::Start(RequestSM::new(networker, f, cmd_ids, nodes, generator))),
        }
    }

    fn process_event(&mut self, ore: Option<RequestEvent>) -> Option<PoolEvent> {
        match ore {
            Some(re) => {
                if let Some((rw, res)) = self.request_wrapper.take().map(|w| w.handle_event(re)) {
                    self.request_wrapper = Some(rw);
                    res
                } else {
                    self.request_wrapper = None;
                    None
                }
            },
            None => None
        }
    }

    fn is_terminal(&self) -> bool {
        self.request_wrapper.as_ref().map(|w| w.is_terminal()).unwrap_or(true)
    }
}

fn _parse_nack(cnt: &mut HashSet<String>, f: usize, raw_msg: &str, cmd_ids: &Vec<i32>, node_alias: &str) -> bool {
    if cnt.len() == f {
        _send_ok_replies(cmd_ids, raw_msg);
        true
    } else {
        cnt.insert(node_alias.to_string());
        false
    }
}

fn _send_ok_replies(cmd_ids: &Vec<i32>, msg: &str) {
    _send_replies(cmd_ids, Ok(msg.to_string()))
}

fn _send_replies(cmd_ids: &Vec<i32>, msg: Result<String, PoolError>) {
    cmd_ids.into_iter().for_each(|id| {
        CommandExecutor::instance().send(
            Command::Ledger(
                LedgerCommand::SubmitAck(id.clone(), msg.clone()))
        ).unwrap();
    });
}

fn _get_msg_result_without_state_proof(msg: &str) -> Result<(SJsonValue, SJsonValue), CommonError> {
    let msg_result: SJsonValue = match serde_json::from_str::<SJsonValue>(msg) {
        Ok(raw_msg) => raw_msg["result"].clone(),
        Err(err) => return Err(CommonError::InvalidStructure(format!("Invalid response structure: {:?}", err))).map_err(map_err_err!())
    };

    let mut msg_result_without_proof: SJsonValue = msg_result.clone();
    msg_result_without_proof.as_object_mut().map(|obj| obj.remove("state_proof"));
    if msg_result_without_proof["data"].is_object() {
        msg_result_without_proof["data"].as_object_mut().map(|obj| obj.remove("stateProofFrom"));
    }
    Ok((msg_result, msg_result_without_proof))
}

pub fn _check_state_proof(msg_result: &SJsonValue, f: usize, gen: &Generator, bls_keys: &HashMap<String, Option<VerKey>>, raw_msg: &str) -> bool {
    debug!("TransactionHandler::process_reply: Try to verify proof and signature");

    match state_proof::parse_generic_reply_for_proof_checking(&msg_result, raw_msg) {
        Some(parsed_sps) => {
            debug!("TransactionHandler::process_reply: Proof and signature are present");
            state_proof::verify_parsed_sp(parsed_sps, bls_keys, f, gen)
        },
        None => false
    }
}

//TODO: mocked one