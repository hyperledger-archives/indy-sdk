//
//  VcxLogger.m
//  vcx
//
//  Created by Evernym on 12/13/18.
//  Copyright © 2018 GuestUser. All rights reserved.
//

#import "VcxLogger.h"
#import "NSError+VcxError.h"
#import "VcxTypes.h"
#import "VcxErrors.h"
#include "vcx.h"

#define levelMappings @{@"1": @"Error", @"2": @"Warning", @"3": @"Info", @"4": @"Debug", @"5": @"Trace"}

void LogCb(const void *context,
        uint32_t level,
        const char *target,
        const char *message,
        const char *modulePath,
        const char *file,
        uint32_t line) {

#ifdef DEBUG
    logMessage(level, message, file, line);
#else
    if(@(level) <= @(3)) {
        logMessage(level, message, file, line);
    }
#endif
}

void logMessage(uint32_t level,
        const char *message,
        const char *file,
        uint32_t line) {
    NSLog(@"%@    %@:%@ | %@", [levelMappings valueForKey:[NSString stringWithFormat:@"%@", @(level)]],
            [NSString stringWithUTF8String:file],
            @(line),
            [NSString stringWithUTF8String:message]);
}

@implementation VcxLogger : NSObject

+ (void)setLogger {
    vcx_set_logger(nil, nil, LogCb, nil);
}

@end

