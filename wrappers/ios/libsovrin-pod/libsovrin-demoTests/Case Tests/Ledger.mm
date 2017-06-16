//
//  Ledger.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 02.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <libsovrin/libsovrin.h>
#import "NSDictionary+JSON.h"

@interface Ledger : XCTestCase

@end

@implementation Ledger

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void) testSendRequestWorksForInvalidPoolHandle
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_send_request_works_for_invalid_pool_handle";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Obtain wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain my DID
    
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"," \
                            "\"cid\":true" \
                            "}"];
    NSString *myDid = nil;
    // NSString *verKey = nil;
    // NSString *pk = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    // 4. Build GET NYM Request
    
    NSString *getNymRequest;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    NSLog(@"getNymRequest: %@", getNymRequest);
    
    // 5. Send request using invalid pool handle
    
    SovrinHandle invalidPoolHandle = poolHandle + 1;
    NSString *getNymResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:invalidPoolHandle
                                                        request:getNymRequest
                                                       response:&getNymResponse];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
    NSLog(@"getNymResponse: %@", getNymResponse);
    
    [TestUtils cleanupStorage];
}

- (void) testSignAndSubmitRequestWorksForInvalidPoolHandle
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_sign_and_submit_request_works_for_invalid_pool_handle";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Obtain wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain trustee DID
    
    NSString * trusteeDidJson = [NSString stringWithFormat:@"{"\
                                 "\"seed\":\"000000000000000000000000Trustee1\"," \
                                 "\"cid\":true" \
                                 "}"];
    NSString *trusteeDid = nil;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);
    
    // 4. Obtain my DID
    
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"00000000000000000000000000000My1\"" \
                            "}"];
    NSString *myDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
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
    
    SovrinHandle invalidPoolHandle = poolHandle + 1;
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:invalidPoolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, PoolLedgerInvalidPoolHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
    NSLog(@"nymResponse: %@", nymResponse);
    
    [TestUtils cleanupStorage];
}

-(void) testSignAndSubmitRequestWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_sign_and_submit_request_works_for_invalid_wallet_handle";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Obtain wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain trustee DID
    
    NSString * trusteeDidJson = [NSString stringWithFormat:@"{"\
                                 "\"seed\":\"000000000000000000000000Trustee1\"," \
                                 "\"cid\":true" \
                                 "}"];
    NSString *trusteeDid = nil;
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);
    
    // 4. Obtain my DID
    
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"00000000000000000000000000000My1\"" \
                            "}"];
    NSString *myDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
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
    
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:invalidWalletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
    NSLog(@"nymResponse: %@", nymResponse);
    
    [TestUtils cleanupStorage];
}

- (void)testSignAndSubmitRequestWorksForNotFoundSigner
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_sign_and_submit_request_works_for_not_found_signer";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Obtain wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain my DID
    
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"00000000000000000000000000000My1\"" \
                            "}"];
    NSString *myDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"myDid: %@", myDid);
    
    
    // 4. Build NYM Request
    
    NSString *trusteeDid = @"some_trustee_did";
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
    XCTAssertEqual(ret.code, WalletNotFoundError, @"PoolUtils::signAndSubmitRequestWithPoolHandle() returned invalid error");
    NSLog(@"nymResponse: %@", nymResponse);
    
    [TestUtils cleanupStorage];
}

- (void) testSignAndSubmitRequestWorksForIncompatibleWalletAndPool
{
    [TestUtils cleanupStorage];
    NSString *poolName1 = @"pool1";
    NSString *poolName2 = @"pool2";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName1
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Obtain wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName2
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain my DID
    
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"00000000000000000000000000000My1\"" \
                            "}"];
    NSString *myDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"myDid: %@", myDid);
    
    
    // 4. Build NYM Request
    
    NSString *trusteeDid = @"some_trustee_did";
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
    
    [TestUtils cleanupStorage];
}



