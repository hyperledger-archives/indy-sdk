//
//  VcxLogger.h
//  vcx
//
//  Created by Evernym on 12/13/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#import <Foundation/Foundation.h>
#import "vcx.h"
#include "vcx.h"

@interface VcxLogger : NSObject

/**
 Set default libvcx logger implementation.

 Allows library user use `env_logger` logger as default implementation.
 More details about `env_logger` and its customization can be found here: https://crates.io/crates/env_logger

 NOTE: You should specify either `pattern` parameter or `RUST_LOG` environment variable to init logger.
 NOTE: Logger can be set only once.

 @param  pattern: (Optional) pattern that corresponds with the log messages to show.
 */
+ (void)setDefaultLogger:(NSString *)pattern;

/**
 Set custom libvcx log function.

 NOTE: Logger can be set only once.

 @param  logCb: function will be called to log a record.
 */
+ (void)setLogger:(id)logCb;

+ (VcxLogger *)sharedInstance;

- (VcxLogger *)init;

@end

void logCallback(const void *context,
        uint32_t level,
        const char *target,
        const char *message,
        const char *modulePath,
        const char *file,
        uint32_t line);
