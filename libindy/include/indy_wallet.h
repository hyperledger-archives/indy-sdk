#ifndef __indy__wallet__included__
#define __indy__wallet__included__

#include "indy_mod.h"
#include "indy_types.h"

/// Create the wallet storage (For example, database creation)
///
/// #Params
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// credentials_json: wallet storage credentials (For example, database credentials)
/// metadata: wallet metadata (For example encrypted keys).
typedef indy_error_t (*indyCreateWalletCb)(const char* name,
                                           const char* config,
                                           const char* credentials_json,
                                           const char* metadata);

/// Open the wallet storage (For example, opening database connection)
///
/// #Params
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// credentials_json: wallet storage credentials (For example, database credentials)
/// storage_handle_p: pointer to store opened storage handle
typedef indy_error_t (*indyOpenWalletCb)(const char* name,
                                         const char* config,
                                         const char* credentials,
                                         indy_handle_t* handle);

/// Close the opened walled storage (For example, closing database connection)
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
typedef indy_error_t (*indyCloseWalletCb)(indy_handle_t handle);

/// Delete the wallet storage (For example, database deletion)
///
/// #Params
/// name: wallet storage name (the same as wallet name)
/// config: wallet storage config (For example, database config)
/// credentials_json: wallet storage credentials (For example, database credentials)
typedef indy_error_t (*indyDeleteWalletCb)(const char* name,
                                           const char* config,
                                           const char* credentials);

/// Create a new record in the wallet storage
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the value of record (pointer to buffer)
/// value_len: the value of record (buffer size)
/// tags_json: the record tags used for search and storing meta information as json:
///   {
///     "tagName1": "tag value 1", // string value
///     "tagName2": 123, // numeric value
///   }
///   Note that null means no tags
typedef indy_error_t (*indyWalletAddRecordCb)(indy_handle_t handle,
                                              const char* type_,
                                              const char* id,
                                              const indy_u8_t *  value,
                                              indy_u32_t         value_len,
                                              const char* tags_json);

/// Update a record value
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// value: the value of record (pointer to buffer)
/// value_len: the value of record (buffer size)
typedef indy_error_t (*indyWalletUpdateRecordValueCb)(indy_handle_t handle,
                                                      const char* type_,
                                                      const char* id,
                                                      const indy_u8_t *  value,
                                                      indy_u32_t         value_len);

/// Update a record tags
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the new record tags used for search and storing meta information as json:
///   {
///     "tagName1": "tag value 1", // string value
///     "tagName2": 123, // numeric value
///   }
///   Note that null means no tags
typedef indy_error_t (*indyWalletUpdateRecordTagsCb)(indy_handle_t handle,
                                                     const char* type_,
                                                     const char* id,
                                                     const char* tags_json);

/// Add new tags to the record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tags_json: the additional record tags as json:
///   {
///     "tagName1": "tag value 1", // string value
///     "tagName2": 123, // numeric value,
///     ...
///   }
///   Note that null means no tags
///   Note if some from provided tags already assigned to the record than
///     corresponding tags values will be replaced
typedef indy_error_t (*indyWalletAddRecordTagsCb)(indy_handle_t handle,
                                                  const char* type_,
                                                  const char* id,
                                                  const char* tags_json);

/// Delete tags from the record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// tag_names_json: the list of tag names to remove from the record as json array:
///   ["tagName1", "tagName2", ...]
///   Note that null means no tag names
typedef indy_error_t (*indyWalletDeleteRecordTagsCb)(indy_handle_t handle,
                                                     const char* type_,
                                                     const char* id,
                                                     const char* tags_names);

/// Delete an existing record in the wallet storage
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: record type
/// id: the id of record
typedef indy_error_t (*indyWalletDeleteRecordCb)(indy_handle_t handle,
                                                 const char* type_,
                                                 const char* id);