//// This workaround is from rust test
//- (void) getTrusteeKeys:(SovrinHandle) walletHandle
//             trusteeDid:(NSString **)trusteeDid
//          trusteeVerkey:(NSString **)trusteeVerkey
//              trusteePk:(NSString **)trusteePk
//{
//    NSError *ret = nil;
//
//    //1. Obtain trusteeVerKey
//    NSString * myDidJson = [NSString stringWithFormat:@"{"\
//                            "\"seed\":\"000000000000000000000000Trustee1\"" \
//                            "}"];
//    NSString *did = nil;
//    NSString *verKey = nil;
//    NSString *pk = nil;
//    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
//                                                          myDidJson:myDidJson
//                                                           outMyDid:&did
//                                                        outMyVerkey:&verKey
//                                                            outMyPk:&pk];
//
//    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDid() failed");
//    XCTAssertNotNil(did, @"myDid is nil!");
//    XCTAssertNotNil(verKey, @"myVerKey is nil!");
//    XCTAssertNotNil(pk, @"myPk is nil!");
//
//    *trusteeDid = did;
//    *trusteeVerkey = verKey;
//    *trusteePk = pk;
//
//    //2. Use obtained trusteeVerKey
//    myDidJson = [NSString stringWithFormat:@"{"\
//                            "\"did\":\"%@\"," \
//                            "\"seed\":\"000000000000000000000000Trustee1\"" \
//                            "}", verKey];
//
//    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
//                                                          myDidJson:myDidJson
//                                                           outMyDid:&did
//                                                        outMyVerkey:&verKey
//                                                            outMyPk:&pk];
//
//    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDid() failed");
//    XCTAssertNotNil(did, @"myDid is nil!");
//    XCTAssertNotNil(verKey, @"myVerKey is nil!");
//    XCTAssertNotNil(pk, @"myPk is nil!");
//
//    *trusteeDid = did;
//    *trusteeVerkey = verKey;
//    *trusteePk = pk;
//
//}


- (void) testSubmitRequestWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"test_submit_tx";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    
    
    NSString *request = [NSString stringWithFormat:@"{"\
                         "\"reqId\":1491566332010860," \
                         "\"identifier\":\"Th7MpTaRZVRYnPiabds81Y\"," \
                         "\"operation\":{"\
                         "\"type\":\"105\","\
                         "\"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\"},"\
                         "\"signature\":\"4o86XfkiJ4e2r3J6Ufoi17UU3W5Zi9sshV6FjBjkVw4sgEQFQov9dxqDEtLbAJAWffCWd5KfAk164QVo7mYwKkiV\"" \
                         "}"];
    
    NSString *responseJson;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:request
                                                       response:&responseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed!");
    NSLog(@"responseJson: %@", responseJson);
    
    NSDictionary *actualReply = [NSDictionary fromString:responseJson];
    
    
    NSString *dataStr = [NSString stringWithFormat:@"{"\
                         "\"dest\":\"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4\"," \
                         "\"identifier\":\"GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL\"," \
                         "\"role\":\"2\"," \
                         "\"verkey\":null" \
                         "}"];
    
    NSString *actualData = actualReply[@"result"][@"data"];
    XCTAssertTrue([actualReply[@"op"] isEqualToString:@"REPLY"], @"Wrong actualReply[op]");
    XCTAssertEqual(actualReply[@"result"][@"reqId"], @(1491566332010860), @"Wrong actualReply[reqId]");
    XCTAssertTrue([actualData isEqualToString:dataStr], "Wrong actualReply[result][data]");
    XCTAssertTrue([actualReply[@"result"][@"identifier"] isEqualToString:@"Th7MpTaRZVRYnPiabds81Y"], @"Wrong actualReply[identifier]" );
    
    [TestUtils cleanupStorage];
}

// MARK: - NYM Requests

- (void) testBuildNymRequestsWorksForOnlyRequiredFields
{
    [TestUtils cleanupStorage];
    
    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
    NSString *verkey = @"Anfh2rjAcxkE249DcdsaQl";
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

- (void) testBuildGetNymRequestWorks
{
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

- (void) testNymRequestWorksWithoutSignature
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_send_get_nym_request_works";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Obtain wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain my DID
    
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"," \
                            "\"cid\":true"\
                            "}"];
    NSString *myDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
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
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"PoolUtils::sendRequestWithPoolHandle() returned invalid error");
    NSLog(@"nymResponse: %@", nymResponse);
    
    [TestUtils cleanupStorage];
}

- (void) testSendGetNymRequestWorks
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"sovrin_send_get_nym_request_works";
    NSString *walletName = @"wallet1";
    NSString *xtype = @"default";
    NSError *ret;
    
    // 1. Obtain pool handle
    SovrinHandle poolHandle = 0;
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Obtain wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName() failed");
    
    // 3. Obtain my DID
    
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"," \
                            "\"cid\":true"\
                            "}"];
    NSString *myDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"myDid: %@", myDid);
    
    // 4. Build NYM Request
    
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
    
    XCTAssertTrue([[getNymResponse allKeys] count] > 0, @"getNymResponse is empty");
    
    [TestUtils cleanupStorage];
}


