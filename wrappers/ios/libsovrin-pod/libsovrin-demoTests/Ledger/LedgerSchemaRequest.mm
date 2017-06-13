//
//  LedgerSchemaRequest.m
//  libsovrin-demo
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
#import <libsovrin/libsovrin.h>
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

- (void) testSchemaRequestWorksForUnknownDid
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"sovrin_schema_request_works_for_unknown_did";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. Obtain my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My3\"" \
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");
    
    // 4. Build schema request
    NSString *schemaData = [NSString stringWithFormat:@"{"\
                            "\"name\":\"gvt2\"," \
                            "\"version\":\"2.0\"," \
                            "\"keys\":[\"name\",\"male\"]" \
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
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");
    
    [TestUtils cleanupStorage];
}

@end
