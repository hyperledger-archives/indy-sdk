//
//  Ledger.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "DidUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerHignCases : XCTestCase

@end

@implementation LedgerHignCases

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testSendRequestWorksForInvalidPoolHandle {
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_send_request_works_for_invalid_pool_handle";
    NSError *ret;

    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Obtain wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Obtain my DID

    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"," \
                            "\"cid\":true" \
                            "}"];
    NSString *myDid = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 4. Build GET NYM Request

    NSString *getNymRequest;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    NSLog(@"getNymRequest: %@", getNymRequest);

    // 5. Send request using invalid pool handle

    IndyHandle invalidPoolHandle = poolHandle + 1;
    NSString *getNymResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:invalidPoolHandle
                                                        request:getNymRequest
                                                       response:&getNymResponse];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
    NSLog(@"getNymResponse: %@", getNymResponse);

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testSignAndSubmitRequestWorksForInvalidPoolHandle {
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_sign_and_submit_request_works_for_invalid_pool_handle";
    NSError *ret;

    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Obtain wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Obtain trustee DID

    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                                 "\"seed\":\"000000000000000000000000Trustee1\"," \
                                 "\"cid\":true" \
                                 "}"];
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:trusteeDidJson
                                                        outMyDid:&trusteeDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);

    // 4. Obtain my DID

    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"00000000000000000000000000000My1\"" \
                            "}"];
    NSString *myDid = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"myDid: %@", myDid);


    // 5. Build NYM Request

    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    NSLog(@"nymRequest: %@", nymRequest);

    // 6. Send and submit request using invalid pool handle

    IndyHandle invalidPoolHandle = poolHandle + 1;
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:invalidPoolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
    NSLog(@"nymResponse: %@", nymResponse);

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testSignAndSubmitRequestWorksForInvalidWalletHandle {
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_sign_and_submit_request_works_for_invalid_wallet_handle";
    NSError *ret;

    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Obtain wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Obtain trustee DID

    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                                 "\"seed\":\"000000000000000000000000Trustee1\"," \
                                 "\"cid\":true" \
                                 "}"];
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:trusteeDidJson
                                                        outMyDid:&trusteeDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);

    // 4. Obtain my DID

    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"00000000000000000000000000000My1\"" \
                            "}"];
    NSString *myDid = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"myDid: %@", myDid);


    // 5. Build NYM Request

    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    NSLog(@"nymRequest: %@", nymRequest);

    // 6. Send and submit request using invalid wallet handle

    IndyHandle invalidWalletHandle = walletHandle + 1;
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:invalidWalletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
    NSLog(@"nymResponse: %@", nymResponse);

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

//- (void)testSignAndSubmitRequestWorksForNotFoundSigner
//{
//    [TestUtils cleanupStorage];
//    NSString *poolName = @"indy_sign_and_submit_request_works_for_not_found_signer";
//    NSString *walletName = @"wallet1";
//    NSString *xtype = @"default";
//    NSError *ret;
//    
//    // 1. Obtain pool handle
//    IndyHandle poolHandle = 0;
//    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
//                                                                 poolHandle:&poolHandle];
//    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");
//    
//    // 2. Obtain wallet handle
//    IndyHandle walletHandle = 0;
//    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
//                                                             walletName:walletName
//                                                                  xtype:xtype
//                                                                 handle:&walletHandle];
//    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
//    
//    // 3. Obtain my DID
//    
//    NSString * myDidJson = [NSString stringWithFormat:@"{"\
//                            "\"seed\":\"00000000000000000000000000000My1\"" \
//                            "}"];
//    NSString *myDid = nil;
//    
//    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
//                                                          myDidJson:myDidJson
//                                                           outMyDid:&myDid
//                                                        outMyVerkey:nil];
//                                                            ];
//    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
//    NSLog(@"myDid: %@", myDid);
//    
//    
//    // 4. Build NYM Request
//    
//    NSString *trusteeDid = @"some_trustee_did";
//    NSString *nymRequest;
//    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
//                                                              targetDid:myDid
//                                                                 verkey:nil
//                                                                  alias:nil
//                                                                   role:nil
//                                                             outRequest:&nymRequest];
//    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
//    NSLog(@"nymRequest: %@", nymRequest);
//    
//    // 5. Sign and submit request
//    
//    NSString *nymResponse;
//    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
//                                                              walletHandle:walletHandle
//                                                              submitterDid:trusteeDid
//                                                               requestJson:nymRequest
//                                                           outResponseJson:&nymResponse];
//    XCTAssertEqual(ret.code, WalletNotFoundError, @"PoolUtils::signAndSubmitRequestWithPoolHandle() returned invalid error");
//    NSLog(@"nymResponse: %@", nymResponse);
//    
//    [TestUtils cleanupStorage];
//}

