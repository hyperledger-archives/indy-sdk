//
//  IndyWallet.h
//  libindy
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"
#import "IndyWalletProtocols.h"
#import <stdio.h>




@interface IndyWallet : NSObject

+ (IndyWallet *)sharedInstance;

/**
 Register Custom Wallet type with provided implementation
 
 - parameter type:
 */
- (NSError *)registerWalletType:(NSString *)type
             withImplementation:(Class<IndyWalletProtocol>)implementation
                     completion:(void (^)(NSError *error)) handler;

/**
 Register Keychain Wallet type with default implementation
*/
- (NSError *)registerIndyKeychainWalletType:(NSString *)type
                     completion:(void (^)(NSError *error)) handler;

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                                 name:(NSString *)name
                                xType:(NSString *)type
                               config:(NSString *)config
                          credentials:(NSString *)credentials
                           completion:(void (^)(NSError *error)) handler;

- (NSError *)openWalletWithName:(NSString *)name
                  runtimeConfig:(NSString *)config
                    credentials:(NSString *)credentials
                     completion:(void (^)(NSError *error, IndyHandle walletHandle )) handler;

- (NSError *)closeWalletWithHandle:(IndyHandle)walletHandle
                        completion:(void (^)(NSError *error ))handler;

- (NSError *)deleteWalletWithName:(NSString *)walletName
                      credentials:(NSString *)credentials
                       completion:(void (^)(NSError *error ))handler;

- (void)cleanupIndyKeychainWallet;


@end


