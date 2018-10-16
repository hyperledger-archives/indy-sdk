macro_rules! c_str {
    ($x:ident) => {
        ::std::ffi::CString::new($x).unwrap()
    };
    ($x:expr) => {
        ::std::ffi::CString::new($x).unwrap()
    }
}

macro_rules! opt_c_str {
    ($x:ident) => {
        $x.map(|s| ::std::ffi::CString::new(s).unwrap())
    }
}

macro_rules! rust_str {
    ($x:ident) => {
        unsafe { ::std::ffi::CStr::from_ptr($x).to_str().unwrap().to_string() }
    }
}

macro_rules! rust_slice {
    ($x:ident, $y:ident) => {
        unsafe { ::std::slice::from_raw_parts($x, $y as usize) }
    }
}

pub mod crypto;
pub mod did;
pub mod logger;
pub mod pairwise;
pub mod wallet;

#[derive(Fail, Debug)]
pub enum IndyError {
    #[fail(display = "CommonInvalidParam1")]
    CommonInvalidParam1,
    #[fail(display = "CommonInvalidParam2")]
    CommonInvalidParam2,
    #[fail(display = "CommonInvalidParam3")]
    CommonInvalidParam3,
    #[fail(display = "CommonInvalidParam4")]
    CommonInvalidParam4,
    #[fail(display = "CommonInvalidParam5")]
    CommonInvalidParam5,
    #[fail(display = "CommonInvalidParam6")]
    CommonInvalidParam6,
    #[fail(display = "CommonInvalidParam7")]
    CommonInvalidParam7,
    #[fail(display = "CommonInvalidParam8")]
    CommonInvalidParam8,
    #[fail(display = "CommonInvalidParam9")]
    CommonInvalidParam9,
    #[fail(display = "CommonInvalidParam10")]
    CommonInvalidParam10,
    #[fail(display = "CommonInvalidParam11")]
    CommonInvalidParam11,
    #[fail(display = "CommonInvalidParam12")]
    CommonInvalidParam12,
    #[fail(display = "CommonInvalidState")]
    CommonInvalidState,
    #[fail(display = "CommonInvalidStructure")]
    CommonInvalidStructure,
    #[fail(display = "CommonIOError")]
    CommonIOError,
    #[fail(display = "CommonInvalidParam13")]
    CommonInvalidParam13,
    #[fail(display = "CommonInvalidParam14")]
    CommonInvalidParam14,
    #[fail(display = "CommonInvalidParam15")]
    CommonInvalidParam15,
    #[fail(display = "CommonInvalidParam16")]
    CommonInvalidParam16,
    #[fail(display = "CommonInvalidParam17")]
    CommonInvalidParam17,
    #[fail(display = "CommonInvalidParam18")]
    CommonInvalidParam18,
    #[fail(display = "CommonInvalidParam19")]
    CommonInvalidParam19,
    #[fail(display = "CommonInvalidParam20")]
    CommonInvalidParam20,
    #[fail(display = "CommonInvalidParam21")]
    CommonInvalidParam21,
    #[fail(display = "CommonInvalidParam22")]
    CommonInvalidParam22,
    #[fail(display = "CommonInvalidParam23")]
    CommonInvalidParam23,
    #[fail(display = "CommonInvalidParam24")]
    CommonInvalidParam24,
    #[fail(display = "CommonInvalidParam25")]
    CommonInvalidParam25,
    #[fail(display = "CommonInvalidParam26")]
    CommonInvalidParam26,
    #[fail(display = "CommonInvalidParam27")]
    CommonInvalidParam27,
    #[fail(display = "WalletInvalidHandle")]
    WalletInvalidHandle,
    #[fail(display = "WalletUnknownTypeError")]
    WalletUnknownTypeError,
    #[fail(display = "WalletTypeAlreadyRegisteredError")]
    WalletTypeAlreadyRegisteredError,
    #[fail(display = "WalletAlreadyExistsError")]
    WalletAlreadyExistsError,
    #[fail(display = "WalletNotFoundError")]
    WalletNotFoundError,
    #[fail(display = "WalletIncompatiblePoolError")]
    WalletIncompatiblePoolError,
    #[fail(display = "WalletAlreadyOpenedError")]
    WalletAlreadyOpenedError,
    #[fail(display = "WalletAccessFailed")]
    WalletAccessFailed,
    #[fail(display = "WalletInputError")]
    WalletInputError,
    #[fail(display = "WalletDecodingError")]
    WalletDecodingError,
    #[fail(display = "WalletStorageError")]
    WalletStorageError,
    #[fail(display = "WalletEncryptionError")]
    WalletEncryptionError,
    #[fail(display = "WalletItemNotFound")]
    WalletItemNotFound,
    #[fail(display = "WalletItemAlreadyExists")]
    WalletItemAlreadyExists,
    #[fail(display = "WalletQueryError")]
    WalletQueryError,
    #[fail(display = "PoolLedgerNotCreatedError")]
    PoolLedgerNotCreatedError,
    #[fail(display = "PoolLedgerInvalidPoolHandle")]
    PoolLedgerInvalidPoolHandle,
    #[fail(display = "PoolLedgerTerminated")]
    PoolLedgerTerminated,
    #[fail(display = "LedgerNoConsensusError")]
    LedgerNoConsensusError,
    #[fail(display = "LedgerInvalidTransaction")]
    LedgerInvalidTransaction,
    #[fail(display = "LedgerSecurityError")]
    LedgerSecurityError,
    #[fail(display = "PoolLedgerConfigAlreadyExistsError")]
    PoolLedgerConfigAlreadyExistsError,
    #[fail(display = "PoolLedgerTimeout")]
    PoolLedgerTimeout,
    #[fail(display = "PoolIncompatibleProtocolVersion")]
    PoolIncompatibleProtocolVersion,
    #[fail(display = "AnoncredsRevocationRegistryFullError")]
    AnoncredsRevocationRegistryFullError,
    #[fail(display = "AnoncredsInvalidUserRevocIndex")]
    AnoncredsInvalidUserRevocIndex,
    #[fail(display = "AnoncredsMasterSecretDuplicateNameError")]
    AnoncredsMasterSecretDuplicateNameError,
    #[fail(display = "AnoncredsProofRejected")]
    AnoncredsProofRejected,
    #[fail(display = "AnoncredsCredentialRevoked")]
    AnoncredsCredentialRevoked,
    #[fail(display = "AnoncredsCredDefAlreadyExistsError")]
    AnoncredsCredDefAlreadyExistsError,
    #[fail(display = "UnknownCryptoTypeError")]
    UnknownCryptoTypeError,
    #[fail(display = "DidAlreadyExistsError")]
    DidAlreadyExistsError,
    #[fail(display = "UnknownPaymentMethod")]
    UnknownPaymentMethod,
    #[fail(display = "IncompatiblePaymentError")]
    IncompatiblePaymentError,
    #[fail(display = "PaymentInsufficientFundsError")]
    PaymentInsufficientFundsError,
    #[fail(display = "PaymentSourceDoesNotExistError")]
    PaymentSourceDoesNotExistError,
    #[fail(display = "PaymentExtraFundsError")]
    PaymentExtraFundsError,
}

