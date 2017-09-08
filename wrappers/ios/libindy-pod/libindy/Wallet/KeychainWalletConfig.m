//
//  KeychainWalletConfig.m
//  libindy
//
//  Created by Anastasia Tarasova on 08/09/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "KeychainWalletConfig.h"

@interface KeychainWalletConfig()

+ (NSUInteger)defaultFreshnessTime;

@end


@implementation KeychainWalletConfig

+ (NSUInteger)defaultFreshnessTime
{
    return 1000;
}

+ (KeychainWalletConfig *)defaultConfig
{
    return [[KeychainWalletConfig alloc] initWithFreshnessTime:[KeychainWalletConfig defaultFreshnessTime]];
}

- (instancetype)initWithJson:(NSDictionary* _Nonnull)json
{
    self = [super init];
    if (self)
    {
        NSUInteger time = [json[@"freshness_time"] integerValue];
        if (time != 0)
        {
            self.freshnessTime = time;
        }
        else
        {
            self.freshnessTime = [KeychainWalletConfig defaultFreshnessTime];
        }
    }
    
    return self;
}

- (instancetype)initWithFreshnessTime:(NSUInteger)freshnessTime
{
    self = [super init];
    if (self)
    {
        self.freshnessTime = freshnessTime;
    }
    
    return self;
}

@end
