//
//  KeychainWallet.h
//  libindy-demo
//

#import "IndyWallet.h"

@interface KeychainWallet : NSObject <IndyWalletImplementation>

+ (KeychainWallet*) sharedInstance;

- (NSString *)walletTypeName;

@end
