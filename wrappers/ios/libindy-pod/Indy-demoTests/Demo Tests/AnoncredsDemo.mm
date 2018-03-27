//
//  Indy_demoTests.m
//  Indy-demoTests
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "NSDictionary+JSON.h"
#import "AnoncredsUtils.h"

@interface AnoncredsDemo : XCTestCase

@end

@implementation AnoncredsDemo

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

- (void)testAnoncredsDemo {
    [TestUtils cleanupStorage];
    NSString *poolName = [TestUtils pool];
    NSString *walletName = @"issuer_wallet";
    NSString *xType = @"default";
    NSString *proverDiD = @"BzfFCYk";

    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    IndyHandle walletHandle = 0;
    NSError *ret;

    // 1. Create wallet

    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xType
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed!");

    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed!");

    // 3. Issuer create Claim Definition for Schema

    NSNumber *schemaSeqNo = @(1);
    NSString *schema = [NSString stringWithFormat:@"{"
                                                          "\"seqNo\":%@,"
                                                          "\"dest\":\"%@\","
                                                          "\"data\":{"
                                                          "\"name\":\"gvt\","
                                                          "\"version\":\"1.0\","
                                                          "\"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]}"
                                                          "}", schemaSeqNo, [TestUtils issuerDid]];

    __block NSString *claimDefJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    NSNumber *claimDefSeqNo = @(1);
    NSMutableDictionary *claimDef = [NSMutableDictionary dictionaryWithDictionary:[NSDictionary fromString:claimDefJSON]];
    claimDef[@"seqNo"] = claimDefSeqNo;

    // 5. Prover create Master Secret
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *masterSecretName = @"master_secret";

    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");

    // 6. Issuer create Claim Offer
    __block NSString *claimOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:walletHandle
                                                                       schemaJson:schema
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:proverDiD
                                                                   claimOfferJson:&claimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    // 6. Prover create Claim Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *schemaKey = @"{\"name\":\"gvt\",\"version\":\"1.0\",\"did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}";
    __block NSString *claimReqJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:[NSDictionary toString:claimDef]
                                                                     proverDid:proverDiD
                                                                claimOfferJson:claimOfferJSON
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:&claimReqJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq() failed!");

    // 7. Issuer create Claim for Claim Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *testClaimJson = @"{\
    \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
    \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
    \"height\":[\"175\",\"175\"],\
    \"age\":[\"28\",\"28\"]\
    }";
    __block NSString *xClaimJSON = nil;


    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimReqJSON
                                                                   claimJson:testClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xClaimJSON
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaim() failed!");

    // 8. Prover process and store Claim
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                 claimsJson:xClaimJSON
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaim() failed!");

    // 9. Prover gets Claims for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *proofReqJSON = [NSString stringWithFormat:@"\
                              {"
                                                                "\"nonce\":\"123432421212\","
                                                                "\"name\":\"proof_req_1\","
                                                                "\"version\":\"0.1\","
                                                                "\"requested_attrs\":{\
                                    \"attr1_referent\":{\
                                        \"name\":\"name\",\
                                        \"restrictions\":[{\"schema_key\":%@}]\
                                    }\
                              },\
                              \"requested_predicates\":{\
                                    \"predicate1_referent\":{\
                                        \"attr_name\":\"age\",\
                                        \"p_type\":\">=\",\
                                        \"value\":18\
                                    }\
                              }\
                            }", schemaKey];

    __block NSString *claimsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofReqJSON
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetClaimsForProofReq() failed!");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claims_for_attr_1 = [[[claims objectForKey:@"attrs"] objectForKey:@"attr1_referent"] objectAtIndex:0];
    XCTAssertTrue(claims_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *claimUUID = [claims_for_attr_1 objectForKey:@"referent"];

    // 10. Prover create Proof for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *requestedClaimsJSON = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_referent\":[\"%@\",true]},\
                                     \"requested_predicates\":{\"predicate1_referent\":\"%@\"}\
                                     }", claimUUID, claimUUID];

    NSString *schemas_json = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, schema];

    NSString *claimDefsJSON = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, claimDefJSON];

    NSString *revocRegsJsons = @"{}";

    NSString *proofJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofReqJSON
                                                         requestedClaimsJson:requestedClaimsJSON
                                                                 schemasJson:schemas_json
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJSON
                                                               revocRegsJson:revocRegsJsons
                                                                outProofJson:&proofJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateProof() failed!");

    // 11. Verifier verify proof

    BOOL valid = false;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJSON
                                                     proofJson:proofJSON
                                                   schemasJson:schemas_json
                                                 claimDefsJson:claimDefsJSON
                                                 revocRegsJson:revocRegsJsons
                                                      outValid:&valid];
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProof() failed!");

    // 12. close wallet
    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"closeWallet() failed!");

    [TestUtils cleanupStorage];
}

