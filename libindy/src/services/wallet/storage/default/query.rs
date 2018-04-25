use rusqlite;
use rusqlite::types::ToSql;

use services::wallet::language::{Operator,TagName,TargetValue};


// Translates Wallet Query Language to SQL
// WQL input is provided as a reference to a top level Operator
// Result is a tuple of query string and query arguments
pub fn wql_to_sql<'a>(class: &'a Vec<u8>, op: &'a Operator, options: Option<&str>) -> (String, Vec<&'a ToSql>) {
    let mut arguments: Vec<&ToSql> = Vec::new();
    let clause_string = operator_to_sql(op, &mut arguments);
    let has_nonencrypted = clause_string.contains("plain_tag");
    let has_encrypted = clause_string.contains("enc_tag");
    let mut query_string = create_query_string(clause_string, has_nonencrypted, has_encrypted);
    query_string.push_str(" AND i.type = ?;");
    arguments.push(class);
    (query_string, arguments)
}


fn create_query_string(clause_string: String, has_nonenc: bool, has_enc: bool) -> String {
    let mut query_string = "SELECT i.id, i.name, i.value, i.key FROM items as i".to_string();
    if has_nonenc {
        query_string.push_str(" INNER JOIN tags_plaintext as plain_tag ON i.id == plain_tag.item_id");
    }
    if has_enc {
        query_string.push_str(" INNER JOIN tags_encrypted as enc_tag ON i.id == enc_tag.item_id");
    }
    query_string.push_str(" WHERE ");
    query_string + &clause_string
}


fn operator_to_sql<'a>(op: &'a Operator, arguments: &mut Vec<&'a ToSql>) -> String {
    match *op {
        Operator::Eq(ref tag_name, ref target_value) => eq_to_sql(tag_name, target_value, arguments),
        Operator::Neq(ref tag_name, ref target_value) => neq_to_sql(tag_name, target_value, arguments),
        Operator::Gt(ref tag_name, ref target_value) => gt_to_sql(tag_name, target_value, arguments),
        Operator::Gte(ref tag_name, ref target_value) => gte_to_sql(tag_name, target_value, arguments),
        Operator::Lt(ref tag_name, ref target_value) => lt_to_sql(tag_name, target_value, arguments),
        Operator::Lte(ref tag_name, ref target_value) => lte_to_sql(tag_name, target_value, arguments),
        Operator::Like(ref tag_name, ref target_value) => like_to_sql(tag_name, target_value, arguments),
        Operator::Regex(ref tag_name, ref target_value) => regex_to_sql(tag_name, target_value, arguments),
        Operator::In(ref tag_name, ref target_values) => in_to_sql(tag_name, target_values, arguments),
        Operator::And(ref suboperators) => and_to_sql(suboperators, arguments),
        Operator::Or(ref suboperators) => or_to_sql(suboperators, arguments),
        Operator::Not(ref suboperator) => not_to_sql(suboperator, arguments),
    }
}


fn eq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value = ?)".to_string()
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(enc_tag.name = ? AND enc_tag.value = ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn neq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value != ?)".to_string()
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(enc_tag.name = ? AND enc_tag.value != ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn gt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value > ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn gte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value >= ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn lt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value < ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn lte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value <= ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn like_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value LIKE ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn regex_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a ToSql>) -> String {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            "(plain_tag.name = ? AND plain_tag.value REGEXP ?)".to_string()
        },
        _ => unreachable!()
    }
}


fn in_to_sql<'a>(name: &'a TagName, values: &'a Vec<TargetValue>, arguments: &mut Vec<&'a ToSql>) -> String {
    let mut in_string = String::new();
    match name {
        &TagName::PlainTagName(ref queried_name) => {
            in_string.push_str("(plain_tag.name = ? AND plain_tag.value IN (");
            arguments.push(queried_name);

            for (index, value) in values.iter().enumerate() {
                if let &TargetValue::Unencrypted(ref target) = value {
                    in_string.push_str("?");
                    arguments.push(target);
                    if index < values.len() - 1 {
                        in_string.push(',');
                    }
                } else { unreachable!() }
            }

            in_string + "))"
        },
        &TagName::EncryptedTagName(ref queried_name) => {
            in_string.push_str("(enc_tag.name = ? AND enc_tag.value IN (");
            arguments.push(queried_name);
            let index_before_last = values.len() - 2;

            for (index, value) in values.iter().enumerate() {
                if let &TargetValue::Encrypted(ref target) = value {
                    in_string.push_str("?");
                    arguments.push(target);
                    if index <= index_before_last {
                        in_string.push(',');
                    }
                } else { unreachable!() }
            }

            in_string + "))"
        },
        _ => unreachable!()
    }
}


fn and_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a ToSql>) -> String {
    join_operators(suboperators, " AND ", arguments)
}


fn or_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a ToSql>) -> String {
    join_operators(suboperators, " OR ", arguments)
}


fn not_to_sql<'a>(suboperator: &'a Operator, arguments: &mut Vec<&'a ToSql>) -> String {
    let suboperator_string = operator_to_sql(suboperator, arguments);
    "NOT (".to_string() + &suboperator_string + ")"
}


fn join_operators<'a>(operators: &'a [Operator], join_str: &str, arguments: &mut Vec<&'a ToSql>) -> String {
    let mut s = String::new();
    s.push('(');
    for (index, operator) in operators.iter().enumerate() {
        let operator_string = operator_to_sql(operator, arguments);
        s.push_str(&operator_string);
        if index < operators.len() - 1 {
            s.push_str(join_str);
        }
    }
    s.push(')');
    s
}


#[cfg(test)]
mod tests {
    use super::*;
    use services::wallet::language;

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
        let (query, arguments) = wql_to_sql(&class, &query, None);
    }
}