//
//  AnoncredsMediumCasesDemos.m
//  libindy-demo
//
//  Created by Anastasia Tarasova on 21.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import <libindy/libindy.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"
#import "NSString+Validation.h"
#import "NSArray+JSON.h"

@interface AnoncredsMediumCasesDemos : XCTestCase

@end

@implementation AnoncredsMediumCasesDemos

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

// MARK: - Demos

- (void)testVerifierVerifyProofWorksForProofDoesNotCorrespondProofRequest
{
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
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@",claimDefJson);
    
    //3. Prover create Master Secret
    NSString *masterSecretName = @"prover_master_secret";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:walletHandle
                                                   masterSecretName:masterSecretName];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret failed");
    
    //4. Prover create Claim Request
    NSString *proverDid = @"BzfFCYk";
    NSString *claimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:[TestUtils issuerDid]
                                                                      schemaSeqNo:schemaSeqNo];
    NSString *claimRequest;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:walletHandle
                                                                                 proverDid:proverDid
                                                                            claimOfferJson:claimOfferJson
                                                                              claimDefJson:claimDefJson
                                                                          masterSecretName:masterSecretName
                                                        outClaimReqJson:&claimRequest];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed");
    XCTAssertTrue([claimRequest isValid], @"invalid claimRequest");
    
    //5. Issuer create Claim
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
    
    // 6. Prover store received Claim
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                 claimsJson:xClaimJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimWithWalletHandle failed");
    
    // 7. Prover gets Claims for Proof Request
    NSString *proofReqJson = [NSString stringWithFormat:@"{"
                              "\"nonce\":\"123432421212\","
                              "\"name\":\"proof_req_1\","
                              "\"version\":\"0.1\","
                              "\"requested_attrs\":{"
                                "\"attr1_uuid\":{"
                                    "\"schema_seq_no\":%@,"
                                    "\"name\":\"name\"}},"
                              "\"requested_predicates\":{}"
                              "}", schemaSeqNo];
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                                      proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson: %@", claimsJson);
    
    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSString *attrUUID = claims[@"attrs"][@"attr1_uuid"][0][@"claim_uuid"];
    XCTAssertTrue([attrUUID isValid], @"invalid attrUUID: %@", attrUUID);
    
    // 8. Prover create Proof
    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"
                                     "\"self_attested_attributes\":{},"
                                     
                                     "\"requested_attrs\":{"
                                            "\"attr1_uuid\":[\"%@\",true]},"
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
    
    // 9. Verifier verify proof
    proofReqJson = [NSString stringWithFormat:@"{"
                                "\"nonce\":\"123432421212\","
                                "\"name\":\"proof_req_1\","
                                "\"version\":\"0.1\","
                                "\"requested_attrs\":{"
                                    "\"attr1_uuid\":{"
                                        "\"schema_seq_no\":%@,"
                                        "\"name\":\"name\"}},"
                               "\"requested_predicates\":{"
                                    "\"predicate1_uuid\":{"
                                        "\"attr_name\":\"age\","
                                        "\"p_type\":\"GE\","
                                        "\"value\":18}}"
                                "}", schemaSeqNo];
    
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

