use utils::crypto::base64::{encode, decode_to_string};
use services::route::jwm::Header;
use serde_json::{to_string, Error, from_str};

pub fn deserialize_jwm_compact (message : String) -> Result<(Header, String, String, String, String), Error> {
    let msg_as_vec: Vec<&str> = message.split('.').collect();
    let header_str = decode_to_string(msg_as_vec[0]);
    let cek = decode_to_string(msg_as_vec[1]);
    let iv = decode_to_string(msg_as_vec[2]);
    let ciphertext = decode_to_string(msg_as_vec[3]);
    let tag = decode_to_string(msg_as_vec[4]);

    match from_str(&header_str) {
        Ok(header) => Ok((header, cek, iv, ciphertext, tag)),
        Err(e) => Err(e)
    }
}

pub fn serialize_jwm_compact(sender_vk : String,
                             recipient_vk : String,
                             cek : String,
                             ciphertext: String,
                             iv : String,
                             tag : String,
                            auth : bool) -> Result<String, Error> {
    let header = match auth {
        true => Header::new_authcrypt_header(recipient_vk, sender_vk),
        false => Header::new_anoncrypt_header(recipient_vk, sender_vk)
    };

    let header_json = to_string(&header)?;

    Ok(format!("{}.{}.{}.{}.{}", encode(&header_json.as_bytes()),
                              encode(&cek.as_bytes()),
                              encode(&iv.as_bytes()),
                              encode(&ciphertext.as_bytes()),
                              encode(&tag.as_bytes())))

}

#[cfg(test)]
mod tests {
    use base64::{encode, URL_SAFE};
    use super::{serialize_jwm_compact, deserialize_jwm_compact};

    #[test]
    fn test_serialize_jwm_compact() {
        let sender_vk = "EFbC4WxDXmFfHoyn7mCBnK".to_string();
        let recipient_vk = "C5q2MDmdG26nVw73yhVhdz".to_string();
        let cek = "encrypted_key".to_string();
        let ciphertext = "unencrypted text which would normally be encrypted already".to_string();
        let iv = "FAKE_IVTOTESTJWMSERIALIZE".to_string();
        let tag = "FAKE_TAGTOTESTJWMSERIALIZE".to_string();
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

        let jwm = serialize_jwm_compact(sender_vk, recipient_vk, cek, ciphertext, iv, tag, auth).unwrap();
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

        let (header, cek_val, iv_val, ciphertext_val, tag_val ) =
            deserialize_jwm_compact(input.to_string()).unwrap();

        assert_eq!(header.kid.unwrap(), recipient_vk);
        assert_eq!(header.jwk.unwrap(), sender_vk);
        assert_eq!(cek_val, cek);
        assert_eq!(iv_val, iv);
        assert_eq!(ciphertext_val, ciphertext);
        assert_eq!(tag_val, tag);
    }
}


