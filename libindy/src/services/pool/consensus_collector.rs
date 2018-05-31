use services::pool::pool::PoolEvent;

pub struct ConsensusCollector {
    networker: Networker
}

impl ConsensusCollector {
    pub fn new() -> Self {
        ConsensusCollector {
            networker: Networker::new()
        }
    }

    pub fn get_event(&self, pe: Option<PoolEvent>) -> Option<PoolEvent> {
        match pe {
            Some(pe) => {
                match pe {
                    PoolEvent::SendRequest => {
                        self.networker.send_request();
                        None
                    }
                    PoolEvent::NodeReply => {
                        // TODO: check consensus
                        Some(PoolEvent::ConsensusReached)
                    }
                    pe => Some(pe)
                }
            }
            _ => None
        }
    }
}