- (void)testAnoncredsWorksForSingleIssuerSingleProver
{
    [TestUtils cleanupStorage];
    
    NSString* poolName = [TestUtils pool];
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
    NSNumber* schemaSeqNo = @1;
    NSString* claimDefJSON = nil;
    NSString* schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson: schemaSeqNo];
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:nil
                                                                          claimDefJson:&claimDefJSON];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");
    
    //4. Prover create Master Secret
    
    NSString *masterSecretName = @"prover_master_secret";
    
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");
    
    //5. Prover store Claim Offer received from Issuer
    
    NSString *claimOfferJson = [[ AnoncredsUtils sharedInstance] getClaimOfferJson: [TestUtils issuerDid]
                                                                       schemaSeqNo:schemaSeqNo];
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer: proverWalletHandle
                                                  claimOfferJson: claimOfferJson ];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed");
    
    //6. Prover get Claim Offers
    
    NSString *filterJson = [NSString stringWithFormat: @"{ \"issuer_did\":\"%@\"}", [TestUtils issuerDid]];
    NSString *claimOffersJson = nil;
    
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:  proverWalletHandle
                                                     filterJson:  filterJson
                                             outClaimOffersJSON: &claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers() failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);
    
    NSArray *claimOffers = (NSArray *)[NSDictionary fromString: claimOffersJson];
    
    XCTAssertTrue(claimOffers, @"claimOffers == nil");
    XCTAssertEqual([claimOffers count], 1, @"[claimOffers count] != 1");
    
    NSDictionary *claimOffer1 = claimOffers[0];
    claimOfferJson = [NSDictionary toString: claimOffer1];
    
    //7. Prover create Claim Request
    NSString* proverDid = @"BzfFCYk";
    NSString* claimReq = nil;
    
    ret = [[ AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq: proverWalletHandle
                                                               proverDid: proverDid
                                                          claimOfferJson: claimOfferJson
                                                            claimDefJson: claimDefJSON
                                                        masterSecretName: masterSecretName
                                                         outClaimReqJson:&claimReq ];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    XCTAssertTrue([claimReq isValid], @"invalid claimRequest: %@", claimReq);
    NSLog(@"claimReqJson: %@", claimReq);
    
    
    //8. Issuer create Claim
    NSString *xclaimJson = nil;
    
    NSString *claimJson = [[ AnoncredsUtils sharedInstance] getGvtClaimJson];
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:claimReq
                                                                   claimJson:claimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xclaimJson
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    XCTAssertTrue([xclaimJson isValid], @"invalid xClaimJson: %@", xclaimJson);
    NSLog(@"xclaimJson: %@", xclaimJson);
    
    // 9. Prover store received Claim
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson: xclaimJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    
    // 10. Prover gets Claims for Proof Request
    NSString *proofReqJson =[ NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attrs\":"\
                             "             {\"attr1_uuid\":"\
                             "                        {"\
                             "                          \"schema_seq_no\":%@,\"name\":\"name\""\
                             "                        }"\
                             "             },"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_uuid\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}"\
                             "             }"\
                             "}", schemaSeqNo ];
    
    NSString *claimsJson = nil;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:proverWalletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson: %@", claimsJson);
    
    NSDictionary *claims = [ NSDictionary fromString: claimsJson];
    NSString *claimUUID = claims[@"attrs"][@"attr1_uuid"][0][@"claim_uuid"];
    
    // 11. Prover create Proof
    NSString* requestedClaimsJson = [ NSString stringWithFormat:@"{"\
                                     "  \"self_attested_attributes\":{\"self1\":\"value\"},"\
                                     "  \"requested_attrs\":{\"attr1_uuid\":[\"%@\",true]},"\
                                     "  \"requested_predicates\":{\"predicate1_uuid\":\"%@\"}"\
                                     "}", claimUUID, claimUUID];
    
    NSString* schemasJson = [NSString stringWithFormat: @"{\"%@\":%@}", claimUUID, schema];
    
    NSString* claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, claimDefJSON];
    NSString* revocRegsJsons = @"{}";
    
    NSString* proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle: proverWalletHandle
                                                                proofReqJson: proofReqJson
                                                         requestedClaimsJson: requestedClaimsJson
                                                                 schemasJson: schemasJson
                                                            masterSecretName: masterSecretName
                                                               claimDefsJson: claimDefsJson
                                                               revocRegsJson: revocRegsJsons
                                                                outProofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed");
    XCTAssertTrue([proofJson isValid], @"invalid proofJson: %@", proofJson);
    
    NSDictionary *proof = [NSDictionary fromString:proofJson];
    NSString *revealedAttrUUID = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_uuid"][1];
    XCTAssertTrue([revealedAttrUUID isEqualToString:@"Alex"]);
    
    NSString *attestedAttrUUID = proof[@"requested_proof"][@"self_attested_attrs"][@"self1"];
    XCTAssertTrue([attestedAttrUUID isEqualToString:@"value"]);
    
    // 12. Verifier verify proof
    BOOL isValid = NO;
    
    ret = [[AnoncredsUtils sharedInstance ] verifierVerifyProof:proofReqJson
                                                      proofJson:proofJson
                                                    schemasJson:schemasJson
                                                  claimDefsJson:claimDefsJson
                                                  revocRegsJson:revocRegsJsons
                                                       outValid:&isValid ];
    
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue( isValid, @"isValid == NO");
    [TestUtils cleanupStorage];
}

