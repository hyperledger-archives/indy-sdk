#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerPoolUpgradeRequest : XCTestCase

@end

@implementation LedgerPoolUpgradeRequest

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testBuildPoolUpgradeRequestsWorksForStartAction
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"109";
    expectedResult[@"operation"][@"name"] = @"upgrade-ios";
    expectedResult[@"operation"][@"version"] = @"2.0.0";
    expectedResult[@"operation"][@"action"] = @"start";
    expectedResult[@"operation"][@"sha256"] = @"f284b";
    expectedResult[@"operation"][@"schedule"] = @"{}";
    expectedResult[@"operation"][@"reinstall"] = @(false);
    expectedResult[@"operation"][@"force"] = @(false);

    NSString *poolUpgradeRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:identifier
                                                                               name:@"upgrade-ios"
                                                                               version:@"2.0.0"
                                                                               action:@"start"
                                                                               sha256:@"f284b"
                                                                               timeout:nil
                                                                               schedule:@"{}"
                                                                               justification:nil
                                                                               reinstall:false
                                                                               force:false
                                                                         resultJson:&poolUpgradeRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolUpgradeRequestJson, @"poolUpgradeRequestJson is nil!");
    NSLog(@"poolUpgradeRequestJson: %@", poolUpgradeRequestJson);

    NSDictionary *request = [NSDictionary fromString:poolUpgradeRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}


- (void)testBuildPoolUpgradeRequestsWorksForCancelAction
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"109";
    expectedResult[@"operation"][@"name"] = @"upgrade-ios";
    expectedResult[@"operation"][@"version"] = @"2.0.0";
    expectedResult[@"operation"][@"action"] = @"cancel";
    expectedResult[@"operation"][@"sha256"] = @"f284b";
    expectedResult[@"operation"][@"reinstall"] = @(false);
    expectedResult[@"operation"][@"force"] = @(false);

    NSString *poolUpgradeRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:identifier
                                                                                    name:@"upgrade-ios"
                                                                                 version:@"2.0.0"
                                                                                  action:@"cancel"
                                                                                  sha256:@"f284b"
                                                                                 timeout:nil
                                                                                schedule:nil
                                                                           justification:nil
                                                                               reinstall:false
                                                                                   force:false
                                                                              resultJson:&poolUpgradeRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolUpgradeRequestJson, @"poolUpgradeRequestJson is nil!");
    NSLog(@"poolUpgradeRequestJson: %@", poolUpgradeRequestJson);

    NSDictionary *request = [NSDictionary fromString:poolUpgradeRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

-(void)testPoolUpgradeRequestsWorks
{
    [TestUtils cleanupStorage];

    NSString* poolName = @"indy_pool_upgrade_request_works";
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

    // start
    // 4. Build pool upgrade request
    int nextYear = [[[NSCalendar currentCalendar]
            components:NSCalendarUnitYear fromDate:[NSDate date]]
            year] + 1;

    NSString *schedule = [NSString stringWithFormat:@"{\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\":\"%d-01-25T12:49:05.258870+00:00\",\n"
                                                     " \"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\":\"%d-01-25T13:49:05.258870+00:00\",\n"
                                                     " \"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\":\"%d-01-25T14:49:05.258870+00:00\",\n"
                                                     " \"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\":\"%d-01-25T15:49:05.258870+00:00\"}",
                                                     nextYear, nextYear, nextYear, nextYear ];
    NSString *poolUpgradeRequestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:trusteeDid
                                                                                    name:@"upgrade-ios-1"
                                                                                 version:@"2.0.0"
                                                                                  action:@"start"
                                                                                  sha256:@"f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398"
                                                                                 timeout:nil
                                                                                schedule:schedule
                                                                           justification:nil
                                                                               reinstall:false
                                                                                   force:false
                                                                              resultJson:&poolUpgradeRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolUpgradeRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolUpgradeRequestJson, @"poolUpgradeRequestJson is nil!");
    NSLog(@"poolUpgradeRequestJson: %@", poolUpgradeRequestJson);

    // 5. Sign and submit pool upgrade request
    NSString *poolUpgradeResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolUpgradeRequestJson
                                                           outResponseJson:&poolUpgradeResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(poolUpgradeResponse, @"poolUpgradeResponse is nil!");

    // cancel
    // 6. Build pool upgrade request
    NSString *poolUpgradeCancelRequestJson = nil;
    ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:trusteeDid
                                                                           name:@"upgrade-ios-1"
                                                                        version:@"2.0.0"
                                                                         action:@"cancel"
                                                                         sha256:@"1c3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398"
                                                                        timeout:nil
                                                                       schedule:nil
                                                                  justification:nil
                                                                      reinstall:false
                                                                          force:false
                                                                     resultJson:&poolUpgradeCancelRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolUpgradeRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolUpgradeCancelRequestJson, @"poolUpgradeCancelRequestJson is nil!");
    NSLog(@"poolUpgradeRequestJson: %@", poolUpgradeCancelRequestJson);

    // 7. Sign and submit pool upgrade request
    NSString *poolUpgradeCancelResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolUpgradeCancelRequestJson
                                                           outResponseJson:&poolUpgradeCancelResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(poolUpgradeCancelResponse, @"poolUpgradeCancelResponse is nil!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

@end
