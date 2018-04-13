//
//  DidMediumCases.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 14.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface DidMediumCases : XCTestCase

@end

@implementation DidMediumCases

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
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"crypto_type\":\"type\"}";
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, UnknownCryptoTypeError, @"DidUtils::createMyDidWithWalletHandle() returned wrong error");
  
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForInvalidSeed
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"seed\":\"seed\"}";
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"DidUtils::createMyDidWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForInvalidDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"did\":\"invalid_base_58_did\"}";
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"DidUtils::createMyDidWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}

- (void)testCreateMyDidWorksForInvalidJson
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    NSString *didJson = @"{\"seed\":\"123\"}";
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:didJson
                                                           outMyDid:nil
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"DidUtils::createMyDidWithWalletHandle() returned wrong error");
    
    [TestUtils cleanupStorage];
}


- (void)testCreateMyDidWorksForDuplicate
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];

    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 2. create my did
    NSString *myDid = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. create duplicate did
    NSString *didJson = [NSString stringWithFormat:@"{\"did\":\"%@\"}", myDid];
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:didJson
                                                        outMyDid:nil
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, DidAlreadyExistsError, @"DidUtils::createMyDidWithWalletHandle() returned wrong error");

    [TestUtils cleanupStorage];
}

// MARK: - Replace keys Start

-(void)testReplaceKeysStartWorksForNotExistingDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. replace keys start
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:@"UnknonwDid11111111111"
                                                  identityJson:@"{}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:nil];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"DidUtils:replaceKeysStartForDid returned wrong code.");
    
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysStartWorksForCorectCryptoType
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];

    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
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
    XCTAssertEqual(ret.code, Success, @"DidUtils:createMyDidWithWalletHandle failed");
    
    // 3. replace keys start
    
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{\"crypto_type\":\"ed25519\"}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils:replaceKeysStartForDid failed");
    XCTAssertFalse([myVerkey isEqualToString:newVerkey], @"myVerkey is equal to newVerkey");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysWorksForInvalidCryptoType
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. create my did
    
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils:createMyDidWithWalletHandle failed");
    
    // 3. replace keys start
    
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{\"crypto_type\":\"type\"}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:nil];
    XCTAssertEqual(ret.code, UnknownCryptoTypeError, @"DidUtils:replaceKeysStartForDid returned wrong error code");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];

}

// MARK: - Store their did

- (void)testStoreTheirDidWorksForInvalidDid
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"invalid_base58_string\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"DidUtils:storeTheirDidWithWalletHandle returned wrong code");
    
    [TestUtils cleanupStorage];
}

- (void)testStoreTheirDidWorksForInvalidVerkey
{
    [TestUtils cleanupStorage];
    NSError *ret = nil;
    NSString *poolName = [TestUtils pool];
    
    // 1. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 2. Store their did
    NSString *identityJson = @"{\"did\":\"did\", \"verkey\":\"invalid_base58\"}";
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:walletHandle
                                                         identityJson:identityJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"DidUtils:storeTheirDidWithWalletHandle returned wrong code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Replace keys demo

- (void)testReplaceKeysDemo
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    
    // 1. create and open pool
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Generate did from trustee seed
    
    NSString *trusteeDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed fot trusteeDid");
    
    // 4. Generate my did
    
    NSString *myDid;
    NSString *myVerkey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed fot myDid");
    
    // 5. Send nym request to ledger
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid()");
    
    NSString *nymResponce;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponce];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle()");
    
    // 6. start replacing of keys
    
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysStartForDid() failed");
    
    // 7. Send nym request to ledger with new verkey
    
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:myDid
                                                              targetDid:myDid
                                                                 verkey:newVerkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponce];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    
    // 8. Send schema request before applying replacing of keys

    NSString *schemaData = @"{\"id\":\"id\", \"name\":\"name\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";
    NSString *schemaRequest;
    
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaData
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");

    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not Success");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:schemaResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    // 9. Apply replacing of keys
    ret = [[DidUtils sharedInstance] replaceKeysApplyForDid:myDid
                                                  walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysApplyForDid() failed");
    
    // 10. Send schema request.
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:nil];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testReplaceKeysWithoutNymTransaction
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    
    // 1. Create and open pool
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. create and open wallet
    
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Generate did from trustee seed
    
    NSString *trusteeDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDidWithWalletHandle() failed fot trusteeDid");
    
    // 4. Generate my did
    
    NSString *myDid;
    NSString *myVerkey;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed fot myDid");
    
    // 5. Send nym request to ledger
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerkey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid()");
    
    NSString *nymResponce;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponce];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle()");
    
    // 6. start replacing of keys
    
    NSString *newVerkey;
    ret = [[DidUtils sharedInstance] replaceKeysStartForDid:myDid
                                                  identityJson:@"{}"
                                                  walletHandle:walletHandle
                                                   outMyVerKey:&newVerkey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysStartForDid() failed");
    
    // 7. Apply replacing of keys
    ret = [[DidUtils sharedInstance] replaceKeysApplyForDid:myDid
                                                  walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysApplyForDid() failed");
    
    // 8. Send schema request before applying replacing of keys

    NSString *schemaData = @"{\"id\":\"id\", \"name\":\"name\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";
    NSString *schemaRequest;
    
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaData
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");
    
    
    // 10. Send schema request.
    NSString *schemaResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not Success");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:schemaResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

@end
