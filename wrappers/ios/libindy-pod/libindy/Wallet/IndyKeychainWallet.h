//
//  KeychainWallet.h
//  libindy-demo
//

#import "IndyWallet.h"

@interface IndyKeychainWallet : NSObject <IndyWalletProtocol>

- (NSString *)walletTypeName;

@end
