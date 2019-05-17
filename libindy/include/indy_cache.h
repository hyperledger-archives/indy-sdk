#ifndef __indy__cache__included__
#define __indy__cache__included__

#ifdef __cplusplus
extern "C" {
#endif

    /// Gets schema json data for specified schema id.
    /// If data is present inside of cache, cached data is returned.
    /// Otherwise data is fetched from the ledger and stored inside of cache for future use.
    ///
    /// EXPERIMENTAL
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// pool_handle: pool handle (created by open_pool_ledger).
    /// wallet_handle: wallet handle (created by open_wallet).
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// id: identifier of schema.
    /// options_json:
    ///  {
    ///    noCache: (bool, optional, false by default) Skip usage of cache,
    ///    noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
    ///    noStore: (bool, optional, false by default) Skip storing fresh data if updated,
    ///    minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
    ///  }
    /// #Returns
    /// Schema json:
    /// {
    ///     id: identifier of schema
    ///     attrNames: array of attribute name strings
    ///     name: Schema's name string
    ///     version: Schema's version string
    ///     ver: Version of the Schema json
    /// }
    extern indy_error_t indy_get_schema(indy_handle_t command_handle,
                                        indy_handle_t pool_handle,
                                        indy_handle_t wallet_handle,
                                        const char *  submitter_did,
                                        const char *  id,
                                        const char *  options_json,
                                        void          (*cb)(indy_handle_t command_handle_,
                                                            indy_error_t  err,
                                                            const char*   schema_json)
                                       );

    /// Gets credential definition json data for specified credential definition id.
    /// If data is present inside of cache, cached data is returned.
    /// Otherwise data is fetched from the ledger and stored inside of cache for future use.
    ///
    /// EXPERIMENTAL
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// pool_handle: pool handle (created by open_pool_ledger).
    /// wallet_handle: wallet handle (created by open_wallet).
    /// submitter_did: DID of the submitter stored in secured Wallet.
    /// id: identifier of credential definition.
    /// options_json:
    ///  {
    ///    noCache: (bool, optional, false by default) Skip usage of cache,
    ///    noUpdate: (bool, optional, false by default) Use only cached data, do not try to update.
    ///    noStore: (bool, optional, false by default) Skip storing fresh data if updated,
    ///    minFresh: (int, optional, -1 by default) Return cached data if not older than this many seconds. -1 means do not check age.
    ///  }
    ///
    /// #Returns
    /// Credential Definition json:
    /// {
    ///     id: string - identifier of credential definition
    ///     schemaId: string - identifier of stored in ledger schema
    ///     type: string - type of the credential definition. CL is the only supported type now.
    ///     tag: string - allows to distinct between credential definitions for the same issuer and schema
    ///     value: Dictionary with Credential Definition's data: {
    ///         primary: primary credential public key,
    ///         Optional<revocation>: revocation credential public key
    ///     },
    ///     ver: Version of the Credential Definition json
    /// }
    extern indy_error_t indy_get_cred_def(indy_handle_t command_handle,
                                          indy_handle_t pool_handle,
                                          indy_handle_t wallet_handle,
                                          const char *  submitter_did,
                                          const char *  id,
                                          const char *  options_json,
                                          void          (*cb)(indy_handle_t command_handle_,
                                                              indy_error_t  err,
                                                              const char*   cred_def_json)
                                         );

    /// Purge schema cache.
    ///
    /// EXPERIMENTAL
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// wallet_handle: wallet handle (created by open_wallet).
    /// options_json:
    ///  {
    ///    maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
    ///  }
    extern indy_error_t indy_purge_schema_cache(indy_handle_t command_handle,
                                                indy_handle_t wallet_handle,
                                                const char *  options_json,
                                                void          (*cb)(indy_handle_t command_handle_,
                                                                    indy_error_t  err)
                                               );

    /// Purge credential definition cache.
    ///
    /// EXPERIMENTAL
    ///
    /// #Params
    /// command_handle: command handle to map callback to caller context.
    /// wallet_handle: wallet handle (created by open_wallet).
    /// options_json:
    ///  {
    ///    maxAge: (int, optional, -1 by default) Purge cached data if older than this many seconds. -1 means purge all.
    ///  }
    extern indy_error_t indy_purge_cred_def_cache(indy_handle_t command_handle,
                                                  indy_handle_t wallet_handle,
                                                  const char *  options_json,
                                                  void          (*cb)(indy_handle_t command_handle_,
                                                                      indy_error_t  err)
                                                 );
#ifdef __cplusplus
}
#endif

#endif
