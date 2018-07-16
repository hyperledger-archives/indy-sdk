#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface LedgerBuildRequest : XCTestCase

@end

@implementation LedgerBuildRequest {
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [TestUtils cleanupStorage];
    [super tearDown];
}

// MARK: Nym request

- (void)testBuildNymRequestsWorksForOnlyRequiredFields {
    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                              targetDid:[TestUtils myDid1]
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"1",
                    @"dest": [TestUtils myDid1]

            }
    };

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
}

- (void)testBuildNymRequestsWorksWithOptionFields {
    NSString *role = @"STEWARD";
    NSString *alias = @"some_alias";

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                              targetDid:[TestUtils myDid1]
                                                                 verkey:[TestUtils myVerkey1]
                                                                  alias:alias
                                                                   role:role
                                                             outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"1",
                    @"dest": [TestUtils myDid1],
                    @"alias": alias,
                    @"role": @"2"

            }
    };

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
}

- (void)testBuildGetNymRequestWorks {
    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"105",
                    @"dest": [TestUtils myDid1]

            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                 targetDid:[TestUtils myDid1]
                                                                outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed!");

    NSDictionary *getNymRequest = [NSDictionary fromString:requestJson];
    XCTAssertTrue([getNymRequest contains:expectedResult], @"Request doesn't contain expectedResult");
}

// MARK: Attrib request

- (void)testBuildAttribRequestsWorksForRawData {
    NSDictionary *raw = @{
            @"endpoint": @{
                    @"ha": [TestUtils endpoint]
            }

    };

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"100",
                    @"dest": [TestUtils trusteeDid],
                    @"raw": [[AnoncredsUtils sharedInstance] toJson:raw]
            }

    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                 targetDid:[TestUtils trusteeDid]
                                                                      hash:nil
                                                                       raw:[[AnoncredsUtils sharedInstance] toJson:raw]
                                                                       enc:nil
                                                                resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");

    NSDictionary *attribRequest = [NSDictionary fromString:requestJson];

    XCTAssertTrue([attribRequest contains:expectedResult], @"attribRequest doesn't contains expectedResult!");
}

- (void)testBuildGetAttribRequestsWorks {
    NSString *raw = @"endpoint";

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"104",
                    @"dest": [TestUtils trusteeDid],
                    @"raw": raw
            }

    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                    targetDid:[TestUtils trusteeDid]
                                                                          raw:raw
                                                                         hash:nil
                                                                          enc:nil
                                                                   resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequestWithSubmitterDid() returned wrong error");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expextedresult");
}

// MARK: Schema request