/// Get an wallet storage record by id
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// id: the id of record
/// options_json: //TODO: FIXME: Think about replacing by bitmask
///  {
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, true by default) Retrieve record tags
///  }
/// record_handle_p: pointer to store retrieved record handle
typedef indy_error_t (*indyWalletGetRecordCb)(indy_handle_t handle,
                                              const char* type_,
                                              const char* id,
                                              const char* options_json,
                                              int32_t* record_handle);

/// Get an id for retrieved wallet storage record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record id
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
typedef indy_error_t (*indyWalletGetRecordIdCb)(indy_handle_t handle,
                                                indy_handle_t record_handle,
                                                char* id);

/// Get an type for retrieved wallet storage record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record type
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
typedef indy_error_t (*indyWalletGetRecordTypeCb)(indy_handle_t handle,
                                                indy_handle_t record_handle,
                                                char* type_);

/// Get an value for retrieved wallet storage record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record value
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
///          Note that null be returned if no value retrieved
typedef indy_error_t (*indyWalletGetRecordValueCb)(indy_handle_t handle,
                                                   indy_handle_t record_handle,
                                                   indy_u8_t *  value,
                                                   indy_u32_t         value_len);

/// Get an tags for retrieved wallet record
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// record_handle: retrieved record handle (See get_record handler)
///
/// returns: record tags as json
///          Note that pointer lifetime the same as retrieved record lifetime
///            (until record_free called)
///          Note that null be returned if no tags retrieved
typedef indy_error_t (*indyWalletGetRecordTagsCb)(indy_handle_t handle,
                                                  indy_handle_t record_handle,
                                                  char* tags_json);

/// Free retrieved wallet record (make retrieved record handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open_wallet_storage)
/// record_handle: retrieved record handle (See wallet_storage_get_wallet_record)
typedef indy_error_t (*indyWalletFreeRecordCb)(indy_handle_t handle,
                                               indy_handle_t record_handle);

/// Get storage metadata
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
///
/// returns: metadata as base64 value
///          Note that pointer lifetime is static
typedef indy_error_t (*indyWalletGetStorageMetadataCb)(indy_handle_t handle,
                                                       char* metadata,
                                                       indy_handle_t metadata_handle);

/// Set storage metadata
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// metadata_p: base64 value of metadata
///
///   Note if storage already have metadata record it will be overwritten.
typedef indy_error_t (*indyWalletSetStorageMetadataCb)(indy_handle_t handle,
                                                       const char* metadata);

/// Free retrieved storage metadata record (make retrieved storage metadata handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open_wallet_storage)
/// metadata_handle: retrieved record handle (See wallet_storage_get_storage_metadata)
typedef indy_error_t (*indyWalletFreeStorageMetadataCb)(indy_handle_t handle,
                                                        indy_handle_t metadata_handle);

/// Search for wallet storage records
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// type_: allows to separate different record types collections
/// query_json: MongoDB style query to wallet record tags:
///  {
///    "tagName": "tagValue",
///    $or: {
///      "tagName2": { $regex: 'pattern' },
///      "tagName3": { $gte: 123 },
///    },
///  }
/// options_json: //TODO: FIXME: Think about replacing by bitmask
///  {
///    retrieveRecords: (optional, true by default) If false only "counts" will be calculated,
///    retrieveTotalCount: (optional, false by default) Calculate total count,
///    retrieveType: (optional, false by default) Retrieve record type,
///    retrieveValue: (optional, true by default) Retrieve record value,
///    retrieveTags: (optional, true by default) Retrieve record tags,
///  }
/// search_handle_p: pointer to store wallet search handle
typedef indy_error_t (*indyWalletOpenSearchCb)(indy_handle_t handle,
                                               const char* type_,
                                               const char* query,
                                               const char* options,
                                               int32_t* search_handle);

/// Search for all wallet storage records
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle_p: pointer to store wallet search handle
typedef indy_error_t (*indyWalletOpenSearchAllCb)(indy_handle_t handle,
                                                  indy_handle_t search_handle);

