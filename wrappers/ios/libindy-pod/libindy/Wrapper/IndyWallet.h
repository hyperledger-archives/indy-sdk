//
//  IndyWallet.h
//  libindy
//

#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@protocol IndyWalletImplementation <NSObject>

@required
- (NSError *)createWithName:(NSString *)name
                     config:(NSString *)config
                credentials:(NSString *)credentials;

@required
- (NSError *)openWithName:(NSString *)name
                   config:(NSString *)config
            runtimeConfig:(NSString *)runtimeConfig
              credentials:(NSString *)credentials
                   handle:(IndyHandle *)handle;
@required
- (NSError *)setValue:(id)value  // can value be of any type???
               forKey:(NSString *)key
            andSubKey:(NSString *)subkey
           withHandle:(IndyHandle)handle;

@required
- (NSError *)getValue:(NSString **)value // can value be of any type???
               forKey:(NSString *)key
            andSubKey:(NSString *)subkey
           withHandle:(IndyHandle)handle;

@required
- (NSError *)close:(IndyHandle)handle;

@required
- (NSError *)deleteWithName:(NSString *)name;

@end

@interface IndyWallet : NSObject


- (NSError*) registerWalletType:(NSString*) type
             withImplementation:(id<IndyWalletImplementation>) implementation;


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
