//
//  LedgerDemo.m
//  Indy-demo
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerDemo : XCTestCase

@end

@implementation LedgerDemo

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void) testLedgerDemo
{
    [TestUtils cleanupStorage];
    NSString *myWalletName = @"my_wallet2";
    NSString *theirWalletName = @"their_wallet3";
    NSString *walletType = @"default";
    NSString *poolName = @"ledger_demo_works";
    XCTestExpectation *completionExpectation;
    NSError *ret;
    
    // 1. Create ledger config from genesis txn file
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");
    
    ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                              poolConfig:poolConfig];
    
    // 2. Open pool ledger
    __block IndyHandle poolHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndyPool openPoolLedgerWithName:poolName
                                  poolConfig:nil
                                  completion:^(NSError *error, IndyHandle h)
            {
                XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
                poolHandle = h;
                [completionExpectation fulfill];
            }];
    XCTAssertEqual(ret.code, Success, @"openPoolLedgerWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 3. Create my wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] createWalletWithPoolName:poolName
                                                           name:myWalletName
                                                          xType:walletType
                                                         config:nil
                                                    credentials:nil
                                                     completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "createWalletWithPoolName got error in completion");
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 4. Open My Wallet. Gets My wallet handle
    __block IndyHandle myWalletHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] openWalletWithName:myWalletName
                                            runtimeConfig:nil
                                              credentials:nil
                                               completion:^(NSError *error, IndyHandle h)
            {
                XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
                myWalletHandle = h;
                [completionExpectation fulfill];
            }];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 5. Create their wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] createWalletWithPoolName:poolName
                                                           name:theirWalletName
                                                          xType:walletType
                                                         config:nil
                                                    credentials:nil
                                                     completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "createWalletWithPoolName got error in completion");
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 6. Open Their Wallet. Gets Their wallet handle
    __block IndyHandle theirWalletHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] openWalletWithName:theirWalletName
                                            runtimeConfig:nil
                                              credentials:nil
                                               completion:^(NSError *error, IndyHandle h)
           {
               XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
               theirWalletHandle = h;
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 7. Create my did
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block NSString *myDid = nil;
    __block NSString *myVerkey = nil;
    __block NSString *myPk = nil;
    ret = [IndySignus createAndStoreMyDidWithWalletHandle:myWalletHandle
                                                  didJSON:@"{}"
                                               completion:^(NSError *error, NSString *did, NSString *verkey, NSString *pk)
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
    
    // 8. Create Their DID from Trustee1 seed
    NSString *theirDidJson = @"{\"seed\":\"000000000000000000000000Trustee1\"}";
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
    
    // 9. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                                                \"pk\":\"%@\",\
                                                                \"verkey\":\"%@\"\
                                   }", theirDid, theirPk, theirVerkey];
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndySignus storeTheirDidWithWalletHandle: myWalletHandle
                                         identityJSON: theirIdentityJson
                                           completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "storeTheirDid() got error in completion");
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 10. Prepare NYM transaction
    // removing signature field does not help
    NSNumber *nymReqId = [[PoolUtils sharedInstance] getRequestId];
    NSString *nymTxnRequest = [NSString stringWithFormat:@"{"\
                               "\"identifier\":\"%@\","\
                               "\"operation\":{"\
                                    "\"dest\":\"%@\","\
                                    "\"type\":\"1\"},"\
                               "\"reqId\":%d"
                               "}", theirDid, myDid, [nymReqId intValue]];
    
    // 11. Send NYM request with signing
    __block NSString *nymTxnResponse;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndyLedger signAndSubmitRequestWithWalletHandle:theirWalletHandle
                                                poolHandle:poolHandle
                                              submitterDID:theirDid
                                               requestJSON:nymTxnRequest
                                                completion:^(NSError *error, NSString *requestResult)
            {
                XCTAssertEqual(error.code, Success, "signAndSubmitRequestWithWalletHandle() got error in completion");
                nymTxnResponse = requestResult;
                [completionExpectation fulfill];
            }];
  //  XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithWalletHandle() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 12. Prepare and send GET_NYM request
    NSNumber *getNymRequestId = [[PoolUtils sharedInstance] getRequestId];
    NSString *getNymTxnRequest = [NSString stringWithFormat:@"{"\
                                  "\"reqId\":%d,"\
                                  "\"identifier\":\"%@\","\
                                  "\"operation\":{"\
                                        "\"type\":\"105\","\
                                        "\"dest\":\"%@\"}"\
                                  "}", [getNymRequestId intValue] , myVerkey, myDid];
    
    __block NSString *getNymTxnResponseJson;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndyLedger submitRequestWithPoolHandle:poolHandle
                                        requestJSON:getNymTxnRequest
                                         completion:^(NSError *error, NSString *requestResult)
           {
               XCTAssertEqual(error.code, Success, "submitRequestWithPoolHandle() got error in completion");
               getNymTxnResponseJson = requestResult;
               [completionExpectation fulfill];
           }];
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"submitRequestWithPoolHandle() failed!");
    
    NSDictionary *getNymTxnResponse = [NSDictionary fromString:getNymTxnResponseJson];
    NSString *dataStr = getNymTxnResponse[@"result"][@"data"];
    NSDictionary *data = [NSDictionary fromString:dataStr];
    XCTAssertNotNil(data[@"dest"], @"data[dest] is nil");
    XCTAssertTrue([data[@"dest"] isEqualToString:myDid], @"wrong dest!");
    
    [TestUtils cleanupStorage];
}

