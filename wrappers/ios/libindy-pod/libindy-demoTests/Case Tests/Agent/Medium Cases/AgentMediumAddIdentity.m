//
//  AgentMediumAddIdentity.m
//  libindy-demo
//
//  Created by Anastasia Tarasova on 18/08/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libindy/libindy.h>
#import "TestUtils.h"

@interface AgentMediumAddIdentity : XCTestCase

@end

@implementation AgentMediumAddIdentity

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

- (void)testAgentAddIdentityWorksForIncomingConnectionRequireLedgerRequestButPoolHandleIsInvalid
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = @"indy_agent_add_identity_works_for_incoming_connection_require_ledger_request_but_pool_handle_is_invalid";
    
    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
    
    // 2. Obtain listener's wallet
    IndyHandle listenerWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&listenerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain trustee's wallet
    IndyHandle trusteeWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&trusteeWalletHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenWalletWithPoolName() failed");
    
    // 4. Create and store listener's did
    NSString *listenerDid;
    NSString *listenerVerKey;
    NSString *listenerPubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWalletHandle
                                                                       seed:nil
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:&listenerVerKey
                                                                    outMyPk:&listenerPubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for listener");
    
    // 5. create trustee did
    
    NSString *trusteeDid;
    ret =[[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:trusteeWalletHandle
                                                                      seed:[TestUtils trusteeSeed]
                                                                  outMyDid:&trusteeDid
                                                               outMyVerkey:nil
                                                                   outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trustee");
    NSString *senderDid = [NSString stringWithString:trusteeDid];
    IndyHandle senderWallet = trusteeWalletHandle;
    
    // 6. Build nym request for listener
    NSString *listenerNymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:listenerDid
                                                                 verkey:listenerVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&listenerNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    
    // 7. Sign and submit listener's nym request
    NSString *listenerNymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:trusteeWalletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:listenerNymRequest
                                                           outResponseJson:&listenerNymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerNymRequest");
    
    // 8. Build listener attribute request
    NSString *rawJson =[NSString stringWithFormat:@"{\"endpoint\":{\"ha\":\"%@\", \"verkey\":\"%@\"}}", [TestUtils endpoint], listenerPubKey];
    NSString *listenerAttributeRequest;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:listenerDid
                                                                 targetDid:listenerDid
                                                                      hash:nil
                                                                       raw:rawJson
                                                                       enc:nil
                                                                resultJson:&listenerAttributeRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");
    
    // 9. Sign and submit listener's attribute request
    NSString *listenerAttributeResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:listenerWalletHandle
                                                              submitterDid:listenerDid
                                                               requestJson:listenerAttributeRequest
                                                           outResponseJson:&listenerAttributeResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed for listenerAttributeResponse");
    
    // 10. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::listenWithEndpoint()");
    
    // 11. Add identity
    IndyHandle invalidPoolHandle = listenerHandle;
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:invalidPoolHandle
                                                       walletHandle:listenerWalletHandle
                                                                did:listenerDid];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::addIdentityForListenerHandle() failed for listenerDid");
    
    /* TODO
     * Currently pool_handle and wallet_handle of add_identity will be checked only at required:
     * when listener will check incoming connection and go to ledger for info.
     * As result, add_identity will be successful but next connect will fail.
     * Possible the test should be split into two:
     * - add_identity_works_for_incompatible_pool_and_wallet
     *    with immediately check in the library
     * - connect_works_for_incorrect_connect_request
     *    actual info in ledger or listener_wallet, wrong public key in sender_wallet
     */
    
    // 12. Connect
    
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:senderWallet
                                                   senderDid:senderDid
                                                 receiverDid:listenerDid
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, CommonInvalidState, @"AgentUtils::connectWithPoolHandle() returned wrong code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWalletHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle: poolHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentAddIdentityWorksForUnknownDid
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 3. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:1
                                                       walletHandle:walletHandle
                                                                did:@"unknownDid"];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"AgentUtils::addIdentityForListenerHandle() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentAddIdentityWorksForInvalidListenerHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. obtain receiver's did
    NSString *receiverDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&receiverDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 4. add identity
    IndyHandle invalidListenerHandle = listenerHandle + 1;
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:invalidListenerHandle
                                                         poolHandle:1
                                                       walletHandle:walletHandle
                                                                did:receiverDid];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::addIdentityForListenerHandle() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentAddIdentityWorksForTwice
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 3. obtain did
    NSString *did;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. add identity 1
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for the first attempt");
    
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::addIdentityForListenerHandle() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentAssIdentityWorksForTwoDinOnSameListener
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 3. obtain did 1
    NSString *did;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. obtain did 2
    NSString *did2;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did2
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed for did2");
    
    // 5. add first identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for did");
    
    // 6. add second identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did2];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for did2");
    
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentAddIdentityWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 3. obtain receiver's did
    NSString *receiverDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&receiverDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 5. add identity
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:1
                                                       walletHandle:invalidWalletHandle
                                                                did:receiverDid];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AgentUtils::addIdentityForListenerHandle() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentAddIdentityWorksAfterRemove
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint() failed");
    
    // 3. obtain did
    NSString *did;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for the first attempt");
    
    // 5. remove identity
    ret = [[AgentUtils sharedInstance] removeIdentity:did
                                       listenerHandle:listenerHandle
                                         walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::removeIdentity() failed");
    
    // 6. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for second attempt");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

@end
