use std::error;
use std::io;
use std::fmt;
use std::num;

#[derive(Debug)]
enum AnoncredsError {
    InvalidData(String)
}

impl fmt::Display for AnoncredsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AnoncredsError::InvalidData(ref description) => write!(f, "Invalid data: {}", description)
        }
    }
}

impl error::Error for AnoncredsError {
    fn description(&self) -> &str {
        match *self {
            AnoncredsError::InvalidData(ref description) => &description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            AnoncredsError::InvalidData(ref description) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn anoncreds_error_can_be_created() {
        let error = AnoncredsError::InvalidData("TEST".to_string());
    }

    #[test]
    fn anoncreds_error_can_be_formatted() {
        let error_formatted = format!("{}", AnoncredsError::InvalidData("TEST".to_string()));
        assert_eq!("Invalid data: TEST", error_formatted);
    }
}