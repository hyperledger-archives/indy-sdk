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
#import "NSString+Validation.h"
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
    
    // WARNING: You may need to change port to 9802, because 9801 is already used in pool. Here and in other similar places too.
    // 3. listen
    NSString *endpoint = @"127.0.0.1:9801";
    
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                       connectionCallback:nil
                                          messageCallback:nil
                                        outListenerHandle:&listenerHandle];
     XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed ");
    
    // 4. Add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed ");
    
    // 5. Store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
     XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed ");
    
    // 6. Connect
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
    NSString *xtype = @"default";
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
                                                                 handle:&trusteeWalletHandle];
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
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:trusteeWalletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed for trusteeDid");
    
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
    NSString *endpoint = @"127.0.0.1:9810";
    NSString *rawJson = [NSString stringWithFormat:@"{\"endpoint\":{\"ha\":\"%@\", \"verkey\":\"%@\"}}", endpoint, listenerPubKey];
    NSString *listenerAttribRequest;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:listenerDid
                                                                 targetDid:listenerDid
                                                                      hash:nil
                                                                       raw:rawJson
                                                                       enc:nil
                                                                resultJson:&listenerAttribRequest];
    
    // 9. Sign and submit attribute request
    NSString *litenerAttribResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:listenerWalletHandle
                                                              submitterDid:listenerDid
                                                               requestJson:listenerAttribRequest outResponseJson:&litenerAttribResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed for listenerAttribRequest");
    XCTAssertTrue([litenerAttribResponse isValid], @"invalid litenerAttribResponse: %@",litenerAttribResponse);
    
    
    
    // 10. listen
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                       connectionCallback:nil
                                          messageCallback:nil
                                        outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:poolHandle
                                                       walletHandle:listenerWalletHandle
                                                                did:listenerDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
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
    NSString *endpoint = @"127.0.0.1:9802";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // In Rust there is some temporary code which will be replaced with sovrin_agent_listen
    // 4. listen
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                       connectionCallback:nil
                                          messageCallback:nil
                                        outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::listenWithEndpoint() failed");
    
    // 4. Add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 5. connect // TODO: Stuck here
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
    NSString *endpoint = @"127.0.0.1:9803";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. Listen
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                           connectionCallback:nil
                                              messageCallback:nil
                                            outListenerHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Add identity

- (void)testAgentAddIdentityWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *endpoint = @"127.0.0.1:9811";
    
    // 1. Create and open receiver's wallet
    SovrinHandle receiverWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"ignore"
                                                             walletName:@"wallet11receiver"
                                                                  xtype:@"default"
                                                                 handle:&receiverWallet];
     XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for receiverWallet");
    
    // 2. listen
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                       connectionCallback:nil
                                          messageCallback:nil
                                        outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithEndpoint() failed");
    
    // 3. Create and store receiver's did
    NSString *receiverDid;
    NSString *receiverVerkey;
    NSString *receiverPk;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:receiverWallet
                                                                       seed:nil
                                                                   outMyDid:&receiverDid
                                                                outMyVerkey:&receiverVerkey
                                                                    outMyPk:&receiverPk];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for receiverDid");
    
    
    // 4. Add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:-1
                                                       walletHandle:receiverWallet
                                                                did:receiverDid];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:receiverWallet
                                                                      theirDid:receiverDid
                                                                       theirPk:receiverPk
                                                                   theirVerkey:receiverVerkey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // TODO: There is some zmq for sockets involved for clean test.
    SovrinHandle connectionHandle = 0;
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:-1
                                                walletHandle:receiverWallet
                                                   senderDid:receiverDid
                                                 receiverDid:receiverDid
                                             messageCallback:nil
                                         outConnectionHandle:&connectionHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    
    [TestUtils cleanupStorage];
}

