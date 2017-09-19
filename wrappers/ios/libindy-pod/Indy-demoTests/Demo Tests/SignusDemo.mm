//
//  SignusDemo.m
//  Indy-demo
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <Indy/Indy.h>
#import "WalletUtils.h"
#import "SignusUtils.h"

@interface SignusDemo : XCTestCase

@end

@implementation SignusDemo

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

- (void)testSignusDemo
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
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block NSString *myDid = nil;
    __block NSString *myVerkey = nil;
    __block NSString *myPk = nil;
    ret = [IndySignus createAndStoreMyDidWithWalletHandle:  myWalletHandle
                                                    didJSON:  myDidJson
                                                 completion: ^(NSError *error, NSString *did, NSString *verkey, NSString *pk)
    {
        XCTAssertEqual(error.code, Success, "createAndStoreMyDid() got error in completion");
        NSLog(@"myDid:");
        NSLog(@"did = %@", did);
        NSLog(@"verkey = %@", verkey);
        NSLog(@"pk = %@", pk);
        myDid = did;
        myVerkey = verkey;
        myPk = pk;
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 6. Create Their DID

    NSString *theirDidJson = @"{}";
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    __block NSString *theirDid = nil;
    __block NSString *theirVerkey = nil;
    __block NSString *theirPk = nil;
    
    ret = [IndySignus createAndStoreMyDidWithWalletHandle:  theirWalletHandle
                                                    didJSON:  theirDidJson
                                                 completion: ^(NSError *error, NSString *did, NSString *verkey, NSString *pk)
    {
        XCTAssertEqual(error.code, Success, "createAndStoreMyDid() got error in completion");
        NSLog(@"theirDid:");
        NSLog(@"did = %@", did);
        NSLog(@"verkey = %@", verkey);
        NSLog(@"pk = %@", pk);
        theirDid = [NSString stringWithString: did];
        theirVerkey = [NSString stringWithString: verkey];
        theirPk = [NSString stringWithString: pk];
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");

    // 7. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                                                 \"pk\":\"%@\",\
                                                                 \"verkey\":\"%@\"\
                                                                }", theirDid, theirPk, theirVerkey];

    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [IndySignus storeTheirDidWithWalletHandle: myWalletHandle identityJSON: theirIdentityJson completion:^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "storeTheirDid() got error in completion");
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
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
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSData *signature = nil;
    
    ret = [IndySignus signWithWalletHandle:theirWalletHandle
                                       did:theirDid
                                   message:message
                                completion:^(NSError *error, NSData *blockSignature)
           {
               XCTAssertEqual(error.code, Success, "sign() got error in completion");
               NSLog(@"signature: %@", signature);
               signature = blockSignature;
               [completionExpectation fulfill];
           }];


    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"sign() failed!");
    
    // 9. I Verify message
    IndyHandle poolHandle = 1;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [IndySignus verifySignatureWithWalletHandle:myWalletHandle
                                           poolHandle:poolHandle
                                                  did:theirDid
                                              message:message
                                            signature:signature
                                           completion:^(NSError *error, BOOL valid)
           {
               XCTAssertEqual(error.code, Success, "verifySignature() got error in completion");
               XCTAssertEqual(YES, valid, "verifySignature() signature is not valid");
               [completionExpectation fulfill];
           }];

    // TODO: There is some error inside closure at rust level
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"verifySignature() failed!");
    
    [TestUtils cleanupStorage];
}


- (void)testSignusDemoForKeychainWallet
{
    [TestUtils cleanupStorage];
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
    NSString *poolName = [TestUtils pool];
    NSString *myWalletName = @"my_wallet5";
    NSString *theirWalletName = @"their_wallet6";
    NSString *xtype = @"keychain";
    NSError *ret = nil;
    XCTestExpectation* completionExpectation = nil;
    
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
    
    NSString *myDidJson = @"{}";
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block NSString *myDid = nil;
    __block NSString *myVerkey = nil;
    __block NSString *myPk = nil;
    ret = [IndySignus createAndStoreMyDidWithWalletHandle:  myWalletHandle
                                                  didJSON:  myDidJson
                                               completion: ^(NSError *error, NSString *did, NSString *verkey, NSString *pk)
           {
               XCTAssertEqual(error.code, Success, "createAndStoreMyDid() got error in completion");
               NSLog(@"myDid:");
               NSLog(@"did = %@", did);
               NSLog(@"verkey = %@", verkey);
               NSLog(@"pk = %@", pk);
               myDid = did;
               myVerkey = verkey;
               myPk = pk;
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 6. Create Their DID
    
    NSString *theirDidJson = @"{}";
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block NSString *theirDid = nil;
    __block NSString *theirVerkey = nil;
    __block NSString *theirPk = nil;
    
    ret = [IndySignus createAndStoreMyDidWithWalletHandle:  theirWalletHandle
                                                  didJSON:  theirDidJson
                                               completion: ^(NSError *error, NSString *did, NSString *verkey, NSString *pk)
           {
               XCTAssertEqual(error.code, Success, "createAndStoreMyDid() got error in completion");
               NSLog(@"theirDid:");
               NSLog(@"did = %@", did);
               NSLog(@"verkey = %@", verkey);
               NSLog(@"pk = %@", pk);
               theirDid = [NSString stringWithString: did];
               theirVerkey = [NSString stringWithString: verkey];
               theirPk = [NSString stringWithString: pk];
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 7. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                   \"pk\":\"%@\",\
                                   \"verkey\":\"%@\"\
                                   }", theirDid, theirPk, theirVerkey];
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [IndySignus storeTheirDidWithWalletHandle: myWalletHandle identityJSON: theirIdentityJson completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "storeTheirDid() got error in completion");
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
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
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSData *signature = nil;
    
    ret = [IndySignus signWithWalletHandle:theirWalletHandle
                                       did:theirDid
                                   message:message
                                completion:^(NSError *error, NSData *blockSignature)
           {
               XCTAssertEqual(error.code, Success, "sign() got error in completion");
               NSLog(@"signature: %@", signature);
               signature = blockSignature;
               [completionExpectation fulfill];
           }];
    
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"sign() failed!");
    
    // 9. I Verify message
    IndyHandle poolHandle = 1;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [IndySignus verifySignatureWithWalletHandle:myWalletHandle
                                           poolHandle:poolHandle
                                                  did:theirDid
                                              message:message
                                            signature:signature
                                           completion:^(NSError *error, BOOL valid)
           {
               XCTAssertEqual(error.code, Success, "verifySignature() got error in completion");
               XCTAssertEqual(YES, valid, "verifySignature() signature is not valid");
               [completionExpectation fulfill];
           }];
    
    // TODO: There is some error inside closure at rust level
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"verifySignature() failed!");
    
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}


@end
