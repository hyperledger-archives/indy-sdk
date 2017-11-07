//
//  AgentHighCases.m
//  Indy-demo
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"

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

-(void)testPrepareMsg
{
    [TestUtils cleanupStorage];
    IndyHandle walletHandle;
    [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"unknownPool" xtype:nil handle:&walletHandle];
    NSString *senderVk;
    [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                 seed:[TestUtils mySeed1]
                                                             outMyDid:nil
                                                          outMyVerkey:&senderVk];
    NSData * outData;
    [[AgentUtils sharedInstance] prepareMsg:[TestUtils message]
                           withWalletHandle:walletHandle
                                   senderVk:senderVk
                                recipientVk:@"kqa2HyagzfMAq42H5f9u3UMwnSBPQx2QfrSyXbUPxMn"
                                     outMsg:&outData];
    XCTAssertNotNil(outData);
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [TestUtils cleanupStorage];
}
@end


