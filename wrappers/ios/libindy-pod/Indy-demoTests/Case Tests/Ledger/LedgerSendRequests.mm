#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface LedgerSendRequests : XCTestCase

@end

@implementation LedgerSendRequests {
    IndyHandle poolHandle;
    IndyHandle walletHandle;
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:[TestUtils pool]
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];

    [TestUtils cleanupStorage];
    [super tearDown];
}

- (void)testSendRequestWorksForInvalidPoolHandle {
    // 1. Build GET NYM Request
    NSString *getNymRequest;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:[TestUtils trusteeDid]
                                                                 targetDid:[TestUtils myDid1]
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");

    // 2. Send request using invalid pool handle
    IndyHandle invalidPoolHandle = poolHandle + 1;
    NSString *getNymResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:invalidPoolHandle
                                                        request:getNymRequest
                                                       response:&getNymResponse];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
}

- (void)testSignAndSubmitRequestWorksForInvalidPoolHandle {
    // 1. Obtain DID
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 2. Build NYM Request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:[TestUtils myDid1]
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");

    // 3. Send and submit request using invalid pool handle
    IndyHandle invalidPoolHandle = poolHandle + 1;
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:invalidPoolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
}

- (void)testSignAndSubmitRequestWorksForInvalidWalletHandle {
    // 1. Obtain DID
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 2. Build NYM Request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:[TestUtils myDid1]
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");

    // 3. Send and submit request using invalid pool handle
    IndyHandle invalidWalletHandle = walletHandle + 1;
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:invalidWalletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
}

- (void)testSubmitRequestWorks {
    NSDictionary *request = @{
            @"reqId": @(1491566332010860),
            @"identifier": [TestUtils trusteeDid],
            @"protocolVersion": @(2),
            @"operation": @{
                    @"type": @"105",
                    @"dest": [TestUtils trusteeDid]
            }
    };

    NSString *responseJson;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:[[AnoncredsUtils sharedInstance] toJson:request]
                                                       response:&responseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed!");

    NSDictionary *actualReply = [NSDictionary fromString:responseJson];

    NSDictionary *actualData = [NSDictionary fromString:actualReply[@"result"][@"data"]];
    XCTAssertTrue([actualReply[@"op"] isEqualToString:@"REPLY"], @"Wrong actualReply[op]");
    XCTAssertTrue([actualReply[@"result"][@"reqId"] isEqualToValue:@(1491566332010860)], @"Wrong actualReply[reqId]");

    XCTAssertTrue([actualData[@"dest"] isEqualToString:[TestUtils trusteeDid]], @"Wrong actualData[dest]");
}

- (void)testSignAndSubmitRequestWorks {
    // 1. create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:nil
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed!");

    // 2. create and store trustee did
    NSString *trusteeDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed!");

    // 3. Build nym request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed!");

    // 4. sign and submit nym request
    NSString *nymResponceJson;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponceJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed!");
}

// MARK: - Sign Request

- (void)testSignRequestWorks {
    // 1. Create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed!");

    NSDictionary *message = @{
            @"reqId": @(1496822211362017764),
            @"identifier": @"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL",
            @"operation": @{
                    @"type": @"1",
                    @"dest": @"VsKV7grR1BUE29mG2Fm2kX",
                    @"dest": @"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa"
            }
    };

    NSDictionary *expectedSignature = @{@"signature": @"XknJouWAcBMLVZ9yo5uXKCDqm8fiHVrgaP5H1MaEEP5vriheYg2YK77MJEQBFBotCVEkdMFi2YhXgY71EvtXNg4"};

    // 3. Sign Request
    NSString *resultJson;
    ret = [[LedgerUtils sharedInstance] signRequestWithWalletHandle:walletHandle
                                                       submitterdid:myDid
                                                        requestJson:[[AnoncredsUtils sharedInstance] toJson:message]
                                                         resultJson:&resultJson];

    NSDictionary *result = [NSDictionary fromString:resultJson];
    XCTAssertTrue([result contains:expectedSignature], @"Wrong Result Json!");
}

