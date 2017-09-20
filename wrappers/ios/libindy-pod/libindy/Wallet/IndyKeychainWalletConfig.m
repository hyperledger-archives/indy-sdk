//
//  IndyKeychainWalletConfig.m
//  libindy
//

#import "IndyKeychainWalletConfig.h"

@interface IndyKeychainWalletConfig()

+ (NSUInteger)defaultFreshnessTime;

@end


@implementation IndyKeychainWalletConfig

+ (NSUInteger)defaultFreshnessTime
{
    return 1000;
}

+ (IndyKeychainWalletConfig *)defaultConfig
{
    return [[IndyKeychainWalletConfig alloc] initWithFreshnessTime:[IndyKeychainWalletConfig defaultFreshnessTime]];
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
            self.freshnessTime = [IndyKeychainWalletConfig defaultFreshnessTime];
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

- (NSString *)toJson
{
    return [NSString stringWithFormat:@"{freshness_time: %lu }", (unsigned long)self.freshnessTime];
}
@end
