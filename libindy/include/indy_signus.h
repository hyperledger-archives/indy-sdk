#ifndef __indy__signus__included__
#define __indy__signus__included__

#ifdef __cplusplus
extern "C" {
#endif

    /// Creates keys (signing and encryption keys) for a new
    /// DID (owned by the caller of the library).
    /// Identity's DID must be either explicitly provided, or taken as the first 16 bit of verkey.
    /// Saves the Identity DID with keys in a secured Wallet, so that it can be used to sign
    /// and encrypt transactions.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// did_json: Identity information as json. Example:
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
    
    extern indy_error_t indy_create_and_store_my_did(indy_handle_t command_handle,
                                                         indy_handle_t wallet_handle,
                                                         const char *    did_json,
                                                         
                                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                                              indy_error_t  err,
                                                                              const char*     did,
                                                                              const char*     verkey,
                                                                              const char*     pk)
                                                        );

    /// Generated new keys (signing and encryption keys) for an existing
    /// DID (owned by the caller of the library).
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// identity_json: Identity information as json. Example:
    /// {
    ///     "seed": string, (optional; if not provide then a random one will be created)
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    /// }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// verkey (for verification of signature) and public_key (for decryption)
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_replace_keys(indy_handle_t command_handle,
                                              indy_handle_t wallet_handle,
                                              const char *    did,
                                              const char *    identity_json,
                                              
                                              void           (*cb)(indy_handle_t xcommand_handle,
                                                                   indy_error_t  err,
                                                                   const char*     verkey,
                                                                   const char*     pk)
                                             );

    /// Saves their DID for a pairwise connection in a secured Wallet,
    /// so that it can be used to verify transaction.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// identity_json: Identity information as json. Example:
    ///     {
    ///        "did": string, (required)
    ///        "verkey": string (optional, if only pk is provided),
    ///        "pk": string (optional, if only verification key is provided),
    ///        "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
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

    extern indy_error_t indy_store_their_did(indy_handle_t command_handle,
                                                 indy_handle_t wallet_handle,
                                                 const char *    identity_json,
                                                 
                                                 void           (*cb)(indy_handle_t xcommand_handle,
                                                                      indy_error_t  err)
                                                );

    /// Signs a message by a signing key associated with my DID. The DID with a signing key
    /// must be already created and stored in a secured wallet (see create_and_store_my_identity)
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
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
    
    extern indy_error_t indy_sign(indy_handle_t command_handle,
                                      indy_handle_t wallet_handle,
                                      const char *    did,
                                      const char *    msg,
                                      
                                      void           (*cb)(indy_handle_t xcommand_handle,
                                                           indy_error_t  err,
                                                           const char* signature)
                                     );
    
    /// Verify a signature created by a key associated with a DID.
    /// If a secure wallet doesn't contain a verkey associated with the given DID,
    /// then verkey is read from the Ledger.
    /// Otherwise either an existing verkey from wallet is used (see wallet_store_their_identity),
    /// or it checks the Ledger (according to freshness settings set during initialization)
    /// whether verkey is still the same and updates verkey for the DID if needed.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// pool_handle: pool handle.
    /// did: DID that signed the message
    /// signature: a signature to be verified
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// valid: true - if signature is valid, false - otherwise
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Ledger*
    /// Crypto*
    
    extern indy_error_t indy_verify_signature(indy_handle_t command_handle,
                                                  indy_handle_t wallet_handle,
                                                  indy_handle_t pool_handle,
                                                  
                                                  const char *    did,
                                                  const char *    signed_msg,

                                                  void           (*cb)(indy_handle_t xcommand_handle,
                                                                       indy_error_t  err,
                                                                       indy_bool_t   valid )
                                                 );

    /// Encrypts a message by a public key associated with a DID.
    /// If a secure wallet doesn't contain a public key associated with the given DID,
    /// then the public key is read from the Ledger.
    /// Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
    /// or it checks the Ledger (according to freshness settings set during initialization)
    /// whether public key is still the same and updates public key for the DID if needed.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// my_did: encrypting DID
    /// did: encrypting DID
    /// msg: a message to be signed
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// an encrypted message and nonce
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Ledger*
    /// Crypto*
    
    extern indy_error_t indy_encrypt(indy_handle_t command_handle,
                                         indy_handle_t wallet_handle,
                                         indy_handle_t pool_handle,
                                         const char *    my_did,
                                         const char *    did,
                                         const char *    msg,
                                         
                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                              indy_error_t  err,
                                                              const char*     encrypted_msg,
                                                              const char*     nonce)
                                       );

    /// Decrypts a message encrypted by a public key associated with my DID.
    /// The DID with a secret key must be already created and
    /// stored in a secured wallet (see wallet_create_and_store_my_identity)
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// my_did: DID
    /// did: DID that signed the message
    /// encrypted_msg: encrypted message
    /// nonce: nonce that encrypted message
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// decrypted message
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    
    extern indy_error_t indy_decrypt(indy_handle_t command_handle,
                                         indy_handle_t wallet_handle,
                                         const char *    my_did,
                                         const char *    did,
                                         const char *    encrypted_msg,
                                         const char *    nonce,
                                         
                                         void           (*cb)(indy_handle_t xcommand_handle,
                                                              indy_error_t  err,
                                                              const char*     decrypted_msg)
                                        );    

#ifdef __cplusplus
}
#endif

#endif
