use std::error;
use std::io;
use std::fmt;
use std::num;

use api::ErrorCode;

#[derive(Debug)]
pub enum PoolError {
    NotCreated(String),
    InvalidData(String),
    InvalidConfiguration(String),
    InvalidHandle(String),
    Io(io::Error)
}

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PoolError::NotCreated(ref description) => write!(f, "Not created: {}", description),
            PoolError::InvalidHandle(ref description) => write!(f, "Invalid Handle: {}", description),
            PoolError::InvalidConfiguration(ref description) => write!(f, "Invalid configuration: {}", description),
            PoolError::InvalidData(ref description) => write!(f, "Invalid data: {}", description),
            PoolError::Io(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for PoolError {
    fn description(&self) -> &str {
        match *self {
            PoolError::NotCreated(ref description) => description,
            PoolError::InvalidHandle(ref description) => description,
            PoolError::InvalidData(ref description) => description,
            PoolError::InvalidConfiguration(ref description) => description,
            PoolError::Io(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            PoolError::NotCreated(ref description) => None,
            PoolError::InvalidHandle(ref description) => None,
            PoolError::InvalidData(ref description) => None,
            PoolError::InvalidConfiguration(ref description) => None,
            PoolError::Io(ref err) => Some(err)
        }
    }
}

impl From<io::Error> for PoolError {
    fn from(err: io::Error) -> PoolError {
        PoolError::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    // TODO: FIXME: Provide tests!!!

    //    #[test]
    //    fn sovrin_error_can_be_created() {
    //        let not_created_error = PoolError::NotCreated("NotCreated".to_string());
    //        let invalid_handle_error = PoolError::InvalidHandle("InvalidHandle".to_string());
    //        let no_consensus_error = PoolError::NoConsensus("NoConsensus".to_string());
    //        let invalid_data_error = PoolError::InvalidData("InvalidData".to_string());
    //        let io_error = PoolError::Io(io::Error());
    //    }
    //
    //    #[test]
    //    fn sovrin_error_can_be_formatted() {
    //        let not_created_error_formatted = format!("{}", PoolError::NotCreated("NotCreated".to_string()));
    //        let invalid_handle_error_formatted = format!("{}", PoolError::InvalidHandle("InvalidHandle".to_string()));
    //        let no_consensus_error_formatted = format!("{}", PoolError::NoConsensus("NoConsensus".to_string()));
    //        let invalid_data_error_formatted = format!("{}", PoolError::InvalidData("InvalidData".to_string()));
    //        let io_error_formatted = format!("{}", PoolError::Io(io::Error()));
    //
    //        assert_eq!("No consensus: NotCreated", not_created_error_formatted);
    //        assert_eq!("No consensus: InvalidHandle", invalid_handle_error_formatted);
    //        assert_eq!("No consensus: NoConsensus", no_consensus_error_formatted);
    //        assert_eq!("Invalid data: InvalidData", invalid_data_error_formatted);
    //        assert_eq!("IO", io_error_formatted);
    //    }
}