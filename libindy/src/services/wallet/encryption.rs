use std::collections::HashMap;
use sodiumoxide::crypto::aead::xchacha20poly1305_ietf;
use sodiumoxide::crypto::auth::hmacsha256;


fn _encrypt_as_searchable(data: &[u8], key1: [u8; 32], key2: [u8; 32]) -> Vec<u8> {
    let hmacsha256::Tag(hash) = hmacsha256::authenticate(data, &hmacsha256::Key(key2));
    let mut nonce: [u8; xchacha20poly1305_ietf::NONCEBYTES] = Default::default();
    nonce.copy_from_slice(&hash[..xchacha20poly1305_ietf::NONCEBYTES]);
    let ct = xchacha20poly1305_ietf::seal(data, None, &xchacha20poly1305_ietf::Nonce(nonce), &xchacha20poly1305_ietf::Key(key1));
    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ct);
    result
}

fn _encrypt_as_not_searchable(data: &[u8], key: [u8; 32]) -> Vec<u8> {
    let nonce = xchacha20poly1305_ietf::gen_nonce();
    let ct = xchacha20poly1305_ietf::seal(data, None, &nonce, &xchacha20poly1305_ietf::Key(key));
    let mut result: Vec<u8> = Default::default();
    result.extend_from_slice(&nonce.0);
    result.extend_from_slice(&ct);
    result
}

fn _decrypt(enc_text: &Vec<u8>, key1: [u8; 32]) -> Result<Vec<u8>, WalletError> {
    let mut nonce: [u8; xchacha20poly1305_ietf::NONCEBYTES] = Default::default();
    nonce.copy_from_slice(&enc_text[..xchacha20poly1305_ietf::NONCEBYTES]);

    let mut cypher_text: Vec<u8> = Default::default();
    cypher_text.extend_from_slice(&enc_text[xchacha20poly1305_ietf::NONCEBYTES..]);

    match xchacha20poly1305_ietf::open(&cypher_text, None, &xchacha20poly1305_ietf::Nonce(nonce), &xchacha20poly1305_ietf::Key(key1)) {
        Err(_) => Err(WalletError::EncryptionError("Decryption error".to_string())),
        Ok(x) => Ok(x)
    }
}

fn _encrypt_tags(tags: &HashMap<String, String>, tag_name_key: [u8; 32], tag_value_key: [u8; 32], tags_hmac_key: [u8; 32]) -> HashMap<Vec<u8>, TagValue> {
    let mut etags: HashMap<Vec<u8>, TagValue> = HashMap::new();

    for (tag_name, tag_value) in tags {
        let ekey = _encrypt_as_searchable(tag_name.as_bytes(), tag_name_key, tags_hmac_key);
        if tag_name.chars().next() == Some('~') {
            etags.insert(ekey, TagValue::Plain(tag_value.to_string()));
        }
        else {
            etags.insert(ekey, TagValue::Encrypted(_encrypt_as_searchable(tag_value.as_bytes(), tag_value_key, tags_hmac_key)));
        }
    }
    etags
}

fn _decrypt_tags(etags: &Option<HashMap<Vec<u8>, TagValue>>, tag_name_key: [u8; 32], tag_value_key: [u8; 32]) -> Result<Option<HashMap<String, String>>, WalletError> {
    match etags {
        &None => Ok(None),
        &Some(ref etags) => {
            let mut tags: HashMap<String, String> = HashMap::new();

            for (ref etag_name, ref etag_value) in etags {
                let tag_name = match _decrypt(&etag_name, tag_name_key) {
                    Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag name".to_string())),
                    Ok(tag_name) => String::from_utf8(tag_name)?
                };

                let tag_value = match etag_value {
                    &&TagValue::Plain(ref plain_value) => plain_value.clone(),
                    &&TagValue::Encrypted(ref evalue) |
                    &&TagValue::Meta(ref evalue) => match _decrypt(&evalue, tag_value_key) {
                        Err(_) => return Err(WalletError::EncryptionError("Unable to decrypt tag value".to_string())),
                        Ok(tag_value) => String::from_utf8(tag_value)?
                    }
                };
                tags.insert(tag_name, tag_value);
            }

            Ok(Some(tags))
        }
    }
}

fn encrypt_query(operator: Operator, keys: &Keys) -> Operator {
    operator.transform(&|op: Operator| -> Operator {encrypt_operator(op, keys)})
}

fn encrypt_operator(op: Operator, keys: &Keys) -> Operator {
    match op {
        Operator::Eq(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Eq(encrypted_name, encrypted_value)
        },
        Operator::Neq(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Neq(encrypted_name, encrypted_value)
        },
       Operator::Gt(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Gt(encrypted_name, encrypted_value)
        },
        Operator::Gte(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Gte(encrypted_name, encrypted_value)
        },
        Operator::Lt(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Lt(encrypted_name, encrypted_value)
        },
        Operator::Lte(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Lte(encrypted_name, encrypted_value)
        },
        Operator::Like(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Like(encrypted_name, encrypted_value)
        },
        Operator::Regex(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys);
            Operator::Regex(encrypted_name, encrypted_value)
        },
        Operator::In(name, values) => {
            let name = match name {
                TagName::EncryptedTagName(ref name) => {
                    let encrypted_name = _encrypt_as_searchable(&name[..], keys.tag_name_key, keys.tags_hmac_key);
                    TagName::EncryptedTagName(encrypted_name)
                },
                TagName::PlainTagName(ref name) => {
                    let encrypted_name = _encrypt_as_searchable(&name[..], keys.tag_name_key, keys.tags_hmac_key);
                    TagName::PlainTagName(encrypted_name)
                },
                _ => panic!("TODO")
            };
            let mut encrypted_values: Vec<TargetValue> = Vec::new();

            for value in values {
                encrypted_values.push(encrypt_name_value(&name, value, keys).1);
            }
            Operator::In(name, encrypted_values)

            // TODO - logic here for refusing if encrypted name has nonencrypted values and vice versa
        },
        _ => op
    }
}

fn encrypt_name_value(name: &TagName, value: TargetValue, keys: &Keys) -> (TagName, TargetValue) {
    match (name, value) {
        (&TagName::EncryptedTagName(ref name), TargetValue::Unencrypted(ref s)) => {
            let encrypted_tag_name = _encrypt_as_searchable(&name[..], keys.tag_name_key, keys.tags_hmac_key);
            let encrypted_tag_value = _encrypt_as_searchable(s.as_bytes(), keys.tag_value_key, keys.tags_hmac_key);
            (TagName::EncryptedTagName(encrypted_tag_name), TargetValue::Encrypted(encrypted_tag_value))
        },
        // TODO - optimize
        (&TagName::PlainTagName(ref name), TargetValue::Unencrypted(ref s)) => {
            let encrypted_tag_name = _encrypt_as_searchable(&name[..], keys.tag_name_key, keys.tags_hmac_key);
            (TagName::PlainTagName(encrypted_tag_name), TargetValue::Unencrypted(s.clone()))
        },
        _ => panic!("TODO")
    }
}
