//
//  LedgerSchemaRequest.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 13.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerSchemaRequest : XCTestCase

@end

@implementation LedgerSchemaRequest

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testBuildSchemaRequestsWorksForCorrectDataJson {
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *data = @"{\"id\":\"id\", \"name\":\"name\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"101";
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"data"][@"name"] = @"name";
    expectedResult[@"operation"][@"data"][@"version"] = @"1.0";
    expectedResult[@"operation"][@"data"][@"attr_names"] = [[NSArray alloc] initWithObjects:@"name", nil];

    NSString *schemaRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:identifier
                                                                               data:data
                                                                         resultJson:&schemaRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");
    XCTAssertNotNil(schemaRequestJson, @"schemaRequestJson is nil!");
    NSLog(@"schemaRequestJson: %@", schemaRequestJson);

    NSDictionary *request = [NSDictionary fromString:schemaRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

- (void)testBuildGetSchemaRequestsWorksForCorrectDataJson {
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *id = @"NcYxiDXkpYi6ov5FcYDi1e:02:name:1.0";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"identifier"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"107";
    expectedResult[@"operation"][@"dest"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"data"][@"name"] = @"name";
    expectedResult[@"operation"][@"data"][@"version"] = @"1.0";

    NSString *getSchemaRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:identifier
                                                                                  id:id
                                                                            resultJson:&getSchemaRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequestWithSubmitterDid() failed");
    NSDictionary *request = [NSDictionary fromString:getSchemaRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

- (void)testSchemaRequestWorksWithoutSignature {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_schema_request_works_without_signature";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");


    // 3. Obtain my did
    NSString *myDid = [[DidUtils sharedInstance] createStoreAndPublishMyDidWithWalletHandle:walletHandle
                                                                                 poolHandle:poolHandle];

    // 4. Build schema request

    NSString *schemaData = @"{\"id\":\"id\", \"name\":\"name\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";

    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaData
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");

    // 5. Send request
    NSString *schemaResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:schemaRequest
                                                       response:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::sendRequestWithPoolHandle() returned not Success");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:schemaResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testSchemaRequestsWorks {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_schema_requests_works";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. Obtain trustee did

    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    NSLog(@"trusteeDid: %@", trusteeDid);

    // 4. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed for myDid");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");

    // 5. Build nym request

    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:@"TRUSTEE"
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");

    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    // 7. Build schema request
    __block NSString *schemaId;
    __block NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:myDid
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequest() failed");
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");

    // 8. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");

    // 9. Build getSchemaRequest
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                         id:schemaId
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");
    XCTAssertNotNil(getSchemaRequest, @"getSchemaRequest is nil!");

//    [NSThread sleepForTimeInterval: 10];

    // 10. Send getSchemaRequest
    NSString *getSchemaResponse = nil;

    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getSchemaRequest
                                                       response:&getSchemaResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getSchemaResponse, @"getSchemaResponse is nil!");

    ret = [[LedgerUtils sharedInstance] parseGetSchemaResponse:getSchemaResponse
                                                      schemaId:&schemaId
                                                    schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::parseGetSchemaResponse() failed");
    XCTAssertNotNil(schemaId, @"schemaId is nil!");
    XCTAssertNotNil(schemaJson, @"schemaJson is nil!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}
@end
