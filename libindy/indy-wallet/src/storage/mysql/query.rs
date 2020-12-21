use indy_api_types::errors::prelude::*;
use indy_utils::crypto::base64;
use serde_json::Value;

use crate::{
    language::{Operator, TagName, TargetValue},
    SearchOptions,
};

pub fn wql_to_sql(
    wallet_id: i64,
    type_: &[u8],
    wql: &Operator,
    options: &SearchOptions,
) -> IndyResult<(String, Vec<Value>)> {
    let mut arguments: Vec<Value> = Vec::new();

    let query_condition = match operator_to_sql(wql, &mut arguments) {
        Ok(query_condition) => query_condition,
        Err(err) => return Err(err),
    };

    let query_string = format!(
        "SELECT {}, name, {}, {} FROM items WHERE {} type = ? AND wallet_id = ?",
        if options.retrieve_type {
            "type"
        } else {
            "NULL"
        },
        if options.retrieve_value {
            "value"
        } else {
            "NULL"
        },
        if options.retrieve_tags {
            "tags"
        } else {
            "NULL"
        },
        if !query_condition.is_empty() {
            query_condition + " AND"
        } else {
            "".to_string()
        }
    );

    arguments.push(base64::encode(type_).into());
    arguments.push(wallet_id.into());

    Ok((query_string, arguments))
}

pub fn wql_to_sql_count(
    wallet_id: i64,
    type_: &[u8],
    wql: &Operator,
) -> IndyResult<(String, Vec<Value>)> {
    let mut arguments: Vec<Value> = Vec::new();

    let query_condition = match operator_to_sql(wql, &mut arguments) {
        Ok(query_condition) => query_condition,
        Err(err) => return Err(err),
    };

    let query_string = format!(
        "SELECT count(*) FROM items i WHERE {} i.type = ? AND i.wallet_id = ?",
        if !query_condition.is_empty() {
            query_condition + " AND"
        } else {
            "".to_string()
        }
    );

    arguments.push(base64::encode(type_).into());
    arguments.push(wallet_id.into());

    Ok((query_string, arguments))
}

fn operator_to_sql(op: &Operator, arguments: &mut Vec<Value>) -> IndyResult<String> {
    match *op {
        Operator::Eq(ref tag_name, ref target_value) => {
            Ok(eq_to_sql(tag_name, target_value, arguments))
        }
        Operator::Neq(ref tag_name, ref target_value) => {
            Ok(neq_to_sql(tag_name, target_value, arguments))
        }
        Operator::Gt(ref tag_name, ref target_value) => {
            gt_to_sql(tag_name, target_value, arguments)
        }
        Operator::Gte(ref tag_name, ref target_value) => {
            gte_to_sql(tag_name, target_value, arguments)
        }
        Operator::Lt(ref tag_name, ref target_value) => {
            lt_to_sql(tag_name, target_value, arguments)
        }
        Operator::Lte(ref tag_name, ref target_value) => {
            lte_to_sql(tag_name, target_value, arguments)
        }
        Operator::Like(ref tag_name, ref target_value) => {
            like_to_sql(tag_name, target_value, arguments)
        }
        Operator::In(ref tag_name, ref target_values) => {
            Ok(in_to_sql(tag_name, target_values, arguments))
        }
        Operator::And(ref suboperators) => and_to_sql(suboperators, arguments),
        Operator::Or(ref suboperators) => or_to_sql(suboperators, arguments),
        Operator::Not(ref suboperator) => not_to_sql(suboperator, arguments),
    }
}

fn eq_to_sql(tag_name: &TagName, tag_value: &TargetValue, arguments: &mut Vec<Value>) -> String {
    let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());

    arguments.push(tag_value.to_plain().into());
    format!("(JSON_UNQUOTE(JSON_EXTRACT(tags, {})) = ?)", tag_path)
}

fn neq_to_sql(tag_name: &TagName, tag_value: &TargetValue, arguments: &mut Vec<Value>) -> String {
    let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());

    arguments.push(tag_value.to_plain().into());
    format!("(JSON_UNQUOTE(JSON_EXTRACT(tags, {})) != ?)", tag_path)
}

fn gt_to_sql(
    tag_name: &TagName,
    tag_value: &TargetValue,
    arguments: &mut Vec<Value>,
) -> IndyResult<String> {
    match (tag_name, tag_value) {
        (&TagName::PlainTagName(_), &TargetValue::Unencrypted(_)) => {
            let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());
            arguments.push(tag_value.to_plain().into());

            Ok(format!(
                "(JSON_UNQUOTE(JSON_EXTRACT(tags, {})) > ?)",
                tag_path
            ))
        }
        _ => Err(err_msg(
            IndyErrorKind::WalletQueryError,
            "Invalid combination of tag name and value for $gt operator",
        )),
    }
}

fn gte_to_sql(
    tag_name: &TagName,
    tag_value: &TargetValue,
    arguments: &mut Vec<Value>,
) -> IndyResult<String> {
    match (tag_name, tag_value) {
        (&TagName::PlainTagName(_), &TargetValue::Unencrypted(_)) => {
            let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());
            arguments.push(tag_value.to_plain().into());

            Ok(format!(
                "(JSON_UNQUOTE(JSON_EXTRACT(tags, {})) >= ?)",
                tag_path
            ))
        }
        _ => Err(err_msg(
            IndyErrorKind::WalletQueryError,
            "Invalid combination of tag name and value for $gt operator",
        )),
    }
}

