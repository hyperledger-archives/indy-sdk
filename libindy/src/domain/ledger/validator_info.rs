use super::constants::GET_VALIDATOR_INFO;

#[derive(Serialize, PartialEq, Debug)]
pub struct GetValidatorInfoOperation {
    #[serde(rename = "type")]
    pub _type: String,
}

impl GetValidatorInfoOperation {
    pub fn new() -> GetValidatorInfoOperation {
        GetValidatorInfoOperation {
            _type: GET_VALIDATOR_INFO.to_string(),
        }
    }
}

