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

// This workaround is from rust test
- (void) getTrusteeKeys:(SovrinHandle) walletHandle
             trusteeDid:(NSString **)trusteeDid
          trusteeVerkey:(NSString **)trusteeVerkey
              trusteePk:(NSString **)trusteePk
{
    NSError *ret = nil;
    
    //1. Obtain trusteeVerKey
    NSString * myDidJson = [NSString stringWithFormat:@"{"\
                            "\"seed\":\"000000000000000000000000Trustee1\"" \
                            "}"];
    NSString *did = nil;
    NSString *verKey = nil;
    NSString *pk = nil;
    
    ret = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:myDidJson
                                              myDid:&did
                                           myVerkey:&verKey
                                               myPk:&pk];
    
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDid() failed");
    XCTAssertNotNil(did, @"myDid is nil!");
    XCTAssertNotNil(verKey, @"myVerKey is nil!");
    XCTAssertNotNil(pk, @"myPk is nil!");
    
    *trusteeDid = did;
    *trusteeVerkey = verKey;
    *trusteePk = pk;
    
    //2. Use obtained trusteeVerKey
    myDidJson = [NSString stringWithFormat:@"{"\
                            "\"did\":\"%@\"," \
                            "\"seed\":\"000000000000000000000000Trustee1\"" \
                            "}", verKey];
   
    ret = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:myDidJson
                                              myDid:&did
                                           myVerkey:&verKey
                                               myPk:&pk];
    
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDid() failed");
    XCTAssertNotNil(did, @"myDid is nil!");
    XCTAssertNotNil(verKey, @"myVerKey is nil!");
    XCTAssertNotNil(pk, @"myPk is nil!");
    
    *trusteeDid = did;
    *trusteeVerkey = verKey;
    *trusteePk = pk;

}

- (void) testNymRequestsWorks
{
    NSLog(@"Ledger: testNymRequestsWorks() started...");
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"pool1";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *ret = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    ret = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfig:&poolHandle
                                                           poolName:poolName];
    XCTAssertEqual(ret.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWallet:poolName
                                                 walletName:walletName
                                                      xtype:xtype
                                                     handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils:createAndOpenWallet failed");

    // 3. Obtain trustee did
    NSString* trusteeDid = nil;
    NSString* trusteeVerKey = nil;
    NSString* trusteePk = nil;
    
    [self getTrusteeKeys:walletHandle
              trusteeDid:&trusteeDid
           trusteeVerkey:&trusteeVerKey
               trusteePk:&trusteePk];
    
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    XCTAssertNotNil(trusteeVerKey, @"trusteeVerKey is nil!");
    XCTAssertNotNil(trusteePk, @"trusteePk is nil!");
    NSLog(@"trusteeDid = %@", trusteeDid);
    NSLog(@"trusteeVerKey = %@", trusteeVerKey);
    NSLog(@"trusteePk = %@", trusteePk);
    
    // 4. Create my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myPk = nil;
    
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                 "\"seed\":\"000000000000000000000000My1\"" \
                 "}"];
    
    ret = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:myDidJson
                                              myDid:&myDid
                                           myVerkey:&myVerKey
                                               myPk:&myPk];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createMyDid() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");
    XCTAssertNotNil(myPk, @"myPk is nil!");
    
    // 5. Build nym request
    
    NSString *nymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNymRequest:trusteeDid
                                              targetDid:myDid
                                                 verkey:myVerKey
                                                   xref:@""
                                                   data:@""
                                                   role:@""
                                             resultJson:&nymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");
    
    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:trusteeDid
                                                 requestJson:nymRequest
                                           responseJson:&nymResponse];
    
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    
    // 7. Build get nym request
    
    NSString* getNymRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildGetNymRequest:myDid
                                                 targetDid:myDid
                                                resultJson:&getNymRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildGetNymRequest() failed");
    XCTAssertNotNil(getNymRequest, @"nymResponse is nil!");
    
    // 8. Send getNymRequest
    
    NSString* getNymResponse = nil;
    ret = [[PoolUtils sharedInstance] sendRequest:poolHandle
                                          request:getNymRequest
                                         response:&getNymResponse];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getNymRequest, @"getNymResponse is nil!");
    
    [TestUtils cleanupStorage];
     NSLog(@"Ledger: testNymRequestsWorks() finished...");
}

