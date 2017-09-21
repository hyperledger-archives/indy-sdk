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

- (void)testAnoncredsDemo
{
    [TestUtils cleanupStorage];
    NSString *poolName = [TestUtils pool];
    NSString *walletName = @"issuer_wallet";
    NSString *xType = @"default";
    XCTestExpectation *completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    IndyHandle walletHandle = 0;
    NSError *ret;
    
    // 1. Create wallet
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xType
                                                          config:nil];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed!");
    
    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::openWalletWithName() failed!");
    
    // 3. Issuer create Claim Definition for Schema
    
    NSNumber *schemaSeqNo = @(1);
    NSString *schema = [ NSString stringWithFormat:@"{"
                        "\"seqNo\":%@,"
                        "\"data\":{"
                            "\"name\":\"gvt\","
                            "\"version\":\"1.0\","
                            "\"keys\":[\"age\",\"sex\",\"height\",\"name\"]}"
                        "}", schemaSeqNo ];
    
    __block NSString *claimDefJSON = nil;
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual( ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");
    
    NSNumber *claimDefSeqNo = @(1);
    NSMutableDictionary *claimDef = [NSMutableDictionary dictionaryWithDictionary:[NSDictionary fromString:claimDefJSON]];
    claimDef[@"seqNo"] = claimDefSeqNo;
    
    // 5. Prover create Master Secret
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *masterSecretName = @"master_secret";
    ret = [IndyAnoncreds proverCreateMasterSecretWithWalletHandle:walletHandle
                                                   masterSecretName:masterSecretName
                                                         completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "proverCreateMasterSecret got error in completion");
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 6. Prover create Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *proverDiD = @"BzfFCYk";
    NSString *claimOfferJSON =  [NSString stringWithFormat: @"{"\
                                 "\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\","\
                                 "\"schema_seq_no\": %@}", schemaSeqNo ];
    __block NSString *claimReqJSON = nil;
    
    ret = [IndyAnoncreds proverCreateAndStoreClaimReqWithWalletHandle:walletHandle
                                                              proverDid:proverDiD
                                                         claimOfferJSON:claimOfferJSON
                                                           claimDefJSON:[NSDictionary toString:claimDef]
                                                       masterSecretName:masterSecretName
                                                             completion:^(NSError *error, NSString *claimReqJSON1)
           {
               XCTAssertEqual(error.code, Success, "proverCreateAndStoreClaimReq got error in completion");
               claimReqJSON = [ NSString stringWithString: claimReqJSON1 ];
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 7. Issuer create Claim for Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString*  testClaimJson = @"{\
    \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
    \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
    \"height\":[\"175\",\"175\"],\
    \"age\":[\"28\",\"28\"]\
    }";
    __block NSString *xClaimJSON = nil;
    
    
    ret = [IndyAnoncreds issuerCreateClaimWithWalletHandle:walletHandle
                                              claimReqJSON:claimReqJSON
                                                 claimJSON:testClaimJson
                                            userRevocIndex:nil
                                                completion:^(NSError* error, NSString* revocRegUpdateJSON, NSString* claimJSON1)
           {
               XCTAssertEqual(error.code, Success, "issuerCreateClaim() got error in completion");
               xClaimJSON = [ NSString stringWithString: claimJSON1];
               [completionExpectation fulfill];
           }];
    
    XCTAssertEqual( ret.code, Success, @"issuerCreateClaim() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 8. Prover process and store Claim
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [IndyAnoncreds proverStoreClaimWithWalletHandle:walletHandle
                                                 claimsJSON:xClaimJSON
                                                 completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "proverStoreClaim() got error in completion");
               [completionExpectation fulfill];
           }];
    
    XCTAssertEqual( ret.code, Success, @"proverStoreClaim() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 9. Prover gets Claims for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString* proofReqJSON = [NSString stringWithFormat: @"\
                              {"
                              "\"nonce\":\"123432421212\","
                              "\"name\":\"proof_req_1\","
                              "\"version\":\"0.1\","
                              "\"requested_attrs\":{\
                                    \"attr1_uuid\":{\
                                        \"schema_seq_no\":%@,\
                                        \"name\":\"name\"\
                                    }\
                              },\
                              \"requested_predicates\":{\
                                    \"predicate1_uuid\":{\
                                        \"attr_name\":\"age\",\
                                        \"p_type\":\"GE\",\
                                        \"value\":18\
                                    }\
                              }\
                            }", schemaSeqNo ];
    
    __block NSString *claimsJson = nil;
    
    ret = [IndyAnoncreds proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                         proofReqJSON:proofReqJSON
                                                           completion:^(NSError* error, NSString* claimsJSON1)
           {
               claimsJson = claimsJSON1;
               XCTAssertEqual(error.code, Success, "proverGetClaimsForProofReq() got error in completion");
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual( ret.code, Success, @"proverGetClaimsForProofReq() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    NSDictionary *claims = [ NSDictionary fromString: claimsJson];
    XCTAssertTrue(claims, @"serialization failed");
    
    NSDictionary *claims_for_attr_1 = [[ [claims objectForKey: @"attrs" ] objectForKey: @"attr1_uuid"] objectAtIndex: 0 ];
    XCTAssertTrue( claims_for_attr_1, @"no object for key \"attr1_uuid\"");
    NSString *claimUUID = [claims_for_attr_1 objectForKey:@"claim_uuid"];
    
    // 10. Prover create Proof for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString* requestedClaimsJSON = [ NSString stringWithFormat: @"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_uuid\":[\"%@\",true]},\
                                     \"requested_predicates\":{\"predicate1_uuid\":\"%@\"}\
                                     }", claimUUID, claimUUID ];
    
    NSString *schemas_json = [NSString stringWithFormat: @"{\"%@\":%@}", claimUUID, schema];
    
    NSString *claimDefsJSON = [NSString stringWithFormat: @"{\"%@\":%@}", claimUUID, claimDefJSON];
    
    NSString *revocRegsJsons = @"{}";
    
    __block NSString *proofJSON = nil;
    
    ret =  [IndyAnoncreds proverCreateProofWithWalletHandle:walletHandle
                                                 proofReqJSON:proofReqJSON
                                          requestedClaimsJSON:requestedClaimsJSON
                                                  schemasJSON:schemas_json
                                             masterSecretName:masterSecretName
                                                claimDefsJSON:claimDefsJSON
                                                revocRegsJSON:revocRegsJsons
                                                   completion:^(NSError* error, NSString* proofJSON1)
            {
                XCTAssertEqual(error.code, Success, "proverCreateProof() got error in completion");
                proofJSON = [ NSString stringWithString: proofJSON1];
                [completionExpectation fulfill];
            }];
    
    XCTAssertEqual( ret.code, Success, @"proverCreateProof() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 11. Verifier verify proof
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [IndyAnoncreds verifierVerifyProofWithWalletHandle:  proofReqJSON
                                                     proofJSON:  proofJSON
                                                   schemasJSON:  schemas_json
                                                 claimDefsJSON:  claimDefsJSON
                                                 revocRegsJSON:  revocRegsJsons
                                                    completion: ^(NSError *error, BOOL valid)
           {
               XCTAssertEqual(error.code, Success, "verifierVerifyProof() got error in completion");
               XCTAssertEqual(valid, true, "verifierVerifyProof() got error in completion");
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProof() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 12. close wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[IndyWallet sharedInstance] closeWalletWithHandle: walletHandle
                                                    completion: ^ (NSError *error)
           {
               XCTAssertEqual(error.code, Success, "closeWallet got error in completion");
               [completionExpectation fulfill];
           }];
    
    XCTAssertEqual(ret.code, Success, @"closeWallet() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    [TestUtils cleanupStorage];
}

- (void)testAnoncredsDemoForKeychainWallet
{
    [TestUtils cleanupStorage];
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    NSString *poolName = [TestUtils pool];
    NSString *walletName = @"issuer_wallet";
    NSString *xType = @"keychain";
    XCTestExpectation *completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    IndyHandle walletHandle = 0;
    NSError *ret;
    
    // 0. register wallet type
    
    ret = [[WalletUtils sharedInstance] registerWalletType:xType];
    
    // 1. Create wallet
    
    ret = [[WalletUtils sharedInstance] createWalletWithPoolName:poolName
                                                      walletName:walletName
                                                           xtype:xType
                                                          config:nil];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::createWalletWithPoolName() failed!");
    
    // 2. Open wallet
    ret = [[WalletUtils sharedInstance] openWalletWithName:walletName
                                                    config:nil
                                                 outHandle:&walletHandle];
    XCTAssertEqual( ret.code, Success, @"WalletUtils::openWalletWithName() failed!");
    
    // 3. Issuer create Claim Definition for Schema
    
    NSNumber *schemaSeqNo = @(1);
    NSString *schema = [ NSString stringWithFormat:@"{"
                        "\"seqNo\":%@,"
                        "\"data\":{"
                        "\"name\":\"gvt\","
                        "\"version\":\"1.0\","
                        "\"keys\":[\"age\",\"sex\",\"height\",\"name\"]}"
                        "}", schemaSeqNo ];
    
    __block NSString *claimDefJSON = nil;
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual( ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");
    
    NSNumber *claimDefSeqNo = @(1);
    NSMutableDictionary *claimDef = [NSMutableDictionary dictionaryWithDictionary:[NSDictionary fromString:claimDefJSON]];
    claimDef[@"seqNo"] = claimDefSeqNo;
    
    // 5. Prover create Master Secret
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *masterSecretName = @"master_secret";
    ret = [IndyAnoncreds proverCreateMasterSecretWithWalletHandle:walletHandle
                                                 masterSecretName:masterSecretName
                                                       completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "proverCreateMasterSecret got error in completion");
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 6. Prover create Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString *proverDiD = @"BzfFCYk";
    NSString *claimOfferJSON =  [NSString stringWithFormat: @"{"\
                                 "\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\","\
                                 "\"schema_seq_no\": %@}", schemaSeqNo ];
    __block NSString *claimReqJSON = nil;
    
    ret = [IndyAnoncreds proverCreateAndStoreClaimReqWithWalletHandle:walletHandle
                                                            proverDid:proverDiD
                                                       claimOfferJSON:claimOfferJSON
                                                         claimDefJSON:[NSDictionary toString:claimDef]
                                                     masterSecretName:masterSecretName
                                                           completion:^(NSError *error, NSString *claimReqJSON1)
           {
               XCTAssertEqual(error.code, Success, "proverCreateAndStoreClaimReq got error in completion");
               claimReqJSON = [ NSString stringWithString: claimReqJSON1 ];
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 7. Issuer create Claim for Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString*  testClaimJson = @"{\
    \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
    \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
    \"height\":[\"175\",\"175\"],\
    \"age\":[\"28\",\"28\"]\
    }";
    __block NSString *xClaimJSON = nil;
    
    
    ret = [IndyAnoncreds issuerCreateClaimWithWalletHandle:walletHandle
                                              claimReqJSON:claimReqJSON
                                                 claimJSON:testClaimJson
                                            userRevocIndex:nil
                                                completion:^(NSError* error, NSString* revocRegUpdateJSON, NSString* claimJSON1)
           {
               XCTAssertEqual(error.code, Success, "issuerCreateClaim() got error in completion");
               xClaimJSON = [ NSString stringWithString: claimJSON1];
               [completionExpectation fulfill];
           }];
    
    XCTAssertEqual( ret.code, Success, @"issuerCreateClaim() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 8. Prover process and store Claim
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [IndyAnoncreds proverStoreClaimWithWalletHandle:walletHandle
                                               claimsJSON:xClaimJSON
                                               completion:^(NSError *error)
           {
               XCTAssertEqual(error.code, Success, "proverStoreClaim() got error in completion");
               [completionExpectation fulfill];
           }];
    
    XCTAssertEqual( ret.code, Success, @"proverStoreClaim() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 9. Prover gets Claims for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString* proofReqJSON = [NSString stringWithFormat: @"\
                              {"
                              "\"nonce\":\"123432421212\","
                              "\"name\":\"proof_req_1\","
                              "\"version\":\"0.1\","
                              "\"requested_attrs\":{\
                              \"attr1_uuid\":{\
                              \"schema_seq_no\":%@,\
                              \"name\":\"name\"\
                              }\
                              },\
                              \"requested_predicates\":{\
                              \"predicate1_uuid\":{\
                              \"attr_name\":\"age\",\
                              \"p_type\":\"GE\",\
                              \"value\":18\
                              }\
                              }\
                              }", schemaSeqNo ];
    
    __block NSString *claimsJson = nil;
    
    ret = [IndyAnoncreds proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                       proofReqJSON:proofReqJSON
                                                         completion:^(NSError* error, NSString* claimsJSON1)
           {
               claimsJson = claimsJSON1;
               XCTAssertEqual(error.code, Success, "proverGetClaimsForProofReq() got error in completion");
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual( ret.code, Success, @"proverGetClaimsForProofReq() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    NSDictionary *claims = [ NSDictionary fromString: claimsJson];
    XCTAssertTrue(claims, @"serialization failed");
    
    NSDictionary *claims_for_attr_1 = [[ [claims objectForKey: @"attrs" ] objectForKey: @"attr1_uuid"] objectAtIndex: 0 ];
    XCTAssertTrue( claims_for_attr_1, @"no object for key \"attr1_uuid\"");
    NSString *claimUUID = [claims_for_attr_1 objectForKey:@"claim_uuid"];
    
    // 10. Prover create Proof for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    NSString* requestedClaimsJSON = [ NSString stringWithFormat: @"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_uuid\":[\"%@\",true]},\
                                     \"requested_predicates\":{\"predicate1_uuid\":\"%@\"}\
                                     }", claimUUID, claimUUID ];
    
    NSString *schemas_json = [NSString stringWithFormat: @"{\"%@\":%@}", claimUUID, schema];
    
    NSString *claimDefsJSON = [NSString stringWithFormat: @"{\"%@\":%@}", claimUUID, claimDefJSON];
    
    NSString *revocRegsJsons = @"{}";
    
    __block NSString *proofJSON = nil;
    
    ret =  [IndyAnoncreds proverCreateProofWithWalletHandle:walletHandle
                                               proofReqJSON:proofReqJSON
                                        requestedClaimsJSON:requestedClaimsJSON
                                                schemasJSON:schemas_json
                                           masterSecretName:masterSecretName
                                              claimDefsJSON:claimDefsJSON
                                              revocRegsJSON:revocRegsJsons
                                                 completion:^(NSError* error, NSString* proofJSON1)
            {
                XCTAssertEqual(error.code, Success, "proverCreateProof() got error in completion");
                proofJSON = [ NSString stringWithString: proofJSON1];
                [completionExpectation fulfill];
            }];
    
    XCTAssertEqual( ret.code, Success, @"proverCreateProof() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 11. Verifier verify proof
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [IndyAnoncreds verifierVerifyProofWithWalletHandle:  proofReqJSON
                                                   proofJSON:  proofJSON
                                                 schemasJSON:  schemas_json
                                               claimDefsJSON:  claimDefsJSON
                                               revocRegsJSON:  revocRegsJsons
                                                  completion: ^(NSError *error, BOOL valid)
           {
               XCTAssertEqual(error.code, Success, "verifierVerifyProof() got error in completion");
               XCTAssertEqual(valid, true, "verifierVerifyProof() got error in completion");
               [completionExpectation fulfill];
               
           }];
    
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProof() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 12. close wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[IndyWallet sharedInstance] closeWalletWithHandle: walletHandle
                                                  completion: ^ (NSError *error)
           {
               XCTAssertEqual(error.code, Success, "closeWallet got error in completion");
               [completionExpectation fulfill];
           }];
    
    XCTAssertEqual(ret.code, Success, @"closeWallet() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}

@end
