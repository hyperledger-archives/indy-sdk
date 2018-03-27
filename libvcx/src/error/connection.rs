
use std::fmt;
use error::ToErrorCode;
use std::error::Error;

#[derive(Debug)]
pub enum ConnectionError {
    GeneralConnectionError(),
    ConnectionNotReady(),
    InviteDetailError(),
    CommonError(u32),
}


impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConnectionError::GeneralConnectionError() => write!(f, "Error with Connection"),
            ConnectionError::InviteDetailError() => write!(f, "Invite Detail Error"),
            ConnectionError::ConnectionNotReady() => write!(f, "Object not ready for specified action"),
            ConnectionError::CommonError(x) => write!(f, "This common error had value: {}", x),
        }
    }
}

impl Error for ConnectionError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            ConnectionError::GeneralConnectionError() => None,
            ConnectionError::ConnectionNotReady() => None,
            ConnectionError::InviteDetailError() => None,
            ConnectionError::CommonError(x) => None,
        }
    }

    // TODO: Either implement this correctly or remove.
    fn description(&self) -> &str {
        match *self {
            ConnectionError::GeneralConnectionError() => "General Connection Error",
            ConnectionError::ConnectionNotReady() => "Connection Not Ready",
            ConnectionError::InviteDetailError() => "Invite Detail Error",
            ConnectionError::CommonError(x) => "Common Error",
        }
    }
}

impl ToErrorCode for ConnectionError {
   fn to_error_code(&self) -> u32 {
       match *self {
           ConnectionError::GeneralConnectionError() => 1002,
           ConnectionError::ConnectionNotReady() => 1005,
           ConnectionError::InviteDetailError() => 9999,
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
        assert_eq!(ConnectionError::GeneralConnectionError().to_error_code(), 1002);
        assert_eq!(ConnectionError::ConnectionNotReady().to_string(), "Object not ready for specified action");
        assert_eq!(ConnectionError::ConnectionNotReady().to_error_code(), 1005);

    }
}