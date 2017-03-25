use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
enum WalletError {
    NotFound(String),
    Io(io::Error)
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::NotFound(ref description) => write!(f, "Not found: {}", description),
            WalletError::Io(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for WalletError {
    fn description(&self) -> &str {
        match *self {
            WalletError::NotFound(ref description) => &description,
            WalletError::Io(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WalletError::NotFound(ref description) => None,
            WalletError::Io(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for WalletError {
    fn from(err: io::Error) -> WalletError {
        WalletError::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn wallet_error_can_be_created() {
        let error = WalletError::NotFound("TEST".to_string());
    }

    #[test]
    fn wallet_error_can_be_formatted() {
        let error_formatted = format!("{}", WalletError::NotFound("TEST".to_string()));
        assert_eq!("Not found: TEST", error_formatted);
    }
}