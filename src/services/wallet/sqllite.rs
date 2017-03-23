extern crate rusqlite;

use services::wallet::{WalletServiceImpl, AnnonCredsWalletServiceImpl, IdentityWalletServiceImpl};
use std::error::Error;
use self::rusqlite::Connection;


pub struct SqliteWalletService {
    conn: Connection
}

impl SqliteWalletService {
    pub fn new() -> SqliteWalletService {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE wallet (
                  key       TEXT NOT NULL,
                  value     TEXT NOT NULL
                  )",
            &[]).unwrap();

        SqliteWalletService { conn: conn }
    }
}

impl WalletServiceImpl for SqliteWalletService {
    fn set(&self, key: &[String], value: String) -> Result<(), Box<Error>> {
        self.conn.execute(
            "INSERT INTO wallet (key, value) VALUES (?1, ?2)",
            &[&key.join("::"), &value]
        );
        Ok(())
    }

    fn get(&self, key: &[String]) -> Option<String> {
        let key_string = key.join("::");

        get_value_by_key(&self.conn, key_string)
    }
}

impl AnnonCredsWalletServiceImpl for SqliteWalletService {
    fn get_master_secret(&self, did: String, schema: String, pk: String) -> Option<String> {
        let key = format!("{}::{}::{}", did, schema, pk);

        get_value_by_key(&self.conn, key)
    }
}

impl IdentityWalletServiceImpl for SqliteWalletService {
    fn get_key_by_did(&self, did: String) -> Option<String> {
        get_value_by_key(&self.conn, did)
    }
}

fn get_value_by_key(conn: &Connection, key: String) -> Option<String> {
    let mut stmt = conn.prepare("SELECT value FROM wallet WHERE key = ?1 LIMIT 1").unwrap();

    let mut rows = stmt.query(&[&key]).unwrap();

    match rows.next() {
        Some(data) =>
            match data {
                Ok(r) => r.get(0),
                _ => None
            },
        _ => None
    }
}