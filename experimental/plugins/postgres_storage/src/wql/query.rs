use postgres::types::ToSql;

use errors::wallet::WalletQueryError;
use language::{Operator,TagName,TargetValue};

// Translates Wallet Query Language to SQL
// WQL input is provided as a reference to a top level Operator
// Result is a tuple of query string and query arguments
pub fn wql_to_sql<'a>(record_type: &'a Vec<u8>, wallet_id: &'a String, op: &'a Operator, _options: Option<&str>) -> Result<(String, Vec<&'a dyn ToSql>), WalletQueryError> {
    let mut arguments: Vec<&dyn ToSql> = Vec::new();
    let clause_string = operator_to_sql(op, &mut arguments, wallet_id)?;
    let mut query_string = "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i".to_string();

    if !clause_string.is_empty() {
        query_string.push_str(" INNER JOIN ");
        query_string.push_str(&clause_string);
    }
    arguments.push(record_type);
    arguments.push(wallet_id);
    query_string.push_str(" WHERE i.type = $$ AND i.wallet_id = $$");

    Ok((convert_query_to_psql_args(&query_string), arguments))
}


pub fn wql_to_sql_count<'a>(record_type: &'a Vec<u8>, wallet_id: &'a String, op: &'a Operator) -> Result<(String, Vec<&'a dyn ToSql>), WalletQueryError> {
    let mut arguments: Vec<&dyn ToSql> = Vec::new();
    let clause_string = operator_to_sql(op, &mut arguments, wallet_id)?;
    let mut query_string = "SELECT count(*) FROM items as i".to_string();
    if !clause_string.is_empty() {
        query_string.push_str(" INNER JOIN ");
        query_string.push_str(&clause_string);
    }
    arguments.push(record_type);
    arguments.push(wallet_id);
    query_string.push_str(" WHERE i.type = $$ AND i.wallet_id = $$");

    Ok((convert_query_to_psql_args(&query_string), arguments))
}

fn convert_query_to_psql_args(query: &str) -> String {
    let mut index = 1;
    let mut s: String = query.to_owned();
    while s.find("$$") != None {
        let arg_str = format!("${}", index);
        s = s.replacen("$$", &arg_str, 1);
        index = index + 1;
    }
    s
}

fn operator_to_sql<'a>(op: &'a Operator, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    match *op {
        Operator::Eq(ref tag_name, ref target_value) => eq_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Neq(ref tag_name, ref target_value) => neq_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Gt(ref tag_name, ref target_value) => gt_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Gte(ref tag_name, ref target_value) => gte_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Lt(ref tag_name, ref target_value) => lt_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Lte(ref tag_name, ref target_value) => lte_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::Like(ref tag_name, ref target_value) => like_to_sql(tag_name, target_value, arguments, wallet_id),
        Operator::In(ref tag_name, ref target_values) => in_to_sql(tag_name, target_values, arguments, wallet_id),
        Operator::And(ref suboperators) => and_to_sql(suboperators, arguments, wallet_id),
        Operator::Or(ref suboperators) => or_to_sql(suboperators, arguments, wallet_id),
        Operator::Not(ref suboperator) => not_to_sql(suboperator, arguments, wallet_id),
    }
}


fn eq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let alias = arguments.len();
    arguments.push(wallet_id);

    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value = $$", alias))
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_encrypted as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value = $$", alias))
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for equality operator".to_string()))
    }
}


fn neq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let alias = arguments.len();
    arguments.push(wallet_id);

    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value != $$", alias))
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_encrypted as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value != $$", alias))
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for inequality operator".to_string()))
    }
}


fn gt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let alias = arguments.len();
    arguments.push(wallet_id);

    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value > $$", alias))
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $gt operator".to_string()))
    }
}


fn gte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let alias = arguments.len();
    arguments.push(wallet_id);

    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value >= $$", alias))
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $gte operator".to_string()))
    }
}


fn lt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let alias = arguments.len();
    arguments.push(wallet_id);

    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value < $$", alias))
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $lt operator".to_string()))
    }
}


