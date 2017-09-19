//
//  AgentMediumConnect.m
//  libindy-demo
//
//  Created by Anastasia Tarasova on 18/08/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//


#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libindy/libindy.h>
#import "TestUtils.h"

@interface AgentMediumConnect : XCTestCase

@end

@implementation AgentMediumConnect
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

// MARK: - Connect

- (void)testAgentConnectWorksForUnknownListener
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. listener wallet
    IndyHandle listenerWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWallet
                                                                       seed:nil
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed listener did");
    
    // 5. obtain sender did
    
    NSString *senderDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:senderWallet
                                                                       seed:nil
                                                                   outMyDid:&senderDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed sender did");
    
    // 6. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 7. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:listenerWallet
                                                                did:listenerDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 8. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:senderWallet
                                                   senderDid:senderDid
                                                 receiverDid:listenerDid
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::connectWithPoolHandle() returned wrong error code");
    
    // 9. close everything
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForInvalidRemoteData
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. listener wallet
    IndyHandle listenerWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&listenerWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for listener wallet");
    
    // 3. trustee wallet
    IndyHandle trusteeWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&trusteeWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for trustee wallet");
    IndyHandle senderWallet = trusteeWallet;
    
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
    
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:trusteeWallet
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed trustee did");
    
    NSString *senderDid = [NSString stringWithString:trusteeDid];
    
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
    
    NSString *invalidAttribData = [NSString stringWithFormat:@"{\"endpoint\":{\"verkey\":\"%@\"}}", listenerPubKey];
    NSString *listenerAttribJson;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:listenerDid
                                                                 targetDid:listenerDid
                                                                      hash:nil
                                                                       raw:invalidAttribData
                                                                       enc:nil
                                                                resultJson:&listenerAttribJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:listenerWallet
                                                              submitterDid:listenerDid
                                                               requestJson:listenerAttribJson
                                                           outResponseJson:nil];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerAttribJson");
    
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
    
    // 10. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:senderWallet
                                                   senderDid:senderDid
                                                 receiverDid:listenerDid
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::connectWithPoolHandle() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForExpiredKeyInWallet
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
    
    ret = [[WalletUtils sharedInstance] registerWalletType:xtype forceCreate:false];
    
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

