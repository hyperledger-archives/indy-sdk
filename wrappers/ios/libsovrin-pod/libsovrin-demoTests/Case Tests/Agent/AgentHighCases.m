//
//  AgentHighCases.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 22.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "PoolUtils.h"
#import "AgentUtils.h"
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"
#import "NSString+Checks.h"
#import "NSArray+JSON.h"

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

- (void)testAgentListerWorksWithSovrinAgentConnect
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool3"
                                                             walletName:@"wallet3"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed ");

    // 2. create DID
    NSString *did;
    NSString *verKey;
    NSString *pubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:&pubKey];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndStoreMyDidWithWalletHandle() failed ");
    
    // 3. listen
    NSString *endpoint = @"tcp://127.0.0.1:9701";
    
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenWithWalletHandle:walletHandle
                                                     endpoint:endpoint
                                           connectionCallback:nil
                                              messageCallback:nil
                                            outListenerHandle:&listenerHandle];
     XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed ");
    
    // 4. Store their did from parts
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:nil];
     XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed ");
    
    // 5. Connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed ");
    [TestUtils cleanupStorage];
}

// MARK: - Agent connect

- (void)testAgentConnectWorksForRemoteData
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_agent_connect_works_for_remote_data";
    NSString *xtype;
    NSError *ret;
    
    // 1. create and open pool ledger config
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed ");
    
    // 2. Create listener's wallet
    SovrinHandle listenerWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                                           walletName:@"wallet10.1"
                                                                                                xtype:xtype
                                                                 handle:&listenerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for listenerWalletHandle");
    
    // 3. Create trustees's wallet
    SovrinHandle trusteeWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:@"wallet10.2"
                                                                  xtype:xtype
                                                                 handle:&listenerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for listenerWalletHandle");
    
    // 4. Obtain listener Did
    NSString *listenerDid;
    NSString *listenerVerKey;
    NSString *listenerPubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWalletHandle
                                                                       seed:nil
                                                                   outMyDid:&listenerDid
                                                                outMyVerkey:&listenerVerKey
                                                                    outMyPk:&listenerPubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for listenerDid");
    
     // 5. Obtain trustee Did
    NSString *trusteeDid;
    NSString *trusteeDidJson = @"{\"seed\":\"000000000000000000000000Trustee1\",\"cid\":true}";
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:listenerWalletHandle
                                                                       seed:trusteeDidJson
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil
                                                                    outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for trusteeDid");
    
    // 6. Build nym request
    NSString *listenerNymJson;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                                                    targetDid:listenerDid
                                                                                       verkey:listenerVerKey
                                                                                        alias:nil
                                                                                         role:nil
                                                             outRequest:&listenerNymJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed for listenerNymJson");
    XCTAssertTrue([listenerNymJson isValid], @"invalid listenerNymJson: %@",listenerNymJson);
    
    // 7. Sign and submit nym request
    NSString *listenerNymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:trusteeWalletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:listenerNymJson
                                                           outResponseJson:&listenerNymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerNymJson");
    XCTAssertTrue([listenerNymResponse isValid], @"invalid listenerNymResponse: %@",listenerNymResponse);
    
    // 8. Sign and submit listener attribute request
    NSString *endpoint = @"127.0.0.1:9710";
    NSString *listenerAttributeRequest;
    NSString *rawJson = [NSString stringWithFormat:@"{\"endpoint\":{\"ha\":\"%@\", \"verkey\":\"%@\"}}", endpoint, listenerPubKey];
    NSString *listenerAttribRequest;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:listenerDid
                                                                 targetDid:listenerDid
                                                                      hash:nil
                                                                       raw:rawJson
                                                                       enc:nil
                                                                resultJson:&listenerAttributeRequest];
    
    // 9. Sign and submit attribute request
    NSString *litenerAttribResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:listenerWalletHandle
                                                              submitterDid:listenerDid
                                                               requestJson:listenerAttribRequest outResponseJson:&litenerAttribResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerAttribRequest");
    XCTAssertTrue([litenerAttribResponse isValid], @"invalid litenerAttribResponse: %@",litenerAttribResponse);
    
    // 10. listen
    ret = [[AgentUtils sharedInstance] listenWithWalletHandle:listenerWalletHandle
                                                     endpoint:endpoint
                                           connectionCallback:nil
                                              messageCallback:nil
                                            outListenerHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    // 11. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:poolHandle
                                                walletHandle:trusteeWalletHandle
                                                   senderDid:trusteeDid
                                                 receiverDid:listenerDid
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    [TestUtils cleanupStorage];
}

- (void)testAgentConnectWorksForAllDataInWalletPresent
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. obtain wallet handle
    SovrinHandle walletHandle;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool1"
                                                             walletName:@"wallet1"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. Obtain did
    NSString *seed = @"sovrin_agent_connect_works_for_a";
    NSString *did;
    NSString *verKey;
    NSString *pubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:seed
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:&pubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. store their did from parts
    NSString *endpoint = @"127.0.0.1:9702";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // In Rust there is some temporary code which will be replaced with sovrin_agent_listen
    
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Sovrin agent listen

