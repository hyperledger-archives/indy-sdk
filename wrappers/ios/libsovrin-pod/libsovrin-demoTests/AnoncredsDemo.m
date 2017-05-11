//
//  libsovrin_demoTests.m
//  libsovrin-demoTests
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>

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
    
    NSError *ret = [[SovrinWallet sharedInstance] createWallet: poolName
                                                          name: walletName
                                                         xType: xType
                                                        config: nil
                                                   credentials: nil
                                                    completion: ^(NSError* error)
                    {
                        
                    }];
    
    NSAssert( ret.code == Success, @"createWallet() failed!");
}


@end