fn lt_to_sql(
    tag_name: &TagName,
    tag_value: &TargetValue,
    arguments: &mut Vec<Value>,
) -> IndyResult<String> {
    match (tag_name, tag_value) {
        (&TagName::PlainTagName(_), &TargetValue::Unencrypted(_)) => {
            let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());
            arguments.push(tag_value.to_plain().into());

            Ok(format!(
                "(JSON_UNQUOTE(JSON_EXTRACT(tags, {})) < ?)",
                tag_path
            ))
        }
        _ => Err(err_msg(
            IndyErrorKind::WalletQueryError,
            "Invalid combination of tag name and value for $lt operator",
        )),
    }
}

fn lte_to_sql(
    tag_name: &TagName,
    tag_value: &TargetValue,
    arguments: &mut Vec<Value>,
) -> IndyResult<String> {
    match (tag_name, tag_value) {
        (&TagName::PlainTagName(_), &TargetValue::Unencrypted(_)) => {
            let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());
            arguments.push(tag_value.to_plain().into());

            Ok(format!(
                "(JSON_UNQUOTE(JSON_EXTRACT(tags, {})) <= ?)",
                tag_path
            ))
        }
        _ => Err(err_msg(
            IndyErrorKind::WalletQueryError,
            "Invalid combination of tag name and value for $lt operator",
        )),
    }
}

fn like_to_sql(
    tag_name: &TagName,
    tag_value: &TargetValue,
    arguments: &mut Vec<Value>,
) -> IndyResult<String> {
    match (tag_name, tag_value) {
        (&TagName::PlainTagName(_), &TargetValue::Unencrypted(_)) => {
            let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());
            arguments.push(tag_value.to_plain().into());

            Ok(format!(
                "(JSON_UNQUOTE(JSON_EXTRACT(tags, {})) LIKE ?)",
                tag_path
            ))
        }
        _ => Err(err_msg(
            IndyErrorKind::WalletQueryError,
            "Invalid combination of tag name and value for $lt operator",
        )),
    }
}

fn in_to_sql(
    tag_name: &TagName,
    tag_values: &Vec<TargetValue>,
    arguments: &mut Vec<Value>,
) -> String {
    let tag_path = format!(r#"'$."{}"'"#, tag_name.to_plain());
    let mut in_string = format!("JSON_UNQUOTE(JSON_EXTRACT(tags, {})) IN (", tag_path);

    for (index, tag_value) in tag_values.iter().enumerate() {
        in_string.push_str("?");
        if index < tag_values.len() - 1 {
            in_string.push(',');
        } else {
            in_string.push(')');
        }

        arguments.push(tag_value.to_plain().into());
    }

    in_string
}

fn and_to_sql(suboperators: &[Operator], arguments: &mut Vec<Value>) -> IndyResult<String> {
    join_operators(suboperators, " AND ", arguments)
}

fn or_to_sql(suboperators: &[Operator], arguments: &mut Vec<Value>) -> IndyResult<String> {
    join_operators(suboperators, " OR ", arguments)
}

fn not_to_sql(suboperator: &Operator, arguments: &mut Vec<Value>) -> IndyResult<String> {
    let suboperator_string = operator_to_sql(suboperator, arguments)?;
    Ok("NOT (".to_string() + &suboperator_string + ")")
}

fn join_operators(
    operators: &[Operator],
    join_str: &str,
    arguments: &mut Vec<Value>,
) -> IndyResult<String> {
    let mut s = String::new();

    if !operators.is_empty() {
        s.push('(');
        for (index, operator) in operators.iter().enumerate() {
            let operator_string = operator_to_sql(operator, arguments)?;

            s.push_str(&operator_string);

            if index < operators.len() - 1 {
                s.push_str(join_str);
            }
        }

        s.push(')');
    }

    Ok(s)
}

// FIXME: It is quite smilar for to_string method of tag and value, but for some reason
// to_string uses "". It is added to avoid potential damage as i have no time
// for investigation.
trait ToPlain {
    fn to_plain(&self) -> String;
}

impl ToPlain for TagName {
    fn to_plain(&self) -> String {
        match *self {
            TagName::EncryptedTagName(ref v) => base64::encode(v),
            TagName::PlainTagName(ref v) => format!("~{}", base64::encode(v)),
        }
    }
}

impl ToPlain for TargetValue {
    fn to_plain(&self) -> String {
        match *self {
            TargetValue::Unencrypted(ref s) => s.to_owned(),
            TargetValue::Encrypted(ref v) => base64::encode(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_and() {
        let condition_1 = Operator::And(vec![
            Operator::Eq(
                TagName::EncryptedTagName(vec![1, 2, 3]),
                TargetValue::Encrypted(vec![4, 5, 6]),
            ),
            Operator::Eq(
                TagName::PlainTagName(vec![7, 8, 9]),
                TargetValue::Unencrypted("spam".to_string()),
            ),
        ]);

        let condition_2 = Operator::And(vec![
            Operator::Eq(
                TagName::EncryptedTagName(vec![10, 11, 12]),
                TargetValue::Encrypted(vec![13, 14, 15]),
            ),
            Operator::Not(Box::new(Operator::Eq(
                TagName::PlainTagName(vec![16, 17, 18]),
                TargetValue::Unencrypted("eggs".to_string()),
            ))),
        ]);

        let query = Operator::Or(vec![condition_1, condition_2]);
        let class = [100, 100, 100];
        let options = SearchOptions::default();

        let (_query, _arguments) = wql_to_sql(1_i64, &class, &query, &options).unwrap();
    }
}
