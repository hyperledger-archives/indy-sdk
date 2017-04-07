extern crate libc;

pub mod anoncreds;
pub mod signus;
pub mod ledger;
pub mod pool;
pub mod wallet;


use self::libc::{c_char};

#[repr(i32)]
pub enum ErrorCode {
    Success = 0,

    // Common errors
    // Called passed invalid session handle
    CommonInvalidSession = 100,

    // Caller passed invalid value as param 1 (null, invalid json and etc..)
    CommonInvalidParam1,

    // Caller passed invalid value as param 2 (null, invalid json and etc..)
    CommonInvalidParam2,

    // Caller passed invalid value as param 3 (null, invalid json and etc..)
    CommonInvalidParam3,

    // Caller passed invalid value as param 4 (null, invalid json and etc..)
    CommonInvalidParam4,

    // Caller passed invalid value as param 5 (null, invalid json and etc..)
    CommonInvalidParam5,

    // Invalid library state was detected in runtime. It signals library bug
    CommonInvalidState,

    // Indicates that api was called before init_library call
    CommonUninitialized,

    // Wallet errors
    // Unknown type of wallet was passed on open_session
    WalletUnknownType = 200,

    // Attempt to register already existing wallet type
    WalletTypeAlreadyRegistered,

    // Requested entity id isn't present in wallet
    WalletNotFound,

    // Wallet files referenced in open_session have invalid data format
    WalletInvalidDataFormat,

    // IO error during access wallet backend
    WalletIOError,

    // Trying to use wallet with pool that has different name
    WalletIncorrectPool,

    // Ledger errors
    // Pool ledger files referenced in open_session have invalid data format
    LedgerPoolInvalidDataFormat = 300,

    // IO error during access pool ledger files
    LedgerPoolIOError,

    // No concensus during ledger operation
    LedgerNoConsensus,

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
    CryptoUnknownType,

    // Revocation registry is full and creation of new registry is necessary
    AnoncredsRevocationRegistryFull = 500,

    AnoncredsInvalidUserRevocIndex,

    AnoncredsNotIssuedError,

    AnoncredsMasterSecretDuplicateNameError,

    AnoncredsClaimNotFoundError,

    AnoncredsProofRejected
}

#[cfg(test)]
mod tests {
    use super::*;
    use commands::{Command, COMMAND_EXECUTOR};
    use commands::wallet::WalletCommand;
    use std::sync::mpsc::channel;

    #[test]
    fn wallet_set_value_command_can_be_sent() {
        let (sender, receiver) = channel();

        let cb = Box::new(move |result| {
            match result {
                Ok(val) => sender.send("OK"),
                Err(err) => sender.send("ERR")
            };
        });

        let ref cmd_executor = *COMMAND_EXECUTOR.lock().unwrap();
        cmd_executor.send(Command::Wallet(WalletCommand::Set(vec!["key".to_string(), "subkey".to_string()], "value".to_string(), cb)));

        match receiver.recv() {
            Ok(result) => {
                assert_eq!("OK", result);
            }
            Err(err) => {
                panic!("Error on result recv: {:?}", err);
            }
        }
    }
}
