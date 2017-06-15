//
//  SovrinWallet.h
//  libsovrin
//

#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@protocol SovrinWalletImplementation <NSObject>

@required
- (NSError *)createWithName:(NSString *)name
                  andConfig:(NSString *)config;

@required
- (NSError *)openWithName:(NSString *)name
               withConfig:(NSString *)config
                andHandle:(SovrinHandle *)handle;
@required
- (NSError *)setValue:(NSString *)value  // can value be of any type???
               forKey:(NSString *)key
            andSubKey:(NSString *)subkey
           withHandle:(SovrinHandle)handle;

@required
- (NSError *)getValue:(NSString **)value // can value be of any type???
               forKey:(NSString *)key
            andSubKey:(NSString *)subkey
           withHandle:(SovrinHandle)handle;

@required
- (NSError *)close:(SovrinHandle)handle;

@required
- (NSError *)deleteWithName:(NSString *)name;

@end

@interface SovrinWallet : NSObject

/*
- (NSError*) registerWalletType:(NSString*) type
             withImplementation:(id<SovrinWalletImplementation>) implementation;
*/

+ (SovrinWallet *)sharedInstance;

- (NSError *)createWalletWithPoolName:(NSString *)poolName
                                 name:(NSString *)name
                                xType:(NSString *)type
                               config:(NSString *)config
                          credentials:(NSString *)credentials
                           completion:(void (^)(NSError *error)) handler;

- (NSError *)openWalletWithName:(NSString *)name
                  runtimeConfig:(NSString *)config
                    credentials:(NSString *)credentials
                     completion:(void (^)(NSError *error, SovrinHandle walletHandle )) handler;

- (NSError *)closeWalletWithHandle:(SovrinHandle)walletHandle
                        completion:(void (^)(NSError *error ))handler;

- (NSError *)deleteWalletWithName:(NSString *)walletName
                      credentials:(NSString *)credentials
                       completion:(void (^)(NSError *error ))handler;

- (NSError *)walletSetSeqNo:(NSNumber *)seqNo
                  forHandle:(SovrinHandle)walletHandle
                     andKey:(NSString *)key
                 completion:(void (^)(NSError *error ))handler;

@end
