use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::SignedResponse;
use v3::messages::connection::problem_report::ProblemReport;
use v3::messages::connection::ping::Ping;
use v3::messages::ack::Ack;
use v3::messages::a2a::A2AMessage;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DidExchangeMessages {
    Connect(),
    InvitationReceived(Invitation),
    ExchangeRequestReceived(Request),
    ExchangeResponseReceived(SignedResponse),
    AckReceived(Ack),
    ProblemReportReceived(ProblemReport),
    PingReceived(Ping),
    Unknown
}

impl From<A2AMessage> for DidExchangeMessages {
    fn from(msg: A2AMessage) -> Self {
        match msg {
            A2AMessage::ConnectionInvitation(invite) => {
                DidExchangeMessages::InvitationReceived(invite)
            }
            A2AMessage::ConnectionRequest(request) => {
                DidExchangeMessages::ExchangeRequestReceived(request)
            }
            A2AMessage::ConnectionResponse(request) => {
                DidExchangeMessages::ExchangeResponseReceived(request)
            }
            A2AMessage::Ping(ping) => {
                DidExchangeMessages::PingReceived(ping)
            }
            A2AMessage::Ack(ack) => {
                DidExchangeMessages::AckReceived(ack)
            }
            A2AMessage::ConnectionProblemReport(report) => {
                DidExchangeMessages::ProblemReportReceived(report)
            }
            _ => {
                DidExchangeMessages::Unknown
            }
        }
    }
}