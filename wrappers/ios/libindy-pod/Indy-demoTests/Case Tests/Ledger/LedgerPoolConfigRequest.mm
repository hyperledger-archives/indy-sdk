#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerPoolConfigRequest : XCTestCase

@end

@implementation LedgerPoolConfigRequest

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testBuildPoolConfigRequestsWorks
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"111";
    expectedResult[@"operation"][@"writes"] = @(true);
    expectedResult[@"operation"][@"force"] = @(false);

    NSString *poolConfigRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildPoolConfigRequestWithSubmitterDid:identifier
                                                                               writes:true
                                                                               force:false
                                                                         resultJson:&poolConfigRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolConfigRequestJson, @"schemaRequestJson is nil!");
    NSLog(@"poolConfigRequestJson: %@", poolConfigRequestJson);

    NSDictionary *request = [NSDictionary fromString:poolConfigRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

-(void)testPoolConfigRequestsWorks
{
    [TestUtils cleanupStorage];

    NSString* poolName = @"indy_pool_config_request_works";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. Obtain trustee did

    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:@"000000000000000000000000Trustee1"
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    NSLog(@"trusteeDid: %@", trusteeDid);

    // set Ledger as readonly
    // 4. Build pool config request
    NSString *poolConfigRequestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolConfigRequestWithSubmitterDid:trusteeDid
                                                                                 writes:false
                                                                                  force:false
                                                                             resultJson:&poolConfigRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolConfigRequestJson, @"poolConfigRequestJson is nil!");

    // 5. Sign and submit pool config request
    NSString *poolConfigResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolConfigRequestJson
                                                           outResponseJson:&poolConfigResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(poolConfigResponse, @"poolConfigResponse is nil!");

    // return Ledger to writable state
    // 6. Build pool config request
    poolConfigRequestJson = nil;
    ret = [[LedgerUtils sharedInstance] buildPoolConfigRequestWithSubmitterDid:trusteeDid
                                                                        writes:true
                                                                         force:false
                                                                    resultJson:&poolConfigRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolConfigRequestJson, @"poolConfigRequestJson is nil!");

    // 7. Sign and submit pool config request
    poolConfigResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolConfigRequestJson
                                                           outResponseJson:&poolConfigResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(poolConfigResponse, @"poolConfigResponse is nil!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

@end
