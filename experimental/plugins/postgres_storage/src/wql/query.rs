use postgres::types::ToSql;
use postgres_storage::WalletScheme;

use errors::wallet::WalletQueryError;
use language::{Operator,TagName,TargetValue};


// Translates Wallet Query Language to SQL
// WQL input is provided as a reference to a top level Operator
// Result is a tuple of query string and query arguments
pub fn wql_to_sql<'a>(class: &'a Vec<u8>, op: &'a Operator, _options: Option<&str>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<(String, Vec<&'a dyn ToSql>), WalletQueryError> {
    let mut arguments: Vec<&dyn ToSql> = Vec::new();
    arguments.push(class);
    let clause_string = operator_to_sql(op, &mut arguments, strategy, wallet_id)?;
    
    let mut query_string = match strategy {
        WalletScheme::MultiWalletMultiTable => {
            str::replace("SELECT i.id, i.name, i.value, i.key, i.type FROM items_$1 as i WHERE i.type = $$", "$1" , &wallet_id).to_string()
        }
        _ => {
            "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i WHERE i.type = $$".to_string()
        }
    };
    
    if !clause_string.is_empty() {
        query_string.push_str(" AND ");
        query_string.push_str(&clause_string);
    }
    Ok((convert_query_to_psql_args(&query_string), arguments))
}


pub fn wql_to_sql_count<'a>(class: &'a Vec<u8>, op: &'a Operator, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<(String, Vec<&'a dyn ToSql>), WalletQueryError> {
    let mut arguments: Vec<&dyn ToSql> = Vec::new();
    arguments.push(class);
    let clause_string = operator_to_sql(op, &mut arguments, strategy, wallet_id)?;
    let mut query_string = match strategy {
        WalletScheme::MultiWalletMultiTable => {
            str::replace("SELECT count(*) FROM \"items_$1\" as i WHERE i.type = $$", "$1" , &wallet_id).to_string()
        }
        _ => {
            "SELECT count(*) FROM items as i WHERE i.type = $$".to_string()
        }
    };
 
    if !clause_string.is_empty() {
        query_string.push_str(" AND ");
        query_string.push_str(&clause_string);
    }
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

fn operator_to_sql<'a>(op: &'a Operator, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match *op {
        Operator::Eq(ref tag_name, ref target_value) => eq_to_sql(tag_name, target_value, arguments, strategy, wallet_id),
        Operator::Neq(ref tag_name, ref target_value) => neq_to_sql(tag_name, target_value, arguments, strategy, wallet_id),
        Operator::Gt(ref tag_name, ref target_value) => gt_to_sql(tag_name, target_value, arguments, strategy, wallet_id),
        Operator::Gte(ref tag_name, ref target_value) => gte_to_sql(tag_name, target_value, arguments, strategy, wallet_id),
        Operator::Lt(ref tag_name, ref target_value) => lt_to_sql(tag_name, target_value, arguments, strategy, wallet_id),
        Operator::Lte(ref tag_name, ref target_value) => lte_to_sql(tag_name, target_value, arguments, strategy, wallet_id),
        Operator::Like(ref tag_name, ref target_value) => like_to_sql(tag_name, target_value, arguments, strategy, wallet_id),
        Operator::In(ref tag_name, ref target_values) => in_to_sql(tag_name, target_values, arguments, strategy, wallet_id),
        Operator::And(ref suboperators) => and_to_sql(suboperators, arguments, strategy, wallet_id),
        Operator::Or(ref suboperators) => or_to_sql(suboperators, arguments, strategy, wallet_id),
        Operator::Not(ref suboperator) => not_to_sql(suboperator, arguments, strategy, wallet_id),
    }
}


fn eq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_$1\" WHERE name = $$ AND value = $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value = $$))".to_string()
                }
            };
            Ok(res)
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_encrypted_$1\" WHERE name = $$ AND value = $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_encrypted WHERE name = $$ AND value = $$))".to_string()
                }
            };
            Ok(res)
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for equality operator".to_string()))
    }
}


fn neq_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_$1\" WHERE name = $$ AND value != $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value != $$))".to_string()
                }
            };
            Ok(res)
        },
        (&TagName::EncryptedTagName(ref queried_name), &TargetValue::Encrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_encrypted_$1\" WHERE name = $$ AND value != $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_encrypted WHERE name = $$ AND value != $$))".to_string()
                }
            };
            Ok(res)
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for inequality operator".to_string()))
    }
}


