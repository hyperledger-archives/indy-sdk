use error::ToErrorCode;
use utils::error::{ INVALID_SCHEMA_CREATION, INVALID_SCHEMA_HANDLE, INVALID_SCHEMA_SEQ_NO};
#[derive(Debug)]
pub enum SchemaError {
    InvalidSchemaCreation(),
    InvalidHandle(),
    InvalidSchemaSeqNo(),
    CommonError(u32),
}

impl ToErrorCode for SchemaError {
    fn to_error_code(&self) -> u32 {
        match *self {
            SchemaError::InvalidSchemaCreation() => INVALID_SCHEMA_CREATION.code_num,
            SchemaError::InvalidHandle() => INVALID_SCHEMA_HANDLE.code_num,
            SchemaError::InvalidSchemaSeqNo() => INVALID_SCHEMA_SEQ_NO.code_num,
            SchemaError::CommonError(x) => x,
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