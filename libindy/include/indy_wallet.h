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
                                                                           const char* runtime_config,
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
                                                                          int32_t* record_handle), // TODO: clarify mutable param

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

                                                  void           (*fn)(indy_handle_t xcommand_handle, indy_error_t err)
                                                  );

    /// Creates a new secure wallet with the given unique name.
    ///
    /// #Params
    /// pool_name: Name of the pool that corresponds to this wallet.
    /// name: Name of the wallet.
    /// xtype(optional): Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with indy_register_wallet_type call.
    /// config(optional): Wallet configuration json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
    /// credentials: Wallet credentials json
    ///   {
    ///       "key": string,
    ///       "rekey": Optional<string>,
    ///       "storage": Optional<object>  List of supported keys are defined by wallet type.
    ///
    ///   }
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_create_wallet(indy_handle_t  command_handle,
                                           const char*    pool_name,
                                           const char*    name,
                                           const char*    xtype,
                                           const char*    config,
                                           const char*    credentials,
                                           void           (*fn)(indy_handle_t xcommand_handle, indy_error_t err)
                                          );
    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with indy_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// #Params
    /// name: Name of the wallet.
    /// runtime_config (optional): Runtime wallet configuration json. if NULL, then default runtime_config will be used. Example:
    /// {
    ///     "freshness_time": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
    ///     ... List of additional supported keys are defined by wallet type.
    /// }
    /// credentials: Wallet credentials json
    ///   {
    ///       "key": string,
    ///       "rekey": Optional<string>,
    ///       "storage": Optional<object>  List of supported keys are defined by wallet type.
    ///
    ///   }
    ///
    /// #Returns
    /// Handle to opened wallet to use in methods that require wallet access.
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_open_wallet(indy_handle_t  command_handle,
                                         const char*    name,
                                         const char*    runtime_config,
                                         const char*    credentials,
                                         void           (*fn)(indy_handle_t xcommand_handle, indy_error_t err, indy_handle_t handle)
                                        );

    /// Lists created wallets as JSON array with each wallet metadata: name, type, name of associated pool
    extern indy_error_t indy_list_wallets(indy_handle_t command_handle,
                                          void          (*fn)(indy_handle_t xcommand_handle, indy_error_t err, const char *const wallets)
                                          );

    /// Closes opened wallet and frees allocated resources.
    ///
    /// #Params
    /// handle: wallet handle returned by indy_open_wallet.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_close_wallet(indy_handle_t  command_handle,
                                          indy_handle_t  handle,
                                          void           (*fn)(indy_handle_t xcommand_handle, indy_error_t err)
                                         );

    /// Deletes created wallet.
    ///
    /// #Params
    /// name: Name of the wallet to delete.
    /// credentials: Wallet credentials json
    ///   {
    ///       "key": string,
    ///       "rekey": Optional<string>,
    ///       "storage": Optional<object>  List of supported keys are defined by wallet type.
    ///
    ///   }
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern indy_error_t indy_delete_wallet(indy_handle_t  command_handle,
                                           const char*    name,
                                           const char*    credentials,
                                           void           (*fn)(indy_handle_t xcommand_handle, indy_error_t err)
                                          );

#ifdef __cplusplus
}
#endif

#endif

