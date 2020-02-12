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
            @"id": [[AnoncredsUtils sharedInstance] credDefId],
            @"schemaId": @"1",
            @"type": @"CL",
            @"tag": @"TAG1",
            @"value": @{
                    @"primary": @{
                            @"n": @"1",
                            @"s": @"2",
                            @"r": @{@"height": @"1", @"master_secret": @"1"},
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
    NSString *id = [[AnoncredsUtils sharedInstance] credDefId];

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
            @"id": [[AnoncredsUtils sharedInstance] revRegId],
            @"revocDefType": @"CL_ACCUM",
            @"tag": @"TAG_1",
            @"credDefId": [[AnoncredsUtils sharedInstance] credDefId],
            @"value": @{
                    @"issuanceType": @"ISSUANCE_ON_DEMAND",
                    @"maxCredNum": @(5),
                    @"tailsHash": @"s",
                    @"tailsLocation": @"http://tails.location.com",
                    @"publicKeys": @{
                            @"accumKey": @{
                                    @"z": @"1 0000000000000000000000000000000000000000000000000000000000001111 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000"
                            }
                    }
            }
    };

    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"113",
                    @"id": [[AnoncredsUtils sharedInstance] revRegId],
                    @"revocDefType": @"CL_ACCUM",
                    @"tag": @"TAG_1",
                    @"credDefId": [[AnoncredsUtils sharedInstance] credDefId],
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
                    @"revocRegDefId": [[AnoncredsUtils sharedInstance] revRegId]
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegDefRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                                id:[[AnoncredsUtils sharedInstance] revRegId]
                                                                        resultJson:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSDictionary *getRevocRegDefRequest = [NSDictionary fromString:requestJson];

    XCTAssertTrue([getRevocRegDefRequest contains:expectedResult], @"getTxnRequest json doesn't contain expectedResult json");
}

// MARK: - Revoc Reg Entry request

