//
//  KeychainWallet.h
//  libindy-demo
//

#import "IndyWallet.h"

@interface KeychainWallet : NSObject <IndyWalletProtocol>

//+ (KeychainWallet *)sharedInstance;

- (instancetype)initWithName:(NSString *)name
               runtimeConfig:(NSString *)runtimeConfig
                 credentials:(NSString *)credentials;


@end
