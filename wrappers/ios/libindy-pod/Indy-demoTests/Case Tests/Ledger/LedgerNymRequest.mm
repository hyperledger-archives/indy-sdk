//
//  Ledger-MediumCases.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 13.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"

@interface LedgerNymRequest : XCTestCase

@end

@implementation LedgerNymRequest

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void) testSendNymRequestWorksForOnlyRequiredFields
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_send_nym_request_works_for_only_required_fields";
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
    NSString* trusteeDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:@"000000000000000000000000Trustee1"
                                                                   outMyDid:&trusteeDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for trustee");
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    
    // 4. Obtain my did
    NSString* myDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for myDid");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 5. Build nym request
    
    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:nil
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

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void) testSendNymRequestWorksWithOptionFields
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_send_nym_request_works_with_option_fields";
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
    NSString *role = @"STEWARD";
    NSString *alias = @"some_alias";
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:alias
                                                                   role:role
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");
    
    // 6. Send and submit nym request

    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void) testNymRequestWorksForWrongRole
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *dest = @"Th7MpTaRZVRYnPiabds81Y";
    NSString *role = @"WRONG_ROLE";
    
    NSString *nymRequest;
    NSError *ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:identifier
                                                                       targetDid:dest
                                                                          verkey:nil
                                                                           alias:nil
                                                                            role:role
                                                                      outRequest:&nymRequest];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");
    
    [TestUtils cleanupStorage];
}

- (void) testNymRequestWorksForWrongSignerRole
{
    [TestUtils cleanupStorage];
    
    NSString *poolName = @"indy_nym_request_works_for_wrong_signer_role";
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
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    
    // 4. Obtain my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{\"cid\":true}"
                                                           outMyDid:&myDid
                                                        outMyVerkey:&myVerKey];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");
    
    // 5. Build nym request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:myVerKey
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");
    
    // 6. Send and submit nym request
    
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequestWithPoolHandle() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    
    // 7. Obtain my did 2
    NSString *myDid2;
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:@"{}"
                                                           outMyDid:&myDid2
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid2, @"myDid is nil!");

    // 8. Build bym request
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:myDid
                                                              targetDid:myDid2
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");
    //TODO: code 0, not 304
    
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() returned not Success");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:nymResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REJECT"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void) testNymRequestWorksForUnknownSignerDid
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_nym_request_works_for_unknown_signer_did";
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
    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                                "\"seed\":\"000000000000000000000000Trustee9\"," \
                                "\"cid\":true" \
                                "}"];
    
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:trusteeDidJson
                                                           outMyDid:&trusteeDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    
    // 4. Obtain my did
    NSString *myDid = nil;
    ret = [[DidUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:nil
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createAndStoreMyDid() failed for myDid");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 5. Build nym request
    NSString *nymRequest;
    ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:trusteeDid
                                                              targetDid:myDid
                                                                 verkey:nil
                                                                  alias:nil
                                                                   role:nil
                                                             outRequest:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");
    
    // 6. Send and submit nym request
    
    NSString *nymResponse;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:trusteeDid
                                                               requestJson:nymRequest
                                                           outResponseJson:&nymResponse];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() returned not Success");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    NSDictionary *response = [NSDictionary fromString:nymResponse];
    XCTAssertTrue([response[@"op"] isEqualToString:@"REQNACK"], @"wrong response type");

    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void) testGetNymRequestWorksForUnknownDid
{
    
    [TestUtils cleanupStorage];
    NSString *poolName = @"indy_get_nym_request_works_for_unknown_did";
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
                           "\"seed\":\"00000000000000000000000000000My3\"" \
                           "}"];
    
    ret = [[DidUtils sharedInstance] createMyDidWithWalletHandle:walletHandle
                                                          myDidJson:myDidJson
                                                           outMyDid:&myDid
                                                        outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"DidUtils::createMyDidWithWalletHandle() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 5. Build getNym request
    NSString *getNymRequest;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:myDid
                                                                 targetDid:myDid
                                                                outRequest:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() failed");
    XCTAssertNotNil(getNymRequest, @"getNymRequest is nil!");
    
    // 6. Send get nym request
    
    NSString *getNymResponse;
    ret = [[PoolUtils sharedInstance] sendRequestWithPoolHandle:poolHandle
                                                        request:getNymRequest
                                                       response:&getNymResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequestWithPoolHandle() failed");
    XCTAssertNotNil(getNymResponse, @"getNymResponse is nil!");
    XCTAssertFalse([getNymResponse isEqualToString:@""], @"getNymResponse is empty!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testBuildNymRequestWorksForInvalidIdentifier
{
    NSString *identifier = @"invalid_base58_identifier";
    NSString *dest = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
    
    NSError *ret = [[LedgerUtils sharedInstance] buildNymRequestWithSubmitterDid:identifier
                                                                       targetDid:dest
                                                                          verkey:nil
                                                                           alias:nil
                                                                            role:nil
                                                                      outRequest:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils::buildNymRequestWithSubmitterDid() returned wrong code");
}

- (void)testBuildGetNymRequestWorksForInvalidIdentifier
{
    NSString *identifier = @"invalid_base58_identifier";
    NSString *dest = @"FYmoFw55GeQH7SRFa37dkx1d2dZ3zUF8ckg7wmL7ofN4";
    
    NSError *ret = [[LedgerUtils sharedInstance] buildGetNymRequestWithSubmitterDid:identifier
                                                                          targetDid:dest
                                                                         outRequest:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils::buildGetNymRequestWithSubmitterDid() returned wrong code");
}

@end