- (void)testAgentListenWorksForAllDataInWalletPresent
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. Obtain wallet handle
    SovrinHandle walletHandle;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool2"
                                                             walletName:@"wallet2"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. Obtain did
    NSString *seed = @"sovrin_agent_listen_works_for_al";
    NSString *did;
    NSString *verKey;
    NSString *pubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:seed
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:&pubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. Store their did from parts
    NSString *endpoint = @"127.0.0.1:9703";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. Listen
    ret = [[AgentUtils sharedInstance] listenWithWalletHandle:walletHandle
                                                     endpoint:endpoint
                                           connectionCallback:nil
                                              messageCallback:nil
                                            outListenerHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Agent seed

- (void)testAgentSendWorksForAllDataInWalletPresent
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. Create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool4"
                                                             walletName:@"wallet4"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. Obtain did
    NSString *did;
    NSString *verKey;
    NSString *pubKey;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&did
                                                                outMyVerkey:&verKey
                                                                    outMyPk:&pubKey];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed");
    
    // 3. Store their did from parts
    NSString *endpoint = @"127.0.0.1:9704";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. listen
    // connection callback
    __block SovrinHandle serverToClientConnectId = 0;
    void (^connectionCallback)(SovrinHandle, SovrinHandle) = ^(SovrinHandle xConnectionHandle, SovrinHandle xListenerHandle) {
        serverToClientConnectId = xConnectionHandle;
        NSLog(@"connectionCallback triggered.");
    };
    
     XCTestExpectation* messageFromClientCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSString *clientMessage;
    // message from client callback
    void (^messageFromClientCallback)(SovrinHandle, NSString *) = ^(SovrinHandle xConnectionHandle, NSString * message) {
        clientMessage = message;
        NSLog(@"messageFromClientCallback triggered with message: %@", message);
        [messageFromClientCompletionExpectation fulfill];
    };
    
    ret = [[AgentUtils sharedInstance] listenWithWalletHandle:walletHandle
                                                     endpoint:endpoint
                                           connectionCallback:connectionCallback
                                              messageCallback:messageFromClientCallback
                                            outListenerHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    // 5. Connect
    // message from server callback
    
    XCTestExpectation* messageFromServerCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSString *serverMessage;
    void (^messageFromServerCallback)(SovrinHandle, NSString *) = ^(SovrinHandle xConnectionHandle, NSString * message) {
        serverMessage = message;
        NSLog(@"messageFromServerCallback triggered with message: %@", message);
        [messageFromServerCompletionExpectation fulfill];
    };

    SovrinHandle clientToServerConnectId = 0;
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:messageFromServerCallback
                                         outConnectionHandle:&clientToServerConnectId];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    NSString *refClientMessage = @"msg_from_client";
    NSString *refServerMessage = @"msg_from_server";
    
    // 6. Send client message
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:clientToServerConnectId
                                                         message:refClientMessage];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::sendWithConnectionHandler() failed");
    
    [self waitForExpectations: @[messageFromClientCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertTrue([clientMessage isEqualToString:refClientMessage], @"wrong clientMessage");
    
    // 7. Send server message
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:serverToClientConnectId
                                                         message:refServerMessage];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::sendWithConnectionHandler() failed");
    
    [self waitForExpectations: @[messageFromServerCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertTrue([serverMessage isEqualToString:refServerMessage], @"wrong serverMessage");
    
    [TestUtils cleanupStorage];
}

// MARK: - Close connection

- (void)testAgentCloseConnectionWorksForOngoing
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. Create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool3"
                                                             walletName:@"wallet3"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. Obtain did
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
    NSString *endpoint = @"127.0.0.1:9705";
    ret = [[AgentUtils sharedInstance] listenWithWalletHandle:walletHandle
                                                     endpoint:endpoint
                                           connectionCallback:nil
                                              messageCallback:nil
                                            outListenerHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    // 4. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 5. connect
    SovrinHandle connectionHandle = 0;
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:&connectionHandle];
     XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    // 6. close connection
    ret = [[AgentUtils sharedInstance] closeConnection:connectionHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::closeConnection() failed");
    
    // 7. try to send message
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:connectionHandle
                                                         message:@""];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler() returned wrong error code");

    [TestUtils cleanupStorage];
}

- (void)testAgentCloseConnectionWorksForIncomingConnection
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. Create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool4"
                                                             walletName:@"wallet4"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. Obtain did
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
    NSString *endpoint = @"127.0.0.1:9706";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. listen
    
    // connection callback
    XCTestExpectation* connectionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"connection completion finished"];

    __block SovrinHandle serverToClientConnectId = 0;
    void (^connectionCallback)(SovrinHandle, SovrinHandle) = ^(SovrinHandle xConnectionHandle, SovrinHandle xListenerHandle) {
        serverToClientConnectId = xConnectionHandle;
        NSLog(@"connectionCallback triggered.");
        [connectionExpectation fulfill];
    };

    ret = [[AgentUtils sharedInstance] listenWithWalletHandle:walletHandle
                                                     endpoint:endpoint
                                           connectionCallback:connectionCallback
                                              messageCallback:nil
                                            outListenerHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    // 5. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    [self waitForExpectations: @[connectionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 6. close connection
    ret = [[AgentUtils sharedInstance] closeConnection:serverToClientConnectId];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::closeConnection() failed");
    
    // 7. send
    NSString *serverMessage = @"msg_from_server";
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:serverToClientConnectId
                                                         message:serverMessage];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler() returned wrong code");

    [TestUtils cleanupStorage];
}

// MARK: - Agent close listener
- (void)testAgentCloseListenerWorks

@end
