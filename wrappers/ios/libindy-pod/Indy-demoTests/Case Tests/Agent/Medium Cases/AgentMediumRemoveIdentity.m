//
//  AgentMediumRemoveIdentity.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 18/08/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface AgentMediumRemoveIdentity : XCTestCase

@end

@implementation AgentMediumRemoveIdentity
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

- (void)testAgentRemoveIdentityWorksForInvalidListenerHandle
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
    IndyHandle invalidListenerHandle = 0;
    ret = [[AgentUtils sharedInstance] removeIdentity:did
                                       listenerHandle:invalidListenerHandle
                                         walletHandle:walletHandle];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::removeIdentity() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentRemoveIdentityWorksForTwice
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
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:receiverDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for the first attempt");
    
    // 5. remove identity (1)
    ret = [[AgentUtils sharedInstance] removeIdentity:receiverDid
                                       listenerHandle:listenerHandle
                                         walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::removeIdentity() failed");
    
    // 6. remove identity (2)
    
    ret = [[AgentUtils sharedInstance] removeIdentity:receiverDid
                                       listenerHandle:listenerHandle
                                         walletHandle:walletHandle];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::removeIdentity() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentRemoveIdentityWorksForInvalidWalletHandle
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
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for the first attempt");
    
    // 5. remove identity
    IndyHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AgentUtils sharedInstance] removeIdentity:did
                                       listenerHandle:listenerHandle
                                         walletHandle:invalidWalletHandle];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AgentUtils::removeIdentity() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentRemoveIdentityWorksForUnknownReceiverDid
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
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 4. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 5. remove identity
    ret = [[AgentUtils sharedInstance] removeIdentity:@"unknownDid"
                                       listenerHandle:listenerHandle
                                         walletHandle:walletHandle];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"AgentUtils::removeIdentity() returned wrong error code");
    
    [[AgentUtils sharedInstance] closeListener:listenerHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    
    [TestUtils cleanupStorage];
}


@end
