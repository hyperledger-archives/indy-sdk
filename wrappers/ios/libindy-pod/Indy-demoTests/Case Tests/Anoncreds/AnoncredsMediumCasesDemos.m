//
//  AnoncredsMediumCasesDemos.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 21.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"
#import "NSString+Validation.h"
#import "NSArray+JSON.h"

@interface AnoncredsMediumCasesDemos : XCTestCase

@end

@implementation AnoncredsMediumCasesDemos

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

// MARK: - Demos

- (void)testVerifierVerifyProofWorksForProofDoesNotCorrespondProofRequest {
    [TestUtils cleanupStorage];

    //1. Create wallet, get wallet handle
    NSError *ret;
    IndyHandle walletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed");

    //2. Issuer schema
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"gvt"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"age\",\"sex\",\"height\",\"name\"]"
                                                                 schemaId:&schemaId
                                                               schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([schemaId isValid], @"invalid schemaId: %@", schemaId);
    XCTAssertTrue([schemaJson isValid], @"invalid schemaJson: %@", schemaJson);

    //3. Issuer create claim definition
    NSString *claimDefId;
    NSString *claimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:[[AnoncredsUtils sharedInstance] defaultClaimDefConfig]
                                                                            claimDefId:&claimDefId
                                                                          claimDefJson:&claimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    XCTAssertTrue([claimDefId isValid], @"invalid claimDefId: %@", claimDefId);
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@", claimDefJson);

    //4. Prover create Master Secret
    NSString *masterSecretName = @"prover_master_secret";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret failed");

    // 5. Issuer create Claim Offer
    NSString *claimOfferJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:walletHandle
                                                                       claimDefId:claimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&claimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    //6. Prover create Claim Request
    NSString *claimRequest;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJSON
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:&claimRequest];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed");
    XCTAssertTrue([claimRequest isValid], @"invalid claimRequest");

    //7. Issuer create Claim
    NSString *xClaimJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getGvtClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&xClaimJson
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([xClaimJson isValid], @"invalid xClaimJson: %@", xClaimJson);

    // 8. Prover store received Claim
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                    claimId:@"ClaimId1"
                                                                 claimsJson:xClaimJson
                                                              revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimWithWalletHandle failed");

    // 9. Prover gets Claims for Proof Request
    NSString *proofReqJson = [NSString stringWithFormat:@"\
                              {"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{\
                                    \"attr1_referent\":{\
                                        \"name\":\"name\"\
                                    }\
                              },\
                              \"requested_predicates\":{}\
                            }"];
    NSString *claimsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetClaimsForProofReq() failed!");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claims_for_attr_1 = [[[claims objectForKey:@"attrs"] objectForKey:@"attr1_referent"] objectAtIndex:0];
    XCTAssertTrue(claims_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *claimReferent = [[claims_for_attr_1 objectForKey:@"cred_info"] objectForKey:@"referent"];

    // 9. Prover create Proof
    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{}\
                                     }", claimReferent];
    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, schemaJson];
    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, claimDefJson];
    NSString *revocInfosJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJson
                                                              revocInfosJSON:revocInfosJson
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProofWithWalletHandle failed");
    XCTAssertTrue([proofJson isValid], @"invalid proofJson: %@", proofJson);

    // 10. Verifier verify proof
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

    // 11. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJson];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = [[[proof objectForKey:@"requested_proof"] objectForKey:@"revealed_attrs"] objectForKey:@"attr1_referent"];
    NSString *id = [revealedAttr1 objectForKey:@"referent"];

    schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, schemaJson];

    claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, claimDefJson];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";

    BOOL isVerified = false;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJSON
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                              revocRegDefsJSON:revocRegDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isVerified];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"verifierVerifyProof() returned wrong code!");
    [TestUtils cleanupStorage];
}

