//
//  LedgerAttribRequest.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 13.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerAttribRequest : XCTestCase

@end

@implementation LedgerAttribRequest

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testAttribRequestWorksForUnknownDid
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_attrib_request_works_for_unknown_did";
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
    NSString *myDid = nil;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build attrib request
    
    NSString *attribRequest;
    NSString *raw = @"{"\
    "\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}"\
    "}";
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                      hash:nil
                                                                       raw:raw
                                                                       enc:nil resultJson:&attribRequest];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::buildAttribRequestWithSubmitterDid() failed");
    XCTAssertNotNil(attribRequest, @"attribRequest is nil!");
    
    // 6. Send request
    NSString *attribResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:attribRequest
                                                       response:&attribResponse];
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(attribResponse, @"attribResponse is nil!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetAttribRequestWorksForUnknownDid
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_get_attrib_request_works_for_unknown_did";
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
    NSString *myDid = nil;
    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My2\"" \
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build get attrib request
    
    NSString *getAttribRequest;
    ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:myDid
                                                                    targetDid:myDid
                                                                         data:@"endpoint"
                                                                   resultJson:&getAttribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequestWithSubmitterDid() failed");
    XCTAssertNotNil(getAttribRequest, @"getAttribRequest is nil!");
    
    // 6. Send request
    NSString *getAttribResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getAttribRequest
                                                       response:&getAttribResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getAttribResponse, @"getAttribResponse is nil!");
    XCTAssertFalse([getAttribResponse isEqualToString:@""], @"getAttribResponse is empty!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testGetAttribrequestWorksForUnknownAttribute
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_get_attrib_request_works_for_unknown_attribute";
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
    NSString *myDid = nil;
    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000Trustee1\","\
                           "\"cid\":true"\
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build get attrib request
    
    NSString *getAttribRequest;
    ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:myDid
                                                                    targetDid:myDid
                                                                         data:@"some_attribute"
                                                                   resultJson:&getAttribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequestWithSubmitterDid() failed");
    XCTAssertNotNil(getAttribRequest, @"getAttribRequest is nil!");
    
    // 6. Send request
    NSString *getAttribResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getAttribRequest
                                                       response:&getAttribResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getAttribResponse, @"getAttribResponse is nil!");
    XCTAssertFalse([getAttribResponse isEqualToString:@""], @"getAttribResponse is empty!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testBuildAttribRequestWorksForInvalidIdentifier
{
    NSString *identifier = @"invalid_base58_identifier";
    
    NSError *ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:identifier
                                                                          targetDid:identifier
                                                                               hash:nil
                                                                                raw:@"{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"
                                                                                enc:nil
                                                                         resultJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure,@"LedgerUtils::buildAttribRequestWithSubmitterDid returned wrong error code");
}

- (void)testBuildGetAttribRequestWorksForInvalidIdentifier
{
    NSString *identifier = @"invalid_base58_identifier";
    
    NSError *ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:identifier
                                                                             targetDid:identifier
                                                                                  data:@"endpoint"
                                                                            resultJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure,@"LedgerUtils::buildGetAttribRequestWithSubmitterDid returned wrong error code");
}

@end