- (void)testAnoncredsDemoForKeychainWallet {
    [TestUtils cleanupStorage];
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    NSString *poolName = [TestUtils pool];
    NSString *walletName = @"issuer_wallet";
    NSString *xType = @"keychain";
    XCTestExpectation *completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    IndyHandle walletHandle = 0;
    NSError *ret;

    // 0. register wallet type

    ret = [[WalletUtils sharedInstance] registerWalletType:xType];

    // 1. Create wallet

    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xType
                                                          config:nil];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed!");

    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::openWalletWithName() failed!");

    // 3. Issuer create Claim Definition for Schema

    NSNumber *schemaSeqNo = @(1);
    NSString *schemaKey = @"{\"name\":\"gvt\",\"version\":\"1.0\",\"did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}";
    NSString *schema = [NSString stringWithFormat:@"{"
                                                          "\"seqNo\":%@,"
                                                          "\"dest\":\"%@\","
                                                          "\"data\":{"
                                                          "\"name\":\"gvt\","
                                                          "\"version\":\"1.0\","
                                                          "\"attr_names\":[\"age\",\"sex\",\"height\",\"name\"]}"
                                                          "}", schemaSeqNo, [TestUtils issuerDid]];

    __block NSString *claimDefJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    NSNumber *claimDefSeqNo = @(1);
    NSMutableDictionary *claimDef = [NSMutableDictionary dictionaryWithDictionary:[NSDictionary fromString:claimDefJSON]];
    claimDef[@"seqNo"] = claimDefSeqNo;

    // 4. Prover create Master Secret

    NSString *masterSecretName = @"master_secret";

    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:walletHandle];

    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");

    // 5. Issuer create Claim Offer
    __block NSString *claimOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:walletHandle
                                                                       schemaJson:schema
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&claimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    // 6. Prover create Claim Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *claimReqJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:[NSDictionary toString:claimDef]
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJSON
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:&claimReqJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq() failed!");

    // 7. Issuer create Claim for Claim Request

    NSString *testClaimJson = @"{\
    \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
    \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
    \"height\":[\"175\",\"175\"],\
    \"age\":[\"28\",\"28\"]\
    }";
    __block NSString *xClaimJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimReqJSON
                                                                   claimJson:testClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xClaimJSON
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaim() failed!");

    // 8. Prover process and store Claim
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                 claimsJson:xClaimJSON
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaim() failed!");

    // 9. Prover gets Claims for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *proofReqJSON = [NSString stringWithFormat:@"\
                              {"
                                                                "\"nonce\":\"123432421212\","
                                                                "\"name\":\"proof_req_1\","
                                                                "\"version\":\"0.1\","
                                                                "\"requested_attrs\":{\
                              \"attr1_referent\":{\
                              \"name\":\"name\",\
                              \"restrictions\":[{\"schema_key\":%@}]\
                              }\
                              },\
                              \"requested_predicates\":{\
                              \"predicate1_referent\":{\
                              \"attr_name\":\"age\",\
                              \"p_type\":\">=\",\
                              \"value\":18\
                              }\
                              }\
                              }", schemaKey];

    __block NSString *claimsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofReqJSON
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetClaimsForProofReq() failed!");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claims_for_attr_1 = [[[claims objectForKey:@"attrs"] objectForKey:@"attr1_referent"] objectAtIndex:0];
    XCTAssertTrue(claims_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *claimUUID = [claims_for_attr_1 objectForKey:@"referent"];

    // 10. Prover create Proof for Proof Request

    NSString *requestedClaimsJSON = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_referent\":[\"%@\",true]},\
                                     \"requested_predicates\":{\"predicate1_referent\":\"%@\"}\
                                     }", claimUUID, claimUUID];

    NSString *schemas_json = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, schema];

    NSString *claimDefsJSON = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, claimDefJSON];

    NSString *revocRegsJsons = @"{}";

    __block NSString *proofJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofReqJSON
                                                         requestedClaimsJson:requestedClaimsJSON
                                                                 schemasJson:schemas_json
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJSON
                                                               revocRegsJson:revocRegsJsons
                                                                outProofJson:&proofJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateProof() failed!");

    // 11. Verifier verify proof

    BOOL valid = false;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJSON
                                                     proofJson:proofJSON
                                                   schemasJson:schemas_json
                                                 claimDefsJson:claimDefsJSON
                                                 revocRegsJson:revocRegsJsons
                                                      outValid:&valid];

    XCTAssertEqual(valid, true, "verifierVerifyProof() got error in completion");

    // 12. close wallet

    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"closeWallet() failed!");

    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}

@end