/// Get total count of records that corresponds to wallet storage search query
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: total count of records that corresponds to wallet storage search query
///          Note -1 will be returned if retrieveTotalCount set to false for search_records
typedef indy_error_t (*indyWalletGetSearchTotalCountCb)(indy_handle_t handle,
                                                        indy_handle_t search_handle,
                                                        indy_u32_t*         total_count);

/// Get the next wallet storage record handle retrieved by this wallet search.
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
///
/// returns: record handle (the same as for get_record handler)
///          Note if no more records WalletNoRecords error will be returned
typedef indy_error_t (*indyWalletFetchSearchNextRecordsCb)(indy_handle_t handle,
                                                           indy_handle_t search_handle,
                                                           indy_handle_t record_handle);

/// Free wallet search (make search handle invalid)
///
/// #Params
/// storage_handle: opened storage handle (See open handler)
/// search_handle: wallet search handle (See search_records handler)
typedef indy_error_t (*indyWalletFreeSearchCb)(indy_handle_t handle,
                                               indy_handle_t search_handle);

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


    extern indy_error_t indy_register_wallet_storage(indy_handle_t  command_handle,
                                                      const char*    type_,
                                                      indyCreateWalletCb create_wallet_cb,
                                                      indyOpenWalletCb open_wallet_cb,
                                                      indyCloseWalletCb close_wallet_cb,
                                                      indyDeleteWalletCb delete_wallet_cb,
                                                      indyWalletAddRecordCb add_record_cb,
                                                      indyWalletUpdateRecordValueCb update_record_value,
                                                      indyWalletUpdateRecordTagsCb update_record_tags_cb,
                                                      indyWalletAddRecordTagsCb add_record_tags_cb,
                                                      indyWalletDeleteRecordTagsCb delete_record_tags_cb,
                                                      indyWalletDeleteRecordCb delete_record_cb,
                                                      indyWalletGetRecordCb get_record_cb,
                                                      indyWalletGetRecordIdCb get_record_id_cb,
                                                      indyWalletGetRecordTypeCb get_record_type_cb,
                                                      indyWalletGetRecordValueCb get_record_value_cb,
                                                      indyWalletGetRecordTagsCb get_records_tags_cb,
                                                      indyWalletFreeRecordCb free_record_cb,
                                                      indyWalletGetStorageMetadataCb get_storage_metadata_cb,
                                                      indyWalletSetStorageMetadataCb set_storage_metadata_cb,
                                                      indyWalletFreeStorageMetadataCb free_storage_metadata_cb,
                                                      indyWalletOpenSearchCb open_search_cb,
                                                      indyWalletOpenSearchAllCb open_search_all_cb,
                                                      indyWalletGetSearchTotalCountCb get_search_total_count_cb,
                                                      indyWalletFetchSearchNextRecordsCb fetch_search_next_record_cb,
                                                      indyWalletFreeSearchCb free_search_cb,
                                                      indy_empty_cb cb
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
    ///             Defaults to $HOME/.indy_client/wallets.
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
                                           indy_empty_cb cb
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
    ///                      Defaults to $HOME/.indy_client/wallets.
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
                                         indy_handle_cb cb
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
                                           indy_empty_cb cb
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
    ///             Defaults to $HOME/.indy_client/wallets.
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
                                           indy_empty_cb cb
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
                                          indy_empty_cb cb
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
    ///             Defaults to $HOME/.indy_client/wallets.
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
                                           indy_empty_cb cb
                                          );

    /// Generate wallet master key.
    /// Returned key is compatible with "RAW" key derivation method.
    /// It allows to avoid expensive key derivation for use cases when wallet keys can be stored in a secure enclave.
    ///
    /// #Params
    /// config: (optional) key configuration json.
    /// {
    ///   "seed": optional<string> Seed that allows deterministic key creation (if not set random one will be used).
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
                                                 indy_str_cb cb
                                                );

#ifdef __cplusplus
}
#endif

#endif

