use indy_api_types::errors::prelude::*;
use rusqlite::types::ToSql;
use crate::language::{Operator, TagName, TargetValue};


// Translates Wallet Query Language to SQL
// WQL input is provided as a reference to a top level Operator
// Result is a tuple of query string and query arguments
pub fn wql_to_sql<'a>(wallet_id: &'a Vec<u8>, op: &'a Operator, _options: Option<&str>) -> Result<(String, Vec<&'a dyn ToSql>), IndyError> {
    let mut arguments: Vec<&dyn ToSql> = Vec::new();
    let clause_string = operator_to_sql(op, &mut arguments, wallet_id)?;
    const BASE: &str = "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i";
    if !clause_string.is_empty() {
        let mut query_string = BASE.to_string();
        query_string.push_str(" INNER JOIN ");
        query_string.push_str(&clause_string);
        // arguments.push(&wallet_id.clone());
        Ok((query_string, arguments))
    } else {
        Ok((BASE.to_string(), arguments))
    }

}


pub fn wql_to_sql_count<'a>(class: &'a Vec<u8>, op: &'a Operator) -> Result<(String, Vec<&'a dyn ToSql>), IndyError> {
    let mut arguments: Vec<&dyn ToSql> = Vec::new();
    arguments.push(class);
    let clause_string = operator_to_sql(op, &mut arguments, class)?;
    let mut query_string = "SELECT count(*) FROM items as i WHERE i.type = ?".to_string();
    if !clause_string.is_empty() {
        query_string.push_str(" AND ");
        query_string.push_str(&clause_string);
    }
    Ok((query_string, arguments))
}


fn operator_to_sql<'a>(op: &'a Operator, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a Vec<u8>) -> IndyResult<String> {
    match *op {
        Operator::Eq(ref tag_name, ref target_value) => eq_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Neq(ref tag_name, ref target_value) => neq_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Gt(ref tag_name, ref target_value) => gt_to_sql(tag_name, target_value, arguments),
        Operator::Gte(ref tag_name, ref target_value) => gte_to_sql(tag_name, target_value, arguments),
        Operator::Lt(ref tag_name, ref target_value) => lt_to_sql(tag_name, target_value, arguments),
        Operator::Lte(ref tag_name, ref target_value) => lte_to_sql(tag_name, target_value, arguments),
        Operator::Like(ref tag_name, ref target_value) => like_to_sql(tag_name, target_value, arguments),
        Operator::In(ref tag_name, ref target_values) => in_to_sql(tag_name, target_values, arguments),
        Operator::And(ref suboperators) => and_to_sql(suboperators, arguments, wallet_id),
        Operator::Or(ref suboperators) => or_to_sql(suboperators, arguments, wallet_id),
        Operator::Not(ref suboperator) => not_to_sql(suboperator, arguments, wallet_id),
    }
}

fn eq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a Vec<u8>) -> IndyResult<String> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(wallet_id);
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("tags_plaintext as ta ON ta.item_id = i.id AND ta.wallet_id = ? AND ta.name = ? AND ta.value = ?".to_string())
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(wallet_id);
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("tags_encrypted as ta ON ta.item_id = i.id AND ta.wallet_id = ? AND ta.name = ? AND ta.value = ?".to_string())
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Invalid combination of tag name and value for equality operator"))
    }
}


fn neq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a Vec<u8>) -> IndyResult<String> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(wallet_id);
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("(i.id in (SELECT item_id FROM tags_plaintext WHERE name = ? AND value != ?))".to_string())
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(wallet_id);
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("(i.id in (SELECT item_id FROM tags_encrypted WHERE name = ? AND value != ?))".to_string())
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Invalid combination of tag name and value for inequality operator"))
    }
}


fn gt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>) -> IndyResult<String> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("(i.id in (SELECT item_id FROM tags_plaintext WHERE name = ? AND value > ?))".to_string())
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Invalid combination of tag name and value for $gt operator"))
    }
}


fn gte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>) -> IndyResult<String> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("(i.id in (SELECT item_id FROM tags_plaintext WHERE name = ? AND value >= ?))".to_string())
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Invalid combination of tag name and value for $gte operator"))
    }
}


