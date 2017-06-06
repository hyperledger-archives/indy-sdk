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

- (NSError *)signAndSubmitRequest:(SovrinHandle)poolHandle
                     walletHandle:(SovrinHandle)walletHandle
                     submitterDid:(NSString *)submitterDid
                      requestJson:(NSString *)requestJson
                     responseJson:(NSString **)responseJson;

- (NSError *) buildNymRequest:(NSString *)submitterDid
                    targetDid:(NSString *)targetDid
                       verkey:(NSString *)verkey
                         xref:(NSString *)xref
                         data:(NSString *)data
                         role:(NSString *)role
                   resultJson:(NSString **)resultJson;

- (NSError *) buildGetNymRequest:(NSString *)submitterDid
                    targetDid:(NSString *)targetDid
                   resultJson:(NSString **)resultJson;

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
