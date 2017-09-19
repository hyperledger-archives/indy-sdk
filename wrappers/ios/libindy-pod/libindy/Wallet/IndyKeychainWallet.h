//
//  IndyKeychainWallet.h
//  libindy-demo
//

#import "IndyWallet.h"

@interface IndyKeychainWallet : NSObject <IndyWalletProtocol>

- (instancetype)initWithName:(NSString *)name
               runtimeConfig:(NSString *)runtimeConfig
                 credentials:(NSString *)credentials;

@end
