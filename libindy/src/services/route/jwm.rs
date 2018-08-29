use serde::Deserialize;
use serde_json;
use serde_json::{Value, Error, from_str};
use base64::{encode_config, decode_config, URL_SAFE};
use utils::json::JsonDecodable;

use errors::common::CommonError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardJSON {
    #[serde(rename = "type")]
    pub type_: String,
    pub to: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWM {
    pub recipients: Vec<Recipient>,
    pub ciphertext: String,
    pub iv: String,
    pub tag: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipient {
    pub header : Header,
    pub encrypted_key : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub typ : String,
    pub alg : String,
    pub enc : Option<String>,
    pub kid : Option<String>,
    pub jwk : Option<String>
}

impl Header {
    pub fn new_authcrypt_header(recipient_vk: String, sender_vk: String) -> Header {
        Header {
            typ: String::from("x-b64nacl"),
            alg: String::from("x-auth"),
            enc: Some(String::from("xsalsa20poly1305")),
            kid: Some(recipient_vk),
            jwk: Some(sender_vk)
        }
    }

    pub fn new_anoncrypt_header(recipient_vk: String, sender_vk: String) -> Header {
        Header {
            typ: String::from("x-b64nacl"),
            alg: String::from("x-auth"),
            enc: Some(String::from("xsalsa20poly1305")),
            kid: Some(recipient_vk),
            jwk: Some(sender_vk)
        }
    }

    pub fn new_plain_header() -> Header {
        Header {
            typ: String::from("x-b64nacl"),
            alg: String::from("x-plain"),
            enc: None,
            kid: None,
            jwk: None

        }
    }
}

impl Recipient {
    pub fn new(header : Header, cek: String) -> Recipient {
        Recipient {
            header,
            encrypted_key: Some(cek)
        }
    }
}

/// This creates a plaintext of the forward message data structure
pub fn format_forward_msg(ciphertext: &[u8], to_did: &str) -> String {
    let encoded_ciphertext = encode_config(ciphertext, URL_SAFE);
    let fwd_json = ForwardJSON {
        type_: String::from("Forward"),
        to: to_did.to_string(),
        message: encoded_ciphertext
    };
    serde_json::to_string(&fwd_json).unwrap_or("Error serializing Forward Message".to_string())
}

pub fn json_serialize_jwm(recipient_vks : Vec<String>,
                        encrypted_keys : Vec<String>,
                        sender_vk : String,
                        auth : bool,
                        ciphertext : String,
                        iv : String,
                        tag : String ) -> String {

    let recipients = create_receipients(encrypted_keys.clone(),
                                        recipient_vks.clone(),
                                        sender_vk.clone(),
                                        auth);
    let jwm = JWM {
        recipients,
        ciphertext,
        iv,
        tag
    };

    serde_json::to_string(&jwm).unwrap_or("Error serializing JWM".to_string())
}

pub fn json_deserialize_jwm(jwm : String) -> JWM {
    let jwm_struct = JWM::from_json(&jwm).unwrap();
    return jwm_struct
}

pub fn create_receipients(encrypted_keys : Vec<String>,
                          recipient_vks : Vec<String>,
                          sender_vk : String,
                          auth : bool) -> Vec<Recipient> {

    let mut recipients_list : Vec<Recipient> = vec![];
    for (i, value) in recipient_vks.iter().enumerate() {
        if auth {
            let header = Header::new_authcrypt_header(value.to_string(),
                                                      sender_vk.clone());
            let recipient = Recipient::new(header, encrypted_keys[i].to_string());
            recipients_list.push(recipient);
        } else {
            let header = Header::new_anoncrypt_header(value.to_string(),
                                                      sender_vk.clone());
            let recipient = Recipient::new(header, encrypted_keys[i].to_string());
            recipients_list.push(recipient);
        }
    }

    recipients_list
}

#[cfg(test)]
mod tests {
    use super::{format_forward_msg, JWM, create_receipients, json_deserialize_jwm, json_serialize_jwm };
    use utils::crypto::base64;
    use serde_json;
    use serde_json::{Value, Error};
    use base64::{encode_config, URL_SAFE};

    #[test]
    fn test_format_forward_msg(){
        //base information
        let message = "unencrypted text converted to byte array".as_bytes();
        let to_did = String::from("did:sov:thisisbobpwdidwithalice#1");
        let message_encoded = encode_config(message, URL_SAFE);
        //create function
        let forward_msg = format_forward_msg(message, &to_did);

        let expected_output = json!({
                "type" : "Forward",
                "to" : to_did,
                "message" : encode_config(message, URL_SAFE)
            });

        let function_output : Value = serde_json::from_str(&forward_msg).unwrap();
        assert_eq!(function_output, expected_output);
    }

    #[test]
    fn test_json_serialize_jwm() {
        let encrypted_keys = vec!["made_up_cek_1".to_string(),
                                            "made_up_cek_2".to_string(),
                                            "made_up_cek_3".to_string()];

        let recipient_vks = vec!["C5q2MDmdG26nVw73yhVhdz".to_string(),
                                            "4q37dbi1XXhaqcfXirTrin".to_string(),
                                            "8fnXqkWJmdex234Pe9EhC".to_string()];

        let sender_vk = "KLPrG3eq3DNZwveVd7NS7i".to_string();
        let auth = true;
        let recipients = create_receipients(encrypted_keys.clone(),
                                              recipient_vks.clone(),
                                                sender_vk.clone(),
                                                          auth);

        let ciphertext = "this is fake ciphertext to test JWMs".to_string();
        let iv = "FAKE_IVTOTESTJWMSERIALIZE".to_string();
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE".to_string();

        let expected_output = json!({
        "recipients" : [
                {
                    "header" : {
                        "typ" : "x-b64nacl",
                        "alg" : "x-auth",
                        "enc" : "xsalsa20poly1305",
                        "kid" : &recipient_vks[0],
                        "jwk" : &sender_vk
                    },
                    "encrypted_key" : &encrypted_keys[0]
                },
                {
                    "header" : {
                        "typ" : "x-b64nacl",
                        "alg" : "x-auth",
                        "enc" : "xsalsa20poly1305",
                        "kid" : &recipient_vks[1],
                        "jwk" : &sender_vk
                    },
                    "encrypted_key" : &encrypted_keys[1]
                },
                {
                    "header" : {
                        "typ" : "x-b64nacl",
                        "alg" : "x-auth",
                        "enc" : "xsalsa20poly1305",
                        "kid" : &recipient_vks[2],
                        "jwk" : &sender_vk
                    },
                    "encrypted_key" : &encrypted_keys[2]
                }
            ],
        "ciphertext" : "this is fake ciphertext to test JWMs",
        "iv" : "FAKE_IVTOTESTJWMSERIALIZE",
        "tag" : "FAKE_TAGTOTESTJWMSERIALIZE"
        });

        let jwm = json_serialize_jwm(recipient_vks.clone(),
                                     encrypted_keys.clone(),
                                     sender_vk.clone(),
                                               auth,
                                      ciphertext.clone(),
                                            iv.clone(),
                                           tag.clone());
        let function_output : Value = serde_json::from_str(&jwm).unwrap();
        assert_eq!(function_output, expected_output);
    }

    fn test_json_deserialize_jwm() {
        let encrypted_keys = vec!["made_up_cek_1".to_string(),
                                            "made_up_cek_2".to_string(),
                                            "made_up_cek_3".to_string()];

        let recipient_vks = vec!["C5q2MDmdG26nVw73yhVhdz".to_string(),
                                            "4q37dbi1XXhaqcfXirTrin".to_string(),
                                            "8fnXqkWJmdex234Pe9EhC".to_string()];

        let sender_vk = "KLPrG3eq3DNZwveVd7NS7i".to_string();
        let auth = true;
        let recipients = create_receipients(encrypted_keys.clone(),
                                              recipient_vks.clone(),
                                                sender_vk.clone(),
                                                          auth);

        let ciphertext = "this is fake ciphertext to test JWMs".to_string();
        let iv = "FAKE_IVTOTESTJWMSERIALIZE".to_string();
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE".to_string();
        let expected_jwm : JWM = JWM {
            recipients : recipients,
            ciphertext : ciphertext.clone(),
            iv : iv.clone(),
            tag : tag.clone()
        };

        let jwm_string = json_serialize_jwm(recipient_vks.clone(),
                                     encrypted_keys.clone(),
                                     sender_vk.clone(),
                                               auth,
                                      ciphertext.clone(),
                                            iv.clone(),
                                           tag.clone());

        let function_output = json_deserialize_jwm(jwm_string);

        assert_match!(function_output, expected_jwm);
    }

    #[test]
    fn test_create_recipients() {
        let encrypted_keys = vec!["made_up_cek_1".to_string(),
                                            "made_up_cek_2".to_string(),
                                            "made_up_cek_3".to_string()];

        let recipient_vks = vec!["C5q2MDmdG26nVw73yhVhdz".to_string(),
                                            "4q37dbi1XXhaqcfXirTrin".to_string(),
                                            "8fnXqkWJmdex234Pe9EhC".to_string()];

        let sender_vk = "KLPrG3eq3DNZwveVd7NS7i".to_string();
        let auth = true;
        let recipients = create_receipients(encrypted_keys.clone(),
                                              recipient_vks.clone(),
                                                sender_vk.clone(),
                                                          auth);
        let expected_output = json!([
            {
                "header" : {
                    "typ" : "x-b64nacl",
                    "alg" : "x-auth",
                    "enc" : "xsalsa20poly1305",
                    "kid" : &recipient_vks[0],
                    "jwk" : &sender_vk
                },
                "encrypted_key" : &encrypted_keys[0]
            },
            {
                "header" : {
                    "typ" : "x-b64nacl",
                    "alg" : "x-auth",
                    "enc" : "xsalsa20poly1305",
                    "kid" : &recipient_vks[1],
                    "jwk" : &sender_vk
                },
                "encrypted_key" : &encrypted_keys[1]
            },
            {
                "header" : {
                    "typ" : "x-b64nacl",
                    "alg" : "x-auth",
                    "enc" : "xsalsa20poly1305",
                    "kid" : &recipient_vks[2],
                    "jwk" : &sender_vk
                },
                "encrypted_key" : &encrypted_keys[2]
            }
        ]);

        let recipients_json = json!(recipients).to_string();

        assert_eq!(recipients_json, expected_output.to_string());
    }
}