- (void)testSignAndSubmitRequestWorksForIncompatibleWalletAndPool {
    [TestUtils cleanupStorage];
    NSString *poolName1 = [TestUtils pool];
    NSString *poolName2 = @"pool2";
    NSError *ret;

    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName1
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Obtain wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName2
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Obtain my DID

    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"00000000000000000000000000000My1\"" \
                            "}"];
    NSString *myDid = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    // 4. Obtain trustee did
    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"," \
                            "\"cid\":true"\
                            "}"];
    NSString *trusteeDid = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:trusteeDidJson
                                                        outMyDid:&trusteeDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");


    // 4. Build NYM Request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    NSLog(@"nymRequest: %@", nymRequest);


    // 5. Sign and submit request

    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, WalletIncompatiblePoolError, @"PoolUtils::signAndSubmitRequestWithPoolHandle() returned invalid error");
    NSLog(@"nymResponse: %@", nymResponse);

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testSubmitRequestWorks {
    [TestUtils cleanupStorage];
    NSString *poolName = @"test_submit_tx";
    NSError *ret;

    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName failed");

    NSString *request = [NSString stringWithFormat:@"{"
            "\"reqId\":1491566332010860,"
            "\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\","
            "\"operation\":{"
            "\"type\":\"105\","
            "\"dest\":\"Th7MpTaRZVRYnPiabds81Y\"},"
            "\"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\""
            "}"];

    NSString *responseJson;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:request
                                                       response:&responseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed!");
    NSLog(@"responseJson: %@", responseJson);

    NSDictionary *actualReply = [NSDictionary fromString:responseJson];

    NSDictionary *actualData = [NSDictionary fromString:actualReply[@"result"][@"data"]];
    XCTAssertTrue([actualReply[@"op"] isEqualToString:@"REPLY"], @"Wrong actualReply[op]");
    XCTAssertTrue([actualReply[@"result"][@"reqId"] isEqualToValue:@(1491566332010860)], @"Wrong actualReply[reqId]");

    XCTAssertTrue([actualData[@"dest"] isEqualToString:@"Th7MpTaRZVRYnPiabds81Y"], @"Wrong actualData[dest]");
    XCTAssertTrue([actualData[@"identifier"] isEqualToString:@"V4SGRU86Z58d6TV7PBUe6f"], @"Wrong actualData[identifier]");
    XCTAssertTrue([actualData[@"role"] isEqualToString:@"2"], @"Wrong actualData[role]");
    XCTAssertTrue([actualData[@"verkey"] isEqualToString:@"~7TYfekw4GUagBnBVCqPjiC"], @"Wrong actualData[verkey]");

    XCTAssertTrue([actualReply[@"result"][@"identifier"] isEqualToString:@"Th7MpTaRZVRYnPiabds81Y"], @"Wrong actualReply[identifier]");
    XCTAssertTrue([actualReply[@"result"][@"dest"] isEqualToString:@"Th7MpTaRZVRYnPiabds81Y"], @"Wrong dest");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testSignAndSubmitRequestWorks {
    [TestUtils cleanupStorage];

    NSError *ret;
    NSString *poolName = @"indy_sign_and_submit_request_works";

    // 1. create and open pool
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed!");

    // 2. create and open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");

    // 3. create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:nil
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed!");

    // 4. create and store trustee did
    NSString *trusteeDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed!");

    // 5. Build nym request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed!");

    // 6. sign and submit nym request
    NSString *nymResponceJson;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponceJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Sign Request

- (void)testSignRequestWorks {
    [TestUtils cleanupStorage];
    NSError *ret;
    // 1. create and open wallet

    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");

    // 2. Create and store my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&myDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed!");

    NSString *message = @"{"
            "\"reqId\":1496822211362017764,"
            "\"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\","
            "\"operation\":{"
            "\"type\":\"1\","
            "\"dest\":\"VsKV7grR1BUE29mG2Fm2kX\","
            "\"verkey\":\"GjZWsBLgZCR18aL468JAT7w9CZRiBnpxUPPgyQxh4voa\""
            "}"
            "}";

    NSMutableDictionary *expectedSignature = [NSMutableDictionary new];
    expectedSignature[@"signature"] = @"65hzs4nsdQsTUqLCLy2qisbKLfwYKZSWoyh1C6CU59p5pfG3EHQXGAsjW4Qw4QdwkrvjSgQuyv8qyABcXRBznFKW";

    // 3. Sign Request

    NSString *resultJson;
    ret = [[LedgerUtils sharedInstance] signRequestWithWalletHandle:walletHandle
                                                       submitterdid:myDid
                                                        requestJson:message
                                                         resultJson:&resultJson];

    NSDictionary *result = [NSDictionary fromString:resultJson];
    XCTAssertTrue([result contains:expectedSignature], @"Wrong Result Json!");

    [TestUtils cleanupStorage];
}

- (void)testSignWorksForUnknownSigner {
    [TestUtils cleanupStorage];
    NSError *ret;

    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");

    NSString *message = @"{\"reqId\":1495034346617224651}";

    ret = [[LedgerUtils sharedInstance] signRequestWithWalletHandle:walletHandle
                                                       submitterdid:@"did"
                                                        requestJson:message
                                                         resultJson:nil];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"LedgerUtils::signRequestWithWalletHandle() returned wrong code!");

    [TestUtils cleanupStorage];
}

- (void)testSignRequestWorksFowInvalidMessageFormat {
    [TestUtils cleanupStorage];
    NSError *ret;

    // 1. create and open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed!");

    // 2. create my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed!");

    NSString *message = @"1495034346617224651";

    ret = [[LedgerUtils sharedInstance] signRequestWithWalletHandle:walletHandle
                                                       submitterdid:myDid
                                                        requestJson:message
                                                         resultJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils::signRequestWithWalletHandle() returned wrong code!");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];

    [TestUtils cleanupStorage];
}

- (void)testSignRequestWorksForInvalidHandle {
    [TestUtils cleanupStorage];
    NSError *ret;

    // 1. create and open wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() returned wrong code!");

    // 2. create my did
    NSString *myDid;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed!");

    NSString *message = @"{\"reqId\":1495034346617224651}";

    ret = [[LedgerUtils sharedInstance] signRequestWithWalletHandle:walletHandle + 1
                                                       submitterdid:myDid
                                                        requestJson:message
                                                         resultJson:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"LedgerUtils::signRequestWithWalletHandle() returned wrong code!");

    [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];

    [TestUtils cleanupStorage];
}

// MARK: - NYM Requests

- (void)testBuildNymRequestsWorksForOnlyRequiredFields {
    [TestUtils cleanupStorage];

    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
    NSError *ret;

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:identifier
                                                              targetDid:dest
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"identifier"] = identifier;
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"1";
    expectedResult[@"operation"][@"dest"] = dest;

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
    [TestUtils cleanupStorage];
}

- (void)testBuildNymRequestsWorksWithOptionFields {
    [TestUtils cleanupStorage];

    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
    NSString *verkey = @"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL";
    NSString *role = @"STEWARD";
    NSString *alias = @"some_alias";
    NSError *ret;

    NSString *requestJson;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:identifier
                                                              targetDid:dest
                                                                 verkey:verkey
                                                                  alias:alias
                                                                   role:role
                                                             outRequest:&requestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed!");

    NSDictionary *request = [NSDictionary fromString:requestJson];

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"identifier"] = identifier;
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"1";
    expectedResult[@"operation"][@"dest"] = dest;
    expectedResult[@"operation"][@"verkey"] = verkey;
    expectedResult[@"operation"][@"alias"] = alias;
    expectedResult[@"operation"][@"role"] = @"2";

    XCTAssertTrue([request contains:expectedResult], @"Request doesn't contain expectedResult");
    [TestUtils cleanupStorage];
}

