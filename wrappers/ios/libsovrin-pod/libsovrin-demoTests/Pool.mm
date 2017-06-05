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

- (void) createPoolLedgerConfigWorks
{
    [TestUtils cleanupStorage];
    
    NSString *res = nil;
    res = [[PoolUtils sharedInstance] createPoolConfig:@"pool_create"];
    
    XCTAssertNotNil(res, @"Pool config is nil!");
    
    [TestUtils cleanupStorage];
};

@end