fn gt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_$1\" WHERE name = $$ AND value > $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value > $$))".to_string()
                }
            };
            Ok(res)
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $gt operator".to_string()))
    }
}


fn gte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_$1\" WHERE name = $$ AND value >= $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value >= $$))".to_string()
                }
            };
            Ok(res)
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $gte operator".to_string()))
    }
}


fn lt_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_$1\" WHERE name = $$ AND value < $$)))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value < $$))".to_string()
                }
            };
            Ok(res)
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $lte operator".to_string()))
    }
}


fn lte_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_$1\" WHERE name = $$ AND value <= $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value <= $$))".to_string()
                }
            };
            Ok(res)
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $lte operator".to_string()))
    }
}


fn like_to_sql<'a>(name: &'a TagName, value: &'a TargetValue, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    match (name, value) {
        (&TagName::PlainTagName(ref queried_name), &TargetValue::Unencrypted(ref queried_value)) => {
            arguments.push(queried_name);
            arguments.push(queried_value);
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_$1\" WHERE name = $$ AND value LIKE $$))", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value LIKE $$))".to_string()
                }
            };
            Ok(res)
        },
        _ => Err(WalletQueryError::StructureErr("Invalid combination of tag name and value for $like operator".to_string()))
    }
}


fn in_to_sql<'a>(name: &'a TagName, values: &'a Vec<TargetValue>, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    let mut in_string = String::new();
    match name {
        &TagName::PlainTagName(ref queried_name) => {
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_plaintext_&1\" WHERE name = $$ AND value IN (", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_plaintext WHERE name = $$ AND value IN (".to_string()
                }
            };
            in_string.push_str(&res);
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

            Ok(in_string + ")))")
        },
        &TagName::EncryptedTagName(ref queried_name) => {
            let res = match strategy {
                WalletScheme::MultiWalletMultiTable => {
                    str::replace("(i.id in (SELECT item_id FROM \"tags_encrypted_&1\" WHERE name = $$ AND value IN (", "$1" , &wallet_id).to_string()
                }
                _ => {
                    "(i.id in (SELECT item_id FROM tags_encrypted WHERE name = $$ AND value IN (".to_string()
                }
            };
            in_string.push_str(&res);
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

            Ok(in_string + ")))")
        },
    }
}


fn and_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    join_operators(suboperators, " AND ", arguments, strategy, wallet_id)
}


fn or_to_sql<'a>(suboperators: &'a [Operator], arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    join_operators(suboperators, " OR ", arguments, strategy, wallet_id)
}


fn not_to_sql<'a>(suboperator: &'a Operator, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    let suboperator_string = operator_to_sql(suboperator, arguments, strategy, wallet_id)?;
    Ok("NOT (".to_string() + &suboperator_string + ")")
}


fn join_operators<'a>(operators: &'a [Operator], join_str: &str, arguments: &mut Vec<&'a dyn ToSql>, strategy: WalletScheme, wallet_id: &std::string::String) -> Result<String, WalletQueryError> {
    let mut s = String::new();
    if operators.len() > 0 {
        s.push('(');
        for (index, operator) in operators.iter().enumerate() {
            let operator_string = operator_to_sql(operator, arguments, strategy, wallet_id)?;
            s.push_str(&operator_string);
            if index < operators.len() - 1 {
                s.push_str(join_str);
            }
        }
        s.push(')');
    }
    Ok(s)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_and_convert_args_works() {
        assert_eq!("This $1 is $2 a $3 string!", convert_query_to_psql_args("This $$ is $$ a $$ string!"));
        assert_eq!("This is a string!", convert_query_to_psql_args("This is a string!"));
    }

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
        let wallet_id = "test";
        let (query, _arguments) = wql_to_sql(&class, &query, None, WalletScheme::DatabasePerWallet, &String::from(wallet_id)).unwrap();
        assert_eq!(query, "SELECT i.id, i.name, i.value, i.key, i.type FROM items as i WHERE i.type = $1 AND (((i.id in (SELECT item_id FROM tags_encrypted WHERE name = $2 AND value = $3)) AND (i.id in (SELECT item_id FROM tags_plaintext WHERE name = $4 AND value = $5))) OR ((i.id in (SELECT item_id FROM tags_encrypted WHERE name = $6 AND value = $7)) AND NOT ((i.id in (SELECT item_id FROM tags_plaintext WHERE name = $8 AND value = $9)))))")
    }
}
