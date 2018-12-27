use std::cell;
use std::fmt;
use std::io;
use std::sync::Arc;

use failure::{Backtrace, Context, Fail};
use indy_crypto::errors::IndyCryptoError;
use log;

use api::ErrorCode;

pub mod prelude {
    pub use super::{err_msg, IndyError, IndyErrorExt, IndyErrorKind, IndyResult, IndyResultExt, ToErrorCode};
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum IndyErrorKind {
    // Common errors
    #[fail(display = "Invalid param 1")]
    InvalidParam1,
    #[fail(display = "Invalid param 2")]
    InvalidParam2,
    #[fail(display = "Invalid param 3")]
    InvalidParam3,
    #[fail(display = "Invalid param 4")]
    InvalidParam4,
    #[fail(display = "Invalid param 5")]
    InvalidParam5,
    #[fail(display = "Invalid param 6")]
    InvalidParam6,
    #[fail(display = "Invalid param 7")]
    InvalidParam7,
    #[fail(display = "Invalid param 8")]
    InvalidParam8,
    #[fail(display = "Invalid param 8")]
    InvalidParam9,
    #[fail(display = "Invalid library state")]
    InvalidState,
    #[fail(display = "Invalid structure")]
    InvalidStructure,
    #[fail(display = "IO error")]
    IOError,
    // Anoncreds errors
    #[fail(display = "Duplicated master secret")]
    MasterSecretDuplicateNameError,
    #[fail(display = "Proof rejected")]
    ProofRejected,
    #[fail(display = "Revocation registry is full")]
    RevocationRegistryFull,
    #[fail(display = "Invalid revocation id")]
    InvalidUserRevocId,
    #[fail(display = "Credential revoked")]
    CredentialRevoked,
    #[fail(display = "Credential definition already exists")]
    CredDefAlreadyExists,
    // Ledger errors
    #[fail(display = "No consensus")]
    NoConsensus,
    #[fail(display = "Invalid transaction")]
    InvalidTransaction,
    #[fail(display = "Item not found on ledger")]
    LedgerItemNotFound,
    // Pool errors
    #[fail(display = "Pool not created")]
    PoolNotCreated,
    #[fail(display = "Invalid pool handle")]
    InvalidPoolHandle,
    #[fail(display = "Pool work terminated")]
    PoolTerminated,
    #[fail(display = "Pool timeout")]
    PoolTimeout,
    #[fail(display = "Pool ledger config already exists")]
    PoolConfigAlreadyExists,
    #[fail(display = "Pool Genesis Transactions are not compatible with Protocol version")]
    PoolIncompatibleProtocolVersion,
    // Crypto errors
    #[fail(display = "Unknown crypto")]
    UnknownCrypto,
    // Wallet errors
    #[fail(display = "Invalid wallet handle was passed")]
    InvalidWalletHandle,
    #[fail(display = "Unknown wallet storage type")]
    UnknownWalletStorageType,
    #[fail(display = "Wallet storage type already registered")]
    WalletStorageTypeAlreadyRegistered,
    #[fail(display = "Wallet with this name already exists")]
    WalletAlreadyExists,
    #[fail(display = "Wallet not found")]
    WalletNotFound,
    #[fail(display = "Wallet already opened")]
    WalletAlreadyOpened,
    #[fail(display = "Wallet security error")]
    WalletAccessFailed,
    #[fail(display = "Wallet encoding error")]
    WalletEncodingError,
    #[fail(display = "Wallet storage error occurred")]
    WalletStorageError,
    #[fail(display = "Wallet encryption error")]
    WalletEncryptionError,
    #[fail(display = "Wallet item not found")]
    WalletItemNotFound,
    #[fail(display = "Wallet item already exists")]
    WalletItemAlreadyExists,
    #[fail(display = "Wallet query error")]
    WalletQueryError,
    // DID errors
    #[fail(display = "DID already exists")]
    DIDAlreadyExists,
    // Payments errors
    #[fail(display = "Unknown payment method type")]
    UnknownPaymentMethodType,
    #[fail(display = "Plugged payment method error")]
    IncompatiblePaymentMethods,
}

#[derive(Debug, Clone)]
pub struct IndyError {
    inner: Arc<Context<IndyErrorKind>>
}

impl Fail for IndyError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for IndyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl IndyError {
    pub fn from_msg<D>(kind: IndyErrorKind, msg: D) -> IndyError
        where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
        IndyError { inner: Arc::new(Context::new(msg).context(kind)) }
    }

