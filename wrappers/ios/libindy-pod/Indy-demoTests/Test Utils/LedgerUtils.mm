//
//  LedgerUtils.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 05.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import "LedgerUtils.h"
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"

@implementation LedgerUtils

+ (LedgerUtils *)sharedInstance {
    static LedgerUtils *instance = nil;
    static dispatch_once_t dispatch_once_block;

    dispatch_once(&dispatch_once_block, ^{
        instance = [LedgerUtils new];
    });

    return instance;
}


- (NSError *)signAndSubmitRequestWithPoolHandle:(IndyHandle)poolHandle
                                   walletHandle:(IndyHandle)walletHandle
                                   submitterDid:(NSString *)submitterDid
                                    requestJson:(NSString *)requestJson
                                outResponseJson:(NSString **)responseJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;

    [IndyLedger signAndSubmitRequest:requestJson
                        submitterDID:submitterDid
                          poolHandle:poolHandle
                        walletHandle:walletHandle
                          completion:^(NSError *error, NSString *resultJson) {
                              err = error;
                              outJson = resultJson;
                              [completionExpectation fulfill];
                          }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (responseJson) {*responseJson = outJson;}

    return err;
}

- (NSError *)submitRequest:(NSString *)request
            withPoolHandle:(IndyHandle)poolHandle
                resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;

    [IndyLedger submitRequest:request poolHandle:poolHandle completion:^(NSError *error, NSString *resultJson) {
        err = error;
        outJson = resultJson;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils defaultTimeout]];

    if (resultJson) {*resultJson = outJson;}

    return err;
}

// MARK: Build nym request

- (NSError *)buildNymRequestWithSubmitterDid:(NSString *)submitterDid
                                   targetDid:(NSString *)targetDid
                                      verkey:(NSString *)verkey
                                       alias:(NSString *)alias
                                        role:(NSString *)role
                                  outRequest:(NSString **)resultJson; {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;

    [IndyLedger buildNymRequestWithSubmitterDid:submitterDid
                                      targetDID:targetDid
                                         verkey:verkey
                                          alias:alias
                                           role:role
                                     completion:^(NSError *error, NSString *json) {
                                         err = error;
                                         outJson = json;
                                         [completionExpectation fulfill];
                                     }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = outJson;}

    return err;
}


- (NSError *)buildGetNymRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDid:(NSString *)targetDid
                                     outRequest:(NSString **)requestJson; {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;

    [IndyLedger buildGetNymRequestWithSubmitterDid:submitterDid
                                         targetDID:targetDid
                                        completion:^(NSError *error, NSString *json) {
                                            err = error;
                                            outJson = json;
                                            [completionExpectation fulfill];
                                        }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (requestJson) {*requestJson = outJson;}

    return err;
}

// MARK: Build Attribute request

- (NSError *)buildAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                      targetDid:(NSString *)targetDid
                                           hash:(NSString *)hash
                                            raw:(NSString *)raw
                                            enc:(NSString *)enc
                                     resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outJson = nil;

    [IndyLedger buildAttribRequestWithSubmitterDid:submitterDid
                                         targetDID:targetDid
                                              hash:hash
                                               raw:raw
                                               enc:enc
                                        completion:^(NSError *error, NSString *requestJson) {
                                            err = error;
                                            outJson = requestJson;
                                            [completionExpectation fulfill];
                                        }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = outJson;}
    return err;
}


- (NSError *)buildGetAttribRequestWithSubmitterDid:(NSString *)submitterDid
                                         targetDid:(NSString *)targetDid
                                               raw:(NSString *)raw
                                              hash:(NSString *)hash
                                               enc:(NSString *)enc
                                        resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outRequest = nil;

    [IndyLedger buildGetAttribRequestWithSubmitterDid:submitterDid
                                            targetDID:targetDid
                                                  raw:raw
                                                 hash:hash
                                                  enc:enc
                                           completion:^(NSError *error, NSString *request) {
                                               err = error;
                                               outRequest = request;
                                               [completionExpectation fulfill];
                                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = outRequest;}
    return err;
}
// MARK: Build schema request

