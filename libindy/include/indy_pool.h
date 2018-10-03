#ifndef __indy__pool_included__
#define __indy__pool_included__

#include "indy_mod.h"
#include "indy_types.h"

#ifdef __cplusplus
extern "C" {
#endif

    /// Creates a new local pool ledger configuration that can be used later to connect pool nodes.
    ///
    /// #Params
    /// config_name: Name of the pool ledger configuration.
    /// config (optional): Pool configuration json. if NULL, then default config will be used. Example:
    /// {
    ///     "genesis_txn": string (optional), A path to genesis transaction file. If NULL, then a default one will be used.
    ///                    If file doesn't exists default one will be created.
    /// }
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Ledger*
    extern indy_error_t indy_create_pool_ledger_config(indy_handle_t command_handle,
                                                       const char *  config_name,
                                                       const char *  config,
                                                       indy_empty_cb cb
                                                       );

    /// Opens pool ledger and performs connecting to pool nodes.
    ///
    /// Pool ledger configuration with corresponded name must be previously created
    /// with indy_create_pool_ledger_config method.
    /// It is impossible to open pool with the same name more than once.
    ///
    /// config_name: Name of the pool ledger configuration.
    /// config (optional): Runtime pool configuration json.
    ///                         if NULL, then default config will be used. Example:
    /// {
    ///     "timeout": int (optional), timeout for network request (in sec).
    ///     "extended_timeout": int (optional), extended timeout for network request (in sec).
    ///     "preordered_nodes": array<string> -  (optional), names of nodes which will have a priority during request sending:
    ///         ["name_of_1st_prior_node",  "name_of_2nd_prior_node", .... ]
    ///         Note: Not specified nodes will be placed in a random way.
    /// }
    ///
    /// #Returns
    /// Handle to opened pool to use in methods that require pool connection.
    ///
    /// #Errors
    /// Common*
    /// Ledger*
    extern indy_error_t indy_open_pool_ledger(indy_handle_t command_handle,
                                              const char *  config_name,
                                              const char *  config,
                                              indy_handle_cb cb
                                              );

    /// Refreshes a local copy of a pool ledger and updates pool nodes connections.
    ///
    /// #Params
    /// handle: pool handle returned by indy_open_pool_ledger
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Ledger*
    extern indy_error_t indy_refresh_pool_ledger(indy_handle_t command_handle,
                                                 indy_handle_t handle,
                                                 indy_empty_cb cb
                                                 );

    /// Lists names of created pool ledgers
    ///
    /// #Params
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    extern indy_error_t indy_list_pools(indy_handle_t command_handle,
                                        indy_str_cb cb
                                        );

    /// Closes opened pool ledger, opened nodes connections and frees allocated resources.
    ///
    /// #Params
    /// handle: pool handle returned by indy_open_pool_ledger.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Ledger*
    extern indy_error_t indy_close_pool_ledger(indy_handle_t command_handle,
                                               indy_handle_t handle,
                                               indy_empty_cb cb
                                               );

    /// Deletes created pool ledger configuration.
    ///
    /// #Params
    /// config_name: Name of the pool ledger configuration to delete.
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    /// Ledger*
    extern indy_error_t indy_delete_pool_ledger_config(indy_handle_t command_handle,
                                                       const char *  config_name,
                                                       indy_empty_cb cb
                                                       );

    /// Set PROTOCOL_VERSION to specific version.
    ///
    /// There is a global property PROTOCOL_VERSION that used in every request to the pool and
    /// specified version of Indy Node which Libindy works.
    ///
    /// By default PROTOCOL_VERSION=1.
    ///
    /// #Params
    /// protocol_version: Protocol version will be used:
    ///     1 - for Indy Node 1.3
    ///     2 - for Indy Node 1.4 and greater
    ///
    /// #Returns
    /// Error code
    ///
    /// #Errors
    /// Common*
    extern indy_error_t indy_set_protocol_version(indy_handle_t command_handle,
                                                  indy_u64_t    protocol_version,
                                                  indy_empty_cb cb
                                                  );
#ifdef __cplusplus
}
#endif

#endif /* __indy__pool_included__ */


