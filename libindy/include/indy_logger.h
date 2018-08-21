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
    /// context: pointer to some logger context that will be available in logger handlers.
    /// enabled: (optional) "enabled" operation handler - calls to determines if a log record would be logged. (false positive if not specified)
    /// log: "log" operation handler - calls to logs a record.
    /// flush: (optional) "flush" operation handler - calls to flushes buffered records (in case of crash or signal).
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
    /// Allows library user use `env_logger` logger as default implementation.
    /// More details about `env_logger` and its customization can be found here: https://crates.io/crates/env_logger
    ///
    /// #Params
    /// pattern: (optional) pattern that corresponds with the log messages to show.
    ///
    /// NOTE: You should specify either `pattern` parameter or `RUST_LOG` environment variable to init logger.
    ///
    /// #Returns
    /// Error code
    
    extern indy_error_t indy_set_default_logger(const char *  pattern );

    /// Get the currently used logger.
    ///
    /// NOTE: if logger is not set dummy implementation would be returned.
    ///
    /// #Params
    /// `context_p` - Reference that will contain logger context.
    /// `enabled_cb_p` - Reference that will contain pointer to enable operation handler.
    /// `log_cb_p` - Reference that will contain pointer to log operation handler.
    /// `flush_cb_p` - Reference that will contain pointer to flush operation handler.
    ///
    /// #Returns
    /// Error code

    extern indy_error_t indy_get_logger(const void*  indy_get_logger,
                                        indy_bool_t (**enabledFn)(const void*  context,
                                                                 indy_u32_t level,
                                                                 const char* target),
                                        void (**logFn)(const void*  context,
                                                      indy_u32_t level,
                                                      const char* target,
                                                      const char* message,
                                                      const char* module_path,
                                                      const char* file,
                                                      indy_u32_t line),
                                        void (**flushFn)(const void*  context)
                                                  );

#ifdef __cplusplus
}
#endif

#endif
