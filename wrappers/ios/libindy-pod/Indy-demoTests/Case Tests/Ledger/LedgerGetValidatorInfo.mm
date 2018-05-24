#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerGetValidatorInfo : XCTestCase

@end

@implementation LedgerGetValidatorInfo

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testBuildGetValidatorInfo
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"119";
    
    
    NSString *getValidatorInfoJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetValidatorInfo:identifier
                                                            resultJson:&getValidatorInfoJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::builGetValidatorInfo failed");
    XCTAssertNotNil(getValidatorInfoJson, @"getValidatorInfoJson is nil!");
    NSLog(@"getValidatorInfoJson: %@", getValidatorInfoJson);
    
    NSDictionary *request = [NSDictionary fromString:getValidatorInfoJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
    
    [TestUtils cleanupStorage];
}

- (void) testGetValidatorInfoWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_send_get_validator_info_request_works";
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
    NSString* trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");

    // 4. Build get validator info request
    
    NSString *getValidatorInfoRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetValidatorInfo:trusteeDid
                                        resultJson:&getValidatorInfoRequest];
    XCTAssertNotNil(getValidatorInfoRequest, @"getValidatorInfoRequest is nil!");
    
    // 5. Sign and Submit nym request
    NSString *getValidatorInfoResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:getValidatorInfoRequest
                                                           outResponseJson:&getValidatorInfoResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getValidatorInfoResponse, @"getValidatorInfoResponse is nil!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}


@end

