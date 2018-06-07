use services::pool::networker::Networker;
use services::pool::events::RequestEvent;
use services::pool::events::PoolEvent;
use std::cell::RefCell;
use std::rc::Rc;

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
    networker: Rc<RefCell<T>>
}

impl<T: Networker> RequestState for ConsensusState<T> {}

struct SingleState<T: Networker> {
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
    networker: Rc<RefCell<T>>
}

impl<T: Networker> RequestState for FullState<T> {}

struct RequestSM<T: RequestState> {
    state: T
}

impl<T: Networker> RequestSM<StartState<T>> {
    pub fn new(networker: Rc<RefCell<T>>) -> Self {
        RequestSM {
            state: StartState {
                networker
            }
        }
    }
}

impl<T: Networker> From<RequestSM<StartState<T>>> for RequestSM<SingleState<T>> {
    fn from(sm: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            state: SingleState {
                networker: sm.state.networker,
            }
        }
    }
}

impl<T: Networker> From<RequestSM<StartState<T>>> for RequestSM<ConsensusState<T>> {
    fn from(val: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            state: ConsensusState {
                networker: val.state.networker.clone()
            }
        }
    }
}

impl<T: Networker> From<RequestSM<StartState<T>>> for RequestSM<FullState<T>> {
    fn from(val: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            state: FullState {
                networker: val.state.networker.clone()
            }
        }
    }
}

impl<T: Networker> From<RequestSM<SingleState<T>>> for RequestSM<FinishState> {
    fn from(_: RequestSM<SingleState<T>>) -> Self {
        //TODO: close connections in networker
        RequestSM {
            state: FinishState {}
        }
    }
}

impl<T: Networker> From<RequestSM<ConsensusState<T>>> for RequestSM<FinishState> {
    fn from(_: RequestSM<ConsensusState<T>>) -> Self {
        //TODO: close connections in networker
        RequestSM {
            state: FinishState {}
        }
    }
}

impl<T: Networker> From<RequestSM<FullState<T>>> for RequestSM<FinishState> {
    fn from(_: RequestSM<FullState<T>>) -> Self {
        //TODO: close connections in networker
        RequestSM {
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
    FullRequest(RequestSM<FullState<T>>),
    Finish(RequestSM<FinishState>)
}

impl<T: Networker> RequestSMWrapper<T> {
    fn handle_event(self, re: RequestEvent) -> (Self, Option<PoolEvent>) {
        match self {
            RequestSMWrapper::Start(request) => {
                match re {
                    RequestEvent::LedgerStatus(ls) => {
                        //TODO: send request to networker
                        (RequestSMWrapper::CatchupConsensus(request.into()), None)
                    },
                    RequestEvent::CatchupReq(cr) => {
                        //TODO: send request to networker
                        (RequestSMWrapper::CatchupSingle(request.into()), None)
                    }
                    _ => (RequestSMWrapper::Start(request), None)
                }
            }
            RequestSMWrapper::Consensus(request) => {
                match re {
//                    RequestEvent::NodeReply => {
                        //TODO: check if consensus reached or failed
//                        (RequestSMWrapper::Finish(request.into()), None)
                        //if failed
//                        (RequestSMWrapper::Finish(request.into()), None)
                        //if still waiting
//                        (RequestSMWrapper::Consensus(request), None)
//                    },
                    _ => (RequestSMWrapper::Consensus(request), None)
                }
            }
            RequestSMWrapper::Single(request) => {
                match re {
//                    RequestEvent::NodeReply => {
//                        (RequestSMWrapper::Finish(request.into()), None)
                        //if failed
//                        (RequestSMWrapper::Finish(request.into()), None) //Some(PoolEvent::NodesBlacklisted)?
                        //if still waiting
//                        (RequestSMWrapper::Single(request), None)
//                    },
                    _ => (RequestSMWrapper::Single(request), None)
                }
            },
            RequestSMWrapper::CatchupConsensus(request) => {
                match re {
                    RequestEvent::LedgerStatus(ls) => {
                        //if consensus reached and catchup is needed
                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::ConsensusReached))
                        //if consensus reached and we are up to date
//                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::Synced))
                        //if failed
//                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::ConsensusFailed))
                        //if still waiting
//                        (RequestSMWrapper::CatchupConsensus(request), None)
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
            RequestSMWrapper::FullRequest(request) => {
                match re {
//                    RequestEvent::NodeReply => {
                        //if all nodes answered
//                        (RequestSMWrapper::Finish(request.into()), None)
                        //if we are still waiting
//                        (RequestSMWrapper::FullRequest(request), None)
//                    }
                    _ => (RequestSMWrapper::FullRequest(request), None)
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
            &RequestSMWrapper::FullRequest(ref request) => request.state.is_terminal(),
        }
    }
}

pub trait RequestHandler<T: Networker> {
    fn new(networker: Rc<RefCell<T>>) -> Self;
    fn process_event(&mut self, ore: Option<RequestEvent>) -> Option<PoolEvent>;
    fn is_terminal(&self) -> bool;
}

pub struct RequestHandlerImpl<T: Networker> {
    request_wrapper: Option<RequestSMWrapper<T>>
}

impl<T: Networker> RequestHandler<T> for RequestHandlerImpl<T> {
    fn new(networker: Rc<RefCell<T>>) -> Self {
        RequestHandlerImpl {
            request_wrapper: Some(RequestSMWrapper::Start(RequestSM::new(networker))),
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

//TODO: mocked one