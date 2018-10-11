use serde::{Deserialize, Deserializer};
use serde::de;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Bundle {
    bundled: Vec<u8>,
}

#[derive(Debug)]
pub enum AgentMsg {
    Forward(ForwardMsg),
    Connect(ConnectMsg),
}

#[derive(Debug)]
pub enum ForwardMsg {
    V1(ForwardV1Msg)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardV1Msg {
    #[serde(rename = "@fwd")]
    pub fwd: String,
    #[serde(rename = "@msg")]
    pub msg: Vec<u8>,
}

#[derive(Debug)]
pub enum ConnectMsg {
    V1(ConnectV1Msg)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectV1Msg {
    #[serde(rename = "fromDID")]
    pub from_did: String,
    #[serde(rename = "fromDIDVerKey")]
    pub from_did_verkey: String,
}

impl<'de> Deserialize<'de> for AgentMsg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value = Value::deserialize(deserializer).map_err(de::Error::custom)?;

        match (value["@type"]["name"].as_str(), value["@type"]["ver"].as_str()) {
            (Some("FWD"), Some("1.0")) => ForwardV1Msg::deserialize(value)
                .map(|msg| AgentMsg::Forward(ForwardMsg::V1(msg)))
                .map_err(de::Error::custom),
            (Some("CONNECT"), Some("1.0")) => ConnectV1Msg::deserialize(value)
                .map(|msg| AgentMsg::Connect(ConnectMsg::V1(msg)))
                .map_err(de::Error::custom),
            _ => Err(de::Error::custom("Unexpected @type field structure."))
        }
    }
}

