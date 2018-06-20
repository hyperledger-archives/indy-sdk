#ifndef __indy__pool_included__
#define __indy__pool_included__

#include "indy_mod.h"
#include "indy_types.h"

#ifdef __cplusplus
extern "C" {
#endif

    extern indy_error_t indy_create_pool_ledger_config(indy_handle_t command_handle,
                                                       const char *  config_name,
                                                       const char *  config,
                                                       void          (*cb)(indy_handle_t xcommand_handle, indy_error_t err)
                                                       );
    
    extern indy_error_t indy_open_pool_ledger(indy_handle_t command_handle,
                                              const char *  config_name,
                                              const char *  config,
                                              void          (*cb)(indy_handle_t xcommand_handle, indy_error_t err, indy_handle_t pool_handle)
                                              );
    
    extern indy_error_t indy_refresh_pool_ledger(indy_handle_t command_handle,
                                                 indy_handle_t handle,
                                                 void          (*cb)(indy_handle_t xcommand_handle, indy_error_t err)
                                                 );

    extern indy_error_t indy_list_pools(indy_handle_t command_handle,
                                        void          (*fn)(indy_handle_t xcommand_handle, indy_error_t err, const char *const pools)
                                        );
    
    extern indy_error_t indy_close_pool_ledger(indy_handle_t command_handle,
                                               indy_handle_t handle,
                                               void          (*cb)(indy_handle_t xcommand_handle, indy_error_t err)
                                               );
    
    extern indy_error_t indy_delete_pool_ledger_config(indy_handle_t command_handle,
                                                       const char *  config_name,
                                                       void          (*cb)(indy_handle_t xcommand_handle, indy_error_t err)
                                                       );

    extern indy_error_t indy_set_protocol_version(indy_handle_t command_handle,
                                                  indy_u64_t    protocol_version,
                                                  void          (*cb)(indy_handle_t xcommand_handle, indy_error_t err)
                                                  );
#ifdef __cplusplus
}
#endif

#endif /* __indy__pool_included__ */


