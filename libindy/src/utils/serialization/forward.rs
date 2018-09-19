use serde_json;
use errors::route::RouteError;
use utils::crypto::base64::encode;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardJSON {
    #[serde(rename = "type")]
    pub type_: String,
    pub to: String,
    pub message: String
}

/// This creates a plaintext of the forward message data structure
pub fn format_forward_msg(ciphertext: &[u8], to_did: &str) -> Result<String, RouteError> {
    let encoded_ciphertext = encode(ciphertext);
    let fwd_json = ForwardJSON {
        type_: String::from("Forward"),
        to: to_did.to_string(),
        message: encoded_ciphertext
    };

    serde_json::to_string(&fwd_json)
        .map_err( | err | RouteError::EncodeError(format!("{}", err)))
}



#[cfg(test)]
mod tests {
    use base64::{encode_config, URL_SAFE};
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_format_forward_msg() {
        //base information
        let message = "unencrypted text converted to byte array".as_bytes();
        let to_did = String::from("did:sov:thisisbobpwdidwithalice#1");
        let message_encoded = encode_config(message, URL_SAFE);

        //create forward message
        let forward_msg = format_forward_msg(message, &to_did).unwrap();

        let expected_output = json!({
                "type" : "Forward",
                "to" : to_did,
                "message" : encode_config(message, URL_SAFE)
            });

        let function_output: Value = serde_json::from_str(&forward_msg).unwrap();
        assert_eq!(function_output, expected_output);
    }
}