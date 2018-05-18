pub mod ledger;
pub mod payments;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ErrorCode
{
    Success = 0,

    // Common errors

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

    // Signus errors
    // Unknown format of DID entity keys
    UnknownCryptoTypeError = 500,

    // Attempt to create duplicate did
    DidAlreadyExistsError = 600,

    // Unknown payment method was given
    PaymentUnknownMethodError = 700,

    //No method were scraped from inputs/outputs or more than one were scraped
    PaymentIncompatibleMethodsError = 701,

    // Insufficient funds on inputs
    PaymentInsufficientFundsError = 702,
}