- (void)testAnoncredsWorksForSingleIssuerSingleProver {
    [TestUtils cleanupStorage];

    NSString *poolName = [TestUtils pool];
    IndyHandle issuerWalletHandle = 0;
    IndyHandle proverWalletHandle = 0;
    NSError *ret = nil;

    //1. Create Issuer wallet, get wallet handle

    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&issuerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Create Prover wallet, get wallet handle
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    // 3. Issuer create Schema
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"gvt"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"age\",\"sex\",\"height\",\"name\"]"
                                                                 schemaId:&schemaId
                                                               schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([schemaId isValid], @"invalid schemaId: %@", schemaId);
    XCTAssertTrue([schemaJson isValid], @"invalid schemaJson: %@", schemaJson);

    //3. Issuer create claim definition
    NSString *claimDefId;
    NSString *claimDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:[[AnoncredsUtils sharedInstance] defaultClaimDefConfig]
                                                                            claimDefId:&claimDefId
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");

    //4. Prover create Master Secret

    NSString *masterSecretName = @"prover_master_secret";

    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");

    // 5. Issuer create Claim Offer
    NSString *claimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerWalletHandle
                                                                       claimDefId:claimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&claimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //6. Prover store Claim Offer received from Issuer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:claimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed");

    //7. Prover get Claim Offers

    NSString *filterJson = [NSString stringWithFormat:@"{ \"issuer_did\":\"%@\"}", [TestUtils issuerDid]];
    NSString *claimOffersJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:proverWalletHandle
                                                     filterJson:filterJson
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers() failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);

    NSArray *claimOffers = (NSArray *) [NSDictionary fromString:claimOffersJson];

    XCTAssertTrue(claimOffers, @"claimOffers == nil");
    XCTAssertEqual([claimOffers count], 1, @"[claimOffers count] != 1");

    NSDictionary *claimOffer1 = claimOffers[0];
    claimOfferJson = [NSDictionary toString:claimOffer1];

    //8. Prover create Claim Request
    NSString *claimReq = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJSON
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJson
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&claimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    XCTAssertTrue([claimReq isValid], @"invalid claimRequest: %@", claimReq);
    NSLog(@"claimReqJson: %@", claimReq);


    //9. Issuer create Claim
    NSString *xclaimJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:claimReq
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getGvtClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&xclaimJson
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    XCTAssertTrue([xclaimJson isValid], @"invalid xClaimJson: %@", xclaimJson);
    NSLog(@"xclaimJson: %@", xclaimJson);

    // 10. Prover store received Claim

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                    claimId:@"ClaimId1"
                                                                 claimsJson:xclaimJson
                                                              revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");


    // 11. Prover gets Claims for Proof Request
    NSString *proofReqJson = [NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attrs\":"\
                             "             {\"attr1_referent\":"\
                             "                        {"\
                             "                          \"name\":\"name\",\"restrictions\":[{\"schema_id\":\"%@\"}]"\
                             "                        },"
                                                                "              \"attr2_referent\":"
                                                                "                        {"
                                                                "                          \"name\":\"phone\""
                                                                "                        }"
                                                                "             },"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_referent\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18}"\
                             "             }"\
                             "}", [[AnoncredsUtils sharedInstance] getGvtSchemaId]];

    NSString *claimsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:proverWalletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetClaimsForProofReq() failed!");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claims_for_attr_1 = [[[claims objectForKey:@"attrs"] objectForKey:@"attr1_referent"] objectAtIndex:0];
    XCTAssertTrue(claims_for_attr_1, @"no object for key \"attr1_referent\"");
    NSString *claimReferent = [[claims_for_attr_1 objectForKey:@"cred_info"] objectForKey:@"referent"];

    // 12. Prover create Proof
    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{\"attr2_referent\":\"value\"},\
                                     \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\"}}\
                                     }", claimReferent, claimReferent];


    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, schemaJson];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimReferent, claimDefJSON];
    NSString *revocInfosJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:proverWalletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJson
                                                              revocInfosJSON:revocInfosJson
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed");
    XCTAssertTrue([proofJson isValid], @"invalid proofJson: %@", proofJson);

    NSDictionary *proof = [NSDictionary fromString:proofJson];
    NSDictionary *revealedAttr1 = [[[proof objectForKey:@"requested_proof"] objectForKey:@"revealed_attrs"] objectForKey:@"attr1_referent"];
    NSString *raw = [revealedAttr1 objectForKey:@"raw"];
    NSString *id = [revealedAttr1 objectForKey:@"referent"];

    XCTAssertTrue([raw isEqualToString:@"Alex"]);

    NSString *attestedAttrUUID = proof[@"requested_proof"][@"self_attested_attrs"][@"attr2_referent"];
    XCTAssertTrue([attestedAttrUUID isEqualToString:@"value"]);

    schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, schemaJson];

    claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", id, claimDefJSON];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";


    // 13. Verifier verify proof
    BOOL isValid = NO;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                              revocRegDefsJSON:revocRegDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isValid];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValid, @"isValid == NO");
    [TestUtils cleanupStorage];
}

