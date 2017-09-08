//
//  KeychainWallet.h
//  libindy-demo
//

#import "IndyWallet.h"

@interface KeychainWallet : NSObject <IndyWalletProtocol>

- (NSString *)walletTypeName;

- (NSString *)poolName;

- (instancetype)initWithName:(NSString *)name
               runtimeConfig:(NSString *)runtimeConfig
                 credentials:(NSString *)credentials;

@end