- (void)testAgentAddIdentityWorksForMultipleKeys
{
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *endpoint = @"127.0.0.1:9814";
    
    // 1. Create and open receiver's wallet
    SovrinHandle receiverWallet = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"ignore"
                                                             walletName:@"wallet14receiver"
                                                                  xtype:@"default"
                                                                 handle:&receiverWallet];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed for receiverWallet");
    
    // 2. listen
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                       connectionCallback:nil
                                          messageCallback:nil
                                        outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithEndpoint() failed");
    
    // 3. Create and store receiver DID 1
    NSString *receiverDid1;
    NSString *receiverVerkey1;
    NSString *receiverPk1;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:receiverWallet
                                                                       seed:nil
                                                                   outMyDid:&receiverDid1
                                                                outMyVerkey:&receiverVerkey1
                                                                    outMyPk:&receiverPk1];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for receiverDid 1");
    
    // 4. Create and store receiver DID 2
    NSString *receiverDid2;
    NSString *receiverVerkey2;
    NSString *receiverPk2;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:receiverWallet
                                                                       seed:nil
                                                                   outMyDid:&receiverDid2
                                                                outMyVerkey:&receiverVerkey2
                                                                    outMyPk:&receiverPk2];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for receiverDid 2");
    
    // TODO: In Rust there is socket code.
    
    // 5. Add identities
    NSMutableArray *receiverDids = [NSMutableArray new];
    [receiverDids addObject:receiverDid1];
    [receiverDids addObject:receiverDid2];
    
    for (NSString *did in receiverDids)
    {
        ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                             poolHandle:-1
                                                           walletHandle:receiverWallet
                                                                    did:did];
        XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed for DID: %@", did);
    }
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:receiverWallet
                                                                      theirDid:receiverDid1
                                                                       theirPk:receiverPk1
                                                                   theirVerkey:receiverVerkey1
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:receiverWallet
                                                                      theirDid:receiverDid2
                                                                       theirPk:receiverPk2
                                                                   theirVerkey:receiverVerkey2
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 6. Connect with DID 1
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:-1
                                                walletHandle:receiverWallet
                                                   senderDid:receiverDid1
                                                 receiverDid:receiverDid1
                                             messageCallback:nil
                                         outConnectionHandle:nil];
     XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed for DID 1");
    
     // 6. Connect with DID 2
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:-1
                                                walletHandle:receiverWallet
                                                   senderDid:receiverDid2
                                                 receiverDid:receiverDid2
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed for DID 2");
    
    [TestUtils cleanupStorage];
}

// MARK: - Remove identity

-(void)testAgentRemoveIdentityWorks
{
    [TestUtils cleanupStorage];
    NSString *endpoint = @"127.0.0.1:9813";
    NSError *ret;
    
    // 1. Obtain receiver's wallet handle
    SovrinHandle receiverWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"ignore"
                                                             walletName:@"wallet13receiver"
                                                                  xtype:@"default"
                                                                 handle:&receiverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. Listen
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                       connectionCallback:nil messageCallback:nil outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithEndpoint() failed");
    
    // 3. Create and store receiver's DID
    NSString *receiverDid;
    NSString *receiverPk;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:receiverWalletHandle
                                                                       seed:nil
                                                                   outMyDid:&receiverDid
                                                                outMyVerkey:nil
                                                                    outMyPk:&receiverPk];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for receiverDid 2");
    
    // 4. Add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:-1
                                                       walletHandle:receiverWalletHandle
                                                                did:receiverDid];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDidWithWalletHandle() failed for receiverDid 2");
    
    // 5. remove identity
    ret = [[AgentUtils sharedInstance] removeIdentity:receiverDid
                                       listenerHandle:listenerHandle
                                         walletHandle:receiverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::removeIdentity() failed");
    
    
    [TestUtils cleanupStorage];
}

// MARK: - Agent send

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
    NSString *endpoint = @"127.0.0.1:9804";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. listen
    // connection callback
    XCTestExpectation* listenerConnectionCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block SovrinHandle serverToClientConnectId = 0;
    void (^connectionCallback)(SovrinHandle, SovrinHandle) = ^(SovrinHandle xListenerHandle, SovrinHandle xConnectionHandle) {
        serverToClientConnectId = xConnectionHandle;
        NSLog(@"AgentHighCases::testAgentSendWorksForAllDataInWalletPresent:: listener's connectionCallback triggered.");
        [listenerConnectionCompletionExpectation fulfill];
    };
    
    XCTestExpectation* messageFromClientCompletionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"listener completion finished"];
    __block NSString *clientMessage;
    // message from client callback
    void (^messageFromClientCallback)(SovrinHandle, NSString *) = ^(SovrinHandle xConnectionHandle, NSString * message) {
        clientMessage = message;
        NSLog(@"AgentHighCases::testAgentSendWorksForAllDataInWalletPresent::messageFromClientCallback triggered with message: %@", message);
        [messageFromClientCompletionExpectation fulfill];
    };
    
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                           connectionCallback:connectionCallback
                                              messageCallback:messageFromClientCallback
                                            outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    // 5. Add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:0
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 6. Connect
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
    
    [self waitForExpectations:@[listenerConnectionCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    
    NSString *refClientMessage = @"msg_from_client";
    NSString *refServerMessage = @"msg_from_server";
    
    // 7. Send client message
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:clientToServerConnectId
                                                         message:refClientMessage];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::sendWithConnectionHandler() failed for client message");
    
    [self waitForExpectations: @[messageFromClientCompletionExpectation] timeout:[TestUtils defaultTimeout]];
    XCTAssertTrue([clientMessage isEqualToString:refClientMessage], @"wrong clientMessage");
    
    // 8. Send server message
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:serverToClientConnectId
                                                         message:refServerMessage];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::sendWithConnectionHandler() failed for server message");
    
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
    NSString *endpoint = @"127.0.0.1:9805";
    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                           connectionCallback:nil
                                              messageCallback:nil
                                            outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    // 4 Add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 5. store their did from parts
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 6. connect
    SovrinHandle connectionHandle = 0;
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:&connectionHandle];
     XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    // 7. close connection
    ret = [[AgentUtils sharedInstance] closeConnection:connectionHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::closeConnection() failed");
    
    // 8. try to send message
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
    NSString *endpoint = @"127.0.0.1:9806";
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
    void (^connectionCallback)(SovrinHandle, SovrinHandle) = ^(SovrinHandle xListenerHandle, SovrinHandle xConnectionHandle) {
        serverToClientConnectId = xConnectionHandle;
        NSLog(@"connectionCallback triggered.");
        [connectionExpectation fulfill];
    };

    SovrinHandle listenerHandle = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                           connectionCallback:connectionCallback
                                              messageCallback:nil
                                            outListenerHandle:&listenerHandle];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithWalletHandle() failed");
    
    // 5. Add Identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandle
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 6. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::connectWithPoolHandle() failed");
    
    [self waitForExpectations: @[connectionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 7. close connection
    ret = [[AgentUtils sharedInstance] closeConnection:serverToClientConnectId];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::closeConnection() failed");
    
    // 8. send
    NSString *serverMessage = @"msg_from_server";
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:serverToClientConnectId
                                                         message:serverMessage];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler() returned wrong code");

    [TestUtils cleanupStorage];
}

