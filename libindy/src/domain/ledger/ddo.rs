use super::constants::GET_DDO;
use domain::crypto::did::DidValue;

#[derive(Serialize, PartialEq, Debug)]
pub struct GetDdoOperation {
    #[serde(rename = "type")]
    pub _type: String,
    pub dest: DidValue
}

impl GetDdoOperation {
    pub fn new(dest: DidValue) -> GetDdoOperation {
        GetDdoOperation {
            _type: GET_DDO.to_string(),
            dest
        }
    }
}