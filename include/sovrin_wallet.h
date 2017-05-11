#ifndef __sovrin__wallet__included__
#define __sovrin__wallet__included__

#ifdef __cplusplus
extern "C" {
#endif

    /// Registers custom wallet implementation.
    ///
    /// It allows library user to provide custom wallet implementation.
    ///
    /// #Params
    /// xtype: Wallet type name.
    /// create: create operation handler
    /// create: open operation handler
    /// set: set operation handler
    /// get: get operation handler
    /// close: close operation handler
    /// free: free operation handler
    ///
    /// #Returns
    /// error code
    ///
    /// #Errors
    /// CommonInvalidParam1
    /// CommonInvalidParam2
    /// CommonInvalidParam3
    /// CommonInvalidParam4
    /// CommonInvalidParam5
    /// WalletTypeAlreadyRegistered
    

    extern sovrin_error_t sovrin_register_wallet_type(const char* xtype,
                                                      sovrin_error_t (*create)(const char* name,
                                                                               const char* config,
                                                                               const char* credentials),
                                                      
                                                      sovrin_error_t (*open)(const char* name,
                                                                             const char* config,
                                                                             const char* credentials,
                                                                             sovrin_handle_t* handle),
                                                      
                                                      sovrin_error_t (*set)(sovrin_handle_t handle,
                                                                            const char* key,
                                                                            const char* sub_key,
                                                                            const char* value),
                                                      
                                                      sovrin_error_t (*get)(sovrin_handle_t handle,
                                                                            const char* key,
                                                                            const char* sub_key,
                                                                            const char* value_ptr,
                                                                            const char* value_life_time),
                                                      
                                                      sovrin_error_t (*close)(sovrin_handle_t handle),
                                                      sovrin_error_t (*delete)(const char* name),
                                                      sovrin_error_t (*free)(sovrin_handle_t handle, const char* str)
                                                      );

    /// Creates a new secure wallet with the given unique name.
    ///
    /// #Params
    /// pool_name: Name of the pool that corresponds to this wallet.
    /// name: Name of the wallet.
    /// xtype(optional): Type of the wallet. Defaults to 'default'.
    ///                  Custom types can be registered with sovrin_register_wallet_type call.
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

    extern sovrin_error_t sovrin_create_wallet(sovrin_handle_t  command_handle,
                                               const char*      pool_name,
                                               const char*      name,
                                               const char*      xtype,
                                               const char*      config,
                                               const char*      credentials,
                                               void            (*fn)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                              );
    /// Opens the wallet with specific name.
    ///
    /// Wallet with corresponded name must be previously created with sovrin_create_wallet method.
    /// It is impossible to open wallet with the same name more than once.
    ///
    /// #Params
    /// name: Name of the wallet.
    /// runtime_config (optional): Runtime wallet configuration json. if NULL, then default runtime_config will be used. Example:
    /// {
    ///     "freshnessTime": string (optional), Amount of minutes to consider wallet value as fresh. Defaults to 24*60.
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

    extern sovrin_error_t sovrin_open_wallet(sovrin_handle_t  command_handle,
                                             const char*      name,
                                             const char*      runtime_config,
                                             const char*      credentials,
                                             void            (*fn)(sovrin_handle_t xcommand_handle, sovrin_error_t err, sovrin_handle_t handle)
                                            );

    /// Closes opened wallet and frees allocated resources.
    ///
    /// #Params
    /// handle: wallet handle returned by sovrin_open_wallet.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern sovrin_error_t sovrin_close_wallet(sovrin_handle_t  command_handle,
                                              sovrin_handle_t  handle,
                                              void            (*fn)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
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

    extern sovrin_error_t sovrin_delete_wallet(sovrin_handle_t  command_handle,
                                               const char*      name,
                                               const char*      credentials,
                                               void            (*fn)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                              );
    
    /// Sets a seq_no (the corresponding Ledger transaction unique sequence number) for the a value
    /// in a secure wallet identified by the given string.
    /// The string identifying the value in the wallet is returned when the value is stored in the wallet.
    ///
    /// #Params
    /// wallet_handle: wallet handler (created by open_wallet).
    /// command_handle: command handle to map callback to user context.
    /// wallet_key: unique string identifying the value in the wallet.
    /// seq_no: transaction sequence number.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Wallet*

    extern sovrin_error_t sovrin_wallet_set_seq_no_for_value(sovrin_handle_t  command_handle,
                                                             sovrin_handle_t  wallet_handle,
                                                             const char*      wallet_key,
                                                             sovrin_i32_t     seq_no,
                                                             void            (*fn)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                             );

#ifdef __cplusplus
}
#endif

#endif

