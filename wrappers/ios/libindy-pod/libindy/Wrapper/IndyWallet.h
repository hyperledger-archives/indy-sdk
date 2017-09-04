//
//  IndyWallet.h
//  libindy
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"
#import "IndyWalletImplementation.h"

@interface IndyWallet : NSObject


- (NSError *)registerWalletType:(NSString *)type
             withImplementation:(id<IndyWalletImplementation>)implementation
                     completion:(void (^)(NSError *error)) handler;


+ (IndyWallet *)sharedInstance;

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

@end
