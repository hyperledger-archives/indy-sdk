use domain::route::*;
use errors::route::RouteError;
use utils::json::JsonDecodable;
use utils::crypto::base64::{encode, decode_to_string};
use serde_json::{from_str};
use serde_json;

pub fn json_serialize_jwm(recipient_vks : &Vec<String>,
                        encrypted_keys : &Vec<String>,
                        sender_vk : Option<&str>,
                        ciphertext : &str,
                        iv : &str,
                        tag : &str,
                        auth : bool) -> Result<String, RouteError> {

    let recipients = create_receipients(encrypted_keys, recipient_vks, sender_vk, auth)?;

    let jwm_full = AMESJson {
        recipients,
        ciphertext: ciphertext.to_string(),
        iv : iv.to_string(),
        tag : tag.to_string()
    };

    serde_json::to_string(&jwm_full)
        .map_err( | err | RouteError::EncodeError(format!("{}", err)))
}

pub fn json_deserialize_jwm(jwm : &str) -> Result<AMES, RouteError> {
    Ok(AMES::AMESFull(AMESJson::from_json(jwm)
        .map_err( | err | RouteError::DecodeError(format!("{}", err)))?))
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
pub mod tests {
    use super::*;
    use serde_json;
    use serde_json::{Value};


    //#[test]
    fn test_json_serialize_jwm() {
        let encrypted_keys = vec!["made_up_cek_1".to_string(),
                                            "made_up_cek_2".to_string(),
                                            "made_up_cek_3".to_string()];

        let recipient_vks = vec!["C5q2MDmdG26nVw73yhVhdz".to_string(),
                                            "4q37dbi1XXhaqcfXirTrin".to_string(),
                                            "8fnXqkWJmdex234Pe9EhC".to_string()];

        let sender_vk = "KLPrG3eq3DNZwveVd7NS7i";
        let auth = true;
        let ciphertext = "this is fake ciphertext to test JWMs";
        let iv = "FAKE_IVTOTESTJWMSERIALIZE";
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE";

        let expected_output : Value = json!({
        "recipients" : [
                {
                    "header" : {
                        "typ" : "x-b64nacl",
                        "alg" : "x-auth",
                        "enc" : "xsalsa20poly1305",
                        "kid" : &recipient_vks[0],
                        "jwk" : &sender_vk
                    },
                    "cek" : &encrypted_keys[0]
                },
                {
                    "header" : {
                        "typ" : "x-b64nacl",
                        "alg" : "x-auth",
                        "enc" : "xsalsa20poly1305",
                        "kid" : &recipient_vks[1],
                        "jwk" : &sender_vk
                    },
                    "cek" : &encrypted_keys[1]
                },
                {
                    "header" : {
                        "typ" : "x-b64nacl",
                        "alg" : "x-auth",
                        "enc" : "xsalsa20poly1305",
                        "kid" : &recipient_vks[2],
                        "jwk" : &sender_vk
                    },
                    "cek" : &encrypted_keys[2]
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
        assert!(function_output.eq(&expected_output));
    }

    //#[test]
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
        let expected_jwm = AMESJson {
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
        assert!(function_output == AMES::AMESFull(expected_jwm));
    }

    //#[test]
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
                "cek" : &encrypted_keys[0]
            },
            {
                "header" : {
                    "typ" : "x-b64nacl",
                    "alg" : "x-auth",
                    "enc" : "xsalsa20poly1305",
                    "kid" : &recipient_vks[1],
                    "jwk" : &sender_vk
                },
                "cek" : &encrypted_keys[1]
            },
            {
                "header" : {
                    "typ" : "x-b64nacl",
                    "alg" : "x-auth",
                    "enc" : "xsalsa20poly1305",
                    "kid" : &recipient_vks[2],
                    "jwk" : &sender_vk
                },
                "cek" : &encrypted_keys[2]
            }
        ]);

        let recipients_json = json!(recipients).to_string();

        assert_eq!(recipients_json, expected_output.to_string());
    }
}