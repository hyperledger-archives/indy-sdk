//
//  libsovrin_demoTests.m
//  libsovrin-demoTests
//


#import <XCTest/XCTest.h>
#import <libsovrin.h>

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
    NSString *walletName = "@issuer_wallet";
    NSString *xType = @"default";
    
    NSError *ret = [SovrinWa]
}


@end
