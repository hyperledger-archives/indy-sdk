//
//  SignusMediumCases.m
//  libindy-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libindy/libindy.h>
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "LedgerUtils.h"
#import "NSDictionary+JSON.h"
#import "NSString+Validation.h"

@interface SignusMediumCases : XCTestCase

@end

@implementation SignusMediumCases

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

- (void)testCreateMyDidWorksForInvalidCryptoType
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"crypto_type\":\"type\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, SignusUnknownCryptoError, @"SignusUtils::createMyDidWithWalletHandle() returned wrong error");
  
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForInvalidSeed
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"seed\":\"seed\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils::createMyDidWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForInvalidDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"did\":\"invalid_base_58_did\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils::createMyDidWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForInvalidJson
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"seed\":\"123\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils::createMyDidWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}

// MARK: - Replace keys

-(void)testReplaceKeysWorksForNotExistingDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    //TODO: may be we must return WalletNotFound in case if key not exists in wallet
    
    // 2. replace keys
    ret = [[SignusUtils sharedInstance] replaceKeysWithWalletHandle:walletHandle
                                                                did:@"8wZcEriaNLNKtteJvx7f8i"
                                                       identityJson:@"{}"
                                                        outMyVerKey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils:replaceKeysWithWalletHandle failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Store their did

- (void)testStoreTheirDidWorksForInvalidCryptoType
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"8wZcEriaNLNKtteJvx7f8i\", \"crypto_type\":\"type\"}";
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, SignusUnknownCryptoError, @"SignusUtils:storeTheirDidWithWalletHandle failed");
    
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksForInvalidDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"invalid_base58_string\"}";
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils:storeTheirDidWithWalletHandle returned wrong code");
    
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksForInvalidVerkey
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = @"pool1";
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"did\", \"verkey\":\"verkey\"}";
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils:storeTheirDidWithWalletHandle returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Sign

- (void)testSignWorksForInvalidMessage
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *myDid;
    NSString *myDidJson = @"{}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    // 3. Sign
    NSString *message = @"1495034346617224651";
    
    ret = [[SignusUtils sharedInstance] signWithWalletHandle:walletHandle
                                                    theirDid:myDid message:message
                                                outSignature:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils::signWithWalletHandle() returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Verify

- (void)testVerifyWorksForInvalidSignatureLen
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
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for did");
    
    // 4. Store did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\", \"verkey\":\"%@\"}", did, verKey];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 5. Verify
    const unsigned char bytes[] = {187, 227, 10, 29, 46, 178, 12, 179, 197, 69, 171, 70, 228, 204, 52, 22, 199, 54, 62, 13, 115, 5, 216, 66, 20, 131, 121, 29, 251, 224, 253, 201, 75, 73, 225, 237, 219, 133, 35, 217, 131, 135, 232, 129, 32};
    NSData *signature = [NSData dataWithBytes:bytes length:sizeof(bytes)];
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    BOOL verified = NO;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:did
                                                       message:message
                                                     signature:signature
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"SignusUtils::verifyWithWalletHandle() returned wrong code");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForGetNymFromLedgerWithIncompatibleWallet
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
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"other_pool_name"
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. create did
    NSString *mydid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&mydid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. Store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\"}", mydid];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");

    // 5. verify
    
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:mydid
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:nil];
    XCTAssertEqual(ret.code, WalletIncompatiblePoolError, @"SignusUtils::verifyWithWalletHandle() returned wrong error code");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}



- (void)testVerifyWorksForGetLedgerNotFoundNym
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
    
    // 3. create  did
    NSString *myDid;
    NSString *myDidJson = @"{\"seed\":\"0000000000000000000000000000Fake\"}";
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    // 4. Store their did
    NSString *identityJson = [NSString stringWithFormat:@"{\"did\":\"%@\"}", myDid];
    ret = [[SignusUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidWithWalletHandle() failed");
    
    // 5. Verify
    
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    
    BOOL verified = NO;
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:myDid
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, CommonInvalidState, @"SignusUtils::verifyWithWalletHandle() returned wrong code");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testVerifyWorksForGetNymFromLedger
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
    
    // 3. create & store trustee did
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");
    
    // 4. create my did
    NSString *myDid;
    NSString *myVerKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[SignusUtils mySeed]
                                                                   outMyDid:&myDid
                                                                outMyVerkey:&myVerKey
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for myDid");
    XCTAssertTrue(myDid, @"invalid did");
    
    // 5. Build nym request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertTrue([nymRequest isValid], @"invalid nymRequest");
    
    // 6. sign and submit nym request
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertTrue([nymResponse isValid], @"invalid nymResponse");
    
    // 7. Verify
    
    BOOL verified = NO;
    NSData *message = [[SignusUtils message] dataUsingEncoding:NSUTF8StringEncoding];
    ret = [[SignusUtils sharedInstance] verifyWithWalletHandle:walletHandle
                                                    poolHandle:poolHandle
                                                           did:myDid
                                                       message:message
                                                     signature:[SignusUtils signature]
                                                   outVerified:&verified];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::verifyWithWalletHandle() failed");
    XCTAssertTrue(verified);
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

@end
