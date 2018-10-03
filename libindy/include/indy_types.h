#ifndef __indy__types__included__
#define __indy__types__included__

#include <stdint.h>
#include "indy_mod.h"

typedef uint8_t       indy_u8_t;
typedef uint32_t      indy_u32_t;
typedef int32_t       indy_i32_t;
typedef int32_t       indy_handle_t;
typedef unsigned int  indy_bool_t;
typedef long long     indy_i64_t;
typedef unsigned long long     indy_u64_t;

typedef void (*indy_empty_cb)(indy_handle_t xcommand_handle,
                              indy_error_t  err);

typedef void (*indy_handle_cb)(indy_handle_t xcommand_handle,
                               indy_error_t  err,
                               indy_handle_t handle);

typedef void (*indy_handle_u32_cb)(indy_handle_t xcommand_handle,
                                   indy_error_t  err,
                                   indy_handle_t handle,
                                   indy_u32_t    num);

typedef void (*indy_str_cb)(indy_handle_t xcommand_handle,
                            indy_error_t  err,
                            const char*   str1);

typedef void (*indy_str_str_cb)(indy_handle_t xcommand_handle,
                                indy_error_t  err,
                                const char*   str1,
                                const char*   str2);

typedef void (*indy_str_str_str_cb)(indy_handle_t xcommand_handle,
                                    indy_error_t  err,
                                    const char*   str1,
                                    const char*   str2,
                                    const char*   str3);

typedef void (*indy_bool_cb)(indy_handle_t xcommand_handle,
                             indy_error_t  err,
                             indy_bool_t   flag);

typedef void (*indy_slice_cb)(indy_handle_t xcommand_handle,
                             indy_error_t  err,
                             const indy_u8_t* slice,
                             indy_u32_t       slice_len);

typedef void (*indy_str_slice_cb)(indy_handle_t xcommand_handle,
                                  indy_error_t  err,
                                  const char *      str1,
                                  const indy_u8_t* slice,
                                  indy_u32_t       slice_len);

typedef void (*indy_str_str_long_cb)(indy_handle_t xcommand_handle,
                                     indy_error_t  err,
                                     const char*   str1,
                                     const char*   str2,
                                     unsigned long long      num);

typedef indy_error_t (*indy_err_str_cb)(indy_handle_t xcommand_handle,
                                        indy_error_t  err,
                                        const char*   str1);

#endif
