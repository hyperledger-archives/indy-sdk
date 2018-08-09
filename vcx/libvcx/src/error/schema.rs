use std::fmt;
use error::ToErrorCode;
use utils::error::{NO_PAYMENT_INFORMATION, INVALID_SCHEMA_CREATION, INVALID_SCHEMA_HANDLE, INVALID_SCHEMA_SEQ_NO};
#[derive(Debug)]
pub enum SchemaError {
    InvalidSchemaCreation(),
    InvalidHandle(),
    InvalidSchemaSeqNo(),
    DuplicateSchema(String),
    UnknownRejection(),
    NoPaymentInformation(),
    CommonError(u32),
}

impl ToErrorCode for SchemaError {
    fn to_error_code(&self) -> u32 {
        match *self {
            SchemaError::InvalidSchemaCreation() => INVALID_SCHEMA_CREATION.code_num,
            SchemaError::InvalidHandle() => INVALID_SCHEMA_HANDLE.code_num,
            SchemaError::InvalidSchemaSeqNo() => INVALID_SCHEMA_SEQ_NO.code_num,
            SchemaError::NoPaymentInformation() => NO_PAYMENT_INFORMATION.code_num,
            SchemaError::UnknownRejection() => 8887,
            SchemaError::DuplicateSchema(ref s) => 8888,
            SchemaError::CommonError(x) => x,
        }
    }
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SchemaError::InvalidSchemaCreation() => write!(f, "{}", INVALID_SCHEMA_CREATION.message),
            SchemaError::InvalidHandle() => write!(f, "{}", INVALID_SCHEMA_HANDLE.message),
            SchemaError::InvalidSchemaSeqNo() => write!(f, "{}", INVALID_SCHEMA_SEQ_NO.message),
            SchemaError::NoPaymentInformation() => write!(f, "{}", NO_PAYMENT_INFORMATION.message),
            SchemaError::UnknownRejection() => write!(f, "Unknown Rejection of Schema Creation, refer to libindy documentation."),
            SchemaError::DuplicateSchema(ref s) => write!(f, "{}", s),
            SchemaError::CommonError(x) => write!(f, "This Schema Common Error had a value of {}", x),
        }
    }
}

impl PartialEq for SchemaError {
    fn eq(&self, other: &SchemaError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_error () {
        assert_eq!(SchemaError::InvalidSchemaCreation().to_error_code(), INVALID_SCHEMA_CREATION.code_num);
    }

}