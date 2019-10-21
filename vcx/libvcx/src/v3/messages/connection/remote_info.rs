use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::Response;
use v3::messages::connection::did_doc::Service;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteConnectionInfo {
    pub label: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "recipientKeys")]
    pub recipient_keys: Vec<String>,
    #[serde(rename = "routingKeys")]
    pub routing_keys: Vec<String>,
}

impl From<Invitation> for RemoteConnectionInfo {
    fn from(invite: Invitation) -> RemoteConnectionInfo {
        RemoteConnectionInfo {
            label: invite.label,
            recipient_keys: invite.recipient_keys,
            routing_keys: invite.routing_keys,
            service_endpoint: invite.service_endpoint,
        }
    }
}

impl From<Request> for RemoteConnectionInfo {
    fn from(request: Request) -> RemoteConnectionInfo {
        let service: Service = request.connection.did_doc.service.get(0).cloned().unwrap();
        RemoteConnectionInfo {
            label: request.label,
            recipient_keys: service.recipient_keys,
            routing_keys: service.routing_keys,
            service_endpoint: service.service_endpoint,
        }
    }
}

impl From<Response> for RemoteConnectionInfo {
    fn from(response: Response) -> RemoteConnectionInfo {
        let service: Service = response.connection.did_doc.service.get(0).cloned().unwrap();
        RemoteConnectionInfo {
            label: String::new(),
            recipient_keys: service.recipient_keys,
            routing_keys: service.routing_keys,
            service_endpoint: service.service_endpoint,
        }
    }
}