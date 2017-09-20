//
//  RegisterWalletType.m
//  libindy-demo
//

#import <Foundation/Foundation.h>


#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import <libindy/libindy.h>
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"

@interface RegisterWallettype : XCTestCase

@end

@implementation RegisterWallettype

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


- (void)testKeychainWalletForAgentConnectWorksForExpiredKey
{
    [TestUtils cleanupStorage];
   [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    
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
    
    // 3. sender wallet
    IndyHandle senderWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&senderWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for sender wallet");
    
    // 4. obtain listener did
    
    NSString *listenerDid;
    NSString *listenerVerKey;
    NSString *listenerPubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWallet
                                                                       seed:nil
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:&listenerVerKey
                                                                    outMyPk:&listenerPubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed listener did");
    
    // 5. obtain trustee did
    
    NSString *senderDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:senderWallet
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&senderDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed sender did");
    
    // 6. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:senderWallet
                                                                      theirDid:listenerDid
                                                                       theirPk:listenerPubKey
                                                                   theirVerkey:listenerVerKey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 7. replace keys
    
    NSString *listenerNewVerKey;
    NSString *listenerNewPubKey;
    
    ret = [[SignusUtils sharedInstance] replaceKeysWithWalletHandle:listenerWallet
                                                                did:listenerDid
                                                       identityJson:@"{}"
                                                        outMyVerKey:&listenerNewVerKey
                                                            outMyPk:&listenerNewPubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::replaceKeysWithWalletHandle() failed");
    
    XCTAssertFalse([listenerVerKey isEqualToString:listenerNewVerKey], @"listener's verKey is the same!");
    XCTAssertFalse([listenerPubKey isEqualToString:listenerNewPubKey], @"listener's pub key is the same!");
    
    // 8. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 9. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:listenerWallet
                                                                did:listenerDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 10. connect hang up expected
    
    BOOL isTimeout = NO;
    ret = [[AgentUtils sharedInstance] connectHangUpExpectedForPoolHandle:poolHandle
                                                             walletHandle:senderWallet
                                                                senderDid:senderDid
                                                              receiverDid:listenerDid
                                                                isTimeout:&isTimeout];
    XCTAssertEqual(isTimeout, YES, @"AgentUtils::connectHandUpExpectedForPoolHandle() succeeded for some reason");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}

@end
