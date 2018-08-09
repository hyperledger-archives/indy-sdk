
use std::fmt;
use error::ToErrorCode;
use std::error::Error;
use utils::error::{ INVALID_CONNECTION_HANDLE,
                   CONNECTION_ERROR, NOT_READY,
                   INVALID_INVITE_DETAILS, INVALID_MSGPACK, INVALID_JSON,
                   UNKNOWN_LIBINDY_ERROR, CANNOT_DELETE_CONNECTION, CREATE_CONNECTION_ERROR,
                   INVALID_WALLET_SETUP, COMMON_ERROR };

#[derive(Debug)]
pub enum ConnectionError {
    CreateError(u32),
    GeneralConnectionError(),
    ConnectionNotReady(),
    InviteDetailError(),
    InvalidHandle(),
    InvalidWalletSetup(),
    InvalidMessagePack(),
    InvalidJson(),
    CannotDeleteConnection(),
    CommonError(u32),
}


impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::InvalidHandle() => write!(f, "{}", INVALID_CONNECTION_HANDLE.message),
            ConnectionError::GeneralConnectionError() => write!(f, "{}", CONNECTION_ERROR.message),
            ConnectionError::InviteDetailError() => write!(f, "{}", INVALID_INVITE_DETAILS.message),
            ConnectionError::CreateError(key) => write!(f, "{},{}", CREATE_CONNECTION_ERROR.message, key),
            ConnectionError::ConnectionNotReady() => write!(f, "{}", NOT_READY.message),
            ConnectionError::InvalidMessagePack() => write!(f, "{}", INVALID_MSGPACK.message),
            ConnectionError::InvalidWalletSetup() => write!(f, "{}", INVALID_WALLET_SETUP.message),
            ConnectionError::CannotDeleteConnection() => write!(f, "{}", CANNOT_DELETE_CONNECTION.message),
            ConnectionError::InvalidJson() => write!(f, "{}", INVALID_JSON.message),
            ConnectionError::CommonError(x) => connection_message(f, x),
        }
    }
}
fn connection_message(f: &mut fmt::Formatter, error_code: u32) -> fmt::Result {
    if error_code == UNKNOWN_LIBINDY_ERROR.code_num {
        // TODO: Make ths better, right now its just example code.
        write!(f, "{}: Code: {} .. starting recovery steps.", UNKNOWN_LIBINDY_ERROR.message, UNKNOWN_LIBINDY_ERROR.code_num)
    } else {
        // TODO: Make ths better, right now its just example code.
        write!(f, "Common Error had a value: {}.", error_code)
    }
}
impl Error for ConnectionError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::InvalidHandle() => None,
            ConnectionError::CreateError(ref key) => None,
            ConnectionError::GeneralConnectionError() => None,
            ConnectionError::ConnectionNotReady() => None,
            ConnectionError::InviteDetailError() => None,
            ConnectionError::InvalidMessagePack() => None,
            ConnectionError::InvalidWalletSetup() => None,
            ConnectionError::InvalidJson() => None,
            ConnectionError::CannotDeleteConnection() => None,
            ConnectionError::CommonError(x) => None,
        }
    }

    // TODO: Either implement this correctly or remove.
    fn description(&self) -> &str {
        match *self {
            ConnectionError::CreateError(ref key) =>  CREATE_CONNECTION_ERROR.message,
            ConnectionError::InvalidMessagePack() => INVALID_MSGPACK.message,
            ConnectionError::InvalidHandle() => INVALID_CONNECTION_HANDLE.message,
            ConnectionError::GeneralConnectionError() => CONNECTION_ERROR.message,
            ConnectionError::ConnectionNotReady() => NOT_READY.message,
            ConnectionError::InviteDetailError() => INVALID_INVITE_DETAILS.message,
            ConnectionError::InvalidWalletSetup() => INVALID_WALLET_SETUP.message,
            ConnectionError::CannotDeleteConnection() => CANNOT_DELETE_CONNECTION.message,
            ConnectionError::InvalidJson() => INVALID_JSON.message,
            ConnectionError::CommonError(x) => COMMON_ERROR.message,
        }
    }
}

impl ToErrorCode for ConnectionError {
   fn to_error_code(&self) -> u32 {
       match *self {
           ConnectionError::InvalidHandle() => INVALID_CONNECTION_HANDLE.code_num,
           ConnectionError::GeneralConnectionError() => CONNECTION_ERROR.code_num,
           ConnectionError::ConnectionNotReady() => NOT_READY.code_num,
           ConnectionError::InviteDetailError() => INVALID_INVITE_DETAILS.code_num,
           ConnectionError::InvalidMessagePack() => INVALID_MSGPACK.code_num,
           ConnectionError::CannotDeleteConnection() => CANNOT_DELETE_CONNECTION.code_num,
           ConnectionError::CreateError(key) => CREATE_CONNECTION_ERROR.code_num,
           ConnectionError::InvalidWalletSetup() => INVALID_WALLET_SETUP.code_num,
           ConnectionError::InvalidJson() => INVALID_JSON.code_num,
           ConnectionError::CommonError(x) => x,
       }
   }
}

impl PartialEq for ConnectionError {
    fn eq(&self, other: &ConnectionError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_error_code(){
        assert_eq!(ConnectionError::GeneralConnectionError().to_string(), "Error with Connection");
        assert_eq!(ConnectionError::GeneralConnectionError().to_error_code(), CONNECTION_ERROR.code_num);
        assert_eq!(ConnectionError::ConnectionNotReady().to_string(), "Object not ready for specified action");
        assert_eq!(ConnectionError::ConnectionNotReady().to_error_code(), NOT_READY.code_num);

    }
}