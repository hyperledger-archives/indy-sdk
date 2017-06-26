//
//  LedgerDemo.m
//  libsovrin-demo
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libsovrin/libsovrin.h>
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
    NSString *myWalletName = @"my_wallet";
    NSString *theirWalletName = @"their_wallet";
    NSString *walletType = @"default";
    NSString *poolName = @"ledger_demo_works";
    XCTestExpectation *completionExpectation;
    NSError *ret;
    
    // 1. Create ledger config from genesis txn file
    ret = [[ PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:poolName
                                                                    nodes:nil
                                                               poolConfig:nil
                                                           genTxnFileName:nil];

    // 2. Open pool ledger
    __block SovrinHandle poolHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [SovrinPool openPoolLedgerWithName:poolName
                                  poolConfig:nil
                                  completion:^(NSError *error, SovrinHandle h)
            {
                XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
                poolHandle = h;
                [completionExpectation fulfill];
            }];
    XCTAssertEqual(ret.code, Success, @"openPoolLedgerWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 3. Create my wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[SovrinWallet sharedInstance] createWalletWithPoolName:poolName
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
    __block SovrinHandle myWalletHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[SovrinWallet sharedInstance] openWalletWithName:myWalletName
                                              runtimeConfig:nil
                                                credentials:nil
                                                 completion:^(NSError *error, SovrinHandle h)
            {
                XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
                myWalletHandle = h;
                [completionExpectation fulfill];
            }];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 5. Create their wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[SovrinWallet sharedInstance] createWalletWithPoolName:poolName
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
    __block SovrinHandle theirWalletHandle = 0;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [[SovrinWallet sharedInstance] openWalletWithName:theirWalletName
                                              runtimeConfig:nil
                                                credentials:nil
                                                 completion:^(NSError *error, SovrinHandle h)
           {
               XCTAssertEqual(error.code, Success, "openPoolLedgerWithName got error in completion");
               theirWalletHandle = h;
               [completionExpectation fulfill];
           }];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 7. Create my did
    NSString *myDidJson = @"{}";
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block NSString *myDid = nil;
    __block NSString *myVerkey = nil;
    __block NSString *myPk = nil;
    ret = [SovrinSignus createAndStoreMyDidWithWalletHandle:  myWalletHandle
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
    
    // 8. Create Their DID from Trustee1 seed
    NSString *theirDidJson = @"{\"seed\":\"000000000000000000000000Trustee1\"}";
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block NSString *theirDid = nil;
    __block NSString *theirVerkey = nil;
    __block NSString *theirPk = nil;
    
    ret = [SovrinSignus createAndStoreMyDidWithWalletHandle:  theirWalletHandle
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
    ret = [SovrinSignus storeTheirDidWithWalletHandle: myWalletHandle
                                         identityJSON: theirIdentityJson
                                           completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "storeTheirDid() got error in completion");
               [completionExpectation fulfill];
           }];
    
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 10. Prepare NYM transaction
    NSNumber *nymReqId = @(1491566332010850);
    NSString *nymTxnRequest = [NSString stringWithFormat:@"{"\
                               "\"identifier\":\"%@\","\
                               "\"operation\":{"\
                                    "\"dest\":\"%@\","\
                                    "\"type\":\"1\"},"\
                               "\"reqId\":%@,"\
                               "\"signature\": null"\
                               "}", theirVerkey, myDid, nymReqId];
    
    // TODO: 110 or 304 error Error. some issue with nymTxnRequest
    // 11. Send NYM request with signing
    __block NSString *nymTxnResponse;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [SovrinLedger signAndSubmitRequestWithWalletHandle:theirWalletHandle
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
    NSNumber *getNymRequestId = @(1491566332010862);//[[PoolUtils sharedInstance] getRequestId];
    NSString *getNymTxnRequest = [NSString stringWithFormat:@"{"\
                                  "\"reqId\":%@,"\
                                  "\"signature\":null,"\
                                  "\"identifier\":\"%@\","\
                                  "\"operation\":{"\
                                    "\"type\":\"105\","\
                                    "\"dest\":\"%@\"}"\
                                  "}", getNymRequestId , myVerkey, myDid];
    
    __block NSString *getNymTxnResponseJson;
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    ret = [SovrinLedger submitRequestWithPoolHandle:poolHandle
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
    XCTAssertTrue([getNymTxnResponse[@"result"][@"data"][@"dest"] isEqualToString:myDid], @"wrong dest!");
    
    [TestUtils cleanupStorage];
}

-(BOOL) validate:(NSString*) key d1: (NSDictionary*) d1 d2: (NSDictionary*) d2
{
    id obj1 = [ d1 objectForKey: key];
    id obj2 = [ d2 objectForKey: key];
    return [ obj1 isEqual: obj2];
}

@end