- (void)testSignWorksForUnknownSigner {
    NSDictionary *message = @{
            @"reqId": @(1496822211362017764),
            @"identifier": @"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL"
    };

    ret = [[LedgerUtils sharedInstance] signRequestWithWalletHandle:walletHandle
                                                       submitterdid:[TestUtils trusteeDid]
                                                        requestJson:[[AnoncredsUtils sharedInstance] toJson:message]
                                                         resultJson:nil];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"LedgerUtils::signRequestWithWalletHandle() returned wrong code!");
}

// MARK: - NYM Requests

- (void)testNymRequestsWorks {
    // 1. Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");

    // 2. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:nil
                                                                outMyDid:&myDid
                                                             outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed");

    // 3. Build nym request
    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");

    // 4. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");

    // 5. Build get nym request
    NSString *getNymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");

    // 6. Send getNymRequest
    NSString *getNymResponseJson = [[LedgerUtils sharedInstance] submitRetry:getNymRequest
                                                                  poolHandle:poolHandle];

    NSDictionary *getNymResponse = [NSDictionary fromString:getNymResponseJson];
    XCTAssertNotNil(getNymResponse[@"result"][@"seqNo"], @"getNymResponse seqNo is empty");
}

// MARK: - Attribute requests

- (void)testAttributeRequestsWorks {
    // 1. obtain did
    NSString *myDid;
    NSString *myVerkey;
    [[DidUtils sharedInstance] createAndStoreAndPublishDidWithWalletHandle:walletHandle
                                                                poolHandle:poolHandle
                                                                       did:&myDid
                                                                    verkey:&myVerkey];

    // 2. Build attrib request
    NSString *attribRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                      hash:nil
                                                                       raw:@"{\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}}"
                                                                       enc:nil
                                                                resultJson:&attribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");

    // 3. Sign and Submit attrib request
    NSString *attribResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:attribRequest
                                                           outResponseJson:&attribResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");

    // 4. Build getAttribRequest
    NSString *getAttribRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:myDid
                                                                    targetDid:myDid
                                                                          raw:@"endpoint"
                                                                         hash:nil
                                                                          enc:nil
                                                                   resultJson:&getAttribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequest() failed");

    // 5. Send getAttribRequest
    NSString *getAttribResponse = [[LedgerUtils sharedInstance] submitRetry:getAttribRequest
                                                                 poolHandle:poolHandle];
}

// MARK: - Schema request

- (void)testSchemaRequestsWorks {
    // 1. obtain did
    NSString *myDid;
    NSString *myVerkey;
    [[DidUtils sharedInstance] createAndStoreAndPublishDidWithWalletHandle:walletHandle
                                                                poolHandle:poolHandle
                                                                       did:&myDid
                                                                    verkey:&myVerkey];

    // 2. Build schema request
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:myDid
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];

    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequest() failed");

    // 3. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    // 4. Build getSchemaRequest
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                           id:schemaId
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");

    [NSThread sleepForTimeInterval:10];

    // 5. Send getSchemaRequest
    NSString *getSchemaResponse = [[LedgerUtils sharedInstance] submitRetry:getSchemaRequest
                                                                 poolHandle:poolHandle];

    // 6. Parse getSchemaResponse
    ret = [[LedgerUtils sharedInstance] parseGetSchemaResponse:getSchemaResponse
                                                      schemaId:&schemaId
                                                    schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::parseGetSchemaResponse() failed");
}

// MARK: - Node request

// Warning: Enable when you need to run this test. It breaks pool after run.

- (void)submitNodeRequestWorksForNewSteward {
    // 1. Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 2. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 3. Build nym request
    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:@"STEWARD"
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 4. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");

    // 5. Build node request
    NSDictionary *data = @{
            @"node_ip": @"10.0.0.100",
            @"node_port": @(9710),
            @"client_ip": @"10.0.0.100",
            @"client_port": @(9709),
            @"alias": @"Node10",
            @"services": @[@"VALIDATOR"],
            @"blskey": @"4N8aUNHSgjQVgkpm8nhNEfDf6txHznoYREg9kirmJrkivgL4oSEimFF6nsQ6M41QvhM2Z33nves5vfSn9n1UwNFJBYtWVnHYMATn76vLuL3zU88KyeAYcHfsih3He6UHcXDxcaecHVz6jhCYz1P2UZn2bDVruL5wXpehgBfBaLKm3Ba"
    };

    NSString *dest = @"A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y"; // random(32) and base58
    NSString *nodeRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:myDid
                                                               targetDid:dest
                                                                    data:[[AnoncredsUtils sharedInstance] toJson:data]
                                                              resultJson:&nodeRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNodeRequestWithSubmitterDid() failed");

    // 6. Sign and submit request
    NSString *nodeResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:nodeRequest
                                                           outResponseJson:&nodeResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
}