- (void)testBuildGetNymRequestWorks {
    [TestUtils cleanupStorage];
    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"identifier"] = identifier;
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"105";
    expectedResult[@"operation"][@"dest"] = dest;

    NSString *getNymRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:identifier
                                                                          targetDid:dest
                                                                         outRequest:&getNymRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed!");

    NSDictionary *getNymRequest = [NSDictionary fromString:getNymRequestJson];
    XCTAssertTrue([getNymRequest contains:expectedResult], @"Request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

- (void)testNymRequestWorksWithoutSignature {
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_nym_request_works_without_signature";
    NSError *ret;

    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Obtain wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Obtain my DID

    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"," \
                            "\"cid\":true"\
                            "}"];
    NSString *myDid = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"myDid: %@", myDid);


    // 4. Build NYM Request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:myDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    NSLog(@"nymRequest: %@", nymRequest);


    // 5. Send request

    NSString *nymResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:nymRequest
                                                       response:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::sendRequestWithPoolHandle() returned not Success");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:nymResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testSendGetNymRequestWorks {
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_send_get_nym_request_works";
    NSError *ret;

    // 1. Obtain pool handle
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Obtain wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Obtain my DID

    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"," \
                            "\"cid\":true"\
                            "}"];
    NSString *myDid = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"myDid: %@", myDid);

    // 4. Build get NYM Request

    NSString *getNymRequest;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    NSLog(@"getNymRequest: %@", getNymRequest);

    // 5. Send request

    NSString *getNymResponseJson;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getNymRequest
                                                       response:&getNymResponseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    NSLog(@"getNymResponseJson: %@", getNymResponseJson);

    NSDictionary *getNymResponse = [NSDictionary fromString:getNymResponseJson];

    XCTAssertNotNil(getNymResponse[@"result"][@"data"], @"getNymResponse data is empty");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}


- (void)testNymRequestsWorks {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_nym_requests_works";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. Obtain trustee did
    NSString *trusteeDid = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");

    // 4. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:nil
                                                                outMyDid:&myDid
                                                             outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed");
    XCTAssertTrue([myDid isValid], @"myDid is invalid!");
    XCTAssertTrue([myVerKey isValid], @"myVerKey is invalid!");

    // 5. Build nym request

    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");

    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    // 7. Build get nym request

    NSString *getNymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(getNymRequest, @"getNymRequest is nil!");

    // 8. Send getNymRequest

    NSString *getNymResponseJson = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getNymRequest
                                                       response:&getNymResponseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getNymResponseJson, @"getNymResponseJson is nil!");

    NSDictionary *getNymResponse = [NSDictionary fromString:getNymResponseJson];
    XCTAssertTrue([[getNymResponse allKeys] count] > 0, @"getNymResponse is empty");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Attribute requests

- (void)testBuildAttribRequestsWorksForRawData {
    [TestUtils cleanupStorage];
    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *raw = @"{"\
    "\"endpoint\":{"\
    "\"ha\":\"127.0.0.1:5555\"}"\
    "}";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"identifier"] = identifier;
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"100";
    expectedResult[@"operation"][@"dest"] = dest;
    expectedResult[@"operation"][@"raw"] = @"{"\
    "\"endpoint\":{"\
    "\"ha\":\"127.0.0.1:5555\"}"\
    "}";
    NSString *attribRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:identifier
                                                                          targetDid:dest
                                                                               hash:nil
                                                                                raw:raw
                                                                                enc:nil
                                                                         resultJson:&attribRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");

    NSDictionary *attribRequest = [NSDictionary fromString:attribRequestJson];
    XCTAssertTrue([attribRequest contains:expectedResult], @"attribRequest doesn't contains expectedResult!");
    [TestUtils cleanupStorage];
}

