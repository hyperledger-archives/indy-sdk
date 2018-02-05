extern crate serde_json;
extern crate term;

use self::term::{Attr, color};
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

pub fn print_list_table(rows: &Vec<serde_json::Value>, headers: &[(&str, &str)], empty_msg: &str) {
    if rows.is_empty() {
        return println_succ!("{}", empty_msg);
    }

    let mut table = Table::new();

    print_header(&mut table, headers);

    for row in rows {
        print_row(&mut table, row, headers);
    }

    table.printstd();
}

pub fn print_table(row: &serde_json::Value, headers: &[(&str, &str)]) {
    let mut table = Table::new();

    print_header(&mut table, headers);

    print_row(&mut table, row, headers);

    table.printstd();
}

pub fn print_header(table: &mut Table, headers: &[(&str, &str)]) {
    let tittles = headers.iter().clone()
        .map(|&(_, ref header)| Cell::new(header)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN))
        ).collect::<Vec<Cell>>();

    table.add_row(Row::new(tittles));
}

pub fn print_row(table: &mut Table, row: &serde_json::Value, headers: &[(&str, &str)]) {
    let columns = headers.iter().clone()
        .map(|&(ref key, _)| {
            let mut value = "-".to_string();
            if row[key].is_string() {
                value = row[key].as_str().unwrap().to_string()
            }
            if row[key].is_i64() {
                value = row[key].as_i64().unwrap().to_string();
            }
            if row[key].is_boolean() {
                value = row[key].as_bool().unwrap().to_string()
            }
            if row[key].is_array() {
                value = row[key].as_array().unwrap()
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            }
            if row[key].is_object() {
                value = row[key].as_object().unwrap()
                    .iter()
                    .map(|(key, value)| format!("{}:{}", key, value))
                    .collect::<Vec<String>>()
                    .join(",");
                value = format!("{{{}}}", value)
            }
            Cell::new(&value)
        })
        .collect::<Vec<Cell>>();
    table.add_row(Row::new(columns));
}