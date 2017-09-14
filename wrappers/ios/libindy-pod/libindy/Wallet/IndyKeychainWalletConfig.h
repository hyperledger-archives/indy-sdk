//
//  IndyKeychainWalletConfig.h
//  libindy
//

#import <Foundation/Foundation.h>

@interface IndyKeychainWalletConfig: NSObject

@property (assign, readwrite) NSUInteger freshnessTime;

+ (IndyKeychainWalletConfig *_Nonnull) defaultConfig;

- (NSString *_Nonnull)toJson;

- (instancetype _Nonnull )initWithJson:(NSDictionary * _Nonnull)json;

- (instancetype _Nonnull )initWithFreshnessTime:(NSUInteger)freshnessTime;

@end
