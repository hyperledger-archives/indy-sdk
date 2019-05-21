use std::cell;
use std::fmt;
use std::io;
use std::sync::Arc;
use std::ffi::CString;
use std::cell::RefCell;
use std::ptr;


use failure::{Backtrace, Context, Fail};
use ursa::errors::{UrsaCryptoError, UrsaCryptoErrorKind};
use log;
use libc::c_char;

use api::ErrorCode;
use utils::ctypes;

pub mod prelude {
    pub use super::{err_msg, IndyError, IndyErrorExt, IndyErrorKind, IndyResult, IndyResultExt, set_current_error, get_current_error_c_json};
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum IndyErrorKind {
    // Common errors
    #[fail(display = "Invalid library state")]
    InvalidState,
    #[fail(display = "Invalid structure")]
    InvalidStructure,
    #[fail(display = "Invalid parameter {}", 0)]
    InvalidParam(u32),
    #[fail(display = "IO error")]
    IOError,
    // Anoncreds errors
    #[fail(display = "Duplicated master secret")]
    MasterSecretDuplicateName,
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
    #[fail(display = "No method were scraped from inputs/outputs or more than one were scraped")]
    IncompatiblePaymentMethods,
    #[fail(display = "Payment insufficient funds on inputs")]
    PaymentInsufficientFunds,
    #[fail(display = "Payment Source does not exist")]
    PaymentSourceDoesNotExist,
    #[fail(display = "Payment operation not supported")]
    PaymentOperationNotSupported,
    #[fail(display = "Payment extra funds")]
    PaymentExtraFunds,
}

#[derive(Debug, Clone)]
pub struct IndyError {
    // FIXME: We have to use Arc as for now we clone messages in pool service
    // FIXME: In theory we can avoid sync by refactoring of pool service
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
        let mut first = true;

        for cause in Fail::iter_chain(self.inner.as_ref()) {
            if first {
                first = false;
                writeln!(f, "Error: {}", cause)?;
            } else {
                writeln!(f, "  Caused by: {}", cause)?;
            }
        }

        Ok(())
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
        let inner = Arc::try_unwrap(self.inner).unwrap();
        IndyError { inner: Arc::new(inner.map(|_| msg).context(kind)) }
    }

