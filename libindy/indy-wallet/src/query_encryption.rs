use indy_api_types::errors::prelude::*;

use super::wallet::Keys;
use super::language::{Operator, TargetValue, TagName};
use super::encryption::encrypt_as_searchable;
use indy_utils::wql::Query;

// Performs encryption of WQL query
// WQL query is provided as top-level Operator
pub(super) fn encrypt_query(query: Query, keys: &Keys) -> IndyResult<Operator> {
    transform(query, keys)
}

fn transform(query: Query, keys: &Keys) -> IndyResult<Operator> {
    match query {
        Query::Eq(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(name, value, keys)?;
            Ok(Operator::Eq(encrypted_name, encrypted_value))
        }
        Query::Neq(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(name, value, keys)?;
            Ok(Operator::Neq(encrypted_name, encrypted_value))
        }
        Query::Gt(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(name, value, keys)?;
            Ok(Operator::Gt(encrypted_name, encrypted_value))
        }
        Query::Gte(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(name, value, keys)?;
            Ok(Operator::Gte(encrypted_name, encrypted_value))
        }
        Query::Lt(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(name, value, keys)?;
            Ok(Operator::Lt(encrypted_name, encrypted_value))
        }
        Query::Lte(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(name, value, keys)?;
            Ok(Operator::Lte(encrypted_name, encrypted_value))
        }
        Query::Like(name, value) => {
            let (encrypted_name, encrypted_value) = encrypt_name_value(name, value, keys)?;
            Ok(Operator::Like(encrypted_name, encrypted_value))
        }
        Query::In(name, values) => {
            let ename = TagName::from(name.clone())?;
            let ename = match ename {
                TagName::EncryptedTagName(ref name) => {
                    let encrypted_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
                    TagName::EncryptedTagName(encrypted_name)
                }
                TagName::PlainTagName(ref name) => {
                    let encrypted_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
                    TagName::PlainTagName(encrypted_name)
                }
            };
            let mut encrypted_values: Vec<TargetValue> = Vec::with_capacity(values.len());

            for value in values {
                encrypted_values.push(encrypt_name_value(name.clone(), value, keys)?.1);
            }
            Ok(Operator::In(ename, encrypted_values))
        }
        Query::And(operators) => Ok(Operator::And(transform_list_operators(operators, keys)?)),
        Query::Or(operators) => Ok(Operator::Or(transform_list_operators(operators, keys)?)),
        Query::Not(boxed_operator) => Ok(Operator::Not(Box::new(transform(*boxed_operator, keys)?)))
    }
}

fn transform_list_operators(operators: Vec<Query>, keys: &Keys) -> IndyResult<Vec<Operator>> {
    let mut transformed = Vec::with_capacity(operators.len());

    for operator in operators {
        let transformed_operator = transform(operator, keys)?;
        transformed.push(transformed_operator);
    }

    Ok(transformed)
}

// Encrypts a single tag name, tag value pair.
// If the tag name is EncryptedTagName enum variant, encrypts both the tag name and the tag value
// If the tag name is PlainTagName enum variant, encrypts only the tag name
fn encrypt_name_value(name: String, value: String, keys: &Keys) -> IndyResult<(TagName, TargetValue)> {
    let name = TagName::from(name)?;
    let value = TargetValue::from(value);
    match (name, value) {
        (TagName::EncryptedTagName(ref name), TargetValue::Unencrypted(ref s)) => {
            let encrypted_tag_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
            let encrypted_tag_value = encrypt_as_searchable(s.as_bytes(), &keys.tag_value_key, &keys.tags_hmac_key);
            Ok((TagName::EncryptedTagName(encrypted_tag_name), TargetValue::Encrypted(encrypted_tag_value)))
        }
        (TagName::PlainTagName(ref name), TargetValue::Unencrypted(ref s)) => {
            let encrypted_tag_name = encrypt_as_searchable(&name[..], &keys.tag_name_key, &keys.tags_hmac_key);
            Ok((TagName::PlainTagName(encrypted_tag_name), TargetValue::Unencrypted(s.clone())))
        }
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Reached invalid combination of tag name and value while encrypting query"))
    }
}