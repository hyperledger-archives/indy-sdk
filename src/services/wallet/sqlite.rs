extern crate rusqlite;

use errors::wallet::WalletError;
use self::rusqlite::Connection;
use services::wallet::Wallet;

impl From<rusqlite::Error> for WalletError {
    fn from(err: rusqlite::Error) -> WalletError {
        WalletError::BackendError(Box::new(err))
    }
}

pub struct SqliteWallet {
    connection: Connection
}

impl SqliteWallet {
    pub fn new() -> Result<SqliteWallet, WalletError> {
        let connection = try!(Connection::open("sovrin.db"));

        try!(connection.execute(
            "CREATE TABLE IF NOT EXISTS wallet (
                          key       TEXT NOT NULL,
                          value     TEXT NOT NULL
                          )",
            &[]));

        Ok(SqliteWallet {
            connection: connection
        })
    }
}

impl Wallet for SqliteWallet {
    fn set(&self, keys: &[&str], value: &str) -> Result<(), WalletError> {
        let string_keys: Vec<String> = keys.to_vec()
            .iter()
            .map(|k| format!("{}", k))
            .collect();

        self.connection.execute(
            "INSERT INTO wallet (key, value) VALUES (?1, ?2)",
            &[&string_keys.join("::"), &value.to_string()]
        )
            .map(|_| ())
            .map_err(|err| WalletError::from(err))
    }

    fn get(&self, keys: &[&str]) -> Result<Option<String>, WalletError> {
        let string_keys: Vec<String> = keys.to_vec()
            .iter()
            .map(|k| format!("{}", k))
            .collect();

        let mut stmt = try!(self.connection.prepare("SELECT value FROM wallet WHERE key = ?1 LIMIT 1"));

        let mut rows = try!(stmt.query(&[&string_keys.join("::")]));

        match rows.next() {
            Some(row) =>
                match row {
                    Ok(r) => Ok(Some(r.get(0))),
                    Err(err) => Err(WalletError::from(err))
                },
            None => Ok(None)
        }
    }
}