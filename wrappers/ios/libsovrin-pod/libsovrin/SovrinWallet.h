//
//  SovrinWallet.h
//  libsovrin
//

#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@protocol SovrinWalletImplementation <NSObject>

@required
- (NSError*) createWithName:(NSString*) name
                  andConfig:(NSString*) config;

@required
- (NSError*) openWithName:(NSString*)     name
               withConfig:(NSString*)     config
                andHandle:(SovrinHandle*) handle;
@required
- (NSError*) setValue:(NSString*)    value  // can value be of any type???
               forKey:(NSString*)    key
            andSubKey:(NSString*)    subkey
           withHandle:(SovrinHandle) handle;

@required
- (NSError*) getValue:(NSString**)   value // can value be of any type???
               forKey:(NSString*)    key
            andSubKey:(NSString*)    subkey
           withHandle:(SovrinHandle) handle;

@required
- (NSError*) close:(SovrinHandle) handle;

@required
- (NSError*) deleteWithName:(NSString*) name;

@end

@interface SovrinWallet : NSObject

- (NSError*) registerWalletType:(NSString*) type
             withImplementation:(id<SovrinWalletImplementation>) implementation;

- (NSError*) createWallet:(NSString*) poolName
                     name:(NSString*) name
                   config:(NSString*) config
                    xType:(NSString*) type
               completion:(NSError* (^)(NSError* error)) handler;

- (NSError*)   openWallet:(SovrinHandle) poolHandle
                     name:(NSString*) name
            runtimeConfig:(NSString*) config
              credentials:(NSString*) credentials
               completion:(NSError* (^)(NSError* error, SovrinHandle walletHandle )) handler;

- (NSError*)   closeWallet:(SovrinHandle) walletHandle
                completion:(NSError* (^)(NSError* error )) handler;

- (NSError*)   deleteWallet:(NSString*) walletName
                 completion:(NSError* (^)(NSError* error )) handler;

- (NSError*) walletSetSeqNo:(NSNumber*) seqNo
                  forHandle:(SovrinHandle) walletHandle
                     andKey:(NSString*) key
                 completion:(NSError* (^)(NSError* error )) handler;

@end
