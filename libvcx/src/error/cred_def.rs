use std::fmt;
use error::ToErrorCode;
use utils::error::{BUILD_CREDENTIAL_DEF_REQ_ERR, CREDENTIAL_DEF_ALREADY_CREATED, CREATE_CREDENTIAL_DEF_ERR };

#[derive(Debug)]
pub enum CredDefError {
    BuildCredDefRequestError(),
    RetrieveCredDefError(),
    CreateCredDefError(),
    CredDefAlreadyCreatedError(),
    SchemaError(String),
    CommonError(u32),
}
impl fmt::Display for CredDefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CredDefError::SchemaError(ref s) => write!(f, "Schema Error: {}", s),
            CredDefError::BuildCredDefRequestError() => write!(f, "Error Building Cred Def Request"),
            CredDefError::RetrieveCredDefError() => write!(f, "Error Retrieving Cred Def Request"),
            CredDefError::CommonError(x) => write!(f, "This Cred Def common error had a value: {}", x),
            CredDefError::CreateCredDefError() => write!(f, "{}", CREATE_CREDENTIAL_DEF_ERR.message ),
            CredDefError::CredDefAlreadyCreatedError() => write!(f, "{}", CREDENTIAL_DEF_ALREADY_CREATED.message ),
        }
    }
}
impl ToErrorCode for CredDefError {
    fn to_error_code(&self) -> u32 {
        match *self {
            CredDefError::SchemaError(ref s) => 7002,
            CredDefError::BuildCredDefRequestError() => BUILD_CREDENTIAL_DEF_REQ_ERR.code_num,
            CredDefError::RetrieveCredDefError() => 7001,
            CredDefError::CreateCredDefError() => CREATE_CREDENTIAL_DEF_ERR.code_num,
            CredDefError::CredDefAlreadyCreatedError() => CREDENTIAL_DEF_ALREADY_CREATED.code_num,
            CredDefError::CommonError(x) => x,
        }
    }
}

impl PartialEq for CredDefError {
    fn eq(&self, other: &CredDefError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}