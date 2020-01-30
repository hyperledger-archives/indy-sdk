#ifndef __indy__did__included__
#define __indy__did__included__

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
    ///     "did": string, (optional;
    ///             if not provided and cid param is false then the first 16 bit of the verkey will be used as a new DID;
    ///             if not provided and cid is true then the full verkey will be used as a new DID;
    ///             if provided, then keys will be replaced - key rotation use case)
    ///     "seed": string, (optional) Seed that allows deterministic did creation (if not set random one will be created).
    ///                                Can be UTF-8, base64 or hex string.
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    ///     "cid": bool, (optional; if not set then false is used;)
    ///     "method_name": string, method name to create fully qualified did (Example:  `did:method_name:NcYxiDXkpYi6ov5FcYDi1e`).
    /// }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///   did: DID generated and stored in the wallet
    ///   verkey: The DIDs verification key
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    
    extern indy_error_t indy_create_and_store_my_did(indy_handle_t command_handle,
                                                     indy_handle_t wallet_handle,
                                                     const char *  did_json,

                                                     void          (*cb)(indy_handle_t  command_handle_,
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
    ///     "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
    ///                                Can be UTF-8, base64 or hex string.
    ///     "crypto_type": string, (optional; if not set then ed25519 curve is used;
    ///               currently only 'ed25519' value is supported for this field)
    /// }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///   verkey: The DIDs verification key
    ///
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_replace_keys_start(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  did,
                                                const char *  identity_json,

                                                void           (*cb)(indy_handle_t command_handle_,
                                                                     indy_error_t  err,
                                                                     const char *const   verkey)
                                               );

    /// Apply temporary keys as main for an existing DID (owned by the caller of the library).
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// did: DID stored in the wallet
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

    extern indy_error_t indy_replace_keys_apply(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  did,

                                                void           (*cb)(indy_handle_t command_handle_,
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
    ///        "verkey": string
    ///             - optional is case of adding a new DID, and DID is cryptonym: did == verkey,
    ///             - mandatory in case of updating an existing DID
    ///     }
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*

   extern indy_error_t indy_store_their_did(indy_handle_t command_handle,
                                            indy_handle_t wallet_handle,
                                            const char *  identity_json,

                                            void           (*cb)(indy_handle_t command_handle_,
                                                                 indy_error_t  err)
                                           );

    /// Returns ver key (key id) for the given DID.
    ///
    /// "indy_key_for_did" call follow the idea that we resolve information about their DID from
    /// the ledger with cache in the local wallet. The "indy_open_wallet" call has freshness parameter
    /// that is used for checking the freshness of cached pool value.
    ///
    /// Note if you don't want to resolve their DID info from the ledger you can use
    /// "indy_key_for_local_did" call instead that will look only to the local wallet and skip
    /// freshness checking.
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
    /// - command_handle_: Command handle to map callback to caller context.
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

    /// Returns ver key (key id) for the given DID.
    ///
    /// "indy_key_for_local_did" call looks data stored in the local wallet only and skips freshness
    /// checking.
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
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    /// - key - The DIDs ver key (key id).
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_key_for_local_did(indy_handle_t     command_handle,
                                              indy_handle_t     wallet_handle,
                                              const char *const did,

                                              void              (*cb)(indy_handle_t     command_handle,
                                                                      indy_error_t      err,
                                                                     const char *const key)
                                             );

    /// Set/replaces endpoint information for the given DID.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did - The DID to resolve endpoint.
    /// address -  The DIDs endpoint address.
    /// transport_key - The DIDs transport key (ver key, key id).
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_set_endpoint_for_did(indy_handle_t     command_handle,
                                                  indy_handle_t     wallet_handle,
                                                  const char *const did,
                                                  const char *const address,
                                                  const char *const transport_key,

                                                  void              (*cb)(indy_handle_t     command_handle,
                                                                          indy_error_t      err)
                                                 );

    /// Returns endpoint information for the given DID.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did - The DID to resolve endpoint.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    /// - endpoint - The DIDs endpoint.
    /// - transport_vk - The DIDs transport key (ver key, key id).
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
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
    /// - command_handle_: command handle to map callback to caller context.
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
    /// - command_handle_: Command handle to map callback to caller context.
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

    /// Retrieves the information about the giving DID in the wallet.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did - The DID to retrieve information.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///   did_with_meta:  {
    ///     "did": string - DID stored in the wallet,
    ///     "verkey": string - The DIDs transport key (ver key, key id),
    ///     "metadata": string - The meta information stored with the DID
    ///   }
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_get_my_did_with_meta(indy_handle_t     command_handle,
                                                  indy_handle_t     wallet_handle,
                                                  const char *const my_did,
                                                  void              (*fn)(indy_handle_t command_handle_, indy_error_t err, const char *const did_with_meta)
                                                 );

    /// Retrieves the information about all DIDs stored in the wallet.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///   dids:  [{
    ///     "did": string - DID stored in the wallet,
    ///     "verkey": string - The DIDs transport key (ver key, key id).,
    ///     "metadata": string - The meta information stored with the DID
    ///   }]
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_list_my_dids_with_meta(indy_handle_t command_handle,
                                                    indy_handle_t wallet_handle,
                                                    void          (*fn)(indy_handle_t command_handle_, indy_error_t err, const char *const dids)
                                                   );

    /// Retrieves abbreviated verkey if it is possible otherwise return full verkey.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// did: DID.
    /// full_verkey: The DIDs verification key,
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - command_handle_: Command handle to map callback to caller context.
    /// - err: Error code.
    ///   verkey: The DIDs verification key in either abbreviated or full form
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_abbreviate_verkey(indy_handle_t command_handle,
                                             const char *const did,
                                             const char *const full_verkey,
                                             void          (*fn)(indy_handle_t command_handle_,
                                                                 indy_error_t err,
                                                                 const char *const verkey)
                                            );

    /// Update DID stored in the wallet to make fully qualified, or to do other DID maintenance.
    ///     - If the DID has no prefix, a prefix will be appended (prepend did:peer to a legacy did)
    ///     - If the DID has a prefix, a prefix will be updated (migrate did:peer to did:peer-new)
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// wallet_handle: Wallet handle (created by open_wallet).
    /// did: target DID stored in the wallet.
    /// prefix: prefix to apply to the DID.
    /// cb: Callback that takes command result as parameter.
    ///
    /// #Returns
    /// Error Code
    /// cb:
    /// - did: fully qualified did
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    /// Crypto*
    extern indy_error_t indy_qualify_did(indy_handle_t     command_handle,
                                         indy_handle_t     wallet_handle,
                                         const char *const did,
                                         const char *const method,

                                         void              (*cb)(indy_handle_t     command_handle,
                                                                 indy_error_t      err,
                                                                 const char *const full_qualified_did)
                                        );

#ifdef __cplusplus
}
#endif

#endif