// MARK: - Cred def request

- (void)testCredDefRequestsWorks {
    // 1. obtain did
    NSString *myDid;
    NSString *myVerkey;
    [[DidUtils sharedInstance] createAndStoreAndPublishDidWithWalletHandle:walletHandle
                                                                poolHandle:poolHandle
                                                                       did:&myDid
                                                                    verkey:&myVerkey];

    // 2. Build schema request
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:myDid
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequest() failed");

    // 3. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    // 4. Build getSchemaRequest
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                           id:schemaId
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");

    // 5. Send getSchemaRequest
    NSString *getSchemaResponse = [[LedgerUtils sharedInstance] submitRetry:getSchemaRequest
                                                                 poolHandle:poolHandle];

    ret = [[LedgerUtils sharedInstance] parseGetSchemaResponse:getSchemaResponse
                                                      schemaId:&schemaId
                                                    schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    // 6. Create credential definition
    NSString *credentialDefId;
    NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:myDid
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:walletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinitionWithWalletHandle failed");

    // 7. Build credential def request
    NSString *credDefRequestJson;
    ret = [[LedgerUtils sharedInstance] buildCredDefRequestWithSubmitterDid:myDid
                                                                       data:credentialDefJSON
                                                                 resultJson:&credDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::buildCredDefRequestWithSubmitterDid() failed");

    // 8. Sign and submit credential def request
    NSString *credDefResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:credDefRequestJson
                                                           outResponseJson:&credDefResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not Success");


    // 9. Build get credential def request
    NSString *getCredDefRequest;
    ret = [[LedgerUtils sharedInstance] buildGetCredDefRequestWithSubmitterDid:myDid
                                                                            id:credentialDefId
                                                                    resultJson:&getCredDefRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetCredDefRequestWithSubmitterDid() failed");

    // 10. Send getCredDefRequest
    NSString *getCredDefResponse = [[LedgerUtils sharedInstance] submitRetry:getCredDefRequest
                                                                  poolHandle:poolHandle];

    ret = [[LedgerUtils sharedInstance] parseGetCredDefResponse:getCredDefResponse
                                                      credDefId:&credentialDefId
                                                    credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::parseGetCredDefResponse() failed");
}

// MARK: - Get txn request

- (void)testGetTxnRequestWorks {
    // 1. Create my did
    NSString *myDid = [[DidUtils sharedInstance] createStoreAndPublishMyDidWithWalletHandle:walletHandle
                                                                                 poolHandle:poolHandle];
    // 2. Build & submit schema request
    NSString *schemaRequest;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:[TestUtils gvtSchema]
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");

    NSString *schemaResponseJson;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponseJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");

    // 3. Build & submit get txn request
    NSDictionary *schemaResponse = [NSDictionary fromString:schemaResponseJson];
    NSNumber *seqNo = (NSNumber *) schemaResponse[@"result"][@"txnMetadata"][@"seqNo"];

    NSString *getTxnRequest;
    ret = [[LedgerUtils sharedInstance] buildGetTxnRequestWithSubmitterDid:myDid
                                                                ledgerType:nil
                                                                      data:seqNo
                                                                resultJson:&getTxnRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSString *getTxnResponseJson;
    ret = [[LedgerUtils sharedInstance] submitRequest:getTxnRequest
                                       withPoolHandle:poolHandle
                                           resultJson:&getTxnResponseJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::submitRequest() failed for getTxnRequest: %@", getTxnRequest);

    // 4. Check getTxnResponse
    NSDictionary *getTxnResponse = [NSDictionary fromString:getTxnResponseJson];

    NSDictionary *getTxnSchemaResult = getTxnResponse[@"result"][@"data"];
    XCTAssertNotNil(getTxnSchemaResult[@"txn"][@"data"][@"data"], @"getTxnSchemaResult[data] is nil");
    XCTAssertNotNil(getTxnSchemaResult[@"txnMetadata"][@"seqNo"], @"getTxnSchemaResult[seqNo] is nil");
}

// MARK: Pool upgrade

- (void)testPoolUpgradeRequestsWorks {
    // 1. Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");

    // start
    // 2. Build pool upgrade request
    int nextYear = [[[NSCalendar currentCalendar]
            components:NSCalendarUnitYear fromDate:[NSDate date]]
            year] + 1;

    NSString *schedule = [NSString stringWithFormat:@"{\"Gw6pDLhcBcoQesN72qfotTgFa7cbuqZpkX3Xo6pLhPhv\":\"%d-01-25T12:49:05.258870+00:00\",\n"
                                                            " \"8ECVSk179mjsjKRLWiQtssMLgp6EPhWXtaYyStWPSGAb\":\"%d-01-25T13:49:05.258870+00:00\",\n"
                                                            " \"DKVxG2fXXTU8yT5N7hGEbXB3dfdAnYv1JczDUHpmDxya\":\"%d-01-25T14:49:05.258870+00:00\",\n"
                                                            " \"4PS3EDQ3dW1tci1Bp6543CfuuebjFrg36kLAUcskGfaA\":\"%d-01-25T15:49:05.258870+00:00\"}",
                                                    nextYear, nextYear, nextYear, nextYear];
    NSString *poolUpgradeRequestJson;
    NSString *name = @"upgrade-ios-1";
    NSString *version = @"2.0.0";
    ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:trusteeDid
                                                                           name:name
                                                                        version:version
                                                                         action:@"start"
                                                                         sha256:@"f284bdc3c1c9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398"
                                                                        timeout:nil
                                                                       schedule:schedule
                                                                  justification:nil
                                                                      reinstall:false
                                                                          force:false
                                                                       package_:nil
                                                                     resultJson:&poolUpgradeRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolUpgradeRequestWithSubmitterDid() failed");

    // 3. Sign and submit pool upgrade request
    NSString *poolUpgradeResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolUpgradeRequestJson
                                                           outResponseJson:&poolUpgradeResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    // cancel
    // 4. Build pool upgrade request
    NSString *poolUpgradeCancelRequestJson = nil;
    ret = [[LedgerUtils sharedInstance] buildPoolUpgradeRequestWithSubmitterDid:trusteeDid
                                                                           name:name
                                                                        version:version
                                                                         action:@"cancel"
                                                                         sha256:@"1c3eb2cc3ac9e24a494e285cb387c69510f28de51c15bb93179d9c7f28705398"
                                                                        timeout:nil
                                                                       schedule:nil
                                                                  justification:nil
                                                                      reinstall:false
                                                                          force:false
                                                                       package_:nil
                                                                     resultJson:&poolUpgradeCancelRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolUpgradeRequestWithSubmitterDid() failed");

    // 5. Sign and submit pool upgrade request
    NSString *poolUpgradeCancelResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolUpgradeCancelRequestJson
                                                           outResponseJson:&poolUpgradeCancelResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
}

- (void)testPoolConfigRequestsWorks {
    // 1. Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");

    // set Ledger as readonly
    // 2. Build pool config request
    NSString *poolConfigRequestJson;
    ret = [[LedgerUtils sharedInstance] buildPoolConfigRequestWithSubmitterDid:trusteeDid
                                                                        writes:false
                                                                         force:false
                                                                    resultJson:&poolConfigRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");

    // 3. Sign and submit pool config request
    NSString *poolConfigResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolConfigRequestJson
                                                           outResponseJson:&poolConfigResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");

    // return Ledger to writable state
    // 4. Build pool config request
    poolConfigRequestJson = nil;
    ret = [[LedgerUtils sharedInstance] buildPoolConfigRequestWithSubmitterDid:trusteeDid
                                                                        writes:true
                                                                         force:false
                                                                    resultJson:&poolConfigRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildPoolConfigRequestWithSubmitterDid() failed");

    // 5. Sign and submit pool config request
    poolConfigResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:poolConfigRequestJson
                                                           outResponseJson:&poolConfigResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
}

- (void)testGetValidatorInfoRequestWorks {
    // Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");

    // Build get validator info request

    NSString *getValidatorInfoRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetValidatorInfo:trusteeDid
                                                   resultJson:&getValidatorInfoRequest];
    XCTAssertNotNil(getValidatorInfoRequest, @"getValidatorInfoRequest is nil!");

    // Sign and Submit request
    NSString *getValidatorInfoResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:getValidatorInfoRequest
                                                           outResponseJson:&getValidatorInfoResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getValidatorInfoResponse, @"getValidatorInfoResponse is nil!");
}

- (void)testSubmitActionWorks {
    // Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");

    // Build get validator info request
    NSString *getValidatorInfoRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetValidatorInfo:trusteeDid
                                                   resultJson:&getValidatorInfoRequest];
    XCTAssertNotNil(getValidatorInfoRequest, @"getValidatorInfoRequest is nil!");

    // Sign request
    ret = [[LedgerUtils sharedInstance] signRequestWithWalletHandle:walletHandle
                                                       submitterdid:trusteeDid
                                                        requestJson:getValidatorInfoRequest
                                                         resultJson:&getValidatorInfoRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signRequestWithWalletHandle() failed");

    NSString *getValidatorInfoResponse = nil;
    ret = [[LedgerUtils sharedInstance] submitAction:getValidatorInfoRequest
                                               nodes:nil
                                             timeout:nil
                                      withPoolHandle:poolHandle
                                          resultJson:&getValidatorInfoResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::submitAction() failed");
    XCTAssertNotNil(getValidatorInfoResponse, @"getValidatorInfoResponse is nil!");
}

// MARK: Get Response Metadata

- (void)getResponseMetadataWorksForNymRequests {
    // 1. Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:[TestUtils trusteeSeed]
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");

    // 2. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:nil
                                                                outMyDid:&myDid
                                                             outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed");

    // 3. Build nym request
    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");

    // 4. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");

    // 5. Get NYM response metadata
    NSString *nymResponseMetadataJson = nil;
    ret = [[LedgerUtils sharedInstance] getResponseMetadata:nymResponse
                                           responseMetadata:&nymResponseMetadataJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::getResponseMetadata() failed");

    NSDictionary *nymResponseMetadata = [NSDictionary fromString:nymResponseMetadataJson];
    XCTAssertNotNil(nymResponseMetadata[@"seqNo"], @"nymResponseMetadata seqNo is empty");
    XCTAssertNotNil(nymResponseMetadata[@"txnTime"], @"nymResponseMetadata txnTime is empty");
    XCTAssertNil(nymResponseMetadata[@"lastTxnTime"], @"nymResponseMetadata lastTxnTime is not empty");
    XCTAssertNil(nymResponseMetadata[@"lastSeqNo"], @"nymResponseMetadata lastSeqNo is not empty");

    // 6. Build get nym request
    NSString *getNymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");

    // 7. Send getNymRequest
    NSString *getNymResponse = [[LedgerUtils sharedInstance] submitRetry:getNymRequest
                                                              poolHandle:poolHandle];

    // 8. Get GET_NYM response data
    NSString *getNymResponseDataJson = nil;
    ret = [[LedgerUtils sharedInstance] parseGetNymResponse:getNymResponse
                                                    nymData:&getNymResponseDataJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::parseGetNymResponse() failed");

    NSDictionary *getNymResponseData = [NSDictionary fromString:getNymResponseDataJson];
    XCTAssertEqual(myDid, getNymResponseData[@"did"], @"LedgerUtils::sendRequestWithPoolHandle() failed");
    XCTAssertEqual(myVerKey, getNymResponseData[@"verkey"], @"LedgerUtils::sendRequestWithPoolHandle() failed");

    // 9. Get GET_NYM response metadata
    NSString *getNymResponseMetadataJson = nil;
    ret = [[LedgerUtils sharedInstance] getResponseMetadata:getNymResponse
                                           responseMetadata:&getNymResponseMetadataJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::getResponseMetadata() failed");

    NSDictionary *getNymResponseMetadata = [NSDictionary fromString:getNymResponseMetadataJson];
    XCTAssertNotNil(getNymResponseMetadata[@"seqNo"], @"getNymResponseMetadata seqNo is empty");
    XCTAssertNotNil(getNymResponseMetadata[@"txnTime"], @"getNymResponseMetadata txnTime is empty");
    XCTAssertNotNil(getNymResponseMetadata[@"lastTxnTime"], @"getNymResponseMetadata lastTxnTime is empty");
    XCTAssertNil(getNymResponseMetadata[@"lastSeqNo"], @"getNymResponseMetadata lastSeqNo is not empty");
}

@end
