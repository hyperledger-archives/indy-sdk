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
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. issuer create claim definition
    // TODO: 109 error
    NSString *schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)];
    NSString *claimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:NO
                                                                          claimDefJson:&claimDefJson
                                                                          claimDefUUID:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle failed");
    XCTAssertNotNil(claimDefJson, @"claimDefJsom is nil!");
    
    // 3. Check claim definition
    NSDictionary *claimDef = [NSDictionary fromString:claimDefJson];
    XCTAssertEqual([claimDef[@"public_key"][@"r"] length], 4, @"wrong length:claimDef[publicKey][r]");
    XCTAssertTrue([claimDef[@"publicKey"][@"n"] length] > 0, @"wrong length:claimDef[publicKey][n]");
    XCTAssertTrue([claimDef[@"publicKey"][@"s"] length] > 0, @"wrong length:claimDef[publicKey][s]");
    XCTAssertTrue([claimDef[@"publicKey"][@"rms"] length] > 0, @"wrong length:claimDef[publicKey][rms]");
    XCTAssertTrue([claimDef[@"publicKey"][@"z"] length] > 0, @"wrong length:claimDef[publicKey][z]");
    XCTAssertTrue([claimDef[@"publicKey"][@"rctxt"] length] > 0, @"wrong length:claimDef[publicKey][rctxt]");
    
    [TestUtils cleanupStorage];
}

- (void)testIssuerCreateAndStoreClaimDefWorksForInvalidWallet
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. init commmon wallet
    SovrinHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    NSString *schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)];
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:invalidWalletHandle
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:NO
                                                                          claimDefJson:nil
                                                                          claimDefUUID:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle failed: returned wrong error code");
    
    [TestUtils cleanupStorage];
}

@end
