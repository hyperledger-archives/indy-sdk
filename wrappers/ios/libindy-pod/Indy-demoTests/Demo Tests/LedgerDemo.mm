//
//  LedgerDemo.m
//  Indy-demo
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

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
    NSString *poolName = @"pool_1";
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
    
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName config:nil poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"openPoolLedgerWithName() failed!");
    
    // 3. Create my wallet
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:myWalletName
                                                           xtype:walletType
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");
    
    // 4. Open My Wallet. Gets My wallet handle
    __block IndyHandle myWalletHandle = 0;
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:myWalletName
                                                    config:nil
                                                 outHandle:&myWalletHandle];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    
    // 5. Create their wallet
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:theirWalletName
                                                           xtype:walletType
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");
    
    // 6. Open Their Wallet. Get Their wallet handle
    __block IndyHandle theirWalletHandle = 0;
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:theirWalletName
                                                    config:nil
                                                 outHandle:&theirWalletHandle];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    
    // 7. Create my did
    
    NSString *myDid = nil;
    NSString *myVerkey = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:myWalletHandle
                                                                       seed:nil
                                                                   outMyDid:&myDid
                                                                outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 8. Create Their DID from Trustee1 seed
    NSString *theirDid = nil;
    NSString *theirVerkey = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:theirWalletHandle
                                                                       seed:@"000000000000000000000000Trustee1"
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 9. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                                                \"verkey\":\"%@\"\
                                   }", theirDid, theirVerkey];
    
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:myWalletHandle
                                                         identityJson:theirIdentityJson];
    XCTAssertEqual(ret.code, Success, @"IndyDid::storeTheirDid() failed!");
    
    // 10. Prepare NYM transaction
    // removing signature field does not help
    NSNumber *nymReqId = [[PoolUtils sharedInstance] getRequestId];
    NSString *nymTxnRequest = [NSString stringWithFormat:@"{"
                               "\"identifier\":\"%@\","
                               "\"operation\":{"
                                    "\"dest\":\"%@\","
                                    "\"type\":\"1\"},"
                               "\"protocolVersion\": 1,"
                               "\"reqId\":%d"
                               "}", theirDid, myDid, [nymReqId intValue]];

    // 11. Send NYM request with signing
    NSString *nymTxnResponse;
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:theirWalletHandle
                                                              submitterDid:theirDid
                                                               requestJson:nymTxnRequest
                                                           outResponseJson:&nymTxnResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithWalletHandle() failed!");
    XCTAssertTrue(nymTxnResponse);
    
    // 12. Prepare and send GET_NYM request
    NSNumber *getNymRequestId = [[PoolUtils sharedInstance] getRequestId];
    NSString *getNymTxnRequest = [NSString stringWithFormat:@"{"
                                  "\"reqId\":%d,"
                                  "\"signature\": null,"
                                  "\"identifier\":\"%@\","
                                  "\"protocolVersion\":1,"
                                  "\"operation\":{"
                                        "\"type\":\"105\","
                                        "\"dest\":\"%@\"}"
                                  "}", [getNymRequestId intValue] , myVerkey, myDid];
    
    __block NSString *getNymTxnResponseJson;
    
    ret = [[LedgerUtils sharedInstance] submitRequest:getNymTxnRequest
                                       withPoolHandle:poolHandle
                                           resultJson:&getNymTxnResponseJson];
    XCTAssertEqual(ret.code, Success, @"submitRequestWithPoolHandle() failed!");
    
    NSDictionary *getNymTxnResponse = [NSDictionary fromString:getNymTxnResponseJson];
    NSString *dataStr = getNymTxnResponse[@"result"][@"data"];
    NSDictionary *data = [NSDictionary fromString:dataStr];
    XCTAssertNotNil(data[@"dest"], @"data[dest] is nil");
    XCTAssertTrue([data[@"dest"] isEqualToString:myDid], @"wrong dest!");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:myWalletHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:theirWalletHandle];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
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
    
    ret = [[PoolUtils sharedInstance] openPoolLedger:poolName config:nil poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"openPoolLedgerWithName() failed!");
    
    // 3. Create my wallet
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:myWalletName
                                                           xtype:walletType
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");
    
    // 4. Open My Wallet. Gets My wallet handle
    __block IndyHandle myWalletHandle = 0;
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:myWalletName config:nil outHandle:&myWalletHandle];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    
    // 5. Create their wallet
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:theirWalletName
                                                           xtype:walletType
                                                          config:nil];
    
    // 6. Open Their Wallet. Gets Their wallet handle
    __block IndyHandle theirWalletHandle = 0;
    
    ret = [[WalletUtils sharedInstance] openWalletWithName:theirWalletName config:nil outHandle:&theirWalletHandle];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");
    
    // 7. Create my did
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *myDid = nil;
    NSString *myVerkey = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:myWalletHandle
                                                                       seed:nil
                                                                   outMyDid:&myDid
                                                                outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 8. Create Their DID from Trustee1 seed
    
    NSString *theirDid = nil;
    NSString *theirVerkey = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:theirWalletHandle
                                                                       seed:@"000000000000000000000000Trustee1"
                                                                   outMyDid:&theirDid
                                                                outMyVerkey:&theirVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");
    
    // 9. Store Their DID
    
    NSString* theirIdentityJson = [NSString stringWithFormat: @"{\"did\":\"%@\",\
                                   \"verkey\":\"%@\"\
                                   }", theirDid, theirVerkey];
    
   
    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:myWalletHandle
                                                         identityJson:theirIdentityJson];
    XCTAssertEqual(ret.code, Success, @"storeTheirDidWithWalletHandle() failed!");
    
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
    NSString *nymTxnResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:theirWalletHandle
                                                              submitterDid:theirDid
                                                               requestJson:nymTxnRequest
                                                           outResponseJson:&nymTxnResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithWalletHandle() failed!");
    
    // 12. Prepare and send GET_NYM request
    NSNumber *getNymRequestId = [[PoolUtils sharedInstance] getRequestId];
    NSString *getNymTxnRequest = [NSString stringWithFormat:@"{"\
                                  "\"reqId\":%d,"\
                                  "\"identifier\":\"%@\","\
                                  "\"operation\":{"\
                                  "\"type\":\"105\","\
                                  "\"dest\":\"%@\"}"\
                                  "}", [getNymRequestId intValue] , myVerkey, myDid];
    
    NSString *getNymTxnResponseJson;
    
    ret = [[LedgerUtils sharedInstance] submitRequest:getNymTxnRequest
                                       withPoolHandle:poolHandle
                                           resultJson:&getNymTxnResponseJson];
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