    pub fn kind(&self) -> IndyErrorKind {
        *self.inner.get_context()
    }

    pub fn extend<D>(self, msg: D) -> IndyError
        where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
        let kind = self.kind();
        self.to_indy(kind, msg)
    }
}

pub fn err_msg<D>(kind: IndyErrorKind, msg: D) -> IndyError
    where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
    IndyError::from_msg(kind, msg)
}

impl From<IndyErrorKind> for IndyError {
    fn from(kind: IndyErrorKind) -> IndyError {
        IndyError { inner: Arc::new(Context::new(kind)) }
    }
}

impl From<Context<IndyErrorKind>> for IndyError {
    fn from(inner: Context<IndyErrorKind>) -> IndyError {
        IndyError { inner: Arc::new(inner) }
    }
}

impl From<io::Error> for IndyError {
    fn from(err: io::Error) -> Self {
        err.context(IndyErrorKind::IOError).into()
    }
}

impl From<zmq::Error> for IndyError {
    fn from(err: zmq::Error) -> Self {
        err.context(IndyErrorKind::IOError).into()
    }
}

impl From<cell::BorrowError> for IndyError {
    fn from(err: cell::BorrowError) -> Self {
        err.context(IndyErrorKind::InvalidState).into()
    }
}

impl From<cell::BorrowMutError> for IndyError {
    fn from(err: cell::BorrowMutError) -> Self {
        err.context(IndyErrorKind::InvalidState).into()
    }
}

impl From<log::SetLoggerError> for IndyError {
    fn from(err: log::SetLoggerError) -> IndyError {
        err.context(IndyErrorKind::InvalidState).into()
    }
}

impl From<IndyCryptoError> for IndyError {
    fn from(err: IndyCryptoError) -> Self {
        if let IndyCryptoError::InvalidState(err) = err {
            IndyError::from_msg(IndyErrorKind::InvalidState, err)
        } else if let IndyCryptoError::IOError(err) = err {
            IndyError::from_msg(IndyErrorKind::IOError, err)
        } else {
            IndyError::from_msg(IndyErrorKind::InvalidStructure, err)
        }
    }
}

pub trait ToErrorCode {
    fn to_error_code(&self) -> ErrorCode;
}

impl<T> ToErrorCode for Result<(), T> where T: ToErrorCode {
    fn to_error_code(&self) -> ErrorCode {
        match self {
            &Ok(()) => ErrorCode::Success,
            &Err(ref err) => err.to_error_code(),
        }
    }
}

impl ToErrorCode for IndyError {
    fn to_error_code(&self) -> ErrorCode {
        self.kind().to_error_code()
    }
}