impl IndyError {
    fn from_err_code(err_code: i32) -> IndyError {
        match err_code {
            100 => IndyError::CommonInvalidParam1,

            // Caller passed invalid value as param 2 (null, invalid json and etc..)
            101 => IndyError::CommonInvalidParam2,

            // Caller passed invalid value as param 3 (null, invalid json and etc..)
            102 => IndyError::CommonInvalidParam3,

            // Caller passed invalid value as param 4 (null, invalid json and etc..)
            103 => IndyError::CommonInvalidParam4,

            // Caller passed invalid value as param 5 (null, invalid json and etc..)
            104 => IndyError::CommonInvalidParam5,

            // Caller passed invalid value as param 6 (null, invalid json and etc..)
            105 => IndyError::CommonInvalidParam6,

            // Caller passed invalid value as param 7 (null, invalid json and etc..)
            106 => IndyError::CommonInvalidParam7,

            // Caller passed invalid value as param 8 (null, invalid json and etc..)
            107 => IndyError::CommonInvalidParam8,

            // Caller passed invalid value as param 9 (null, invalid json and etc..)
            108 => IndyError::CommonInvalidParam9,

            // Caller passed invalid value as param 10 (null, invalid json and etc..)
            109 => IndyError::CommonInvalidParam10,

            // Caller passed invalid value as param 11 (null, invalid json and etc..)
            110 => IndyError::CommonInvalidParam11,

            // Caller passed invalid value as param 11 (null, invalid json and etc..)
            111 => IndyError::CommonInvalidParam12,

            // Invalid library state was detected in runtime. It signals library bug
            112 => IndyError::CommonInvalidState,

            // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
            113 => IndyError::CommonInvalidStructure,

            // IO Error
            114 => IndyError::CommonIOError,

            // Caller passed invalid value as param 13 (null, invalid json and etc..)
            115 => IndyError::CommonInvalidParam13,

            // Caller passed invalid value as param 14 (null, invalid json and etc..)
            116 => IndyError::CommonInvalidParam14,

            // Caller passed invalid value as param 15 (null, invalid json and etc..)
            117 => IndyError::CommonInvalidParam15,

            // Caller passed invalid value as param 16 (null, invalid json and etc..)
            118 => IndyError::CommonInvalidParam16,

            // Caller passed invalid value as param 17 (null, invalid json and etc..)
            119 => IndyError::CommonInvalidParam17,

            // Caller passed invalid value as param 18 (null, invalid json and etc..)
            120 => IndyError::CommonInvalidParam18,

            // Caller passed invalid value as param 19 (null, invalid json and etc..)
            121 => IndyError::CommonInvalidParam19,

            // Caller passed invalid value as param 20 (null, invalid json and etc..)
            122 => IndyError::CommonInvalidParam20,

            // Caller passed invalid value as param 21 (null, invalid json and etc..)
            123 => IndyError::CommonInvalidParam21,

            // Caller passed invalid value as param 22 (null, invalid json and etc..)
            124 => IndyError::CommonInvalidParam22,

            // Caller passed invalid value as param 23 (null, invalid json and etc..)
            125 => IndyError::CommonInvalidParam23,

            // Caller passed invalid value as param 24 (null, invalid json and etc..)
            126 => IndyError::CommonInvalidParam24,

            // Caller passed invalid value as param 25 (null, invalid json and etc..)
            127 => IndyError::CommonInvalidParam25,

            // Caller passed invalid value as param 26 (null, invalid json and etc..)
            128 => IndyError::CommonInvalidParam26,

            // Caller passed invalid value as param 27 (null, invalid json and etc..)
            129 => IndyError::CommonInvalidParam27,

            // Wallet errors
            // Caller passed invalid wallet handle
            200 => IndyError::WalletInvalidHandle,

            // Unknown type of wallet was passed on create_wallet
            201 => IndyError::WalletUnknownTypeError,

            // Attempt to register already existing wallet type
            202 => IndyError::WalletTypeAlreadyRegisteredError,

            // Attempt to create wallet with name used for another exists wallet
            203 => IndyError::WalletAlreadyExistsError,

            // Requested entity id isn't present in wallet
            204 => IndyError::WalletNotFoundError,

            // Trying to use wallet with pool that has different name
            205 => IndyError::WalletIncompatiblePoolError,

            // Trying to open wallet that was opened already
            206 => IndyError::WalletAlreadyOpenedError,

            // Attempt to open encrypted wallet with invalid credentials
            207 => IndyError::WalletAccessFailed,

            // Input provided to wallet operations is considered not valid
            208 => IndyError::WalletInputError,

            // Decoding of wallet data during input/output failed
            209 => IndyError::WalletDecodingError,

            // Storage error occurred during wallet operation
            210 => IndyError::WalletStorageError,

            // Error during encryption-related operations
            211 => IndyError::WalletEncryptionError,

            // Requested wallet item not found
            212 => IndyError::WalletItemNotFound,

            // Returned if wallet's add_record operation is used with record name that already exists
            213 => IndyError::WalletItemAlreadyExists,

            // Returned if provided wallet query is invalid
            214 => IndyError::WalletQueryError,

            // Ledger errors
            // Trying to open pool ledger that wasn't created before
            300 => IndyError::PoolLedgerNotCreatedError,

            // Caller passed invalid pool ledger handle
            301 => IndyError::PoolLedgerInvalidPoolHandle,

            // Pool ledger terminated
            302 => IndyError::PoolLedgerTerminated,

            // No concensus during ledger operation
            303 => IndyError::LedgerNoConsensusError,

            // Attempt to parse invalid transaction response
            304 => IndyError::LedgerInvalidTransaction,

            // Attempt to send transaction without the necessary privileges
            305 => IndyError::LedgerSecurityError,

            // Attempt to create pool ledger config with name used for another existing pool
            306 => IndyError::PoolLedgerConfigAlreadyExistsError,

            // Timeout for action
            307 => IndyError::PoolLedgerTimeout,

            // Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol version.
            // Call pool.indy_set_protocol_version to set correct Protocol version.
            308 => IndyError::PoolIncompatibleProtocolVersion,

            // Revocation registry is full and creation of new registry is necessary
            400 => IndyError::AnoncredsRevocationRegistryFullError,

            401 => IndyError::AnoncredsInvalidUserRevocIndex,

            // Attempt to generate master secret with duplicated name
            404 => IndyError::AnoncredsMasterSecretDuplicateNameError,

            405 => IndyError::AnoncredsProofRejected,

            406 => IndyError::AnoncredsCredentialRevoked,

            // Attempt to create credential definition with duplicated did schema pair
            407 => IndyError::AnoncredsCredDefAlreadyExistsError,

            // Signus errors
            // Unknown format of DID entity keys
            500 => IndyError::UnknownCryptoTypeError,

            // Attempt to create duplicate did
            600 => IndyError::DidAlreadyExistsError,

            // Unknown payment method was given
            700 => IndyError::UnknownPaymentMethod,

            //No method were scraped from inputs/outputs or more than one were scraped
            701 => IndyError::IncompatiblePaymentError,

            // Insufficient funds on inputs
            702 => IndyError::PaymentInsufficientFundsError,

            // No such source on a ledger
            703 => IndyError::PaymentSourceDoesNotExistError,

            // Extra funds on inputs
            705 => IndyError::PaymentExtraFundsError,

            _ => panic!("Unknown libindy error")
        }
    }
}



