//
//  SignusUtils.h
//  libindy-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libindy/libindy.h>

@interface SignusUtils : XCTestCase

+ (SignusUtils *)sharedInstance;

- (NSError *)signWithWalletHandle:(IndyHandle)walletHandle
                         theirDid:(NSString *)theirDid
                          message:(NSString *)message
                     outSignature:(NSString **)signature;

- (NSError *)createMyDidWithWalletHandle:(IndyHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey
                                 outMyPk:(NSString **)myPk;

- (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                            seed:(NSString *)seed
                                        outMyDid:(NSString **)myDid
                                     outMyVerkey:(NSString **)myVerkey
                                         outMyPk:(NSString **)myPk;

- (NSError *)storeTheirDidWithWalletHandle:(IndyHandle)walletHandle
                              identityJson:(NSString *)identityJson;

- (NSError *)storeTheirDidFromPartsWithWalletHandle:(IndyHandle)walletHandle
                                           theirDid:(NSString *)theirDid
                                            theirPk:(NSString *)theirPk
                                        theirVerkey:(NSString *)theirVerkey
                                           endpoint:(NSString *)endpoint;

- (NSError *)replaceKeysWithWalletHandle:(IndyHandle)walletHandle
                                     did:(NSString *)did
                            identityJson:(NSString *)identityJson
                             outMyVerKey:(NSString **)myVerKey
                                 outMyPk:(NSString **)myPk;

- (NSError *)verifyWithWalletHandle:(IndyHandle)walletHandle
                         poolHandle:(IndyHandle)poolHandle
                                did:(NSString *)did
                          signature:(NSString *)signature
                        outVerified:(BOOL *)verified;
@end
