extern crate rusqlite;

use errors::wallet::WalletError;
use self::rusqlite::Connection;
use services::wallet::Wallet;
use std::io;
use std::io::ErrorKind;

pub struct SqliteWallet {
    connection: Connection
}

impl SqliteWallet {
    pub fn new() -> Result<SqliteWallet, WalletError> {
        let connection =
            try!(Connection::open("sovrin.db")
                .map_err(|err| WalletError::from(io::Error::new(ErrorKind::NotConnected, err)))
            );

        try!(connection.execute(
            "CREATE TABLE IF NOT EXISTS wallet (
                          key       TEXT NOT NULL,
                          value     TEXT NOT NULL
                          )",
            &[])
            .map_err(|err| WalletError::from(io::Error::new(ErrorKind::InvalidData, err))));

        Ok(SqliteWallet {
            connection: connection
        })
    }
}

impl Wallet for SqliteWallet {
    fn set(&self, keys: &[&String], value: &String) -> Result<(), WalletError> {
        let string_keys: Vec<String> = keys.to_vec()
            .iter()
            .map(|k| format!("{}", k))
            .collect();

        self.connection.execute(
            "INSERT INTO wallet (key, value) VALUES (?1, ?2)",
            &[&string_keys.join("::"), value]
        )
            .map(|_| ())
            .map_err(|err| WalletError::from(io::Error::new(ErrorKind::InvalidData, err)))
    }

    fn get(&self, keys: &[&String]) -> Result<String, WalletError> {
        let string_keys: Vec<String> = keys.to_vec()
            .iter()
            .map(|k| format!("{}", k))
            .collect();

        let mut stmt = try!(
            self.connection.prepare("SELECT value FROM wallet WHERE key = ?1 LIMIT 1")
                .map_err(|err| WalletError::from(io::Error::new(ErrorKind::InvalidData, err)))
        );

        let mut rows = try!(
            stmt.query(&[&string_keys.join("::")])
                .map_err(|err| WalletError::from(io::Error::new(ErrorKind::InvalidData, err)))
        );

        rows.next()
            .ok_or(WalletError::NotFound("Value not found".to_string()))
            .and_then(|row|
                row
                    .map(|r| r.get(0))
                    .map_err(|err| WalletError::NotFound(format!("{}", err)))
            )
    }
}