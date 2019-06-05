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

- (NSError *)submitAction:(NSString *)request
                    nodes:(NSString *)nodes
                  timeout:(NSNumber *)timeout
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
                                                id:(NSString *)id
                                        resultJson:(NSString **)resultJson;

- (NSError *)parseGetSchemaResponse:(NSString *)getSchemaResponse
                           schemaId:(NSString **)schemaId
                         schemaJson:(NSString **)schemaJson;

// MARK: - Node request
- (NSError *)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                         data:(NSString *)data
                                   resultJson:(NSString **)resultJson;

// MARK: - Get validator info request
- (NSError *)buildGetValidatorInfo:(NSString *)submitterDid
                        resultJson:(NSString **)resultJson;

// MARK: - CredDef Request
- (NSError *)buildCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                            data:(NSString *)data
                                      resultJson:(NSString **)resultJson;

- (NSError *)buildGetCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                 id:(NSString *)id
                                         resultJson:(NSString **)resultJson;

- (NSError *)parseGetCredDefResponse:(NSString *)getCredDefResponse
                           credDefId:(NSString **)credDefId
                         credDefJson:(NSString **)credDefJson;

// MARK: - Get Txn request
- (NSError *)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                     ledgerType:(NSString *)ledgerType
                                           data:(NSNumber *)data
                                     resultJson:(NSString **)resultJson;

// MARK: - Pool Config request
- (NSError *)buildPoolConfigRequestWithSubmitterDid:(NSString *)submitterDid
                                             writes:(BOOL)writes
                                              force:(BOOL)force
                                         resultJson:(NSString **)resultJson;

// MARK: - Pool Restart request
- (NSError *)buildPoolRestartRequestWithSubmitterDid:(NSString *)submitterDid
                                              action:(NSString *)action
                                            datetime:(NSString *)datetime
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
                                            package_:(NSString *)package_
                                          resultJson:(NSString **)resultJson;

// MARK: - Revocation registry definition request
- (NSError *)buildRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                data:(NSString *)data
                                          resultJson:(NSString **)resultJson;

- (NSError *)buildGetRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                     id:(NSString *)id
                                             resultJson:(NSString **)resultJson;

- (NSError *)parseGetRevocRegDefResponse:(NSString *)getRevocRegDefResponse
                           revocRegDefId:(NSString **)revocRegDefId
                         revocRegDefJson:(NSString **)revocRegDefJson;


// MARK: - Revocation registry entry request
- (NSError *)buildRevocRegEntryRequestWithSubmitterDid:(NSString *)submitterDid
                                                  type:(NSString *)type
                                         revocRegDefId:(NSString *)revocRegDefId
                                                 value:(NSString *)value
                                            resultJson:(NSString **)resultJson;

- (NSError *)buildGetRevocRegRequestWithSubmitterDid:(NSString *)submitterDid
                                       revocRegDefId:(NSString *)revocRegDefId
                                           timestamp:(NSNumber *)timestamp
                                          resultJson:(NSString **)resultJson;

- (NSError *)parseGetRevocRegResponse:(NSString *)getRevocRegResponse
                        revocRegDefId:(NSString **)revocRegDefId
                         revocRegJson:(NSString **)revocRegJson
                            timestamp:(NSNumber **)timestamp;

- (NSError *)buildGetRevocRegDeltaRequestWithSubmitterDid:(NSString *)submitterDid
                                            revocRegDefId:(NSString *)revocRegDefId
                                                     from:(NSNumber *)from
                                                       to:(NSNumber *)to
                                               resultJson:(NSString **)resultJson;

- (NSError *)parseGetRevocRegDeltaResponse:(NSString *)getRevocRegDeltaResponse
                             revocRegDefId:(NSString **)revocRegDefId
                         revocRegDeltaJson:(NSString **)revocRegDeltaJson
                                 timestamp:(NSNumber **)timestamp;

// MARK: - Sign Request
- (NSError *)signRequestWithWalletHandle:(IndyHandle)walletHandle
                            submitterdid:(NSString *)submitterDid
                             requestJson:(NSString *)requestJson
                              resultJson:(NSString **)resultJson;

- (NSError *)multiSignRequestWithWalletHandle:(IndyHandle)walletHandle
                                 submitterdid:(NSString *)submitterDid
                                  requestJson:(NSString *)requestJson
                                   resultJson:(NSString **)resultJson;

// MARK: - Auth Rule request
- (NSError *)buildAuthRuleRequestWithSubmitterDid:(NSString *)submitterDid
                                          txnType:(NSString *)txnType
                                           action:(NSString *)action
                                            field:(NSString *)field
                                         oldValue:(NSString *)oldValue
                                         newValue:(NSString *)newValue
                                       constraint:(NSString *)constraint
                                       outRequest:(NSString **)resultJson;

- (NSError *)buildAuthRulesRequestWithSubmitterDid:(NSString *)submitterDid
                                              data:(NSString *)data
                                        outRequest:(NSString **)resultJson;

- (NSError *)buildGetAuthRuleRequestWithSubmitterDid:(NSString *)submitterDid
                                             txnType:(NSString *)txnType
                                              action:(NSString *)action
                                               field:(NSString *)field
                                            oldValue:(NSString *)oldValue
                                            newValue:(NSString *)newValue
                                          outRequest:(NSString **)resultJson;

// MARK: - Author agreement
- (NSError *)buildTxnAuthorAgreementRequestWithSubmitterDid:(NSString *)submitterDid
                                                       text:(NSString *)text
                                                    version:(NSString *)version
                                                 outRequest:(NSString **)resultJson;

- (NSError *)buildGetTxnAuthorAgreementRequestWithSubmitterDid:(NSString *)submitterDid
                                                          data:(NSString *)data
                                                    outRequest:(NSString **)resultJson;

// MARK: - Acceptance mechanism
- (NSError *)buildAcceptanceMechanismsRequestWithSubmitterDid:(NSString *)submitterDid
                                                          aml:(NSString *)aml
                                                      version:(NSString *)version
                                                   amlContext:(NSString *)amlContext
                                                   outRequest:(NSString **)resultJson;

- (NSError *)buildGetAcceptanceMechanismsRequestWithSubmitterDid:(NSString *)submitterDid
                                                       timestamp:(NSNumber *)timestamp
                                                         version:(NSString *)version
                                                      outRequest:(NSString **)resultJson;

// MARK: - Author Metadata
- (NSError *)appendTxnAuthorAgreementAcceptanceToRequest:(NSString *)requestJson
                                                    text:(NSString *)text
                                                 version:(NSString *)version
                                               taaDigest:(NSString *)taaDigest
                                             accMechType:(NSString *)accMechType
                                        timeOfAcceptance:(NSNumber *)timeOfAcceptance
                                              outRequest:(NSString **)resultJson;

// MARK: - Response Metadata
- (NSError *)getResponseMetadata:(NSString *)response
                responseMetadata:(NSString **)responseMetadata;


@end