- (void) testAttributeRequestsWorks
{
    NSLog(@"Ledger: testAttributeRequestsWorks() started...");
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"pool1";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *res = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    res = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfig:&poolHandle
                                                           poolName:poolName];
     XCTAssertEqual(res.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWallet:poolName
                                                 walletName:walletName
                                                      xtype:xtype
                                                     handle:&walletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils:createAndOpenWallet failed");
    
    // 3. Obtain trustee did
    NSString* trusteeDid = nil;
    NSString* trusteeVerKey = nil;
    NSString* trusteePk = nil;
    
    [self getTrusteeKeys:walletHandle
              trusteeDid:&trusteeDid
           trusteeVerkey:&trusteeVerKey
               trusteePk:&trusteePk];
    
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    XCTAssertNotNil(trusteeVerKey, @"trusteeVerKey is nil!");
    XCTAssertNotNil(trusteePk, @"trusteePk is nil!");
    
    // 4. Create my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myPk = nil;
    
    NSString* myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000My1\"," \
                           "}"];
    res = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:myDidJson
                                              myDid:&myDid
                                           myVerkey:&myVerKey
                                               myPk:&myPk];
    XCTAssertEqual(res.code, Success, @"SignusUtils::createMyDid() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");
    XCTAssertNotNil(myPk, @"myPk is nil!");
    
    // 5. Build nym request
    
    NSString *nymRequest = nil;
    res = [[LedgerUtils sharedInstance] buildNymRequest:trusteeDid
                                              targetDid:myDid
                                                 verkey:myVerKey
                                                   xref:nil
                                                   data:nil
                                                   role:nil
                                             resultJson:&nymRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");

    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:trusteeDid
                                                 requestJson:nymRequest
                                                responseJson:&nymResponse];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");

    // 7. Build attrib request
    NSString *rawJson = [NSString stringWithFormat:@"{"\
                         "\"endpoint\":{\"ha\":\"127.0.0.1:5555\"}" \
                         "}"];
    
    
    NSString* attribRequest = nil;
    res = [[LedgerUtils sharedInstance] buildAttribRequest:myDid
                                                 targetDid:myDid
                                                      hash:nil
                                                       raw:rawJson
                                                       enc:nil
                                               resultJson:&attribRequest];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildAttribRequest() failed");
    XCTAssertNotNil(nymResponse, @"attribRequest is nil!");
    
    // 8. Sign and Submit attrib request
    NSString* attribResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:myDid
                                                 requestJson:attribRequest
                                                responseJson:&attribResponse];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"attribResponse is nil!");
    
    // 9. Build getAttribRequest
    NSString* getAttribRequest = nil;
    res = [[LedgerUtils sharedInstance] buildGetAttribRequest:myDid
                                                    targetDid:myDid
                                                         data:@"endpoint"
                                                  resultJson:&getAttribRequest];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildGetAttribRequest() failed");
    XCTAssertNotNil(nymResponse, @"getAttribRequest is nil!");
    
    // 10. Send getAttribRequest
    NSString* getAttribResponse = nil;
    res = [[PoolUtils sharedInstance] sendRequest:poolHandle
                                          request:getAttribRequest
                                         response:&getAttribResponse];
    
    XCTAssertEqual(res.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getAttribResponse, @"getAttribRequest is nil!");
    
    [TestUtils cleanupStorage];
    
    NSLog(@"Ledger: testAttributeRequestsWorks() started...");
}

