//
//  CryptoDemo.m
//  Indy-demo
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface CryptoDemo : XCTestCase

@end

@implementation CryptoDemo

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

- (void)testCryptoDemo
{
    [TestUtils cleanupStorage];

    NSString *poolName = [TestUtils pool];
    NSString *myWalletName = @"my_wallet4";
    NSString *theirWalletName = @"their_wallet5";
    NSString *xtype = @"default";
    NSError *ret = nil;
    XCTestExpectation* completionExpectation = nil;
    
    IndyHandle myWalletHandle = 0;
    IndyHandle theirWalletHandle = 0;

    //TODO CREATE ISSUER, PROVER, VERIFIER WALLETS
    //1. Create my wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:myWalletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed for my wallet!");
    
    // 2. Open my wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:myWalletName
                                                    config:nil
                                                 outHandle:&myWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed for my wallet!");

    // 3. Create Their Wallet
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:theirWalletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed for their wallet!");
    
    // 4. Open their wallet
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:theirWalletName
                                                     config:nil
                                                 outHandle:&theirWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed for their wallet!");
    
    // 5. Create My DID
    
    NSString *myDidJson = @"{}";
    
    NSString *myDid = nil;
    NSString *myVerkey = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:myWalletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 6. Create Their DID

    NSString *theirDidJson = @"{}";

    NSString *theirDid = nil;
    NSString *theirVerkey = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:theirWalletHandle
                                                          myDidJson:theirDidJson
                                                           outMyDid:&theirDid
                                                        outMyVerkey:&theirVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");

    // 7. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                                                 \"verkey\":\"%@\"\
                                                                }", theirDid, theirVerkey];


    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:myWalletHandle
                                                         identityJson:theirIdentityJson];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 8. Their Sign message
    
    NSString* messageJson = @"{"\
                         "  \"reqId\":1495034346617224651,"
                         "  \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\","
                         "  \"operation\":{"
                         "        \"type\":\"1\","
                         "        \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"}"
                         "}";
    NSData *message = [messageJson dataUsingEncoding:NSUTF8StringEncoding];
    
    NSData *signature = nil;
    
    ret = [[CryptoUtils sharedInstance] signMessage:message key:theirVerkey walletHandle:theirWalletHandle outSignature:&signature];
    XCTAssertEqual(ret.code, Success, @"sign() failed!");
    
    // 9. I Verify message
    IndyHandle poolHandle = 1;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    BOOL verified = false;
    ret = [[CryptoUtils sharedInstance] verifySignature:signature forMessage:message key:theirVerkey outIsValid:&verified];

    XCTAssertEqual(ret.code, Success, @"verifySignature() failed!");
    XCTAssertEqual(YES, verified, "verifySignature() signature is not valid");
    
    [TestUtils cleanupStorage];
}


- (void)testCryptoDemoForKeychainWallet
{
    [TestUtils cleanupStorage];
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
    NSString *poolName = [TestUtils pool];
    NSString *myWalletName = @"my_wallet5";
    NSString *theirWalletName = @"their_wallet6";
    NSString *xtype = @"keychain";
    NSError *ret = nil;
    
    IndyHandle myWalletHandle = 0;
    IndyHandle theirWalletHandle = 0;
    
    //TODO CREATE ISSUER, PROVER, VERIFIER WALLETS
    
    // 0. Register wallet type
    
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    
    //1. Create my wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:myWalletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed for my wallet!");
    
    // 2. Open my wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:myWalletName
                                                    config:nil
                                                 outHandle:&myWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed for my wallet!");
    
    // 3. Create Their Wallet
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:theirWalletName
                                                           xtype:xtype
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed for their wallet!");
    
    // 4. Open their wallet
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:theirWalletName
                                                    config:nil
                                                 outHandle:&theirWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed for their wallet!");
    
    // 5. Create My DID
    
    NSString *myDid = nil;
    NSString *myVerkey = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:myWalletHandle
                                                                       seed:nil
                                                                   outMyDid:&myDid
                                                                outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 6. Create Their DID
    
    NSString *theirDid = nil;
    NSString *theirVerkey = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:theirWalletHandle
                                                                       seed:nil
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 7. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                   \"verkey\":\"%@\"\
                                   }", theirDid, theirVerkey];
    
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:myWalletHandle
                                                         identityJson:theirIdentityJson];
    XCTAssertEqual(ret.code, Success, @"IndyDid::storeTheirDid() failed!");
    
    // 8. Their Sign message
    
    NSString* messageJson = @"{"\
    "  \"reqId\":1495034346617224651,"
    "  \"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\","
    "  \"operation\":{"
    "        \"type\":\"1\","
    "        \"dest\":\"4efZu2SXufS556yss7W5k6Po37jt4371RM4whbPKBKdB\"}"
    "}";
    NSData *message = [messageJson dataUsingEncoding:NSUTF8StringEncoding];
    
    NSData *signature = nil;
    ret = [[CryptoUtils sharedInstance] signMessage:message key:theirVerkey walletHandle:theirWalletHandle outSignature:&signature];

    XCTAssertEqual(ret.code, Success, @"sign() failed!");
    
    // 9. I Verify message
    IndyHandle poolHandle = 1;
    
    BOOL verified = false;
    ret = [[CryptoUtils sharedInstance] verifySignature:signature forMessage:message key:theirVerkey outIsValid:&verified];
    XCTAssertEqual(ret.code, Success, @"verifySignature() failed!");
    XCTAssertEqual(YES, verified, "verifySignature() signature is not valid");
    
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}


@end
