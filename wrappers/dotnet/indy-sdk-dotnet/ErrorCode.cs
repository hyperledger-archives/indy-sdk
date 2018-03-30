namespace Hyperledger.Indy
{
    /// <summary>
    /// Error codes
    /// </summary>
    public enum ErrorCode
    {
        /// <summary>
        /// Call succeeded.
        /// </summary>
        Success = 0,

        // Common errors

        /// <summary>
        /// Caller passed invalid value as param 1 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam1 = 100,

        /// <summary>
        /// Caller passed invalid value as param 2 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam2 = 101,

        /// <summary>
        /// Caller passed invalid value as param 3 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam3 = 102,

        /// <summary>
        /// Caller passed invalid value as param 4 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam4 = 103,

        /// <summary>
        /// Caller passed invalid value as param 5 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam5 = 104,

        /// <summary>
        /// Caller passed invalid value as param 6 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam6 = 105,

        /// <summary>
        /// Caller passed invalid value as param 7 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam7 = 106,

        /// <summary>
        /// Caller passed invalid value as param 8 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam8 = 107,

        /// <summary>
        /// Caller passed invalid value as param 9 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam9 = 108,

        /// <summary>
        /// Caller passed invalid value as param 10 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam10 = 109,

        /// <summary>
        /// Caller passed invalid value as param 11 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam11 = 110,

        /// <summary>
        /// Caller passed invalid value as param 12 (null, invalid json and etc..)
        /// </summary>
        CommonInvalidParam12 = 111,

        /// <summary>
        /// Invalid library state was detected in runtime. It signals library bug
        /// </summary>
        CommonInvalidState = 112,

        /// <summary>
        /// Object (json, config, key, claim and etc...) passed by library caller has invalid structure
        /// </summary>
        CommonInvalidStructure = 113,

        /// <summary>
        /// IO Error
        /// </summary>
        CommonIOError = 114,

        // Wallet errors

        /// <summary>
        /// Caller passed invalid wallet handle
        /// </summary>
        WalletInvalidHandle = 200,

        /// <summary>
        /// Unknown type of wallet was passed on create_wallet
        /// </summary>
        WalletUnknownTypeError = 201,

        /// <summary>
        /// Attempt to register already existing wallet type
        /// </summary>
        WalletTypeAlreadyRegisteredError = 202,

        /// <summary>
        /// Attempt to create wallet with name used for another exists wallet
        /// </summary>
        WalletAlreadyExistsError = 203,
 
        /// <summary>
        /// Requested entity id isn't present in wallet
        /// </summary>
        WalletNotFoundError = 204,

        /// <summary>
        /// Trying to use wallet with pool that has different name
        /// </summary>
        WalletIncompatiblePoolError = 205,

        /// <summary>
        /// Trying to open wallet that was opened already
        /// </summary>
        WalletAlreadyOpenedError = 206,

        // Ledger errors

        /// <summary>
        /// Trying to open pool ledger that wasn't created before
        /// </summary>
        PoolLedgerNotCreatedError = 300,
 
        /// <summary>
        /// Caller passed invalid pool ledger handle
        /// </summary>
        PoolLedgerInvalidPoolHandle = 301,

        /// <summary>
        /// Pool ledger terminated
        /// </summary>
        PoolLedgerTerminated = 302,

        /// <summary>
        /// No consensus during ledger operation
        /// </summary>
        LedgerNoConsensusError = 303,

        /// <summary>
        /// Attempt to send unknown or incomplete transaction message
        /// </summary>
        LedgerInvalidTransaction = 304,

        /// <summary>
        /// Attempt to send transaction without the necessary privileges
        /// </summary>
        LedgerSecurityError = 305,

        /// <summary>
        /// Attempt to create pool ledger config with name used for another existing pool
        /// </summary>
        PoolLedgerConfigAlreadyExistsError = 306,


        /// <summary>
        /// Pool ledger timeout
        /// </summary>
        PoolLedgerTimeout = 307,

        // Crypto errors

        /// <summary>
        /// Revocation registry is full and creation of new registry is necessary
        /// </summary>
        AnoncredsRevocationRegistryFullError = 400,

        /// <summary>
        /// Invalid user revocation index
        /// </summary>
        AnoncredsInvalidUserRevocIndex = 401,

        /// <summary>
        /// Accumulator is full
        /// </summary>
        AnoncredsAccumulatorIsFull = 402,

        /// <summary>
        /// Not issued error
        /// </summary>
        AnoncredsNotIssuedError = 403,

        /// <summary>
        /// Attempt to generate master secret with duplicated name
        /// </summary>
        AnoncredsMasterSecretDuplicateNameError = 404,

        /// <summary>
        /// Proof rejected
        /// </summary>
        AnoncredsProofRejected = 405,

        /// <summary>
        /// Claim revoked
        /// </summary>
        AnoncredsClaimRevoked = 406,

        // Signus errors
        // 
        /// <summary>
        /// Unknown format of DID entity keys
        /// </summary>
        SignusUnknownCryptoError = 500
    }
}
