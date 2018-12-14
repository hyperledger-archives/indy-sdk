//
//  IndyLogger.h
//  libindy
//

#import <Foundation/Foundation.h>

@interface IndyLogger : NSObject

/**
 Set default libindy logger implementation.

 Allows library user use `env_logger` logger as default implementation.
 More details about `env_logger` and its customization can be found here: https://crates.io/crates/env_logger

 NOTE: You should specify either `pattern` parameter or `RUST_LOG` environment variable to init logger.
 NOTE: Logger can be set only once.

 @param  pattern: (Optional) pattern that corresponds with the log messages to show.
 */
+ (void)setDefaultLogger:(NSString *)pattern;

/**
 Set custom libindy log function.

 NOTE: Logger can be set only once.

 @param  logCb: function will be called to log a record.
 */
+ (void)setLogger:(id)logCb;

+ (IndyLogger *)sharedInstance;

- (IndyLogger *)init;

@end

void logCallback(const void *context,
        uint32_t level,
        const char *target,
        const char *message,
        const char *modulePath,
        const char *file,
        uint32_t line);
