use error::{cred_def, connection};

#[derive(Debug)]
pub enum BaseError {
    ConnectionError(connection::ConnectionError),
    CredentialDefinitionError(cred_def::CredDefError),
    GeneralError(),
}

impl From<connection::ConnectionError> for BaseError {
    fn from(err: connection::ConnectionError) -> BaseError { BaseError::ConnectionError(err) }
}

impl From<cred_def::CredDefError> for BaseError {
    fn from(err: cred_def::CredDefError) -> BaseError { BaseError::CredentialDefinitionError(err)}
}