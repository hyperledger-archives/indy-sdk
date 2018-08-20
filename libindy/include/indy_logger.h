#ifndef __indy__logger_included__
#define __indy__logger_included__

#include "indy_mod.h"
#include "indy_types.h"

#ifdef __cplusplus
extern "C" {
#endif

    /// Set custom logger implementation.
    ///
    /// Allows library user to provide custom logger implementation as set of handlers.
    ///
    /// #Params
    /// context: logger context
    /// enabled: "enabled" operation handler (false positive if not specified)
    /// log: "log" operation handler
    /// flush: "flush" operation handler
    ///
    /// #Returns
    /// Error code

    extern indy_error_t indy_set_logger(const void*  context,
                                        indy_bool_t (*enabledFn)(const void*  context,
                                                                 indy_u32_t level,
                                                                 const char* target),
                                        void (*logFn)(const void*  context,
                                                      indy_u32_t level,
                                                      const char* target,
                                                      const char* message,
                                                      const char* module_path,
                                                      const char* file,
                                                      indy_u32_t line),
                                        void (*flushFn)(const void*  context)
                                                  );

    /// Set default logger implementation.
    ///
    /// Allows library user use default "environment" logger implementation.
    ///
    /// #Params
    /// level: min level of message to log
    ///
    /// #Returns
    /// Error code
    
    extern indy_error_t indy_set_default_logger(const char *  level );

    /// Get the currently used logger.
    ///
    /// NOTE: if logger is not set dummy implementation would be returned
    ///
    /// #Params
    /// `logger_p` - Reference that will contain logger pointer.
    ///
    /// #Returns
    /// Error code

    extern indy_error_t indy_get_logger(const void*  logger_p);

#ifdef __cplusplus
}
#endif

#endif
