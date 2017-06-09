//
//  LedgerUtils.h
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 05.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

@interface LedgerUtils : XCTestCase

+ (LedgerUtils *)sharedInstance;

- (NSError *)signAndSubmitRequestWithPoolHandle:(SovrinHandle)poolHandle
                                   walletHandle:(SovrinHandle)walletHandle
                                   submitterDid:(NSString *)submitterDid
                                    requestJson:(NSString *)requestJson
                                outResponseJson:(NSString **)responseJson;

- (NSError *) buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                       verkey:(NSString *)verkey
                                         data:(NSString *)data
                                         role:(NSString *)role
                                   outRequest:(NSString **)resultJson;

- (NSError *) buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                       targetDid:(NSString *)targetDid
                                      outRequest:(NSString **)requestJson;

- (NSError *)buildAttribRequest:(NSString *)submitterDid
                      targetDid:(NSString *)targetDid
                           hash:(NSString *)hash
                            raw:(NSString *)raw
                            enc:(NSString *)enc
                    resultJson:(NSString **)resultJson;

- (NSError *)buildGetAttribRequest:(NSString *)submitterDid
                         targetDid:(NSString *)targetDid
                              data:(NSString *)data
                       resultJson:(NSString **)resultJson;

- (NSError *)buildSchemaRequest:(NSString *)submitterDid
                           data:(NSString *)data
                     resultJson:(NSString **)resultJson;

- (NSError *)buildGetSchemaRequest:(NSString *)submitterDid
                              dest:(NSString *)dest
                              data:(NSString *)data
                        resultJson:(NSString **)resultJson;

- (NSError *)buildNodeRequest:(NSString *) submitterDid
                    targetDid:(NSString *) targetDid
                         data:(NSString *) data
                   resultJson:(NSString **) resultJson;

- (NSError *)buildClaimDefTxn:(NSString *) submitterDid
                         xref:(NSString *) xref
                signatureType:(NSString *) signatureType
                         data:(NSString *) data
                   resultJson:(NSString**) resultJson;

- (NSError *)buildGetClaimDefTxn:(NSString *) submitterDid
                            xref:(NSString *) xref
                   signatureType:(NSString *) signatureType
                          origin:(NSString *) origin
                      resultJson:(NSString**) resultJson;


@end
