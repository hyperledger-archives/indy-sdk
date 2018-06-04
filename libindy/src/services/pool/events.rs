pub enum ConsensusCollectorEvent {
    SendRequest,
    NodeReply,
    StartConsensus
}

pub enum NetworkerEvent {
    SendOneRequest,
    SendAllRequest
}

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

impl Into<Option<ConsensusCollectorEvent>> for PoolEvent {
    fn into(self) -> Option<ConsensusCollectorEvent> {
        match self {
            PoolEvent::NodeReply => Some(ConsensusCollectorEvent::NodeReply),
            PoolEvent::SendRequest => Some(ConsensusCollectorEvent::SendRequest),
            _ => None
        }
    }
}

impl Into<Option<NetworkerEvent>> for ConsensusCollectorEvent {
    fn into(self) -> Option<NetworkerEvent> {
        match self {
            ConsensusCollectorEvent::SendRequest => {
                //TODO: check if we actually need consensus!! acknowledge it with Slava
                //TODO: if we don't, we send one request
                //Some(NetworkerEvent::SendOneRequest)
                //TODO: if we do, we send all requests
                Some(NetworkerEvent::SendAllRequest)
            },
            _ => None
        }
    }
}