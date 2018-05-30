use std::string;

use serde_json;

use errors::wallet::WalletQueryError;
use base64;

#[derive(Debug)]
pub enum TagName {
    EncryptedTagName(Vec<u8>),
    PlainTagName(Vec<u8>),
}


impl TagName {
    fn from(s: String) -> Result<TagName, WalletQueryError> {
        let mut v = s.into_bytes();
        match v.first() {
            Some(&126) => match v.get(1) {
                Some(_) => {
                    v.remove(0);
                    Ok(TagName::PlainTagName(v))
                },
                None => Err(WalletQueryError::StructureErr("Tag name must not be empty".to_string())),
            },
            Some(_) => Ok(TagName::EncryptedTagName(v)),
            None => Err(WalletQueryError::StructureErr("Tag name must not be empty".to_string()))
        }
    }
}

impl string::ToString for TagName {
    fn to_string(&self) -> String {
        match *self {
            TagName::EncryptedTagName(ref v) => base64::encode(v),
            TagName::PlainTagName(ref v) => format!("~{}", base64::encode(v))
        }
    }
}


#[derive(Debug)]
pub enum TargetValue {
    Unencrypted(String),
    Encrypted(Vec<u8>),
}

impl From<String> for TargetValue {
    fn from(s: String) -> TargetValue {
        TargetValue::Unencrypted(s)
    }
}

impl string::ToString for TargetValue {
    fn to_string(&self) -> String {
        match *self {
            TargetValue::Unencrypted(ref s) => s.clone(),
            TargetValue::Encrypted(ref v) => base64::encode(v),
        }
    }
}



#[derive(Debug)]
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
    Regex(TagName, TargetValue),
    Like(TagName, TargetValue),
    In(TagName, Vec<TargetValue>),
}


