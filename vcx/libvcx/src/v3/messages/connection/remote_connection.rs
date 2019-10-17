use v3::messages::connection::request::Request;
use v3::messages::connection::did_doc::Service;

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteConnectionInfo {
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "recipientKeys")]
    pub recipient_keys: Vec<String>,
    #[serde(rename = "routingKeys")]
    pub routing_keys: Vec<String>,
}

impl From<Request> for RemoteConnectionInfo {
    fn from(mut request: Request) -> RemoteConnectionInfo {
        let service: Service = request.connection.did_doc.service.pop().unwrap();
        RemoteConnectionInfo {
            recipient_keys: service.recipient_keys,
            routing_keys: service.routing_keys,
            service_endpoint: service.service_endpoint,
        }
    }
}