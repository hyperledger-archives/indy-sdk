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

    //2. Issuer create claim definition
    NSNumber *schemaSeqNo = @(1);
    NSString *schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:schemaSeqNo];
    NSString *claimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&claimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@", claimDefJson);

    //3. Prover create Master Secret
    NSString *masterSecretName = @"prover_master_secret";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret failed");

    // 4. Issuer create Claim Offer
    NSString *claimOfferJSON = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:walletHandle
                                                                       schemaJson:schema
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&claimOfferJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");

    //5. Prover create Claim Request
    NSString *claimRequest;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJSON
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:&claimRequest];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed");
    XCTAssertTrue([claimRequest isValid], @"invalid claimRequest");

    //6. Issuer create Claim
    NSString *claimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    NSString *xClaimJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xClaimJson
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([xClaimJson isValid], @"invalid xClaimJson: %@", xClaimJson);

    // 7. Prover store received Claim
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                 claimsJson:xClaimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimWithWalletHandle failed");

    // 8. Prover gets Claims for Proof Request
    NSString *proofReqJson = [NSString stringWithFormat:@"{"
                                                                "\"nonce\":\"123432421212\","
                                                                "\"name\":\"proof_req_1\","
                                                                "\"version\":\"0.1\","
                                                                "\"requested_attrs\":{"
                                                                "\"attr1_referent\":{"
                                                                "\"name\":\"name\",\"restrictions\":[{\"schema_key\":%@}]}},"
                                                                "\"requested_predicates\":{}"
                                                                "}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson: %@", claimsJson);

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSString *attrUUID = claims[@"attrs"][@"attr1_referent"][0][@"referent"];
    XCTAssertTrue([attrUUID isValid], @"invalid attrUUID: %@", attrUUID);

    // 9. Prover create Proof
    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"
                                                                       "\"self_attested_attributes\":{},"

                                                                       "\"requested_attrs\":{"
                                                                       "\"attr1_referent\":[\"%@\",true]},"
                                                                       "\"requested_predicates\":{}"
                                                                       "}", attrUUID];
    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", attrUUID, schema];
    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", attrUUID, claimDefJson];
    NSString *revocRegsJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJson
                                                               revocRegsJson:revocRegsJson
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProofWithWalletHandle failed");
    XCTAssertTrue([proofJson isValid], @"invalid proofJson: %@", proofJson);

    // 10. Verifier verify proof
    proofReqJson = [NSString stringWithFormat:@"{"
                                                      "\"nonce\":\"123432421212\","
                                                      "\"name\":\"proof_req_1\","
                                                      "\"version\":\"0.1\","
                                                      "\"requested_attrs\":{"
                                                      "\"attr1_referent\":{"
                                                      "\"name\":\"name\",\"restrictions\":[{\"schema_key\":%@}]}},"
                                                      "\"requested_predicates\":{"
                                                      "\"predicate1_referent\":{"
                                                      "\"attr_name\":\"age\","
                                                      "\"p_type\":\">=\","
                                                      "\"value\":18}}"
                                                      "}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    BOOL isVerified = false;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isVerified];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::verifierVerifyProof returned wrong code");
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

    //3. Issuer create claim definition
    NSNumber *schemaSeqNo = @1;
    NSString *claimDefJSON = nil;
    NSString *schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:schemaSeqNo];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:nil
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");

    //4. Prover create Master Secret

    NSString *masterSecretName = @"prover_master_secret";

    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");

    // 5. Issuer create Claim Offer
    NSString *claimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerWalletHandle
                                                                       schemaJson:schema
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

    NSString *claimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:claimReq
                                                                   claimJson:claimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xclaimJson
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    XCTAssertTrue([xclaimJson isValid], @"invalid xClaimJson: %@", xclaimJson);
    NSLog(@"xclaimJson: %@", xclaimJson);

    // 10. Prover store received Claim

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:xclaimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");


    // 11. Prover gets Claims for Proof Request
    NSString *proofReqJson = [NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attrs\":"\
                             "             {\"attr1_referent\":"\
                             "                        {"\
                             "                          \"name\":\"name\",\"restrictions\":[{\"schema_key\":%@}]"\
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
                             "}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    NSString *claimsJson = nil;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:proverWalletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson: %@", claimsJson);

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSString *claimUUID = claims[@"attrs"][@"attr1_referent"][0][@"referent"];

    // 12. Prover create Proof
    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"\
                                     "  \"self_attested_attributes\":{\"attr2_referent\":\"value\"},"\
                                     "  \"requested_attrs\":{\"attr1_referent\":[\"%@\",true]},"\
                                     "  \"requested_predicates\":{\"predicate1_referent\":\"%@\"}"\
                                     "}", claimUUID, claimUUID];

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, schema];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, claimDefJSON];
    NSString *revocRegsJsons = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:proverWalletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJson
                                                               revocRegsJson:revocRegsJsons
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed");
    XCTAssertTrue([proofJson isValid], @"invalid proofJson: %@", proofJson);

    NSDictionary *proof = [NSDictionary fromString:proofJson];
    NSString *revealedAttrUUID = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"][1];
    XCTAssertTrue([revealedAttrUUID isEqualToString:@"Alex"]);

    NSString *attestedAttrUUID = proof[@"requested_proof"][@"self_attested_attrs"][@"attr2_referent"];
    XCTAssertTrue([attestedAttrUUID isEqualToString:@"value"]);

    // 13. Verifier verify proof
    BOOL isValid = NO;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJsons
                                                      outValid:&isValid];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValid, @"isValid == NO");
    [TestUtils cleanupStorage];
}

