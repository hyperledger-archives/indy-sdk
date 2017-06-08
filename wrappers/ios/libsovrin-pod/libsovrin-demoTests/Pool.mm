//
//  Pool.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 05.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import <libsovrin/libsovrin.h>
#import "NSDictionary+JSON.h"

@interface Pool : XCTestCase

@end

@implementation Pool

- (void) testCreatePoolLedgerConfigWorks
{
    [TestUtils cleanupStorage];
    
    NSString *res = nil;
    res = [[PoolUtils sharedInstance] createPoolConfig:@"pool_create"];
    
    XCTAssertNotNil(res, @"Pool config is nil!");
    
    [TestUtils cleanupStorage];
};

- (void) testOpenPoolLedgerWorks
{
    [TestUtils cleanupStorage];
    
    NSString *name = @"pool_open";
    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfig:name];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfig() failed!");
    
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:name config:nil poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    
    [TestUtils cleanupStorage];
}

- (void)testOpenPoolLedgerWorksForTwice
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool_open_twice";
    
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfig:poolName];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfig() failed!");
    
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");
    
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    // TODO: PoolLedgerInvalidConfiguration is returned.
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::openPoolLedger() failed!");
    
    [TestUtils cleanupStorage];
}

- (void) testSovrinSubmitRequestWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"test_submit_tx";
    
    // 1. Create pool ledger config
    NSError *ret = [[PoolUtils sharedInstance] createPoolLedgerConfig:poolName];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfig() failed!");
    
    // 2. Open pool ledger
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::openPoolLedger() failed!");

    
    NSString *request = [NSString stringWithFormat:@"{"\
                         "\"reqId\":1491566332010860," \
                         "\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\"," \
                         "\"operation\":{"\
                                "\"type\":\"105\","\
                                "\"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\"},"\
                         "\"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"" \
                         "}"];
    
    NSString *responseJson;
    // TODO: 110 error, response is empty
    ret = [[PoolUtils sharedInstance] sendRequest:poolHandle
                                          request:request
                                         response:&responseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed!");
    NSLog(@"responseJson: %@", responseJson);
    
    NSDictionary *actualReply = [NSDictionary fromString:responseJson];
    
    
    NSString *dataStr = [NSString stringWithFormat:@"{"\
                         "\"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\"," \
                         "\"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\"," \
                         "\"role\":\"2\"," \
                         "\"verkey\":null" \
                         "}"];
    
    NSString *actualData = actualReply[@"result"][@"data"];
    XCTAssertTrue([actualReply[@"op"] isEqualToString:@"REPLY"], @"Wrong actualReply[op]");
    XCTAssertEqual(actualReply[@"result"][@"reqId"], @(1491566332010860), @"Wrong actualReply[reqId]");
    XCTAssertTrue([actualData isEqualToString:dataStr], "Wrong actualReply[result][data]");
    XCTAssertTrue([actualReply[@"result"][@"identifier"] isEqualToString:@"Th7MpTaRZVRYnPiabds81Y"], @"Wrong actualReply[identifier]" );

    [TestUtils cleanupStorage];
}

@end
