use std::fmt;
use error::ToErrorCode;
use utils::error::{BUILD_CLAIM_DEF_REQ_ERR, CLAIM_DEF_ALREADY_CREATED, CREATE_CLAIM_DEF_ERR };

#[derive(Debug)]
pub enum CredDefError {
    BuildCredDefRequestError(),
    CreateCredDefError(),
    CredDefAlreadyCreatedError(),
    CommonError(u32),
}
impl fmt::Display for CredDefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CredDefError::BuildCredDefRequestError() => write!(f, "Error Building Cred Def Request"),
            CredDefError::CommonError(x) => write!(f, "This Cred Def common error had a value: {}", x),
            CredDefError::CreateCredDefError() => write!(f, "{}", CREATE_CLAIM_DEF_ERR.message ),
            CredDefError::CredDefAlreadyCreatedError() => write!(f, "{}", CLAIM_DEF_ALREADY_CREATED.message ),
        }
    }
}
impl ToErrorCode for CredDefError {
    fn to_error_code(&self) -> u32 {
        match *self {
            CredDefError::BuildCredDefRequestError() => BUILD_CLAIM_DEF_REQ_ERR.code_num,
            CredDefError::CreateCredDefError() => CREATE_CLAIM_DEF_ERR.code_num,
            CredDefError::CredDefAlreadyCreatedError() => CLAIM_DEF_ALREADY_CREATED.code_num,
            CredDefError::CommonError(x) => x,
        }
    }
}

impl PartialEq for CredDefError {
    fn eq(&self, other: &CredDefError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}