fn lte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let alias = arguments.len();
    arguments.push(wallet_id);

    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value <= $$", alias))
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $lte operator".to_string()))
    }
}


fn like_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let alias = arguments.len();
    arguments.push(wallet_id);

    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            Ok(format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value LIKE $$", alias))

        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $like operator".to_string()))
    }
}


fn in_to_sql<'a>(name: &'a TagName, values: &'a Vec<TargetValue>, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let mut in_string = String::new();
    let alias = arguments.len();
    arguments.push(wallet_id);

    match name {
        &TagName::PlainTagName(ref queried_name) => {
            in_string.push_str(&format!("tags_plaintext as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value IN (", alias));
            arguments.push(queried_name);

            for (index, value) in values.iter().enumerate() {
                if let &TargetValue::Unencrypted(ref target) = value {
                    in_string.push_str("$$");
                    arguments.push(target);
                    if index < values.len() - 1 {
                        in_string.push(',');
                    }
                } else {
                    return Err(WalletQueryError::StructureErr("Encrypted tag value in $in for nonencrypted tag name".to_string()))
                }
            }

            Ok(in_string + ")")
        },
        &TagName::EncryptedTagName(ref queried_name) => {
            in_string.push_str(&format!("tags_encrypted as alias{0} ON alias{0}.item_id = i.id AND alias{0}.wallet_id = $$ AND alias{0}.name = $$ AND alias{0}.value IN (", alias));
            arguments.push(queried_name);
            let index_before_last = values.len() - 2;

            for (index, value) in values.iter().enumerate() {
                if let &TargetValue::Encrypted(ref target) = value {
                    in_string.push_str("$$");
                    arguments.push(target);
                    if index <= index_before_last {
                        in_string.push(',');
                    }
                } else {
                    return Err(WalletQueryError::StructureErr("Unencrypted tag value in $in for encrypted tag name".to_string()))
                }
            }

            Ok(in_string + ")")
        },
    }
}


fn and_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    join_operators(suboperators, " INNER JOIN ", arguments, wallet_id)
}


fn or_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    Err(WalletQueryError::StructureErr("$or operation not supported".to_string()))
}


fn not_to_sql<'a>(suboperator: &'a Operator, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    Err(WalletQueryError::StructureErr("$not operation not supported".to_string()))
}


