use std::fmt;
use error::ToErrorCode;

#[derive(Debug)]
pub enum MessageError {
    MessagePackError(),
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageError::MessagePackError() => write!(f, "Deserialize Error for Message Pack"),
        }
    }
}
impl ToErrorCode for MessageError {
    fn to_error_code(&self) -> u32 {
       match *self {
           MessageError::MessagePackError() => 8001,
       }
    }

}
