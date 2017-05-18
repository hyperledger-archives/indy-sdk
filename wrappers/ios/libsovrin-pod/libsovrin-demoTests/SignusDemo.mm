//
//  SignusDemo.m
//  libsovrin-demo
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libsovrin/libsovrin.h>

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

- (void)testSignus
{
    [TestUtils cleanupStorage];

    NSString *poolName = @"pool1";
    NSString *myWalletName = @"my_wallet";
    NSString *theirWalletName = @"their_wallet";
    NSString *xtype = @"default";
    NSError *ret = nil;

    __block SovrinHandle myWalletHandle;
    __block SovrinHandle theirWalletHandle;
    

    //TODO CREATE ISSUER, PROVER, VERIFIER WALLETS
    //1. Create My Wallet
    
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [[SovrinWallet sharedInstance] createWallet:  poolName
                                                 name:  myWalletName
                                                xType:  xtype
                                               config:  nil
                                          credentials:  nil
                                           completion: ^(NSError* error)
    {
        XCTAssertEqual(error.code, Success, "createWallet got error in completion");
        [completionExpectation fulfill];        
    }];

    NSAssert( ret.code == Success, @"createWallet() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

     //2. Create Their Wallet

    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] createWallet:  poolName
                                                 name:  theirWalletName
                                                xType:  xtype
                                               config:  nil
                                          credentials:  nil
                                           completion: ^(NSError* error)
    {
        XCTAssertEqual(error.code, Success, "createWallet got error in completion");
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"createWallet() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    //3. Open My Wallet. Gets My wallet handle
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] openWallet: myWalletName
                                      runtimeConfig: nil
                                        credentials: nil
                                         completion: ^(NSError* error, SovrinHandle walletHandle)

    {
        XCTAssertEqual(error.code, Success, "openWallet got error in completion");
        myWalletHandle = walletHandle;
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"openWallet() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    //4. Open Their Wallet. Gets Their wallet handle
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] openWallet: theirWalletName
                                      runtimeConfig: nil
                                        credentials: nil
                                         completion: ^(NSError* error, SovrinHandle walletHandle)
           
    {
        XCTAssertEqual(error.code, Success, "openWallet got error in completion");
        theirWalletHandle = walletHandle;
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"openWallet() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 5. Create My DID
    
    NSString *myDidJson = @"{}";
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [SovrinSignus createAndStoreMyDid:  myWalletHandle
                                    didJSON:  myDidJson
                                 completion: ^(NSError *error, NSString *did, NSString *verkey, NSString *pk)
    {
        XCTAssertEqual(error.code, Success, "createAndStoreMyDid() got error in completion");
        NSLog(@"myDid:");
        NSLog(@"did = %@", did);
        NSLog(@"verkey = %@", verkey);
        NSLog(@"pk = %@", pk);
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"createAndStoreMyDid() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 6. Create Their DID

    NSString *theirDidJson = @"{}";
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    __block NSString *theirDid = nil;
    __block NSString *theirVerkey = nil;
    __block NSString *theirPk = nil;
    
    ret = [SovrinSignus createAndStoreMyDid:  theirWalletHandle
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
    
    NSAssert( ret.code == Success, @"createAndStoreMyDid() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    // 7. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                                                 \"pk\":\"%@\",\
                                                                 \"verkey\":\"%@\"\
                                                                }", theirDid, theirPk, theirVerkey];

    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [SovrinSignus storeTheirDid: myWalletHandle identityJSON: theirIdentityJson completion:^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "storeTheirDid() got error in completion");
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"createAndStoreMyDid() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 8. Their Sign message
    
    NSString* message = @"test message";
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    __block NSString *theirSignature = nil;
    
    ret = [SovrinSignus sign:  theirWalletHandle
                         did:  theirDid
                         msg:  message
                  completion: ^(NSError *error, NSString *signature)
    {
        XCTAssertEqual(error.code, Success, "sign() got error in completion");
        NSLog(@"signature: %@", signature);
        theirSignature = [NSString stringWithString: signature];
        [completionExpectation fulfill];
    }];

    NSAssert( ret.code == Success, @"sign() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 9. I Verify message
    SovrinHandle poolHandle = 1;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [SovrinSignus verifySignature:  myWalletHandle
                                   pool:  poolHandle
                                    did:  theirDid
                                    msg:  message
                              signature:  theirSignature
                             completion: ^(NSError *error, BOOL valid)
    {
        XCTAssertEqual(error.code, Success, "verifySignature() got error in completion");
        XCTAssertEqual(YES, valid, "verifySignature() signature is not valid");
        [completionExpectation fulfill];
    }];

    NSAssert( ret.code == Success, @"verifySignature() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
}

@end
