use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum CommonError {
    InvalidParam1(String),
    InvalidParam2(String),
    InvalidParam3(String),
    InvalidParam4(String),
    InvalidParam5(String),
    InvalidState(String)
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CommonError::InvalidParam1(ref description) => write!(f, "Invalid param 1: {}", description),
            CommonError::InvalidParam2(ref description) => write!(f, "Invalid param 2: {}", description),
            CommonError::InvalidParam3(ref description) => write!(f, "Invalid param 3: {}", description),
            CommonError::InvalidParam4(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidParam5(ref description) => write!(f, "Invalid param 4: {}", description),
            CommonError::InvalidState(ref description) => write!(f, "Invalid library state: {}", description)
        }
    }
}

impl error::Error for CommonError {
    fn description(&self) -> &str {
        match *self {
            CommonError::InvalidParam1(ref description) => description,
            CommonError::InvalidParam2(ref description) => description,
            CommonError::InvalidParam3(ref description) => description,
            CommonError::InvalidParam4(ref description) => description,
            CommonError::InvalidParam5(ref description) => description,
            CommonError::InvalidState(ref description) => description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            CommonError::InvalidParam1(ref description) => None,
            CommonError::InvalidParam2(ref description) => None,
            CommonError::InvalidParam3(ref description) => None,
            CommonError::InvalidParam4(ref description) => None,
            CommonError::InvalidParam5(ref description) => None,
            CommonError::InvalidState(ref description) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    // TODO: FIXME: Provide tests!
}