impl Operator {
    pub fn transform(self, f: &Fn(Operator) -> Result<Operator, WalletQueryError>) -> Result<Operator, WalletQueryError> {
        match self {
            Operator::And(operators) => {
                let mut transformed = Vec::new();

                for operator in operators {
                    let transformed_operator = Operator::transform(operator, f)?;
                    transformed.push(transformed_operator);
                }

                Ok(Operator::And(transformed))
            }
            Operator::Or(operators) => {
                let mut transformed = Vec::new();

                for operator in operators {
                    let transformed_operator = Operator::transform(operator, f)?;
                    transformed.push(transformed_operator);
                }

                Ok(Operator::Or(transformed))
            }
            Operator::Not(boxed_operator) => {
                Ok(Operator::Not(Box::new(Operator::transform(*boxed_operator, f)?)))
            }
            _ => Ok(f(self)?)
        }
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


fn join_operator_strings(operators: &Vec<Operator>) -> String {
    operators.iter()
             .map(|o: &Operator| -> String { o.to_string() })
             .collect::<Vec<String>>()
             .join(",")
}


impl string::ToString for Operator {
    fn to_string(&self) -> String {
        match *self {
            Operator::Eq(ref tag_name, ref tag_value) => format!("\"{}\": \"{}\"", tag_name.to_string(), tag_value.to_string()),
            Operator::Neq(ref tag_name, ref tag_value) => format!("\"{}\": {{\"$neq\": \"{}\"}}", tag_name.to_string(), tag_value.to_string()),
            Operator::Gt(ref tag_name, ref tag_value) => format!("\"{}\": {{\"$gt\": \"{}\"}}", tag_name.to_string(), tag_value.to_string()),
            Operator::Gte(ref tag_name, ref tag_value) => format!("\"{}\": {{\"$gte\": \"{}\"}}", tag_name.to_string(), tag_value.to_string()),
            Operator::Lt(ref tag_name, ref tag_value) => format!("\"{}\": {{\"$lt\": \"{}\"}}", tag_name.to_string(), tag_value.to_string()),
            Operator::Lte(ref tag_name, ref tag_value) => format!("\"{}\": {{\"$lte\": \"{}\"}}", tag_name.to_string(), tag_value.to_string()),
            Operator::Like(ref tag_name, ref tag_value) => format!("\"{}\": {{\"$like\": \"{}\"}}", tag_name.to_string(), tag_value.to_string()),
            Operator::Regex(ref tag_name, ref tag_value) => format!("\"{}\": {{\"$regex\": \"{}\"}}", tag_name.to_string(), tag_value.to_string()),
            Operator::Not(ref stmt) => format!("\"$not\": {{{}}}", stmt.to_string()),
            Operator::And(ref operators) => format!("{{{}}}", join_operator_strings(operators)),
            Operator::Or(ref operators) => format!("\"$or\": [{}])", join_operator_strings(operators)),
            Operator::In(ref tag_name, ref tag_values) => {
                let strings = tag_values.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(", ");
                format!("\"{}\": {{\"$in\": [{}]}}", tag_name.to_string(), strings)
            },
        }
    }
}


pub fn parse_from_json(json: &str) -> Result<Operator, WalletQueryError> {
    if let serde_json::Value::Object(map) = serde_json::from_str(json)? {
        parse(map)
    } else {
        Err(WalletQueryError::StructureErr("Query must be JSON object".to_string()))
    }
}


fn parse(map: serde_json::Map<String, serde_json::Value>) -> Result<Operator, WalletQueryError> {
    let mut operators: Vec<Operator> = Vec::new();

    for (key, value) in map.into_iter() {
        let suboperator = parse_operator(key, value)?;
        operators.push(suboperator);
    }

    let top_operator = Operator::And(operators);
    Ok(top_operator.optimise())
}


fn parse_operator(key: String, value: serde_json::Value) -> Result<Operator, WalletQueryError> {
    match (&*key, value) {
        ("$or", serde_json::Value::Array(values)) => {
            let mut operators: Vec<Operator> = Vec::new();

            for value in values.into_iter() {
                if let serde_json::Value::Object(map) = value {
                    let suboperator = parse(map)?;
                    operators.push(suboperator);
                } else {
                    return Err(WalletQueryError::StructureErr("$or must be array of JSON objects".to_string()));
                }
            }

            Ok(Operator::Or(operators))
        },
        ("$or", _) => Err(WalletQueryError::StructureErr("$or must be array of JSON objects".to_string())),
        ("$not", serde_json::Value::Object(map)) => {
            let operator = parse(map)?;
            Ok(Operator::Not(Box::new(operator)))
        },
        ("$not", _) => Err(WalletQueryError::StructureErr("$not must be JSON object".to_string())),
        (_, serde_json::Value::String(value)) => Ok(Operator::Eq(TagName::from(key)?, TargetValue::from(value))),
        (_, serde_json::Value::Object(map)) => {
            if map.len() == 1 {
                let (operator_name, value) = map.into_iter().next().unwrap();
                parse_single_operator(operator_name, key, value)
            } else {
                Err(WalletQueryError::StructureErr(format!("{} value must be JSON object of length 1", key)))
            }
        },
        (_, _) => Err(WalletQueryError::StructureErr(format!("Unsupported value for key: {}", key)))
    }
}


fn parse_single_operator(operator_name: String, key: String, value: serde_json::Value) -> Result<Operator, WalletQueryError> {
    match (&*operator_name, value) {
        ("$neq", serde_json::Value::String(s)) => Ok(Operator::Neq(TagName::from(key)?, TargetValue::from(s))),
        ("$neq", _) => Err(WalletQueryError::ValueErr("$neq must be used with string".to_string())),
        ("$gt", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Gt(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(WalletQueryError::StructureErr("$gt must be used only for nonencrypted tag".to_string()))
            }
        },
        ("$gt", _) => Err(WalletQueryError::ValueErr("$gt must be used with string".to_string())),
        ("$gte", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Gte(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(WalletQueryError::StructureErr("$gte must be used only for nonencrypted tag".to_string()))
            }
        },
        ("$gte", _) => Err(WalletQueryError::ValueErr("$gte must be used with string".to_string())),
        ("$lt", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Lt(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(WalletQueryError::StructureErr("$lt must be used only for nonencrypted tag".to_string()))
            }
        },
        ("$lt", _) => Err(WalletQueryError::ValueErr("$lt must be used with string".to_string())),
        ("$lte", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Lte(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(WalletQueryError::StructureErr("$lte must be used only for nonencrypted tag".to_string()))
            }
        },
        ("$lte", _) => Err(WalletQueryError::ValueErr("$lte must be used with string".to_string())),
        ("$like", serde_json::Value::String(s)) => {
            let target_name = TagName::from(key)?;
            match target_name {
                TagName::PlainTagName(_) => Ok(Operator::Like(target_name, TargetValue::from(s))),
                TagName::EncryptedTagName(_) => Err(WalletQueryError::StructureErr("$like must be used only for nonencrypted tag".to_string()))
            }
        },
        ("$like", _) => Err(WalletQueryError::ValueErr("$like must be used with string".to_string())),
        ("$in", serde_json::Value::Array(values)) => {
            let mut target_values: Vec<TargetValue> = Vec::new();

            for v in values.into_iter() {
                if let serde_json::Value::String(s) = v {
                    target_values.push(TargetValue::from(s));
                } else {
                    return Err(WalletQueryError::ValueErr("$in must be used with array of strings".to_string()));
                }
            }

            Ok(Operator::In(TagName::from(key)?, target_values))
        },
        ("$in", _) => Err(WalletQueryError::ValueErr("$in must be used with array of strings".to_string())),
        (_, _) => Err(WalletQueryError::ValueErr(format!("Bad operator: {}", operator_name)))
    }
}