fn join_operators<'a>(operators: &'a [Operator], join_str: &str, arguments: &mut Vec<&'a dyn ToSql>, wallet_id: &'a String) -> Result<String, WalletQueryError> {
    let mut s = String::new();
    if operators.len() > 0 {
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

    fn get_expected_query(table_name: &str, operator: &str) -> String {

        return format!("SELECT i.id, i.name, i.value, i.key, i.type FROM items as i INNER JOIN {} as alias0 ON alias0.item_id = i.id AND alias0.wallet_id = $1 AND alias0.name = $2 AND alias0.value {} $3 WHERE i.type = $4 AND i.wallet_id = $5", table_name, operator)
    }

    fn setup_tag_search() -> (String, Vec<u8>) {
        let wallet_id = "walletID".to_string();
        let class = vec![100,100,100];

        return (wallet_id, class);
    }

    const TAGS_ENCRYPTED_TABLE_NAME: &str = "tags_encrypted";
    const TAGS_PLAIN_TABLE_NAME: &str = "tags_plaintext";

    #[test]
    fn simple_and_convert_args_works() {
        assert_eq!("This $1 is $2 a $3 string!", convert_query_to_psql_args("This $$ is $$ a $$ string!"));
        assert_eq!("This is a string!", convert_query_to_psql_args("This is a string!"));
    }

    #[test]
    fn should_format_valid_and_statement() {
        let condition = Operator::And(vec![
            Operator::Eq(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6])),
            Operator::Eq(TagName::PlainTagName(vec![7,8,9]), TargetValue::Unencrypted("spam".to_string())),
        ]);
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i INNER JOIN tags_encrypted as alias0 ON alias0.item_id = i.id AND alias0.wallet_id = $1 AND alias0.name = $2 AND alias0.value = $3 INNER JOIN tags_plaintext as alias3 ON alias3.item_id = i.id AND alias3.wallet_id = $4 AND alias3.name = $5 AND alias3.value = $6 WHERE i.type = $7 AND i.wallet_id = $8");
        assert_eq!(_arguments.len(), 8);
    }

    #[test]
    fn should_format_valid_count_statement() {
        let condition = Operator::And(vec![
            Operator::Eq(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6])),
            Operator::Eq(TagName::PlainTagName(vec![7,8,9]), TargetValue::Unencrypted("spam".to_string())),
        ]);
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql_count(&class, &wallet_id, &condition).unwrap();

        assert_eq!(query, "SELECT count(*) FROM items as i INNER JOIN tags_encrypted as alias0 ON alias0.item_id = i.id AND alias0.wallet_id = $1 AND alias0.name = $2 AND alias0.value = $3 INNER JOIN tags_plaintext as alias3 ON alias3.item_id = i.id AND alias3.wallet_id = $4 AND alias3.name = $5 AND alias3.value = $6 WHERE i.type = $7 AND i.wallet_id = $8");
        assert_eq!(_arguments.len(), 8);
    }

    #[test]
    fn should_format_valid_encrypted_tags_eq_statement() {
        let condition = Operator::Eq(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_ENCRYPTED_TABLE_NAME, "="));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    fn should_format_valid_plain_tags_eq_statement() {
        let condition = Operator::Eq(TagName::PlainTagName(vec![1,2,3]), TargetValue::Unencrypted("unencrypted".to_string()));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_PLAIN_TABLE_NAME, "="));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    #[should_panic(expected = "Invalid combination of tag name and value for equality operator")]
    fn should_panic_when_invalid_eq_arguments_used() {
        let condition = Operator::Eq(TagName::PlainTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    fn should_format_valid_encrypted_tags_neq_statement() {
        let condition = Operator::Neq(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_ENCRYPTED_TABLE_NAME, "!="));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    fn should_format_valid_plain_tags_neq_statement() {
        let condition = Operator::Neq(TagName::PlainTagName(vec![1,2,3]), TargetValue::Unencrypted("unencrypted".to_string()));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_PLAIN_TABLE_NAME, "!="));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    #[should_panic(expected = "Invalid combination of tag name and value for inequality operator")]
    fn should_panic_when_invalid_neq_arguments_used() {
        let condition = Operator::Neq(TagName::PlainTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    fn should_format_valid_plain_tags_gt_statement() {
        let condition = Operator::Gt(TagName::PlainTagName(vec![1,2,3]), TargetValue::Unencrypted("unencrypted".to_string()));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_PLAIN_TABLE_NAME, ">"));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    #[should_panic(expected = "Invalid combination of tag name and value for $gt operator")]
    fn should_panic_when_invalid_gt_arguments_used() {
        let condition = Operator::Gt(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    fn should_format_valid_plain_tags_gte_statement() {
        let condition = Operator::Gte(TagName::PlainTagName(vec![1,2,3]), TargetValue::Unencrypted("unencrypted".to_string()));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_PLAIN_TABLE_NAME, ">="));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    #[should_panic(expected = "Invalid combination of tag name and value for $gte operator")]
    fn should_panic_when_invalid_gte_statement() {
        let condition = Operator::Gte(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    fn should_format_valid_plain_tags_lt_statement() {
        let condition = Operator::Lt(TagName::PlainTagName(vec![1,2,3]), TargetValue::Unencrypted("unencrypted".to_string()));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_PLAIN_TABLE_NAME, "<"));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    #[should_panic(expected = "Invalid combination of tag name and value for $lt operator")]
    fn should_panic_when_invalid_lt_arguments_used() {
        let condition = Operator::Lt(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    fn should_format_valid_plain_tags_lte_statement() {
        let condition = Operator::Lte(TagName::PlainTagName(vec![1,2,3]), TargetValue::Unencrypted("unencrypted".to_string()));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_PLAIN_TABLE_NAME, "<="));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    #[should_panic(expected = "Invalid combination of tag name and value for $lte operator")]
    fn should_panic_when_invalid_lte_arguments_used() {
        let condition = Operator::Lte(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    fn should_format_valid_plain_tags_like_statement() {
        let condition = Operator::Like(TagName::PlainTagName(vec![1,2,3]), TargetValue::Unencrypted("unencrypted".to_string()));
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, get_expected_query(TAGS_PLAIN_TABLE_NAME, "LIKE"));
        assert_eq!(_arguments.len(), 5)
    }

    #[test]
    #[should_panic(expected = "Invalid combination of tag name and value for $like operator")]
    fn should_panic_when_invalid_like_arguments_used() {
        let condition = Operator::Like(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6]));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    fn should_format_valid_encrypted_tags_in_statement() {
        let arguments: Vec<TargetValue> = vec![
            TargetValue::Encrypted(vec![4,5,6]),
            TargetValue::Encrypted(vec![1,2,3]),
        ];
        let condition = Operator::In(TagName::EncryptedTagName(vec![1,2,3]), arguments);
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i INNER JOIN tags_encrypted as alias0 ON alias0.item_id = i.id AND alias0.wallet_id = $1 AND alias0.name = $2 AND alias0.value IN ($3,$4) WHERE i.type = $5 AND i.wallet_id = $6");
        assert_eq!(_arguments.len(), 6)
    }

    #[test]
    fn should_format_valid_plain_tags_in_statement() {
        let arguments: Vec<TargetValue> = vec![
            TargetValue::Unencrypted("un".to_string()),
            TargetValue::Unencrypted("un".to_string()),
        ];
        let condition = Operator::In(TagName::PlainTagName(vec![1,2,3]), arguments);
        let (wallet_id, class) = setup_tag_search();

        let (query, _arguments) = wql_to_sql(&class, &wallet_id, &condition, None).unwrap();

        assert_eq!(query, "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i INNER JOIN tags_plaintext as alias0 ON alias0.item_id = i.id AND alias0.wallet_id = $1 AND alias0.name = $2 AND alias0.value IN ($3,$4) WHERE i.type = $5 AND i.wallet_id = $6");
        assert_eq!(_arguments.len(), 6)
    }

    #[test]
    #[should_panic(expected = "Unencrypted tag value in $in for encrypted tag name")]
    fn should_panic_when_invalid_encrypted_tags_in_arguments_used() {
        let arguments: Vec<TargetValue> = vec![
            TargetValue::Unencrypted("un".to_string()),
            TargetValue::Encrypted(vec![4,5,6]),
        ];
        let condition = Operator::In(TagName::EncryptedTagName(vec![1,2,3]), arguments);
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    #[should_panic(expected = "Encrypted tag value in $in for nonencrypted tag name")]
    fn should_panic_when_invalid_plain_tags_in_arguments_used() {
        let arguments: Vec<TargetValue> = vec![
            TargetValue::Encrypted(vec![4,5,6]),
            TargetValue::Unencrypted("un".to_string()),
        ];
        let condition = Operator::In(TagName::PlainTagName(vec![1,2,3]), arguments);
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    #[should_panic(expected = "$or operation not supported")]
    fn should_panic_on_or_operation_since_not_supported() {
        let condition = Operator::Or(vec![
            Operator::Eq(TagName::EncryptedTagName(vec![1,2,3]), TargetValue::Encrypted(vec![4,5,6])),
            Operator::Eq(TagName::PlainTagName(vec![7,8,9]), TargetValue::Unencrypted("spam".to_string())),
        ]);
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }

    #[test]
    #[should_panic(expected = "$not operation not supported")]
    fn should_panic_on_not_operation_since_not_supported() {
        let condition = Operator::Not(Box::from(Operator::Eq(TagName::EncryptedTagName(vec![1, 2, 3]), TargetValue::Encrypted(vec![4, 5, 6]))));
        let (wallet_id, class) = setup_tag_search();

        wql_to_sql(&class, &wallet_id, &condition, None).unwrap();
    }
}
