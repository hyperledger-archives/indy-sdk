extern crate serde_json;
extern crate term;

use self::term::{Attr, color};
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

pub fn print_table(headers: &Vec<(String, String)>, rows: &Vec<serde_json::Value>) {
    let mut table = Table::new();

    let tittles = headers.iter().clone()
        .map(|&(_, ref header)| Cell::new(header)
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN))
        ).collect::<Vec<Cell>>();

    table.add_row(Row::new(tittles));

    for row in rows {
        let columns = headers.iter().clone()
            .map(|&(ref key, _)| Cell::new(row[key].as_str().unwrap_or("-")))
            .collect::<Vec<Cell>>();

        table.add_row(Row::new(columns));
    }

    table.printstd();
}