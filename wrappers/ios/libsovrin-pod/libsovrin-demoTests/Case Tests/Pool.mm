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


@end
