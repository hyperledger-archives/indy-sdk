//
//  AnoncredsHighCase.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 16.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"

@interface AnoncredsHignCases : XCTestCase

@end

@implementation AnoncredsHignCases

- (void)setUp
{
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown
{
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

// MARK: - Issuer create and store claim def

- (void)testIssuerCreateAndStoreClaimDefWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. init commmon wallet
    SovrinHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefjson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. issuer create claim definition
    NSString *schema = [[AnoncredsUtils sharedInstance] ];
    
    [TestUtils cleanupStorage];
}

@end
