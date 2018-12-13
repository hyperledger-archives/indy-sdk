//
//  IndyLogger.m
//  Indy
//
//  Created by Evernym on 12/12/18.
//  Copyright © 2018 Hyperledger. All rights reserved.
//

#import <Foundation/Foundation.h>
#include "indy_core.h"
#import "IndyLogger.h"

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

@implementation IndyLogger : NSObject

+ (void)setLogger {
    indy_set_logger(nil, nil, LogCb, nil);
}

@end
