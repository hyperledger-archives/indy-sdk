//
//  SignusUtils.h
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface SignusUtils : XCTestCase

+ (SignusUtils *)sharedInstance;

- (NSError *)signWithWalletHandle:(SovrinHandle)walletHandle
                         theirDid:(NSString *)theirDid
                          message:(NSString *)message
                     outSignature:(NSString **)signature;

- (NSError *)createMyDidWithWalletHandle:(SovrinHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey
                                 outMyPk:(NSString **)myPk;

- (NSError *)createAndStoreMyDidWithWalletHandle:(SovrinHandle)walletHandle
                                            seed:(NSString *)seed
                                        outMyDid:(NSString **)myDid
                                     outMyVerkey:(NSString **)myVerkey
                                         outMyPk:(NSString **)myPk;

- (NSError *)storeTheirDidWithWalletHandle:(SovrinHandle)walletHandle
                              identityJson:(NSString *)identityJson;

- (NSError *)replaceKeysWithWalletHandle:(SovrinHandle)walletHandle
                                     did:(NSString *)did
                            identityJson:(NSString *)identityJson
                             outMyVerKey:(NSString **)myVerKey
                                 outMyPk:(NSString **)myPk;

- (NSError *)verifyWithWalletHandle:(SovrinHandle)walletHandle
                         poolHandle:(SovrinHandle)poolHandle
                                did:(NSString *)did
                          signature:(NSString *)signature
                        outVerified:(BOOL *)verified;
@end
