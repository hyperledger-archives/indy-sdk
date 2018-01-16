//
//  RegisterWalletType.m
//  Indy-demo
//

#import <Foundation/Foundation.h>


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <Indy/Indy.h>
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"

@interface RegisterWalletType : XCTestCase

@end

@implementation RegisterWalletType

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

- (void)testAgentConnectWorksForExpiredKeyInLedger
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    NSString *xtype = @"keychain";
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    
    // register wallet type
    
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype];
    
    // 2. listener wallet
    IndyHandle listenerWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:xtype
                                                                 handle:&listenerWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for listener wallet");
    
    // 3. trustee wallet
    IndyHandle trusteeWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&trusteeWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for trustee wallet");
    
    // 4. obtain listener did
    
    NSString *listenerDid;
    NSString *listenerVerKey;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWallet
                                                                       seed:nil
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:&listenerVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed listener did");
    
    // 5. obtain trustee did
    
    NSString *trusteeDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:trusteeWallet
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed trustee did");
    
//    NSString *senderDid = [NSString stringWithString:trusteeDid];
//    IndyHandle senderWallet = trusteeWallet;
    
    // 6. Build & submit nym request
    
    NSString *listenerNymJson;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:listenerDid
                                                                 verkey:listenerVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&listenerNymJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:trusteeWallet
                                                              submitterDid:trusteeDid
                                                               requestJson:listenerNymJson
                                                           outResponseJson:nil];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerNymJson");
    
    // 7. Build & submit attrib request
    
//    NSString *invalidAttribData = [NSString stringWithFormat:@"{\"endpoint\":{\"ha\":\"%@\", \"verkey\":\"%@\"}}", [TestUtils endpoint], listenerPubKey];
//    NSString *listenerAttribJson;
//    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:listenerDid
//                                                                 targetDid:listenerDid
//                                                                      hash:nil
//                                                                       raw:invalidAttribData
//                                                                       enc:nil
//                                                                resultJson:&listenerAttribJson];
//    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");
//    
//    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
//                                                              walletHandle:listenerWallet
//                                                              submitterDid:listenerDid
//                                                               requestJson:listenerAttribJson
//                                                           outResponseJson:nil];
//    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerAttribJson");
//    
//    // 8. replace keys
//    
//    NSString *listenerNewVerKey;
//    NSString *listenerNewPubKey;
//    
//    ret = [[DidUtils sharedInstance] replaceKeysForDid:listenerDid
//                                             identityJson:@"{}"
//                                             walletHandle:listenerWallet
//                                               poolHandle:poolHandle
//                                              outMyVerKey:&listenerNewVerKey
//                                                  outMyPk:&listenerNewPubKey];
//    XCTAssertEqual(ret.code, Success, @"DidUtils::replaceKeysForDid() failed");
//    
//    
//    
//    XCTAssertFalse([listenerVerKey isEqualToString:listenerNewVerKey], @"listener's verKey is the same!");
//    XCTAssertFalse([listenerPubKey isEqualToString:listenerNewPubKey], @"listener's pub key is the same!");
    
    // 9. listen
    
//    IndyHandle listenerHandle = 0;
//    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
//                                      connectionCallback:nil
//                                         messageCallback:nil
//                                       outListenerHandle:&listenerHandle];
//    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 9. add identity
//    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
//                                                         poolHandle:poolHandle
//                                                       walletHandle:listenerWallet
//                                                                did:listenerDid];
//    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 10. connect hang up expected
    
//    BOOL isTimeout = NO;
//    ret = [[AgentUtils sharedInstance] connectHangUpExpectedForPoolHandle:poolHandle
//                                                             walletHandle:senderWallet
//                                                                senderDid:senderDid
//                                                              receiverDid:listenerDid
//                                                                isTimeout:&isTimeout];
//    XCTAssertEqual(isTimeout, YES, @"AgentUtils::connectHandUpExpectedForPoolHandle() succeeded for some reason");
//
//    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:trusteeWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

@end