fn lt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>) -> IndyResult<String> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("(i.id in (SELECT item_id FROM tags_plaintext WHERE name = ? AND value < ?))".to_string())
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Invalid combination of tag name and value for $lt operator"))
    }
}


fn lte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>) -> IndyResult<String> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("(i.id in (SELECT item_id FROM tags_plaintext WHERE name = ? AND value <= ?))".to_string())
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Invalid combination of tag name and value for $lte operator"))
    }
}


fn like_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>) -> IndyResult<String> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok("(i.id in (SELECT item_id FROM tags_plaintext WHERE name = ? AND value LIKE ?))".to_string())
        },
        _ => Err(err_msg(IndyErrorKind::WalletQueryError, "Invalid combination of tag name and value for $like operator"))
    }
}


fn in_to_sql<'a>(name: &'a TagName, values: &'a Vec<TargetValue>, arguments: &mut Vec<&'a dyn ToSql>) -> IndyResult<String> {
    let mut in_string = String::new();
    match *name {
        TagName::PlainTagName(ref queried_name) => {
            in_string.push_str("(i.id in (SELECT item_id FROM tags_plaintext WHERE name = ? AND value IN (");
            arguments.push(queried_name);

            for (index, value) in values.iter().enumerate() {
                if let TargetValue::Unencrypted(ref target) = *value {
                    in_string.push_str("?");
                    arguments.push(target);
                    if index < values.len() - 1 {
                        in_string.push(',');
                    }
                } else {
                    return Err(err_msg(IndyErrorKind::WalletQueryError, "Encrypted tag value in $in for nonencrypted tag name"))
                }
            }

            Ok(in_string + ")))")
        },
        TagName::EncryptedTagName(ref queried_name) => {
            in_string.push_str("(i.id in (SELECT item_id FROM tags_encrypted WHERE name = ? AND value IN (");
            arguments.push(queried_name);
            let index_before_last = values.len() - 2;

            for (index, value) in values.iter().enumerate() {
                if let TargetValue::Encrypted(ref target) = *value {
                    in_string.push_str("?");
                    arguments.push(target);
                    if index <= index_before_last {
                        in_string.push(',');
                    }
                } else {
                    return Err(err_msg(IndyErrorKind::WalletQueryError, "Unencrypted tag value in $in for encrypted tag name"))
                }
            }

            Ok(in_string + ")))")
        },
    }
}


fn and_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a Vec<u8>) -> IndyResult<String> {
    join_operators(suboperators, " INNER JOIN ", arguments, wallet_id)
}

fn or_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a Vec<u8>) -> IndyResult<String> {
    join_operators(suboperators, " OR ", arguments, wallet_id)
}


fn not_to_sql<'a>(suboperator: &'a Operator, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a Vec<u8>) -> IndyResult<String> {
    let suboperator_string = operator_to_sql(suboperator, arguments, wallet_id)?;
    Ok("NOT (".to_string() + &suboperator_string + ")")
}


fn join_operators<'a>(operators: &'a [Operator], join_str: &str, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a Vec<u8>) -> IndyResult<String> {
    let mut s = String::new();
    if !operators.is_empty() {
        for (index, operator) in operators.iter().enumerate() {
            let operator_string = operator_to_sql(operator, arguments, wallet_id)?;
            s.push_str(&operator_string);
            if index < operators.len() - 1 {
                s.push_str(join_str);
            }
        }
    }
    Ok(s)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_and() {
        let condition_1 = Operator::And(vec![
            Operator::Eq(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6])),
            Operator::Eq(TagName::PlainTagName(vec![7,8,9]), TargetValue::Unencrypted("spam".to_string())),
        ]);
        let condition_2 = Operator::And(vec![
            Operator::Eq(TagName::EncryptedTagName(vec![10,11,12]), TargetValue::Encrypted(vec![13,14,15])),
            Operator::Not(Box::new(Operator::Eq(TagName::PlainTagName(vec![16,17,18]), TargetValue::Unencrypted("eggs".to_string()))))
        ]);
        let query = Operator::Or(vec![condition_1, condition_2]);
        let class = vec![100,100,100];
        let (_query, _arguments) = wql_to_sql(&class, &query, None).unwrap();
    }
}
