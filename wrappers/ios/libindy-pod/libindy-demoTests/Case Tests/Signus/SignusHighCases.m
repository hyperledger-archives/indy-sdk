//
//  SignusHighCases.m
//  libindy-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libindy/libindy.h>
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
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did with invalid wallet handle
    IndyHandle invalidWalletHandle = walletHandle + 1;
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
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    IndyHandle invalidWalletHandle = walletHandle + 1;
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
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                                  xtype:nil
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
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                                  xtype:nil
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
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\"}";
    IndyHandle invalidWalletHandle = walletHandle + 1;
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
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                                  xtype:nil
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
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[SignusUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myDidJson = [NSString stringWithFormat:@"{\"seed\":\"%@\"}",[SignusUtils mySeed]];
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Sign
    
    NSData *signature;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] signWithWalletHandle:walletHandle
                                                    theirDid:myDid
                                                     message:message
                                                outSignature:&signature];
    XCTAssertTrue([signature isEqualToData:[SignusUtils signature]], @"SignusUtils::signWithWalletHandle() failed. Signature is not verified");
    
    [TestUtils cleanupStorage];
}

- (void)testSignWorksForUnknownSigner
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[SignusUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    NSData *signature;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] signWithWalletHandle:walletHandle
                                                    theirDid:@"did"
                                                     message:message
                                                outSignature:&signature];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"SignusUtils::signWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testSignWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[SignusUtils pool]
                                                                  xtype:nil
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
    
    // 3. Sign
    NSData *signature;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[SignusUtils sharedInstance] signWithWalletHandle:invalidWalletHandle
                                                    theirDid:myDid
                                                     message:message
                                                outSignature:&signature];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils::signWithWalletHandle() returned wrong code");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

// MARK: - Verify

- (void)testVerifyWorksForVerkeyCachedInWallet
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. create did
    NSString *did;
    NSString *verKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    XCTAssertTrue(did, @"invalid did");
    XCTAssertTrue(verKey, @"invalid verKey");
    
    // 4. Store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", did, verKey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 5. Verify
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    BOOL verified = NO;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:did
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::verifyWithWalletHandle() failed");
    XCTAssertTrue(verified, @"verifying failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForGetVerkeyFromLedger
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. trustee did
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");
    XCTAssertTrue(trusteeDid, @"invalid did");
    
    // 4. my did
    NSString *myDid;
    NSString *myVerKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:&myVerKey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed for myDid");
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
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\"}",myDid];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 8. Verify
    BOOL verified = NO;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:myDid
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::verifyWithWalletHandle() failed");
    XCTAssertTrue(verified, @"verifying failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForExpiredNym
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    NSString *walletName = @"wallet1";
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
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
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:config
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    // 4. trustee did
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");
    XCTAssertTrue(trusteeDid, @"invalid did");
    
    // 5. my did
    NSString *myDid;
    NSString *myVerKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:&myVerKey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
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
    BOOL verified = NO;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:myDid
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::verifyWithWalletHandle() failed");
    XCTAssertTrue(verified, @"verifying failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}


- (void)testVerifyWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    
    // 3. Verify
    BOOL verified = NO;
    NSData * message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:invalidWalletHandle
                                                    poolHandle:poolHandle
                                                           did:@"did"
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils::verifyWithWalletHandle() failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForInvalidPoolHandle
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. Verify
    BOOL verified = NO;
    NSData * message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    IndyHandle invalidPoolHandle = poolHandle + 1;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:invalidPoolHandle
                                                           did:@"did"
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"SignusUtils::verifyWithWalletHandle() failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

// MARK: - Encrypt

- (void)testEncryptWorksForPubKeyCachedInWallet
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. my did
    NSString *myDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(myDid, @"invalid did");

    
    // 3. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");
    
    // 5. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}",theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 6. encrypt
    NSData *encryptedMessage;
    NSData *nonce;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] encryptWithWalletHandle:walletHandle
                                                     poolHandle:poolHandle
                                                          myDid:myDid
                                                            did:theirDid
                                                        message:message
                                            outEncryptedMessage:&encryptedMessage
                                                       outNonce:&nonce];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::encryptWithWalletHandle() failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testEncryptWorksForGetPubKeyFromLedger
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. trustee did
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(trusteeDid, @"invalid did");
    
    
    // 3. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for their did");
    
    
    // 4. Build & Submit nym request
    
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:theirDid
                                                                 verkey:theirVerkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:nil];
    // 5. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\"}",theirDid];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 6. encrypt
    NSData *encryptedMessage;
    NSData *nonce;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] encryptWithWalletHandle:walletHandle
                                                     poolHandle:poolHandle
                                                          myDid:trusteeDid
                                                            did:theirDid
                                                        message:message
                                            outEncryptedMessage:&encryptedMessage
                                                       outNonce:&nonce];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::encryptWithWalletHandle() failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testEncryptWorksForGetNymFromLedger
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. trustee did
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(trusteeDid, @"invalid did");
    
    
    // 4. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for their did");
    
    // 4. Build & Submit nym request
    
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:theirDid
                                                                 verkey:theirVerkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:nil];
    
    // 5. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}",theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 6. encrypt
    NSData *encryptedMessage;
    NSData *nonce;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] encryptWithWalletHandle:walletHandle
                                                     poolHandle:poolHandle
                                                          myDid:trusteeDid
                                                            did:theirDid
                                                        message:message
                                            outEncryptedMessage:&encryptedMessage
                                                       outNonce:&nonce];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::encryptWithWalletHandle() failed");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testEncryptWorksForExriredNym
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    NSString *walletName = @"wallet1";
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
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
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:config
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:openWalletWithName failed");
    
    // 4. trustee did
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(trusteeDid, @"invalid did");
    
    
    // 5. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for their did");

    // 6. Build & Submit nym request
    
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:theirDid
                                                                 verkey:theirVerkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:nil];
    
    // 7. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}",theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 8. encrypt
    NSData *encryptedMessage;
    NSData *nonce;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] encryptWithWalletHandle:walletHandle
                                                     poolHandle:poolHandle
                                                          myDid:trusteeDid
                                                            did:theirDid
                                                        message:message
                                            outEncryptedMessage:&encryptedMessage
                                                       outNonce:&nonce];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::encryptWithWalletHandle() failed");

    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testEncryptWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. my did
    NSString *myDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(myDid, @"invalid did");
    
    
    // 4. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");

    // 5. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}",theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 6. encrypt
    IndyHandle invalidPoolHandle = poolHandle + 1;
    NSData *encryptedMessage;
    NSData *nonce;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] encryptWithWalletHandle:walletHandle
                                                     poolHandle:invalidPoolHandle
                                                          myDid:myDid
                                                            did:theirDid
                                                        message:message
                                            outEncryptedMessage:&encryptedMessage
                                                       outNonce:&nonce];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"SignusUtils::encryptWithWalletHandle() returned wrong code");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