- (void)testAnoncredsWorksForMultiplyIssuerSingleProver {
    [TestUtils cleanupStorage];

    NSString *poolName = [TestUtils pool];
    NSError *ret;

    //1. Issuer1 create wallet, get wallet handles

    IndyHandle issuerGvtWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&issuerGvtWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Issuer2 create wallet, get wallet handles

    IndyHandle issuerXyzWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&issuerXyzWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //3. Prover create wallet, get wallet handles

    IndyHandle proverWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    NSMutableDictionary *schemas = [NSMutableDictionary new];
    NSMutableDictionary *claimDefs = [NSMutableDictionary new];

    //4. Issuer create GVT Schema
    NSString *gvtSchemaId;
    NSString *gvtSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"gvt"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"age\",\"sex\",\"height\",\"name\"]"
                                                                 schemaId:&gvtSchemaId
                                                               schemaJson:&gvtSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([gvtSchemaId isValid], @"invalid gvtSchemaId: %@", gvtSchemaId);
    XCTAssertTrue([gvtSchemaJson isValid], @"invalid gvtSchemaJson: %@", gvtSchemaJson);

    //4. Issuer create XYZ Schema
    NSString *xyzSchemaId;
    NSString *xyzSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"xyz"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"period\",\"status\"]"
                                                                 schemaId:&xyzSchemaId
                                                               schemaJson:&xyzSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([xyzSchemaId isValid], @"invalid gvtSchemaId: %@", gvtSchemaId);
    XCTAssertTrue([xyzSchemaJson isValid], @"invalid gvtSchemaJson: %@", gvtSchemaJson);

    //4. Issuer1 create claim definition by GVT Schema
    __block NSString *issuer1GvtClaimDefId;
    __block NSString *issuer1GvtClaimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerGvtWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:gvtSchemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:[[AnoncredsUtils sharedInstance] defaultClaimDefConfig]
                                                                            claimDefId:&issuer1GvtClaimDefId
                                                                          claimDefJson:&issuer1GvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    //5. Issuer2 create claim definition by XYZ Schema

    NSString *issuer2XyzClaimDefId;
    NSString *issuer2XyzClaimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerXyzWalletHandle
                                                                             issuerDid:[TestUtils issuer2Did]
                                                                            schemaJson:xyzSchemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:@"{\"support_revocation\": false}"
                                                                            claimDefId:&issuer2XyzClaimDefId
                                                                          claimDefJson:&issuer2XyzClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    //6. Prover create Master Secret

    NSString *masterSecretName1 = @"prover_master_secret_issuer_1";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName1
                                                            walletHandle:proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed for issuer 1");

    // 7. Issuer1 create Claim Offer
    NSString *issuer1GvtClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerGvtWalletHandle
                                                                       claimDefId:issuer1GvtClaimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer1GvtClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //8. Prover store Claim Offer received from Issuer1
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer1GvtClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed for issuer 1");

    //9. Issuer2 create Claim Offer
    NSString *issuer2XyzClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerXyzWalletHandle
                                                                       claimDefId:issuer2XyzClaimDefId
                                                                        issuerDid:[TestUtils issuer2Did]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer2XyzClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //10. Prover store Claim Offer received from Issuer2
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer2XyzClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed for issuer 2");

    //12. Prover create Claim Request for Issuer1 GVT claim offer

    NSString *issuer1GvtClaimReq;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:issuer1GvtClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:issuer1GvtClaimOfferJson
                                                              masterSecretName:masterSecretName1
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&issuer1GvtClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //13. Issuer1 create GVT Claim
    NSString *issuer1GvtClaim;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerGvtWalletHandle
                                                                claimReqJson:issuer1GvtClaimReq
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getGvtClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&issuer1GvtClaim
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerGvtWalletHandle");

    //14. Prover store received GVT Claim
    NSString *gvtClaim1Id = @"ClaimId1";
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                    claimId:gvtClaim1Id
                                                                 claimsJson:issuer1GvtClaim
                                                              revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");

    //15. Prover create Claim Request for xyz claim offer
    NSString *issuer2XyzClaimReq;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:issuer2XyzClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:issuer2XyzClaimOfferJson
                                                              masterSecretName:masterSecretName1
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&issuer2XyzClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //16. Issuer create XYZ Claim
    NSString *issuer2XyzClaim;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerXyzWalletHandle
                                                                claimReqJson:issuer2XyzClaimReq
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getXyzClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&issuer2XyzClaim
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerXyzWalletHandle");

    // 17. Prover store received XYZ Claim
    NSString *xyzClaim1Id = @"ClaimId2";
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                    claimId:xyzClaim1Id
                                                                 claimsJson:issuer2XyzClaim
                                                              revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed on step 16");

    // 18. Prover gets Claims for Proof Request

    NSString *proofReqJson = [NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attrs\":"\
                             "             {\"attr1_referent\":"\
                             "                        {"\
                             "                          \"name\":\"name\",\"restrictions\":[{\"schema_id\":\"%@\"}]"\
                             "                        },"\
                             "              \"attr2_referent\":"\
                             "                        {"\
                             "                          \"name\":\"status\",\"restrictions\":[{\"schema_id\":\"%@\"}]"\
                             "                        }"\
                             "             },"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_referent\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18},"\
                             "              \"predicate2_referent\":"\
                             "                      {\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}"\
                             "             }"\
                             "}", [[AnoncredsUtils sharedInstance] getGvtSchemaId], [[AnoncredsUtils sharedInstance] getXyzSchemaId]];

    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:proverWalletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claimForAttr1 = claims[@"attrs"][@"attr1_referent"][0];
    NSDictionary *claimForAttr2 = claims[@"attrs"][@"attr2_referent"][0];

    XCTAssertTrue(claimForAttr1, @"no object for key \"attr1_referent\"");
    XCTAssertTrue(claimForAttr2, @"no object for key \"attr2_referent\"");

    NSDictionary *claimForPredicate1 = claims[@"predicates"][@"predicate1_referent"][0];
    NSDictionary *claimForPredicate2 = claims[@"predicates"][@"predicate2_referent"][0];

    XCTAssertTrue(claimForPredicate1, @"no object for key \"predicate1_referent\"");
    XCTAssertTrue(claimForPredicate2, @"no object for key \"predicate2_referent\"");

    // 19. Prover create Proof
    NSString *claim_attr_1_UUID = claimForAttr1[@"cred_info"][@"referent"];
    NSString *claim_attr_2_UUID = claimForAttr2[@"cred_info"][@"referent"];
    NSString *claim_predicate_1_UUID = claimForPredicate1[@"cred_info"][@"referent"];
    NSString *claim_predicate_2_UUID = claimForPredicate2[@"cred_info"][@"referent"];

    XCTAssertNotNil(claim_attr_1_UUID, @"claim_attr_1_UUID = nil");
    XCTAssertNotNil(claim_attr_2_UUID, @"claim_attr_2_UUID = nil");
    XCTAssertNotNil(claim_predicate_1_UUID, @"claim_predicate_1_UUID = nil");
    XCTAssertNotNil(claim_predicate_2_UUID, @"claim_predicate_2_UUID = nil");

    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true},\
                                                          \"attr2_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\"}, \
                                                               \"predicate2_referent\":{\"cred_id\":\"%@\"}}\
                                     }", claim_attr_1_UUID, claim_attr_2_UUID, claim_predicate_1_UUID, claim_predicate_2_UUID];

    NSString *schemasJson = [NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                                                       gvtClaim1Id, gvtSchemaJson,
                                                       xyzClaim1Id, xyzSchemaJson];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                                                         gvtClaim1Id, issuer1GvtClaimDefJson,
                                                         xyzClaim1Id, issuer2XyzClaimDefJson];

    NSString *revocInfosJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:proverWalletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName1
                                                               claimDefsJson:claimDefsJson
                                                              revocInfosJSON:revocInfosJson
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed on step 18");

    // 20. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJson];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];
    NSString *gvtSubProofId = revealedAttr1[@"referent"];

    NSDictionary *revealedAttr2 = proof[@"requested_proof"][@"revealed_attrs"][@"attr2_referent"];
    NSString *xyzSubProofId = revealedAttr2[@"referent"];

    schemasJson = [NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                                             gvtSubProofId, gvtSchemaJson,
                                             xyzSubProofId, xyzSchemaJson];

    claimDefsJson = [NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                                               gvtSubProofId, issuer1GvtClaimDefJson,
                                               xyzSubProofId, issuer2XyzClaimDefJson];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";

    BOOL isValidJson = NO;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegDefsJSON:revocRegDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isValidJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValidJson, @"proof is not verified!");

    [TestUtils cleanupStorage];
}


