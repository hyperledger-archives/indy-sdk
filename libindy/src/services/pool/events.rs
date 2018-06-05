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

impl Into<Option<NetworkerEvent>> for RequestEvent {
    fn into(self) -> Option<NetworkerEvent> {
        match self {
            RequestEvent::LedgerStatus => Some(NetworkerEvent::SendAllRequest),
            _ => None
        }
    }
}