use errors::prelude::*;

use super::wallet::Keys;
use super::language::{Operator,TargetValue,TagName};
use super::encryption::encrypt_as_searchable;


// Performs encryption of WQL query
// WQL query is provided as top-level Operator
// Recursively transforms operators using encrypt_operator function
pub(super) fn encrypt_query(operator: Operator, keys: &Keys) -> IndyResult<Operator> {
    operator.transform(&|op: Operator| -> Result<Operator, IndyError> {encrypt_operator(op, keys)})
}


fn encrypt_operator(op: Operator, keys: &Keys) -> Result<Operator, IndyError> {
    match op {
        Operator::Eq(name, value)  => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys)?;
            Ok(Operator::Eq(encrypted_name, encrypted_value))
        },
        Operator::Neq(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys)?;
            Ok(Operator::Neq(encrypted_name, encrypted_value))
        },
       Operator::Gt(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys)?;
            Ok(Operator::Gt(encrypted_name, encrypted_value))
        },
        Operator::Gte(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys)?;
            Ok(Operator::Gte(encrypted_name, encrypted_value))
        },
        Operator::Lt(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys)?;
            Ok(Operator::Lt(encrypted_name, encrypted_value))
        },
        Operator::Lte(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys)?;
            Ok(Operator::Lte(encrypted_name, encrypted_value))
        },
        Operator::Like(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(&name, value, keys)?;
            Ok(Operator::Like(encrypted_name, encrypted_value))
        },
        Operator::In(name, values) => {
            let name = match name {
                TagName::EncryptedTagName(ref name) => {
                    let encrypted_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
                    TagName::EncryptedTagName(encrypted_name)
                },
                TagName::PlainTagName(ref name) => {
                    let encrypted_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
                    TagName::PlainTagName(encrypted_name)
                }
            };
            let mut encrypted_values: Vec<TargetValue> = Vec::with_capacity(values.len());

            for value in values {
                encrypted_values.push(encrypt_name_value(&name, value, keys)?.1);
            }
            Ok(Operator::In(name, encrypted_values))
        },
        _ => Ok(op)
    }
}


// Encrypts a single tag name, tag value pair.
// If the tag name is EncryptedTagName enum variant, encrypts both the tag name and the tag value
// If the tag name is PlainTagName enum variant, encrypts only the tag name
fn encrypt_name_value(name: &TagName, value: TargetValue, keys: &Keys) -> IndyResult<(TagName, TargetValue)> {
    match (name, value) {
        (&TagName::EncryptedTagName(ref name), TargetValue::Unencrypted(ref s)) => {
            let encrypted_tag_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
            let encrypted_tag_value = encrypt_as_searchable(s.as_bytes(), &keys.tag_value_key, &keys.tags_hmac_key);
            Ok((TagName::EncryptedTagName(encrypted_tag_name), TargetValue::Encrypted(encrypted_tag_value)))
        },
        (&TagName::PlainTagName(ref name), TargetValue::Unencrypted(ref s)) => {
            let encrypted_tag_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
            Ok((TagName::PlainTagName(encrypted_tag_name), TargetValue::Unencrypted(s.clone())))
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Reached invalid combination of tag name and value while encrypting query"))
    }
}