- (void)testAnoncredsWorksForMultiplyIssuerSingleProver
{
    [TestUtils cleanupStorage];
    
    NSString *issuer2Did = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString *proverDid = @"BzfFCYk";
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
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                                  xtype:nil
                                                                 handle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");
    
    NSMutableDictionary* schemas = [ NSMutableDictionary new];
    NSMutableDictionary* claimDefs = [ NSMutableDictionary new];
    
    //4. Issuer1 create claim definition by gvt schema
    
    NSNumber* gvtSchemaSeqNo = @1;
    NSString* gvtSchema = [[ AnoncredsUtils sharedInstance] getGvtSchemaJson: gvtSchemaSeqNo];
    NSString* gvtClaimDefJson = nil;
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerGvtWalletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:gvtSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&gvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle() failed");
    NSLog(@"gvtSchena: %@", gvtSchema);
    
    [schemas setValue: gvtSchema forKey: [gvtSchemaSeqNo stringValue]];
    [claimDefs setValue: gvtClaimDefJson forKey: [TestUtils issuerDid]];
    
    //5. Issuer1 create claim definition by xyz schema
    
    NSNumber* xyzSchemaSeqNo = @2;
    NSString* xyzClaimDefJson = nil;
    NSString* xyzSchema = [[AnoncredsUtils sharedInstance] getXyzSchemaJson: xyzSchemaSeqNo];
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerXyzWalletHandle
                                                                             issuerDid:issuer2Did
                                                                            schemaJson:xyzSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&xyzClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");
    NSLog(@"xyzClaimDefJson: %@", xyzClaimDefJson);
    
    schemas[[xyzSchemaSeqNo stringValue]] = xyzSchema;
    claimDefs[issuer2Did] = xyzClaimDefJson;
    
    //6. Prover create Master Secret for Issuer1
    
    NSString* masterSecretName1 = @"prover_master_secret_issuer_1";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName1];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed for issuer 1");
    
    //7. Prover create Master Secret for Issuer2
    NSString* masterSecretName2 = @"prover_master_secret_issuer_2";
    
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName2];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed for issuer 2");
    
    //8. Prover store Claim Offer received from Issuer1
    NSString* issuer1ClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:[TestUtils issuerDid]
                                                                             schemaSeqNo:gvtSchemaSeqNo];
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer1ClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed for issuer 1");
    
    //9. Prover store Claim Offer received from Issuer2
    NSString* issuer2ClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuer2Did
                                                                             schemaSeqNo:xyzSchemaSeqNo];
    
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer2ClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed for issuer 2");
    
    //10. Prover get Claim Offers
    
    NSString* filterJson = @"{}";
    NSString* claimOffsersJson = nil;
    
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:proverWalletHandle
                                                     filterJson:filterJson
                                             outClaimOffersJSON:&claimOffsersJson];
    
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverGetClaimOffers() failed");
    NSLog(@"claimOffsersJson: %@", claimOffsersJson);
    
    NSArray *claimOffers = (NSArray *)[NSDictionary fromString: claimOffsersJson];
    
    XCTAssertTrue(claimOffers, @"claimOffers == nil");
    XCTAssertEqual([claimOffers count], 2, @"[claimOffers count] != 2");
    
    NSDictionary *claimOffer1 = claimOffers[0];
    NSDictionary *claimOffer2 = claimOffers[1];
    
    XCTAssertTrue(claimOffer1, @"claimOffer1 == nil");
    XCTAssertTrue(claimOffer2, @"claimOffer2 == nil");
    
    NSString* claimOffer1Json = [NSDictionary toString: claimOffer1];
    NSString* claimOffer2Json = [NSDictionary toString: claimOffer2];
    
    XCTAssertTrue(claimOffer1Json, @"claimOffer1Json == nil");
    XCTAssertTrue(claimOffer2Json, @"claimOffer2Json == nil");
    
    NSString* claimOffer1_issuerDid = claimOffer1[@"issuer_did"];
    NSString* claimOffer2_issuerDid = claimOffer2[@"issuer_did"];
    
    NSString* claimOffer = [claimOffer1_issuerDid isEqual: [TestUtils issuerDid]] ? claimOffer1Json : claimOffer2Json;
    
    //11. Prover create Claim Request for gvt claim offer
    
    NSString* gvtClaimReq;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:proverWalletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:gvtClaimDefJson
                                                       masterSecretName:masterSecretName1
                                                        outClaimReqJson:&gvtClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    
    //12. Issuer create GVT Claim
    NSString* revocRegUpdateJson;
    NSString* gvtClaimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerGvtWalletHandle
                                                                claimReqJson:gvtClaimReq
                                                                   claimJson:gvtClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&gvtClaimJson
                                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerGvtWalletHandle");
    
    //13. Prover store received GVT Claim
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:gvtClaimJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    //14. Prover create Claim Request for xyz claim offer
    
    claimOffer = [claimOffer2_issuerDid isEqual: issuer2Did] ? claimOffer2Json : claimOffer1Json;
    NSString* xyzClaimReq;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq: proverWalletHandle
                                                              proverDid: proverDid
                                                         claimOfferJson: claimOffer
                                                           claimDefJson: xyzClaimDefJson
                                                       masterSecretName: masterSecretName1
                                                        outClaimReqJson:&xyzClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    
    //15. Issuer create XYZ Claim
    
    NSString *xyzClaimJson = [[AnoncredsUtils sharedInstance] getXyzClaimJson];
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerXyzWalletHandle
                                                                claimReqJson:xyzClaimReq
                                                                   claimJson:xyzClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xyzClaimJson
                                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for issuerXyzWalletHandle");
    
    // 16. Prover store received XYZ Claim
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:xyzClaimJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed on step 16");
    
    // 17. Prover gets Claims for Proof Request
    
    NSString *proofReqJson =[ NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","\
                             " \"version\":\"0.1\","\
                             " \"requested_attrs\":"\
                             "             {\"attr1_uuid\":"\
                             "                        {"\
                             "                          \"schema_seq_no\":%d,\"name\":\"name\""\
                             "                        },"\
                             "              \"attr2_uuid\":"\
                             "                        {"\
                             "                          \"schema_seq_no\":%d,\"name\":\"status\""\
                             "                        }"\
                             "             },"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_uuid\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18},"\
                             "              \"predicate2_uuid\":"\
                             "                      {\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}"\
                             "             }"\
                             "}", [gvtSchemaSeqNo intValue], [xyzSchemaSeqNo intValue] ];
    
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:proverWalletHandle
                                                                     proofRequestJson:proofReqJson
                                                                        outClaimsJson:&claimsJson];
    
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");
    
    NSDictionary *claims = [ NSDictionary fromString: claimsJson];
    XCTAssertTrue(claims,  @"serialization failed");
    
    NSDictionary *claimForAttr1 = claims[@"attrs"][@"attr1_uuid"][0];
    NSDictionary *claimForAttr2 = claims[@"attrs"][@"attr2_uuid"][0];
    
    XCTAssertTrue( claimForAttr1, @"no object for key \"attr1_uuid\"");
    XCTAssertTrue( claimForAttr2, @"no object for key \"attr2_uuid\"");
    
    NSDictionary *claimForPredicate1 = claims[@"predicates"][@"predicate1_uuid"][0];
    NSDictionary *claimForPredicate2 = claims[@"predicates"][@"predicate2_uuid"][0];
    
    XCTAssertTrue( claimForPredicate1, @"no object for key \"predicate1_uuid\"");
    XCTAssertTrue( claimForPredicate2, @"no object for key \"predicate2_uuid\"");
    
    // 18. Prover create Proof
    
    NSString *claim_attr_1_UUID = claimForAttr1[@"claim_uuid"];
    NSString *claim_attr_2_UUID = claimForAttr2[@"claim_uuid"];
    NSString *claim_predicate_1_UUID = claimForPredicate1[@"claim_uuid"];
    NSString *claim_predicate_2_UUID = claimForPredicate2[@"claim_uuid"];
    
    XCTAssertNotNil( claim_attr_1_UUID, @"claim_attr_1_UUID = nil");
    XCTAssertNotNil( claim_attr_2_UUID, @"claim_attr_2_UUID = nil");
    XCTAssertNotNil( claim_predicate_1_UUID, @"claim_predicate_1_UUID = nil");
    XCTAssertNotNil( claim_predicate_2_UUID, @"claim_predicate_2_UUID = nil");
    
    NSString *requestedClaimsJson = [ NSString stringWithFormat:@"{"\
                                     "  \"self_attested_attributes\":{},"\
                                     "  \"requested_attrs\":{\"attr1_uuid\":[\"%@\",true], "\
                                     "                       \"attr2_uuid\":[\"%@\",true]},"\
                                     "  \"requested_predicates\":{\"predicate1_uuid\":\"%@\","\
                                     "                            \"predicate2_uuid\":\"%@\"}"\
                                     "}", claim_attr_1_UUID, claim_attr_2_UUID,
                                     claim_predicate_1_UUID, claim_predicate_2_UUID];
    
    
    NSArray *uniqueClaims = [[AnoncredsUtils sharedInstance] getUniqueClaimsFrom:claims];
    XCTAssertNotNil(uniqueClaims, @"AnoncredsUtils::getUniqueClaimsFrom: failed");
    
    // obtain unique claims
    NSDictionary *uniqueClaim1 = uniqueClaims[0];
    NSDictionary *uniqueClaim2 = uniqueClaims[1];
    XCTAssertNotNil(uniqueClaim1, @"uniqueClaim1 = nil");
    XCTAssertNotNil(uniqueClaim2, @"uniqueClaim1 = nil");
    
    // Configure schemasJson
    // get claim's uuids
    NSString *unique_claim_1_UUID = uniqueClaim1[@"claim_uuid"];
    NSString *unique_claim_2_UUID = uniqueClaim2[@"claim_uuid"];
    XCTAssertNotNil(unique_claim_1_UUID, @"unique_claim_1_UUID = nil");
    XCTAssertNotNil(unique_claim_1_UUID, @"unique_claim_2_UUID = nil");
    
    // get issuer dids
    NSString *unique_claim_1_issuer_did = uniqueClaim1[@"issuer_did"];
    NSString *unique_claim_2_issuer_did = uniqueClaim2[@"issuer_did"];
    XCTAssertNotNil(unique_claim_1_issuer_did, @"unique_claim_1_issuer_did = nil");
    XCTAssertNotNil(unique_claim_2_issuer_did, @"unique_claim_2_issuer_did = nil");
    
    // get schema indexes from claims
    NSInteger unique_claim_1_schema_index = [uniqueClaim1[@"schema_seq_no"] integerValue];
    NSInteger unique_claim_2_schema_index = [uniqueClaim2[@"schema_seq_no"] integerValue];
    XCTAssertNotNil(unique_claim_1_UUID, @"unique_claim_1_schema_index = nil");
    XCTAssertNotNil(unique_claim_1_UUID, @"unique_claim_2_schema_index = nil");
    
    
    // get schemas
    NSString *schemaForUniqueClaim1 = schemas[[NSString stringWithFormat:@"%ld", (long)unique_claim_1_schema_index]];
    NSString *schemaForUniqueClaim2 = schemas[[NSString stringWithFormat:@"%ld", (long)unique_claim_2_schema_index]];
    XCTAssertNotNil(schemaForUniqueClaim1, @"schemaForUniqueClaim1 = nil");
    XCTAssertNotNil(schemaForUniqueClaim2, @"schemaForUniqueClaim2 = nil");
    
    NSString *schemasJson = [ NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                             unique_claim_1_UUID, schemaForUniqueClaim1,
                             unique_claim_2_UUID, schemaForUniqueClaim2];
    
    // Configure claimDefsJson
    
    // get claim defines
    NSString *claimDefForUniqueClaim1 = claimDefs[unique_claim_1_issuer_did];
    NSString *claimDefForUniqueClaim2 = claimDefs[unique_claim_2_issuer_did];
    XCTAssertNotNil(claimDefForUniqueClaim1, @"claimDefForUniqueClaim1 = nil");
    XCTAssertNotNil(claimDefForUniqueClaim2, @"claimDefForUniqueClaim2 = nil");
    
    NSString *claimDefsJson = [ NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                               unique_claim_1_UUID, claimDefForUniqueClaim1,
                               unique_claim_2_UUID, claimDefForUniqueClaim2];
    
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
    
    // 19. Verifier verify proof
    
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


