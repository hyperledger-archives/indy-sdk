use services::pool::types::LedgerStatus;
use services::pool::types::CatchupReq;
use services::pool::types::CatchupRep;
use services::pool::types::Message;

pub enum NetworkerEvent {
    SendOneRequest,
    SendAllRequest
}
#[derive(Clone)]
pub enum PoolEvent {
    CheckCache,
    NodeReply(
        String, // reply
    ),
    Close,
    Refresh,
    ConsensusReached,
    ConsensusFailed,
    PoolOutdated,
    Synced,
    NodesBlacklisted,
    SendRequest(
        String, // request
    ),
    Timeout
}

#[derive(Clone)]
pub enum RequestEvent {
    LedgerStatus(
        LedgerStatus
    ),
    CatchupReq(CatchupReq),
    CatchupRep(CatchupRep),
    None
}

impl RequestEvent {
    pub fn get_req_id(&self) -> String {
        unimplemented!()
    }
}

impl From<Message> for RequestEvent {
    fn from(msg: Message) -> Self {
        match msg {
            Message::CatchupReq(req) => RequestEvent::CatchupReq(req),
            Message::CatchupRep(rep) => RequestEvent::CatchupRep(rep),
            Message::LedgerStatus(ls) => RequestEvent::LedgerStatus(ls),
            _ => RequestEvent::None
        }
    }
}

impl Into<Option<RequestEvent>> for PoolEvent {
    fn into(self) -> Option<RequestEvent> {
        match self {
            PoolEvent::NodeReply(msg) => {
                _parse_msg(&msg).map(Message::into)
            },
            PoolEvent::SendRequest(msg) => {
                unimplemented!() //TODO: parse
            }
            _ => None
        }
    }
}


impl Into<Option<NetworkerEvent>> for RequestEvent {
    fn into(self) -> Option<NetworkerEvent> {
        match self {
            RequestEvent::LedgerStatus(_) => Some(NetworkerEvent::SendAllRequest),
            _ => None
        }
    }
}

fn _parse_msg(msg: &str) -> Option<Message> {
    match Message::from_raw_str(msg).map_err(map_err_trace!()) {
        Ok(msg) => Some(msg),
        Err(err) => None
    }
}