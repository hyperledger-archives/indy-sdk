//
//  SignusHighCases.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libsovrin/libsovrin.h>
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "LedgerUtils.h"
#import "NSDictionary+JSON.h"
#import <CoreBitcoin+Categories.h>

@interface SignusHignCases : XCTestCase

@end

@implementation SignusHignCases

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
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *myDid;
    NSString *myVerKey;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertEqual([[myDid dataFromBase58] length] , 16, @"length of myDid != 16");
    XCTAssertEqual([[myVerKey dataFromBase58] length], 32, @"length of myVerKey != 32");
    
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksWithSeed
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = @"{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue([myDid isEqualToString:@"NcYxiDXkpYi6ov5FcYDi1e"], @"wrong myDid!");
    XCTAssertTrue([myVerKey isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myVerKey!");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksAsCid
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = @"{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\", "\
        "\"cid\":true}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue([myDid isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myDid!");
    XCTAssertTrue([myVerKey isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myVerKey!");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksWithPassedDid
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Obtain my did
    NSString *did = @"8wZcEriaNLNKtteJvx7f8i";
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = [NSString stringWithFormat:@"{\"did\":\"%@\", "\
                           "\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}", did];

    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue([myDid isEqualToString:did], @"wrong myDid!");
    XCTAssertTrue([myVerKey isEqualToString:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"], @"wrong myVerKey!");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForinvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did with invalid wallet handle
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:invalidWalletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:nil
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils::createMyDidWithWalletHandle() returned wrong error code");
    [TestUtils cleanupStorage];
}

// MARK: - Replace keys

- (void)testReplaceKeysWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Replace keys
    ret = [[SignusUtils sharedInstance] replaceKeysWithWalletHandle:walletHandle
                                                                did:myDid
                                                       identityJson:@"{}"
                                                        outMyVerKey:nil
                                                            outMyPk:nil];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysWorksForInvalidDid
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Replace keys
    ret = [[SignusUtils sharedInstance] replaceKeysWithWalletHandle:walletHandle
                                                                did:@"invalid_base58_string"
                                                       identityJson:@"{}"
                                                        outMyVerKey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils:replaceKeysWithWalletHandle failed");
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() returned wrong error code");
    
    // 3. Replace keys with invalid wallet handle
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[SignusUtils sharedInstance] replaceKeysWithWalletHandle:invalidWalletHandle
                                                                did:myDid
                                                       identityJson:@"{}"
                                                        outMyVerKey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils:replaceKeysWithWalletHandle failed");
    [TestUtils cleanupStorage];
}

// MARK: - Store their did
- (void)testStoreTheidDidWorks
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\"}";
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils:storeTheirDid failed");
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksForInvalidJson
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"field\":\"value\"}";
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils:storeTheirDid returned wrong error");
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksForInvalidHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\"}";
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:invalidWalletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils:storeTheirDid returned wrong error");
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksWithVerkey
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\","\
                                "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"}";
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils:storeTheirDid() failed");
    [TestUtils cleanupStorage];
}

// MARK: - Sign

- (void)testSignWorks
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myDidJson = @"{\"seed\":\"000000000000000000000000Trustee1\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Sign
    NSString *message = @"{"\
                        "\"reqId\":1496822211362017764,"\
                        "\"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\","\
                        "\"operation\":{"\
                            "\"type\":\"1\","\
                            "\"dest\":\"VsKV7grR1BUE29mG2Fm2kX\","\
                            "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\""\
                        "}}";
    
    NSMutableDictionary *expectedSignature = [NSMutableDictionary new];
    expectedSignature[@"signature"] = @"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW";
    
    NSString *signatureJson;
    ret = [[SignusUtils sharedInstance] signWithWalletHandle:walletHandle
                                                    theirDid:myDid message:message
                                                outSignature:&signatureJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::signWithWalletHandle() failed");
    
    NSDictionary *signature = [NSDictionary fromString:signatureJson];
    XCTAssertTrue([signature contains:expectedSignature]);
    
    [TestUtils cleanupStorage];
}

- (void)testSignWorksForUnknownDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Sign
    NSString *message = @"{"\
    "\"reqId\":1495034346617224651}";
    
    NSString *signatureJson;
    ret = [[SignusUtils sharedInstance] signWithWalletHandle:walletHandle
                                                    theirDid:@"some_did"
                                                     message:message
                                                outSignature:&signatureJson];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"SignusUtils::signWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testSignWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myDidJson = @"{\"seed\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Sign
    NSString *message = @"{"\
    "\"reqId\":1495034346617224651}";
    
    NSString *signatureJson;
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[SignusUtils sharedInstance] signWithWalletHandle:invalidWalletHandle
                                                    theirDid:myDid
                                                     message:message
                                                outSignature:&signatureJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils::signWithWalletHandle() returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Verify

- (void)testVerifyWorksForVerkeyCachedInWallet
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"sovrin_verify_works_for_verkey_cached_in_wallet";
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. create did
    NSString *did;
    NSString *verKey;
    NSString *didJson = @"{\"seed\":\"000000000000000000000000Trustee1\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:&did
                                                        outMyVerkey:&verKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue(did, @"invalid did");
    XCTAssertTrue(verKey, @"invalid verKey");
    
    // 4. Store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", did, verKey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 5. Verify
    NSString *message = @"{"\
    "\"reqId\":1496822211362017764,"\
    "\"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\","\
    "\"operation\":{"\
        "\"type\":\"1\","\
        "\"dest\":\"VsKV7grR1BUE29mG2Fm2kX\","\
        "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\""\
    "},"\
    "\"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"}";
    
    BOOL verified = NO;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:did
                                                     signature:message
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::verifyWithWalletHandle() failed");
    XCTAssertTrue(verified, @"verifying failed");
    
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForGetVerkeyFromLedger
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"sovrin_verify_works_for_get_verkey_from_ledger";
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. trustee did
    NSString *trusteeDid;
    NSString *trusteeDidJson = @"{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue(trusteeDid, @"invalid did");
    
    // 4. my did
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = @"{\"seed\":\"00000000000000000000000000000My1\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue(myDid, @"invalid did");
    XCTAssertTrue(myVerKey, @"invalid verkey");
    
    // 5. Build nym request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
     XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    
    // 6. Sign and submit request
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    
    // 7. Store their did
    NSString *message = @"{"\
    "\"reqId\":1496822211362017764,"\
    "\"signature\":\"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai\"}";
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\"}",myDid];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 8. Verify
    BOOL verified = NO;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:myDid
                                                     signature:message
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::verifyWithWalletHandle() failed");
    XCTAssertTrue(verified, @"verifying failed");
    
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForExpiredNym
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"sovrin_verify_works_for_expired_nym";
    NSString *walletName = @"wallet1";
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:nil
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createWalletWithPoolName failed");
    
    // 3. Open wallet
    NSString *config = @"{\"freshness_time\":1}";
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:config
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    // 4. trustee did
    NSString *trusteeDid;
    NSString *trusteeDidJson = @"{\"seed\":\"000000000000000000000000Trustee1\", \"cid\":true}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue(trusteeDid, @"invalid did");
    
    // 5. my did
    NSString *myDid;
    NSString *myVerKey;
    NSString *myDidJson = @"{\"seed\":\"00000000000000000000000000000My1\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertTrue(myDid, @"invalid did");
    XCTAssertTrue(myVerKey, @"invalid verkey");
    
    // 6. Build nym request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    
    // 7. Sign and submit request
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    
    // 7. Store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}",myDid, myVerKey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 8. Verify
    NSString *message = @"{"\
    "\"reqId\":1496822211362017764,"\
    "\"signature\":\"tibTuE59pZn1sCeZpNL5rDzpkpqV3EkDmRpFTizys9Gr3ZieLdGEGyq4h8jsVWW9zSaXSRnfYcVb1yTjUJ7vJai\"}";
    
    BOOL verified = NO;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:myDid
                                                     signature:message
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::verifyWithWalletHandle() failed");
    XCTAssertTrue(verified, @"verifying failed");
    
    [TestUtils cleanupStorage];
}


