// Create the Error ErrorKind ResultExt and Result types
error_chain! {
    errors {
        CommonInvalidParam1
        CommonInvalidParam2
        CommonInvalidParam3
        CommonInvalidParam4
        CommonInvalidParam5
        CommonInvalidParam6
        CommonInvalidParam7
        CommonInvalidParam8
        CommonInvalidParam9
        CommonInvalidParam10
        CommonInvalidParam11
        CommonInvalidParam12
        CommonInvalidState
        CommonInvalidStructure
        CommonIOError
        CommonInvalidParam13
        CommonInvalidParam14
        CommonInvalidParam15
        CommonInvalidParam16
        CommonInvalidParam17
        CommonInvalidParam18
        CommonInvalidParam19
        CommonInvalidParam20
        CommonInvalidParam21
        CommonInvalidParam22
        CommonInvalidParam23
        CommonInvalidParam24
        CommonInvalidParam25
        CommonInvalidParam26
        CommonInvalidParam27
        WalletInvalidHandle
        WalletUnknownTypeError
        WalletTypeAlreadyRegisteredError
        WalletAlreadyExistsError
        WalletNotFoundError
        WalletIncompatiblePoolError
        WalletAlreadyOpenedError
        WalletAccessFailed
        WalletInputError
        WalletDecodingError
        WalletStorageError
        WalletEncryptionError
        WalletItemNotFound
        WalletItemAlreadyExists
        WalletQueryError
        PoolLedgerNotCreatedError
        PoolLedgerInvalidPoolHandle
        PoolLedgerTerminated
        LedgerNoConsensusError
        LedgerInvalidTransaction
        LedgerSecurityError
        PoolLedgerConfigAlreadyExistsError
        PoolLedgerTimeout
        PoolIncompatibleProtocolVersion
        AnoncredsRevocationRegistryFullError
        AnoncredsInvalidUserRevocIndex
        AnoncredsMasterSecretDuplicateNameError
        AnoncredsProofRejected
        AnoncredsCredentialRevoked
        AnoncredsCredDefAlreadyExistsError
        UnknownCryptoTypeError
        DidAlreadyExistsError
        UnknownPaymentMethod
        IncompatiblePaymentError
        PaymentInsufficientFundsError
        PaymentSourceDoesNotExistError
        PaymentExtraFundsError
    }
}

