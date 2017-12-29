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

- (void)testBuildSchemaRequestsWorksForMissedFieldInDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *data = @"{\"name\":\"name\"}";
    
    NSString *schemaRequest;
    NSError *ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:identifier
                                                                               data:data
                                                                         resultJson:&schemaRequest];
     XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils:buildSchemaRequestWithSubmitterDid returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testBuildSchemaRequestsWorksForInvalidDataJsonFormat
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *data = @"{\"name\":\"name\", "\
                       "\"attr_names\":\"name\"}";
    
    NSString *schemaRequest;
    NSError *ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:identifier
                                                                               data:data
                                                                         resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils:buildSchemaRequestWithSubmitterDid returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testBuildGetSchemaRequestsWorksForInvalidDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *data = @"{\"name\":\"name\"}";
    
    NSString *getSchemaRequest;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:identifier
                                                                                  dest:identifier
                                                                                  data:data
                                                                            resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils:buildGetSchemaRequestWithSubmitterDid returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testSchemaRequestWorksForUnknownDid
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"indy_schema_request_works_for_unknown_did";
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
    NSString* myDid = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My3\"" \
                           "}"];
    
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build schema request
    NSString *schemaData = [NSString stringWithFormat:@"{"\
                            "\"name\":\"gvt2\"," \
                            "\"version\":\"2.0\"," \
                            "\"attr_names\":[\"name\",\"male\"]" \
                            "}"];
    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaData
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequest() failed");
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");
    
    // 5. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() returned not Success");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:schemaResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetSchemaRequestWorksForUnknownName
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"indy_get_schema_request_works_for_unknown_name";
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
    NSString* myDid = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"" \
                           "}"];
    
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build schema request
    NSString *getSchemaData = [NSString stringWithFormat:@"{"\
                            "\"name\":\"schema_name\"," \
                            "\"version\":\"2.0\"" \
                            "}"];
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                         dest:myDid
                                                                         data:getSchemaData
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequestWithSubmitterDid() failed");
    XCTAssertNotNil(getSchemaRequest, @"getSchemaRequest is nil!");
    
    // 5. Send request
    NSString *getSchemaResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getSchemaRequest
                                                       response:&getSchemaResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getSchemaResponse, @"getSchemaResponse is nil!");
    XCTAssertFalse([getSchemaResponse isEqualToString:@""], @"getSchemaResponse is enpty!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

@end
