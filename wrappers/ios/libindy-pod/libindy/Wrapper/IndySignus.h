//
//  IndySignus.h
//  libindy
//


#import <Foundation/Foundation.h>
#import "IndyTypes.h"

@interface IndySignus : NSObject

+ (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                         didJSON:(NSString *)didJson
                                      completion:(void (^)(NSError *error, NSString *did, NSString *verkey, NSString *pk)) handler;

+ (NSError *)replaceKeysWithWalletHandle:(IndyHandle)walletHandle
                                     did:(NSString *)did
                            identityJSON:(NSString *)json
                              completion:(void (^)(NSError *error, NSString *verkey, NSString *pk)) handler;

+ (NSError *)storeTheirDidWithWalletHandle:(IndyHandle)walletHandle
                              identityJSON:(NSString *)json
                                completion:(void (^)(NSError *error)) handler;

+ (NSError *)signWithWalletHandle:(IndyHandle)walletHandle
                              did:(NSString *)did
                              msg:(NSString *)msg
                       completion:(void (^)(NSError *error, NSString *signature)) handler;

+ (NSError *)verifySignatureWithWalletHandle:(IndyHandle)walletHandle
                                  poolHandle:(IndyHandle)poolHandle
                                         did:(NSString *)did
                                   signature:(NSString *)signature
                                  completion:(void (^)(NSError *error, BOOL valid)) handler;

+ (NSError *)encryptWithWalletHandle:(IndyHandle)walletHandle
                                pool:(IndyHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                                 msg:(NSString *)msg
          completion:(void (^)(NSError *error, NSString *encryptedMsg, NSString *nonce)) handler;

+ (NSError *)decryptWithWalletHandle:(IndyHandle)walletHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                        encryptedMsg:(NSString *)msg
                               nonce:(NSString *)nonce
                          completion:(void (^)(NSError *error, NSString *decryptedMsg)) handler;

@end
