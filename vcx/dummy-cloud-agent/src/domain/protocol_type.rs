use std::sync::Mutex;

lazy_static! {
    static ref PROTOCOL_TYPE: Mutex<ProtocolTypes> = Mutex::new(ProtocolTypes::default());
}

pub struct ProtocolType {}

impl ProtocolType {
    pub fn set(protocol_type_config: Option<ProtocolTypes>) {
        let protocol_type = protocol_type_config.map(ProtocolTypes::from).unwrap_or(ProtocolTypes::default());
        let mut p_t = PROTOCOL_TYPE.lock().unwrap();
        *p_t = protocol_type;
    }

    pub fn get() -> ProtocolTypes {
        PROTOCOL_TYPE.lock().unwrap().clone()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum ProtocolTypes {
    #[serde(rename = "1.0")]
    V1,
    #[serde(rename = "2.0")]
    V2,
}

impl Default for ProtocolTypes {
    fn default() -> Self {
        ProtocolTypes::V1
    }
}

impl From<std::string::String> for ProtocolTypes {
    fn from(protocol_type: String) -> ProtocolTypes {
        match protocol_type.as_str() {
            "1.0" => ProtocolTypes::V1,
            "2.0" => ProtocolTypes::V2,
            type_ @ _ => {
                error!("Unsupported protocol type: {:?}. Use default one", type_);
                ProtocolTypes::default()
            }
        }
    }
}