- (void)testAnoncredsWorksForSingleIssuerMultiplyClaimsSingleProver
{
    [TestUtils cleanupStorage];
    
    NSString* issuerDid = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString* proverDid = @"BzfFCYk";
    
    NSString* poolName = [TestUtils pool];
    NSError*  ret = nil;
    
    //1. Issuer create wallet, get wallet handles
    
    IndyHandle issuerWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                                  xtype: nil
                                                                 handle: &issuerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");
    
    //2. Prover create wallet, get wallet handles
    
    IndyHandle proverWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                                  xtype: nil
                                                                 handle: &proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");
    
    NSMutableDictionary* schemas = [ NSMutableDictionary new]; //[Int: String]
    NSMutableDictionary* claimDefs = [ NSMutableDictionary new];
    
    //3. Issuer create claim definition by gvt schema
    
    NSNumber* gvtSchemaSeqNo = @1;
    
    NSString* gvtSchema = [[ AnoncredsUtils sharedInstance] getGvtSchemaJson: gvtSchemaSeqNo];
    NSString* gvtClaimDefJson = nil;
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:issuerDid
                                                                            schemaJson:gvtSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&gvtClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed on step 3");
    
    [schemas setValue: gvtSchema forKey: [gvtSchemaSeqNo stringValue]];
    NSString *key = [[AnoncredsUtils sharedInstance] getClaimDefIdForIssuerDid:issuerDid
                                                                   schemaSeqNo:gvtSchemaSeqNo];
    [claimDefs setValue: gvtClaimDefJson forKey: key];
    
    //4. Issuer create claim definition by xyz schema
    
    NSNumber* xyzSchemaSeqNo = @2;
    NSString* xyzClaimDefJson;
    NSString* xyzSchema = [[AnoncredsUtils sharedInstance] getXyzSchemaJson: xyzSchemaSeqNo];
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:issuerWalletHandle
                                                                             issuerDid:issuerDid
                                                                            schemaJson:xyzSchema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:&xyzClaimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle() failed on step 4 for xyzSchema.");
    
    [schemas setValue:xyzSchema forKey:[xyzSchemaSeqNo stringValue]];
    key = [[AnoncredsUtils sharedInstance] getClaimDefIdForIssuerDid:issuerDid
                                                         schemaSeqNo:xyzSchemaSeqNo];
    [claimDefs setValue: xyzClaimDefJson forKey:key];
    
    //5. Prover create Master Secret for Issuer
    
    NSString* masterSecretName = @"prover_master_secret_issuer";
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");
    
    //6. Prover store GVT Claim Offer received from Issuer
    
    NSString* issuerGVTClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                               schemaSeqNo:gvtSchemaSeqNo];
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuerGVTClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed on step 6");
    
    //7. Prover store XYZ Claim Offer received from Issuer
    
    NSString* issuerXYZClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                               schemaSeqNo:xyzSchemaSeqNo];
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuerXYZClaimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed on step 7");
    
    //8. Prover get Claim Offers
    
    NSString* filterJson = [NSString stringWithFormat:@"{"\
                            " \"issuer_did\": \"%@\" "\
                            " }", issuerDid];
    NSString* claimOffsersJson = nil;
    
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:proverWalletHandle
                                                     filterJson:filterJson
                                             outClaimOffersJSON:&claimOffsersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils:: proverGetClaimOffers() failed");
    
    NSArray *claimOffers = (NSArray *)[NSDictionary fromString: claimOffsersJson];
    
    XCTAssertTrue(claimOffers, @"claimOffers == nil");
    XCTAssertEqual([claimOffers count], 2, @"[claimOffers count] != 2");
    
    NSDictionary *claimOffer1 = [claimOffers objectAtIndex: 0];
    NSDictionary *claimOffer2 = [claimOffers objectAtIndex: 1];
    
    XCTAssertTrue(claimOffer1, @"claimOffer1 == nil");
    XCTAssertTrue(claimOffer2, @"claimOffer2 == nil");
    
    NSString* claimOffer1Json = [NSDictionary toString: claimOffer1];
    NSString* claimOffer2Json = [NSDictionary toString: claimOffer2];
    
    XCTAssertTrue(claimOffer1Json, @"claimOffer1Json == nil");
    XCTAssertTrue(claimOffer2Json, @"claimOffer2Json == nil");
    
    //9. Prover create Claim Request for gvt claim offer
    
    NSNumber* claimOffer1_schemaSeqNo = claimOffer1[@"schema_seq_no"];
    NSNumber* claimOffer2_schemaSeqNo = claimOffer2[@"schema_seq_no"];
    
    NSString* claimOffer = [claimOffer1_schemaSeqNo isEqual: gvtSchemaSeqNo] ? claimOffer1Json : claimOffer2Json;
    
    NSString* gvtClaimReq = nil;
    
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:proverWalletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:gvtClaimDefJson
                                                       masterSecretName:masterSecretName
                                                        outClaimReqJson:&gvtClaimReq];
    
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    
    //10. Issuer create GVT Claim
    
    NSString* revocRegUpdateJson = nil;
    NSString* gvtClaimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:gvtClaimReq
                                                                   claimJson:gvtClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&gvtClaimJson
                                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for gvtClaimReq ");
    
    //11. Prover store received GVT Claim
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:gvtClaimJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    //12. Prover create Claim Request for xyz claim offer
    
    claimOffer = [claimOffer2_schemaSeqNo isEqual: xyzSchemaSeqNo] ? claimOffer2Json : claimOffer1Json;
    NSString* xyzClaimReq = nil;
    
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq: proverWalletHandle
                                                              proverDid: proverDid
                                                         claimOfferJson: claimOffer
                                                           claimDefJson: xyzClaimDefJson
                                                       masterSecretName: masterSecretName
                                                        outClaimReqJson: &xyzClaimReq];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    
    //13. Issuer create XYZ Claim
    
    NSString *xyzClaimJson = [[AnoncredsUtils sharedInstance] getXyzClaimJson];
    
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:issuerWalletHandle
                                                                claimReqJson:xyzClaimReq
                                                                   claimJson:xyzClaimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:&xyzClaimJson
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed for xyzClaimReq");
    
    //14. Prover store received XYZ Claim
    
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:proverWalletHandle
                                                                 claimsJson:xyzClaimJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    //15. Prover gets Claims for Proof Request
    
    NSString *proofReqJson =[ NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
                             " \"name\":\"proof_req_1\","
                             " \"version\":\"0.1\","
                             " \"requested_attrs\":"\
                             "             {\"attr1_uuid\":"\
                             "                        {"\
                             "                          \"schema_seq_no\":%ld,\"name\":\"name\""\
                             "                        }},"\
                             " \"requested_predicates\":"\
                             "             {"\
                             "              \"predicate1_uuid\":"\
                             "                      {\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18},"\
                             "              \"predicate2_uuid\":"\
                             "                      {\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}"\
                             "             }"\
                             "}", (long)[gvtSchemaSeqNo integerValue] ];
    
    NSString *claimsJson = nil;
    
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle: proverWalletHandle
                                                                     proofRequestJson: proofReqJson
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");
    
    NSDictionary *claims = [ NSDictionary fromString: claimsJson];
    XCTAssertTrue(claims,  @"serialization failed");
    XCTAssertEqual([claims[@"attrs"] count], 1, @"claims.attrs.count != 1");
    XCTAssertEqual([claims[@"predicates"] count], 2, @"claims.predicates.count != 1");
    
    NSDictionary *claimForAttr1 = claims[@"attrs"][@"attr1_uuid"][0];
    XCTAssertTrue( claimForAttr1, @"no object for key \"attr1_uuid\"");
    
    NSDictionary *claimForPredicate1 = claims[@"predicates"][@"predicate1_uuid"][0];
    NSDictionary *claimForPredicate2 = claims[@"predicates"][@"predicate2_uuid"][0];
    
    XCTAssertTrue( claimForPredicate1, @"no object for key \"predicate1_uuid\"");
    XCTAssertTrue( claimForPredicate2, @"no object for key \"predicate2_uuid\"");
    
    //16. Prover create Proof
    
    NSString *claim_attr_1_UUID = claimForAttr1[@"claim_uuid"];
    NSString *claim_predicate_1_UUID = claimForPredicate1[@"claim_uuid"];
    NSString *claim_predicate_2_UUID = claimForPredicate2[@"claim_uuid"];
    
    XCTAssertTrue( claim_attr_1_UUID, @"claim_attr_1_UUID = nil");
    XCTAssertTrue( claim_predicate_1_UUID, @"claim_predicate_1_UUID = nil");
    XCTAssertTrue( claim_predicate_2_UUID, @"claim_predicate_2_UUID = nil");
    
    NSString *requestedClaimsJson = [ NSString stringWithFormat:@"{"\
                                     "  \"self_attested_attributes\":{},"\
                                     "  \"requested_attrs\":{\"attr1_uuid\":[\"%@\",true]}, "\
                                     "  \"requested_predicates\":{\"predicate1_uuid\":\"%@\","\
                                     "                            \"predicate2_uuid\":\"%@\"}"\
                                     "}", claim_attr_1_UUID, claim_predicate_1_UUID, claim_predicate_2_UUID ];
    
    NSArray *uniqueClaims = [[AnoncredsUtils sharedInstance] getUniqueClaimsFrom:claims];
    XCTAssertTrue(uniqueClaims, @"AnoncredsUtils::getUniqueClaimsFrom: failed");
    
    // obtain unique claims
    NSDictionary *uniqueClaim1 = uniqueClaims[0];
    NSDictionary *uniqueClaim2 = uniqueClaims[1];
    XCTAssertTrue(uniqueClaim1, @"uniqueClaim1 = nil");
    XCTAssertTrue(uniqueClaim2, @"uniqueClaim1 = nil");
    
    // Configure schemasJson
    // get claim's uuids
    NSString *unique_claim_1_UUID = uniqueClaim1[@"claim_uuid"];
    NSString *unique_claim_2_UUID = uniqueClaim2[@"claim_uuid"];
    XCTAssertTrue(unique_claim_1_UUID, @"unique_claim_1_UUID = nil");
    XCTAssertTrue(unique_claim_1_UUID, @"unique_claim_2_UUID = nil");
    
    // get schema indexes from claims
    NSInteger unique_claim_1_schema_index = [uniqueClaim1[@"schema_seq_no"] integerValue];
    NSInteger unique_claim_2_schema_index = [uniqueClaim2[@"schema_seq_no"] integerValue];
    XCTAssertTrue(unique_claim_1_UUID, @"unique_claim_1_schema_index = nil");
    XCTAssertTrue(unique_claim_1_UUID, @"unique_claim_2_schema_index = nil");

    // get schemas
    NSString *schemaForUniqueClaim1 = schemas[[NSString stringWithFormat:@"%ld", (long)unique_claim_1_schema_index]];
    NSString *schemaForUniqueClaim2 = schemas[[NSString stringWithFormat:@"%ld", (long)unique_claim_2_schema_index]];
    XCTAssertTrue(schemaForUniqueClaim1, @"schemaForUniqueClaim1 = nil");
    XCTAssertTrue(schemaForUniqueClaim2, @"schemaForUniqueClaim2 = nil");
    
    NSString *schemasJson = [ NSString stringWithFormat:@"{"\
                             " \"%@\": %@, "\
                             " \"%@\": %@}",
                             unique_claim_1_UUID, schemaForUniqueClaim1,
                             unique_claim_2_UUID, schemaForUniqueClaim2];
    
    // -------- Configure claimDefsJson -------------------------
    
    
    // get claim def ids
    NSString *claimDefId1 = [[AnoncredsUtils sharedInstance] getClaimDefIdForIssuerDid:uniqueClaim1[@"issuer_did"]
                                                                           schemaSeqNo:uniqueClaim1[@"schema_seq_no"]];
    NSString *claimDefId2 = [[AnoncredsUtils sharedInstance] getClaimDefIdForIssuerDid:uniqueClaim2[@"issuer_did"]
                                                                           schemaSeqNo:uniqueClaim2[@"schema_seq_no"]];
    
    NSString *claimDefsJson = [ NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                               unique_claim_1_UUID, claimDefs[claimDefId1],
                               unique_claim_2_UUID, claimDefs[claimDefId2]];
    
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
    NSString *revealedAttrUUID = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_uuid"][1];
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