-(void)testSchemaRequestsWorks
{
    NSLog(@"Ledger: testSchemaRequestsWorks() started...");
    [TestUtils cleanupStorage];

    NSString* poolName = @"pool1";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *res = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    res = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfig:&poolHandle
                                                           poolName:poolName];
    XCTAssertEqual(res.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWallet:poolName
                                                 walletName:walletName
                                                      xtype:xtype
                                                     handle:&walletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils:createAndOpenWallet failed");
    
    // 3. Obtain trustee did
    NSString *trusteeDid = nil;
    NSString *trusteeVerKey = nil;
    NSString *trusteePk = nil;
    
    [self getTrusteeKeys:walletHandle
              trusteeDid:&trusteeDid
           trusteeVerkey:&trusteeVerKey
               trusteePk:&trusteePk];
    
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    XCTAssertNotNil(trusteeVerKey, @"trusteeVerKey is nil!");
    XCTAssertNotNil(trusteePk, @"trusteePk is nil!");
    
    // 4. Create my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    NSString *myPk = nil;
    
    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000My1\"" \
                           "}"];
    res = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:myDidJson
                                              myDid:&myDid
                                           myVerkey:&myVerKey
                                               myPk:&myPk];
    XCTAssertEqual(res.code, Success, @"SignusUtils::createMyDid() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    // XCTAssertNotNil(myVerKey, @"myVerKey is nil!"); // can be nil?
    XCTAssertNotNil(myPk, @"myPk is nil!");
    
    // 5. Build nym request
    
    NSString *nymRequest = nil;
    res = [[LedgerUtils sharedInstance] buildNymRequest:trusteeDid
                                              targetDid:myDid
                                                 verkey:myVerKey
                                                   xref:@""
                                                   data:@""
                                                   role:@""
                                             resultJson:&nymRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");
    
    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:trusteeDid
                                                 requestJson:nymRequest
                                                responseJson:&nymResponse];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    
    // 7. Build schema request
    NSString *schemaData = [NSString stringWithFormat:@"{"\
                            "\"name\":\"gvt2\"," \
                            "\"version\":\"2.0\"," \
                            "\"keys\":[\"name\",\"male\"]" \
                            "}"];
    NSString *schemaRequest = nil;
    res = [[LedgerUtils sharedInstance] buildSchemaRequest:myDid
                                                      data:schemaData
                                                resultJson:&schemaRequest];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildSchemaRequest() failed");
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");
    
    // 8. Sign and submit schema request
    NSString *schemaResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:myDid
                                                 requestJson:schemaRequest
                                                responseJson:&schemaResponse];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");
    
    // 9. Build getSchemaRequest
    NSString *getSchemaData = [NSString stringWithFormat:@"{"\
                               "\"name\":\"gvt2\"," \
                               "\"version\":\"2.0\"" \
                               "}"];
    NSString *getSchemaRequest = nil;
    res = [[LedgerUtils sharedInstance] buildGetSchemaRequest:myDid
                                                         dest:myDid
                                                         data:getSchemaData
                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");
    XCTAssertNotNil(getSchemaRequest, @"getSchemaRequest is nil!");
    
    // 10. Sign and seng getSchemaRequest
    NSString *getSchemaResponse = nil;
    res = [[PoolUtils sharedInstance] sendRequest:poolHandle
                                          request:getSchemaRequest
                                         response:&getSchemaResponse];
    XCTAssertEqual(res.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getSchemaResponse, @"getSchemaResponse is nil!");
    
    [TestUtils cleanupStorage];
    NSLog(@"Ledger: testSchemaRequestsWorks() finished...");
}

- (void)testNodeRequestWorks
{
    NSLog(@"Ledger: testNodeRequestWorks() started...");
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"pool1";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *res = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    res = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfig:&poolHandle
                                                           poolName:poolName];
    XCTAssertEqual(res.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWallet:poolName
                                                 walletName:walletName
                                                      xtype:xtype
                                                     handle:&walletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils:createAndOpenWallet failed");
    
    // 3. Obtain trustee did
    NSString *trusteeDid = nil;
    NSString *trusteeVerKey = nil;
    NSString *trusteePk = nil;
    
    [self getTrusteeKeys:walletHandle
              trusteeDid:&trusteeDid
           trusteeVerkey:&trusteeVerKey
               trusteePk:&trusteePk];
    
    NSString *trusteeDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000Trustee1\"" \
                           "}"];

    res = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:trusteeDidJson
                                              myDid:&trusteeDid
                                           myVerkey:&trusteeVerKey
                                               myPk:&trusteePk];
    
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    XCTAssertNotNil(trusteeVerKey, @"trusteeVerKey is nil!");
    XCTAssertNotNil(trusteePk, @"trusteePk is nil!");
    
    // 4. Create my did
    NSString *myDid = nil;
    NSString *myVerKey = nil;
    NSString *myPk = nil;
    
    NSString *myDidJson = [NSString stringWithFormat:@"{"\
                           "\"seed\":\"000000000000000000000000Steward1\"" \
                           "}"];
    res = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:myDidJson
                                              myDid:&myDid
                                           myVerkey:&myVerKey
                                               myPk:&myPk];
    XCTAssertEqual(res.code, Success, @"SignusUtils::createMyDid() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");
    XCTAssertNotNil(myPk, @"myPk is nil!");
    
    // 5. Build nym request
    
    NSString *nymRequest = nil;
    res = [[LedgerUtils sharedInstance] buildNymRequest:trusteeVerKey
                                              targetDid:myDid
                                                 verkey:myVerKey
                                                   xref:nil
                                                   data:nil
                                                   role:nil
                                             resultJson:&nymRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequestResult is nil!");
    
    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:trusteeDid
                                                 requestJson:nymRequest
                                                responseJson:&nymResponse];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    
    // 7. Build Node request
    NSString *nodeIp = [PoolUtils nodeIp];
    NSString *nodeData = [NSString stringWithFormat:@"{"\
                            "\"node_ip\":\"%@\"," \
                            "\"node_port\":\"9710\"," \
                            "\"client_ip\":\"%@\"," \
                            "\"client_port\":\"9709\"," \
                           "\"alias\":\"Node5\"," \
                           "\"services\":[\"VALIDATOR\"]" \
                            "}", nodeIp, nodeIp];
 

    NSString *nodeRequest = nil;
    res = [[LedgerUtils sharedInstance] buildNodeRequest:myVerKey
                                               targetDid:myDid
                                                    data:nodeData
                                              resultJson:&nodeRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildNodeRequest() failed");
    XCTAssertNotNil(nodeRequest, @"nodeRequest is nil!");
    
    // 8. Sign and submit node request
    NSString *nodeResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:myDid
                                                 requestJson:nodeRequest
                                                responseJson:&nodeResponse];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nodeResponse, @"nodeResponse is nil!");
    
    // TODO: Correct handling of reject

    [TestUtils cleanupStorage];
    NSLog(@"Ledger: testNodeRequestsWorks() finished...");
}

