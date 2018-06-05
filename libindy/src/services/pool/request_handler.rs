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

struct ConsensusState {}

impl RequestState for ConsensusState {}

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

impl<'sm, T: Networker> From<RequestSM<StartState<T>>> for RequestSM<SingleState<T>> {
    fn from(sm: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            state: SingleState {
                networker: sm.state.networker,
            }
        }
    }
}

impl<'sm, T: Networker> From<RequestSM<StartState<T>>> for RequestSM<ConsensusState> {
    fn from(_: RequestSM<StartState<T>>) -> Self {
        RequestSM {
            state: ConsensusState {}
        }
    }
}

impl<'sm, T: Networker> From<RequestSM<SingleState<T>>> for RequestSM<FinishState> {
    fn from(_: RequestSM<SingleState<T>>) -> Self {
        RequestSM {
            state: FinishState {}
        }
    }
}

impl From<RequestSM<ConsensusState>> for RequestSM<FinishState> {
    fn from(_: RequestSM<ConsensusState>) -> Self {
        RequestSM {
            state: FinishState {}
        }
    }
}

enum RequestSMWrapper<T: Networker> {
    Start(RequestSM<StartState<T>>),
    Single(RequestSM<SingleState<T>>),
    Consensus(RequestSM<ConsensusState>),
    Finish(RequestSM<FinishState>)
}

impl<T: Networker> RequestSMWrapper<T> {
    fn handle_event(self, re: RequestEvent) -> (Self, Option<PoolEvent>) {
        match self {
            RequestSMWrapper::Start(request) => {
                match re {
                    RequestEvent::LedgerStatus => {
                        //TODO: send request to networker
                        (RequestSMWrapper::Consensus(request.into()), None)
                    }
                    _ => (RequestSMWrapper::Start(request), None)
                }
            }
            RequestSMWrapper::Consensus(request) => {
                match re {
                    RequestEvent::NodeReply => {
                        //TODO: check if consensus reached or failed
                        (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::ConsensusReached))
                    },
                    _ => (RequestSMWrapper::Consensus(request), None)
                }
            }
            RequestSMWrapper::Single(request) => {
                match re {
                    RequestEvent::NodeReply => (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::NodeReply)),
                    _ => (RequestSMWrapper::Single(request), None)
                }
            },
            RequestSMWrapper::Finish(request) => (RequestSMWrapper::Finish(request), None)
        }
    }
}

pub trait RequestHandler<T: Networker> {
    fn new(networker: Rc<RefCell<T>>) -> Self;
    fn process_event(&mut self, ore: Option<RequestEvent>) -> Option<PoolEvent>;
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
}

//TODO: mocked one