#ifndef __sovrin__pool_included__
#define __sovrin__pool_included__

#include "mod.h"

extern "C"
{
    extern ErrorCode sovrin_create_pool_ledger_config(long         command_handle,
                                                      const char * config_name,
                                                      const char * config,
                                                      ErrorCode    (*cb)(long xcommand_handle, ErrorCode err)
                                                      );
    
    extern ErrorCode sovrin_open_pool_ledger(long         command_handle,
                                             const char * config_name,
                                             const char * config,
                                             ErrorCode    (*cb)(long xcommand_handle, ErrorCode err, long pool_handle)
                                             );
    
    extern ErrorCode sovrin_refresh_pool_ledger(long       command_hangle,
                                                long       handle,
                                                ErrorCode  (*cb)(long xcommand_handle, ErrorCode err)
                                                );
    
    extern ErrorCode sovrin_close_pool_ledger(long       command_hangle,
                                              long       handle,
                                              ErrorCode  (*cb)(long xcommand_handle, ErrorCode err)
                                              );
    
    extern ErrorCode sovrin_delete_pool_ledger_config(long         command_handle,
                                                      const char * config_name,
                                                      ErrorCode    (*cb)(long xcommand_handle, ErrorCode err)
                                                      );    
}

#endif /* __sovrin__pool_included__ */

