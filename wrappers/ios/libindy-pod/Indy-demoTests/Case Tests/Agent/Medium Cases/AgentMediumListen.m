//
//  AgentMediumListen.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 18/08/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface AgentMediumListen : XCTestCase

@end

@implementation AgentMediumListen

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

- (void)testAgentListenWorksForEndpointAlreadyInUse
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. listen first
    
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint failed");
    
    // 2. listen second
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:nil];
    XCTAssertEqual(ret.code, CommonIOError, @"AgentUtils::listenForEndpoint returned wrong error code");
    
    // 3. close listener
    ret = [[AgentUtils sharedInstance] closeListener:listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::closeListener failed");
    
    [TestUtils cleanupStorage];
}

- (void)testAgentListenWorksForInvalidEndpoint
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    ret = [[AgentUtils sharedInstance] listenForEndpoint:@"127.0.0"
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:nil];
    XCTAssertEqual(ret.code, CommonIOError, @"AgentUtils::listenForEndpoint returned wrong error code");
    
    [TestUtils cleanupStorage];
}

- (void)testAgentListenWorksForRejectUnknownSender
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. pool
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");
    
    // 2. listener handle
    
    IndyHandle listenerWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&listenerWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed for listenerWallet");
    
    // 3. sender wallet
    IndyHandle senderWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&senderWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed for senderWallet");
    
    // 4. obtain listener did
    NSString *listenerDid;
    NSString *listenerVerkey;
    NSString *listenerPubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWallet
                                                                       seed:nil
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:&listenerVerkey
                                                                    outMyPk:&listenerPubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle failed for listenerDid");
    
    // 5. obtain sender did
    NSString *senderDid;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:senderWallet
                                                                       seed:nil
                                                                   outMyDid:&senderDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle failed for senderDid");
    
    // 6. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:senderWallet
                                                                      theirDid:listenerDid
                                                                       theirPk:listenerPubkey
                                                                   theirVerkey:listenerVerkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle failed");
    
    // 7. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint failed");
    
    // 8. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:listenerWallet
                                                                did:listenerDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle failed");
    
    // 9. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:senderWallet
                                                   senderDid:senderDid
                                                 receiverDid:listenerDid
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, CommonInvalidState, @"AgentUtils::connectWithPoolHandle returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentListenWorksForRejectExpiredSavedSenderData
{
    [TestUtils cleanupStorage];
    
    NSError *ret;
    NSString *poolName = [TestUtils pool];
    
    // 1. pool
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");
    
    // 2. listener handle
    
    IndyHandle listenerWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&listenerWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed for listenerWallet");
    
    // 3. sender wallet
    IndyHandle senderWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&senderWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed for senderWallet");
    
    // 4. obtain listener did
    NSString *listenerDid;
    NSString *listenerVerkey;
    NSString *listenerPubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWallet
                                                                       seed:[TestUtils trusteeSeed]
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:&listenerVerkey
                                                                    outMyPk:&listenerPubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle failed for listenerDid");
    
    // 5. obtain sender did
    NSString *senderDid;
    NSString *senderVerkey;
    NSString *senderPubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:senderWallet
                                                                       seed:nil
                                                                   outMyDid:&senderDid
                                                                outMyVerkey:&senderVerkey
                                                                    outMyPk:&senderPubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle failed for senderDid");
    
    // 6. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:listenerWallet
                                                                      theirDid:senderDid
                                                                       theirPk:senderPubkey
                                                                   theirVerkey:senderVerkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle failed");
    
    // 7. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint failed");
    
    // 8. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:listenerWallet
                                                                did:listenerDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle failed");
    
    // 9. replace keys
    ret = [[SignusUtils sharedInstance] replaceKeysWithWalletHandle:senderWallet
                                                                did:senderDid
                                                       identityJson:@"{}"
                                                        outMyVerKey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::replaceKeysWithWalletHandle failed");
    
    // 10. store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:senderWallet
                                                                      theirDid:listenerDid
                                                                       theirPk:listenerPubkey
                                                                   theirVerkey:listenerVerkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle failed");
    
    // 11. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:senderWallet
                                                   senderDid:senderDid
                                                 receiverDid:listenerDid
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, CommonInvalidState, @"AgentUtils::connectWithPoolHandle returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}

@end
