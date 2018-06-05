#[derive(Copy, Clone)]
pub enum ConsensusCollectorEvent {
    NodeReply,
    StartConsensus
}

pub enum NetworkerEvent {
    SendOneRequest,
    SendAllRequest
}

#[derive(Copy, Clone)]
pub enum PoolEvent {
    CheckCache,
    NodeReply,
    Close,
    Refresh,
    ConsensusReached,
    ConsensusFailed,
    PoolOutdated,
    Synced,
    NodesBlacklisted,
    SendRequest,
    Timeout
}

#[derive(Copy, Clone)]
pub enum RequestEvent {
    LedgerStatus,
    NodeReply,
    ConsensusReached,
    ConsensusFailed,
}

impl Into<Option<RequestEvent>> for PoolEvent {
    fn into(self) -> Option<RequestEvent> {
        match self {
            PoolEvent::NodeReply => Some(RequestEvent::NodeReply),
            PoolEvent::SendRequest => None, //TODO: parse event type and send corresponding one
            _ => None
        }
    }
}

impl Into<Option<ConsensusCollectorEvent>> for RequestEvent {
    fn into(self) -> Option<ConsensusCollectorEvent> {
        match self {
            RequestEvent::LedgerStatus => Some(ConsensusCollectorEvent::StartConsensus),
            RequestEvent::NodeReply => Some(ConsensusCollectorEvent::NodeReply),
            _ => None
        }
    }
}

impl Into<Option<NetworkerEvent>> for ConsensusCollectorEvent {
    fn into(self) -> Option<NetworkerEvent> {
        match self {
            ConsensusCollectorEvent::StartConsensus => Some(NetworkerEvent::SendAllRequest),
            _ => None
        }
    }
}