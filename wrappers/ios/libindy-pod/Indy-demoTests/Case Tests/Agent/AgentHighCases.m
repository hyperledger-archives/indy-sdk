//
//  AgentHighCases.m
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "CryptoUtils.h"

@interface AgentHignCases : XCTestCase

@end

@implementation AgentHignCases

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

- (void)testParseMessageWorksForAuthenticatedMessage
{
    [TestUtils cleanupStorage];
    IndyHandle walletHandle;
    [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"unknownPool" xtype:nil handle:&walletHandle];

    NSString *senderVk = nil, *recipientVk = nil;
    NSError *ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                                   keyJson:[NSString stringWithFormat:@"{\"seed\": \"%@\"}", [TestUtils mySeed1]]
                                                                 outVerkey:&senderVk];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(senderVk);
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\": \"%@\"}", [TestUtils mySeed2]]
                                                        outVerkey:&recipientVk];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(recipientVk);

    NSData *encrypted = nil;
    ret = [[AgentUtils sharedInstance] prepareMsg:[TestUtils message]
                                 withWalletHandle:walletHandle
                                         senderVk:senderVk
                                      recipientVk:recipientVk
                                           outMsg:&encrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:prepareMsg failed");
    XCTAssertNotNil(encrypted);

    NSData *decrypted = nil;
    NSString *outSenderVk = nil;
    ret = [[AgentUtils sharedInstance] parseMsg:encrypted
                               withWalletHandle:walletHandle
                                    recipientVk:recipientVk
                                    outSenderVk:&outSenderVk
                                         outMsg:&decrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:parseMsg failed");
    XCTAssertEqualObjects(outSenderVk, senderVk);
    XCTAssertEqualObjects(decrypted, [TestUtils message]);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

- (void)testParseMessageWorksForAnonymousMessage
{
    [TestUtils cleanupStorage];
    IndyHandle walletHandle;
    [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"unknownPool" xtype:nil handle:&walletHandle];

    NSString *senderVk = nil, *recipientVk = nil;
    NSError *ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                                   keyJson:[NSString stringWithFormat:@"{\"seed\": \"%@\"}", [TestUtils mySeed1]]
                                                                 outVerkey:&senderVk];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(senderVk);
    ret = [[CryptoUtils sharedInstance] createKeyWithWalletHandle:walletHandle
                                                          keyJson:[NSString stringWithFormat:@"{\"seed\": \"%@\"}", [TestUtils mySeed2]]
                                                        outVerkey:&recipientVk];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:createKeyWithWalletHandle failed");
    XCTAssertNotNil(recipientVk);

    NSData *encrypted = nil;
    ret = [[AgentUtils sharedInstance] prepareAnonymousMsg:[TestUtils message]
                                               recipientVk:recipientVk
                                                    outMsg:&encrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:prepareAnonymousMsg failed");
    XCTAssertNotNil(encrypted);

    NSData *decrypted = nil;
    NSString *outSenderVk = nil;
    ret = [[AgentUtils sharedInstance] parseMsg:encrypted
                               withWalletHandle:walletHandle
                                    recipientVk:recipientVk
                                    outSenderVk:&outSenderVk
                                         outMsg:&decrypted];
    XCTAssertEqual(ret.code, Success, @"CryptoUtils:parseMsg failed");
    XCTAssertEqualObjects(outSenderVk, nil);
    XCTAssertEqualObjects(decrypted, [TestUtils message]);

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}

@end