- (void)testAgentConnectWorksForExpiredKeyInLedger
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. listener wallet
    IndyHandle listenerWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&listenerWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for listener wallet");
    
    // 3. trustee wallet
    IndyHandle trusteeWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&trusteeWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for trustee wallet");
    IndyHandle senderWallet = trusteeWallet;
    
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
    
    NSString *trusteeDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:trusteeWallet
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed trustee did");
    
    NSString *senderDid = [NSString stringWithString:trusteeDid];
    
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
    
    NSString *invalidAttribData = [NSString stringWithFormat:@"{\"endpoint\":{\"ha\":\"%@\", \"verkey\":\"%@\"}}", [TestUtils endpoint], listenerPubKey];
    NSString *listenerAttribJson;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:listenerDid
                                                                 targetDid:listenerDid
                                                                      hash:nil
                                                                       raw:invalidAttribData
                                                                       enc:nil
                                                                resultJson:&listenerAttribJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:listenerWallet
                                                              submitterDid:listenerDid
                                                               requestJson:listenerAttribJson
                                                           outResponseJson:nil];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerAttribJson");
    
    // 8. replace keys
    
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
    
    // 9. listen
    
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
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForIncompatibleWalletAndPool
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"other_pool"
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. obtain my did
    
    NSString *did;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 5. add identity
    
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 6. connect
    
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, WalletIncompatiblePoolError, @"AgentUtils::connectWithPoolHandle() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForUnknownSenderDid
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. create and open pool ledger
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. obtain my did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 5. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 6. add identity
    
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 7. connect
    
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:walletHandle
                                                   senderDid:@"unknownDid"
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"AgentUtils::connectWithPoolHandle() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentconnectWorksForInvalidListener
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. obtain my did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. connect hang up expected
    
    BOOL isTimeout = NO;
    ret = [[AgentUtils sharedInstance] connectHangUpExpectedForPoolHandle:0
                                                             walletHandle:walletHandle
                                                                senderDid:did
                                                              receiverDid:did
                                                                isTimeout:&isTimeout];
    XCTAssertTrue(isTimeout, @"AgentUtils::connectHandUpExpectedForPoolHandle() succeeded for some reason");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForClosedListener
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. obtain my did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 5. add identity
    
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 6. close listener
    
    ret = [[AgentUtils sharedInstance] closeListener:listenerHandle];
    
    // 7. connect hang up expected
    
    BOOL isTimeout = NO;
    ret = [[AgentUtils sharedInstance] connectHangUpExpectedForPoolHandle:0
                                                             walletHandle:walletHandle
                                                                senderDid:did
                                                              receiverDid:did
                                                                isTimeout:&isTimeout];
    XCTAssertTrue(isTimeout, @"AgentUtils::connectHandUpExpectedForPoolHandle() succeeded for some reason");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForConnectWithoutAddIdentity
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. obtain my did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 5. connect hang up expected
    
    BOOL isTimeout = NO;
    ret = [[AgentUtils sharedInstance] connectHangUpExpectedForPoolHandle:0
                                                             walletHandle:walletHandle
                                                                senderDid:did
                                                              receiverDid:did
                                                                isTimeout:&isTimeout];
    XCTAssertTrue(isTimeout, @"AgentUtils::connectHandUpExpectedForPoolHandle() succeeded for some reason");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForConnectAfterRemoveIdentity
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. obtain my did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 5. add identity
    
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 6. remove identity
    
    ret = [[AgentUtils sharedInstance] removeIdentity:did
                                       listenerHandle:listenerHandle
                                         walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::removeIdentity() failed");
    
    // 7. connect hang up expected
    
    BOOL isTimeout = NO;
    ret = [[AgentUtils sharedInstance] connectHangUpExpectedForPoolHandle:0
                                                             walletHandle:walletHandle
                                                                senderDid:did
                                                              receiverDid:did
                                                                isTimeout:&isTimeout];
    XCTAssertTrue(isTimeout, @"AgentUtils::connectHandUpExpectedForPoolHandle() succeeded for some reason");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForPassedCallback
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. pool handle
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. obtain my did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 5. listen
    
    // connection callback
    XCTestExpectation* listenerConnectionCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block IndyHandle connectionHandleFromCallback;
    void (^connectionCallback)(IndyHandle, IndyHandle) = ^(IndyHandle xListenerHandle, IndyHandle xConnectionHandle) {
        NSLog(@"AgentHighCases::testAgentListenWorksForPassedOnConnectCallback:: listener's connectionCallback triggered.");
        connectionHandleFromCallback = xConnectionHandle;
        [listenerConnectionCompletionExpectation fulfill];
    };

    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:connectionCallback
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 6. add identity
    
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 7. connect
    
    // message from server callback
    
    XCTestExpectation* messageFromClientCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSString *clientMessage;
    void (^messageFromClientCallback)(IndyHandle, NSString *) = ^(IndyHandle xConnectionHandle, NSString * message) {
        clientMessage = message;
        NSLog(@"messageFromClientCallback triggered with message: %@", message);
        [messageFromClientCompletionExpectation fulfill];
    };

    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:messageFromClientCallback
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    [self waitForExpectations:@[listenerConnectionCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 8. send
    
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:connectionHandleFromCallback
                                                         message:[TestUtils clientMessage]];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::sendWithConnectionHandler() failed");
    
    [self waitForExpectations: @[messageFromClientCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertTrue([clientMessage isEqualToString:[TestUtils clientMessage]], @"wrong client message");
    
    // 9. clean
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    [[PoolUtils sharedInstance] closeHandle: poolHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForTwice
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. pool handle
    
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. listener wallet
    IndyHandle listenerWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
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
    NSString *listenerVerkey;
    NSString *listenerPubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWallet
                                                                       seed:nil
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:&listenerVerkey
                                                                    outMyPk:&listenerPubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for listener did");
    
    // 5. obtain sender did
    
    NSString *senderDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:senderWallet
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&senderDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for sender did");
    
    // 6. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:senderWallet
                                                                      theirDid:listenerDid
                                                                       theirPk:listenerPubkey
                                                                   theirVerkey:listenerVerkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 7. listen
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 8. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:listenerWallet
                                                                did:listenerDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 9. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:senderWallet
                                                   senderDid:senderDid
                                                 receiverDid:listenerDid
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    // 10. connect hang up expected
    BOOL isTimeout = NO;
    ret = [[AgentUtils sharedInstance] connectHangUpExpectedForPoolHandle:poolHandle
                                                             walletHandle:senderWallet
                                                                senderDid:senderDid
                                                              receiverDid:listenerDid
                                                                isTimeout:&isTimeout];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectHandUpExpectedForPoolHandle() failed");
    
    // 11. close
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}



@end