- (void)testBuildRevocRegEntryRequestWorks {
    NSDictionary *data = @{
            @"value": @{
                    @"accum": @"1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000"
            },
            @"ver": @"1.0"
    };

    NSDictionary *expectedResult = @{
            @"operation": @{
                    @"type": @"114",
                    @"revocRegDefId": [[AnoncredsUtils sharedInstance] revRegId],
                    @"revocDefType": @"CL_ACCUM",
                    @"value": @{
                            @"accum": @"1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000 1 0000000000000000000000000000000000000000000000000000000000000000"
                    }
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildRevocRegEntryRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                             type:@"CL_ACCUM"
                                                                    revocRegDefId:[[AnoncredsUtils sharedInstance] revRegId]
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
                    @"revocRegDefId": [[AnoncredsUtils sharedInstance] revRegId],
                    @"timestamp": @(100)
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                  revocRegDefId:[[AnoncredsUtils sharedInstance] revRegId]
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
                    @"revocRegDefId": [[AnoncredsUtils sharedInstance] revRegId],
                    @"from": @(0),
                    @"to": @(100)
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetRevocRegDeltaRequestWithSubmitterDid:[TestUtils issuerDid]
                                                                       revocRegDefId:[[AnoncredsUtils sharedInstance] revRegId]
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
                                                                       package_:nil
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
                                                                       package_:nil
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

// MARK: Auth Rule request

- (void)testBuildAuthRuleRequestsWorks {
    NSDictionary *constraint = @{
            @"sig_count": @(1),
            @"role": @"0",
            @"constraint_id": @"ROLE",
            @"need_to_be_owner": @(false)
    };

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"120",
                    @"auth_type": @"1",
                    @"auth_action": @"ADD",
                    @"field": @"role",
                    @"new_value": @"101",
                    @"constraint": constraint,
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildAuthRuleRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                     txnType:@"NYM"
                                                                      action:@"ADD"
                                                                       field:@"role"
                                                                    oldValue:nil
                                                                    newValue:@"101"
                                                                  constraint:[NSDictionary toString:constraint]
                                                                  outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAuthRuleRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
}

- (void)testBuildAuthRulesRequestsWorks {
    NSDictionary *constraint = @{
            @"sig_count": @(1),
            @"role": @"0",
            @"constraint_id": @"ROLE",
            @"need_to_be_owner": @(false)
    };

    NSArray *data = @[
        @{
            @"auth_type": @"1",
            @"auth_action": @"ADD",
            @"field": @"role",
            @"new_value": @"101",
            @"constraint": constraint
        },
        @{
            @"auth_type": @"1",
            @"auth_action": @"EDIT",
            @"field": @"role",
            @"old_value": @"0",
            @"new_value": @"101",
            @"constraint": constraint
        },
    ];

    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"122",
                    @"rules": data,
            }
    };
    
    NSError *error;
    NSData *jsonData = [NSJSONSerialization dataWithJSONObject:data options:NSJSONWritingPrettyPrinted error:&error];
    NSString *dataJson = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
    
    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildAuthRulesRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                         data:dataJson
                                                                   outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAuthRulesRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
}

- (void)testBuildGetAuthRuleRequestsWorks {
    NSDictionary *expectedResult = @{
            @"identifier": [TestUtils trusteeDid],
            @"operation": @{
                    @"type": @"121",
                    @"auth_type": @"1",
                    @"auth_action": @"ADD",
                    @"field": @"role",
                    @"new_value": @"101",
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetAuthRuleRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                        txnType:@"NYM"
                                                                         action:@"ADD"
                                                                          field:@"role"
                                                                       oldValue:nil
                                                                       newValue:@"101"
                                                                     outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAuthRuleRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
}

// MARK: Author Agreement request

- (void)testBuildTxnAuthorAgreementRequestWorks {
    NSDictionary *expectedResult = @{
            @"type": @"4",
            @"text": @"indy agreement",
            @"version": @"1.0.0",
            @"ratification_ts": @(12345),
            @"retirement_ts": @(54321),
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildTxnAuthorAgreementRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                                  text:@"indy agreement"
                                                                               version:@"1.0.0"
                                                                 ratificationTimestamp:@(12345)
                                                                   retirementTimestamp:@(54321)
                                                                            outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildTxnAuthorAgreementRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
}

- (void)testBuildDisableAllTxnAuthorAgreementsRequestWithSubmitterDid {
    NSDictionary *expectedResult = @{
            @"type": @"8",
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildDisableAllTxnAuthorAgreementsRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                                       outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildDisableAllTxnAuthorAgreementsRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([expectedResult isEqualToDictionary:request[@"operation"]], @"Wrong Result Json!");
}

- (void)testBuildGetTxnAuthorAgreementRequestWorks {
    NSDictionary *expectedResult = @{
            @"type": @"6",
            @"version": @"1.0.0",
    };

    NSDictionary *data = @{
            @"version": @"1.0.0",
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetTxnAuthorAgreementRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                                     data:[NSDictionary toString:data]
                                                                               outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildTxnAuthorAgreementRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([expectedResult isEqualToDictionary:request[@"operation"]], @"Wrong Result Json!");
}

// MARK: Acceptance Mechanism

- (void)testBuildAcceptanceMechanismRequestWorks {
    NSDictionary *aml = @{
            @"acceptance mechanism label 1": @"description",
    };
    NSString *version = @"1.0.0";
    NSString *context = @"some context";

    NSDictionary *expectedResult = @{
            @"type": @"5",
            @"aml": aml,
            @"version": version,
            @"amlContext": context,
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildAcceptanceMechanismsRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                                     aml:[NSDictionary toString:aml]
                                                                                 version:version
                                                                              amlContext:context
                                                                              outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAcceptanceMechanismsRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([expectedResult isEqualToDictionary:request[@"operation"]], @"Wrong Result Json!");
}

- (void)testBuildGetAcceptanceMechanismRequestWorksForTimestamp {
    NSDictionary *expectedResult = @{
            @"type": @"7",
            @"timestamp": @(123456789),
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetAcceptanceMechanismsRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                                  timestamp:@(123456789)
                                                                                    version:nil
                                                                                 outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAcceptanceMechanismsRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([expectedResult isEqualToDictionary:request[@"operation"]], @"Wrong Result Json!");
}

- (void)testBuildGetAcceptanceMechanismsRequestWorksForVersion {
    NSString *version = @"1.0.0";

    NSDictionary *expectedResult = @{
            @"type": @"7",
            @"version": version,
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildGetAcceptanceMechanismsRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                                 timestamp:nil
                                                                                   version:version
                                                                                outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAcceptanceMechanismsRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([expectedResult isEqualToDictionary:request[@"operation"]], @"Wrong Result Json!");
}

// MARK: Author Agreement Acceptance Data

- (void)testAppendTxnAuthorAgreementAcceptanceToRequestWorks {
    NSDictionary *request = @{
            @"reqId": @(1496822211362017764),
            @"identifier": @"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            @"operation": @{
                    @"type": @"1",
                    @"dest": @"VsKV7grR1BUE29mG2Fm2kX",
                    @"dest": @"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] appendTxnAuthorAgreementAcceptanceToRequest:[NSDictionary toString:request]
                                                                               text:@"some agreement text"
                                                                            version:@"1.0.0"
                                                                          taaDigest:@"050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e"
                                                                        accMechType:@"acceptance type 1"
                                                                   timeOfAcceptance:@(123379200)
                                                                         outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildTxnAuthorAgreementRequestWithSubmitterDid() failed!");
    NSDictionary *expectedMeta = @{
            @"mechanism": @"acceptance type 1",
            @"taaDigest": @"050e52a57837fff904d3d059c8a123e3a04177042bf467db2b2c27abd8045d5e",
            @"time": @(123379200),
    };

    request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([expectedMeta isEqualToDictionary:request[@"taaAcceptance"]], @"Wrong Result Json!");
}

// MARK: Endorser

- (void)testAppendEndorserToRequestWorks {
    NSDictionary *request = @{
            @"reqId": @(1496822211362017764),
            @"identifier": @"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            @"operation": @{
                    @"type": @"1",
                    @"dest": @"VsKV7grR1BUE29mG2Fm2kX",
                    @"dest": @"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
            }
    };

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] appendEndorserToRequest:[NSDictionary toString:request]
                                                    endorserDid:[TestUtils trusteeDid]
                                                     outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildTxnAuthorAgreementRequestWithSubmitterDid() failed!");

    request = [NSDictionary fromString:requestJson];

    XCTAssertTrue([[TestUtils trusteeDid] isEqualToString:request[@"endorser"]]);
}

@end
