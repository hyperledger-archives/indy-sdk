use services::pool::networker::Networker;
use services::pool::consensus_collector::ConsensusCollector;
use services::pool::events::RequestEvent;
use services::pool::events::PoolEvent;
use services::pool::networker::ZMQNetworker;
use std::marker::PhantomData;

trait RequestState {
    fn is_terminal(&self) -> bool;
}

struct StartState<'st, T: Networker + 'st> {
    networker: &'st T
}

impl<'st, T: Networker> RequestState for StartState<'st, T> {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct ConsensusState<'st, T: Networker + 'st, CC: ConsensusCollector<'st, T>> {
    _pd: &'st PhantomData<T>,
    consensus_collector: CC
}

impl<'st, T: Networker, CC: ConsensusCollector<'st, T>> RequestState for ConsensusState<'st, T, CC> {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct SingleState<'st, T: Networker + 'st> {
    networker: &'st T
}

impl<'st, T: Networker> RequestState for SingleState<'st, T> {
    fn is_terminal(&self) -> bool {
        false
    }
}

struct FinishState {
}

impl RequestState for FinishState {
    fn is_terminal(&self) -> bool {
        true
    }
}

struct RequestSM<T: RequestState> {
    state: T
}

impl<'sm, T: Networker> RequestSM<StartState<'sm, T>> {
    pub fn new(networker: &'sm T) -> Self {
        RequestSM {
            state: StartState {
                networker
            }
        }
    }
}

impl<'sm, T: Networker> From<RequestSM<StartState<'sm, T>>> for RequestSM<SingleState<'sm, T>> {
    fn from(sm: RequestSM<StartState<'sm, T>>) -> Self {
        RequestSM {
            state: SingleState {
                networker: sm.state.networker,
            }
        }
    }
}

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<(RequestSM<StartState<'sm, T>>, CC)> for RequestSM<ConsensusState<'sm, T, CC>> {
    fn from((_, cc): (RequestSM<StartState<'sm, T>>, CC)) -> Self {
        RequestSM {
            state: ConsensusState {
                _pd: &PhantomData,
                consensus_collector: cc,
            }
        }
    }
}

impl<'sm, T: Networker> From<RequestSM<SingleState<'sm, T>>> for RequestSM<FinishState> {
    fn from(_: RequestSM<SingleState<'sm, T>>) -> Self {
        RequestSM {
            state: FinishState {}
        }
    }
}

impl<'sm, T: Networker, CC: ConsensusCollector<'sm, T>> From<RequestSM<ConsensusState<'sm, T, CC>>> for RequestSM<FinishState> {
    fn from(_: RequestSM<ConsensusState<'sm, T, CC>>) -> Self {
        RequestSM {
            state: FinishState {}
        }
    }
}

enum RequestSMWrapper<'wr, T: Networker + 'wr, CC: ConsensusCollector<'wr, T>> {
    Start(RequestSM<StartState<'wr, T>>),
    Single(RequestSM<SingleState<'wr, T>>),
    Consensus(RequestSM<ConsensusState<'wr, T, CC>>),
    Finish(RequestSM<FinishState>)
}

impl<'wr, T: Networker, CC: ConsensusCollector<'wr, T>> RequestSMWrapper<'wr, T, CC> {
    fn handle_event(self, re: RequestEvent) -> (Self, Option<PoolEvent>) {
        match self {
            RequestSMWrapper::Start(request) => {
                match re {
                    RequestEvent::LedgerStatus => {
                        let mut cc = CC::new(request.state.networker);
                        cc.process_event(re.into());
                        (RequestSMWrapper::Consensus((request, cc).into()), None)
                    }
                    _ => (RequestSMWrapper::Start(request), None)
                }
            }
            RequestSMWrapper::Consensus(mut request) => {
                let re = request.state.consensus_collector.process_event(re.into()).unwrap_or(re);
                match re {
                    RequestEvent::ConsensusReached => (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::ConsensusReached)),
                    RequestEvent::ConsensusFailed => (RequestSMWrapper::Finish(request.into()), Some(PoolEvent::ConsensusFailed)),
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

pub trait RequestHandler<'trequest, T: Networker> {
    fn new(networker: &'trequest T) -> Self;
    fn process_event(&mut self, ore: Option<RequestEvent>) -> Option<PoolEvent>;
}

pub struct RequestHandlerImpl<'request, T: Networker + 'request, CC: ConsensusCollector<'request, T>> {
    request_wrapper: Option<RequestSMWrapper<'request, T, CC>>
}

impl<'request, T: Networker, CC: ConsensusCollector<'request, T>> RequestHandler<'request, T> for RequestHandlerImpl<'request, T, CC> {
    fn new(networker: &'request T) -> Self {
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