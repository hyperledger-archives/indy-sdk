//
//  Ledger-MediumCases.m
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

@interface Ledger : XCTestCase

@end

@implementation Ledger

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void) testSendNymRequestWorksForOnlyRequiredFields
{
    [TestUtils cleanupStorage];
    
    [TestUtils cleanupStorage];
}

@end
