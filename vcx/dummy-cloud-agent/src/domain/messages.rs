use serde::{de, Deserialize, Deserializer, ser, Serialize, Serializer};
use serde_json::{self, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bundle {
    pub bundled: Vec<Vec<u8>>,
}

// TODO: For simplification we avoid complex versioning logic
// TODO: There should be additional enum level for versions
#[derive(Debug)]
pub enum Message {
    Forward(Forward),
    Connect(Connect),
    Connected(Connected),
    CreateKey(CreateKey),
    KeyCreated(KeyCreated)
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

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match (value["@type"]["name"].as_str(), value["@type"]["ver"].as_str()) {
            (Some("CONNECT"), Some("1.0")) => {
                Connect::deserialize(value)
                    .map(|msg| Message::Connect(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CONNECTED"), Some("1.0")) => {
                Connected::deserialize(value)
                    .map(|msg| Message::Connected(msg))
                    .map_err(de::Error::custom)
            }
            (Some("CREATE_KEY"), Some("1.0")) => {
                CreateKey::deserialize(value)
                    .map(|msg| Message::CreateKey(msg))
                    .map_err(de::Error::custom)
            }
            (Some("KEY_CREATED"), Some("1.0")) => {
                KeyCreated::deserialize(value)
                    .map(|msg| Message::KeyCreated(msg))
                    .map_err(de::Error::custom)
            }
            (Some("FWD"), Some("1.0")) => {
                Forward::deserialize(value)
                    .map(|msg| Message::Forward(msg))
                    .map_err(de::Error::custom)
            }
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let value = match self {
            Message::Connect(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONNECT", "ver": "1.0"}));
                value
            }
            Message::Connected(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CONNECTED", "ver": "1.0"}));
                value
            }
            Message::Forward(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "FWD", "ver": "1.0"}));
                value
            }
            Message::CreateKey(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "CREATE_KEY", "ver": "1.0"}));
                value
            }
            Message::KeyCreated(msg) => {
                let mut value = serde_json::to_value(msg).map_err(ser::Error::custom)?;
                value.as_object_mut().unwrap().insert("@type".into(), json!({"name": "KEY_CREATED", "ver": "1.0"}));
                value
            }
        };

        value.serialize(serializer)
    }
}