- (void)testBuildAttribRequestsWorksForMissedAttribute {
    [TestUtils cleanupStorage];
    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"Th7MpTaRZVRYnPiabds81Y";

    NSString *attribRequest;
    NSError *ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:identifier
                                                                          targetDid:dest
                                                                               hash:nil
                                                                                raw:nil
                                                                                enc:nil
                                                                         resultJson:&attribRequest];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils::buildAttribRequestWithSubmitterDid() returned wrong error");

    [TestUtils cleanupStorage];
}

- (void)testBuildGetAttribRequestsWorks {
    [TestUtils cleanupStorage];
    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *raw = @"endpoint";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"identifier"] = identifier;
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"104";
    expectedResult[@"operation"][@"dest"] = dest;
    expectedResult[@"operation"][@"raw"] = raw;

    NSString *getAttribRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:identifier
                                                                             targetDid:dest
                                                                                   raw:raw
                                                                                  hash:nil
                                                                                   enc:nil
                                                                            resultJson:&getAttribRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequestWithSubmitterDid() returned wrong error");

    NSDictionary *request = [NSDictionary fromString:getAttribRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expextedresult");

    [TestUtils cleanupStorage];
}

- (void)testAttribRequestWorksWithoutSignature {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_attrib_request_works_without_signature";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");


    // 3. Obtain my did
    NSString *myDid = nil;
    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"" \
                           "}"];

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    XCTAssertNotNil(myDid, @"myDid is nil!");

    // 4. Build attrib request

    NSString *attribRequest = nil;
    NSString *raw = @"{"\
    "\"endpoint\":{"\
    "\"ha\":\"127.0.0.1:5555\"}}";
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                      hash:nil
                                                                       raw:raw
                                                                       enc:nil
                                                                resultJson:&attribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");
    XCTAssertNotNil(attribRequest, @"nymRequestResult is nil!");

    // 5. Send request
    NSString *attribResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:attribRequest
                                                       response:&attribResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::testAttribRequestWorksWithoutSignature() returned not Success");
    XCTAssertNotNil(attribResponse, @"attribResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:attribResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testAttributeRequestsWorks {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_attrib_requests_works";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWallet failed");

    // 3. Obtain trustee did
    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");

    // 4. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:nil
                                                                outMyDid:&myDid
                                                             outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for myDid");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");

    // 5. Build nym request

    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");

    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    // 7. Build attrib request
    NSString *rawJson = [NSString stringWithFormat:@"{"\
                         "\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}" \
                         "}"];

    NSString *attribRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                      hash:nil
                                                                       raw:rawJson
                                                                       enc:nil
                                                                resultJson:&attribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");
    XCTAssertNotNil(attribRequest, @"attribRequest is nil!");

    // 8. Sign and Submit attrib request
    NSString *attribResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:attribRequest
                                                           outResponseJson:&attribResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(attribResponse, @"attribResponse is nil!");

    // 9. Build getAttribRequest
    NSString *getAttribRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:myDid
                                                                    targetDid:myDid
                                                                          raw:@"endpoint"
                                                                         hash:nil
                                                                          enc:nil
                                                                   resultJson:&getAttribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequest() failed");
    XCTAssertNotNil(getAttribRequest, @"getAttribRequest is nil!");

    // 10. Send getAttribRequest
    NSString *getAttribResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getAttribRequest
                                                       response:&getAttribResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getAttribResponse, @"getAttribResponse is nil!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Schema request



// MARK: - Node request