- (void)testAnoncredsWorksForSingleIssuerMultiplyClaimsSingleProver {
    [TestUtils cleanupStorage];

    NSString *poolName = [TestUtils pool];
    NSError *ret = nil;

    //1. Issuer create wallet, get wallet handles

    IndyHandle issuerWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&issuerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Prover create wallet, get wallet handles

    IndyHandle proverWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                                  xtype:nil
                                                                 handle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //4. Issuer create GVT Schema
    NSString *gvtSchemaId;
    NSString *gvtSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"gvt"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"age\",\"sex\",\"height\",\"name\"]"
                                                                 schemaId:&gvtSchemaId
                                                               schemaJson:&gvtSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([gvtSchemaId isValid], @"invalid gvtSchemaId: %@", gvtSchemaId);
    XCTAssertTrue([gvtSchemaJson isValid], @"invalid gvtSchemaJson: %@", gvtSchemaJson);

    //4. Issuer create XYZ Schema
    NSString *xyzSchemaId;
    NSString *xyzSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaForIssuerDID:[TestUtils issuerDid]
                                                                     name:@"xyz"
                                                                  version:@"1.0"
                                                                    attrs:@"[\"period\",\"status\"]"
                                                                 schemaId:&xyzSchemaId
                                                               schemaJson:&xyzSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    XCTAssertTrue([xyzSchemaId isValid], @"invalid gvtSchemaId: %@", gvtSchemaId);
    XCTAssertTrue([xyzSchemaJson isValid], @"invalid gvtSchemaJson: %@", gvtSchemaJson);

    //4. Issuer create claim definition by GVT Schema
    __block NSString *issuer1GvtClaimDefId;
    __block NSString *issuer1GvtClaimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:gvtSchemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:[[AnoncredsUtils sharedInstance] defaultClaimDefConfig]
                                                                            claimDefId:&issuer1GvtClaimDefId
                                                                          claimDefJson:&issuer1GvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    //5. Issuer create claim definition by XYZ Schema

    NSString *issuer1XyzClaimDefId;
    NSString *issuer1XyzClaimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:xyzSchemaJson
                                                                                   tag:@"TAG1"
                                                                                  type:nil
                                                                            configJson:@"{\"support_revocation\": false}"
                                                                            claimDefId:&issuer1XyzClaimDefId
                                                                          claimDefJson:&issuer1XyzClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateClaimDefinifionWithWalletHandle failed");

    //6. Prover create Master Secret

    NSString *masterSecretName1 = @"prover_master_secret_issuer_1";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName1
                                                            walletHandle:proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed for issuer 1");

    // 7. Issuer create GVT Claim Offer
    NSString *issuer1GvtClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerWalletHandle
                                                                       claimDefId:issuer1GvtClaimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer1GvtClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //8. Prover store Claim Offer received from Issuer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer1GvtClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed for issuer 1");

    //9. Issuer create Claim Offer
    NSString *issuer1XyzClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerWalletHandle
                                                                       claimDefId:issuer1XyzClaimDefId
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer1XyzClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //10. Prover store Claim Offer received from Issuer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer1XyzClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed for issuer 2");

    //12. Prover create Claim Request for Issuer GVT claim offer

    NSString *issuer1GvtClaimReq;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:issuer1GvtClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:issuer1GvtClaimOfferJson
                                                              masterSecretName:masterSecretName1
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&issuer1GvtClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //13. Issuer create GVT Claim
    NSString *issuer1GvtClaim;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:issuer1GvtClaimReq
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getGvtClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&issuer1GvtClaim
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerGvtWalletHandle");

    //14. Prover store received GVT Claim
    NSString *gvtClaim1Id = @"ClaimId1";
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                    claimId:gvtClaim1Id
                                                                 claimsJson:issuer1GvtClaim
                                                              revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");

    //15. Prover create Claim Request for xyz claim offer
    NSString *issuer1XyzClaimReq;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:issuer1XyzClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:issuer1XyzClaimOfferJson
                                                              masterSecretName:masterSecretName1
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&issuer1XyzClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //16. Issuer create XYZ Claim
    NSString *issuer1XyzClaim;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:issuer1XyzClaimReq
                                                             claimValuesJson:[[AnoncredsUtils sharedInstance] getXyzClaimValuesJson]
                                                                    revRegId:nil
                                                           tailsReaderHandle:nil
                                                              userRevocIndex:nil
                                                                outClaimJson:&issuer1XyzClaim
                                                        outRevocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerXyzWalletHandle");

    // 17. Prover store received XYZ Claim
    NSString *xyzClaim1Id = @"ClaimId2";
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                    claimId:xyzClaim1Id
                                                                 claimsJson:issuer1XyzClaim
                                                              revRegDefJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed on step 16");

    // 18. Prover gets Claims for Proof Request

    NSString *proofReqJson = [NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attrs\":"\
                             "             {\"attr1_referent\":"\
                             "                        {"\
                             "                          \"name\":\"name\",\"restrictions\":[{\"schema_id\":\"%@\"}]"\
                             "                        },"\
                             "              \"attr2_referent\":"\
                             "                        {"\
                             "                          \"name\":\"status\",\"restrictions\":[{\"schema_id\":\"%@\"}]"\
                             "                        }"\
                             "             },"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_referent\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18},"\
                             "              \"predicate2_referent\":"\
                             "                      {\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}"\
                             "             }"\
                             "}", [[AnoncredsUtils sharedInstance] getGvtSchemaId], [[AnoncredsUtils sharedInstance] getXyzSchemaId]];

    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:proverWalletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");

    NSDictionary *claimForAttr1 = claims[@"attrs"][@"attr1_referent"][0];
    NSDictionary *claimForAttr2 = claims[@"attrs"][@"attr2_referent"][0];

    XCTAssertTrue(claimForAttr1, @"no object for key \"attr1_referent\"");
    XCTAssertTrue(claimForAttr2, @"no object for key \"attr2_referent\"");

    NSDictionary *claimForPredicate1 = claims[@"predicates"][@"predicate1_referent"][0];
    NSDictionary *claimForPredicate2 = claims[@"predicates"][@"predicate2_referent"][0];

    XCTAssertTrue(claimForPredicate1, @"no object for key \"predicate1_referent\"");
    XCTAssertTrue(claimForPredicate2, @"no object for key \"predicate2_referent\"");

    // 19. Prover create Proof
    NSString *claim_attr_1_UUID = claimForAttr1[@"cred_info"][@"referent"];
    NSString *claim_attr_2_UUID = claimForAttr2[@"cred_info"][@"referent"];
    NSString *claim_predicate_1_UUID = claimForPredicate1[@"cred_info"][@"referent"];
    NSString *claim_predicate_2_UUID = claimForPredicate2[@"cred_info"][@"referent"];

    XCTAssertNotNil(claim_attr_1_UUID, @"claim_attr_1_UUID = nil");
    XCTAssertNotNil(claim_attr_2_UUID, @"claim_attr_2_UUID = nil");
    XCTAssertNotNil(claim_predicate_1_UUID, @"claim_predicate_1_UUID = nil");
    XCTAssertNotNil(claim_predicate_2_UUID, @"claim_predicate_2_UUID = nil");

    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{\
                                     \"self_attested_attributes\":{},\
                                     \"requested_attrs\":{\"attr1_referent\":{\"cred_id\":\"%@\",\"revealed\":true},\
                                                          \"attr2_referent\":{\"cred_id\":\"%@\",\"revealed\":true}},\
                                     \"requested_predicates\":{\"predicate1_referent\":{\"cred_id\":\"%@\"}, \
                                                               \"predicate2_referent\":{\"cred_id\":\"%@\"}}\
                                     }", claim_attr_1_UUID, claim_attr_2_UUID, claim_predicate_1_UUID, claim_predicate_2_UUID];

    NSString *schemasJson = [NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                                                       gvtClaim1Id, gvtSchemaJson,
                                                       xyzClaim1Id, xyzSchemaJson];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                                                         gvtClaim1Id, issuer1GvtClaimDefJson,
                                                         xyzClaim1Id, issuer1XyzClaimDefJson];

    NSString *revocInfosJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:proverWalletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName1
                                                               claimDefsJson:claimDefsJson
                                                              revocInfosJSON:revocInfosJson
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed on step 18");

    // 20. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJson];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];
    NSString *gvtSubProofId = revealedAttr1[@"referent"];

    NSDictionary *revealedAttr2 = proof[@"requested_proof"][@"revealed_attrs"][@"attr2_referent"];
    NSString *xyzSubProofId = revealedAttr2[@"referent"];

    schemasJson = [NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                                             gvtSubProofId, gvtSchemaJson,
                                             xyzSubProofId, xyzSchemaJson];

    claimDefsJson = [NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                                               gvtSubProofId, issuer1GvtClaimDefJson,
                                               xyzSubProofId, issuer1XyzClaimDefJson];

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";

    BOOL isValidJson = NO;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                              revocRegDefsJSON:revocRegDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isValidJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValidJson, @"proof is not verified!");

    [TestUtils cleanupStorage];
}

@end