// MARK: - Agent close listener
- (void)testAgentCloseListenerWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. Create and open wallet
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:@"pool8"
                                                             walletName:@"wallet8"
                                                                  xtype:@"default"
                                                                 handle:&walletHandle];
     XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 2. Create and store did
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
    NSString *endpoint = @"127.0.0.1:9808";
    ret = [[SignusUtils sharedInstance] storeTheirDidFromPartsWithWalletHandle:walletHandle
                                                                      theirDid:did
                                                                       theirPk:pubKey
                                                                   theirVerkey:verKey
                                                                      endpoint:endpoint];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::storeTheirDidFromPartsWithWalletHandle() failed");
    
    // 4. Listen
    
    XCTestExpectation* connectionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"connection completion finished"];

    SovrinHandle listenerHandler = 0;
    __block SovrinHandle serverToClientConenctId = 0;
    ret = [[AgentUtils sharedInstance] listenForEndpoint:endpoint
                                       connectionCallback:^(SovrinHandle xListenerHandle, SovrinHandle xConnectionHandle)
    {
        serverToClientConenctId = xConnectionHandle;
        [connectionExpectation fulfill];
    }
                                          messageCallback:nil
                                        outListenerHandle:&listenerHandler];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::listenWithEndpoint() failed");
    
    // 5. add identity
    ret = [[AgentUtils sharedInstance] addIdentityForListenerHandle:listenerHandler
                                                         poolHandle:-1
                                                       walletHandle:walletHandle
                                                                did:did];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::addIdentityForListenerHandle() failed");
    
    // 6. connect
    ret = [[AgentUtils sharedInstance] connectWithPoolHandle:0
                                                walletHandle:walletHandle
                                                   senderDid:did
                                                 receiverDid:did
                                             messageCallback:nil
                                         outConnectionHandle:nil];
    
    [self waitForExpectations: @[connectionExpectation] timeout:[TestUtils defaultTimeout]];

    // 7. close Listener
    ret = [[AgentUtils sharedInstance] closeListener: listenerHandler];
    XCTAssertEqual(ret.code, Success, @"AgentUtils::closeListener() failed");
    
    // 8. send
    NSString *serverMessage = @"msg_from_server";
    ret = [[AgentUtils sharedInstance] sendWithConnectionHandler:serverToClientConenctId
                                                         message:serverMessage];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AgentUtils::sendWithConnectionHandler() returned wrong error code.");
    
    [TestUtils cleanupStorage];
}

- (void)testAllAgentHighCasesTests
{
    [self testAgentListerWorksWithSovrinAgentConnect];
    [self testAgentConnectWorksForRemoteData];
    [self testAgentConnectWorksForAllDataInWalletPresent];
    [self testAgentListenWorksForAllDataInWalletPresent];
    [self testAgentAddIdentityWorks];
    [self testAgentAddIdentityWorksForMultipleKeys];
    [self testAgentRemoveIdentityWorks];
    [self testAgentSendWorksForAllDataInWalletPresent];
    [self testAgentCloseConnectionWorksForOngoing];
    [self testAgentCloseConnectionWorksForIncomingConnection];
    [self testAgentCloseListenerWorks];
}

- (void)testSequence
{
    [self testAgentAddIdentityWorks];
    [self testAgentAddIdentityWorksForMultipleKeys];
}

@end