- (void) testClaimDefRequests
{
    NSLog(@"Ledger: testClaimDefRequests() started...");
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"pool1";
    NSString* walletName = @"wallet1";
    NSString* xtype = @"default";
    NSError *res = nil;
    
    // 1. Create and open pool ledger config, get pool handle
    SovrinHandle poolHandle = 0;
    
    res = [[PoolUtils sharedInstance] createAndOpenPoolLedgerConfig:&poolHandle
                                                           poolName:poolName];
    XCTAssertEqual(res.code, Success, @"PoolUtils:createAndOpenPoolLedgerConfig:poolName failed");
    
    // 2. Create and open wallet, get wallet handle
    SovrinHandle walletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWallet:poolName
                                                 walletName:walletName
                                                      xtype:xtype
                                                     handle:&walletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils:createAndOpenWallet failed");
    
    // 3. Obtain trustee did
    NSString* trusteeDid = nil;
    NSString* trusteeVerKey = nil;
    NSString* trusteePk = nil;
    
    [self getTrusteeKeys:walletHandle
              trusteeDid:&trusteeDid
           trusteeVerkey:&trusteeVerKey
               trusteePk:&trusteePk];
    
    XCTAssertNotNil(trusteeDid, @"trusteeDid is nil!");
    XCTAssertNotNil(trusteeVerKey, @"trusteeVerKey is nil!");
    XCTAssertNotNil(trusteePk, @"trusteePk is nil!");
    
    [TestUtils cleanupStorage];
    NSLog(@"Ledger: testClaimDefRequests() finished...");
    
    // 4. Create my did
    NSString* myDid = nil;
    NSString* myVerKey = nil;
    NSString* myPk = nil;
    
    NSString* myDidJson = [NSString stringWithFormat:@"{}"];
    res = [[SignusUtils sharedInstance] createMyDid:walletHandle
                                          myDidJson:myDidJson
                                              myDid:&myDid
                                           myVerkey:&myVerKey
                                               myPk:&myPk];
    XCTAssertEqual(res.code, Success, @"SignusUtils::createMyDid() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    XCTAssertNotNil(myVerKey, @"myVerKey is nil!");
    XCTAssertNotNil(myPk, @"myPk is nil!");
    
    
    // 5. Build nym request
    
    NSString *nymRequest = nil;
    res = [[LedgerUtils sharedInstance] buildNymRequest:trusteeDid
                                              targetDid:myDid
                                                 verkey:myVerKey
                                                   xref:nil
                                                   data:nil
                                                   role:nil
                                             resultJson:&nymRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildNymRequest() failed");
    XCTAssertNotNil(nymRequest, @"nymRequest is nil!");
    
    // 6. Sign and Submit nym request
    NSString *nymResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:trusteeDid
                                                 requestJson:nymRequest
                                                responseJson:&nymResponse];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nymResponse, @"nymResponse is nil!");
    
    // 7. Build schema request
    NSString *schemaData = [NSString stringWithFormat:@"{"\
                            "\"name\":\"gvt2\"," \
                            "\"version\":\"2.0\"," \
                            "\"keys\":[\"name\",\"male\"]" \
                            "}"];
    NSString *schemaRequest = nil;
    res = [[LedgerUtils sharedInstance] buildSchemaRequest:myDid
                                                      data:schemaData
                                                resultJson:&schemaRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildSchemaRequest() failed");
    XCTAssertNotNil(schemaRequest, @"schemaRequest is nil!");

    // 8. Sign and submit schema request
    NSString *schemaResponse = nil;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:myDid
                                                 requestJson:schemaRequest
                                                responseJson:&schemaResponse];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(schemaResponse, @"schemaResponse is nil!");

    // 9. Build getSchemaRequest
    NSString *getSchemaData = [NSString stringWithFormat:@"{"\
                               "\"name\":\"gvt2\"," \
                               "\"version\":\"2.0\"" \
                               "}"];
    NSString *getSchemaRequest = nil;
    res = [[LedgerUtils sharedInstance] buildGetSchemaRequest:myDid
                                                         dest:myDid
                                                         data:getSchemaData
                                                   resultJson:&getSchemaRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildGetSchemaRequest() failed");
    XCTAssertNotNil(getSchemaRequest, @"getSchemaRequest is nil!");

    // 10. Sign and seng getSchemaRequest
    NSString *getSchemaResponseJson = nil;
    res = [[PoolUtils sharedInstance] sendRequest:poolHandle
                                          request:getSchemaRequest
                                         response:&getSchemaResponseJson];
    XCTAssertEqual(res.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getSchemaResponseJson, @"getSchemaResponseJson is nil!");

    // 11. Handle getSchemaResponse Json
    NSDictionary *getSchemaResponse = [NSDictionary fromString: getSchemaResponseJson];
    
    NSMutableDictionary *schema = [NSMutableDictionary new];
    schema[@"name"] = getSchemaResponse[@"result"][@"data"][@"name"];
    schema[@"keys"] = getSchemaResponse[@"result"][@"data"][@"keys"];
    schema[@"version"] = getSchemaResponse[@"result"][@"data"][@"version"];
    schema[@"seq_no"] = getSchemaResponse[@"result"][@"seq_no"];
    
    // 12. Create claim definition
    
    NSString *claimDefJson;
    NSString *claimDefUUID;
    res = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifion:walletHandle
                                                            schemaJson:[NSDictionary toString:schema]
                                                          claimDefJson:&claimDefJson claimDefUUID:&claimDefUUID];
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifion() failed");
    XCTAssertNotNil(claimDefJson, @"claimDefJson is nil!");
    XCTAssertNotNil(claimDefUUID, @"claimDefUUID is nil!");
    
    // 13. Handle claim definition
    
    NSDictionary *claimDefinition = [NSDictionary fromString:claimDefJson];
    
    NSMutableDictionary *claimDefData = [NSMutableDictionary new];
    claimDefData[@"primary"] = claimDefinition[@"public_key"];
    claimDefData[@"revocation"] = claimDefinition[@"public_key_revocation"];
    
    // 14. Build claim definition txn request
    
    NSString *claimDefRequest;
    res = [[LedgerUtils sharedInstance] buildClaimDefTxn:myDid
                                                    xref:schema[@"seq_no"]
                                           signatureType:claimDefinition[@"signature_type"]
                                                    data:[NSDictionary toString:claimDefData]
                                              resultJson:&claimDefRequest];
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildClaimDefTxn() failed");
    XCTAssertNotNil(claimDefRequest, @"claimDefRequest is nil!");
    
    // 15. Sign and submit claimDefRequest
    NSString *claimDefResponse;
    res = [[LedgerUtils sharedInstance] signAndSubmitRequest:poolHandle
                                                walletHandle:walletHandle
                                                submitterDid:myDid
                                                 requestJson:claimDefRequest
                                                responseJson:&claimDefResponse];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(claimDefResponse, @"claimDefResponse is nil!");
    
    // 16. Build get claim definition request
    NSString *getClaimDefRequest;
    res = [[LedgerUtils sharedInstance] buildGetClaimDefTxn:myDid
                                                       xref:schema[@"seq_no"]
                                              signatureType:claimDefinition[@"signature_type"]
                                                     origin:getSchemaResponse[@"result"][@"data"][@"origin"]
                                                 resultJson:&getClaimDefRequest];
    
    XCTAssertEqual(res.code, Success, @"LedgerUtils::buildGetClaimDefTxn() failed");
    XCTAssertNotNil(getClaimDefRequest, @"getClaimDefRequest is nil!");
    
    // 17. Send get claim def request
    NSString *getClaimDefResponse;
    res = [[PoolUtils sharedInstance] sendRequest:poolHandle
                                          request:getClaimDefRequest
                                         response:&getClaimDefResponse];
    XCTAssertEqual(res.code, Success, @"PoolUtils::sendRequest() failed");
    XCTAssertNotNil(getClaimDefResponse, @"getClaimDefResponse is nil!");
    
    [TestUtils cleanupStorage];
    NSLog(@"Ledger: testClaimDefRequests() finished...");
}

@end
