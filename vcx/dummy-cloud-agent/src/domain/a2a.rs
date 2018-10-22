use failure::*;
use futures::*;
use indy::crypto;
use rmp_serde;
use serde::{de, Deserialize, Deserializer, ser, Serialize, Serializer};
use serde_json::{self, Value};
use utils::futures::*;

// TODO: For simplification we avoid complex versioning logic
// TODO: There should be additional enum level for versions
#[derive(Debug)]
pub enum A2AMessage {
    Forward(Forward),
    Connect(Connect),
    Connected(Connected),
    CreateKey(CreateKey),
    KeyCreated(KeyCreated),
    SignUp(SignUp),
    SignedUp(SignedUp),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Forward {
    #[serde(rename = "@fwd")]
    pub fwd: String,
    #[serde(rename = "@msg")]
    pub msg: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connect {
    #[serde(rename = "fromDID")]
    pub from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    pub from_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connected {
    #[serde(rename = "withPairwiseDID")]
    pub with_pairwise_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub with_pairwise_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateKey {
    #[serde(rename = "fromDID")]
    pub from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    pub from_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KeyCreated {
    #[serde(rename = "withPairwiseDID")]
    pub with_pairwise_did: String,
    #[serde(rename = "withPairwiseDIDVerKey")]
    pub with_pairwise_did_verkey: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUp {}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedUp {}

impl<'de> Deserialize<'de> for A2AMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match (value["@type"]["name"].as_str(), value["@type"]["ver"].as_str()) {
            (Some("CONNECT"), Some("1.0")) => {
                Connect::deserialize(value)
                    .map(|msg| A2AMessage::Connect(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONNECTED"), Some("1.0")) => {
                Connected::deserialize(value)
                    .map(|msg| A2AMessage::Connected(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CREATE_KEY"), Some("1.0")) => {
                CreateKey::deserialize(value)
                    .map(|msg| A2AMessage::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            (Some("FWD"), Some("1.0")) => {
                Forward::deserialize(value)
                    .map(|msg| A2AMessage::Forward(msg))
                    .map_err(de::Error::custom)
            }
            (Some("KEY_CREATED"), Some("1.0")) => {
                KeyCreated::deserialize(value)
                    .map(|msg| A2AMessage::KeyCreated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("SIGNUP"), Some("1.0")) => {
                SignUp::deserialize(value)
                    .map(|msg| A2AMessage::SignUp(msg))
                    .map_err(de::Error::custom)
            }
            (Some("SIGNED_UP"), Some("1.0")) => {
                SignedUp::deserialize(value)
                    .map(|msg| A2AMessage::SignedUp(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

impl Serialize for A2AMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            A2AMessage::Connect(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONNECT", "ver": "1.0"}));
                value
            }
            A2AMessage::Connected(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONNECTED", "ver": "1.0"}));
                value
            }
            A2AMessage::Forward(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "FWD", "ver": "1.0"}));
                value
            }
            A2AMessage::CreateKey(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CREATE_KEY", "ver": "1.0"}));
                value
            }
            A2AMessage::KeyCreated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "KEY_CREATED", "ver": "1.0"}));
                value
            }
            A2AMessage::SignUp(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "SIGNUP", "ver": "1.0"}));
                value
            }
            A2AMessage::SignedUp(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "SIGNED_UP", "ver": "1.0"}));
                value
            }
        };

        value.serialize(serializer)
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct A2AMsgsBundle {
    pub bundled: Vec<Vec<u8>>,
}

impl A2AMessage {
    pub fn bundle_plain(msgs: &[A2AMessage]) -> Result<Vec<u8>, Error> {
        msgs
            .iter()
            .map(|msg| rmp_serde::to_vec_named(msg))
            .collect::<Result<Vec<_>, _>>()
            .map(|msgs| A2AMsgsBundle { bundled: msgs })
            .and_then(|bundle| rmp_serde::to_vec_named(&bundle))
            .map_err(|err| err.context("Can't bundle messages").into())
    }

    pub fn bundle_authcrypted(wallet_handle: i32,
                              sender_vk: &str,
                              recipient_vk: &str,
                              msgs: &[A2AMessage]) -> BoxedFuture<Vec<u8>, Error> {
        let bundle = ftry!(Self::bundle_plain(msgs));

        crypto::auth_crypt(wallet_handle, &sender_vk, &recipient_vk, &bundle)
            .from_err()
            .into_box()
    }

    pub fn unbundle(bundle: &[u8]) -> Result<Vec<A2AMessage>, Error> {
        rmp_serde::from_slice::<A2AMsgsBundle>(bundle)
            .and_then(|bundle| {
                bundle.bundled
                    .iter()
                    .map(|msg| rmp_serde::from_slice::<A2AMessage>(msg))
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|err| err.context("Can't unbundle messages").into())
    }

    pub fn unbundle_anoncrypted(wallet_handle: i32,
                                recipient_vk: &str,
                                bundle: &[u8]) -> BoxedFuture<Vec<A2AMessage>, Error> {
        crypto::anon_decrypt(wallet_handle, recipient_vk, bundle)
            .from_err()
            .and_then(|bundle| {
                Self::unbundle(&bundle)
            })
            .into_box()
    }

    pub fn unbundle_authcrypted(wallet_handle: i32,
                                recipient_vk: &str,
                                bundle: &[u8]) -> BoxedFuture<(String, Vec<A2AMessage>), Error> {
        crypto::auth_decrypt(wallet_handle, recipient_vk, bundle)
            .from_err()
            .and_then(|(sender_vk, bundle)| {
                Self::unbundle(&bundle).map(|msgs| (sender_vk, msgs))
            })
            .into_box()
    }
}