- (NSError *)buildSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSString *)data
                                     resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildSchemaRequestWithSubmitterDid:submitterDid
                                              data:data
                                        completion:^(NSError *error, NSString *request) {
                                            err = error;
                                            result = request;
                                            [completionExpectation fulfill];
                                        }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)buildGetSchemaRequestWithSubmitterDid:(NSString *)submitterDid
                                                id:(NSString *)id
                                        resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildGetSchemaRequestWithSubmitterDid:submitterDid
                                                   id:id
                                           completion:^(NSError *error, NSString *request) {
                                               err = error;
                                               result = request;
                                               [completionExpectation fulfill];
                                           }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)parseGetSchemaResponse:(NSString *)getSchemaResponse
                           schemaId:(NSString **)schemaId
                         schemaJson:(NSString **)schemaJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outSchemaId = nil;
    __block NSString *outSchemaJson = nil;

    [IndyLedger parseGetSchemaResponse:getSchemaResponse
                            completion:^(NSError *error, NSString *id, NSString *json) {
                                err = error;
                                outSchemaId = id;
                                outSchemaJson = json;
                                [completionExpectation fulfill];
                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (schemaId) {*schemaId = outSchemaId;}
    if (schemaJson) {*schemaJson = outSchemaJson;}
    return err;
}

// MARK: Build Node request

- (NSError *)buildNodeRequestWithSubmitterDid:(NSString *)submitterDid
                                    targetDid:(NSString *)targetDid
                                         data:(NSString *)data
                                   resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildNodeRequestWithSubmitterDid:submitterDid
                                       targetDid:targetDid
                                            data:data
                                      completion:^(NSError *error, NSString *request) {
                                          err = error;
                                          result = request;
                                          [completionExpectation fulfill];
                                      }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

// MARK: Build cred definition request

- (NSError *)buildCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                            data:(NSString *)data
                                      resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildCredDefRequestWithSubmitterDid:submitterDid
                                               data:data
                                         completion:^(NSError *error, NSString *request) {
                                             err = error;
                                             result = request;
                                             [completionExpectation fulfill];
                                         }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)buildGetCredDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                 id:(NSString *)id
                                         resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildGetCredDefRequestWithSubmitterDid:submitterDid
                                                    id:id
                                            completion:^(NSError *error, NSString *request) {
                                                err = error;
                                                result = request;
                                                [completionExpectation fulfill];
                                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)parseGetCredDefResponse:(NSString *)getCredDefResponse
                           credDefId:(NSString **)credDefId
                         credDefJson:(NSString **)credDefJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outId = nil;
    __block NSString *outJson = nil;

    [IndyLedger parseGetCredDefResponse:(NSString *) getCredDefResponse
                             completion:^(NSError *error, NSString *id, NSString *json) {
                                 err = error;
                                 outId = id;
                                 outJson = json;
                                 [completionExpectation fulfill];
                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (credDefId) {*credDefId = outId;}
    if (credDefJson) {*credDefJson = outJson;}
    return err;
}

// MARK: Buildget txn request

- (NSError *)buildGetTxnRequestWithSubmitterDid:(NSString *)submitterDid
                                           data:(NSNumber *)data
                                     resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildGetTxnRequestWithSubmitterDid:submitterDid
                                              data:data
                                        completion:^(NSError *error, NSString *request) {
                                            err = error;
                                            result = request;
                                            [completionExpectation fulfill];
                                        }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)buildPoolConfigRequestWithSubmitterDid:(NSString *)submitterDid
                                             writes:(BOOL)writes
                                              force:(BOOL)force
                                         resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildPoolConfigRequestWithSubmitterDid:submitterDid
                                                writes:writes
                                                 force:force
                                            completion:^(NSError *error, NSString *request) {
                                                err = error;
                                                result = request;
                                                [completionExpectation fulfill];
                                            }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)buildPoolRestartRequestWithSubmitterDid:(NSString *)submitterDid
                                              action:(NSString *)action
                                            datetime:(NSString *)datetime
                                          resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;
    
    [IndyLedger buildPoolRestartRequestWithSubmitterDid:submitterDid
                                                 action:action
                                               datetime:datetime
                                             completion:^(NSError *error, NSString *request) {
                                                 err = error;
                                                 result = request;
                                                 [completionExpectation fulfill];
                                             }];
    
    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];
    
    if (resultJson) {*resultJson = result;}
    return err;
}


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
                                          resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildPoolUpgradeRequestWithSubmitterDid:submitterDid
                                                   name:name
                                                version:version
                                                 action:action
                                                 sha256:sha256
                                                timeout:timeout
                                               schedule:schedule
                                          justification:justification
                                              reinstall:reinstall
                                                  force:force
                                             completion:^(NSError *error, NSString *request) {
                                                 err = error;
                                                 result = request;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)buildRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                data:(NSString *)data
                                          resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildRevocRegDefRequestWithSubmitterDid:submitterDid
                                                   data:(NSString *) data
                                             completion:^(NSError *error, NSString *request) {
                                                 err = error;
                                                 result = request;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)buildGetRevocRegDefRequestWithSubmitterDid:(NSString *)submitterDid
                                                     id:(NSString *)id
                                             resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildGetRevocRegDefRequestWithSubmitterDid:submitterDid
                                                        id:id
                                                completion:^(NSError *error, NSString *request) {
                                                    err = error;
                                                    result = request;
                                                    [completionExpectation fulfill];
                                                }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)parseGetRevocRegDefResponse:(NSString *)getRevocRegDefResponse
                           revocRegDefId:(NSString **)revocRegDefId
                         revocRegDefJson:(NSString **)revocRegDefJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outId = nil;
    __block NSString *outJson = nil;

    [IndyLedger parseGetRevocRegDefResponse:(NSString *) getRevocRegDefResponse
                                 completion:^(NSError *error, NSString *id, NSString *json) {
                                     err = error;
                                     outId = id;
                                     outJson = json;
                                     [completionExpectation fulfill];
                                 }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revocRegDefId) {*revocRegDefId = outId;}
    if (revocRegDefJson) {*revocRegDefJson = outJson;}
    return err;
}