- (void) testNymRequestsWorks
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"sovrin_nym_requests_works";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. Obtain trustee did
    
    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                                "\"seed\":\"000000000000000000000000Trustee1\"," \
                                "\"cid\":true"\
                                "}"];
    NSString *trusteeDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);
    
    // 4. Obtain my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"" \
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
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
    
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");
    
    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    
    // 7. Build get nym request
    
    NSString* getNymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(getNymRequest, @"getNymRequest is nil!");
    
    // 8. Send getNymRequest
    
    NSString* getNymResponseJson = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getNymRequest
                                                       response:&getNymResponseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getNymResponseJson, @"getNymResponseJson is nil!");
    
    NSDictionary *getNymResponse = [NSDictionary fromString:getNymResponseJson];
    XCTAssertTrue([[getNymResponse allKeys] count] > 0, @"getNymResponse is empty");
    
    [TestUtils cleanupStorage];
}

// MARK: - Attribute requests

- (void) testBuildAttribRequestsWorksForRawData
{
    [TestUtils cleanupStorage];
    NSString* identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString* dest = @"Th7MpTaRZVRYnPiabds81Y";
    NSString* raw = @"{"\
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

- (void) testBuildAttribRequestsWorksForMissedAttribute
{
    [TestUtils cleanupStorage];
    NSString* identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString* dest = @"Th7MpTaRZVRYnPiabds81Y";
    
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

- (void) testBuildGetAttribRequestsWorks
{
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
                                                                                  data:raw
                                                                            resultJson:&getAttribRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequestWithSubmitterDid() returned wrong error");
    
    NSDictionary *request = [NSDictionary fromString:getAttribRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expextedresult");
    
    [TestUtils cleanupStorage];
}

- (void) testAttribRequestWorksWithoutSignature
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"sovrin_attrib_request_works_without_signature";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    
    // 3. Obtain my did
    NSString* myDid = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"" \
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
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
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not LedgerInvalidTransaction");
    XCTAssertNotNil(attribResponse, @"attribResponse is nil!");
    
    [TestUtils cleanupStorage];
}

- (void) testAttributeRequestsWorks
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"sovrin_attrib_requests_works";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWallet failed");
    
    // 3. Obtain trustee did
    NSString* trusteeDid = nil;
    NSString* trusteeDidJson = [NSString stringWithFormat:@"{"\
                                "\"seed\":\"000000000000000000000000Trustee1\"," \
                                "\"cid\":true" \
                                "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    
    // 4. Obtain my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"" \
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
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
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
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
    
    NSString* attribRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildAttribRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                      hash:nil
                                                                       raw:rawJson
                                                                       enc:nil
                                                                resultJson:&attribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildAttribRequestWithSubmitterDid() failed");
    XCTAssertNotNil(attribRequest, @"attribRequest is nil!");
    
    // 8. Sign and Submit attrib request
    NSString* attribResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:attribRequest
                                                           outResponseJson:&attribResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(attribResponse, @"attribResponse is nil!");
    
    // 9. Build getAttribRequest
    NSString* getAttribRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetAttribRequestWithSubmitterDid:myDid
                                                                    targetDid:myDid
                                                                         data:@"endpoint"
                                                                   resultJson:&getAttribRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetAttribRequest() failed");
    XCTAssertNotNil(getAttribRequest, @"getAttribRequest is nil!");
    
    // 10. Send getAttribRequest
    NSString* getAttribResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getAttribRequest
                                                       response:&getAttribResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getAttribResponse, @"getAttribResponse is nil!");
    
    [TestUtils cleanupStorage];
}

// MARK: - Schema request

- (void) testBuildSchemaRequestsWorksForCorrectDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *data = @"{"\
    "\"name\":\"name\","\
    "\"version\":\"1.0\","\
    "\"keys\":[\"name\",\"male\"]}";
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"101";
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"data"][@"name"] = @"name";
    expectedResult[@"operation"][@"data"][@"version"] = @"1.0";
    expectedResult[@"operation"][@"data"][@"keys"] = [[NSArray alloc] initWithObjects:@"name", @"male", nil];
    
    NSString *schemaRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:identifier
                                                                               data:data
                                                                         resultJson:&schemaRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");
    XCTAssertNotNil(schemaRequestJson, @"schemaRequestJson is nil!");
    NSLog(@"schemaRequestJson: %@", schemaRequestJson);
    
    NSDictionary *request = [NSDictionary fromString:schemaRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
    
    [TestUtils cleanupStorage];
}

