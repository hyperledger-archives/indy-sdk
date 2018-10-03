#ifndef __indy__blob_storage__included__
#define __indy__blob_storage__included__

#ifdef __cplusplus
extern "C" {
#endif


    extern indy_error_t indy_open_blob_storage_reader(indy_handle_t  command_handle,
                                                      const char*    type_,
                                                      const char*    config_json,
                                                      indy_handle_cb cb
                                                     );

    extern indy_error_t indy_open_blob_storage_writer(indy_handle_t  command_handle,
                                                      const char*    type_,
                                                      const char*    config_json,
                                                      indy_handle_cb cb
                                                     );


#ifdef __cplusplus
}
#endif

#endif