impl ToErrorCode for IndyErrorKind {
    fn to_error_code(&self) -> ErrorCode {
        match self {
            IndyErrorKind::InvalidParam1 => ErrorCode::CommonInvalidParam1,
            IndyErrorKind::InvalidParam2 => ErrorCode::CommonInvalidParam2,
            IndyErrorKind::InvalidParam3 => ErrorCode::CommonInvalidParam3,
            IndyErrorKind::InvalidParam4 => ErrorCode::CommonInvalidParam4,
            IndyErrorKind::InvalidParam5 => ErrorCode::CommonInvalidParam5,
            IndyErrorKind::InvalidParam6 => ErrorCode::CommonInvalidParam6,
            IndyErrorKind::InvalidParam7 => ErrorCode::CommonInvalidParam7,
            IndyErrorKind::InvalidParam8 => ErrorCode::CommonInvalidParam8,
            IndyErrorKind::InvalidParam9 => ErrorCode::CommonInvalidParam9,
            IndyErrorKind::InvalidState => ErrorCode::CommonInvalidState,
            IndyErrorKind::InvalidStructure => ErrorCode::CommonInvalidStructure,
            IndyErrorKind::IOError => ErrorCode::CommonIOError,
            IndyErrorKind::MasterSecretDuplicateNameError => ErrorCode::AnoncredsMasterSecretDuplicateNameError,
            IndyErrorKind::ProofRejected => ErrorCode::AnoncredsProofRejected,
            IndyErrorKind::RevocationRegistryFull => ErrorCode::AnoncredsRevocationRegistryFullError,
            IndyErrorKind::InvalidUserRevocId => ErrorCode::AnoncredsInvalidUserRevocId,
            IndyErrorKind::CredentialRevoked => ErrorCode::AnoncredsCredentialRevoked,
            IndyErrorKind::CredDefAlreadyExists => ErrorCode::AnoncredsCredDefAlreadyExistsError,
            IndyErrorKind::NoConsensus => ErrorCode::LedgerNoConsensusError,
            IndyErrorKind::InvalidTransaction => ErrorCode::LedgerInvalidTransaction,
            IndyErrorKind::LedgerItemNotFound => ErrorCode::LedgerNotFound,
            IndyErrorKind::PoolNotCreated => ErrorCode::PoolLedgerNotCreatedError,
            IndyErrorKind::InvalidPoolHandle => ErrorCode::PoolLedgerInvalidPoolHandle,
            IndyErrorKind::PoolTerminated => ErrorCode::PoolLedgerTerminated,
            IndyErrorKind::PoolTimeout => ErrorCode::PoolLedgerTimeout,
            IndyErrorKind::PoolConfigAlreadyExists => ErrorCode::PoolLedgerConfigAlreadyExistsError,
            IndyErrorKind::PoolIncompatibleProtocolVersion => ErrorCode::PoolIncompatibleProtocolVersion,
            IndyErrorKind::UnknownCrypto => ErrorCode::UnknownCryptoTypeError,
            IndyErrorKind::InvalidWalletHandle => ErrorCode::WalletInvalidHandle,
            IndyErrorKind::UnknownWalletStorageType => ErrorCode::WalletUnknownTypeError,
            IndyErrorKind::WalletStorageTypeAlreadyRegistered => ErrorCode::WalletTypeAlreadyRegisteredError,
            IndyErrorKind::WalletAlreadyExists => ErrorCode::WalletAlreadyExistsError,
            IndyErrorKind::WalletNotFound => ErrorCode::WalletNotFoundError,
            IndyErrorKind::WalletAlreadyOpened => ErrorCode::WalletAlreadyOpenedError,
            IndyErrorKind::WalletAccessFailed => ErrorCode::WalletAccessFailed,
            IndyErrorKind::WalletEncodingError => ErrorCode::WalletDecodingError,
            IndyErrorKind::WalletStorageError => ErrorCode::WalletStorageError,
            IndyErrorKind::WalletEncryptionError => ErrorCode::WalletEncryptionError,
            IndyErrorKind::WalletItemNotFound => ErrorCode::WalletItemNotFound,
            IndyErrorKind::WalletItemAlreadyExists => ErrorCode::WalletAlreadyExistsError,
            IndyErrorKind::WalletQueryError => ErrorCode::WalletQueryError,
            IndyErrorKind::DIDAlreadyExists => ErrorCode::DidAlreadyExistsError,
            IndyErrorKind::UnknownPaymentMethodType => ErrorCode::PaymentUnknownMethodError,
            IndyErrorKind::IncompatiblePaymentMethods => ErrorCode::PaymentIncompatibleMethodsError,
        }
    }
}

impl From<ErrorCode> for IndyResult<()> {
    fn from(_err: ErrorCode) -> IndyResult<()> {
        // FIXME: Implement
        unimplemented!()
    }
}

impl From<ErrorCode> for IndyError {
    fn from(err: ErrorCode) -> IndyError {
        if err == ErrorCode::Success {
            err_msg(IndyErrorKind::InvalidState, "Try to interpret success as an error")
        } else {
            unimplemented!()
        }
    }
}


pub type IndyResult<T> = Result<T, IndyError>;

/// Extension methods for `Result`.
pub trait IndyResultExt<T, E> {
    fn to_indy<D>(self, kind: IndyErrorKind, msg: D) -> IndyResult<T> where D: fmt::Display + Send + Sync + 'static;
}

impl<T, E> IndyResultExt<T, E> for Result<T, E> where E: Fail
{
    fn to_indy<D>(self, kind: IndyErrorKind, msg: D) -> IndyResult<T> where D: fmt::Display + Send + Sync + 'static {
        self.map_err(|err| err.context(msg).context(kind).into())
    }
}

/// Extension methods for `Error`.
pub trait IndyErrorExt {
    fn to_indy<D>(self, kind: IndyErrorKind, msg: D) -> IndyError where D: fmt::Display + Send + Sync + 'static;
}

impl<E> IndyErrorExt for E where E: Fail
{
    fn to_indy<D>(self, kind: IndyErrorKind, msg: D) -> IndyError where D: fmt::Display + Send + Sync + 'static {
        self.context(msg).context(kind).into()
    }
}

