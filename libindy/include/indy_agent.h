#ifndef __indy__agent__included__
#define __indy__agent__included__

#ifdef __cplusplus
extern "C" {
#endif

    extern indy_error_t indy_pack_message(indy_handle_t     command_handle,
                                        indy_handle_t     wallet_handle,
                                        const char *const ,

                                        void              (*cb)(indy_handle_t     command_handle,
                                                                indy_error_t      err,
                                                                const char *const vk)
                                       );