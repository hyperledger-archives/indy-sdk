use std::fmt;
use error::ToErrorCode;
use utils::error::{NO_PAYMENT_INFORMATION, INVALID_CREDENTIAL_DEF_HANDLE, BUILD_CREDENTIAL_DEF_REQ_ERR,
                   CREDENTIAL_DEF_ALREADY_CREATED, CREATE_CREDENTIAL_DEF_ERR, INVALID_REVOCATION_DETAILS,
                   INVALID_REV_REG_DEF_CREATION, INVALID_REV_ENTRY};

#[derive(Debug)]
pub enum CredDefError {
    BuildCredDefRequestError(),
    RetrieveCredDefError(),
    CreateCredDefError(),
    CredDefAlreadyCreatedError(),
    InvalidHandle(),
    SchemaError(String),
    NoPaymentInformation(),
    CommonError(u32),
    InvalidRevocationDetails(),
    InvalidRevocationEntry(),
    CreateRevRegDefError(),
}
impl fmt::Display for CredDefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CredDefError::SchemaError(ref s) => write!(f, "Schema Error: {}", s),
            CredDefError::BuildCredDefRequestError() => write!(f, "Error Building Cred Def Request"),
            CredDefError::RetrieveCredDefError() => write!(f, "Error Retrieving Cred Def Request"),
            CredDefError::InvalidHandle() => write!(f, "Invalid credentialdef handle"),
            CredDefError::CommonError(x) => write!(f, "This Cred Def common error had a value: {}", x),
            CredDefError::CreateCredDefError() => write!(f, "{}", CREATE_CREDENTIAL_DEF_ERR.message ),
            CredDefError::NoPaymentInformation() => write!(f, "{}", NO_PAYMENT_INFORMATION.message ),
            CredDefError::CredDefAlreadyCreatedError() => write!(f, "{}", CREDENTIAL_DEF_ALREADY_CREATED.message ),
            CredDefError::InvalidRevocationDetails() => write!(f, "{}", INVALID_REVOCATION_DETAILS.message ),
            CredDefError::CreateRevRegDefError() => write!(f, "{}", INVALID_REV_REG_DEF_CREATION.message ),
            CredDefError::InvalidRevocationEntry() => write!(f, "{}", INVALID_REV_ENTRY.message ),
        }
    }
}
impl ToErrorCode for CredDefError {
    fn to_error_code(&self) -> u32 {
        match *self {
            CredDefError::SchemaError(ref s) => 7002,
            CredDefError::BuildCredDefRequestError() => BUILD_CREDENTIAL_DEF_REQ_ERR.code_num,
            CredDefError::RetrieveCredDefError() => 7001,
            CredDefError::InvalidHandle() => INVALID_CREDENTIAL_DEF_HANDLE.code_num,
            CredDefError::CreateCredDefError() => CREATE_CREDENTIAL_DEF_ERR.code_num,
            CredDefError::NoPaymentInformation() => NO_PAYMENT_INFORMATION.code_num,
            CredDefError::CredDefAlreadyCreatedError() => CREDENTIAL_DEF_ALREADY_CREATED.code_num,
            CredDefError::InvalidRevocationDetails() => INVALID_REVOCATION_DETAILS.code_num,
            CredDefError::CreateRevRegDefError() => INVALID_REV_REG_DEF_CREATION.code_num,
            CredDefError::InvalidRevocationEntry() => INVALID_REV_ENTRY.code_num,
            CredDefError::CommonError(x) => x,
        }
    }
}

impl PartialEq for CredDefError {
    fn eq(&self, other: &CredDefError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}