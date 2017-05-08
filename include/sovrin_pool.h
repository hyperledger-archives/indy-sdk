#ifndef __sovrin__pool_included__
#define __sovrin__pool_included__

#include "mod.h"

extern "C"
{
    extern sovrin_error_t sovrin_create_pool_ledger_config(sovrin_handle_t command_handle,
                                                           const char *    config_name,
                                                           const char *    config,
                                                           sovrin_error_t  (cb*)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                           );
    
    extern sovrin_error_t sovrin_open_pool_ledger(sovrin_handle_t command_handle,
                                                  const char *    config_name,
                                                  const char *    config,
                                                  sovrin_error_t  (cb*)(sovrin_handle_t xcommand_handle, sovrin_error_t err, sovrin_handle_t pool_handle)
                                                  );
    
    extern sovrin_error_t sovrin_refresh_pool_ledger(sovrin_handle_t command_hangle,
                                                     sovrin_handle_t handle,
                                                     sovrin_error_t  (cb*)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                     );
    
    extern sovrin_error_t sovrin_close_pool_ledger(sovrin_handle_t command_hangle,
                                                   sovrin_handle_t handle,
                                                   sovrin_error_t  (cb*)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                   );
    
    extern sovrin_error_t sovrin_delete_pool_ledger_config(sovrin_handle_t command_handle,
                                                           const char *    config_name,
                                                           sovrin_error_t  (cb*)(sovrin_handle_t xcommand_handle, sovrin_error_t err)
                                                           );
}

#endif /* __sovrin__pool_included__ */


