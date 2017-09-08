//
//  KeychainWalletConfig.h
//  libindy
//

#import <Foundation/Foundation.h>

@interface KeychainWalletConfig: NSObject

@property (assign, readwrite) NSUInteger freshnessTime;

+ (KeychainWalletConfig *_Nonnull) defaultConfig;

- (NSString *_Nonnull)toJson;

- (instancetype _Nonnull )initWithJson:(NSDictionary * _Nonnull)json;

- (instancetype _Nonnull )initWithFreshnessTime:(NSUInteger)freshnessTime;

@end
