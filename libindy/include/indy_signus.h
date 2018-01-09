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
                                                     const char *  did_json,

                                                     void          (*cb)(indy_handle_t  xcommand_handle,
                                                                          indy_error_t  err,
                                                                          const char *const   did,
                                                                          const char *const   verkey)
                                                    );

    /// Generated temporary keys (signing and encryption keys) for an existing
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

    extern indy_error_t indy_replace_keys_start(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  did,
                                                const char *  identity_json,

                                                void           (*cb)(indy_handle_t xcommand_handle,
                                                                     indy_error_t  err,
                                                                     const char *const   verkey)
                                               );

    /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// did
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_replace_keys_apply(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  did,

                                                void           (*cb)(indy_handle_t xcommand_handle,
                                                                     indy_error_t  err)
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
    ///        "verkey": string (optional, can be avoided if did is cryptonym: did == verkey)
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
                                             const char *  identity_json,

                                             void           (*cb)(indy_handle_t xcommand_handle,
                                                                  indy_error_t  err)
                                            );

    /// Returns ver key (key id) for the given DID.
    ///
    /// "indy_key_for_did" call follow the idea that we resolve information about their DID from
    /// the ledger with cache in the local wallet. The "indy_open_wallet" call has freshness parameter
    /// that is used for checking the freshness of cached pool value.
    ///
    /// Note if you don't want to resolve their DID info from the ledger you can use
    /// "indy_key_for_local_did" call instead that will look only to local wallet and skip
    /// freshness checking
    ///
    /// Note that indy_create_and_store_my_did makes similar wallet record as indy_create_key.
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did - The DID to resolve key.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - xcommand_handle: Command handle to map callback to caller context.
    /// - err: Error code.
    /// - key - The DIDs ver key (key id).
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_key_for_did(indy_handle_t     command_handle,
                                         indy_handle_t     pool_handle,
                                         indy_handle_t     wallet_handle,
                                         const char *const did,

                                         void              (*cb)(indy_handle_t     command_handle,
                                                                 indy_error_t      err,
                                                                 const char *const key)
                                        );

    //// Returns ver key (key id) for the given DID.
    ///
    /// "indy_key_for_local_did" call looks data stored in the local wallet only and skips freshness checking.
    ///
    /// Note if you want to get fresh data from the ledger you can use "indy_key_for_did" call
    /// instead.
    ///
    /// Note that "indy_create_and_store_my_did" makes similar wallet record as "indy_create_key".
    /// As result we can use returned ver key in all generic crypto and messaging functions.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did - The DID to resolve key.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - xcommand_handle: Command handle to map callback to caller context.
    /// - err: Error code.
    /// - key - The DIDs ver key (key id).
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_key_for_local_did(indy_handle_t     command_handle,
                                              indy_handle_t     pool_handle,
                                              indy_handle_t     wallet_handle,
                                              const char *const did,

                                              void              (*cb)(indy_handle_t     command_handle,
                                                                      indy_error_t      err,
                                                                     const char *const key)
                                             );

    extern indy_error_t indy_set_endpoint_for_did(indy_handle_t     command_handle,
                                                  indy_handle_t     wallet_handle,
                                                  const char *const did,
                                                  const char *const address,
                                                  const char *const transport_key,

                                                  void              (*cb)(indy_handle_t     command_handle,
                                                                          indy_error_t      err)
                                                 );

    extern indy_error_t indy_get_endpoint_for_did(indy_handle_t     command_handle,
                                                  indy_handle_t     wallet_handle,
                                                  indy_handle_t     pool_handle,
                                                  const char *const did,

                                                  void              (*cb)(indy_handle_t     command_handle,
                                                                          indy_error_t      err,
                                                                          const char *const address,
                                                                          const char *const transport_vk)
                                                 );

    /// Saves/replaces the meta information for the giving DID in the wallet.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did - the DID to store metadata.
    /// metadata - the meta information that will be store with the DID.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - xcommand_handle: command handle to map callback to caller context.
    /// - err: Error code.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_set_did_metadata(indy_handle_t     command_handle,
                                              indy_handle_t     wallet_handle,
                                              const char *const did,
                                              const char *const metadata,

                                              void              (*cb)(indy_handle_t     command_handle,
                                                                      indy_error_t      err)
                                             );

    /// Retrieves the meta information for the giving DID in the wallet.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did - The DID to retrieve metadata.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - xcommand_handle: Command handle to map callback to caller context.
    /// - err: Error code.
    /// - metadata - The meta information stored with the DID; Can be null if no metadata was saved for this DID.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_get_did_metadata(indy_handle_t     command_handle,
                                              indy_handle_t     wallet_handle,
                                              const char *const did,

                                              void              (*cb)(indy_handle_t     command_handle,
                                                                      indy_error_t      err,
                                                                      const char *const metadata)
                                             );

    /// Signs a message by a signing key associated with my DID. The DID with a signing key
    /// must be already created and stored in a secured wallet (see create_and_store_my_identity)
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// did: signing DID
    /// message_raw: a pointer to first byte of message to be signed
    /// message_len: a message length
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// a signature string
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    
    extern indy_error_t indy_sign(indy_handle_t      command_handle,
                                  indy_handle_t      wallet_handle,
                                  const char *       did,
                                  const indy_u8_t *  message_raw,
                                  indy_u32_t         message_len,

                                  void           (*cb)(indy_handle_t    xcommand_handle,
                                                       indy_error_t     err,
                                                       const indy_u8_t *const signature_raw,
                                                       indy_u32_t       signature_len)
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
    /// message_raw: a pointer to first byte of message to be signed
    /// message_len: a message length
    /// signature_raw: a a pointer to first byte of signature to be verified
    /// signature_len: a signature length
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
    
    extern indy_error_t indy_verify_signature(indy_handle_t      command_handle,
                                              indy_handle_t      wallet_handle,
                                              indy_handle_t      pool_handle,

                                              const char *       did,
                                              const indy_u8_t *  message_raw,
                                              indy_u32_t         message_len,
                                              const indy_u8_t *  signature_raw,
                                              indy_u32_t         signature_len,

                                              void           (*cb)(indy_handle_t xcommand_handle,
                                                                   indy_error_t  err,
                                                                   indy_bool_t   valid )
                                             );

    /// Encrypts a message by public-key (associated with their did) authenticated-encryption scheme
    /// using nonce.
    /// If a secure wallet doesn't contain a public key associated with the given DID,
    /// then the public key is read from the Ledger.
    /// Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
    /// or it checks the Ledger (according to freshness settings set during initialization)
    /// whether public key is still the same and updates public key for the DID if needed.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// pool_handle: pool handle.
    /// my_did: encrypted DID
    /// their_did: encrypted DID
    /// message_raw: a pointer to first byte of message that to be encrypted
    /// message_len: a message length
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
    
    extern indy_error_t indy_encrypt(indy_handle_t      command_handle,
                                     indy_handle_t      wallet_handle,
                                     indy_handle_t      pool_handle,
                                     const char *       my_did,
                                     const char *       their_did,
                                     const indy_u8_t *  message_raw,
                                     indy_u32_t         message_len,

                                     void           (*cb)(indy_handle_t     xcommand_handle,
                                                          indy_error_t      err,
                                                          const indy_u8_t *const encrypted_msg_raw,
                                                          indy_u32_t        encrypted_msg_len,
                                                          const indy_u8_t *const  nonce_raw,
                                                          indy_u32_t        nonce_len)
                                     );

    /// Decrypts a message by public-key authenticated-encryption scheme using nonce.
    /// The DID with a secret key must be already created and
    /// stored in a secured wallet (see wallet_create_and_store_my_identity)
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// pool_handle: pool handle.
    /// my_did: encrypted DID
    /// their_did: encrypted DID that signed the message
    /// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
    /// encrypted_msg_len: a message length
    /// nonce_raw: a pointer to first byte of nonce that encrypted message
    /// nonce_len: a nonce length
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// decrypted message
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    
    extern indy_error_t indy_decrypt(indy_handle_t      command_handle,
                                     indy_handle_t      wallet_handle,
                                     indy_handle_t      pool_handle,
                                     const char *       my_did,
                                     const char *       their_did,
                                     const indy_u8_t*   encrypted_msg_raw,
                                     indy_u32_t         encrypted_msg_len,
                                     const indy_u8_t*   nonce_raw,
                                     indy_u32_t         nonce_len,

                                     void           (*cb)(indy_handle_t     xcommand_handle,
                                                          indy_error_t      err,
                                                          const indy_u8_t *const  decrypted_msg_raw,
                                                          indy_u32_t        decrypted_msg_len)
                                    );


    /// Encrypts a message by public-key (associated with did) anonymous-encryption scheme.
    /// If a secure wallet doesn't contain a public key associated with the given DID,
    /// then the public key is read from the Ledger.
    /// Otherwise either an existing public key from wallet is used (see wallet_store_their_identity),
    /// or it checks the Ledger (according to freshness settings set during initialization)
    /// whether public key is still the same and updates public key for the DID if needed.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// pool_handle: pool handle.
    /// command_handle: command handle to map callback to user context.
    /// did: encrypted DID
    /// message_raw: a pointer to first byte of message that to be encrypted
    /// message_len: a message length
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

    extern indy_error_t indy_encrypt_sealed(indy_handle_t      command_handle,
                                            indy_handle_t      wallet_handle,
                                            const char *       did,
                                            const indy_u8_t *  message_raw,
                                            indy_u32_t         message_len,

                                            void           (*cb)(indy_handle_t     xcommand_handle,
                                                                 indy_error_t      err,
                                                                 const indy_u8_t *const  encrypted_msg_raw,
                                                                 indy_u32_t        encrypted_msg_len)
                                           );

    /// Decrypts a message by public-key anonymous-encryption scheme.
    /// The DID with a secret key must be already created and
    /// stored in a secured wallet (see wallet_create_and_store_my_identity)
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// did: DID that signed the message
    /// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
    /// encrypted_msg_len: a message length
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// decrypted message
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_decrypt_sealed(indy_handle_t      command_handle,
                                            indy_handle_t      wallet_handle,
                                            const char *       did,
                                            const indy_u8_t*   encrypted_msg_raw,
                                            indy_u32_t         encrypted_msg_len,

                                            void           (*cb)(indy_handle_t     xcommand_handle,
                                                                 indy_error_t      err,
                                                                 const indy_u8_t *const  decrypted_msg_raw,
                                                                 indy_u32_t        decrypted_msg_len)
                                           );

#ifdef __cplusplus
}
#endif

#endif
