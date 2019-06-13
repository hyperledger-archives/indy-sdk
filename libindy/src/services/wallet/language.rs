use std::string;

use serde_json;

use errors::prelude::*;
use utils::crypto::base64;

#[derive(Debug, PartialEq, Hash, Clone)]
pub enum TagName {
    EncryptedTagName(Vec<u8>),
    PlainTagName(Vec<u8>),
}


impl TagName {
    fn from(s: String) -> IndyResult<TagName> {
        if s.is_empty() || s.starts_with('~') && s.len() == 1 {
            return Err(err_msg(IndyErrorKind::WalletQueryError, "Tag name must not be empty"));
        }

        if s.starts_with('~') {
            Ok(TagName::PlainTagName(s.into_bytes()[1..].to_vec()))
        } else {
            Ok(TagName::EncryptedTagName(s.into_bytes()))
        }
    }

    pub fn from_utf8(&self) -> IndyResult<String> {
        let tag_name = match *self {
            TagName::EncryptedTagName(ref v) => v,
            TagName::PlainTagName(ref v) => v,
        };

        String::from_utf8(tag_name.clone()).map_err(|err| {
            err_msg(IndyErrorKind::InvalidStructure, format!("Failed to convert message to UTF-8 {}", err))
        })
    }
}

impl string::ToString for TagName {
    fn to_string(&self) -> String {
        match *self {
            TagName::EncryptedTagName(ref v) => format!(r#""{}""#, base64::encode(v)),
            TagName::PlainTagName(ref v) => format!(r#""~{}""#, base64::encode(v))
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub enum TargetValue {
    Unencrypted(String),
    Encrypted(Vec<u8>),
}

impl TargetValue {
    pub fn value(&self) -> String {
        match *self {
            TargetValue::Unencrypted(ref s) => s.to_string(),
            TargetValue::Encrypted(ref v) => base64::encode(v),
        }
    }
}

impl From<String> for TargetValue {
    fn from(s: String) -> TargetValue {
        TargetValue::Unencrypted(s)
    }
}

impl string::ToString for TargetValue {
    fn to_string(&self) -> String {
        match *self {
            TargetValue::Unencrypted(ref s) => format!(r#""{}""#, s),
            TargetValue::Encrypted(ref v) => format!(r#""{}""#, base64::encode(v)),
        }
    }
}


#[derive(Debug, Hash, Clone)]
pub enum Operator {
    And(Vec<Operator>),
    Or(Vec<Operator>),
    Not(Box<Operator>),
    Eq(TagName, TargetValue),
    Neq(TagName, TargetValue),
    Gt(TagName, TargetValue),
    Gte(TagName, TargetValue),
    Lt(TagName, TargetValue),
    Lte(TagName, TargetValue),
    Like(TagName, TargetValue),
    In(TagName, Vec<TargetValue>),
}


impl Operator {
    pub fn transform(self, f: &Fn(Operator) -> IndyResult<Operator>) -> IndyResult<Operator> {
        match self {
            Operator::And(operators) => Ok(Operator::And(Operator::transform_list_operators(operators, f)?)),
            Operator::Or(operators) => Ok(Operator::Or(Operator::transform_list_operators(operators, f)?)),
            Operator::Not(boxed_operator) => Ok(Operator::Not(Box::new(Operator::transform(*boxed_operator, f)?))),
            _ => Ok(f(self)?)
        }
    }

    fn transform_list_operators(operators: Vec<Operator>, f: &Fn(Operator) -> IndyResult<Operator>) -> IndyResult<Vec<Operator>> {
        let mut transformed = Vec::new();

        for operator in operators {
            let transformed_operator = Operator::transform(operator, f)?;
            transformed.push(transformed_operator);
        }

        Ok(transformed)
    }

    fn optimise(self) -> Operator {
        match self {
            Operator::Not(boxed_operator) => if let Operator::Not(nested_operator) = *boxed_operator {
                *nested_operator
            } else {
                Operator::Not(boxed_operator)
            },
            Operator::And(mut suboperators) => if suboperators.len() == 1 {
                suboperators.remove(0)
            } else {
                Operator::And(suboperators)
            },
            Operator::Or(mut suboperators) => if suboperators.len() == 1 {
                suboperators.remove(0)
            } else {
                Operator::Or(suboperators)
            },
            Operator::In(key, mut targets) => if targets.len() == 1 {
                Operator::Eq(key, targets.remove(0))
            } else {
                Operator::In(key, targets)
            },
            _ => self
        }
    }
}

impl string::ToString for Operator {
    fn to_string(&self) -> String {
        match *self {
            Operator::Eq(ref tag_name, ref tag_value) => format!(r#"{{{}:{}}}"#, tag_name.to_string(), tag_value.to_string()),
            Operator::Neq(ref tag_name, ref tag_value) => format!(r#"{{{}:{{"$neq":{}}}}}"#, tag_name.to_string(), tag_value.to_string()),
            Operator::Gt(ref tag_name, ref tag_value) => format!(r#"{{{}:{{"$gt":{}}}}}"#, tag_name.to_string(), tag_value.to_string()),
            Operator::Gte(ref tag_name, ref tag_value) => format!(r#"{{{}:{{"$gte":{}}}}}"#, tag_name.to_string(), tag_value.to_string()),
            Operator::Lt(ref tag_name, ref tag_value) => format!(r#"{{{}:{{"$lt":{}}}}}"#, tag_name.to_string(), tag_value.to_string()),
            Operator::Lte(ref tag_name, ref tag_value) => format!(r#"{{{}:{{"$lte":{}}}}}"#, tag_name.to_string(), tag_value.to_string()),
            Operator::Like(ref tag_name, ref tag_value) => format!(r#"{{{}:{{"$like":{}}}}}"#, tag_name.to_string(), tag_value.to_string()),
            Operator::In(ref tag_name, ref tag_values) => {
                format!(
                    r#"{{{}:{{"$in":[{}]}}}}"#,
                    tag_name.to_string(),
                    tag_values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")
                )
            }
            Operator::And(ref operators) => {
                if operators.len() > 0 {
                    format!(
                        r#"{{"$and":[{}]}}"#,
                        operators.iter().map(|o: &Operator| { o.to_string() }).collect::<Vec<String>>().join(","))
                } else { "{}".to_string() }
            },
            Operator::Or(ref operators) => {
                if operators.len() > 0 {
                    format!(
                        r#"{{"$or":[{}]}}"#,
                        operators.iter().map(|o: &Operator| { o.to_string() }).collect::<Vec<String>>().join(","))
                } else { "{}".to_string() }
            },
            Operator::Not(ref stmt) => format!(r#"{{"$not":{}}}"#, stmt.to_string()),
        }
    }
}

pub fn parse_from_json(json: &str) -> IndyResult<Operator> {
    let query = serde_json::from_str::<serde_json::Value>(json)
        .to_indy(IndyErrorKind::WalletQueryError, "Query is malformed json")?;

    if let serde_json::Value::Object(map) = query {
        parse(map)
    } else {
        Err(err_msg(IndyErrorKind::WalletQueryError, "Query must be JSON object"))
    }
}


fn parse(map: serde_json::Map<String, serde_json::Value>) -> IndyResult<Operator> {
    let mut operators: Vec<Operator> = Vec::new();

    for (key, value) in map.into_iter() {
        let suboperator = parse_operator(key, value)?;
        operators.push(suboperator);
    }

    let top_operator = Operator::And(operators);
    Ok(top_operator.optimise())
}


fn parse_operator(key: String, value: serde_json::Value) -> IndyResult<Operator> {
    match (&*key, value) {
        ("$and", serde_json::Value::Array(values)) => {
            let mut operators: Vec<Operator> = Vec::new();

            for value in values.into_iter() {
                if let serde_json::Value::Object(map) = value {
                    let suboperator = parse(map)?;
                    operators.push(suboperator);
                } else {
                    return Err(err_msg(IndyErrorKind::WalletQueryError, "$and must be array of JSON objects"));
                }
            }

            Ok(Operator::And(operators))
        }
        ("$and", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$and must be array of JSON objects")),
        ("$or", serde_json::Value::Array(values)) => {
            let mut operators: Vec<Operator> = Vec::new();

            for value in values.into_iter() {
                if let serde_json::Value::Object(map) = value {
                    let suboperator = parse(map)?;
                    operators.push(suboperator);
                } else {
                    return Err(err_msg(IndyErrorKind::WalletQueryError, "$or must be array of JSON objects"));
                }
            }

            Ok(Operator::Or(operators))
        }
        ("$or", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$or must be array of JSON objects")),
        ("$not", serde_json::Value::Object(map)) => {
            let operator = parse(map)?;
            Ok(Operator::Not(Box::new(operator)))
        }
        ("$not", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$not must be JSON object")),
        (_, serde_json::Value::String(value)) => Ok(Operator::Eq(TagName::from(key)?, TargetValue::from(value))),
        (_, serde_json::Value::Object(map)) => {
            if map.len() == 1 {
                let (operator_name, value) = map.into_iter().next().unwrap();
                parse_single_operator(operator_name, key, value)
            } else {
                Err(err_msg(IndyErrorKind::WalletQueryError, format!("{} value must be JSON object of length 1", key)))
            }
        }
        (_, _) => Err(err_msg(IndyErrorKind::WalletQueryError, format!("Unsupported value for key: {}", key)))
    }
}

fn parse_single_operator(operator_name: String, key: String, value: serde_json::Value) -> IndyResult<Operator> {
    match (&*operator_name, value) {
        ("$neq", serde_json::Value::String(s)) => Ok(Operator::Neq(TagName::from(key)?, TargetValue::from(s))),
        ("$neq", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$neq must be used with string")),
        ("$gt", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Gt(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(err_msg(IndyErrorKind::WalletQueryError, "$gt must be used only for nonencrypted tag"))
            }
        }
        ("$gt", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$gt must be used with string")),
        ("$gte", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Gte(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(err_msg(IndyErrorKind::WalletQueryError, "$gte must be used only for nonencrypted tag"))
            }
        }
        ("$gte", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$gte must be used with string")),
        ("$lt", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Lt(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(err_msg(IndyErrorKind::WalletQueryError, "$lt must be used only for nonencrypted tag"))
            }
        }
        ("$lt", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$lt must be used with string")),
        ("$lte", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Lte(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(err_msg(IndyErrorKind::WalletQueryError, "$lte must be used only for nonencrypted tag"))
            }
        }
        ("$lte", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$lte must be used with string")),
        ("$like", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Like(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(err_msg(IndyErrorKind::WalletQueryError, "$like must be used only for nonencrypted tag"))
            }
        }
        ("$like", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$like must be used with string")),
        ("$in", serde_json::Value::Array(values)) => {
            let mut target_values: Vec<TargetValue> = Vec::new();

            for v in values.into_iter() {
                if let serde_json::Value::String(s) = v {
                    target_values.push(TargetValue::from(s));
                } else {
                    return Err(err_msg(IndyErrorKind::WalletQueryError, "$in must be used with array of strings"));
                }
            }

            Ok(Operator::In(TagName::from(key)?, target_values))
        }
        ("$in", _) => Err(err_msg(IndyErrorKind::WalletQueryError, "$in must be used with array of strings")),
        (_, _) => Err(err_msg(IndyErrorKind::WalletQueryError, format!("Unknown operator: {}", operator_name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use rand::distributions::{Alphanumeric, Standard};
    use std::hash::Hash;
    use std::collections::HashSet;

    fn _random_vector(len: usize) -> Vec<u8> {
        thread_rng().sample_iter(&Standard).take(len).collect()
    }

    fn _random_string(len: usize) -> String {
        thread_rng().sample_iter(&Alphanumeric).take(len).collect()
    }

    trait ToVec {
        fn to_vec(&self) -> Vec<u8>;
    }

    impl ToVec for String {
        fn to_vec(&self) -> Vec<u8> {
            self.as_bytes().to_owned()
        }
    }

    fn vec_to_set<T>(vec: &Vec<T>) -> HashSet<T> where T: Eq + Hash + Clone {
        let mut result: HashSet<T> = HashSet::new();
        for x in vec {
            result.insert((*x).clone());
        }
        result
    }

    impl PartialEq for Operator {
        fn eq(&self, other: &Operator) -> bool {
            match (self, other) {
                (Operator::Eq(name, value), Operator::Eq(other_name, other_value))
                | (Operator::Neq(name, value), Operator::Neq(other_name, other_value))
                | (Operator::Gt(name, value), Operator::Gt(other_name, other_value))
                | (Operator::Gte(name, value), Operator::Gte(other_name, other_value))
                | (Operator::Lt(name, value), Operator::Lt(other_name, other_value))
                | (Operator::Lte(name, value), Operator::Lte(other_name, other_value))
                | (Operator::Like(name, value), Operator::Like(other_name, other_value)) => {
                    name == other_name && value == other_value
                },
                (Operator::In(name, values), Operator::In(other_name, other_values)) => {
                    name == other_name && vec_to_set(values) == vec_to_set(other_values)
                },
                (Operator::Not(operator), Operator::Not(other_operator)) => operator == other_operator,
                (Operator::And(operators), Operator::And(other_operators))
                | (Operator::Or(operators), Operator::Or(other_operators)) => {
                    vec_to_set(operators) == vec_to_set(other_operators)
                },
                (_, _) => false
            }
        }
    }

    impl Eq for Operator {}

    /// parse

    #[test]
    fn test_simple_operator_empty_json_parse() {
        let json = "{}";

        let query = parse_from_json(json).unwrap();

        let expected = Operator::And(vec![]);

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_explicit_empty_and_parse() {
        let json = r#"{"$and":[]}"#;

        let query = parse_from_json(json).unwrap();

        let expected = Operator::And(vec![]);

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_empty_or_parse() {
        let json = r#"{"$or":[]}"#;

        let query = parse_from_json(json).unwrap();

        let expected = Operator::Or(vec![]);

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_empty_not_parse() {
        let json = r#"{"$not":{}}"#;

        let query = parse_from_json(json).unwrap();

        let expected = Operator::Not(Box::new(Operator::And(vec![])));

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_eq_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":"{}"}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Eq(
            TagName::PlainTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_eq_encrypted_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"{}":"{}"}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Eq(
            TagName::EncryptedTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_neq_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$neq":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Neq(
            TagName::PlainTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_neq_encrypted_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"{}":{{"$neq":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Neq(
            TagName::EncryptedTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_gt_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$gt":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Gt(
            TagName::PlainTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_gte_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$gte":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Gte(
            TagName::PlainTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_lt_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$lt":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Lt(
            TagName::PlainTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_lte_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$lte":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Lte(
            TagName::PlainTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_like_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$like":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Like(
            TagName::PlainTagName(name1.to_vec()),
            TargetValue::Unencrypted(value1.clone())
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_in_plaintext_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$in":["{}"]}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::In(
            TagName::PlainTagName(name1.to_vec()),
            vec![TargetValue::Unencrypted(value1.clone())]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_in_plaintexts_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let value2 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"~{}":{{"$in":["{}","{}","{}"]}}}}"#, name1, value1, value2, value3);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::In(
            TagName::PlainTagName(name1.to_vec()),
            vec![
                TargetValue::Unencrypted(value1.clone()),
                TargetValue::Unencrypted(value2.clone()),
                TargetValue::Unencrypted(value3.clone()),
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_in_encrypted_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"{}":{{"$in":["{}"]}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::In(
            TagName::EncryptedTagName(name1.to_vec()),
            vec![TargetValue::Unencrypted(value1.clone())]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_simple_operator_in_encrypted_values_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let value2 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"{}":{{"$in":["{}","{}","{}"]}}}}"#, name1, value1, value2, value3);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::In(
            TagName::EncryptedTagName(name1.to_vec()),
            vec![
                TargetValue::Unencrypted(value1.clone()),
                TargetValue::Unencrypted(value2.clone()),
                TargetValue::Unencrypted(value3.clone()),
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":"{}"}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_neq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$neq":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_gt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$gt":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_gte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$gte":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_lt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$lt":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_lte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$lte":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_like_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$like":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_in_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$in":["{}"]}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.to_vec()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_one_not_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"$not":{{"~{}":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.to_vec()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_short_and_with_multiple_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"~{}":"{}","~{}":"{}","~{}":"{}"}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":"{}"}},{{"~{}":"{}"}},{{"~{}":"{}"}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_neq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_gt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_gte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_lt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_lte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_like_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_in_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.to_vec()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name2.to_vec()),
                    vec![TargetValue::Unencrypted(value2.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name3.to_vec()),
                    vec![TargetValue::Unencrypted(value3.clone())]
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_not_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.to_vec()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name2.to_vec()),
                            TargetValue::Unencrypted(value2.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name3.to_vec()),
                            TargetValue::Unencrypted(value3.clone())
                        )
                    )
                ),
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_with_multiple_mixed_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);
        let name4 = _random_string(10);
        let value4 = _random_string(10);
        let name5 = _random_string(10);
        let value5 = _random_string(10);
        let name6 = _random_string(10);
        let value6 = _random_string(10);
        let name7 = _random_string(10);
        let value7 = _random_string(10);
        let name8 = _random_string(10);
        let value8a = _random_string(10);
        let value8b = _random_string(10);
        let name9 = _random_string(10);
        let value9 = _random_string(10);

        let json = format!(r#"{{"$and":[{{"~{}":"{}"}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$in":["{}","{}"]}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
                           name4, value4,
                           name5, value5,
                           name6, value6,
                           name7, value7,
                           name8, value8a, value8b,
                           name9, value9,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::And(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name4.to_vec()),
                    TargetValue::Unencrypted(value4.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name5.to_vec()),
                    TargetValue::Unencrypted(value5.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name6.to_vec()),
                    TargetValue::Unencrypted(value6.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name7.to_vec()),
                    TargetValue::Unencrypted(value7.clone())
                ),
                Operator::In(
                    TagName::PlainTagName(name8.to_vec()),
                    vec![
                        TargetValue::Unencrypted(value8a.clone()),
                        TargetValue::Unencrypted(value8b.clone())
                    ]
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name9.to_vec()),
                            TargetValue::Unencrypted(value9.clone())
                        )
                    )
                ),
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":"{}"}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_neq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$neq":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_gt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$gt":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_gte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$gte":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_lt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$lt":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_lte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$lte":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_like_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$like":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_in_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$in":["{}"]}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.to_vec()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_one_not_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"$not":{{"~{}":"{}"}}}}]}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.to_vec()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":"{}"}},{{"~{}":"{}"}},{{"~{}":"{}"}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_neq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_gt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_gte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_lt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_lte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_like_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_in_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.to_vec()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name2.to_vec()),
                    vec![TargetValue::Unencrypted(value2.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name3.to_vec()),
                    vec![TargetValue::Unencrypted(value3.clone())]
                )
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_not_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.to_vec()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name2.to_vec()),
                            TargetValue::Unencrypted(value2.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name3.to_vec()),
                            TargetValue::Unencrypted(value3.clone())
                        )
                    )
                ),
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_or_with_multiple_mixed_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);
        let name4 = _random_string(10);
        let value4 = _random_string(10);
        let name5 = _random_string(10);
        let value5 = _random_string(10);
        let name6 = _random_string(10);
        let value6 = _random_string(10);
        let name7 = _random_string(10);
        let value7 = _random_string(10);
        let name8 = _random_string(10);
        let value8a = _random_string(10);
        let value8b = _random_string(10);
        let name9 = _random_string(10);
        let value9 = _random_string(10);

        let json = format!(r#"{{"$or":[{{"~{}":"{}"}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$in":["{}","{}"]}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
                           name4, value4,
                           name5, value5,
                           name6, value6,
                           name7, value7,
                           name8, value8a, value8b,
                           name9, value9,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Or(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.to_vec()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.to_vec()),
                    TargetValue::Unencrypted(value3.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name4.to_vec()),
                    TargetValue::Unencrypted(value4.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name5.to_vec()),
                    TargetValue::Unencrypted(value5.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name6.to_vec()),
                    TargetValue::Unencrypted(value6.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name7.to_vec()),
                    TargetValue::Unencrypted(value7.clone())
                ),
                Operator::In(
                    TagName::PlainTagName(name8.to_vec()),
                    vec![
                        TargetValue::Unencrypted(value8a.clone()),
                        TargetValue::Unencrypted(value8b.clone())
                    ]
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name9.to_vec()),
                            TargetValue::Unencrypted(value9.clone())
                        )
                    )
                ),
            ]
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_eq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":"{}"}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::Eq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_neq_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":{{"$neq":"{}"}}}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::Neq(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_gt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":{{"$gt":"{}"}}}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::Gt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_gte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":{{"$gte":"{}"}}}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::Gte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_lt_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":{{"$lt":"{}"}}}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::Lt(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_lte_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":{{"$lte":"{}"}}}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::Lte(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_like_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":{{"$like":"{}"}}}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::Like(
                    TagName::PlainTagName(name1.to_vec()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_not_with_one_in_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);

        let json = format!(r#"{{"$not":{{"~{}":{{"$in":["{}"]}}}}}}"#, name1, value1);

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::In(
                    TagName::PlainTagName(name1.to_vec()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                )
            )
        );

        assert_eq!(query, expected);
    }

    #[test]
    fn test_and_or_not_complex_case_parse() {
        let name1 = _random_string(10);
        let value1 = _random_string(10);
        let name2 = _random_string(10);
        let value2 = _random_string(10);
        let name3 = _random_string(10);
        let value3 = _random_string(10);
        let name4 = _random_string(10);
        let value4 = _random_string(10);
        let name5 = _random_string(10);
        let value5 = _random_string(10);
        let name6 = _random_string(10);
        let value6 = _random_string(10);
        let name7 = _random_string(10);
        let value7 = _random_string(10);
        let name8 = _random_string(10);
        let value8 = _random_string(10);

        let json = format!(r#"{{"$not":{{"$and":[{{"~{}":"{}"}},{{"$or":[{{"~{}":{{"$gt":"{}"}}}},{{"$not":{{"~{}":{{"$lte":"{}"}}}}}},{{"$and":[{{"~{}":{{"$lt":"{}"}}}},{{"$not":{{"~{}":{{"$gte":"{}"}}}}}}]}}]}},{{"$not":{{"~{}":{{"$like":"{}"}}}}}},{{"$and":[{{"~{}":"{}"}},{{"$not":{{"~{}":{{"$neq":"{}"}}}}}}]}}]}}}}"#,
                           name1, value1,
                           name2, value2,
                           name3, value3,
                           name4, value4,
                           name5, value5,
                           name6, value6,
                           name7, value7,
                           name8, value8,
        );

        let query = parse_from_json(&json).unwrap();

        let expected = Operator::Not(
            Box::new(
                Operator::And(
                    vec![
                        Operator::Eq(
                            TagName::PlainTagName(name1.to_vec()),
                            TargetValue::Unencrypted(value1.clone())
                        ),
                        Operator::Or(
                            vec![
                                Operator::Gt(
                                    TagName::PlainTagName(name2.to_vec()),
                                    TargetValue::Unencrypted(value2.clone())
                                ),
                                Operator::Not(
                                    Box::new(
                                        Operator::Lte(
                                            TagName::PlainTagName(name3.to_vec()),
                                            TargetValue::Unencrypted(value3.clone())
                                        )
                                    )
                                ),
                                Operator::And(
                                    vec![
                                        Operator::Lt(
                                            TagName::PlainTagName(name4.to_vec()),
                                            TargetValue::Unencrypted(value4.clone())
                                        ),
                                        Operator::Not(
                                            Box::new(
                                                Operator::Gte(
                                                    TagName::PlainTagName(name5.to_vec()),
                                                    TargetValue::Unencrypted(value5.clone())
                                                )
                                            )
                                        ),
                                    ]
                                )
                            ]
                        ),
                        Operator::Not(
                            Box::new(
                                Operator::Like(
                                    TagName::PlainTagName(name6.to_vec()),
                                    TargetValue::Unencrypted(value6.clone())
                                )
                            )
                        ),
                        Operator::And(
                            vec![
                                Operator::Eq(
                                    TagName::PlainTagName(name7.to_vec()),
                                    TargetValue::Unencrypted(value7.clone())
                                ),
                                Operator::Not(
                                    Box::new(
                                        Operator::Neq(
                                            TagName::PlainTagName(name8.to_vec()),
                                            TargetValue::Unencrypted(value8.clone())
                                        )
                                    )
                                ),
                            ]
                        )
                    ]
                )
            )
        );

        assert_eq!(query, expected);
    }

    /// to string

    #[test]
    fn test_simple_operator_empty_and_to_string() {
        let query = Operator::And(vec![]);

        let json = query.to_string();

        let expected = "{}";

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_empty_or_to_string() {
        let query = Operator::Or(vec![]);

        let json = query.to_string();

        let expected = "{}";

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_empty_not_to_string() {
        let query = Operator::Not(Box::new(Operator::And(vec![])));

        let json = query.to_string();

        let expected = r#"{"$not":{}}"#;

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_eq_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Eq(
            TagName::PlainTagName(name1.clone()),
            TargetValue::Unencrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":"{}"}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_eq_encrypted_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_vector(10);

        let query = Operator::Eq(
            TagName::EncryptedTagName(name1.clone()),
            TargetValue::Encrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"{}":"{}"}}"#, base64::encode(&name1), base64::encode(&value1));

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_neq_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Neq(
            TagName::PlainTagName(name1.clone()),
            TargetValue::Unencrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$neq":"{}"}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_neq_encrypted_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_vector(10);

        let query = Operator::Neq(
            TagName::EncryptedTagName(name1.clone()),
            TargetValue::Encrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"{}":{{"$neq":"{}"}}}}"#, base64::encode(&name1), base64::encode(&value1));

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_gt_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Gt(
            TagName::PlainTagName(name1.clone()),
            TargetValue::Unencrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$gt":"{}"}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_gte_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Gte(
            TagName::PlainTagName(name1.clone()),
            TargetValue::Unencrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$gte":"{}"}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_lt_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Lt(
            TagName::PlainTagName(name1.clone()),
            TargetValue::Unencrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$lt":"{}"}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_lte_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Lte(
            TagName::PlainTagName(name1.clone()),
            TargetValue::Unencrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$lte":"{}"}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_like_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Like(
            TagName::PlainTagName(name1.clone()),
            TargetValue::Unencrypted(value1.clone())
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$like":"{}"}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_in_plaintext_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::In(
            TagName::PlainTagName(name1.clone()),
            vec![TargetValue::Unencrypted(value1.clone())]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$in":["{}"]}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_in_plaintexts_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let value2 = _random_string(10);
        let value3 = _random_string(10);

        let query = Operator::In(
            TagName::PlainTagName(name1.clone()),
            vec![
                TargetValue::Unencrypted(value1.clone()),
                TargetValue::Unencrypted(value2.clone()),
                TargetValue::Unencrypted(value3.clone()),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"~{}":{{"$in":["{}","{}","{}"]}}}}"#, base64::encode(&name1), value1, value2, value3);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_in_encrypted_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_vector(10);

        let query = Operator::In(
            TagName::EncryptedTagName(name1.clone()),
            vec![TargetValue::Encrypted(value1.clone())]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"{}":{{"$in":["{}"]}}}}"#, base64::encode(&name1), base64::encode(&value1));

        assert_eq!(json, expected);
    }

    #[test]
    fn test_simple_operator_in_encrypted_values_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_vector(10);
        let value2 = _random_vector(10);
        let value3 = _random_vector(10);

        let query = Operator::In(
            TagName::EncryptedTagName(name1.clone()),
            vec![
                TargetValue::Encrypted(value1.clone()),
                TargetValue::Encrypted(value2.clone()),
                TargetValue::Encrypted(value3.clone()),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"{}":{{"$in":["{}","{}","{}"]}}}}"#,
                               base64::encode(&name1),
                               base64::encode(&value1),
                               base64::encode(&value2),
                               base64::encode(&value3)
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":"{}"}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_neq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$neq":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_gt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$gt":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_gte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$gte":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_lt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$lt":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_lte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$lte":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_like_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$like":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_in_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.clone()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$in":["{}"]}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_one_not_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.clone()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"$not":{{"~{}":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":"{}"}},{{"~{}":"{}"}},{{"~{}":"{}"}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3,
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_neq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_gt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_gte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_lt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_lte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_like_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_in_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.clone()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name2.clone()),
                    vec![TargetValue::Unencrypted(value2.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name3.clone()),
                    vec![TargetValue::Unencrypted(value3.clone())]
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_not_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.clone()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name2.clone()),
                            TargetValue::Unencrypted(value2.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name3.clone()),
                            TargetValue::Unencrypted(value3.clone())
                        )
                    )
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3,
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_with_multiple_mixed_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);
        let name4 = _random_vector(10);
        let value4 = _random_string(10);
        let name5 = _random_vector(10);
        let value5 = _random_string(10);
        let name6 = _random_vector(10);
        let value6 = _random_string(10);
        let name7 = _random_vector(10);
        let value7 = _random_string(10);
        let name8 = _random_vector(10);
        let value8a = _random_string(10);
        let value8b = _random_string(10);
        let name9 = _random_vector(10);
        let value9 = _random_string(10);

        let query = Operator::And(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name4.clone()),
                    TargetValue::Unencrypted(value4.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name5.clone()),
                    TargetValue::Unencrypted(value5.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name6.clone()),
                    TargetValue::Unencrypted(value6.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name7.clone()),
                    TargetValue::Unencrypted(value7.clone())
                ),
                Operator::In(
                    TagName::PlainTagName(name8.clone()),
                    vec![
                        TargetValue::Unencrypted(value8a.clone()),
                        TargetValue::Unencrypted(value8b.clone())
                    ]
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name9.clone()),
                            TargetValue::Unencrypted(value9.clone())
                        )
                    )
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$and":[{{"~{}":"{}"}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$in":["{}","{}"]}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3,
                               base64::encode(&name4), value4,
                               base64::encode(&name5), value5,
                               base64::encode(&name6), value6,
                               base64::encode(&name7), value7,
                               base64::encode(&name8), value8a, value8b,
                               base64::encode(&name9), value9,
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":"{}"}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_neq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$neq":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_gt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$gt":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_gte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$gte":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_lt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$lt":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_lte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$lte":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_like_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$like":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_in_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.clone()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$in":["{}"]}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_one_not_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.clone()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"$not":{{"~{}":"{}"}}}}]}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Eq(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":"{}"}},{{"~{}":"{}"}},{{"~{}":"{}"}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3,
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_neq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Neq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$neq":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_gt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Gt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_gte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Gte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_lt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Lt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_lte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Lte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_like_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Like(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$like":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_in_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::In(
                    TagName::PlainTagName(name1.clone()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name2.clone()),
                    vec![TargetValue::Unencrypted(value2.clone())]
                ),
                Operator::In(
                    TagName::PlainTagName(name3.clone()),
                    vec![TargetValue::Unencrypted(value3.clone())]
                )
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}},{{"~{}":{{"$in":["{}"]}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_not_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name1.clone()),
                            TargetValue::Unencrypted(value1.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name2.clone()),
                            TargetValue::Unencrypted(value2.clone())
                        )
                    )
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name3.clone()),
                            TargetValue::Unencrypted(value3.clone())
                        )
                    )
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3,
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_or_with_multiple_mixed_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);
        let name4 = _random_vector(10);
        let value4 = _random_string(10);
        let name5 = _random_vector(10);
        let value5 = _random_string(10);
        let name6 = _random_vector(10);
        let value6 = _random_string(10);
        let name7 = _random_vector(10);
        let value7 = _random_string(10);
        let name8 = _random_vector(10);
        let value8a = _random_string(10);
        let value8b = _random_string(10);
        let name9 = _random_vector(10);
        let value9 = _random_string(10);

        let query = Operator::Or(
            vec![
                Operator::Eq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                ),
                Operator::Neq(
                    TagName::PlainTagName(name2.clone()),
                    TargetValue::Unencrypted(value2.clone())
                ),
                Operator::Gt(
                    TagName::PlainTagName(name3.clone()),
                    TargetValue::Unencrypted(value3.clone())
                ),
                Operator::Gte(
                    TagName::PlainTagName(name4.clone()),
                    TargetValue::Unencrypted(value4.clone())
                ),
                Operator::Lt(
                    TagName::PlainTagName(name5.clone()),
                    TargetValue::Unencrypted(value5.clone())
                ),
                Operator::Lte(
                    TagName::PlainTagName(name6.clone()),
                    TargetValue::Unencrypted(value6.clone())
                ),
                Operator::Like(
                    TagName::PlainTagName(name7.clone()),
                    TargetValue::Unencrypted(value7.clone())
                ),
                Operator::In(
                    TagName::PlainTagName(name8.clone()),
                    vec![
                        TargetValue::Unencrypted(value8a.clone()),
                        TargetValue::Unencrypted(value8b.clone())
                    ]
                ),
                Operator::Not(
                    Box::new(
                        Operator::Eq(
                            TagName::PlainTagName(name9.clone()),
                            TargetValue::Unencrypted(value9.clone())
                        )
                    )
                ),
            ]
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$or":[{{"~{}":"{}"}},{{"~{}":{{"$neq":"{}"}}}},{{"~{}":{{"$gt":"{}"}}}},{{"~{}":{{"$gte":"{}"}}}},{{"~{}":{{"$lt":"{}"}}}},{{"~{}":{{"$lte":"{}"}}}},{{"~{}":{{"$like":"{}"}}}},{{"~{}":{{"$in":["{}","{}"]}}}},{{"$not":{{"~{}":"{}"}}}}]}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3,
                               base64::encode(&name4), value4,
                               base64::encode(&name5), value5,
                               base64::encode(&name6), value6,
                               base64::encode(&name7), value7,
                               base64::encode(&name8), value8a, value8b,
                               base64::encode(&name9), value9,
        );

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_eq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::Eq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":"{}"}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_neq_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::Neq(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":{{"$neq":"{}"}}}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_gt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::Gt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":{{"$gt":"{}"}}}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_gte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::Gte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":{{"$gte":"{}"}}}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_lt_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::Lt(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":{{"$lt":"{}"}}}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_lte_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::Lte(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":{{"$lte":"{}"}}}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_like_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::Like(
                    TagName::PlainTagName(name1.clone()),
                    TargetValue::Unencrypted(value1.clone())
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":{{"$like":"{}"}}}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_not_with_one_in_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::In(
                    TagName::PlainTagName(name1.clone()),
                    vec![TargetValue::Unencrypted(value1.clone())]
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"~{}":{{"$in":["{}"]}}}}}}"#, base64::encode(&name1), value1);

        assert_eq!(json, expected);
    }

    #[test]
    fn test_and_or_not_complex_case_to_string() {
        let name1 = _random_vector(10);
        let value1 = _random_string(10);
        let name2 = _random_vector(10);
        let value2 = _random_string(10);
        let name3 = _random_vector(10);
        let value3 = _random_string(10);
        let name4 = _random_vector(10);
        let value4 = _random_string(10);
        let name5 = _random_vector(10);
        let value5 = _random_string(10);
        let name6 = _random_vector(10);
        let value6 = _random_string(10);
        let name7 = _random_vector(10);
        let value7 = _random_string(10);
        let name8 = _random_vector(10);
        let value8 = _random_string(10);

        let query = Operator::Not(
            Box::new(
                Operator::And(
                    vec![
                        Operator::Eq(
                            TagName::PlainTagName(name1.clone()),
                            TargetValue::Unencrypted(value1.clone())
                        ),
                        Operator::Or(
                            vec![
                                Operator::Gt(
                                    TagName::PlainTagName(name2.clone()),
                                    TargetValue::Unencrypted(value2.clone())
                                ),
                                Operator::Not(
                                    Box::new(
                                        Operator::Lte(
                                            TagName::PlainTagName(name3.clone()),
                                            TargetValue::Unencrypted(value3.clone())
                                        )
                                    )
                                ),
                                Operator::And(
                                    vec![
                                        Operator::Lt(
                                            TagName::PlainTagName(name4.clone()),
                                            TargetValue::Unencrypted(value4.clone())
                                        ),
                                        Operator::Not(
                                            Box::new(
                                                Operator::Gte(
                                                    TagName::PlainTagName(name5.clone()),
                                                    TargetValue::Unencrypted(value5.clone())
                                                )
                                            )
                                        ),
                                    ]
                                )
                            ]
                        ),
                        Operator::Not(
                            Box::new(
                                Operator::Like(
                                    TagName::PlainTagName(name6.clone()),
                                    TargetValue::Unencrypted(value6.clone())
                                )
                            )
                        ),
                        Operator::And(
                            vec![
                                Operator::Eq(
                                    TagName::PlainTagName(name7.clone()),
                                    TargetValue::Unencrypted(value7.clone())
                                ),
                                Operator::Not(
                                    Box::new(
                                        Operator::Neq(
                                            TagName::PlainTagName(name8.clone()),
                                            TargetValue::Unencrypted(value8.clone())
                                        )
                                    )
                                ),
                            ]
                        )
                    ]
                )
            )
        );

        let json = query.to_string();

        let expected = format!(r#"{{"$not":{{"$and":[{{"~{}":"{}"}},{{"$or":[{{"~{}":{{"$gt":"{}"}}}},{{"$not":{{"~{}":{{"$lte":"{}"}}}}}},{{"$and":[{{"~{}":{{"$lt":"{}"}}}},{{"$not":{{"~{}":{{"$gte":"{}"}}}}}}]}}]}},{{"$not":{{"~{}":{{"$like":"{}"}}}}}},{{"$and":[{{"~{}":"{}"}},{{"$not":{{"~{}":{{"$neq":"{}"}}}}}}]}}]}}}}"#,
                               base64::encode(&name1), value1,
                               base64::encode(&name2), value2,
                               base64::encode(&name3), value3,
                               base64::encode(&name4), value4,
                               base64::encode(&name5), value5,
                               base64::encode(&name6), value6,
                               base64::encode(&name7), value7,
                               base64::encode(&name8), value8,
        );

        assert_eq!(json, expected);

    }
}

