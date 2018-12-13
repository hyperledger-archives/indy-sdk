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

extern void LogCb(const void *context,
        uint32_t level,
        const char *target,
        const char *message,
        const char *modulePath,
        const char *file,
        uint32_t line);

void logMessage(uint32_t level,
        const char *message,
        const char *file,
        uint32_t line);

@interface VcxLogger : NSObject

+ (void)setLogger;

@end
