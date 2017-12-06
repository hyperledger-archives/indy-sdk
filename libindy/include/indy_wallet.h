#ifndef __indy__wallet__included__
#define __indy__wallet__included__

#ifdef __cplusplus
extern "C" {
#endif

    /// Registers custom wallet implementation.
    ///
    /// It allows library user to provide custom wallet implementation.
    ///
    /// #Params
    /// command_handle: Command handle to map callback to caller context.
    /// xtype: Wallet type name.
    /// create: WalletType create operation handler
    /// open: WalletType open operation handler
    /// set: Wallet set operation handler
    /// get: Wallet get operation handler
    /// get_not_expired: Wallet get_not_expired operation handler
    /// list: Wallet list operation handler
    /// close: Wallet close operation handler
    /// delete: WalletType delete operation handler
    /// free: Handler that allows to de-allocate strings allocated in caller code
    ///
    /// #Returns
    /// Error code
    

    extern indy_error_t indy_register_wallet_type(indy_handle_t  command_handle,
                                                  const char*    xtype,
                                                  indy_error_t (*createFn)(const char* name,
                                                                             const char* config,
                                                                             const char* credentials),

                                                  indy_error_t (*openFn)(const char* name,
                                                                           const char* config,
                                                                           const char* runtime_config,
                                                                           const char* credentials,
                                                                           indy_handle_t* handle),

                                                  indy_error_t (*setFn)(indy_handle_t handle,
                                                                          const char* key,
                                                                          const char* value),

                                                  indy_error_t (*getFn)(indy_handle_t handle,
                                                                          const char* key,
                                                                          const char ** const value_ptr),

                                                  indy_error_t (*getNotExpiredFn)(indy_handle_t handle,
                                                                          const char* key,
                                                                          const char ** const value_ptr),

                                                  indy_error_t (*listFn)(indy_handle_t handle,
                                                                          const char* key,
                                                                          const char ** const values_json_ptr),

                                                  indy_error_t (*closeFn)(indy_handle_t handle),
                                                  indy_error_t (*deleteFn)(const char* name,
                                                                             const char* config,
                                                                             const char* credentials),

                                                  indy_error_t (*freeFn)(indy_handle_t handle, const char* str),
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
    /// credentials(optional): Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default config will be used.
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
    /// credentials(optional): Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
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
    /// credentials(optional): Wallet credentials json. List of supported keys are defined by wallet type.
    ///                    if NULL, then default credentials will be used.
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

