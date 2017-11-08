//
//  LedgerNodeRequest.m
//  Indy-demo
//

#import <XCTest/XCTest.h>
#import "PoolUtils.h"
#import "TestUtils.h"
#import "WalletUtils.h"
#import "SignusUtils.h"
#import "LedgerUtils.h"
#import "AnoncredsUtils.h"
#import <Indy/Indy.h>
#import "NSDictionary+JSON.h"

@interface LedgerNodeRequest : XCTestCase

@end

@implementation LedgerNodeRequest

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testBuildNodeRequestWorksForMissedFieldInDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *dest = @"some_dest";
    NSString *data = @"{\"node_ip\":\"ip\","\
                       "\"node_port\":1,"\
                       "\"client_ip\":\"ip\","\
                       "\"client_port\":1}";
    
    NSString *nodeRequest;
    NSError *ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:identifier
                                                                        targetDid:dest
                                                                             data:data
                                                                       resultJson:&nodeRequest];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils:buildNodeRequestWithSubmitterDid returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testBuildNodeRequestWorksForWrongService
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *dest = @"some_dest";
    NSString *data = @"{\"node_ip\":\"ip\","\
                        "\"node_port\":1,"\
                        "\"client_ip\":\"ip\","\
                        "\"client_port\":1,"\
                        "\"alias\":\"some\","\
                        "\"services\":[\"SERVICE\"]}";

    NSString *nodeRequest;
    NSError *ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:identifier
                                                                        targetDid:dest
                                                                             data:data
                                                                       resultJson:&nodeRequest];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils:buildNodeRequestWithSubmitterDid returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testSendNodeRequestWorksForWrongRole
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"indy_send_node_request_works_for_wrong_role";
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
    NSString* myDid = nil;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:@"000000000000000000000000Trustee1"
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDid() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build schema request
    NSString *nodeData = [NSString stringWithFormat:@"{"\
                            "\"node_ip\":\"10.0.0.100\"," \
                            "\"node_port\":9710," \
                            "\"client_ip\":\"10.0.0.100\"," \
                            "\"client_port\":9709," \
                            "\"alias\":\"Node5\"," \
                            "\"services\":[\"VALIDATOR\"]," \
                            "\"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\""
                            "}"];
    NSString *nodeRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:myDid
                                                               targetDid:myDid
                                                                    data:nodeData
                                                              resultJson:&nodeRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNodeRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nodeRequest, @"nodeRequest is nil!");
 
    // 5. Sign and submit request
    NSString *nodeResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:nodeRequest
                                                           outResponseJson:&nodeResponse];
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nodeResponse, @"nodeResponse is nil!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testSubmitNodeRequestWorksForAlreadyHasNode
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"indy_submit_node_request_works_for_already_has_node";
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
    NSString* myDid = nil;
    ret = [[SignusUtils sharedInstance] createAndStoreMyDidWithWalletHandle:walletHandle
                                                                       seed:@"000000000000000000000000Steward1"
                                                                   outMyDid:&myDid
                                                                outMyVerkey:nil];
    XCTAssertEqual(ret.code, Success, @"SignusUtils::createAndStoreMyDid() failed");
    XCTAssertNotNil(myDid, @"myDid is nil!");
    
    // 4. Build schema request
    NSString *nodeData = [NSString stringWithFormat:@"{"\
                          "\"node_ip\":\"10.0.0.100\"," \
                          "\"node_port\":9710," \
                          "\"client_ip\":\"10.0.0.100\"," \
                          "\"client_port\":9709," \
                          "\"alias\":\"Node5\"," \
                          "\"services\":[\"VALIDATOR\"],"
                          "\"blskey\": \"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\""
                          "}"];
    NSString *nodeRequest = nil;
    ret = [[LedgerUtils sharedInstance] buildNodeRequestWithSubmitterDid:myDid
                                                               targetDid:myDid
                                                                    data:nodeData
                                                              resultJson:&nodeRequest];
    XCTAssertEqual(ret.code, Success, @"LedgerUtils::buildNodeRequestWithSubmitterDid() failed");
    XCTAssertNotNil(nodeRequest, @"nodeRequest is nil!");
    
    // 5. Sign and submit request
    NSString *nodeResponse = nil;
    ret = [[LedgerUtils sharedInstance] signAndSubmitRequestWithPoolHandle:poolHandle
                                                              walletHandle:walletHandle
                                                              submitterDid:myDid
                                                               requestJson:nodeRequest
                                                           outResponseJson:&nodeResponse];
    XCTAssertEqual(ret.code, LedgerInvalidTransaction, @"LedgerUtils::signAndSubmitRequest() failed");
    XCTAssertNotNil(nodeResponse, @"nodeResponse is nil!");
    
    [[PoolUtils sharedInstance] closeHandle:poolHandle];
    [TestUtils cleanupStorage];
}

- (void)testBuildClaimDefRequestWorksForInvalidDataJson
{
    [TestUtils cleanupStorage];
    NSString *identifier = @"some_identifier";
    NSString *signature_type = @"CL";
    NSNumber *schemaSeqNo = @(1);
    NSString *data = @"{"\
                        "\"primary\":{"\
                            "\"n\":\"1\","\
                            "\"s\":\"2\","\
                            "\"rsm\":\"3\","\
                            "\"r\":{\"name\":\"1\"}"\
                        "}}";
    
    NSString *claimDefTxn;
    NSError *ret = [[LedgerUtils sharedInstance] buildClaimDefTxnWithSubmitterDid:identifier
                                                                             xref:schemaSeqNo
                                                                    signatureType:signature_type
                                                                             data:data
                                                                       resultJson:&claimDefTxn];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"LedgerUtils::buildClaimDefTxnWithSubmitterDid() failed");
    
    [TestUtils cleanupStorage];
}

@end
