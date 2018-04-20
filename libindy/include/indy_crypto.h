#ifndef __indy__crypto__included__
#define __indy__crypto__included__

#ifdef __cplusplus
extern "C" {
#endif



    /// Creates keys pair and stores in the wallet.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// key_json: Key information as json. Example:
    /// {
    ///     "seed": string, // Optional (if not set random one will be used); Seed information that allows deterministic key creation.
    ///     "crypto_type": string, // Optional (if not set then ed25519 curve is used); Currently only 'ed25519' value is supported for this field.
    /// }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - xcommand_handle: command handle to map callback to caller context.
    /// - err: Error code.
    /// - verkey: Ver key of generated key pair, also used as key identifier
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_create_key(indy_handle_t     command_handle,
                                        indy_handle_t     wallet_handle,
                                        const char *const key_json,

                                        void              (*cb)(indy_handle_t     command_handle,
                                                                indy_error_t      err,
                                                                const char *const vk)
                                       );

    /// Saves/replaces the meta information for the giving key in the wallet.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// verkey - the key (verkey, key id) to store metadata.
    /// metadata - the meta information that will be store with the key.
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
    extern indy_error_t indy_set_key_metadata(indy_handle_t     command_handle,
                                              indy_handle_t     wallet_handle,
                                              const char *const verkey,
                                              const char *const metadata,

                                              void              (*cb)(indy_handle_t     command_handle,
                                                                      indy_error_t      err)
                                              );

    /// Retrieves the meta information for the giving key in the wallet.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// verkey - The key (verkey, key id) to retrieve metadata.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - xcommand_handle: Command handle to map callback to caller context.
    /// - err: Error code.
    /// - metadata - The meta information stored with the key; Can be null if no metadata was saved for this key.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_get_key_metadata(indy_handle_t     command_handle,
                                              indy_handle_t     wallet_handle,
                                              const char *const verkey,

                                              void              (*cb)(indy_handle_t     command_handle,
                                                                      indy_error_t      err,
                                                                      const char *const metadata)
                                             );




    /// Signs a message with a key.
    ///
    /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    /// for specific DID.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// signer_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
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
    extern indy_error_t indy_crypto_sign(indy_handle_t      command_handle,
                                         indy_handle_t      wallet_handle,
                                         const char *       signer_vk,
                                         const indy_u8_t *  message_raw,
                                         indy_u32_t         message_len,

                                         void           (*cb)(indy_handle_t    xcommand_handle,
                                                              indy_error_t     err,
                                                              const indy_u8_t* signature_raw,
                                                              indy_u32_t       signature_len)
                                        );

    /// Verify a signature with a verkey.
    ///
    /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    /// for specific DID.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// signer_vk: verkey of signer of the message
    /// message_raw: a pointer to first byte of message that has been signed
    /// message_len: a message length
    /// signature_raw: a pointer to first byte of signature to be verified
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
    extern indy_error_t indy_crypto_verify(indy_handle_t      command_handle,
                                           const char *       signer_vk,
                                           const indy_u8_t *  message_raw,
                                           indy_u32_t         message_len,
                                           const indy_u8_t *  signature_raw,
                                           indy_u32_t         signature_len,

                                           void           (*cb)(indy_handle_t xcommand_handle,
                                                                indy_error_t  err,
                                                                indy_bool_t   valid )
                                          );

    /// Encrypt a message by authenticated-encryption scheme.
    ///
    /// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
    /// Using Recipient's public key, Sender can compute a shared secret key.
    /// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
    /// That shared secret key can be used to verify that the encrypted message was not tampered with,
    /// before eventually decrypting it.
    ///
    /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    /// for specific DID.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handle (created by open_wallet).
    /// sender_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
    /// recipient_vk: id (verkey) of their key
    /// message_raw: a pointer to first byte of message that to be encrypted
    /// message_len: a message length
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// an encrypted message as a pointer to array of bytes.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Ledger*
/// Crypto*
    extern indy_error_t indy_crypto_auth_crypt(indy_handle_t      command_handle,
                                               indy_handle_t      wallet_handle,
                                               const char *       sender_vk,
                                               const char *       recipient_vk,
                                               const indy_u8_t *  message_raw,
                                               indy_u32_t         message_len,

                                               void           (*cb)(indy_handle_t     xcommand_handle,
                                                                    indy_error_t      err,
                                                                    const indy_u8_t*  encrypted_msg_raw,
                                                                    indy_u32_t        encrypted_msg_len)
                                              );

    /// Decrypt a message by authenticated-encryption scheme.
    ///
    /// Sender can encrypt a confidential message specifically for Recipient, using Sender's public key.
    /// Using Recipient's public key, Sender can compute a shared secret key.
    /// Using Sender's public key and his secret key, Recipient can compute the exact same shared secret key.
    /// That shared secret key can be used to verify that the encrypted message was not tampered with,
    /// before eventually decrypting it.
    ///
    /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    /// for specific DID.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// recipient_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
    /// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
    /// encrypted_msg_len: a message length
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// sender verkey and decrypted message as a pointer to array of bytes
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_crypto_auth_decrypt(indy_handle_t      command_handle,
                                                 indy_handle_t      wallet_handle,
                                                 const char *       recipient_vk,
                                                 const indy_u8_t*   encrypted_msg_raw,
                                                 indy_u32_t         encrypted_msg_len,

                                                 void           (*cb)(indy_handle_t     xcommand_handle,
                                                                      indy_error_t      err,
                                                                      const char *      sender_vk,
                                                                      const indy_u8_t*  decrypted_msg_raw,
                                                                      indy_u32_t        decrypted_msg_len)
                                                );


    /// Encrypts a message by anonymous-encryption scheme.
    ///
    /// Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
    /// Only the Recipient can decrypt these messages, using its private key.
    /// While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
    ///
    /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    /// for specific DID.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// recipient_vk: verkey of message recipient
    /// message_raw: a pointer to first byte of message that to be encrypted
    /// message_len: a message length
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// an encrypted message as a pointer to array of vytes
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Ledger*
    /// Crypto*
    extern indy_error_t indy_crypto_anon_crypt(indy_handle_t      command_handle,
                                               const char *       recipient_vk,
                                               const indy_u8_t *  message_raw,
                                               indy_u32_t         message_len,

                                               void           (*cb)(indy_handle_t     xcommand_handle,
                                                                    indy_error_t      err,
                                                                    const indy_u8_t*  encrypted_msg_raw,
                                                                    indy_u32_t        encrypted_msg_len)
                                               );

    /// Decrypts a message by anonymous-encryption scheme.
    ///
    /// Sealed boxes are designed to anonymously send messages to a Recipient given its public key.
    /// Only the Recipient can decrypt these messages, using its private key.
    /// While the Recipient can verify the integrity of the message, it cannot verify the identity of the Sender.
    ///
    /// Note to use DID keys with this function you can call indy_key_for_did to get key id (verkey)
    /// for specific DID.
    ///
    /// #Params
    /// command_handle: command handle to map callback to user context.
    /// wallet_handle: wallet handler (created by open_wallet).
    /// recipient_vk: id (verkey) of my key. The key must be created by calling indy_create_key or indy_create_and_store_my_did
    /// encrypted_msg_raw: a pointer to first byte of message that to be decrypted
    /// encrypted_msg_len: a message length
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// decrypted message as a pointer to an array of bytes
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_crypto_anon_decrypt(indy_handle_t      command_handle,
                                                 indy_handle_t      wallet_handle,
                                                 const char *       recipient_vk,
                                                 const indy_u8_t*   encrypted_msg,
                                                 indy_u32_t         encrypted_len,

                                                 void           (*cb)(indy_handle_t     xcommand_handle,
                                                                      indy_error_t      err,
                                                                      const indy_u8_t*  decrypted_msg_raw,
                                                                      indy_u32_t        decrypted_msg_len)
                                                 );

#ifdef __cplusplus
}
#endif

#endif
