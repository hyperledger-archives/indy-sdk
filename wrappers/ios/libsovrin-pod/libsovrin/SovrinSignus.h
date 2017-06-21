//
//  SovrinSignus.h
//  libsovrin
//


#import <Foundation/Foundation.h>
#import "SovrinTypes.h"

@interface SovrinSignus : NSObject

+ (NSError *)createAndStoreMyDidWithWalletHandle:(SovrinHandle)walletHandle
                                         didJSON:(NSString *)didJson
                                      completion:(void (^)(NSError *error, NSString *did, NSString *verkey, NSString *pk)) handler;

+ (NSError *)replaceKeysWithWalletHandle:(SovrinHandle)walletHandle
                                     did:(NSString *)did
                            identityJSON:(NSString *)json
                              completion:(void (^)(NSError *error, NSString *verkey, NSString *pk)) handler;

+ (NSError *)storeTheirDidWithWalletHandle:(SovrinHandle)walletHandle
                              identityJSON:(NSString *)json
                                completion:(void (^)(NSError *error)) handler;

+ (NSError *)signWithWalletHandle:(SovrinHandle)walletHandle
                              did:(NSString *)did
                              msg:(NSString *)msg
                       completion:(void (^)(NSError *error, NSString *signature)) handler;

+ (NSError *)verifySignatureWithWalletHandle:(SovrinHandle)walletHandle
                                  poolHandle:(SovrinHandle)poolHandle
                                         did:(NSString *)did
                                   signature:(NSString *)signature
                                  completion:(void (^)(NSError *error, BOOL valid)) handler;

+ (NSError *)encryptWithWalletHandle:(SovrinHandle)walletHandle
                                pool:(SovrinHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                                 msg:(NSString *)msg
          completion:(void (^)(NSError *error, NSString *encryptedMsg, NSString *nonce)) handler;

+ (NSError *)decryptWithWalletHandle:(SovrinHandle)walletHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                        encryptedMsg:(NSString *)msg
                               nonce:(NSString *)nonce
                          completion:(void (^)(NSError *error, NSString *decryptedMsg)) handler;


@end