- (void) testBuildGetSchemaRequestsWorksForCorrectDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *data = @"{"\
    "\"name\":\"name\","\
    "\"version\":\"1.0\"}";
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    
    expectedResult[@"identifier"] = @"some_identifier";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"101";
    expectedResult[@"operation"][@"dest"] = @"some_identifier";
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"data"][@"name"] = @"name";
    expectedResult[@"operation"][@"data"][@"version"] = @"1.0";
    
    NSString *getSchemaRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:identifier
                                                                                  dest:identifier
                                                                                  data:data resultJson:&getSchemaRequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequestWithSubmitterDid() failed");
    NSDictionary *request = [NSDictionary fromString:getSchemaRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
    
    [TestUtils cleanupStorage];
}

- (void) testSchemaRequestWorksWithoutSignature
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"sovrin_schema_request_works_without_signature";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    
    // 3. Obtain my did
    NSString* myDid = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000Trustee1\"," \
                           "\"cid\":true"\
                           "}"];
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build schema request
    
    NSString *schemaData = @"{"\
    "\"name\":\"gvt2\","\
    "\"version\":\"2.0\","\
    "\"keys\":[\"name\",\"male\"]}";
    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaData
                                                                resultJson:&schemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildSchemaRequestWithSubmitterDid() failed");
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");
    
    // 5. Send request
    NSString *schemaResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:schemaRequest
                                                       response:&schemaResponse];
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"LedgerUtils::sendRequestWithPoolHandle() returned not LedgerInvalidTransaction");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");
    
    [TestUtils cleanupStorage];
}

-(void)testSchemaRequestsWorks
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"pool3";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. Obtain trustee did
    
    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                                "\"seed\":\"000000000000000000000000Trustee1\"," \
                                "\"cid\":true"\
                                "}"];
    NSString *trusteeDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);
    
    // 4. Obtain my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"" \
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
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
    NSString *schemaData = [NSString stringWithFormat:@"{"\
                            "\"name\":\"gvt2\"," \
                            "\"version\":\"2.0\"," \
                            "\"keys\":[\"name\",\"male\"]" \
                            "}"];
    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaData
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
    NSString *getSchemaData = [NSString stringWithFormat:@"{"\
                               "\"name\":\"gvt2\"," \
                               "\"version\":\"2.0\"" \
                               "}"];
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                         dest:myDid
                                                                         data:getSchemaData
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
    
    [TestUtils cleanupStorage];
}

// MARK: - Node request

- (void) testBuildNodeRequestWorksForCorrectDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *dest = @"some_dest";
    NSString *data = @"{"\
    "\"node_ip\":\"ip\","\
    "\"node_port\":1,"\
    "\"client_ip\":\"ip\","\
    "\"client_port\":1,"\
    "\"alias\":\"some\","\
    "\"services\":[\"VALIDATOR\"]}";
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    
    expectedResult[@"identifier"] = @"some_identifier";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"0";
    expectedResult[@"operation"][@"dest"] = @"some_dest";
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"data"][@"node_ip"] = @"ip";
    expectedResult[@"operation"][@"data"][@"node_port"] = @(1);
    expectedResult[@"operation"][@"data"][@"client_ip"] = @"ip";
    expectedResult[@"operation"][@"data"][@"client_port"] = @(1);
    expectedResult[@"operation"][@"data"][@"alias"] = @"some";
    expectedResult[@"operation"][@"data"][@"services"] = [[NSArray alloc] initWithObjects:@"VALIDATOR", nil];
    
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

- (void) testSendNodeRequestWorksWithoutSignature
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"sovrin_send_node_request_works_without_signature";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    
    // 3. Obtain my did
    NSString* myDid = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000Steward1\"" \
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build node request
    
    NSString *nodeData = @"{"\
    "\"node_ip\":\"10.0.0.100\","\
    "\"node_port\":9710,"\
    "\"client_ip\":\"10.0.0.100\","\
    "\"client_port\":9709,"\
    "\"alias\":\"Node5\","\
    "\"services\":[\"VALIDATOR\"]}";
    
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
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() returned not LedgerInvalidTransaction");
    XCTAssertNotNil(nodeResponse, @"nodeResponse is nil!");
    
    [TestUtils cleanupStorage];
}

