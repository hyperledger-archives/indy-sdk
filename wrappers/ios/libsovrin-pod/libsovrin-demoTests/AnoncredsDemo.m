//
//  libsovrin_demoTests.m
//  libsovrin-demoTests
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

static NSTimeInterval defaultTimeout = 3;

@interface AnoncredsDemo : XCTestCase

@end

@implementation AnoncredsDemo

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

- (void)testAnoncredsDemo
{
    NSString *poolName = @"pool1";
    NSString *walletName = @"issuer_wallet";
    NSString *xType = @"default";
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSError *ret = [[SovrinWallet sharedInstance] createWallet: poolName
                                                          name: walletName
                                                         xType: xType
                                                        config: nil
                                                   credentials: nil
                                                    completion: ^ (NSError* error)
    {
        XCTAssertEqual(error.code, Success, "createWallet got error in completion");
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"createWallet() failed!");

    __block SovrinHandle walletHandle = 0;

    ret = [[SovrinWallet sharedInstance] openWallet: walletName
                                      runtimeConfig: nil
                                        credentials: nil
                                         completion: ^ (NSError* error, SovrinHandle handle)
    {
        XCTAssertEqual(error.code, Success, "openWallet got error in completion");
        walletHandle = handle;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"openWallet() failed!");
    
    ret = [[SovrinWallet sharedInstance] closeWallet: walletHandle
                                          completion: ^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "closeWallet got error in completion");
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"closeWallet() failed!");
    
}


@end
