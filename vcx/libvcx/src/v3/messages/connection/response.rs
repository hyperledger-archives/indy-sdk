use error::prelude::*;
use utils::libindy::crypto;
use base64;
use rust_base58::ToBase58;
use time;

use messages::thread::Thread;
use v3::messages::A2AMessage;
use v3::messages::connection::did_doc::*;
use v3::messages::{MessageType, MessageId, A2AMessageKinds};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    pub connection: ConnectionData
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConnectionData {
    pub did: String,
    pub did_doc: DidDoc,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignedResponse {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    #[serde(rename = "@id")]
    pub id: MessageId,
    #[serde(rename = "~thread")]
    pub thread: Thread,
    #[serde(rename = "connection~sig")]
    pub connection_sig: ConnectionSignature
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConnectionSignature {
    #[serde(rename = "@type")]
    pub msg_type: MessageType,
    pub signature: String,
    pub sig_data: String,
    pub signers: String,
}

impl Response {
    pub fn create() -> Response {
        Response {
            msg_type: MessageType::build(A2AMessageKinds::Response),
            id: MessageId::new(),
            thread: Thread::new(),
            connection: ConnectionData {
                did: String::new(),
                did_doc: DidDoc {
                    context: String::from(CONTEXT),
                    id: String::new(),
                    public_key: vec![],
                    authentication: vec![],
                    service: vec![Service {
                        // TODO: FIXME Several services????
                        id: String::from("did:example:123456789abcdefghi;did-communication"),
                        type_: String::from("did-communication"),
                        priority: 0,
                        service_endpoint: String::new(),
                        recipient_keys: Vec::new(),
                        routing_keys: Vec::new(),
                    }],
                }
            },
        }
    }

    pub fn set_did(mut self, did: String) -> Response {
        self.connection.did = did.clone();
        self.connection.did_doc.id = did;
        self
    }

    pub fn set_service_endpoint(mut self, service_endpoint: String) -> Response {
        self.connection.did_doc.set_service_endpoint(service_endpoint);
        self
    }

    pub fn set_recipient_keys(mut self, recipient_keys: Vec<String>) -> Response {
        self.connection.did_doc.set_recipient_keys(recipient_keys);
        self
    }

    pub fn set_routing_keys(mut self, routing_keys: Vec<String>) -> Response {
        self.connection.did_doc.set_routing_keys(routing_keys);
        self
    }

    pub fn set_thread(mut self, thread: Thread) -> Response {
        self.thread = thread;
        self
    }

    pub fn encode(&self, key: &str) -> VcxResult<SignedResponse> {
        let connection_data = json!(self.connection).to_string();
        let now = time::get_time().sec as u64;

        let sig_data = format!("{}{}", now, connection_data);

        let sig_data = base64::encode_config(&sig_data.as_bytes(), base64::URL_SAFE);

        let signature = crypto::sign(key, sig_data.as_bytes())?;

        let signature = base64::encode_config(&signature, base64::URL_SAFE);

        let signers = base64::encode_config(&key, base64::URL_SAFE);

        let connection_sig = ConnectionSignature {
            msg_type: MessageType::build(A2AMessageKinds::Ed25519Signature),
            signature,
            sig_data,
            signers,
        };

        let signed_response = SignedResponse {
            msg_type: self.msg_type.clone(),
            id: self.id.clone(),
            thread: self.thread.clone(),
            connection_sig,
        };

        Ok(signed_response)
    }
}

impl SignedResponse {
    pub fn decode(self, invite_key: &str) -> VcxResult<Response> {
        let signers = base64::decode_config(&self.connection_sig.signers.as_bytes(), base64::URL_SAFE)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot decode ConnectionResponse: {:?}", err)))?;

        let signature = base64::decode_config(&self.connection_sig.signature.as_bytes(), base64::URL_SAFE)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot decode ConnectionResponse: {:?}", err)))?;

        let key = signers.to_base58();

        if !crypto::verify(&key, &self.connection_sig.sig_data.as_bytes(), &signature)? {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, "ConnectionResponse signature is invalid"));
        }

        if !crypto::verify(&invite_key, &self.connection_sig.sig_data.as_bytes(), &signature)? {
            return Err(VcxError::from_msg(VcxErrorKind::InvalidJson, "ConnectionResponse signature is invalid for original Invite recipient key"));
        }

        let sig_data = base64::decode_config(&self.connection_sig.sig_data.as_bytes(), base64::URL_SAFE)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, format!("Cannot decode ConnectionResponse: {:?}", err)))?;

        let sig_data = &sig_data[8..];
        let connection: ConnectionData = ::serde_json::from_slice(&sig_data)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidJson, err.to_string()))?;

        Ok(Response {
            msg_type: self.msg_type,
            id: self.id,
            thread: self.thread,
            connection,
        })
    }

    pub fn to_a2a_message(&self) -> A2AMessage {
        A2AMessage::ConnectionResponse(self.clone()) // TODO: THINK how to avoid clone
    }
}