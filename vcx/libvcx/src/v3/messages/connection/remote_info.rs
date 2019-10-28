use v3::messages::connection::invite::Invitation;
use v3::messages::connection::request::Request;
use v3::messages::connection::response::Response;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RemoteConnectionInfo {
    pub label: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "recipientKeys")]
    pub recipient_keys: Vec<String>,
    #[serde(default)]
    #[serde(rename = "routingKeys")]
    pub routing_keys: Vec<String>,
}

impl RemoteConnectionInfo {
    pub fn set_label(&mut self, label: String){
        self.label = label
    }
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
        let (recipient_keys, routing_keys) = request.connection.did_doc.resolve_keys();
        let service_endpoint = request.connection.did_doc.get_endpoint();
        RemoteConnectionInfo {
            label: request.label,
            recipient_keys,
            routing_keys,
            service_endpoint,
        }
    }
}

impl From<Response> for RemoteConnectionInfo {
    fn from(response: Response) -> RemoteConnectionInfo {
        let (recipient_keys, routing_keys) = response.connection.did_doc.resolve_keys();
        let service_endpoint = response.connection.did_doc.get_endpoint();
        RemoteConnectionInfo {
            label: String::new(),
            recipient_keys,
            routing_keys,
            service_endpoint,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::connection::did_doc::tests::*;
    use v3::messages::connection::invite::tests::*;
    use v3::messages::connection::request::tests::*;
    use v3::messages::connection::response::tests::*;

    pub fn _info() -> RemoteConnectionInfo {
        RemoteConnectionInfo {
            label: _label(),
            recipient_keys: _recipient_keys(),
            routing_keys: _routing_keys(),
            service_endpoint: _service_endpoint(),
        }
    }

    #[test]
    fn test_remote_connection_info_from_invitation() {
        assert_eq!(_info(), RemoteConnectionInfo::from(_invitation()));
    }

    #[test]
    fn test_remote_connection_info_from_request() {
        assert_eq!(_info(), RemoteConnectionInfo::from(_request()));
    }

    #[test]
    fn test_remote_connection_info_from_response() {
        let mut info = _info();
        info.label = String::new();
        assert_eq!(info, RemoteConnectionInfo::from(_response()));
    }
}