- (void)testBuildNodeRequestWorksForCorrectDataJson {
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *dest = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
    NSString *data = @"{"\
    "\"node_ip\":\"ip\","\
    "\"node_port\":1,"\
    "\"client_ip\":\"ip\","\
    "\"client_port\":1,"\
    "\"alias\":\"some\","\
    "\"services\":[\"VALIDATOR\"],"
            "\"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"identifier"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"0";
    expectedResult[@"operation"][@"dest"] = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"data"][@"node_ip"] = @"ip";
    expectedResult[@"operation"][@"data"][@"node_port"] = @(1);
    expectedResult[@"operation"][@"data"][@"client_ip"] = @"ip";
    expectedResult[@"operation"][@"data"][@"client_port"] = @(1);
    expectedResult[@"operation"][@"data"][@"alias"] = @"some";
    expectedResult[@"operation"][@"data"][@"services"] = [[NSArray alloc] initWithObjects:@"VALIDATOR", nil];
    expectedResult[@"operation"][@"data"][@"blskey"] = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";


    NSString *nodeRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:identifier
                                                                        targetDid:dest
                                                                             data:data
                                                                       resultJson:&nodeRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNodeRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nodeRequestJson, @"schemaRequestJson is nil!");
    NSLog(@"nodeRequestJson: %@", nodeRequestJson);

    NSDictionary *request = [NSDictionary fromString:nodeRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

- (void)testSendNodeRequestWorksWithoutSignature {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_send_node_request_works_without_signature";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");


    // 3. Obtain my did
    NSString *myDid = nil;
    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000Steward1\"" \
                           "}"];

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");

    // 4. Build node request

    NSString *nodeData = @"{"\
    "\"node_ip\":\"10.0.0.100\","\
    "\"node_port\":9710,"\
    "\"client_ip\":\"10.0.0.100\","\
    "\"client_port\":9709,"\
    "\"alias\":\"Node5\","
            "\"services\":[\"VALIDATOR\"],"
            "\"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\"}";

    NSString *nodeRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:myDid
                                                               targetDid:myDid
                                                                    data:nodeData
                                                              resultJson:&nodeRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNodeRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nodeRequest, @"nodeRequest is nil!");

    // 5. Send request
    NSString *nodeResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:nodeRequest
                                                       response:&nodeResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not Success");
    XCTAssertNotNil(nodeResponse, @"nodeResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:nodeResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// Warning: Enable when you need to run this test. It breaks pool after run.

- (void)testSubmitNodeRequestWorksForNewSteward {
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_submit_node_request_works_for_new_steward";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWallet failed");

    // 3. Obtain trustee did
    NSString *trusteeDid = nil;
    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                                "\"seed\":\"000000000000000000000000Trustee1\"," \
                                "\"cid\":true"\
                                "}"];

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:trusteeDidJson
                                                        outMyDid:&trusteeDid
                                                     outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");

    // 4. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"," \
                           "\"cid\":true"\
                           "}"];

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");

    // 5. Build nym request

    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:@"STEWARD"
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");

    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    // 7. Build node request

    NSString *nodeData = @"{"\
    "\"node_ip\":\"10.0.0.100\","\
    "\"node_port\":9710,"\
    "\"client_ip\":\"10.0.0.100\","\
    "\"client_port\":9709,"\
    "\"alias\":\"Node5\","\
    "\"services\":[\"VALIDATOR\"]}";

    NSString *dest = @"A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y"; // random(32) and base58
    NSString *nodeRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:myDid
                                                               targetDid:dest
                                                                    data:nodeData
                                                              resultJson:&nodeRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNodeRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nodeRequest, @"nodeRequest is nil!");

    // 8. Sign and submit request
    NSString *nodeResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:nodeRequest
                                                           outResponseJson:&nodeResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(nodeResponse, @"nodeResponse is nil!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Cred def request

- (void)testBuildCredDefRequestWorksForCorrectDataJson {
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *data = @"{"
            "\"ver\":\"1.0\","
            "\"id\":\"cred_def_id\","
            "\"schemaId\":\"1\","
            "\"type\":\"CL\","
            "\"tag\":\"rag1\","
            "\"value\":{"
            "\"primary\":{"
            "\"n\":\"1\","
            "\"s\":\"2\","
            "\"rms\":\"3\","
            "\"r\":{"
            "\"height\":\"1\"},"
            "\"rctxt\":\"1\","
            "\"z\":\"1\""
            "}"
            "}"
            "}";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];

    expectedResult[@"identifier"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"ref"] = @(1);
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];

    NSMutableDictionary *primary = [NSMutableDictionary new];
    primary[@"n"] = @"1";
    primary[@"s"] = @"2";
    primary[@"rms"] = @"3";
    primary[@"r"] = [NSMutableDictionary new];
    primary[@"r"][@"height"] = @"1";
    primary[@"rctxt"] = @"1";
    primary[@"z"] = @"1";

    expectedResult[@"operation"][@"data"][@"primary"] = primary;
    expectedResult[@"operation"][@"data"][@"type"] = @"102";
    expectedResult[@"operation"][@"data"][@"signature_type"] = @"CL";

    NSString *credDefrequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildCredDefRequestWithSubmitterDid:identifier
                                                                                data:data
                                                                          resultJson:&credDefrequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildCredDefRequestWithSubmitterDid() failed");
    XCTAssertNotNil(credDefrequestJson, @"credDefrequestJson is nil!");
    NSLog(@"credDefrequestJson: %@", credDefrequestJson);

    NSDictionary *request = [NSDictionary fromString:credDefrequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");

    [TestUtils cleanupStorage];
}

- (void)testBuildGetCredDefRequestWorks {
    [TestUtils cleanupStorage];
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *id = @"NcYxiDXkpYi6ov5FcYDi1e:03:CL:1";

    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"identifier"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"108";
    expectedResult[@"operation"][@"ref"] = @(1);
    expectedResult[@"operation"][@"signature_type"] = @"CL";
    expectedResult[@"operation"][@"origin"] = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSString *getCredDefRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetCredDefRequestWithSubmitterDid:identifier
                                                                                     id:id
                                                                             resultJson:&getCredDefRequestJson];

    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetCredDefRequestWithSubmitterDid() failed");

    NSDictionary *request = [NSDictionary fromString:getCredDefRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
    [TestUtils cleanupStorage];
}

