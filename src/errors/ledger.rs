use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum LedgerError {
    NoConsensus(String),
    TimedOut(String),
    InvalidData(String),
    Io(io::Error)
}

impl fmt::Display for LedgerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedgerError::NoConsensus(ref description) => write!(f, "No consensus: {}", description),
            LedgerError::TimedOut(ref description) => write!(f, "Timeout: {}", description),
            LedgerError::InvalidData(ref description) => write!(f, "Invalid data: {}", description),
            LedgerError::Io(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for LedgerError {
    fn description(&self) -> &str {
        match *self {
            LedgerError::NoConsensus(ref description) => &description,
            LedgerError::TimedOut(ref description) => &description,
            LedgerError::InvalidData(ref description) => &description,
            LedgerError::Io(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            LedgerError::NoConsensus(ref description) => None,
            LedgerError::TimedOut(ref description) => None,
            LedgerError::InvalidData(ref description) => None,
            LedgerError::Io(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for LedgerError {
    fn from(err: io::Error) -> LedgerError {
        LedgerError::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn sovrin_error_can_be_created() {
        let no_consensus_error = LedgerError::NoConsensus("TEST".to_string());
        let invalid_data_error = LedgerError::InvalidData("TEST".to_string());
        let timed_out_error = LedgerError::TimedOut("TEST".to_string());
    }

    #[test]
    fn sovrin_error_can_be_formatted() {
        let no_consensus_error_formatted = format!("{}", LedgerError::NoConsensus("TEST".to_string()));
        let invalid_data_error_formatted = format!("{}", LedgerError::InvalidData("TEST".to_string()));
        let timed_out_error_formatted = format!("{}", LedgerError::TimedOut("TEST".to_string()));

        assert_eq!("No consensus: TEST", no_consensus_error_formatted);
        assert_eq!("Invalid data: TEST", invalid_data_error_formatted);
        assert_eq!("Timeout: TEST", timed_out_error_formatted);
    }
}