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

@interface IndyLogger ()

@property(strong, readwrite) NSMutableArray *callbacks;

@end

@implementation IndyLogger : NSObject

+ (void)setDefaultLogger:(NSString *)pattern {
    indy_set_default_logger([pattern UTF8String]);
}

+ (void)setLogger:(id)logCb {
    [IndyLogger sharedInstance].callbacks[0] = [logCb copy];
    indy_set_logger(nil, nil, logCallback, nil);
}

+ (IndyLogger *)sharedInstance {
    static IndyLogger *instance = nil;
    static dispatch_once_t dispatch_once_block;
    
    dispatch_once(&dispatch_once_block, ^{
        instance = [IndyLogger new];
    });
    
    return instance;
}

- (IndyLogger *)init {
    self = [super init];
    if (self) {
        self.callbacks = [[NSMutableArray alloc] init];
    }
    return self;
}

void logCallback(const void *context,
        uint32_t level,
        const char *target,
        const char *message,
        const char *modulePath,
        const char *file,
        uint32_t line) {
    id block = [IndyLogger sharedInstance].callbacks[0];

    void (^completion)(NSObject *, NSNumber *, NSString *, NSString *, NSString *, NSString *, NSNumber *) =
    (void (^)(NSObject *context, NSNumber *level, NSString *target, NSString *message, NSString *modulePath, NSString *file, NSNumber *line)) block;
    NSObject *sarg0 = (__bridge NSObject*)context;
    NSNumber *sarg1 = @(level);
    NSString *sarg2 = [NSString stringWithUTF8String:target];
    NSString *sarg3 = [NSString stringWithUTF8String:message];
    NSString *sarg4 = [NSString stringWithUTF8String:modulePath];
    NSString *sarg5 = [NSString stringWithUTF8String:file];
    NSNumber *sarg6 = @(line);

    if (completion) {
        completion(sarg0, sarg1, sarg2, sarg3, sarg4, sarg5, sarg6);
    }
}

@end