- (NSError *)buildRevocRegEntryRequestWithSubmitterDid:(NSString *)submitterDid
                                                  type:(NSString *)type
                                         revocRegDefId:(NSString *)revocRegDefId
                                                 value:(NSString *)value
                                            resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildRevocRegEntryRequestWithSubmitterDid:submitterDid
                                                     type:(NSString *) type
                                            revocRegDefId:(NSString *) revocRegDefId
                                                    value:(NSString *) value
                                               completion:^(NSError *error, NSString *request) {
                                                   err = error;
                                                   result = request;
                                                   [completionExpectation fulfill];
                                               }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)buildGetRevocRegRequestWithSubmitterDid:(NSString *)submitterDid
                                       revocRegDefId:(NSString *)revocRegDefId
                                           timestamp:(NSNumber *)timestamp
                                          resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildGetRevocRegRequestWithSubmitterDid:submitterDid
                                          revocRegDefId:revocRegDefId
                                              timestamp:timestamp
                                             completion:^(NSError *error, NSString *request) {
                                                 err = error;
                                                 result = request;
                                                 [completionExpectation fulfill];
                                             }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)parseGetRevocRegResponse:(NSString *)getRevocRegResponse
                        revocRegDefId:(NSString **)revocRegDefId
                         revocRegJson:(NSString **)revocRegJson
                            timestamp:(NSNumber **)timestamp {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outId = nil;
    __block NSString *outJson = nil;
    __block NSNumber *outTimestamp = nil;

    [IndyLedger parseGetRevocRegResponse:(NSString *) getRevocRegResponse
                              completion:^(NSError *error, NSString *id, NSString *json, NSNumber *oTimestamp) {
                                  err = error;
                                  outId = id;
                                  outJson = json;
                                  outTimestamp = oTimestamp;
                                  [completionExpectation fulfill];
                              }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revocRegDefId) {*revocRegDefId = outId;}
    if (revocRegJson) {*revocRegJson = outJson;}
    if (timestamp) {*timestamp = outTimestamp;}
    return err;
}

- (NSError *)buildGetRevocRegDeltaRequestWithSubmitterDid:(NSString *)submitterDid
                                            revocRegDefId:(NSString *)revocRegDefId
                                                     from:(NSNumber *)from
                                                       to:(NSNumber *)to
                                               resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger buildGetRevocRegDeltaRequestWithSubmitterDid:submitterDid
                                               revocRegDefId:revocRegDefId
                                                        from:from
                                                          to:to
                                                  completion:^(NSError *error, NSString *request) {
                                                      err = error;
                                                      result = request;
                                                      [completionExpectation fulfill];
                                                  }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

- (NSError *)parseGetRevocRegDeltaResponse:(NSString *)getRevocRegDeltaResponse
                             revocRegDefId:(NSString **)revocRegDefId
                         revocRegDeltaJson:(NSString **)revocRegDeltaJson
                                 timestamp:(NSNumber **)timestamp {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *outId = nil;
    __block NSString *outJson = nil;
    __block NSNumber *outTimestamp = nil;

    [IndyLedger parseGetRevocRegDeltaResponse:(NSString *) getRevocRegDeltaResponse
                                   completion:^(NSError *error, NSString *id, NSString *json, NSNumber *oTimestamp) {
                                       err = error;
                                       outId = id;
                                       outJson = json;
                                       outTimestamp = oTimestamp;
                                       [completionExpectation fulfill];
                                   }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (revocRegDefId) {*revocRegDefId = outId;}
    if (revocRegDeltaJson) {*revocRegDeltaJson = outJson;}
    if (timestamp) {*timestamp = outTimestamp;}
    return err;
}

// MARK: - Sign Request

- (NSError *)signRequestWithWalletHandle:(IndyHandle)walletHandle
                            submitterdid:(NSString *)submitterDid
                             requestJson:(NSString *)requestJson
                              resultJson:(NSString **)resultJson {
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];
    __block NSError *err = nil;
    __block NSString *result = nil;

    [IndyLedger signRequest:requestJson submitterDid:submitterDid walletHandle:walletHandle completion:^(NSError *error, NSString *signResult) {
        err = error;
        result = signResult;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations:@[completionExpectation] timeout:[TestUtils longTimeout]];

    if (resultJson) {*resultJson = result;}
    return err;
}

@end
