//
//  DidHighCases.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <Indy/Indy.h>
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "NSDictionary+JSON.h"
#import <CoreBitcoin+Categories.h>

@interface DidHignCases : XCTestCase

@end

@implementation DidHignCases

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

// MARK: - Create my did

- (void)testCreateMyDidWorksForEmptyJson
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *myDid;
    NSString *myVerKey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertEqual([[myDid dataFromBase58] length] , 16, @"length of myDid != 16");
    XCTAssertEqual([[myVerKey dataFromBase58] length], 32, @"length of myVerKey != 32");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksWithSeed
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = @"{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}";
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue([myDid isEqualToString:@"NcYxiDXkpYi6ov5FcYDi1e"], @"wrong myDid!");
    XCTAssertTrue([myVerKey isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myVerKey!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksAsCid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = @"{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", "\
        "\"cid\":true}";
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue([myDid isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myDid!");
    XCTAssertTrue([myVerKey isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myVerKey!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksWithPassedDid
{
    [TestUtils cleanupStorage];
    NSString *poolName = [TestUtils pool];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *did = @"8wZcEriaNLNKtteJvx7f8i";
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = [NSString stringWithFormat:@"{\"did\":\"%@\", "\
                           "\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}", did];

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue([myDid isEqualToString:did], @"wrong myDid!");
    XCTAssertTrue([myVerKey isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myVerKey!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorkForExistsCryptoType
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{\"crypto_type\":\"ed25519\"}"
                                                           outMyDid:nil
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() returned wrong error code");
    
    // 3. close wallet
    [[WalletUtils sharedInstance] closeWalletWithHandle: walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForinvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did with invalid wallet handle
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:invalidWalletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:nil
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"DidUtils::createMyDidWithWalletHandle() returned wrong error code");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Replace keys Start

- (void)testReplaceKeysStartWorks
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myVerkey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Replace keys
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];

    XCTAssertFalse([myVerkey isEqualToString:newVerkey], @"verkey is the same!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myVerkey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() returned wrong error code");
    
    // 3. Replace keys with invalid wallet handle
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{}"
                                                  walletHandle:invalidWalletHandle
                                                   outMyVerKey:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"DidUtils:replaceKeysStartForDid failed");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysStartWorksForSeed
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myVerkey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() returned wrong error code");
    
    // 3. replace keys
    
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];

    XCTAssertEqual(ret.code, Success, @"DidUtils:replaceKeysStartForDid failed");
    XCTAssertTrue([newVerkey isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong newVerkey");
    XCTAssertFalse([myVerkey isEqualToString:newVerkey], @"verkey is the same!");
    
 
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Replace keys apply

- (void)replaceKeysApplyWorks
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myVerkey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Replace keys start
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysStartForDid() failed");
    
    XCTAssertFalse([myVerkey isEqualToString:newVerkey], @"verkey is the same!");
    
    // 4. Replace keys apply
    
    ret = [[DidUtils sharedInstance] replaceKeysApplyForDid:myDid
                                                  walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysApplyForDid() failed");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysApplyWorksWithoutCallingReplaceStart
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Replace keys apply
    
    ret = [[DidUtils sharedInstance] replaceKeysApplyForDid:myDid
                                                  walletHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"DidUtils::replaceKeysApplyForDid() returned wrong error code.");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysApplyWorksForUnknownDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Replace keys start
    
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysStartForDid() failed");
    
    // 4. replace keys apply
    
    ret = [[DidUtils sharedInstance] replaceKeysApplyForDid:@"UnknonwDid11111111111"
                                                  walletHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"DidUtils::replaceKeysApplyForDid() returned wrong error code.");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysApplyWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Replace keys start
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysStartForDid() failed");
    
    // 4. replace keys apply
    
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[DidUtils sharedInstance] replaceKeysApplyForDid:myDid
                                                  walletHandle:invalidWalletHandle];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"DidUtils::replaceKeysApplyForDid() returned wrong error code.");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Store their did
- (void)testStoreTheidDidWorks
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"DidUtils:storeTheirDid failed");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksForInvalidJson
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"field\":\"value\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"DidUtils:storeTheirDid returned wrong error");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\"}";
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:invalidWalletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"DidUtils:storeTheirDid returned wrong error");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksWithVerkey
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\","\
                                "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"DidUtils:storeTheirDid() failed");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testStoretheirDidWorksWithoutDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"DidUtils:storeTheirDidWithWalletHandle() returned wrong code");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];

}

- (void)testStoreTheirDidWorksForCorrectCryptoType
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", \"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\", \"crypto_type\": \"ed25519\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"DidUtils:storeTheirDidWithWalletHandle() failed");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testStoreTheitDidWorksWithAbbreviatedVerkey
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", \"verkey\":\"~NcYxiDXkpYi6ov5FcYDi1e\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"DidUtils:storeTheirDidWithWalletHandle() failed");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

@end
