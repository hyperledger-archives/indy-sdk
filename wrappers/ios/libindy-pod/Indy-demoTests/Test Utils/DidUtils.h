//
//  DidUtils.h
//  Indy-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface DidUtils : XCTestCase

+ (DidUtils *)sharedInstance;

// MARK: - Instance methods

- (NSError *)createMyDidWithWalletHandle:(IndyHandle)walletHandle
                               myDidJson:(NSString *)myDidJson
                                outMyDid:(NSString **)myDid
                             outMyVerkey:(NSString **)myVerkey;

- (NSError *)createAndStoreMyDidWithWalletHandle:(IndyHandle)walletHandle
                                            seed:(NSString *)seed
                                        outMyDid:(NSString **)myDid
                                     outMyVerkey:(NSString **)myVerkey;

- (NSError *)createAndStoreAndPublishDidWithWalletHandle:(IndyHandle)walletHandle
                                              poolHandle:(IndyHandle)poolHandle
                                                     did:(NSString **)did
                                                  verkey:(NSString **)verkey;

- (NSError *)storeTheirDidWithWalletHandle:(IndyHandle)walletHandle
                              identityJson:(NSString *)identityJson;

- (NSError *)storeTheirDidFromPartsWithWalletHandle:(IndyHandle)walletHandle
                                           theirDid:(NSString *)theirDid
                                        theirVerkey:(NSString *)theirVerkey;

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

- (NSString *)createStoreAndPublishMyDidWithWalletHandle:(IndyHandle)walletHandle
                                              poolHandle:(IndyHandle)poolHandle;

- (NSError *)keyForDid:(NSString *)did
            poolHandle:(IndyHandle)poolHandle
          walletHandle:(IndyHandle)walletHandle
                   key:(NSString **)key;

- (NSError *)keyForLocalDid:(NSString *)did
               walletHandle:(IndyHandle)walletHandle
                        key:(NSString **)key;

- (NSError *)setEndpointAddress:(NSString *)address
                   transportKey:(NSString *)transportKey
                         forDid:(NSString *)did
                   walletHandle:(IndyHandle)walletHandle;

- (NSError *)getEndpointForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                    poolHandle:(IndyHandle)poolHandle
                       address:(NSString **)address
                  transportKey:(NSString **)transportKey;

- (NSError *)setMetadata:(NSString *)metadata
                  forDid:(NSString *)did
            walletHandle:(IndyHandle)walletHandle;

- (NSError *)getMetadataForDid:(NSString *)did
                  walletHandle:(IndyHandle)walletHandle
                      metadata:(NSString **)metadata;

- (NSError *)abbreviateVerkey:(NSString *)did
                   fullVerkey:(NSString *)fullVerkey
                       verkey:(NSString **)verkey;

- (NSError *)listMyDidsWithMeta:(IndyHandle)walletHandle
                       metadata:(NSString **)metadata;
@end
