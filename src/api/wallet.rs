extern crate libc;

use api::ErrorCode;

use self::libc::{c_char, c_uchar};

/// Creates keys (signing and encryption keys) for a new
/// Identity (owned by the caller of the library).
/// Identity's DID can be either explicitly provided, or taken as the first 16 bit of verkey.
/// Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
/// and encrypt transactions.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// identity_json: Identity information as json. Example:
/// {
///     "did": string, (optional; if not provided then the first 16 bit of the verkey will be used
///             as a new DID; if provided, then keys will be replaced - key rotation use case)
///     "seed": string, (optional; if not provide then a random one will be created)
///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
///               currently only 'ed25519' value is supported for this field)
/// }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// DID, verkey (for verification of signature) and public_key (for decryption)
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
pub  extern fn wallet_create_and_store_my_identity(session_handle: i32, command_handle: i32,
                                                   identity_json: *const c_char,
                                                   cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                                                 did: *const c_char,
                                                                 verkey: *const c_char,
                                                                 pk: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Saves their Identity for a pairwise connection in a secured Wallet,
/// so that it can be used to verify transaction.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// identity_json: Identity information as json. Example:
///     {
///        "did": string, (required)
///        "verkey": string, (optional; if only public key for decryption is provided),
///        "pk": string (optional, if only verification key sis provided),
///     }
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
pub  extern fn wallet_store_their_identity(session_handle: i32, command_handle: i32,
                                           identity_json: *const c_char,
                                           cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Signs a message by a signing key associated with my DID. The DID with a signing key
/// must be already created and stored in a secured wallet (see wallet_create_and_store_my_identity)
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// did: signing DID
/// msg: a message to be signed
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// a signature string
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
pub  extern fn wallet_sign(session_handle: i32, command_handle: i32,
                           did: *const c_char,
                           msg: *const c_char,
                           cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                         signature: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Verify a signature created by a key associated with a DID.
/// If a secure wallet doesn't contain a verkey associated with the given DID,
/// then verkey is read from the Ledger.
/// Otherwise either an existing verkey from wallet is used (see wallet_store_their_identity),
/// or it checks the Ledger (according to freshness settings set during initialization)
/// whether verkey is still the same and updates verkey for the DID if needed.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// did: DID that signed the message
/// msg: message
/// signature: a signature to be verified
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// None
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
pub  extern fn wallet_verify(session_handle: i32, command_handle: i32,
                             did: *const c_char,
                             msg: *const c_char,
                             signature: *const c_char,
                             cb: extern fn(xcommand_handle: i32, err: ErrorCode)) -> ErrorCode {
    unimplemented!();
}

/// Encrypts a message by a public key associated with a DID.
/// If a secure wallet doesn't contain a public key associated with the given DID,
/// then the public key is read from the Ledger.
/// Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
/// or it checks the Ledger (according to freshness settings set during initialization)
/// whether public key is still the same and updates public key for the DID if needed.
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// did: encrypting DID
/// msg: a message to be signed
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// an encrypted message
///
/// #Errors
/// Common*
/// Wallet*
/// Ledger*
/// Crypto*
pub  extern fn wallet_encrypt(session_handle: i32, command_handle: i32,
                              did: *const c_char,
                              msg: *const c_char,
                              cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                            encrypted_msg: *const c_char)) -> ErrorCode {
    unimplemented!();
}

/// Decrypts a message encrypted by a public key associated with my DID.
/// The DID with a secret key must be already created and
/// stored in a secured wallet (see wallet_create_and_store_my_identity)
///
/// #Params
/// session_handle: session handler (created by open_session).
/// command_handle: command handle to map callback to session.
/// did: DID that signed the message
/// encrypted_msg: encrypted message
/// cb: Callback that takes command result as parameter.
///
/// #Returns
/// decrypted message
///
/// #Errors
/// Common*
/// Wallet*
/// Crypto*
pub  extern fn wallet_decrypt(session_handle: i32, command_handle: i32,
                              did: *const c_char,
                              encrypted_msg: *const c_char,
                              cb: extern fn(xcommand_handle: i32, err: ErrorCode,
                                            decrypted_msg: *const c_char)) -> ErrorCode {
    unimplemented!();
}