- (void) testSubmitNodeRequestWorksForNewSteward
{
    [TestUtils cleanupStorage];
    NSString* poolName = @"sovrin_submit_node_request_works_for_new_steward";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfigWithName() failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWallet failed");
    
    // 3. Obtain trustee did
    NSString* trusteeDid = nil;
    NSString* trusteeDidJson = [NSString stringWithFormat:@"{"\
                                "\"seed\":\"000000000000000000000000Trustee1\"," \
                                "\"cid\":true"\
                                "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    
    // 4. Obtain my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"00000000000000000000000000000My1\"," \
                           "\"cid\":true"\
                           "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
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
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
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
    
    NSString *dest = @"A5iWQVT3k8Zo9nXj4otmeqaUziPQPCiDqcydXkAJBk1Y";
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
    // TODO: 304 - LedgerInvalidTransaction
    [TestUtils cleanupStorage];
}

// MARK: - Claim def requests
- (void) testBuildClaimDefRequestWorksForCorrectDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *signatureType = @"CL";
    NSString *schemaSeqNo = @"1";
    NSString *data = @"{"\
    "\"primary\":{"\
    "\"n\":\"1\","\
    "\"s\":\"2\","\
    "\"rms\":\"3\","\
    "\"r\":{"\
    "\"name\":\"1\"},"\
    "\"rctxt\":\"1\","\
    "\"z\":\"1\"}}";
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    
    expectedResult[@"identifier"] = @"some_identifier";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"ref"] = @(1);
    expectedResult[@"operation"][@"data"] = [NSMutableDictionary new];
    
    NSMutableDictionary *primary = [NSMutableDictionary new];
    primary[@"n"] = @"1";
    primary[@"s"] = @"2";
    primary[@"rms"] = @"3";
    primary[@"r"] = [NSMutableDictionary new];
    primary[@"r"][@"name"] = @"1";
    primary[@"rctxt"] = @"1";
    primary[@"z"] = @"1";
    
    expectedResult[@"operation"][@"data"][@"primary"] = primary;
    expectedResult[@"operation"][@"data"][@"type"] = @"102";
    expectedResult[@"operation"][@"data"][@"signature_type"] = @"CL";
    
    NSString *claimDefrequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildClaimDefTxnWithSubmitterDid:identifier
                                                                             xref:schemaSeqNo
                                                                    signatureType:signatureType
                                                                             data:data
                                                                       resultJson:&claimDefrequestJson];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildClaimDefTxnWithSubmitterDid() failed");
    XCTAssertNotNil(claimDefrequestJson, @"claimDefrequestJson is nil!");
    NSLog(@"claimDefrequestJson: %@", claimDefrequestJson);
    
    NSDictionary *request = [NSDictionary fromString:claimDefrequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
    
    [TestUtils cleanupStorage];
}

- (void) testBuildGetClaimDefRequestWorks
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *xref = @"1";
    NSString *signatureType = @"signature_type";
    NSString *origin = @"some_origin";
    
    NSMutableDictionary *expectedResult = [NSMutableDictionary new];
    expectedResult[@"identifier"] = @"some_identifier";
    expectedResult[@"operation"] = [NSMutableDictionary new];
    expectedResult[@"operation"][@"type"] = @"108";
    expectedResult[@"operation"][@"ref"] = @(1);
    expectedResult[@"operation"][@"signature_type"] = @"signature_type";
    expectedResult[@"operation"][@"origin"] = @"some_origin";
    
    NSString *getClaimDefRequestJson;
    NSError *ret = [[LedgerUtils sharedInstance] buildGetClaimDefTxnWithSubmitterDid:identifier
                                                                                xref:xref
                                                                       signatureType:signatureType
                                                                              origin:origin
                                                                          resultJson:&getClaimDefRequestJson];
    
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetClaimDefTxnWithSubmitterDid() failed");
    XCTAssertNotNil(getClaimDefRequestJson, @"getClaimDefRequestJson is nil!");
    NSLog(@"getClaimDefRequestJson: %@", getClaimDefRequestJson);
    
    NSDictionary *request = [NSDictionary fromString:getClaimDefRequestJson];
    XCTAssertTrue([request contains:expectedResult], @"request doesn't contain expectedResult");
    [TestUtils cleanupStorage];
}

