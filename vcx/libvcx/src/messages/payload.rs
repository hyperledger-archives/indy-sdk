use messages::message_type::*;
use messages::to_u8;
use settings::{ProtocolTypes, get_protocol_type};
use utils::error;
use utils::libindy::crypto;

use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum Payloads {
    PayloadV1(PayloadV1),
    PayloadV2(PayloadV2),
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PayloadV1 {
    #[serde(rename = "@type")]
    type_: PayloadTypeV1,
    #[serde(rename = "@msg")]
    pub msg: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PayloadV2 {
    #[serde(rename = "@type")]
    type_: PayloadTypeV2,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@msg")]
    pub msg: String,
    #[serde(rename = "~thread")]
    pub thread: Thread,
}

impl Payloads {
    // TODO: Refactor Error
    // this will become a CommonError, because multiple types (Connection/Issuer Credential) use this function
    // Possibly this function moves out of this file.
    // On second thought, this should stick as a ConnectionError.
    pub fn encrypt(my_vk: &str, their_vk: &str, data: &str, msg_type: PayloadKinds, thread: Option<Thread>) -> Result<Vec<u8>, u32> {
        match ProtocolTypes::from(get_protocol_type()) {
            ProtocolTypes::V1 => {
                let payload = PayloadV1 {
                    type_: PayloadTypes::build_v1(msg_type, "json"),
                    msg: data.to_string(),
                };

                let bytes = rmp_serde::to_vec_named(&payload)
                    .map_err(|err| {
                        error!("could not encode create_keys msg: {}", err);
                        error::INVALID_MSGPACK.code_num
                    })?;

                trace!("Sending payload: {:?}", bytes);
                crypto::prep_msg(&my_vk, &their_vk, &bytes)
            }
            ProtocolTypes::V2 => {
                let thread = thread.ok_or(error::INVALID_CONNECTION_HANDLE.code_num)?;

                let payload = PayloadV2 {
                    type_: PayloadTypes::build_v2(msg_type),
                    id: String::new(),
                    msg: data.to_string(),
                    thread,
                };

                let message = ::serde_json::to_string(&payload)
                    .map_err(|err| {
                        error!("could not serialize create_keys msg: {}", err);
                        error::INVALID_MSGPACK.code_num
                    })?;

                let receiver_keys = ::serde_json::to_string(&vec![&their_vk]).or(Err(error::SERIALIZATION_ERROR.code_num))?;

                trace!("Sending payload: {:?}", message.as_bytes());
                crypto::pack_message(Some(my_vk), &receiver_keys, message.as_bytes())
            }
        }
    }

    pub fn decrypt(my_vk: &str, payload: &Vec<i8>) -> Result<(String, Option<Thread>), u32> {
        match ProtocolTypes::from(get_protocol_type()) {
            ProtocolTypes::V1 => {
                let (_, data) = crypto::parse_msg(&my_vk, &to_u8(payload))?;

                let my_payload: PayloadV1 = rmp_serde::from_slice(&data[..])
                    .map_err(|err| {
                        error!("could not deserialize bundle with i8 or u8: {}", err);
                        error::INVALID_MSGPACK.code_num
                    })?;
                Ok((my_payload.msg, None))
            }
            ProtocolTypes::V2 => {
                Payloads::decrypt_payload_v2(my_vk, payload)
            }
        }
    }

    pub fn decrypt_payload_v2(my_vk: &str, payload: &Vec<i8>) -> Result<(String, Option<Thread>), u32> {
        let unpacked_msg = crypto::unpack_message(&to_u8(payload))?;

        let message: ::serde_json::Value = ::serde_json::from_slice(unpacked_msg.as_slice())
            .or(Err(error::INVALID_JSON.code_num))?;

        let message = message["message"].as_str().ok_or(error::INVALID_JSON.code_num)?.to_string();

        let mut my_payload: PayloadV2 = serde_json::from_str(&message)
            .map_err(|err| {
                error!("could not deserialize bundle with i8 or u8: {}", err);
                error::INVALID_MSGPACK.code_num
            })?;

        if my_payload.thread.thid.is_none() {
            my_payload.thread.thid = Some(my_payload.id);
        }

        Ok((my_payload.msg, Some(my_payload.thread)))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum PayloadTypes {
    PayloadTypeV1(PayloadTypeV1),
    PayloadTypeV2(PayloadTypeV2),
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct PayloadTypeV1 {
    name: String,
    ver: String,
    fmt: String,
}

type PayloadTypeV2 = MessageTypeV2;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum PayloadKinds {
    CredOffer,
    CredReq,
    Cred,
    Proof,
    ProofRequest,
    Other(String)
}

impl PayloadKinds {
    fn family(&self) -> MessageFamilies {
        match self {
            PayloadKinds::CredOffer => MessageFamilies::CredentialExchange,
            PayloadKinds::CredReq => MessageFamilies::CredentialExchange,
            PayloadKinds::Cred => MessageFamilies::CredentialExchange,
            PayloadKinds::Proof => MessageFamilies::CredentialExchange,
            PayloadKinds::ProofRequest => MessageFamilies::CredentialExchange,
            PayloadKinds::Other(family) => MessageFamilies::Unknown(family.to_string()),
        }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        match get_protocol_type() {
            ProtocolTypes::V1 => {
                match self {
                    PayloadKinds::CredOffer => "CRED_OFFER",
                    PayloadKinds::CredReq => "CRED_REQ",
                    PayloadKinds::Cred => "CRED",
                    PayloadKinds::ProofRequest => "PROOF_REQUEST",
                    PayloadKinds::Proof => "PROOF",
                    PayloadKinds::Other(kind) => kind,
                }
            }
            ProtocolTypes::V2 => {
                match self {
                    PayloadKinds::CredOffer => "credential-offer",
                    PayloadKinds::CredReq => "credential-request",
                    PayloadKinds::Cred => "credential",
                    PayloadKinds::ProofRequest => "presentation-request",
                    PayloadKinds::Proof => "presentation",
                    PayloadKinds::Other(kind) => kind,
                }
            }
        }
    }
}

impl PayloadTypes {
    pub fn build_v1(kind: PayloadKinds, fmt: &str) -> PayloadTypeV1 {
        PayloadTypeV1 {
            name: kind.name().to_string(),
            ver: MESSAGE_VERSION_V1.to_string(),
            fmt: fmt.to_string(),
        }
    }

    pub fn build_v2(kind: PayloadKinds) -> PayloadTypeV2 {
        PayloadTypeV2 {
            did: DID.to_string(),
            family: kind.family(),
            version: kind.family().version().to_string(),
            type_: kind.name().to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Thread {
    pub thid: Option<String>,
    pub pthid: Option<String>,
    pub sender_order: u32,
    pub received_orders: HashMap<String, u32>,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            thid: None,
            pthid: None,
            sender_order: 0,
            received_orders: HashMap::new(),
        }
    }

    pub fn increment_receiver(&mut self, did: &str) {
        self.received_orders.entry(did.to_string())
            .and_modify(|e| *e += 1)
            .or_insert(0);
    }
}