// MARK: - Decrypt

- (void)testDecryptWorks
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. my did
    NSString *myDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(myDid, @"invalid did");
    
    
    // 3. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");

    // 4. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}",theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 5. decrypt
    
    NSData *decryptedMessage;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] decryptWithWalletHandle:walletHandle
                                                          myDid:myDid
                                                            did:theirDid
                                               encryptedMessage:[SignusUtils encryptedMessage]
                                                          nonce:[SignusUtils nonce]
                                            outDecryptedMessage:&decryptedMessage];
    XCTAssertTrue([decryptedMessage isEqualToData:message], @"SignusUtils::decryptWithWalletHandle() failed. Decrypted mesage doesn't match message");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testDecryptWorksForOtherCoder
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    
    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. my did
    NSString *myDid;
    NSString *myVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&myDid
                                                                outMyVerkey:&myVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(myDid, @"invalid did");
    
    
    // 4. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");
    
    // 5. store myDid
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", myDid, myVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 6. store theirDid
    identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 7. encrypt
    NSData *encryptedMessage;
    NSData *nonce;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] encryptWithWalletHandle:walletHandle
                                                     poolHandle:poolHandle
                                                          myDid:myDid
                                                            did:myDid
                                                        message:message
                                            outEncryptedMessage:&encryptedMessage
                                                       outNonce:&nonce];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::encryptWithWalletHandle() failed");

    // 8. decrypt
    
    NSData *decryptedMessage;
    ret = [[SignusUtils sharedInstance] decryptWithWalletHandle:walletHandle
                                                          myDid:myDid
                                                            did:theirDid
                                               encryptedMessage:[SignusUtils encryptedMessage]
                                                          nonce:[SignusUtils nonce]
                                            outDecryptedMessage:&decryptedMessage];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils::decryptWithWalletHandle() returned wrong error code.");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testDecryptWorksForNonceNotCorrespondMessage
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. my did
    NSString *myDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(myDid, @"invalid did");
    
    
    // 4. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");
    
    // 5. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed for theirDid");
    
    // 6. decrypt
    
    NSString *nonce = @"acS2SQgDdfE3Goxa1AhcWCa4kEMqSelv7";
    ret = [[SignusUtils sharedInstance] decryptWithWalletHandle:walletHandle
                                                          myDid:myDid
                                                            did:theirDid
                                               encryptedMessage:[SignusUtils encryptedMessage]
                                                          nonce:[nonce dataUsingEncoding:NSUTF8StringEncoding]
                                            outDecryptedMessage:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils::decryptWithWalletHandle() returned wrong error code.");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testDecryptWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    
    NSError *ret = nil;
    NSString *poolName = [SignusUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 2. my did
    NSString *myDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(myDid, @"invalid did");
    
    
    // 4. their did
    NSString *theirDid;
    NSString *theirVerkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");

    // 5. store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", theirDid, theirVerkey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed for theirDid");
    
    // 6. decrypt
    
    IndyHandle invalidWalletHandle = walletHandle + 1;
    NSData *decryptedMessage;
    ret = [[SignusUtils sharedInstance] decryptWithWalletHandle:invalidWalletHandle
                                                          myDid:myDid
                                                            did:theirDid
                                               encryptedMessage:[SignusUtils encryptedMessage]
                                                          nonce:[SignusUtils nonce]
                                            outDecryptedMessage:&decryptedMessage];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"SignusUtils::decryptWithWalletHandle() returned wrong error code.");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}


@end
