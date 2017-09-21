//
//  AgentMediumSend.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 18/08/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface AgentMediumSend : XCTestCase

@end

@implementation AgentMediumSend
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

- (void)testAgentSendWorksForInvalidConnectionHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed");
    
    // 2. obtain did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle failed");
    
    // 3. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle failed");
    
    // 4. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint failed");
    
    // 5. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
     XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle failed");
    
    // 6. connect
    IndyHandle connectionHandle = 0;
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:&connectionHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle failed");
    
    // 7. send
    IndyHandle invalidConnectionHandle = connectionHandle + 100;
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:invalidConnectionHandle
                                                         message:[TestUtils clientMessage]];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler returned wrong code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentSendWorksForClosedConnection
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed");
    
    // 2. obtain did
    
    NSString *did;
    NSString *verkey;
    NSString *pubkey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verkey
                                                                    outMyPk:&pubkey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle failed");
    
    // 3. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubkey
                                                                   theirVerkey:verkey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle failed");
    
    // 4. listen
    IndyHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenForEndpoint failed");
    
    // 5. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle failed");
    
    // 6. connect
    IndyHandle connectionHandle = 0;
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:&connectionHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle failed");
    
    // 7. close connection
    ret = [[AgentUtils sharedInstance] closeConnection:connectionHandle];
    
    // 7. send
    IndyHandle invalidConnectionHandle = connectionHandle + 100;
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:invalidConnectionHandle
                                                         message:[TestUtils clientMessage]];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler returned wrong code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentSendWorksForClosedListenerIncomingConenction
{
    [TestUtils cleanupStorage];
    NSString *poolName = [TestUtils pool];
    NSError *ret;
    
    // 1. pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");
    
    // 2. listener wallet
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
    
    // 4. obtain listener's did
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
                                                                       seed:[TestUtils trusteeSeed]
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
    XCTAssertEqual(ret.code, Success, @"SignusUtils::listenForEndpoint failed");
    
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
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle failed");
    
    [self waitForExpectations:@[listenerConnectionCompletionExpectation] timeout:[TestUtils shortTimeout]];
    
    // 10. close listener
    ret = [[AgentUtils sharedInstance] closeListener:listenerHandle];
    
    // 11. send
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:connectionHandleFromCallback
                                                         message:[TestUtils serverMessage]];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler return wrong error code");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// TODO: This test is disabled in Rust
- (void)testAgentSendWorksForRemovedIdentity
{
    [TestUtils cleanupStorage];
    NSString *poolName = [TestUtils pool];
    NSError *ret;
    
    // 1. pool ledger
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");
    
    // 2. listener wallet
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
    
    // 4. obtain listener's did
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
                                                                       seed:[TestUtils trusteeSeed]
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
    XCTAssertEqual(ret.code, Success, @"SignusUtils::listenForEndpoint failed");
    
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
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle failed");
    
    [self waitForExpectations:@[listenerConnectionCompletionExpectation] timeout:[TestUtils shortTimeout]];
    
    
    
    // 10. remove identity
    ret = [[AgentUtils sharedInstance] removeIdentity:listenerDid
                                       listenerHandle:listenerHandle
                                         walletHandle:listenerWallet];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::removeIdentity failed");
    
    // 11. send
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:connectionHandleFromCallback
                                                         message:[TestUtils serverMessage]];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler return wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:listenerWallet];
    [[WalletUtils sharedInstance] closeWalletWithHandle:senderWallet];
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    
    [TestUtils cleanupStorage];
}



@end
