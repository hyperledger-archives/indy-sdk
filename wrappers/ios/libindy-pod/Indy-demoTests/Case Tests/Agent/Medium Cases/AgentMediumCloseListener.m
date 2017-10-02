//
//  AgentMediumCloseListener.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 18/08/2017.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>


#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface AgentMediumCloseListener : XCTestCase

@end

@implementation AgentMediumCloseListener
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

- (void)testAgentCloseListenerWorksForIncorrectHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. obtain did
    NSString *did;
    NSString *verKey;
    NSString *pubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:&pubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. listen

    XCTestExpectation* messageExpectation = [[ XCTestExpectation alloc] initWithDescription: @"message completion finished"];
    
    IndyHandle listenerHandler = 0;
    __block NSString* messageFromClient;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:^(IndyHandle connectionHandle, NSString *message)
           {
               messageFromClient = message;
               [messageExpectation fulfill];
           }
                                       outListenerHandle:&listenerHandler];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithEndpoint() failed");
    
    // 4. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandler
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 5. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 6. Connect
    IndyHandle connectionHandle = 0;
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:&connectionHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    // 7. Close listener
    IndyHandle incorrectListenerHandle = connectionHandle;
    ret = [[AgentUtils sharedInstance] closeListener:incorrectListenerHandle];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::closeListener() returned wrong code");
    
    // 8. send
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:connectionHandle
                                                         message:[TestUtils clientMessage]];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::sendWithConnectionHandler() failed");
    
    // 9. wait for message callback
    [self waitForExpectations: @[messageExpectation] timeout:[TestUtils defaultTimeout]];
    
    XCTAssertTrue([messageFromClient isEqualToString:[TestUtils clientMessage]], @"wrong message from client!");
    
    [[AgentUtils sharedInstance] closeListener: listenerHandler];
    [[WalletUtils sharedInstance] closeWalletWithHandle: walletHandle];
    
    [TestUtils cleanupStorage];
}

- (void)testAgentCloseListenerWorksForTwice
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. obtain did
    NSString *did;
    NSString *verKey;
    NSString *pubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:&pubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:[TestUtils endpoint]];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");

    // 4. listen
    
    IndyHandle listenerHandler = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:[TestUtils endpoint]
                                      connectionCallback:nil
                                         messageCallback:nil
                                       outListenerHandle:&listenerHandler];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithEndpoint() failed");
    
    // 5. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandler
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 6. Close listener (1)
    ret = [[AgentUtils sharedInstance] closeListener:listenerHandler];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::closeListener() failed");
    
    // 7. Close listener (2)
    
    ret = [[AgentUtils sharedInstance] closeListener:listenerHandler];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::closeListener() returned wrong code");
    
    [[WalletUtils sharedInstance] closeWalletWithHandle: walletHandle];
    
    [TestUtils cleanupStorage];
}

@end
