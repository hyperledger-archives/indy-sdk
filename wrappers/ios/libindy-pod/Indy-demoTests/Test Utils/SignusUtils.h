//
//  SignusUtils.h
//  Indy-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface SignusUtils : XCTestCase

+ (SignusUtils *)sharedInstance;

// MARK: - Instance methods

- (NSError *)signWithWalletHandle:(IndyHandle)walletHandle
                         theirDid:(NSString *)theirDid
                          message:(NSData *)message
                     outSignature:(NSData **)signature;

- (NSError *)createMyDidWithWalletHandle:(IndyHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey;

- (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                            seed:(NSString *)seed
                                        outMyDid:(NSString **)myDid
                                     outMyVerkey:(NSString **)myVerkey;

- (NSError *)storeTheirDidWithWalletHandle:(IndyHandle)walletHandle
                              identityJson:(NSString *)identityJson;

- (NSError *)storeTheirDidFromPartsWithWalletHandle:(IndyHandle)walletHandle
                                           theirDid:(NSString *)theirDid
                                        theirVerkey:(NSString *)theirVerkey
                                           endpoint:(NSString *)endpoint;

- (NSError *)replaceKeysStartForDid:(NSString *)did
                       identityJson:(NSString *)identityJson
                       walletHandle:(IndyHandle)walletHandle
                        outMyVerKey:(NSString **)myVerKey;

- (NSError *)replaceKeysApplyForDid:(NSString *)did
                       walletHandle:(IndyHandle)walletHandle;

- (NSError *)replaceKeysForDid:(NSString *)did
                  identityJson:(NSString *)identityJson
                  walletHandle:(IndyHandle)walletHandle
                    poolHandle:(IndyHandle)poolHandle
                   outMyVerKey:(NSString **)myVerKey;

- (NSError *)verifyWithWalletHandle:(IndyHandle)walletHandle
                         poolHandle:(IndyHandle)poolHandle
                                did:(NSString *)did
                            message:(NSData *)message
                          signature:(NSData *)signature
                        outVerified:(BOOL *)verified;

- (NSError *)encryptWithWalletHandle:(IndyHandle)walletHandle
                          poolHandle:(IndyHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                             message:(NSData *)message
                 outEncryptedMessage:(NSData **)encryptedMessage
                            outNonce:(NSData **)nonce;

- (NSError *)decryptWithWalletHandle:(IndyHandle)walletHandle
                          poolHandle:(IndyHandle)poolHandle
                               myDid:(NSString *)myDid
                                 did:(NSString *)did
                    encryptedMessage:(NSData *)encryptedMessage
                               nonce:(NSData *)nonce
                 outDecryptedMessage:(NSData **)decryptedMessage;
@end
