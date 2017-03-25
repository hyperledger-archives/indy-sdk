use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum SovrinError {
    NoConsensus(String),
    TimedOut(String),
    InvalidData(String),
    Io(io::Error)
}

impl fmt::Display for SovrinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SovrinError::NoConsensus(ref description) => write!(f, "No consensus: {}", description),
            SovrinError::TimedOut(ref description) => write!(f, "Timeout: {}", description),
            SovrinError::InvalidData(ref description) => write!(f, "Invalid data: {}", description),
            SovrinError::Io(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for SovrinError {
    fn description(&self) -> &str {
        match *self {
            SovrinError::NoConsensus(ref description) => &description,
            SovrinError::TimedOut(ref description) => &description,
            SovrinError::InvalidData(ref description) => &description,
            SovrinError::Io(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SovrinError::NoConsensus(ref description) => None,
            SovrinError::TimedOut(ref description) => None,
            SovrinError::InvalidData(ref description) => None,
            SovrinError::Io(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for SovrinError {
    fn from(err: io::Error) -> SovrinError {
        SovrinError::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn sovrin_error_can_be_created() {
        let no_consensus_error = SovrinError::NoConsensus("TEST".to_string());
        let invalid_data_error = SovrinError::InvalidData("TEST".to_string());
        let timed_out_error = SovrinError::TimedOut("TEST".to_string());
    }

    #[test]
    fn sovrin_error_can_be_formatted() {
        let no_consensus_error_formatted = format!("{}", SovrinError::NoConsensus("TEST".to_string()));
        let invalid_data_error_formatted = format!("{}", SovrinError::InvalidData("TEST".to_string()));
        let timed_out_error_formatted = format!("{}", SovrinError::TimedOut("TEST".to_string()));

        assert_eq!("No consensus: TEST", no_consensus_error_formatted);
        assert_eq!("Invalid data: TEST", invalid_data_error_formatted);
        assert_eq!("Timeout: TEST", timed_out_error_formatted);
    }
}