- (void)testAnoncredsWorksForMultiplyIssuerSingleProver {
    [TestUtils cleanupStorage];

    NSString *issuer2Did = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
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

    //4. Issuer1 create claim definition by gvt schema

    NSNumber *gvtSchemaSeqNo = @1;
    NSString *gvtSchema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:gvtSchemaSeqNo];
    NSString *gvtClaimDefJson = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerGvtWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:gvtSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&gvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle() failed");
    NSLog(@"gvtSchena: %@", gvtSchema);

    [schemas setValue:gvtSchema forKey:@"gvt"];
    [claimDefs setValue:gvtClaimDefJson forKey:@"gvt"];

    //5. Issuer1 create claim definition by xyz schema

    NSNumber *xyzSchemaSeqNo = @2;
    NSString *xyzClaimDefJson = nil;
    NSString *xyzSchema = [[AnoncredsUtils sharedInstance] getXyzSchemaJson:xyzSchemaSeqNo];
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerXyzWalletHandle
                                                                             issuerDid:issuer2Did
                                                                            schemaJson:xyzSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&xyzClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");
    NSLog(@"xyzClaimDefJson: %@", xyzClaimDefJson);

    schemas[@"xyz"] = xyzSchema;
    claimDefs[@"xyz"] = xyzClaimDefJson;

    //6. Prover create Master Secret for Issuer1

    NSString *masterSecretName1 = @"prover_master_secret_issuer_1";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName1
                                                            walletHandle:proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed for issuer 1");

    // 7. Issuer1 create Claim Offer
    NSString *issuer1ClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerGvtWalletHandle
                                                                       schemaJson:gvtSchema
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer1ClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //8. Prover store Claim Offer received from Issuer1
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer1ClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed for issuer 1");

    //9. Issuer2 create Claim Offer
    NSString *issuer2ClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerXyzWalletHandle
                                                                       schemaJson:xyzSchema
                                                                        issuerDid:issuer2Did
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuer2ClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //10. Prover store Claim Offer received from Issuer2
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer2ClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed for issuer 2");

    //11. Prover get Claim Offers

    NSString *filterJson = @"{}";
    NSString *claimOffsersJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:proverWalletHandle
                                                     filterJson:filterJson
                                             outClaimOffersJSON:&claimOffsersJson];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverGetClaimOffers() failed");
    NSLog(@"claimOffsersJson: %@", claimOffsersJson);

    NSArray *claimOffers = (NSArray *) [NSDictionary fromString:claimOffsersJson];

    XCTAssertTrue(claimOffers, @"claimOffers == nil");
    XCTAssertEqual([claimOffers count], 2, @"[claimOffers count] != 2");

    NSDictionary *claimOffer1 = claimOffers[0];
    NSDictionary *claimOffer2 = claimOffers[1];

    XCTAssertTrue(claimOffer1, @"claimOffer1 == nil");
    XCTAssertTrue(claimOffer2, @"claimOffer2 == nil");

    NSString *claimOffer1Json = [NSDictionary toString:claimOffer1];
    NSString *claimOffer2Json = [NSDictionary toString:claimOffer2];

    XCTAssertTrue(claimOffer1Json, @"claimOffer1Json == nil");
    XCTAssertTrue(claimOffer2Json, @"claimOffer2Json == nil");

    NSString *claimOffer1_issuerDid = claimOffer1[@"issuer_did"];
    NSString *claimOffer2_issuerDid = claimOffer2[@"issuer_did"];

    NSString *claimOffer = [claimOffer1_issuerDid isEqual:[TestUtils issuerDid]] ? claimOffer1Json : claimOffer2Json;

    //12. Prover create Claim Request for gvt claim offer

    NSString *gvtClaimReq;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:gvtClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOffer
                                                              masterSecretName:masterSecretName1
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&gvtClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //13. Issuer create GVT Claim
    NSString *revocRegUpdateJson;
    NSString *gvtClaimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerGvtWalletHandle
                                                                claimReqJson:gvtClaimReq
                                                                   claimJson:gvtClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&gvtClaimJson
                                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerGvtWalletHandle");

    //14. Prover store received GVT Claim

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:gvtClaimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");

    //15. Prover create Claim Request for xyz claim offer

    claimOffer = [claimOffer2_issuerDid isEqual:issuer2Did] ? claimOffer2Json : claimOffer1Json;
    NSString *xyzClaimReq;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:xyzClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOffer
                                                              masterSecretName:masterSecretName1
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&xyzClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //16. Issuer create XYZ Claim

    NSString *xyzClaimJson = [[AnoncredsUtils sharedInstance] getXyzClaimJson];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerXyzWalletHandle
                                                                claimReqJson:xyzClaimReq
                                                                   claimJson:xyzClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xyzClaimJson
                                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerXyzWalletHandle");

    // 17. Prover store received XYZ Claim

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:xyzClaimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed on step 16");

    // 18. Prover gets Claims for Proof Request

    NSString *proofReqJson = [NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attrs\":"\
                             "             {\"attr1_referent\":"\
                             "                        {"\
                             "                          \"name\":\"name\",\"restrictions\":[{\"schema_key\":%@}]"\
                             "                        },"\
                             "              \"attr2_referent\":"\
                             "                        {"\
                             "                          \"name\":\"status\",\"restrictions\":[{\"schema_key\":%@}]"\
                             "                        }"\
                             "             },"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_referent\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18},"\
                             "              \"predicate2_referent\":"\
                             "                      {\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}"\
                             "             }"\
                             "}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey], [[AnoncredsUtils sharedInstance] getXyzSchemaKey]];

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

    NSString *claim_attr_1_UUID = claimForAttr1[@"referent"];
    NSString *claim_attr_2_UUID = claimForAttr2[@"referent"];
    NSString *claim_predicate_1_UUID = claimForPredicate1[@"referent"];
    NSString *claim_predicate_2_UUID = claimForPredicate2[@"referent"];

    XCTAssertNotNil(claim_attr_1_UUID, @"claim_attr_1_UUID = nil");
    XCTAssertNotNil(claim_attr_2_UUID, @"claim_attr_2_UUID = nil");
    XCTAssertNotNil(claim_predicate_1_UUID, @"claim_predicate_1_UUID = nil");
    XCTAssertNotNil(claim_predicate_2_UUID, @"claim_predicate_2_UUID = nil");

    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"\
                                     "  \"self_attested_attributes\":{},"\
                                     "  \"requested_attrs\":{\"attr1_referent\":[\"%@\",true], "\
                                     "                       \"attr2_referent\":[\"%@\",true]},"\
                                     "  \"requested_predicates\":{\"predicate1_referent\":\"%@\","\
                                     "                            \"predicate2_referent\":\"%@\"}"\
                                     "}", claim_attr_1_UUID, claim_attr_2_UUID,
                                                               claim_predicate_1_UUID, claim_predicate_2_UUID];


    NSArray *uniqueClaims = [[AnoncredsUtils sharedInstance] getUniqueClaimsFrom:claims];
    XCTAssertNotNil(uniqueClaims, @"AnoncredsUtils::getUniqueClaimsFrom: failed");

    // obtain unique claims
    NSDictionary *uniqueClaim1 = uniqueClaims[0];
    NSDictionary *uniqueClaim2 = uniqueClaims[1];
    XCTAssertNotNil(uniqueClaim1, @"uniqueClaim1 = nil");
    XCTAssertNotNil(uniqueClaim2, @"uniqueClaim1 = nil");

    NSString *schemasJson = [NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                                                       uniqueClaim1[@"referent"], schemas[uniqueClaim1[@"schema_key"][@"name"]],
                                                       uniqueClaim2[@"referent"], schemas[uniqueClaim2[@"schema_key"][@"name"]]];

    // Configure claimDefsJson

    NSString *claimDefsJson = [NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                                                         uniqueClaim1[@"referent"], claimDefs[uniqueClaim1[@"schema_key"][@"name"]],
                                                         uniqueClaim2[@"referent"], claimDefs[uniqueClaim2[@"schema_key"][@"name"]]];

    NSString *revocRegsJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:proverWalletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName1
                                                               claimDefsJson:claimDefsJson
                                                               revocRegsJson:revocRegsJson
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed on step 18");

    // 20. Verifier verify proof

    BOOL isValidJson = NO;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
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

    NSMutableDictionary *schemas = [NSMutableDictionary new]; //[String: String]
    NSMutableDictionary *claimDefs = [NSMutableDictionary new];

    //3. Issuer create claim definition by gvt schema

    NSNumber *gvtSchemaSeqNo = @1;
    NSString *gvtSchema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:gvtSchemaSeqNo];
    NSString *gvtClaimDefJson = nil;

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:gvtSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&gvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed on step 3");

    [schemas setValue:gvtSchema forKey:@"gvt"];
    [claimDefs setValue:gvtClaimDefJson forKey:@"gvt"];

    //4. Issuer create claim definition by xyz schema

    NSNumber *xyzSchemaSeqNo = @2;
    NSString *xyzClaimDefJson;
    NSString *xyzSchema = [[AnoncredsUtils sharedInstance] getXyzSchemaJson:xyzSchemaSeqNo];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:xyzSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&xyzClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle() failed on step 4 for xyzSchema.");

    [schemas setValue:xyzSchema forKey:@"xyz"];
    [claimDefs setValue:xyzClaimDefJson forKey:@"xyz"];

    //5. Prover create Master Secret for Issuer

    NSString *masterSecretName = @"prover_master_secret_issuer";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:masterSecretName
                                                            walletHandle:proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");

    //6. Issuer create GVT Claim Offer
    NSString *issuerGVTClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerWalletHandle
                                                                       schemaJson:gvtSchema
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuerGVTClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //7. Prover store GVT Claim Offer received from Issuer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuerGVTClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed on step 6");

    //8. Issuer create XYZ Claim Offer
    NSString *issuerXYZClaimOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimOfferWithWalletHandle:issuerWalletHandle
                                                                       schemaJson:xyzSchema
                                                                        issuerDid:[TestUtils issuerDid]
                                                                        proverDid:[TestUtils proverDid]
                                                                   claimOfferJson:&issuerXYZClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreClaimDef() failed!");


    //9. Prover store XYZ Claim Offer received from Issuer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuerXYZClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed on step 7");

    //10. Prover create Claim Request for gvt claim offer

    NSString *gvtClaimReq = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:gvtClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:issuerGVTClaimOfferJson
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&gvtClaimReq];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //11. Issuer create GVT Claim

    NSString *revocRegUpdateJson = nil;
    NSString *gvtClaimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:gvtClaimReq
                                                                   claimJson:gvtClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&gvtClaimJson
                                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for gvtClaimReq ");

    //12. Prover store received GVT Claim

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:gvtClaimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");

    //13. Prover create Claim Request for xyz claim offer

    NSString *xyzClaimReq = nil;

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:xyzClaimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:issuerXYZClaimOfferJson
                                                              masterSecretName:masterSecretName
                                                                  walletHandle:proverWalletHandle
                                                               outClaimReqJson:&xyzClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //14. Issuer create XYZ Claim

    NSString *xyzClaimJson = [[AnoncredsUtils sharedInstance] getXyzClaimJson];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:xyzClaimReq
                                                                   claimJson:xyzClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xyzClaimJson
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for xyzClaimReq");

    //15. Prover store received XYZ Claim

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:xyzClaimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");

    //16. Prover gets Claims for Proof Request

    NSString *proofReqJson = [NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","
                                                                " \"version\":\"0.1\","
                                                                " \"requested_attrs\":"\
                             "             {\"attr1_referent\":"\
                             "                        {"\
                             "                          \"name\":\"name\",\"restrictions\":[{\"schema_key\":%@}]"\
                             "                        }},"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_referent\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\">=\",\"value\":18},"\
                             "              \"predicate2_referent\":"\
                             "                      {\"attr_name\":\"period\",\"p_type\":\">=\",\"value\":5}"\
                             "             }"\
                             "}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    NSString *claimsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:proverWalletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertTrue(claims, @"serialization failed");
    XCTAssertEqual([claims[@"attrs"] count], 1, @"claims.attrs.count != 1");
    XCTAssertEqual([claims[@"predicates"] count], 2, @"claims.predicates.count != 1");

    NSDictionary *claimForAttr1 = claims[@"attrs"][@"attr1_referent"][0];
    XCTAssertTrue(claimForAttr1, @"no object for key \"attr1_referent\"");

    NSDictionary *claimForPredicate1 = claims[@"predicates"][@"predicate1_referent"][0];
    NSDictionary *claimForPredicate2 = claims[@"predicates"][@"predicate2_referent"][0];

    XCTAssertTrue(claimForPredicate1, @"no object for key \"predicate1_referent\"");
    XCTAssertTrue(claimForPredicate2, @"no object for key \"predicate2_referent\"");

    //17. Prover create Proof

    NSString *claim_attr_1_UUID = claimForAttr1[@"referent"];
    NSString *claim_predicate_1_UUID = claimForPredicate1[@"referent"];
    NSString *claim_predicate_2_UUID = claimForPredicate2[@"referent"];

    XCTAssertTrue(claim_attr_1_UUID, @"claim_attr_1_UUID = nil");
    XCTAssertTrue(claim_predicate_1_UUID, @"claim_predicate_1_UUID = nil");
    XCTAssertTrue(claim_predicate_2_UUID, @"claim_predicate_2_UUID = nil");

    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"\
                                     "  \"self_attested_attributes\":{},"\
                                     "  \"requested_attrs\":{\"attr1_referent\":[\"%@\",true]}, "\
                                     "  \"requested_predicates\":{\"predicate1_referent\":\"%@\","\
                                     "                            \"predicate2_referent\":\"%@\"}"\
                                     "}", claim_attr_1_UUID, claim_predicate_1_UUID, claim_predicate_2_UUID];

    NSArray *uniqueClaims = [[AnoncredsUtils sharedInstance] getUniqueClaimsFrom:claims];
    XCTAssertNotNil(uniqueClaims, @"AnoncredsUtils::getUniqueClaimsFrom: failed");

    // obtain unique claims
    NSDictionary *uniqueClaim1 = uniqueClaims[0];
    NSDictionary *uniqueClaim2 = uniqueClaims[1];
    XCTAssertNotNil(uniqueClaim1, @"uniqueClaim1 = nil");
    XCTAssertNotNil(uniqueClaim2, @"uniqueClaim1 = nil");

    // Configure schemasJson
    NSString *schemasJson = [NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                                                       uniqueClaim1[@"referent"], schemas[uniqueClaim1[@"schema_key"][@"name"]],
                                                       uniqueClaim2[@"referent"], schemas[uniqueClaim2[@"schema_key"][@"name"]]];

    // Configure claimDefsJson
    NSString *claimDefsJson = [NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                                                         uniqueClaim1[@"referent"], claimDefs[uniqueClaim1[@"schema_key"][@"name"]],
                                                         uniqueClaim2[@"referent"], claimDefs[uniqueClaim2[@"schema_key"][@"name"]]];

    NSString *revocRegsJson = @"{}";
    NSString *proofJson;

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:proverWalletHandle
                                                                proofReqJson:proofReqJson
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:masterSecretName
                                                               claimDefsJson:claimDefsJson
                                                               revocRegsJson:revocRegsJson
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed");

    NSDictionary *proof = [NSDictionary fromString:proofJson];
    NSString *revealedAttrUUID = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"][1];
    XCTAssertTrue([revealedAttrUUID isEqualToString:@"Alex"]);

    //17. Verifier verify proof

    BOOL isValidJson = NO;

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isValidJson];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValidJson, @"isValidJsoif false");

    [TestUtils cleanupStorage];
}

@end
