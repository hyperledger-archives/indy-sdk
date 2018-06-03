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
    SendRequest
}

type ConsensusCollectorEventOption = Option<ConsensusCollectorEvent>;
type NetworkerEventOption = Option<NetworkerEvent>;

impl From<PoolEvent> for ConsensusCollectorEventOption {
    fn from(pe: PoolEvent) -> Self {
        match pe {
            PoolEvent::NodeReply => Some(ConsensusCollectorEvent::NodeReply),
            PoolEvent::SendRequest => Some(ConsensusCollectorEvent::SendRequest),
            _ => None
        }
    }
}

impl From<ConsensusCollectorEventOption> for NetworkerEventOption {
    fn from(cce: ConsensusCollectorEvent) -> Self {
        match cce {
            Some(ConsensusCollectorEvent::SendRequest) => {
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