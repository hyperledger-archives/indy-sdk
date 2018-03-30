#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerPoolRestartRequest : XCTestCase

@end

@implementation LedgerPoolRestartRequest

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testBuildPoolRestartRequestsWorksForStartAction
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"118";
    expectedResult[@"operation"][@"action"] = @"start";
    expectedResult[@"operation"][@"datetime"] = @"0";

    NSString *poolRestartRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildPoolRestartRequestWithSubmitterDid:identifier
                                                                               action:@"start"
                                                                               datetime:@"0"
                                                                         resultJson:&poolRestartRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolRestartRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolRestartRequestJson, @"poolRestartRequestJson is nil!");
    NSLog(@"poolRestartRequestJson: %@", poolRestartRequestJson);

    NSDictionary *request = [NSDictionary fromString:poolRestartRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

- (void)testBuildPoolRestartRequestsWorksForCancelAction
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"118";
    expectedResult[@"operation"][@"action"] = @"cancel";
    
    NSString *poolRestartRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildPoolRestartRequestWithSubmitterDid:identifier
                                                                                  action:@"cancel"
                                                                                datetime:nil
                                                                              resultJson:&poolRestartRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolRestartRequestWithSubmitterDid() failed");
    XCTAssertNotNil(poolRestartRequestJson, @"poolRestartRequestJson is nil!");
    NSLog(@"poolRestartRequestJson: %@", poolRestartRequestJson);
    
    NSDictionary *request = [NSDictionary fromString:poolRestartRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
    
    [TestUtils cleanupStorage];
}

@end
