use serde_json;
use serde_json::from_str;
use utils::crypto::base64::{encode, decode_to_string};
use utils::json::JsonDecodable;

use errors::route::RouteError;

pub struct Payload {
    pub iv: Vec<u8>,
    pub tag: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub sym_key: Vec<u8>
}

pub struct JWMData {
    pub header: Header,
    pub cek: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub iv: Vec<u8>,
    pub tag: Vec<u8>
}

pub struct CEK {
    pub cek : String,
    pub nonce : String,
    pub their_vk : String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForwardJSON {
    #[serde(rename = "type")]
    pub type_: String,
    pub to: String,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWMFull {
    pub recipients: Vec<Recipient>,
    pub ciphertext: String,
    pub iv: String,
    pub tag: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipient {
    pub header : Header,
    pub encrypted_key : String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
    pub typ : String,
    pub alg : String,
    pub enc : String,
    pub kid : String,
    pub jwk : Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JWMCompact {
    pub header : Header,
    pub cek : String,
    pub iv : String,
    pub ciphertext : String,
    pub tag : String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JWM {
    JWMFull(JWMFull),
    JWMCompact(JWMCompact)
}

impl Header {
    pub fn new_authcrypt_header(recipient_vk: &str, sender_vk: &str) -> Header {
        Header {
            typ: String::from("x-b64nacl"),
            alg: String::from("x-auth"),
            enc: String::from("xsalsa20poly1305"),
            kid: String::from(recipient_vk),
            jwk: Some(String::from(sender_vk)),
        }
    }

    pub fn new_anoncrypt_header(recipient_vk: &str) -> Header {
        Header {
            typ: String::from("x-b64nacl"),
            alg: String::from("x-anon"),
            enc: String::from("xsalsa20poly1305"),
            kid: String::from(recipient_vk),
            jwk: Some(String::from("")),
        }
    }
}

impl Recipient {
    pub fn new(header : Header, cek: String) -> Recipient {
        Recipient {
            header,
            encrypted_key: cek
        }
    }
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

pub fn json_serialize_jwm(recipient_vks : &Vec<String>,
                        encrypted_keys : &Vec<String>,
                        sender_vk : Option<&str>,
                        ciphertext : &str,
                        iv : &str,
                        tag : &str,
                        auth : bool) -> Result<String, RouteError> {

    let recipients = create_receipients(encrypted_keys, recipient_vks, sender_vk, auth)?;

    let jwm_full = JWMFull {
        recipients,
        ciphertext: ciphertext.to_string(),
        iv : iv.to_string(),
        tag : tag.to_string()
    };

    serde_json::to_string(&jwm_full)
        .map_err( | err | RouteError::EncodeError(format!("{}", err)))
}

pub fn json_deserialize_jwm(jwm : &str) -> Result<JWM, RouteError> {
    Ok(JWM::JWMFull(JWMFull::from_json(jwm)
        .map_err( | err | RouteError::DecodeError(format!("{}", err)))?))
}

pub fn deserialize_jwm_compact (message : &str) -> Result<JWM, RouteError> {
    let msg_as_vec: Vec<&str> = message.split('.').collect();
    let header_str = decode_to_string(msg_as_vec[0])?;
    let cek = decode_to_string(msg_as_vec[1])?;
    let iv = decode_to_string(msg_as_vec[2])?;
    let ciphertext = decode_to_string(msg_as_vec[3])?;
    let tag = decode_to_string(msg_as_vec[4])?;

    let header : Header = from_str(&header_str)
        .map_err( | err | RouteError::DecodeError(format!("{}", err)))?;

    Ok(JWM::JWMCompact(JWMCompact {
        header,
        cek,
        iv,
        ciphertext,
        tag
    }))
}

pub fn serialize_jwm_compact(recipient_vk : &str,
                             cek : &str,
                             sender_vk : Option<&str>,
                             ciphertext: &str,
                             iv : &str,
                             tag : &str,
                            auth : bool) -> Result<String, RouteError> {
    let header = match auth {
        true => match sender_vk {
            Some(vk) => Ok(Header::new_authcrypt_header(recipient_vk, vk)),
            None => Err(RouteError::MissingKeyError("No key included to build recipients list".to_string()))
        }
        false => Ok(Header::new_anoncrypt_header(recipient_vk))
    }?;

    let header_json = serde_json::to_string(&header).map_err(|err| RouteError::EncodeError(format!("Failed to encode jwm compact: {:?}", err)))?;

    Ok(format!("{}.{}.{}.{}.{}", encode(&header_json.as_bytes()),
                              encode(&cek.as_bytes()),
                              encode(&iv.as_bytes()),
                              encode(&ciphertext.as_bytes()),
                              encode(&tag.as_bytes())))

}

pub fn create_receipients(encrypted_keys : &Vec<String>,
                          recipient_vks : &Vec<String>,
                          sender_vk : Option<&str>,
                          auth : bool) -> Result<Vec<Recipient>, RouteError> {

    let mut recipients_list : Vec<Recipient> = vec![];
    for (i, value) in recipient_vks.iter().enumerate() {
        if auth {
            if sender_vk.is_some() {
                let header = Header::new_authcrypt_header(value,
                                                          sender_vk.unwrap());
                let recipient = Recipient::new(header, encrypted_keys[i].clone());
                recipients_list.push(recipient);
            } else {
                return Err(RouteError::MissingKeyError("No sender_vk included".to_string()))
            }
        } else {
            let header = Header::new_anoncrypt_header(value);
            let recipient = Recipient::new(header, encrypted_keys[i].clone());
            recipients_list.push(recipient);
        }
    }

    Ok(recipients_list)
}


#[cfg(test)]
mod tests {
    use super::{format_forward_msg, create_receipients, json_deserialize_jwm, json_serialize_jwm, serialize_jwm_compact, deserialize_jwm_compact, JWM, JWMFull};
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
        let forward_msg = format_forward_msg(message, &to_did).unwrap();

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

        let sender_vk = "KLPrG3eq3DNZwveVd7NS7i";
        let auth = true;
        let recipients = create_receipients(&encrypted_keys,
                                                                         &recipient_vks,
                                                               Some(sender_vk),
                                                                         auth);

        let ciphertext = "this is fake ciphertext to test JWMs";
        let iv = "FAKE_IVTOTESTJWMSERIALIZE";
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE";

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

        let jwm = json_serialize_jwm(&recipient_vks,
                                     &encrypted_keys,
                                     Some(sender_vk),
                                     ciphertext, iv, tag, auth ).unwrap();
        let function_output : Value = serde_json::from_str(&jwm).unwrap();
        assert_eq!(function_output, expected_output);
    }

    #[test]
    fn test_json_deserialize_jwm() {
        let encrypted_keys = vec!["made_up_cek_1".to_string(),
                                            "made_up_cek_2".to_string(),
                                            "made_up_cek_3".to_string()];

        let recipient_vks = vec!["C5q2MDmdG26nVw73yhVhdz".to_string(),
                                            "4q37dbi1XXhaqcfXirTrin".to_string(),
                                            "8fnXqkWJmdex234Pe9EhC".to_string()];

        let sender_vk = "KLPrG3eq3DNZwveVd7NS7i";
        let auth = true;

        let recipients = create_receipients(&encrypted_keys,
                                                          &recipient_vks,
                                                          Some(sender_vk),
                                                          auth);

        let ciphertext = "this is fake ciphertext to test JWMs";
        let iv = "FAKE_IVTOTESTJWMSERIALIZE";
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE";
        let expected_jwm = JWMFull {
            recipients : recipients.unwrap(),
            ciphertext : ciphertext.to_string(),
            iv : iv.to_string(),
            tag : tag.to_string()
        };

        let jwm_string = json_serialize_jwm(&recipient_vks,
                                                   &encrypted_keys,
                                                   Some(sender_vk),
                                                   ciphertext,
                                                   iv,
                                                   tag, auth).unwrap();

        let function_output = json_deserialize_jwm(&jwm_string).unwrap();

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

        let sender_vk = "KLPrG3eq3DNZwveVd7NS7i";
        let auth = true;

        let recipients = create_receipients(&encrypted_keys,
                                                          &recipient_vks,
                                                          Some(sender_vk),
                                                          auth).unwrap();

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

    #[test]
    fn test_serialize_jwm_compact() {
        let sender_vk = "EFbC4WxDXmFfHoyn7mCBnK";
        let recipient_vk = "C5q2MDmdG26nVw73yhVhdz";
        let cek = "encrypted_key";
        let ciphertext = "unencrypted text which would normally be encrypted already";
        let iv = "FAKE_IVTOTESTJWMSERIALIZE";
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE";
        let auth = true;

        //these were checked using an online encoder (https://simplycalc.com/base64-decode.php)
        let header_encoded = "eyJ0eXAiOiJ4LWI2NG5hY2wiLCJhbGciOiJ4LWF1dGgiLCJlbmMiOiJ4c2Fsc2EyMHBvbHkxMzA1Iiwia2lkIjoiQzVxMk1EbWRHMjZuVnc3M3loVmhkeiIsImp3ayI6IkVGYkM0V3hEWG1GZkhveW43bUNCbksifQ==";
        let cek_encoded = "ZW5jcnlwdGVkX2tleQ==";
        let iv_encoded = "RkFLRV9JVlRPVEVTVEpXTVNFUklBTElaRQ==";
        let ciphertext_encoded = "dW5lbmNyeXB0ZWQgdGV4dCB3aGljaCB3b3VsZCBub3JtYWxseSBiZSBlbmNyeXB0ZWQgYWxyZWFkeQ==";
        let tag_encoded = "RkFLRV9UQUdUT1RFU1RKV01TRVJJQUxJWkU=";

        let expected_result = format!("{}.{}.{}.{}.{}", header_encoded,
                                      cek_encoded,
                                      iv_encoded,
                                      ciphertext_encoded,
                                      tag_encoded);

        let jwm = serialize_jwm_compact(recipient_vk, cek,Some(sender_vk), ciphertext, iv, tag, auth).unwrap();
        assert_eq!(jwm, expected_result);
    }

    #[test]
    fn test_deserialize_jwm_compact() {
        let sender_vk = "EFbC4WxDXmFfHoyn7mCBnK".to_string();
        let recipient_vk = "C5q2MDmdG26nVw73yhVhdz".to_string();
        let cek = "encrypted_key".to_string();
        let ciphertext = "unencrypted text which would normally be encrypted already".to_string();
        let iv = "FAKE_IVTOTESTJWMSERIALIZE".to_string();
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE".to_string();
        let auth = true;

        let input = "eyJ0eXAiOiJ4LWI2NG5hY2wiLCJhbGciOiJ4LWF1dGgiLCJlbmMiOiJ4c2Fsc2EyMHBvbHkx\
        MzA1Iiwia2lkIjoiQzVxMk1EbWRHMjZuVnc3M3loVmhkeiIsImp3ayI6IkVGYkM0V3hEWG1GZkhveW43bUNCbksifQ==\
        .ZW5jcnlwdGVkX2tleQ==.RkFLRV9JVlRPVEVTVEpXTVNFUklBTElaRQ==.dW5lbmNyeXB0ZWQgdGV4dCB3aGljaCB3b3\
        VsZCBub3JtYWxseSBiZSBlbmNyeXB0ZWQgYWxyZWFkeQ==.RkFLRV9UQUdUT1RFU1RKV01TRVJJQUxJWkU=";

        let jwm =
            deserialize_jwm_compact(input).unwrap();

       if let JWM::JWMCompact(jwmc) = jwm {
                assert_eq!(jwmc.header.kid, recipient_vk);
                assert_eq!(jwmc.header.jwk.unwrap(), sender_vk);
                assert_eq!(jwmc.cek, cek);
                assert_eq!(jwmc.iv, iv);
                assert_eq!(jwmc.ciphertext, ciphertext);
                assert_eq!(jwmc.tag, tag);
        }
    }
}