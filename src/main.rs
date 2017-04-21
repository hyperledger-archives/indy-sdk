extern crate rusqlite;

use rusqlite::Connection;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None
    };
    conn.execute("INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
                 &[&me.name, &me.data]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}