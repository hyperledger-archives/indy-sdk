#ifndef __indy__wallet__included__
#define __indy__wallet__included__

#ifdef __cplusplus
extern "C" {
#endif

    /// Registers custom wallet storage implementation.
    ///
    /// It allows library user to provide custom wallet implementation.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// type_: Wallet type name.
    /// create: WalletType create operation handler
    /// open: WalletType open operation handler
    /// close: Wallet close operation handler
    /// delete: WalletType delete operation handler
    /// add_record: WalletType add record operation handler
    /// update_record_value: WalletType update record value operation handler
    /// update_record_tags: WalletType update record tags operation handler
    /// add_record_tags: WalletType add record tags operation handler
    /// delete_record_tags: WalletType delete record tags operation handler
    /// delete_record: WalletType delete record operation handler
    /// get_record: WalletType get record operation handler
    /// get_record_id: WalletType get record id operation handler
    /// get_record_type: WalletType get record type operation handler
    /// get_record_value: WalletType get record value operation handler
    /// get_record_tags: WalletType get record tags operation handler
    /// free_record: WalletType free record operation handler
    /// search_records: WalletType search records operation handler
    /// search_all_records: WalletType search all records operation handler
    /// get_search_total_count: WalletType get search total count operation handler
    /// fetch_search_next_record: WalletType fetch search next record operation handler
    /// free_search: WalletType free search operation handler
    /// free: Handler that allows to de-allocate strings allocated in caller code
    ///
    /// #Returns
    /// Error code
    

    extern indy_error_t indy_register_wallet_type(indy_handle_t  command_handle,
                                                  const char*    type_,
                                                  indy_error_t (*createFn)(const char* name,
                                                                           const char* config,
                                                                           const char* credentials_json,
                                                                           const char* metadata),

                                                  indy_error_t (*openFn)(const char* name,
                                                                           const char* config,
                                                                           const char* credentials,
                                                                           indy_handle_t* handle),

                                                  indy_error_t (*closeFn)(indy_handle_t handle),

                                                  indy_error_t (*deleteFn)(const char* name,
                                                                           const char* config,
                                                                           const char* credentials),

                                                  indy_error_t (*addRecordFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* id,
                                                                           const indy_u8_t *  value,
                                                                           indy_u32_t         value_len,
                                                                           const char* tags_json),

                                                  indy_error_t (*updateRecordValueFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* id,
                                                                           const indy_u8_t *  value,
                                                                           indy_u32_t         value_len),

                                                  indy_error_t (*updateRecordTagsFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* id,
                                                                           const char* tags_json),

                                                  indy_error_t (*addRecordTagsFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* id,
                                                                           const char* tags_json),

                                                  indy_error_t (*deleteRecordTagsFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* id,
                                                                           const char* tags_names),

                                                  indy_error_t (*deleteRecordFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* id),

                                                  indy_error_t (*getRecordFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* id,
                                                                           const char* options_json,
                                                                           int32_t* record_handle),

                                                  indy_error_t (*getRecordIdFn)(indy_handle_t handle,
                                                                           indy_handle_t record_handle,
                                                                           char* id),

                                                  indy_error_t (*getRecordValueFn)(indy_handle_t handle,
                                                                           indy_handle_t record_handle,
                                                                           indy_u8_t *  value,
                                                                           indy_u32_t         value_len),

                                                  indy_error_t (*getRecordTagsFn)(indy_handle_t handle,
                                                                           indy_handle_t record_handle,
                                                                           char* tags_json),

                                                  indy_error_t (*freeRecordFn)(indy_handle_t handle,
                                                                           indy_handle_t record_handle),

                                                  indy_error_t (*getStorageMetadataFn)(indy_handle_t handle,
                                                                           char* metadata,
                                                                           indy_handle_t metadata_handle),

                                                  indy_error_t (*setStorageMetadataFn)(indy_handle_t handle,
                                                                           const char* metadata),

                                                  indy_error_t (*freeStorageMetadataFn)(indy_handle_t handle,
                                                                           indy_handle_t metadata_handle),

                                                  indy_error_t (*openSearchFn)(indy_handle_t handle,
                                                                           const char* type_,
                                                                           const char* query,
                                                                           const char* options,
                                                                           int32_t* search_handle),

                                                  indy_error_t (*openSearchAllFn)(indy_handle_t handle,
                                                                           indy_handle_t search_handle),

                                                  indy_error_t (*getSearchTotalCountFn)(indy_handle_t handle,
                                                                           indy_handle_t search_handle,
                                                                           indy_u32_t*         total_count),

                                                  indy_error_t (*fetchSearchNextRecordsFn)(indy_handle_t handle,
                                                                           indy_handle_t search_handle,
                                                                           indy_handle_t record_handle),

                                                  indy_error_t (*freeSearchFn)(indy_handle_t handle,
                                                                           indy_handle_t search_handle),

                                                  void         (*fn)(indy_handle_t command_handle_, indy_error_t err)
                                                  );

    /// Create a new secure wallet.
    ///
    /// #Params
    /// config: Wallet configuration json.
    /// {
    ///   "id": string, Identifier of the wallet.
    ///         Configured storage uses this identifier to lookup exact wallet data placement.
    ///   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
    ///                  'Default' storage type allows to store wallet data in the local file.
    ///                  Custom storage types can be registered with indy_register_wallet_storage call.
    ///   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
    ///                     Can be optional if storage supports default configuration.
    ///                     For 'default' storage type configuration is:
    ///   {
    ///     "path": optional<string>, Path to the directory with wallet files.
    ///             Defaults to $HOME/.indy_client/wallet.
    ///             Wallet will be stored in the file {path}/{id}/sqlite.db
    ///   }
    /// }
    /// credentials: Wallet credentials json
    /// {
    ///   "key": string, Key or passphrase used for wallet key derivation.
    ///                  Look to key_derivation_method param for information about supported key derivation methods.
    ///   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
    ///                          Can be optional if storage supports default configuration.
    ///                          For 'default' storage type should be empty.
    ///   "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
    ///                          ARGON2I_MOD - derive secured wallet master key (used by default)
    ///                          ARGON2I_INT - derive secured wallet master key (less secured but faster)
    ///                          RAW - raw wallet key master provided (skip derivation).
    ///                                RAW keys can be generated with indy_generate_wallet_key call
    /// }
    ///
    /// #Returns
    /// err: Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_create_wallet(indy_handle_t  command_handle,
                                           const char*    config,
                                           const char*    credentials,
                                           void           (*fn)(indy_handle_t command_handle_, indy_error_t err)
                                          );

    /// Open the wallet.
    ///
    /// Wallet must be previously created with indy_create_wallet method.
    ///
    /// #Params
    /// config: Wallet configuration json.
    ///   {
    ///       "id": string, Identifier of the wallet.
    ///             Configured storage uses this identifier to lookup exact wallet data placement.
    ///       "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
    ///                       'Default' storage type allows to store wallet data in the local file.
    ///                       Custom storage types can be registered with indy_register_wallet_storage call.
    ///       "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
    ///                         Can be optional if storage supports default configuration.
    ///                         For 'default' storage type configuration is:
    ///           {
    ///              "path": optional<string>, Path to the directory with wallet files.
    ///                      Defaults to $HOME/.indy_client/wallet.
    ///                      Wallet will be stored in the file {path}/{id}/sqlite.db
    ///           }
    ///
    ///   }
    /// credentials: Wallet credentials json
    ///   {
    ///       "key": string, Key or passphrase used for wallet key derivation.
    ///                      Look to key_derivation_method param for information about supported key derivation methods.
    ///       "rekey": optional<string>, If present than wallet master key will be rotated to a new one.
    ///       "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
    ///                              Can be optional if storage supports default configuration.
    ///                              For 'default' storage type should be empty.
    ///       "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
    ///                          ARGON2I_MOD - derive secured wallet master key (used by default)
    ///                          ARGON2I_INT - derive secured wallet master key (less secured but faster)
    ///                          RAW - raw wallet key master provided (skip derivation).
    ///                                RAW keys can be generated with indy_generate_wallet_key call
    ///       "rekey_derivation_method": optional<string> Algorithm to use for wallet rekey derivation:
    ///                          ARGON2I_MOD - derive secured wallet master rekey (used by default)
    ///                          ARGON2I_INT - derive secured wallet master rekey (less secured but faster)
    ///                          RAW - raw wallet key master provided (skip derivation).
    ///                                RAW keys can be generated with indy_generate_wallet_key call
    ///   }
    ///
    /// #Returns
    /// err: Error code
    /// handle: Handle to opened wallet to use in methods that require wallet access.
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_open_wallet(indy_handle_t  command_handle,
                                         const char*    config,
                                         const char*    credentials,
                                         void           (*fn)(indy_handle_t command_handle_, indy_error_t err, indy_handle_t handle)
                                        );

    /// Exports opened wallet
    ///
    /// #Params:
    /// wallet_handle: wallet handle returned by indy_open_wallet
    /// export_config: JSON containing settings for input operation.
    ///   {
    ///     "path": <string>, Path of the file that contains exported wallet content
    ///     "key": <string>, Key or passphrase used for wallet export key derivation.
    ///                     Look to key_derivation_method param for information about supported key derivation methods.
    ///     "key_derivation_method": optional<string> Algorithm to use for export key derivation:
    ///                              ARGON2I_MOD - derive secured export key (used by default)
    ///                              ARGON2I_INT - derive secured export key (less secured but faster)
    ///                              RAW - raw export key provided (skip derivation).
    ///                                RAW keys can be generated with indy_generate_wallet_key call
    ///   }
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_export_wallet(indy_handle_t  command_handle,
                                           indy_handle_t  wallet_handle,
                                           const char*    export_config_json,
                                           void           (*fn)(indy_handle_t command_handle_, indy_error_t err)
                                           );


    /// Creates a new secure wallet and then imports its content
    /// according to fields provided in import_config
    /// This can be seen as an indy_create_wallet call with additional content import
    ///
    /// #Params
    /// config: Wallet configuration json.
    /// {
    ///   "id": string, Identifier of the wallet.
    ///         Configured storage uses this identifier to lookup exact wallet data placement.
    ///   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
    ///                  'Default' storage type allows to store wallet data in the local file.
    ///                  Custom storage types can be registered with indy_register_wallet_storage call.
    ///   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
    ///                     Can be optional if storage supports default configuration.
    ///                     For 'default' storage type configuration is:
    ///   {
    ///     "path": optional<string>, Path to the directory with wallet files.
    ///             Defaults to $HOME/.indy_client/wallet.
    ///             Wallet will be stored in the file {path}/{id}/sqlite.db
    ///   }
    /// }
    /// credentials: Wallet credentials json
    /// {
    ///   "key": string, Key or passphrase used for wallet key derivation.
    ///                  Look to key_derivation_method param for information about supported key derivation methods.
    ///   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
    ///                          Can be optional if storage supports default configuration.
    ///                          For 'default' storage type should be empty.
    ///   "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
    ///                             ARGON2I_MOD - derive secured wallet master key (used by default)
    ///                             ARGON2I_INT - derive secured wallet master key (less secured but faster)
    ///                             RAW - raw wallet key master provided (skip derivation).
    ///                                RAW keys can be generated with indy_generate_wallet_key call
    /// }
    /// import_config: Import settings json.
    /// {
    ///   "path": <string>, path of the file that contains exported wallet content
    ///   "key": <string>, key used for export of the wallet
    /// }
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_import_wallet(indy_handle_t  command_handle,
                                           const char*    config,
                                           const char*    credentials,
                                           const char*    import_config_json,
                                           void           (*fn)(indy_handle_t command_handle_, indy_error_t err)
                                           );

    /// Closes opened wallet and frees allocated resources.
    ///
    /// #Params
    /// wallet_handle: wallet handle returned by indy_open_wallet.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_close_wallet(indy_handle_t  command_handle,
                                          indy_handle_t  wallet_handle,
                                          void           (*fn)(indy_handle_t command_handle_, indy_error_t err)
                                         );

    /// Deletes created wallet.
    ///
    /// #Params
    /// config: Wallet configuration json.
    /// {
    ///   "id": string, Identifier of the wallet.
    ///         Configured storage uses this identifier to lookup exact wallet data placement.
    ///   "storage_type": optional<string>, Type of the wallet storage. Defaults to 'default'.
    ///                  'Default' storage type allows to store wallet data in the local file.
    ///                  Custom storage types can be registered with indy_register_wallet_storage call.
    ///   "storage_config": optional<object>, Storage configuration json. Storage type defines set of supported keys.
    ///                     Can be optional if storage supports default configuration.
    ///                     For 'default' storage type configuration is:
    ///   {
    ///     "path": optional<string>, Path to the directory with wallet files.
    ///             Defaults to $HOME/.indy_client/wallet.
    ///             Wallet will be stored in the file {path}/{id}/sqlite.db
    ///   }
    /// }
    /// credentials: Wallet credentials json
    /// {
    ///   "key": string, Key or passphrase used for wallet key derivation.
    ///                  Look to key_derivation_method param for information about supported key derivation methods.
    ///   "storage_credentials": optional<object> Credentials for wallet storage. Storage type defines set of supported keys.
    ///                          Can be optional if storage supports default configuration.
    ///                          For 'default' storage type should be empty.
    ///   "key_derivation_method": optional<string> Algorithm to use for wallet key derivation:
    ///                             ARGON2I_MOD - derive secured wallet master key (used by default)
    ///                             ARGON2I_INT - derive secured wallet master key (less secured but faster)
    ///                             RAW - raw wallet key master provided (skip derivation).
    ///                                RAW keys can be generated with indy_generate_wallet_key call
    /// }
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_delete_wallet(indy_handle_t  command_handle,
                                           const char*    config,
                                           const char*    credentials,
                                           void           (*fn)(indy_handle_t command_handle_, indy_error_t err)
                                          );

    /// Generate wallet master key.
    /// Returned key is compatible with "RAW" key derivation method.
    /// It allows to avoid expensive key derivation for use cases when wallet keys can be stored in a secure enclave.
    ///
    /// #Params
    /// config: (optional) key configuration json.
    /// {
    ///   "seed": string, (optional) Seed that allows deterministic key creation (if not set random one will be created).
    ///                              Can be UTF-8, base64 or hex string.
    /// }
    ///
    /// #Returns
    /// err: Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*
    extern indy_error_t indy_generate_wallet_key(indy_handle_t     command_handle,
                                                 const char *const config,

                                                 void              (*cb)(indy_handle_t     command_handle,
                                                                         indy_error_t      err,
                                                                         const char *const key)
                                                );

#ifdef __cplusplus
}
#endif

#endif