- (void)testBuildSchemaRequestsWorks {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"101",
                    @"data": @{
                            @"name": [TestUtils gvtSchemaName],
                            @"version": [TestUtils schemaVersion],
                            @"attr_names": @[@"name"]
                    }
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                      data:[TestUtils gvtSchema]
                                                                resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

- (void)testBuildGetSchemaRequestsWorks {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"107",
                    @"dest": [TestUtils issuerDid],
                    @"data": @{
                            @"name": [TestUtils gvtSchemaName],
                            @"version": [TestUtils schemaVersion]
                    }
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                           id:[[AnoncredsUtils sharedInstance] getGvtSchemaId]
                                                                   resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

// MARK: Cred Def request

- (void)testBuildCredDefRequestWorksForCorrectDataJson {
    NSDictionary *data = @{
            @"ver": @"1.0",
            @"id": @"id",
            @"schemaId": @"1",
            @"type": @"CL",
            @"tag": @"TAG1",
            @"value": @{
                    @"primary": @{
                            @"n": @"1",
                            @"s": @"2",
                            @"r": @{@"height": @"1",@"master_secret": @"1"},
                            @"rctxt": @"1",
                            @"z": @"1"
                    }
            }
    };

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils issuerDid],
            @"operation": @{
                    @"ref": @(1),
                    @"data": @{
                            @"type": @"102",
                            @"signature_type": @"CL",
                            @"tag": @"TAG1",
                            @"primary": data[@"value"][@"primary"]
                    }
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildCredDefRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                       data:[[AnoncredsUtils sharedInstance] toJson:data]
                                                                 resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildCredDefRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

- (void)testBuildGetCredDefRequestWorks {
    NSString *id = @"NcYxiDXkpYi6ov5FcYDi1e:03:CL:1:TAG";

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils issuerDid],
            @"operation": @{
                    @"ref": @(1),
                    @"type": @"108",
                    @"signature_type": @"CL",
                    @"teg": @"TAG1",
                    @"origin": [TestUtils issuerDid]
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetCredDefRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                            id:id
                                                                    resultJson:&requestJson];

    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetCredDefRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

// MARK: - Revoc Reg Def request

- (void)testBuildRevocRegDefRequestWorks {
    NSDictionary *data = @{
            @"ver": @"1.0",
            @"id": @"RevocRegID",
            @"revocDefType": @"CL_ACCUM",
            @"tag": @"TAG_1",
            @"credDefId": @"CredDefID",
            @"value": @{
                    @"issuanceType": @"ISSUANCE_ON_DEMAND",
                    @"maxCredNum": @(5),
                    @"tailsHash": @"s",
                    @"tailsLocation": @"http://tails.location.com",
                    @"publicKeys": @{
                            @"accumKey": @{
                                    @"z": @"1111 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
                            }
                    }
            }
    };

    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"113",
                    @"id": @"RevocRegID",
                    @"revocDefType": @"CL_ACCUM",
                    @"tag": @"TAG_1",
                    @"credDefId": @"CredDefID",
                    @"value": data[@"value"]
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildRevocRegDefRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                           data:[[AnoncredsUtils sharedInstance] toJson:data]
                                                                     resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildRevocRegDefRequestWithSubmitterDid() failed");

    NSDictionary *revocRegDefReques = [NSDictionary fromString:requestJson];

    XCTAssertTrue([revocRegDefReques contains:expectedResult], @"getTxnRequest json doesn't contain expectedResult json");
}

- (void)testBuildGetRevocRegDefRequestWorks {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"115",
                    @"revocRegDefId": @"RevocRegID"
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegDefRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                                id:@"RevocRegID"
                                                                        resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSDictionary *getRevocRegDefRequest = [NSDictionary fromString:requestJson];

    XCTAssertTrue([getRevocRegDefRequest contains:expectedResult], @"getTxnRequest json doesn't contain expectedResult json");
}

// MARK: - Revoc Reg Entry request

- (void)testBuildRevocRegEntryRequestWorks {
    NSDictionary *data = @{
            @"value": @{
                    @"accum": @"false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
            },
            @"ver": @"1.0"
    };

    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"114",
                    @"revocRegDefId": @"RevocRegID",
                    @"revocDefType": @"CL_ACCUM",
                    @"value": @{
                            @"accum": @"false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
                    }
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildRevocRegEntryRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                             type:@"CL_ACCUM"
                                                                    revocRegDefId:@"RevocRegID"
                                                                            value:[[AnoncredsUtils sharedInstance] toJson:data]
                                                                       resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildRevocRegEntrtyRequestWithSubmitterDid() failed");

    NSDictionary *revocRegEntryReques = [NSDictionary fromString:requestJson];

    XCTAssertTrue([revocRegEntryReques contains:expectedResult], @"revocRegEntryReques json doesn't contain expectedResult json");
}

// MARK: - Revoc Reg request

- (void)testBuildGetRevocRegRequestWorks {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"116",
                    @"revocRegDefId": @"RevRegId",
                    @"timestamp": @(100)
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                  revocRegDefId:@"RevRegId"
                                                                      timestamp:@(100)
                                                                     resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetRevocRegRequestWithSubmitterDid() failed");

    NSDictionary *getRevocRegReques = [NSDictionary fromString:requestJson];

    XCTAssertTrue([getRevocRegReques contains:expectedResult], @"getRevocRegReques json doesn't contain expectedResult json");
}

// MARK: - Revoc Reg Delta request

- (void)testBuildGetRevocRegDeltaRequestWorks {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"117",
                    @"revocRegDefId": @"RevRegId",
                    @"from": @(0),
                    @"to": @(100)
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegDeltaRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                       revocRegDefId:@"RevRegId"
                                                                                from:@(0)
                                                                                  to:@(100)
                                                                          resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetRevocRegRequestWithSubmitterDid() failed");

    NSDictionary *getRevocRegDeltaReques = [NSDictionary fromString:requestJson];

    XCTAssertTrue([getRevocRegDeltaReques contains:expectedResult], @"getRevocRegDeltaReques json doesn't contain expectedResult json");
}

// MARK: Pool upgrade

- (void)testBuildPoolUpgradeRequestsWorksForStartAction {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"109",
                    @"name": @"upgrade-ios",
                    @"version": @"2.0.0",
                    @"action": @"start",
                    @"sha256": @"f284b",
                    @"schedule": @"{}",
                    @"reinstall": @(NO),
                    @"force": @(NO)
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                           name:@"upgrade-ios"
                                                                        version:@"2.0.0"
                                                                         action:@"start"
                                                                         sha256:@"f284b"
                                                                        timeout:nil
                                                                       schedule:@"{}"
                                                                  justification:nil
                                                                      reinstall:false
                                                                          force:false
                                                                     resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}


- (void)testBuildPoolUpgradeRequestsWorksForCancelAction {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"109",
                    @"name": @"upgrade-ios",
                    @"version": @"2.0.0",
                    @"action": @"cancel",
                    @"sha256": @"f284b",
                    @"reinstall": @(NO),
                    @"force": @(NO)
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                           name:@"upgrade-ios"
                                                                        version:@"2.0.0"
                                                                         action:@"cancel"
                                                                         sha256:@"f284b"
                                                                        timeout:nil
                                                                       schedule:nil
                                                                  justification:nil
                                                                      reinstall:false
                                                                          force:false
                                                                     resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

// MARK: Pool restart

- (void)testBuildPoolRestartRequestsWorksForStartAction {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"118",
                    @"action": @"start",
                    @"datetime": @"0"
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolRestartRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                         action:@"start"
                                                                       datetime:@"0"
                                                                     resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolRestartRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

- (void)testBuildPoolRestartRequestsWorksForCancelAction {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"118",
                    @"action": @"cancel"
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolRestartRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                         action:@"cancel"
                                                                       datetime:nil
                                                                     resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolRestartRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

- (void)testBuildGetValidatorInfo {
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"119";


    NSString *getValidatorInfoJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetValidatorInfo:identifier
                                                            resultJson:&getValidatorInfoJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::builGetValidatorInfo failed");
    XCTAssertNotNil(getValidatorInfoJson, @"getValidatorInfoJson is nil!");
    NSLog(@"getValidatorInfoJson: %@", getValidatorInfoJson);

    NSDictionary *request = [NSDictionary fromString:getValidatorInfoJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

// MARK: Pool config

- (void)testBuildPoolConfigRequestsWorks {
    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"111",
                    @"writes": @(YES),
                    @"force": @(NO)
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolConfigRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                        writes:true
                                                                         force:false
                                                                    resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

// MARK: Node request

- (void)testBuildNodeRequestWorksForCorrectDataJson {
    NSDictionary *data = @{
            @"node_ip": @"10.0.0.2",
            @"node_port": @(9998),
            @"client_ip": @"10.0.0.2",
            @"client_port": @(9999),
            @"alias": @"Node1",
            @"services": @[@"VALIDATOR"],
            @"blskey": @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
    };

    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"0",
                    @"dest": [TestUtils trusteeVerkey],
                    @"data": data
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                               targetDid:[TestUtils trusteeVerkey]
                                                                    data:[[AnoncredsUtils sharedInstance] toJson:data]
                                                              resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNodeRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
}

// MARK: - Get txn request

- (void)testBuildGetTxnRequest {
    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"3",
                    @"data": @(1),
                    @"ledgerId": @(1)
            }
    };

    NSString *requestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetTxnRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                         ledgerType:nil
                                                                               data:@(1)
                                                                         resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSDictionary *getTxnRequest = [NSDictionary fromString:requestJson];

    XCTAssertTrue([getTxnRequest contains:expectedResult], @"getTxnRequest json doesn't contain expectedResult json");
}

@end
