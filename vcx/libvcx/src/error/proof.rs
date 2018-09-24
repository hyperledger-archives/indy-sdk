use std::fmt;
use error::ToErrorCode;
use utils::error::{INVALID_JSON, INVALID_PROOF_HANDLE, INVALID_PROOF, INVALID_PROOF_CREDENTIAL_DATA, INVALID_SCHEMA,
NOT_READY, INVALID_CONNECTION_HANDLE, CONNECTION_ERROR, FAILED_PROOF_COMPLIANCE, CREATE_PROOF_ERROR };


#[derive(Debug)]
pub enum ProofError{
    InvalidHandle(),
    InvalidProof(),
    InvalidCredData(),
    InvalidSchema(),
    ProofNotReadyError(),
    ProofMessageError(u32),
    ProofConnectionError(),
    // TODO: this could take a parameter
    CreateProofError(),
    InvalidConnection(),
    FailedProofCompliance(),
    InvalidJson(),
    CommonError(u32),
}

impl fmt::Display for ProofError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProofError::InvalidHandle() => write!(f, "{}", INVALID_PROOF_HANDLE.message),
            ProofError::InvalidProof() => write!(f, "{}", INVALID_PROOF.message),
            ProofError::InvalidSchema() => write!(f, "{}", INVALID_SCHEMA.message),
            ProofError::InvalidCredData() => write!(f, "{}", INVALID_PROOF_CREDENTIAL_DATA.message),
            ProofError::InvalidConnection() => write!(f, "{}",  CONNECTION_ERROR.message),
            ProofError::ProofNotReadyError() => write!(f, "{}", NOT_READY.message),
            ProofError::ProofConnectionError() => write!(f, "{}", INVALID_CONNECTION_HANDLE.message),
            ProofError::FailedProofCompliance() => write!(f, "{}", FAILED_PROOF_COMPLIANCE.message),
            ProofError::CreateProofError() => write!(f, "{}", CREATE_PROOF_ERROR.message),
            ProofError::ProofMessageError(x) => write!(f, "Proof Error: Message Error value: , {}", x),
            ProofError::InvalidJson() => write!(f, "{}", INVALID_JSON.message),
            ProofError::CommonError(x) => write!(f, "This Proof Error Common Error had value: {}", x),
        }
    }
}

impl PartialEq for ProofError {
    fn eq(&self, other: &ProofError) -> bool {
        self.to_error_code() == other.to_error_code()
    }
}

impl ToErrorCode for ProofError {
    fn to_error_code(&self) -> u32 {
        match *self {
            ProofError::InvalidHandle() => INVALID_PROOF_HANDLE.code_num,
            ProofError::InvalidProof() => INVALID_PROOF.code_num,
            ProofError::InvalidSchema() => INVALID_SCHEMA.code_num,
            ProofError::InvalidCredData() => INVALID_PROOF_CREDENTIAL_DATA.code_num,
            ProofError::InvalidConnection() => CONNECTION_ERROR.code_num,
            ProofError::CreateProofError() => CREATE_PROOF_ERROR.code_num,
            ProofError::ProofNotReadyError() => NOT_READY.code_num,
            ProofError::ProofConnectionError() => INVALID_CONNECTION_HANDLE.code_num,
            ProofError::FailedProofCompliance() => FAILED_PROOF_COMPLIANCE.code_num,
            ProofError::InvalidJson() => INVALID_JSON.code_num,
            ProofError::ProofMessageError(x) => x,
            ProofError::CommonError(x) => x,
        }
    }
}