- (void) testLedgerDemoForKeychainWallet
{
    [TestUtils cleanupStorage];
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    NSString *myWalletName = @"ledger_my_wallet2";
    NSString *theirWalletName = @"ledger_their_wallet3";
    NSString *walletType = @"keychain";
    NSString *poolName = @"ledger_demo_works_for_keychain_wallet";
    XCTestExpectation *completionExpectation;
    NSError *ret;
    
    // 0. register wallet type
    
    ret = [[WalletUtils sharedInstance] registerWalletType:walletType];
    
    // 1. Create ledger config from genesis txn file
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:poolName
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    
    ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                              poolConfig:poolConfig];
    
    // 2. Open pool ledger
    __block IndyHandle poolHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndyPool openPoolLedgerWithName:poolName
                                poolConfig:nil
                                completion:^(NSError *error, IndyHandle h)
           {
               XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
               poolHandle = h;
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"openPoolLedgerWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 3. Create my wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] createWalletWithPoolName:poolName
                                                           name:myWalletName
                                                          xType:walletType
                                                         config:nil
                                                    credentials:nil
                                                     completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "createWalletWithPoolName got error in completion");
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 4. Open My Wallet. Gets My wallet handle
    __block IndyHandle myWalletHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] openWalletWithName:myWalletName
                                            runtimeConfig:nil
                                              credentials:nil
                                               completion:^(NSError *error, IndyHandle h)
           {
               XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
               myWalletHandle = h;
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 5. Create their wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] createWalletWithPoolName:poolName
                                                           name:theirWalletName
                                                          xType:walletType
                                                         config:nil
                                                    credentials:nil
                                                     completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "createWalletWithPoolName got error in completion");
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 6. Open Their Wallet. Gets Their wallet handle
    __block IndyHandle theirWalletHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[IndyWallet sharedInstance] openWalletWithName:theirWalletName
                                            runtimeConfig:nil
                                              credentials:nil
                                               completion:^(NSError *error, IndyHandle h)
           {
               XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
               theirWalletHandle = h;
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 7. Create my did
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block NSString *myDid = nil;
    __block NSString *myVerkey = nil;
    __block NSString *myPk = nil;
    ret = [IndySignus createAndStoreMyDidWithWalletHandle:myWalletHandle
                                                  didJSON:@"{}"
                                               completion:^(NSError *error, NSString *did, NSString *verkey, NSString *pk)
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
    
    // 8. Create Their DID from Trustee1 seed
    NSString *theirDidJson = @"{\"seed\":\"000000000000000000000000Trustee1\"}";
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
    
    // 9. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                   \"pk\":\"%@\",\
                                   \"verkey\":\"%@\"\
                                   }", theirDid, theirPk, theirVerkey];
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndySignus storeTheirDidWithWalletHandle: myWalletHandle
                                       identityJSON: theirIdentityJson
                                         completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "storeTheirDid() got error in completion");
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 10. Prepare NYM transaction
    // removing signature field does not help
    NSNumber *nymReqId = [[PoolUtils sharedInstance] getRequestId];
    NSString *nymTxnRequest = [NSString stringWithFormat:@"{"\
                               "\"identifier\":\"%@\","\
                               "\"operation\":{"\
                               "\"dest\":\"%@\","\
                               "\"type\":\"1\"},"\
                               "\"reqId\":%d"
                               "}", theirDid, myDid, [nymReqId intValue]];
    
    // 11. Send NYM request with signing
    __block NSString *nymTxnResponse;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndyLedger signAndSubmitRequestWithWalletHandle:theirWalletHandle
                                                poolHandle:poolHandle
                                              submitterDID:theirDid
                                               requestJSON:nymTxnRequest
                                                completion:^(NSError *error, NSString *requestResult)
           {
               XCTAssertEqual(error.code, Success, "signAndSubmitRequestWithWalletHandle() got error in completion");
               nymTxnResponse = requestResult;
               [completionExpectation fulfill];
           }];
    //  XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithWalletHandle() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 12. Prepare and send GET_NYM request
    NSNumber *getNymRequestId = [[PoolUtils sharedInstance] getRequestId];
    NSString *getNymTxnRequest = [NSString stringWithFormat:@"{"\
                                  "\"reqId\":%d,"\
                                  "\"identifier\":\"%@\","\
                                  "\"operation\":{"\
                                  "\"type\":\"105\","\
                                  "\"dest\":\"%@\"}"\
                                  "}", [getNymRequestId intValue] , myVerkey, myDid];
    
    __block NSString *getNymTxnResponseJson;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [IndyLedger submitRequestWithPoolHandle:poolHandle
                                      requestJSON:getNymTxnRequest
                                       completion:^(NSError *error, NSString *requestResult)
           {
               XCTAssertEqual(error.code, Success, "submitRequestWithPoolHandle() got error in completion");
               getNymTxnResponseJson = requestResult;
               [completionExpectation fulfill];
           }];
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"submitRequestWithPoolHandle() failed!");
    
    NSDictionary *getNymTxnResponse = [NSDictionary fromString:getNymTxnResponseJson];
    NSString *dataStr = getNymTxnResponse[@"result"][@"data"];
    NSDictionary *data = [NSDictionary fromString:dataStr];
    XCTAssertNotNil(data[@"dest"], @"data[dest] is nil");
    XCTAssertTrue([data[@"dest"] isEqualToString:myDid], @"wrong dest!");
    
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}

-(BOOL) validate:(NSString*) key d1: (NSDictionary*) d1 d2: (NSDictionary*) d2
{
    id obj1 = [ d1 objectForKey: key];
    id obj2 = [ d2 objectForKey: key];
    return [ obj1 isEqual: obj2];
}

@end
