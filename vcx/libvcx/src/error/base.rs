
use error::connection::ConnectionError;

#[derive(Debug)]
pub enum BaseError {
    ConnectionError(ConnectionError),
    GeneralError(),
}

impl From<ConnectionError> for BaseError {
    fn from(err: ConnectionError) -> BaseError { BaseError::ConnectionError(err) }
}