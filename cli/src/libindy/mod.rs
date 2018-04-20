pub mod did;
pub mod pool;
pub mod wallet;
pub mod ledger;
mod callbacks;
mod results;

pub type IndyHandle = i32;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ErrorCode
{
    Success = 0,

    // Common errors

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1 = 100,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2 = 101,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3 = 102,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4 = 103,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5 = 104,

    // Caller passed invalid value as param 6 (null, invalid json and etc..)
    CommonInvalidParam6 = 105,

    // Caller passed invalid value as param 7 (null, invalid json and etc..)
    CommonInvalidParam7 = 106,

    // Caller passed invalid value as param 8 (null, invalid json and etc..)
    CommonInvalidParam8 = 107,

    // Caller passed invalid value as param 9 (null, invalid json and etc..)
    CommonInvalidParam9 = 108,

    // Caller passed invalid value as param 10 (null, invalid json and etc..)
    CommonInvalidParam10 = 109,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam11 = 110,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam12 = 111,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState = 112,

    // Object (json, config, key, credential and etc...) passed by library caller has invalid structure
    CommonInvalidStructure = 113,

    // IO Error
    CommonIOError = 114,

    // Wallet errors
    // Caller passed invalid wallet handle
    WalletInvalidHandle = 200,

    // Unknown type of wallet was passed on create_wallet
    WalletUnknownTypeError = 201,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegisteredError = 202,

    // Attempt to create wallet with name used for another exists wallet
    WalletAlreadyExistsError = 203,

    // Requested entity id isn't present in wallet
    WalletNotFoundError = 204,

    // Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError = 205,

    // Trying to open wallet that was opened already
    WalletAlreadyOpenedError = 206,

    // Attempt to open encrypted wallet with invalid credentials
    WalletAccessFailed = 207,

    // Ledger errors
    // Trying to open pool ledger that wasn't created before
    PoolLedgerNotCreatedError = 300,

    // Caller passed invalid pool ledger handle
    PoolLedgerInvalidPoolHandle = 301,

    // Pool ledger terminated
    PoolLedgerTerminated = 302,

    // No concensus during ledger operation
    LedgerNoConsensusError = 303,

    // Attempt to send transaction without the necessary privileges
    LedgerSecurityError = 305,

    // Attempt to create pool ledger config with name used for another existing pool
    PoolLedgerConfigAlreadyExistsError = 306,

    // Timeout for action
    PoolLedgerTimeout = 307,

    // Revocation registry is full and creation of new registry is necessary
    AnoncredsRevocationRegistryFullError = 400,

    AnoncredsInvalidUserRevocIndex = 401,

    AnoncredsAccumulatorIsFull = 402,

    AnoncredsNotIssuedError = 403,

    // Attempt to generate master secret with duplicated name
    AnoncredsMasterSecretDuplicateNameError = 404,

    AnoncredsProofRejected = 405,

    AnoncredsCredentialRevoked = 406,

    // Attempt to create credential definition with duplicated did schema pair
    AnoncredsCredDefAlreadyExistsError = 407,

    // Signus errors
    // Unknown format of DID entity keys
    UnknownCryptoTypeError = 500,

    // Attempt to create duplicate did
    DidAlreadyExistsError = 600
}

impl ErrorCode {
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #[allow(unused)]
    pub fn description(&self) -> &'static str {
        match self {
            CommonInvalidParam1 => "Caller passed invalid value as param 1",
            CommonInvalidParam2 => "Caller passed invalid value as param 2",
            CommonInvalidParam3 => "Caller passed invalid value as param 3",
            CommonInvalidParam4 => "Caller passed invalid value as param 4",
            CommonInvalidParam5 => "Caller passed invalid value as param 5",
            CommonInvalidParam6 => "Caller passed invalid value as param 6",
            CommonInvalidParam7 => "Caller passed invalid value as param 7",
            CommonInvalidParam8 => "Caller passed invalid value as param 8",
            CommonInvalidParam9 => "Caller passed invalid value as param 9",
            CommonInvalidParam10 => "Caller passed invalid value as param 10",
            CommonInvalidParam11 => "Caller passed invalid value as param 11",
            CommonInvalidParam12 => "Caller passed invalid value as param 12",
            CommonInvalidState => "Invalid library state was detected in runtime. It signals library bug",
            CommonInvalidStructure => "Object (json, config, key, credential and etc...) passed by library caller has invalid structure",
            CommonIOError => "IO Error",
            WalletInvalidHandle => "Caller passed invalid wallet handle",
            WalletUnknownTypeError => "Caller passed invalid wallet handle",
            WalletTypeAlreadyRegisteredError => "Attempt to register already existing wallet type",
            WalletAlreadyExistsError => "Attempt to create wallet with name used for another exists wallet",
            WalletNotFoundError => "Requested entity id isn't present in wallet",
            WalletIncompatiblePoolError => "Trying to use wallet with pool that has different name",
            WalletAccessFailed => "Trying to open wallet encrypted wallet with invalid credentials",
            WalletAlreadyOpenedError => "Trying to open wallet that was opened already",
            PoolLedgerNotCreatedError => "Trying to open pool ledger that wasn't created before",
            PoolLedgerInvalidPoolHandle => "Caller passed invalid pool ledger handle",
            PoolLedgerTerminated => "Pool ledger terminated",
            LedgerNoConsensusError => "No concensus during ledger operation",
            LedgerInvalidTransaction => "Attempt to send unknown or incomplete transaction message",
            LedgerSecurityError => "Attempt to send transaction without the necessary privileges",
            PoolLedgerConfigAlreadyExistsError => "Attempt to create pool ledger config with name used for another existing pool",
            AnoncredsRevocationRegistryFullError => "Revocation registry is full and creation of new registry is necessary",
            AnoncredsInvalidUserRevocIndex => "Invalid user revocation index",
            AnoncredsAccumulatorIsFull => "Revocation accumulator is full",
            AnoncredsNotIssuedError => "Not issued",
            AnoncredsMasterSecretDuplicateNameError => "Attempt to generate master secret with duplicated name",
            AnoncredsProofRejected => "Proof rejected",
            AnoncredsCredentialRevoked => "Credential revoked",
            AnoncredsCredDefAlreadyExistsError => "Credential definition already exists",
            UnknownCryptoTypeError => "Unknown format of DID entity keys",
            DidAlreadyExistsError => "Did already exists",
        }
    }
}