    pub fn map<D>(self, kind: IndyErrorKind, msg: D) -> IndyError
        where D: fmt::Display + fmt::Debug + Send + Sync + 'static {
        let inner = Arc::try_unwrap(self.inner).unwrap();
        IndyError { inner: Arc::new(inner.map(|_| msg).context(kind)) }
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

impl From<UrsaCryptoError> for IndyError {
    fn from(err: UrsaCryptoError) -> Self {
        let message = format!("UrsaCryptoError: {}", Fail::iter_causes(&err).map(|e| e.to_string()).collect::<String>());

        match err.kind() {
            UrsaCryptoErrorKind::InvalidState => IndyError::from_msg(IndyErrorKind::InvalidState, message),
            UrsaCryptoErrorKind::InvalidStructure => IndyError::from_msg(IndyErrorKind::InvalidStructure, message),
            UrsaCryptoErrorKind::IOError => IndyError::from_msg(IndyErrorKind::IOError, message),
            UrsaCryptoErrorKind::InvalidRevocationAccumulatorIndex => IndyError::from_msg(IndyErrorKind::InvalidUserRevocId, message),
            UrsaCryptoErrorKind::RevocationAccumulatorIsFull => IndyError::from_msg(IndyErrorKind::RevocationRegistryFull, message),
            UrsaCryptoErrorKind::ProofRejected => IndyError::from_msg(IndyErrorKind::ProofRejected, message),
            UrsaCryptoErrorKind::CredentialRevoked => IndyError::from_msg(IndyErrorKind::CredentialRevoked, message),
            UrsaCryptoErrorKind::InvalidParam(_) => IndyError::from_msg(IndyErrorKind::InvalidStructure, message),
        }
    }
}

impl<T> From<IndyResult<T>> for ErrorCode {
    fn from(r: Result<T, IndyError>) -> ErrorCode {
        match r {
            Ok(_) => ErrorCode::Success,
            Err(err) => err.into(),
        }
    }
}

impl From<IndyError> for ErrorCode {
    fn from(err: IndyError) -> ErrorCode {
        set_current_error(&err);
        err.kind().into()
    }
}

impl From<IndyErrorKind> for ErrorCode {
    fn from(code: IndyErrorKind) -> ErrorCode {
        match code {
            IndyErrorKind::InvalidState => ErrorCode::CommonInvalidState,
            IndyErrorKind::InvalidStructure => ErrorCode::CommonInvalidStructure,
            IndyErrorKind::InvalidParam(num) =>
                match num {
                    1 => ErrorCode::CommonInvalidParam1,
                    2 => ErrorCode::CommonInvalidParam2,
                    3 => ErrorCode::CommonInvalidParam3,
                    4 => ErrorCode::CommonInvalidParam4,
                    5 => ErrorCode::CommonInvalidParam5,
                    6 => ErrorCode::CommonInvalidParam6,
                    7 => ErrorCode::CommonInvalidParam7,
                    8 => ErrorCode::CommonInvalidParam8,
                    9 => ErrorCode::CommonInvalidParam9,
                    10 => ErrorCode::CommonInvalidParam10,
                    11 => ErrorCode::CommonInvalidParam11,
                    12 => ErrorCode::CommonInvalidParam12,
                    13 => ErrorCode::CommonInvalidParam13,
                    14 => ErrorCode::CommonInvalidParam14,
                    15 => ErrorCode::CommonInvalidParam15,
                    16 => ErrorCode::CommonInvalidParam16,
                    17 => ErrorCode::CommonInvalidParam17,
                    18 => ErrorCode::CommonInvalidParam18,
                    19 => ErrorCode::CommonInvalidParam19,
                    20 => ErrorCode::CommonInvalidParam20,
                    21 => ErrorCode::CommonInvalidParam21,
                    22 => ErrorCode::CommonInvalidParam22,
                    23 => ErrorCode::CommonInvalidParam23,
                    24 => ErrorCode::CommonInvalidParam24,
                    25 => ErrorCode::CommonInvalidParam25,
                    26 => ErrorCode::CommonInvalidParam26,
                    27 => ErrorCode::CommonInvalidParam27,
                    _ => ErrorCode::CommonInvalidState
                },
            IndyErrorKind::IOError => ErrorCode::CommonIOError,
            IndyErrorKind::MasterSecretDuplicateName => ErrorCode::AnoncredsMasterSecretDuplicateNameError,
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
            IndyErrorKind::WalletItemAlreadyExists => ErrorCode::WalletItemAlreadyExists,
            IndyErrorKind::WalletQueryError => ErrorCode::WalletQueryError,
            IndyErrorKind::DIDAlreadyExists => ErrorCode::DidAlreadyExistsError,
            IndyErrorKind::UnknownPaymentMethodType => ErrorCode::PaymentUnknownMethodError,
            IndyErrorKind::IncompatiblePaymentMethods => ErrorCode::PaymentIncompatibleMethodsError,
            IndyErrorKind::PaymentInsufficientFunds => ErrorCode::PaymentInsufficientFundsError,
            IndyErrorKind::PaymentSourceDoesNotExist => ErrorCode::PaymentSourceDoesNotExistError,
            IndyErrorKind::PaymentOperationNotSupported => ErrorCode::PaymentOperationNotSupportedError,
            IndyErrorKind::PaymentExtraFunds => ErrorCode::PaymentExtraFundsError,
        }
    }
}

impl From<ErrorCode> for IndyResult<()> {
    fn from(err: ErrorCode) -> IndyResult<()> {
        if err == ErrorCode::Success {
            Ok(())
        } else {
            Err(err.into())
        }
    }
}

impl From<ErrorCode> for IndyError {
    fn from(err: ErrorCode) -> IndyError {
        err_msg(err.into(), format!("Plugin returned error"))
    }
}

impl From<ErrorCode> for IndyErrorKind {
    fn from(err: ErrorCode) -> IndyErrorKind {
        match err {
            ErrorCode::CommonInvalidState => IndyErrorKind::InvalidState,
            ErrorCode::CommonInvalidStructure => IndyErrorKind::InvalidStructure,
            ErrorCode::CommonInvalidParam1 => IndyErrorKind::InvalidParam(1),
            ErrorCode::CommonInvalidParam2 => IndyErrorKind::InvalidParam(2),
            ErrorCode::CommonInvalidParam3 => IndyErrorKind::InvalidParam(3),
            ErrorCode::CommonInvalidParam4 => IndyErrorKind::InvalidParam(4),
            ErrorCode::CommonInvalidParam5 => IndyErrorKind::InvalidParam(5),
            ErrorCode::CommonInvalidParam6 => IndyErrorKind::InvalidParam(6),
            ErrorCode::CommonInvalidParam7 => IndyErrorKind::InvalidParam(7),
            ErrorCode::CommonInvalidParam8 => IndyErrorKind::InvalidParam(8),
            ErrorCode::CommonInvalidParam9 => IndyErrorKind::InvalidParam(9),
            ErrorCode::CommonInvalidParam10 => IndyErrorKind::InvalidParam(10),
            ErrorCode::CommonInvalidParam11 => IndyErrorKind::InvalidParam(11),
            ErrorCode::CommonInvalidParam12 => IndyErrorKind::InvalidParam(12),
            ErrorCode::CommonInvalidParam13 => IndyErrorKind::InvalidParam(13),
            ErrorCode::CommonInvalidParam14 => IndyErrorKind::InvalidParam(14),
            ErrorCode::CommonInvalidParam15 => IndyErrorKind::InvalidParam(15),
            ErrorCode::CommonInvalidParam16 => IndyErrorKind::InvalidParam(16),
            ErrorCode::CommonInvalidParam17 => IndyErrorKind::InvalidParam(17),
            ErrorCode::CommonInvalidParam18 => IndyErrorKind::InvalidParam(18),
            ErrorCode::CommonInvalidParam19 => IndyErrorKind::InvalidParam(19),
            ErrorCode::CommonInvalidParam20 => IndyErrorKind::InvalidParam(20),
            ErrorCode::CommonInvalidParam21 => IndyErrorKind::InvalidParam(21),
            ErrorCode::CommonInvalidParam22 => IndyErrorKind::InvalidParam(22),
            ErrorCode::CommonInvalidParam23 => IndyErrorKind::InvalidParam(23),
            ErrorCode::CommonInvalidParam24 => IndyErrorKind::InvalidParam(24),
            ErrorCode::CommonInvalidParam25 => IndyErrorKind::InvalidParam(25),
            ErrorCode::CommonInvalidParam26 => IndyErrorKind::InvalidParam(26),
            ErrorCode::CommonInvalidParam27 => IndyErrorKind::InvalidParam(27),
            ErrorCode::CommonIOError => IndyErrorKind::IOError,
            ErrorCode::AnoncredsMasterSecretDuplicateNameError => IndyErrorKind::MasterSecretDuplicateName,
            ErrorCode::AnoncredsProofRejected => IndyErrorKind::ProofRejected,
            ErrorCode::AnoncredsRevocationRegistryFullError => IndyErrorKind::RevocationRegistryFull,
            ErrorCode::AnoncredsInvalidUserRevocId => IndyErrorKind::InvalidUserRevocId,
            ErrorCode::AnoncredsCredentialRevoked => IndyErrorKind::CredentialRevoked,
            ErrorCode::AnoncredsCredDefAlreadyExistsError => IndyErrorKind::CredDefAlreadyExists,
            ErrorCode::LedgerNoConsensusError => IndyErrorKind::NoConsensus,
            ErrorCode::LedgerInvalidTransaction => IndyErrorKind::InvalidTransaction,
            ErrorCode::LedgerNotFound => IndyErrorKind::LedgerItemNotFound,
            ErrorCode::PoolLedgerNotCreatedError => IndyErrorKind::PoolNotCreated,
            ErrorCode::PoolLedgerInvalidPoolHandle => IndyErrorKind::InvalidPoolHandle,
            ErrorCode::PoolLedgerTerminated => IndyErrorKind::PoolTerminated,
            ErrorCode::PoolLedgerTimeout => IndyErrorKind::PoolTimeout,
            ErrorCode::PoolLedgerConfigAlreadyExistsError => IndyErrorKind::PoolConfigAlreadyExists,
            ErrorCode::PoolIncompatibleProtocolVersion => IndyErrorKind::PoolIncompatibleProtocolVersion,
            ErrorCode::UnknownCryptoTypeError => IndyErrorKind::UnknownCrypto,
            ErrorCode::WalletInvalidHandle => IndyErrorKind::InvalidWalletHandle,
            ErrorCode::WalletUnknownTypeError => IndyErrorKind::UnknownWalletStorageType,
            ErrorCode::WalletTypeAlreadyRegisteredError => IndyErrorKind::WalletStorageTypeAlreadyRegistered,
            ErrorCode::WalletAlreadyExistsError => IndyErrorKind::WalletAlreadyExists,
            ErrorCode::WalletNotFoundError => IndyErrorKind::WalletNotFound,
            ErrorCode::WalletAlreadyOpenedError => IndyErrorKind::WalletAlreadyOpened,
            ErrorCode::WalletAccessFailed => IndyErrorKind::WalletAccessFailed,
            ErrorCode::WalletDecodingError => IndyErrorKind::WalletEncodingError,
            ErrorCode::WalletStorageError => IndyErrorKind::WalletStorageError,
            ErrorCode::WalletEncryptionError => IndyErrorKind::WalletEncryptionError,
            ErrorCode::WalletItemNotFound => IndyErrorKind::WalletItemNotFound,
            ErrorCode::WalletItemAlreadyExists => IndyErrorKind::WalletItemAlreadyExists,
            ErrorCode::WalletQueryError => IndyErrorKind::WalletQueryError,
            ErrorCode::DidAlreadyExistsError => IndyErrorKind::DIDAlreadyExists,
            ErrorCode::PaymentUnknownMethodError => IndyErrorKind::UnknownPaymentMethodType,
            ErrorCode::PaymentIncompatibleMethodsError => IndyErrorKind::IncompatiblePaymentMethods,
            ErrorCode::PaymentInsufficientFundsError => IndyErrorKind::PaymentInsufficientFunds,
            ErrorCode::PaymentSourceDoesNotExistError => IndyErrorKind::PaymentSourceDoesNotExist,
            ErrorCode::PaymentOperationNotSupportedError => IndyErrorKind::PaymentOperationNotSupported,
            ErrorCode::PaymentExtraFundsError => IndyErrorKind::PaymentExtraFunds,
            _code => IndyErrorKind::InvalidState
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

thread_local! {
    pub static CURRENT_ERROR_C_JSON: RefCell<Option<CString>> = RefCell::new(None);
}

pub fn set_current_error(err: &IndyError) {
    CURRENT_ERROR_C_JSON.try_with(|error| {
        let error_json = json!({
            "message": err.to_string(),
            "backtrace": err.backtrace().map(|bt| bt.to_string())
        }).to_string();
        error.replace(Some(ctypes::string_to_cstring(error_json)));
    })
        .map_err(|err| error!("Thread local variable access failed with: {:?}", err)).ok();
}

pub fn get_current_error_c_json() -> *const c_char {
    let mut value = ptr::null();

    CURRENT_ERROR_C_JSON.try_with(|err|
        err.borrow().as_ref().map(|err| value = err.as_ptr())
    )
        .map_err(|err| error!("Thread local variable access failed with: {:?}", err)).ok();

    value
}