- (void)testCredDefRequestWorksWithoutSignature {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_cred_def_request_works_without_signature";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. Obtain trustee did

    NSString *trusteeDid = nil;

    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);

    // 4. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:@"{}"
                                                        outMyDid:&myDid
                                                     outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");

    // 5. Build nym request

    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:@"TRUSTEE"
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");

    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    // 7. Build schema request
    __block NSString *schemaId;
    __block NSString *schemaJson;
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
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");

    // 8. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");

    // 9. Build getSchemaRequest
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                           id:schemaId
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");
    XCTAssertNotNil(getSchemaRequest, @"getSchemaRequest is nil!");

    // 10. Send getSchemaRequest
    NSString *getSchemaResponse = nil;

    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getSchemaRequest
                                                       response:&getSchemaResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getSchemaResponse, @"getSchemaResponse is nil!");

    ret = [[LedgerUtils sharedInstance] parseGetSchemaResponse:getSchemaResponse
                                                      schemaId:&schemaId
                                                    schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaId, @"schemaResponse is nil!");
    XCTAssertNotNil(schemaJson, @"schemaResponse is nil!");

    // 11. Create credential definition
    __block NSString *credentialDefId;
    __block NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:myDid
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:walletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinitionWithWalletHandle failed");
    XCTAssertTrue([credentialDefId isValid], @"invalid credentialDefId: %@", credentialDefId);
    XCTAssertTrue([credentialDefJSON isValid], @"invalid credentialDefJSON: %@", credentialDefJSON);

    // 12. Build credential def request
    NSString *credDefRequestJson;
    ret = [[LedgerUtils sharedInstance] buildCredDefRequestWithSubmitterDid:myDid
                                                                       data:credentialDefJSON
                                                                 resultJson:&credDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::buildCredDefRequestWithSubmitterDid() failed");
    XCTAssertNotNil(credDefRequestJson, @"credDefRequestJson is nil!");


    // 13. Sign and submit cred def request

    NSString *credDefResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:credDefRequestJson
                                                       response:&credDefResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not Success");
    XCTAssertNotNil(credDefResponse, @"credDefResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:credDefResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testCredDefRequestsWorks {
    [TestUtils cleanupStorage];

    NSString *poolName = @"indy_cred_def_requests_works";
    NSError *ret = nil;

    // 1. Create and open pool ledger config, get pool handle
    IndyHandle poolHandle = 0;

    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");

    // 2. Create and open wallet, get wallet handle
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");

    // 3. Obtain trustee did

    NSString *trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                    seed:@"000000000000000000000000Trustee1"
                                                                outMyDid:&trusteeDid
                                                             outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trusteeDid");
    NSLog(@"trusteeDid: %@", trusteeDid);

    // 4. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    NSString *myDidJson = [NSString stringWithFormat:@"{}"];

    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                       myDidJson:myDidJson
                                                        outMyDid:&myDid
                                                     outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");

    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");

    // 5. Build nym request

    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:@"TRUSTEE"
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");

    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    // 7. Build schema request
    __block NSString *schemaId;
    __block NSString *schemaJson;
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
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");

    // 8. Sign and submit schema request
    NSString *schemaResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");

    // 9. Build getSchemaRequest
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                           id:schemaId
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");
    XCTAssertNotNil(getSchemaRequest, @"getSchemaRequest is nil!");

    // 10. Send getSchemaRequest
    NSString *getSchemaResponse = nil;

    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getSchemaRequest
                                                       response:&getSchemaResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getSchemaResponse, @"getSchemaResponse is nil!");

    ret = [[LedgerUtils sharedInstance] parseGetSchemaResponse:getSchemaResponse
                                                      schemaId:&schemaId
                                                    schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaId, @"schemaResponse is nil!");
    XCTAssertNotNil(schemaJson, @"schemaResponse is nil!");

    // 11. Create credential definition
    __block NSString *credentialDefId;
    __block NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:myDid
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:walletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinitionWithWalletHandle failed");
    XCTAssertTrue([credentialDefId isValid], @"invalid credentialDefId: %@", credentialDefId);
    XCTAssertTrue([credentialDefJSON isValid], @"invalid credentialDefJSON: %@", credentialDefJSON);

    // 12. Build credential def request
    NSString *credDefRequestJson;
    ret = [[LedgerUtils sharedInstance] buildCredDefRequestWithSubmitterDid:myDid
                                                                       data:credentialDefJSON
                                                                 resultJson:&credDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::buildCredDefRequestWithSubmitterDid() failed");
    XCTAssertNotNil(credDefRequestJson, @"credDefRequestJson is nil!");


    // 13. Sign and submit credential def request

    NSString *credDefResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:credDefRequestJson
                                                           outResponseJson:&credDefResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not Success");
    XCTAssertNotNil(credDefResponse, @"credDefResponse is nil!");


    // 14. Build get credential def request
    NSString *getCredDefRequest;
    ret = [[LedgerUtils sharedInstance] buildGetCredDefRequestWithSubmitterDid:myDid
                                                                            id:credentialDefId
                                                                    resultJson:&getCredDefRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetCredDefRequestWithSubmitterDid() failed");
    XCTAssertNotNil(getCredDefRequest, @"getCredDefRequest is nil!");

    // 15. Send getCredDefRequest
    NSString *getCredDefResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getCredDefRequest
                                                       response:&getCredDefResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getCredDefResponse, @"getCredDefResponse is nil!");

    ret = [[LedgerUtils sharedInstance] parseGetCredDefResponse:getCredDefResponse
                                                      credDefId:&credentialDefId
                                                    credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::parseGetCredDefResponse() failed");
    XCTAssertNotNil(credentialDefId, @"credentialDefId is nil!");
    XCTAssertNotNil(credentialDefJSON, @"credentialDefJSON is nil!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Get txn request

- (void)testBuildGetTxnRequest {
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSNumber *data = @(1);

    NSString *extectedResultJson = @"{\"identifier\":\"NcYxiDXkpYi6ov5FcYDi1e\","
            "\"operation\":{\"type\":\"3\",\"data\":1}}";

    NSDictionary *expectedResult = [NSDictionary fromString:extectedResultJson];

    NSString *getTxnRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetTxnRequestWithSubmitterDid:identifier
                                                                               data:data
                                                                         resultJson:&getTxnRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSDictionary *getTxnRequest = [NSDictionary fromString:getTxnRequestJson];

    XCTAssertTrue([getTxnRequest contains:expectedResult], @"getTxnRequest json doesn't contain expectedResult json");
}


// TODO - Still does not pass
- (void)testGetTxnRequestWorks {
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = @"indy_get_txn_request_works";

    // 1. Create and open pool ledger config
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Create and open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Create my did
    NSString *myDid = [[DidUtils sharedInstance] createStoreAndPublishMyDidWithWalletHandle:walletHandle
                                                                                 poolHandle:poolHandle];

    // 4. Build schema data json
    NSString *schemaDataJson = @"{\"id\":\"id\", \"name\":\"name\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";

    // 5. Build & submit schema request
    NSString *schemaRequest;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaDataJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");

    NSString *schemaResponseJson;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponseJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");

    // 9. Build & submit get txn request

    NSDictionary *schemaResponse = [NSDictionary fromString:schemaResponseJson];
    NSNumber *seqNo = (NSNumber *) schemaResponse[@"result"][@"seqNo"];

    NSString *getTxnRequest;
    ret = [[LedgerUtils sharedInstance] buildGetTxnRequestWithSubmitterDid:myDid
                                                                      data:seqNo
                                                                resultJson:&getTxnRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSString *getTxnResponseJson;
    ret = [[LedgerUtils sharedInstance] submitRequest:getTxnRequest
                                       withPoolHandle:poolHandle
                                           resultJson:&getTxnResponseJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::submitRequest() failed for getTxnRequest: %@", getTxnRequest);

    // 10. Check getTxnResponse
    NSDictionary *getTxnResponse = [NSDictionary fromString:getTxnResponseJson];

    NSDictionary *getTxnSchemaResult = getTxnResponse[@"result"][@"data"];
    XCTAssertNotNil(getTxnSchemaResult[@"data"], @"getTxnSchemaResult[data] is nil");
    XCTAssertNotNil(getTxnSchemaResult[@"seqNo"], @"getTxnSchemaResult[seqNo] is nil");

    NSDictionary *getTxnSchemaData = getTxnSchemaResult[@"data"];

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// TODO: Still doesn't work
- (void)testGetTxnRequestWorksForInvalidSeqNo {
    [TestUtils cleanupStorage];
    NSError *ret;
    NSString *poolName = @"indy_get_txn_request_works_for_invalid_seq_no";

    // 1. Create and open pool ledger config
    IndyHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerWithPoolName:poolName
                                                               poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerWithPoolName() failed");

    // 2. Create and open wallet
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");

    // 3. Create my did
    NSString *myDid = [[DidUtils sharedInstance] createStoreAndPublishMyDidWithWalletHandle:walletHandle
                                                                                 poolHandle:poolHandle];

    // 4. Build schema data json
    NSString *schemaDataJson = @"{\"id\":\"id\", \"name\":\"name\",\"version\":\"1.0\",\"attrNames\":[\"name\"],\"ver\":\"1.0\"}";

    // 5. Build & submit schema request
    NSString *schemaRequest;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaDataJson
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");

    NSString *schemaResponseJson;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:schemaRequest
                                                           outResponseJson:&schemaResponseJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");

    // 9. Build & submit get txn request

    NSDictionary *schemaResponse = [NSDictionary fromString:schemaResponseJson];
    NSNumber *seqNo = (NSNumber *) schemaResponse[@"result"][@"seqNo"];
    seqNo = [NSNumber numberWithInt:[seqNo intValue] + 10];

    NSString *getTxnRequest;
    ret = [[LedgerUtils sharedInstance] buildGetTxnRequestWithSubmitterDid:myDid
                                                                      data:seqNo
                                                                resultJson:&getTxnRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSString *getTxnResponseJson;
    ret = [[LedgerUtils sharedInstance] submitRequest:getTxnRequest
                                       withPoolHandle:poolHandle
                                           resultJson:&getTxnResponseJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::submitRequest() failed for getTxnRequest: %@", getTxnRequest);

    // 10. Check getTxnResponse
    NSDictionary *getTxnResponse = [NSDictionary fromString:getTxnResponseJson];

    XCTAssertTrue([getTxnResponse[@"result"][@"data"] isEqual:[NSNull null]], @"data field in getTxnResponse shall be nil");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

// MARK: - Revoc Reg def request

- (void)testBuildRevocRegDefRequestWorks {
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *data = @"{\n"
            "        \"ver\": \"1.0\",\n"
            "        \"id\": \"RevocRegID\",\n"
            "        \"revocDefType\": \"CL_ACCUM\",\n"
            "        \"tag\": \"TAG1\",\n"
            "        \"credDefId\": \"CredDefID\",\n"
            "        \"value\": {\n"
            "            \"issuanceType\": \"ISSUANCE_ON_DEMAND\",\n"
            "            \"maxCredNum\": 5,\n"
            "            \"tailsHash\": \"s\",\n"
            "            \"tailsLocation\": \"http://tails.location.com\",\n"
            "            \"publicKeys\": {\n"
            "                \"accumKey\": {\n"
            "                    \"z\": \"1111 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"\n"
            "                }\n"
            "            }\n"
            "        }\n"
            "    }";

    NSString *extectedResultJson = @"\"operation\":{\"type\":\"113\",\"id\":\"RevocRegID\",\"revocDefType\":\"CL_ACCUM\",\"tag\":\"TAG_1\",\"credDefId\":\"CredDefID\",\"value\":{\"issuanceType\":\"ISSUANCE_ON_DEMAND\",\"maxCredNum\":5,\"publicKeys\":{\"accumKey\":{\"z\":\"1111 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"}},\"tailsHash\":\"s\",\"tailsLocation\":\"http://tails.location.com\"}}";

    NSDictionary *expectedResult = [NSDictionary fromString:extectedResultJson];

    NSString *revocRegDefRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildRevocRegDefRequestWithSubmitterDid:identifier
                                                                                    data:data
                                                                              resultJson:&revocRegDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSDictionary *revocRegDefReques = [NSDictionary fromString:revocRegDefRequestJson];

    XCTAssertTrue([revocRegDefReques contains:expectedResult], @"getTxnRequest json doesn't contain expectedResult json");
}

- (void)testBuildGetRevocRegDefRequestWorks {
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *id = @"RevocRegID";

    NSString *extectedResultJson = @"\"operation\": {\n"
            "            \"type\": \"115\",\n"
            "            \"id\": RevocRegID\n"
            "        }";

    NSDictionary *expectedResult = [NSDictionary fromString:extectedResultJson];

    NSString *getRevocRegDefRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetRevocRegDefRequestWithSubmitterDid:identifier
                                                                                         id:id
                                                                                 resultJson:&getRevocRegDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetTxnRequestWithSubmitterDid() failed");

    NSDictionary *getRevocRegDefRequest = [NSDictionary fromString:getRevocRegDefRequestJson];

    XCTAssertTrue([getRevocRegDefRequest contains:expectedResult], @"getTxnRequest json doesn't contain expectedResult json");
}

// MARK: - Revoc Reg Entry request

- (void)testBuildRevocRegEntryRequestWorks {
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *data = @"{\"value\":{\"accum\":\"false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"}, \"ver\":\"1.0\"}";

    NSString *extectedResultJson = @"\"operation\":{\"type\":\"114\",\"revocRegDefId\":\"RevocRegID\",\"revocDefType\":\"CL_ACCUM\",\"value\":{\"accum\":\"false 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\"}}";

    NSDictionary *expectedResult = [NSDictionary fromString:extectedResultJson];

    NSString *revocRegEntryRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildRevocRegEntryRequestWithSubmitterDid:identifier
                                                                                      type:@"CL_ACCUM"
                                                                             revocRegDefId:@"RevocRegID"
                                                                                     value:data
                                                                                resultJson:&revocRegEntryRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildRevocRegEntrtyRequestWithSubmitterDid() failed");

    NSDictionary *revocRegEntryReques = [NSDictionary fromString:revocRegEntryRequestJson];

    XCTAssertTrue([revocRegEntryReques contains:expectedResult], @"revocRegEntryReques json doesn't contain expectedResult json");
}

// MARK: - Revoc Reg request

- (void)testBuildGetRevocRegRequestWorks {
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSString *extectedResultJson = @"\"operation\":{\"type\":\"116\",\"revocRegDefId\":\"RevRegId\",\"timestamp\":100}";

    NSDictionary *expectedResult = [NSDictionary fromString:extectedResultJson];

    NSString *getRevocRegRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetRevocRegRequestWithSubmitterDid:identifier
                                                                           revocRegDefId:@"RevRegId"
                                                                               timestamp:@(100)
                                                                              resultJson:&getRevocRegRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetRevocRegRequestWithSubmitterDid() failed");

    NSDictionary *getRevocRegReques = [NSDictionary fromString:getRevocRegRequestJson];

    XCTAssertTrue([getRevocRegReques contains:expectedResult], @"getRevocRegReques json doesn't contain expectedResult json");
}

// MARK: - Revoc Reg Delta request

- (void)testBuildGetRevocRegDeltaRequestWorks {
    NSString *identifier = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSString *extectedResultJson = @"\"operation\":{\"type\":\"117\",\"revocRegDefId\":\"RevRegId\",\"from\":0,\"to\":100}";

    NSDictionary *expectedResult = [NSDictionary fromString:extectedResultJson];

    NSString *getRevocRegDeltaRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetRevocRegDeltaRequestWithSubmitterDid:identifier
                                                                                revocRegDefId:@"RevRegId"
                                                                                         from:@(0)
                                                                                           to:@(100)
                                                                                   resultJson:&getRevocRegDeltaRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetRevocRegRequestWithSubmitterDid() failed");

    NSDictionary *getRevocRegDeltaReques = [NSDictionary fromString:getRevocRegDeltaRequestJson];

    XCTAssertTrue([getRevocRegDeltaReques contains:expectedResult], @"getRevocRegDeltaReques json doesn't contain expectedResult json");
}
@end