- (void)testVerifyWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"sovrin_verify_works_for_invalid_wallet_handle";
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    
    // 3. Verify
    NSString *message = @"{"\
    "\"reqId\":1496822211362017764,"\
    "\"identifier\":GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL,"\
    "\"operation\":{"\
        "\"type\":\"1\","\
        "\"dest\":\"VsKV7grR1BUE29mG2Fm2kX\","\
        "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"},"\
    "\"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"}";
    
    // 4. Verify
    BOOL verified = NO;
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:invalidWalletHandle
                                                    poolHandle:poolHandle
                                                           did:@"did"
                                                     signature:message
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils::verifyWithWalletHandle() failed");
    
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForInvalidPoolHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"sovrin_verify_works_for_invalid_pool_handle";
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. Verify
    NSString *message = @"{"\
    "\"reqId\":1496822211362017764,"\
    "\"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\","\
    "\"operation\":{"\
        "\"type\":\"1\","\
        "\"dest\":\"VsKV7grR1BUE29mG2Fm2kX\","\
        "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\"},"\
    "\"signature\":\"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW\"}";
    
    // 4. Verify
    BOOL verified = NO;
    SovrinHandle invalidPoolHandle = poolHandle + 1;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:invalidPoolHandle
                                                           did:@"did"
                                                     signature:message
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"SignusUtils::verifyWithWalletHandle() failed");
    
    [TestUtils cleanupStorage];
}



@end
