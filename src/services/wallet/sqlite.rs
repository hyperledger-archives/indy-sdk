extern crate rusqlite;

use errors::WalletError;
use services::wallet::{Wallet, AnoncredsWallet, IdentityWallet};
use std::error::Error;
use self::rusqlite::Connection;
use self::rusqlite::Error::QueryReturnedNoRows;

pub struct SqliteWallet {
    connection: Connection
}

impl SqliteWallet {
    pub fn new() -> SqliteWallet {
        match Connection::open("sovrin.db") {
            Ok(connection) => {
                connection.execute(
                    "CREATE TABLE IF NOT EXISTS wallet (
                          key       TEXT NOT NULL,
                          value     TEXT NOT NULL
                          )",
                    &[]);

                SqliteWallet {
                    connection: connection
                }
            },
            Err(err) => panic!("{}", err)
        }
    }
}

impl Wallet for SqliteWallet {
    fn set(&self, keys: &[&String], value: &String) -> Result<(), WalletError> {
        let string_keys: Vec<String> = keys.to_vec()
            .iter()
            .map(|k| format!("{}", k))
            .collect();

        match self.connection.execute(
            "INSERT INTO wallet (key, value) VALUES (?1, ?2)",
            &[&string_keys.join("::"), value]
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(WalletError::UnknownError(Box::new(err)))
        }
    }

    fn get(&self, keys: &[&String]) -> Result<String, WalletError> {
        let string_keys: Vec<String> = keys.to_vec()
            .iter()
            .map(|k| format!("{}", k))
            .collect();

        match self.connection.prepare("SELECT value FROM wallet WHERE key = ?1 LIMIT 1") {
            Ok(mut stmt) =>
                match stmt.query(&[&string_keys.join("::")]) {
                    Ok(mut rows) =>
                        match rows.next() {
                            Some(row) => match row {
                                Ok(r) => Ok(r.get(0)),
                                Err(err) => Err(WalletError::NotFoundError)
                            },
                            _ => Err(WalletError::NotFoundError)
                        },
                    Err(err) => Err(WalletError::UnknownError(Box::new(err)))
                },
            Err(err) => Err(WalletError::UnknownError(Box::new(err)))
        }
    }
}