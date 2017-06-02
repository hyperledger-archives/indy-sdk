//
//  LedgerDemo.m
//  libsovrin-demo
//


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libsovrin/libsovrin.h>

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

- (void)testLedger
{
    NSString *poolName = @"ledgerPool";
    XCTestExpectation* completionExpectation = nil;
    
    [TestUtils cleanupStorage];
    
    NSError *ret = [[ PoolUtils sharedInstance] createPoolLedgerConfig: poolName];
    XCTAssertEqual(ret.code, Success, "createPoolLedgerConfig failed");
    
    NSString* config = [[PoolUtils sharedInstance] createPoolConfig: poolName ];
    __block SovrinHandle poolHandle = 0;

    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [SovrinPool openPoolWithName:  poolName
                             andConfig:  config
                            completion: ^(NSError *error, SovrinHandle handle)
    {
        XCTAssertEqual(error.code, Success, "openPoolWithName got error in completion");
        poolHandle = handle;
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"openPoolWithName() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    
    NSString *request = @"{"
                        @"        \"reqId\":1491566332010860,"
                        @"        \"identifier\":\"Th7MpTaRZVRYnPiabds81Y\","
                        @"        \"operation\":{"
                        @"            \"type\":\"105\","
                        @"            \"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\""
                        @"        },"
                        @"        \"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\""
                        @"    }";
    
    __block NSString *result = nil;
    
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [SovrinLedger submitRequest:poolHandle
                          requestJSON:request
                           completion:^ (NSError *error, NSString *requestResultJSON)
    {
        XCTAssertEqual(error.code, Success, "submitRequest() got error in completion");
        result = [NSString stringWithString: requestResultJSON];
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"submitRequest() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    NSError *error;
    
    NSDictionary *dictionary1 = [NSJSONSerialization JSONObjectWithData: [NSData dataWithBytes:[result UTF8String]
                                                                                       length:[result length]]
                                                                                      options:kNilOptions
                                                                                        error:&error];
    NSAssert( dictionary1, @"dictionary1 must not be nil!");
    
    NSString *str = @"{"\
                    @"  \"op\": \"REPLY\","\
                    @"  \"result\": {"\
                    @"        \"reqId\": 1491566332010860"\
                    @"    }"\
                    @"}";

    NSDictionary *dictionary2 = [NSJSONSerialization JSONObjectWithData:  [NSData dataWithBytes:[str UTF8String]
                                                                                         length:[str length]]
                                                                options:  kNilOptions
                                                                  error: &error];
    
    NSAssert( [self validate:@"op" d1: dictionary1 d2: dictionary2] == YES, @"unexpected result");
    NSDictionary *r1 = [ dictionary1 objectForKey: @"result"];
    NSDictionary *r2 = [ dictionary2 objectForKey: @"result"];
    NSAssert( [self validate:@"reqId" d1: r1 d2: r2] == YES, @"unexpected result");
    NSLog(@"test ended");
}

-(BOOL) validate:(NSString*) key d1: (NSDictionary*) d1 d2: (NSDictionary*) d2
{
    id obj1 = [ d1 objectForKey: key];
    id obj2 = [ d2 objectForKey: key];
    return [ obj1 isEqual: obj2];
}

@end
