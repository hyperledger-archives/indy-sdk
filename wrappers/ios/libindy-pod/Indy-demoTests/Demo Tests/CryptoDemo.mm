//
//  CryptoDemo.m
//  Indy-demo
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface CryptoDemo : XCTestCase

@end

@implementation CryptoDemo {
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

- (void)testCryptoDemo {
    NSString *myWalletConfig = @"{\"id\":\"my_wallet4\"}";
    NSString *theirWalletConfig = @"{\"id\":\"their_wallet4\"}";

    IndyHandle myWalletHandle = 0;
    IndyHandle theirWalletHandle = 0;

    //1. Create my wallet
    ret = [[WalletUtils sharedInstance] createWalletWithConfig:myWalletConfig];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed for my wallet!");

    // 2. Open my wallet
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:myWalletConfig
                                                 outHandle:&myWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed for my wallet!");

    // 3. Create Their Wallet
    ret = [[WalletUtils sharedInstance] createWalletWithConfig:theirWalletConfig];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed for their wallet!");

    // 4. Open their wallet
    ret = [[WalletUtils sharedInstance] openWalletWithConfig:theirWalletConfig
                                                 outHandle:&theirWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed for their wallet!");

    // 5. Create My DID
    NSString *myDid = nil;
    NSString *myVerkey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:myWalletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:&myVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");

    // 6. Create Their DID
    NSString *theirDid = nil;
    NSString *theirVerkey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:theirWalletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&theirDid
                                                     outMyVerkey:&theirVerkey];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");

    // 7. Store Their DID
    NSString *theirIdentityJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"did": theirDid,
            @"verkey": theirVerkey
    }];

    ret = [[DidUtils sharedInstance] storeTheirDidWithWalletHandle:myWalletHandle
                                                      identityJson:theirIdentityJson];
    XCTAssertEqual(ret.code, Success, @"createAndStoreMyDid() failed!");

    // 8. Their Sign message
    NSString *messageJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"reqId": @(1496822211362017764),
            @"identifier": @"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            @"operation": @{
                    @"type": @"1",
                    @"dest": @"VsKV7grR1BUE29mG2Fm2kX",
                    @"dest": @"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
            }
    }];
    NSData *message = [messageJson dataUsingEncoding:NSUTF8StringEncoding];

    NSData *signature = nil;
    ret = [[CryptoUtils sharedInstance] signMessage:message
                                                key:theirVerkey
                                       walletHandle:theirWalletHandle
                                       outSignature:&signature];
    XCTAssertEqual(ret.code, Success, @"sign() failed!");

    // 9. I Verify message
    BOOL verified = false;
    ret = [[CryptoUtils sharedInstance] verifySignature:signature
                                             forMessage:message
                                                    key:theirVerkey
                                             outIsValid:&verified];

    XCTAssertEqual(ret.code, Success, @"verifySignature() failed!");
    XCTAssertEqual(YES, verified, "verifySignature() signature is not valid");

    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:myWalletHandle];
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:theirWalletHandle];

}

@end
