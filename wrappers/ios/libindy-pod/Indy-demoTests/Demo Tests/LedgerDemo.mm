#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface LedgerDemo : XCTestCase

@end

@implementation LedgerDemo {
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [TestUtils cleanupStorage];
    [super tearDown];
}

- (void)testLedgerDemo {
    NSString *myWalletConfig = @"{\"id\":\"my_wallet4\"}";
    NSString *theirWalletConfig = @"{\"id\":\"their_wallet\"}";

    // 1. Create ledger config from genesis txn file
    NSString *txnFilePath = [[PoolUtils sharedInstance] createGenesisTxnFileForTestPool:[TestUtils pool]
                                                                             nodesCount:nil
                                                                            txnFilePath:nil];
    NSString *poolConfig = [[PoolUtils sharedInstance] poolConfigJsonForTxnFilePath:txnFilePath];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createPoolLedgerConfigWithPoolName() failed!");

    ret = [[PoolUtils sharedInstance] createPoolLedgerConfigWithPoolName:[TestUtils pool]
                                                              poolConfig:poolConfig];

    // 2. Open pool ledger
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] openPoolLedger:[TestUtils pool]
                                              config:nil
                                         poolHandler:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"openPoolLedgerWithName() failed!");

    // 3. Create my wallet
    ret = [[WalletUtils sharedInstance] createWalletWithConfig:myWalletConfig];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");

    // 4. Open My Wallet. Gets My wallet handle
     IndyHandle myWalletHandle;
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:myWalletConfig
                                                 outHandle:&myWalletHandle];
    XCTAssertEqual(ret.code, Success, @"openWalletWithName() failed!");

    // 5. Create their wallet
    ret = [[WalletUtils sharedInstance] createWalletWithConfig:theirWalletConfig];
    XCTAssertEqual(ret.code, Success, @"createWalletWithPoolName() failed!");

    // 6. Open Their Wallet. Get Their wallet handle
     IndyHandle theirWalletHandle;
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:theirWalletConfig
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
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&theirDid
                                                             outMyVerkey:&theirVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");

    // 9. Prepare NYM transaction
    NSString *nymTxnRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:theirDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymTxnRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");

    // 10. Send NYM request
    NSString *nymTxnResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:theirWalletHandle
                                                              submitterDid:theirDid
                                                               requestJson:nymTxnRequest
                                                           outResponseJson:&nymTxnResponse];
    XCTAssertEqual(ret.code, Success, @"signAndSubmitRequestWithWalletHandle() failed!");

    // 12. Prepare and send GET_NYM request
    NSString *getNymTxnRequest;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymTxnRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");

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

    [[WalletUtils sharedInstance] closeWalletWithHandle:myWalletHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:theirWalletHandle];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
}

@end