- (void) testClaimDefRequestsWorks
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"sovrin_claim_def_requests_works";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfigWithName:poolName
                                                                 poolHandle:&poolHandle];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:walletName
                                                                  xtype:xtype
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWalletWithPoolName failed");
    
    // 3. Obtain trustee did
    
    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                                "\"seed\":\"000000000000000000000000Trustee1\"," \
                                "\"cid\":true"\
                                "}"];
    NSString *trusteeDid = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    NSLog(@"trusteeDid: %@", trusteeDid);
    
    // 4. Obtain my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myDidJson = [NSString stringWithFormat:@"{}"];
    
    ret = [[SignusUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey
                                                            outMyPk:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDidWithWalletHandle() failed");
    
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
    NSString *schemaData = [NSString stringWithFormat:@"{"\
                            "\"name\":\"gvt2\"," \
                            "\"version\":\"2.0\"," \
                            "\"keys\":[\"name\",\"male\"]" \
                            "}"];
    NSString *schemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildSchemaRequestWithSubmitterDid:myDid
                                                                      data:schemaData
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
    NSString *getSchemaData = [NSString stringWithFormat:@"{"\
                               "\"name\":\"gvt2\"," \
                               "\"version\":\"2.0\"" \
                               "}"];
    NSString *getSchemaRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetSchemaRequestWithSubmitterDid:myDid
                                                                         dest:myDid
                                                                         data:getSchemaData
                                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");
    XCTAssertNotNil(getSchemaRequest, @"getSchemaRequest is nil!");
    
    // 10. Send getSchemaRequest
    NSString *getSchemaResponseJson = nil;
    
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getSchemaRequest
                                                       response:&getSchemaResponseJson];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getSchemaResponseJson, @"getSchemaResponseJson is nil!");
    
    NSDictionary *response = [NSDictionary fromString:getSchemaResponseJson];
    
    NSMutableDictionary *schema = [NSMutableDictionary new];
    schema[@"name"] = response[@"result"][@"data"][@"name"];
    schema[@"keys"] = response[@"result"][@"data"][@"keys"];
    schema[@"version"] = response[@"result"][@"data"][@"version"];
    schema[@"seq_no"] = response[@"result"][@"seqNo"];
    NSNumber *seqNo = response[@"result"][@"seqNo"];
    
    // 11. Create claim definition
    NSString *claimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                            schemaJson:[NSDictionary toString:schema]
                                                                        createNonRevoc:NO
                                                                          claimDefJson:&claimDefJson
                                                                          claimDefUUID:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle() failed");
    XCTAssertNotNil(claimDefJson, @"claimDefJson is nil!");
    
    NSDictionary *claimDef = [NSDictionary fromString:claimDefJson];
    
    NSMutableDictionary *claimDefData = [NSMutableDictionary new];
    claimDefData[@"primary"] = claimDef[@"public_key"];
    claimDefData[@"revocation"] = claimDef[@"public_key_revocation"];
    NSString *claimDefDataJson = [NSDictionary toString:claimDefData];
    
    // 12. Build claim def request
    NSString *claimDefRequestJson;
    ret = [[LedgerUtils sharedInstance] buildClaimDefTxnWithSubmitterDid:myDid
                                                                    xref:[seqNo stringValue]
                                                           signatureType:claimDef[@"signature_type"]
                                                                    data:claimDefDataJson
                                                              resultJson:&claimDefRequestJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::buildClaimDefTxnWithSubmitterDid() failed");
    XCTAssertNotNil(claimDefRequestJson, @"claimDefRequestJson is nil!");
    
    
    // 13. Sign and submit claim def request
    NSString *claimDefResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:claimDefRequestJson
                                                           outResponseJson:&claimDefResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(claimDefResponse, @"claimDefResponse is nil!");
    
    // 14. Build get claim def request
    // TODO: 103 Error, origin is equal to myDid
    NSString *getClaimDefRequest;
    NSString *origin = response[@"result"][@"data"][@"origin"];
    ret = [[LedgerUtils sharedInstance] buildGetClaimDefTxnWithSubmitterDid:myDid
                                                                       xref:[seqNo stringValue]
                                                              signatureType:claimDef[@"signatyre_Type"]
                                                                     origin:origin
                                                                 resultJson:&getClaimDefRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetClaimDefTxnWithSubmitterDid() failed");
    XCTAssertNotNil(getClaimDefRequest, @"getClaimDefRequest is nil!");
    
    // 15. Send getClaimDefRequest
    NSString *getClaimDefResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getClaimDefRequest
                                                       response:&getClaimDefResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getClaimDefResponse, @"getClaimDefResponse is nil!");
    
    [TestUtils cleanupStorage];
}

@end
