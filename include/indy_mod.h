#ifndef __indy__mod_included__
#define __indy__mod_included__

typedef enum
{
    Success = 0,

    // Common errors

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1 = 100,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5,

    // Caller passed invalid value as param 6 (null, invalid json and etc..)
    CommonInvalidParam6,

    // Caller passed invalid value as param 7 (null, invalid json and etc..)
    CommonInvalidParam7,

    // Caller passed invalid value as param 8 (null, invalid json and etc..)
    CommonInvalidParam8,

    // Caller passed invalid value as param 9 (null, invalid json and etc..)
    CommonInvalidParam9,

    // Caller passed invalid value as param 10 (null, invalid json and etc..)
    CommonInvalidParam10,

    // Caller passed invalid value as param 11 (null, invalid json and etc..)
    CommonInvalidParam11,

    // Caller passed invalid value as param 12 (null, invalid json and etc..)
    CommonInvalidParam12,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState,

    // Object (json, config, key, claim and etc...) passed by library caller has invalid structure
    CommonInvalidStructure,

    // IO Error
    CommonIOError,

    // Wallet errors
    // Caller passed invalid wallet handle
    WalletInvalidHandle = 200,

    // Unknown type of wallet was passed on create_wallet
    WalletUnknownTypeError,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegisteredError,

    // Attempt to create wallet with name used for another exists wallet
    WalletAlreadyExistsError,

    // Requested entity id isn't present in wallet
    WalletNotFoundError,

    // Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError,

    // Trying to open wallet that was opened already
    WalletAlreadyOpenedError,

    // Ledger errors
    // Trying to open pool ledger that wasn't created before
    PoolLedgerNotCreatedError = 300,

    // Caller passed invalid pool ledger handle
    PoolLedgerInvalidPoolHandle,

    // Pool ledger terminated
    PoolLedgerTerminated,

    // No concensus during ledger operation
    LedgerNoConsensusError,

    // Attempt to send unknown or incomplete transaction message
    LedgerInvalidTransaction,

    // Attempt to send transaction without the necessary privileges
    LedgerSecurityError,

    // Revocation registry is full and creation of new registry is necessary
    AnoncredsRevocationRegistryFullError = 400,

    AnoncredsInvalidUserRevocIndex,

    AnoncredsAccumulatorIsFull,

    AnoncredsNotIssuedError,

    // Attempt to generate master secret with dupplicated name
    AnoncredsMasterSecretDuplicateNameError,

    AnoncredsProofRejected,

    // Signus errors
    // Unknown format of DID entity keys
    SignusUnknownCryptoError = 500

} indy_error_t;

#endif