impl ErrorKind {
    pub fn from_err_code(err_code: i32) -> ErrorKind {
        match err_code {
            100 => ErrorKind::CommonInvalidParam1,

            // Caller passed invalid value as param 2 (null, invalid json and etc..)
            101 => ErrorKind::CommonInvalidParam2,

            // Caller passed invalid value as param 3 (null, invalid json and etc..)
            102 => ErrorKind::CommonInvalidParam3,

            // Caller passed invalid value as param 4 (null, invalid json and etc..)
            103 => ErrorKind::CommonInvalidParam4,

            // Caller passed invalid value as param 5 (null, invalid json and etc..)
            104 => ErrorKind::CommonInvalidParam5,

            // Caller passed invalid value as param 6 (null, invalid json and etc..)
            105 => ErrorKind::CommonInvalidParam6,

            // Caller passed invalid value as param 7 (null, invalid json and etc..)
            106 => ErrorKind::CommonInvalidParam7,

            // Caller passed invalid value as param 8 (null, invalid json and etc..)
            107 => ErrorKind::CommonInvalidParam8,

            // Caller passed invalid value as param 9 (null, invalid json and etc..)
            108 => ErrorKind::CommonInvalidParam9,

            // Caller passed invalid value as param 10 (null, invalid json and etc..)
            109 => ErrorKind::CommonInvalidParam10,

            // Caller passed invalid value as param 11 (null, invalid json and etc..)
            110 => ErrorKind::CommonInvalidParam11,

            // Caller passed invalid value as param 11 (null, invalid json and etc..)
            111 => ErrorKind::CommonInvalidParam12,

            // Invalid library state was detected in runtime. It signals library bug
            112 => ErrorKind::CommonInvalidState,

            // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
            113 => ErrorKind::CommonInvalidStructure,

            // IO Error
            114 => ErrorKind::CommonIOError,

            // Caller passed invalid value as param 13 (null, invalid json and etc..)
            115 => ErrorKind::CommonInvalidParam13,

            // Caller passed invalid value as param 14 (null, invalid json and etc..)
            116 => ErrorKind::CommonInvalidParam14,

            // Caller passed invalid value as param 15 (null, invalid json and etc..)
            117 => ErrorKind::CommonInvalidParam15,

            // Caller passed invalid value as param 16 (null, invalid json and etc..)
            118 => ErrorKind::CommonInvalidParam16,

            // Caller passed invalid value as param 17 (null, invalid json and etc..)
            119 => ErrorKind::CommonInvalidParam17,

            // Caller passed invalid value as param 18 (null, invalid json and etc..)
            120 => ErrorKind::CommonInvalidParam18,

            // Caller passed invalid value as param 19 (null, invalid json and etc..)
            121 => ErrorKind::CommonInvalidParam19,

            // Caller passed invalid value as param 20 (null, invalid json and etc..)
            122 => ErrorKind::CommonInvalidParam20,

            // Caller passed invalid value as param 21 (null, invalid json and etc..)
            123 => ErrorKind::CommonInvalidParam21,

            // Caller passed invalid value as param 22 (null, invalid json and etc..)
            124 => ErrorKind::CommonInvalidParam22,

            // Caller passed invalid value as param 23 (null, invalid json and etc..)
            125 => ErrorKind::CommonInvalidParam23,

            // Caller passed invalid value as param 24 (null, invalid json and etc..)
            126 => ErrorKind::CommonInvalidParam24,

            // Caller passed invalid value as param 25 (null, invalid json and etc..)
            127 => ErrorKind::CommonInvalidParam25,

            // Caller passed invalid value as param 26 (null, invalid json and etc..)
            128 => ErrorKind::CommonInvalidParam26,

            // Caller passed invalid value as param 27 (null, invalid json and etc..)
            129 => ErrorKind::CommonInvalidParam27,

            // Wallet errors
            // Caller passed invalid wallet handle
            200 => ErrorKind::WalletInvalidHandle,

            // Unknown type of wallet was passed on create_wallet
            201 => ErrorKind::WalletUnknownTypeError,

            // Attempt to register already existing wallet type
            202 => ErrorKind::WalletTypeAlreadyRegisteredError,

            // Attempt to create wallet with name used for another exists wallet
            203 => ErrorKind::WalletAlreadyExistsError,

            // Requested entity id isn't present in wallet
            204 => ErrorKind::WalletNotFoundError,

            // Trying to use wallet with pool that has different name
            205 => ErrorKind::WalletIncompatiblePoolError,

            // Trying to open wallet that was opened already
            206 => ErrorKind::WalletAlreadyOpenedError,

            // Attempt to open encrypted wallet with invalid credentials
            207 => ErrorKind::WalletAccessFailed,

            // Input provided to wallet operations is considered not valid
            208 => ErrorKind::WalletInputError,

            // Decoding of wallet data during input/output failed
            209 => ErrorKind::WalletDecodingError,

            // Storage error occurred during wallet operation
            210 => ErrorKind::WalletStorageError,

            // Error during encryption-related operations
            211 => ErrorKind::WalletEncryptionError,

            // Requested wallet item not found
            212 => ErrorKind::WalletItemNotFound,

            // Returned if wallet's add_record operation is used with record name that already exists
            213 => ErrorKind::WalletItemAlreadyExists,

            // Returned if provided wallet query is invalid
            214 => ErrorKind::WalletQueryError,

            // Ledger errors
            // Trying to open pool ledger that wasn't created before
            300 => ErrorKind::PoolLedgerNotCreatedError,

            // Caller passed invalid pool ledger handle
            301 => ErrorKind::PoolLedgerInvalidPoolHandle,

            // Pool ledger terminated
            302 => ErrorKind::PoolLedgerTerminated,

            // No concensus during ledger operation
            303 => ErrorKind::LedgerNoConsensusError,

            // Attempt to parse invalid transaction response
            304 => ErrorKind::LedgerInvalidTransaction,

            // Attempt to send transaction without the necessary privileges
            305 => ErrorKind::LedgerSecurityError,

            // Attempt to create pool ledger config with name used for another existing pool
            306 => ErrorKind::PoolLedgerConfigAlreadyExistsError,

            // Timeout for action
            307 => ErrorKind::PoolLedgerTimeout,

            // Attempt to open Pool for witch Genesis Transactions are not compatible with set Protocol version.
            // Call pool.indy_set_protocol_version to set correct Protocol version.
            308 => ErrorKind::PoolIncompatibleProtocolVersion,

            // Revocation registry is full and creation of new registry is necessary
            400 => ErrorKind::AnoncredsRevocationRegistryFullError,

            401 => ErrorKind::AnoncredsInvalidUserRevocIndex,

            // Attempt to generate master secret with duplicated name
            404 => ErrorKind::AnoncredsMasterSecretDuplicateNameError,

            405 => ErrorKind::AnoncredsProofRejected,

            406 => ErrorKind::AnoncredsCredentialRevoked,

            // Attempt to create credential definition with duplicated did schema pair
            407 => ErrorKind::AnoncredsCredDefAlreadyExistsError,

            // Signus errors
            // Unknown format of DID entity keys
            500 => ErrorKind::UnknownCryptoTypeError,

            // Attempt to create duplicate did
            600 => ErrorKind::DidAlreadyExistsError,

            // Unknown payment method was given
            700 => ErrorKind::UnknownPaymentMethod,

            //No method were scraped from inputs/outputs or more than one were scraped
            701 => ErrorKind::IncompatiblePaymentError,

            // Insufficient funds on inputs
            702 => ErrorKind::PaymentInsufficientFundsError,

            // No such source on a ledger
            703 => ErrorKind::PaymentSourceDoesNotExistError,

            // Extra funds on inputs
            705 => ErrorKind::PaymentExtraFundsError,

            _ => panic!("Unknown libindy error")
        }
    }
}