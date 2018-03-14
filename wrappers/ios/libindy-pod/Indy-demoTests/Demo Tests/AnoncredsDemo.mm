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

    //3. Issuer create GVT Schema
    __block NSString *schemaId;
    __block NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"gvt"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"age\",\"sex\",\"height\",\"name\"]"
                                                                 schemaId:&schemaId
                                                               schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([schemaId isValid], @"invalid schemaId: %@", schemaId);
    XCTAssertTrue([schemaJson isValid], @"invalid schemaJson: %@", schemaJson);

    // 4. Issuer create Claim Definition for Schema
    __block NSString *claimDefId;
    __block NSString *claimDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:[[AnoncredsUtils sharedInstance] defaultClaimDefConfig]
                                                                            claimDefId:&claimDefId
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    XCTAssertTrue([claimDefId isValid], @"invalid claimDefId: %@", claimDefId);
    XCTAssertTrue([claimDefJSON isValid], @"invalid claimDefJSON: %@", claimDefJSON);

    // 5. Prover create Master Secret
    NSString *masterSecretName = @"master_secret";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");

    // 6. Issuer create Claim Offer
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *claimOfferJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:walletHandle
                                                                       claimDefId:claimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&claimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    // 7. Prover create Claim Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *claimReqJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJSON
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJSON
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:&claimReqJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq() failed!");

    // 8. Issuer create Claim for Claim Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *xClaimJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimReqJSON
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getGvtClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&xClaimJSON
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaim() failed!");

    // 9. Prover process and store Claim
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                    claimId:@"ClaimId1"
                                                                 claimsJson:xClaimJSON
                                                              revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"proverStoreClaim() failed!");

    // 10. Prover gets Claims for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *proofReqJSON = [NSString stringWithFormat:@"\
                              {"
                            "\"nonce\":\"123432421212\","
                            "\"name\":\"proof_req_1\","
                            "\"version\":\"0.1\","
                            "\"requested_attrs\":{\
                                    \"attr1_referent\":{\
                                        \"name\":\"name\"\
                                    }\
                              },\
                              \"requested_predicates\":{\
                                    \"predicate1_referent\":{\
                                        \"attr_name\":\"age\",\
                                        \"p_type\":\">=\",\
                                        \"value\":18\
                                    }\
                              }\
                            }"];

    __block NSString *claimsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofReqJSON
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetClaimsForProofReq() failed!");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claims_for_attr_1 = [[[claims objectForKey:@"attrs"] objectForKey:@"attr1_referent"] objectAtIndex:0];
    XCTAssertTrue(claims_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *claimReferent = [[claims_for_attr_1 objectForKey:@"cred_info"] objectForKey:@"referent"];

    // 11. Prover create Proof for Proof Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    NSString *requestedClaimsJSON = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\"}}\
                                     }", claimReferent, claimReferent];

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, schemaJson];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, claimDefJSON];

    NSString *revocInfosJson = @"{}";

    NSString *proofJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofReqJSON
                                                         requestedClaimsJson:requestedClaimsJSON
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJson
                                                              revocInfosJSON:revocInfosJson
                                                                outProofJson:&proofJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateProof() failed!");

    // 12. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJSON];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = [[[proof objectForKey:@"requested_proof"] objectForKey:@"revealed_attrs"] objectForKey:@"attr1_referent"];
    NSString *id = [revealedAttr1 objectForKey:@"referent"];

    schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, schemaJson];

    claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, claimDefJSON];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";


    BOOL valid = false;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJSON
                                                     proofJson:proofJSON
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                              revocRegDefsJSON:revocRegDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&valid];
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProof() failed!");
    XCTAssertTrue(valid);

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

    //3. Issuer create GVT Schema
    __block NSString *schemaId;
    __block NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"gvt"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"age\",\"sex\",\"height\",\"name\"]"
                                                                 schemaId:&schemaId
                                                               schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    // 3. Issuer create Claim Definition for Schema
    __block NSString *claimDefId;
    __block NSString *claimDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:[[AnoncredsUtils sharedInstance] defaultClaimDefConfig]
                                                                            claimDefId:&claimDefId
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    // 4. Prover create Master Secret

    NSString *masterSecretName = @"master_secret";

    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:walletHandle];

    XCTAssertEqual(ret.code, Success, @"proverCreateMasterSecret() failed!");

    // 5. Issuer create Claim Offer
    __block NSString *claimOfferJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:walletHandle
                                                                       claimDefId:claimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&claimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    // 6. Prover create Claim Request
    completionExpectation = [[XCTestExpectation alloc] initWithDescription:@"completion finished"];

    __block NSString *claimReqJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJSON
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJSON
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:&claimReqJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateAndStoreClaimReq() failed!");

    // 7. Issuer create Claim for Claim Request

    __block NSString *xClaimJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimReqJSON
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getGvtClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&xClaimJSON
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaim() failed!");

    // 8. Prover process and store Claim
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                    claimId:@"ClaimId2"
                                                                 claimsJson:xClaimJSON
                                                              revRegDefJSON:nil];
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
                                        \"name\":\"name\"\
                                    }\
                              },\
                              \"requested_predicates\":{\
                                    \"predicate1_referent\":{\
                                        \"attr_name\":\"age\",\
                                        \"p_type\":\">=\",\
                                        \"value\":18\
                                    }\
                              }\
                            }"];

    __block NSString *claimsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofReqJSON
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetClaimsForProofReq() failed!");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claims_for_attr_1 = [[[claims objectForKey:@"attrs"] objectForKey:@"attr1_referent"] objectAtIndex:0];
    XCTAssertTrue(claims_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *claimReferent = [[claims_for_attr_1 objectForKey:@"cred_info"] objectForKey:@"referent"];

    // 10. Prover create Proof for Proof Request

    NSString *requestedClaimsJSON = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\"}}\
                                     }", claimReferent, claimReferent];

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, schemaJson];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, claimDefJSON];

    NSString *revocInfosJson = @"{}";

    NSString *proofJSON = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofReqJSON
                                                         requestedClaimsJson:requestedClaimsJSON
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJson
                                                              revocInfosJSON:revocInfosJson
                                                                outProofJson:&proofJSON];
    XCTAssertEqual(ret.code, Success, @"proverCreateProof() failed!");

    // 11. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJSON];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = [[[proof objectForKey:@"requested_proof"] objectForKey:@"revealed_attrs"] objectForKey:@"attr1_referent"];
    NSString *id = [revealedAttr1 objectForKey:@"referent"];

    schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, schemaJson];

    claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, claimDefJSON];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";


    BOOL valid = false;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJSON
                                                     proofJson:proofJSON
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                              revocRegDefsJSON:revocRegDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&valid];
    XCTAssertEqual(ret.code, Success, @"verifierVerifyProof() failed!");
    XCTAssertTrue(valid);

    // 12. close wallet

    ret = [[WalletUtils sharedInstance] closeWalletWithHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"closeWallet() failed!");

    [[IndyWallet sharedInstance] cleanupIndyKeychainWallet];
    [TestUtils cleanupStorage];
}

@end
