//
//  LedgerUtils.h
//  Indy-demo
//
//  Created by Anastasia Tarasova on 05.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>

@interface LedgerUtils : XCTestCase

+ (LedgerUtils *)sharedInstance;

- (NSError *)signAndSubmitRequestWithPoolHandle:(IndyHandle)poolHandle
                                   walletHandle:(IndyHandle)walletHandle
                                   submitterDid:(NSString *)submitterDid
                                    requestJson:(NSString *)requestJson
                                outResponseJson:(NSString **)responseJson;

- (NSError *)submitRequest:(NSString *)request
            withPoolHandle:(IndyHandle)poolHandle
                resultJson:(NSString **)resultJson;


// MARK: - Nym request
- (NSError *)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                                   targetDid:(NSString *)targetDid
                                      verkey:(NSString *)verkey
                                       alias:(NSString *)alias
                                        role:(NSString *)role
                                  outRequest:(NSString **)resultJson;

- (NSError *)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDid:(NSString *)targetDid
                                     outRequest:(NSString **)requestJson;

// MARK: - Attrib request
- (NSError *)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDid:(NSString *)targetDid
                                           hash:(NSString *)hash
                                            raw:(NSString *)raw
                                            enc:(NSString *)enc
                                     resultJson:(NSString **)resultJson;

- (NSError *)buildGetAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                         targetDid:(NSString *)targetDid
                                               raw:(NSString *)raw
                                              hash:(NSString *)hash
                                               enc:(NSString *)enc
                                        resultJson:(NSString **)resultJson;

// MARK: - Schema request
- (NSError *)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     resultJson:(NSString **)resultJson;

- (NSError *)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                              dest:(NSString *)dest
                                              data:(NSString *)data
                                        resultJson:(NSString **)resultJson;

// MARK: - Node request
- (NSError *)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                         data:(NSString *)data
                                   resultJson:(NSString **)resultJson;

// MARK: - ClaimDefTxn
- (NSError *)buildClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                         xref:(NSNumber *)xref
                                signatureType:(NSString *)signatureType
                                         data:(NSString *)data
                                   resultJson:(NSString **)resultJson;

- (NSError *)buildGetClaimDefTxnWithSubmitterDid:(NSString *)submitterDid
                                            xref:(NSNumber *)xref
                                   signatureType:(NSString *)signatureType
                                          origin:(NSString *)origin
                                      resultJson:(NSString **)resultJson;

// MARK: - Get Txn request
- (NSError *)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSNumber *)data
                                     resultJson:(NSString **)resultJson;

// MARK: - Pool Config request
- (NSError *)buildPoolConfigRequestWithSubmitterDid:(NSString *)submitterDid
                                             writes:(BOOL)writes
                                              force:(BOOL)force
                                         resultJson:(NSString **)resultJson;

// MARK: - Pool Upgrade request
- (NSError *)buildPoolUpgradeRequestWithSubmitterDid:(NSString *)submitterDid
                                                name:(NSString *)name
                                             version:(NSString *)version
                                              action:(NSString *)action
                                              sha256:(NSString *)sha256
                                             timeout:(NSNumber *)timeout
                                            schedule:(NSString *)schedule
                                       justification:(NSString *)justification
                                           reinstall:(BOOL)reinstall
                                               force:(BOOL)force
                                          resultJson:(NSString **)resultJson;

// MARK: - Sign Request
- (NSError *)signRequestWithWalletHandle:(IndyHandle)walletHandle
                            submitterdid:(NSString *)submitterDid
                             requestJson:(NSString *)requestJson
                              resultJson:(NSString **)resultJson;


@end
