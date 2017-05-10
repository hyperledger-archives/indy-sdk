#ifndef __sovrin__pool_included__
#define __sovrin__pool_included__

#include "sovrin_mod.h"
#include "sovrin_types.h"

#ifdef __cplusplus
extern "C" {
#endif

    extern sovrin_error_t sovrin_create_pool_ledger_config(sovrin_handle_t command_handle,
                                                           const char *    config_name,
                                                           const char *    config,
                                                           void            (*cb)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                           );
    
    extern sovrin_error_t sovrin_open_pool_ledger(sovrin_handle_t command_handle,
                                                  const char *    config_name,
                                                  const char *    config,
                                                  void            (*cb)(sovrin_handle_t xcommand_handle, sovrin_error_t err, sovrin_handle_t pool_handle)
                                                  );
    
    extern sovrin_error_t sovrin_refresh_pool_ledger(sovrin_handle_t command_hangle,
                                                     sovrin_handle_t handle,
                                                     void            (*cb)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                     );
    
    extern sovrin_error_t sovrin_close_pool_ledger(sovrin_handle_t command_hangle,
                                                   sovrin_handle_t handle,
                                                   void            (*cb)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                   );
    
    extern sovrin_error_t sovrin_delete_pool_ledger_config(sovrin_handle_t command_handle,
                                                           const char *    config_name,
                                                           void            (*cb)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                           );
#ifdef __cplusplus
}
#endif

#endif /* __sovrin__pool_included__ */


