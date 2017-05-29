typedef NS_ENUM(NSInteger, SovrinErrorCode)
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
    
    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState,
    
    CommonInvalidStructure,
    
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
    
    // Wallet files referenced in open_wallet have invalid data format
    WalletInvalidDataFormat,
    
    // IO error during access wallet backend
    WalletIOError,
    
    // Trying to use wallet with pool that has different name
    WalletIncompatiblePoolError,
    
    // Trying to open wallet with invalid configuration
    WalletInvalidConfiguration,
    
    // Error in wallet backend
    WalletBackendError,
    
    // Ledger errors
    // Trying to open pool ledger that wasn't created before
    PoolLedgerNotCreatedError = 300,
    
    // Invalid pool ledger configuration was passed to open_pool_ledger or create_pool_ledger
    PoolLedgerInvalidConfiguration,
    
    // Pool ledger files referenced in open_pool_ledger have invalid data format
    PoolLedgerInvalidDataFormat,
    
    // Caller passed invalid pool ledger handle
    PoolLedgerInvalidPoolHandle,
    
    // IO error during access pool ledger files
    PoolLedgerIOError,
    
    // No concensus during ledger operation
    LedgerNoConsensusError,
    
    // Attempt to send unknown or incomplete transaction message
    LedgerInvalidTransaction,
    
    // Attempt to send transaction without the necessary privileges
    LedgerSecurityError,
    
    // IO error during sending of ledger transactions or catchup process
    LedgerIOError,
    
    // Crypto errors
    // Invalid structure of any crypto promitives (keys, signatures, seeds and etc...)
    CryptoInvalidStructure = 400,
    
    // Unknown crypto type was requested for signing/verifiyng or encoding/decoding
    CryptoUnknownTypeError,
    
    // Revocation registry is full and creation of new registry is necessary
    CryptoRevocationRegistryFullError,
    
    CryptoInvalidUserRevocIndex,
    
    // Error in crypto backend
    CryptoBackendError,
    
    AnoncredsNotIssuedError,
    
    // Attempt to generate master secret with dupplicated name
    AnoncredsMasterSecretDuplicateNameError,
    
    ProofRejected
};
