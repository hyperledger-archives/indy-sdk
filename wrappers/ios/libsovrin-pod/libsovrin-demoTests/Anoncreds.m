//
//  Anoncreds.m
//  libsovrin-demo
//


#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"

@interface Anoncreds : XCTestCase

@end

@implementation Anoncreds

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

-(void) testAnoncredsWorksForSingleIssuerSingleProverTest
{
    NSLog(@"anoncredsWorksForSingleIssuerSingleProverTest() started...");
    [TestUtils cleanupStorage];
    
    NSString* poolName = @"pool1";
    NSString* issuerWalletName = @"issuer_wallet";
    NSString* proverWalletName = @"prover_wallet";
    NSString* xtype = @"default";
    SovrinHandle issuerWalletHandle = 0;
    SovrinHandle proverWalletHandle = 0;
    NSError *res = nil;
    
    //1. Create Issuer wallet, get wallet handle
    
    res = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:issuerWalletName
                                                                  xtype:xtype
                                                                 handle:&issuerWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Create Prover wallet, get wallet handle
    res = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:poolName
                                                             walletName:proverWalletName
                                                                  xtype:xtype
                                                                 handle:&proverWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createAndOpenWallet() failed");
    
    //3. Issuer create claim definition
    NSString* issuerDid = @"some_issuer_did";
    NSNumber* schemaSeqNo = @1;
    NSNumber* claimDefSeqNo = @1;
    NSString* claimDefJSON = nil;
    NSString* schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson: schemaSeqNo];

    res = [[AnoncredsUtils sharedInstance] createClaimDefinitionAndSetLink: issuerWalletHandle
                                                                    schema: schema
                                                                     seqNo: schemaSeqNo
                                                                   outJson:&claimDefJSON];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");

    //4. Prover create Master Secret
    
    NSString *masterSecretName = @"prover_master_secret";
    
    res = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");
    
    //5. Prover store Claim Offer received from Issuer
    
    NSString *claimOfferJson = [[ AnoncredsUtils sharedInstance] getClaimOfferJson: issuerDid
                                                                             seqNo: claimDefSeqNo ];
    
    res = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer: proverWalletHandle
                                                  claimOfferJson: claimOfferJson ];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed");

    //6. Prover get Claim Offers
    
    NSString *filterJson = [NSString stringWithFormat: @"{ \"issuer_did\":\"%@\"}", issuerDid];
    NSString *claimOffersJson = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:  proverWalletHandle
                                                     filterJson:  filterJson
                                             outClaimOffersJSON: &claimOffersJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverGetClaimOffers() failed");
    NSArray *claimOffers = (NSArray *)[NSDictionary fromString: claimOffersJson];
    
    XCTAssertTrue(claimOffers, @"claimOffers == nil");
    XCTAssertEqual([claimOffers count], 1, @"[claimOffers count] != 1");
    
    NSDictionary *claimOffer1 = claimOffers[0];
    claimOfferJson = [NSDictionary toString: claimOffer1];

    // TODO: add more asserts here
    
    //7. Prover create Claim Request
    NSString* proverDid = @"some_prover_did";
    NSString* claimReq = nil;
    
    res = [[ AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq: proverWalletHandle
                                                               proverDid: proverDid
                                                          claimOfferJson: claimOfferJson
                                                            claimDefJson: claimDefJSON
                                                        masterSecretName: masterSecretName
                                                         outClaimReqJson:&claimReq ];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    NSLog(@"claimReqJson: %@", claimReq);
    

    //8. Issuer create Claim
    NSString *revocRegUpdateJson = nil;
    NSString *xclaimJson = nil;
    
    NSString *claimJson = [[ AnoncredsUtils sharedInstance] getGvtClaimJson];
    
    res = [[AnoncredsUtils sharedInstance] issuerCreateClaim:issuerWalletHandle
                                                   claimJson:claimJson
                                                claimReqJson:claimReq
                                                outClaimJson:&xclaimJson
                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    XCTAssertNotNil(xclaimJson, @"xclaimJson is nil!");
    XCTAssertNotNil(revocRegUpdateJson, @"revocRegUpdateJson is nil!");
    NSLog(@"xclaimJson: %@", xclaimJson);
    NSLog(@"revocRegUpdateJson: %@", revocRegUpdateJson);
    // TODO: revocRegUpdateJson is empty
    
    // 9. Prover store received Claim
    
    // TODO: 110 error
    res = [[AnoncredsUtils sharedInstance] proverStoreClaim: proverWalletHandle
                                                 claimsJson: xclaimJson];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    
    // 10. Prover gets Claims for Proof Request
    NSString *proofReqJson =[ NSString stringWithFormat:@"{"\
                                                         " \"nonce\":\"123432421212\","\
                                                         " \"requested_attrs\":"\
                                                         "             {\"attr1_uuid\":"\
                                                         "                        {"\
                                                         "                          \"schema_seq_no\":%ld,\"name\":\"name\""\
                                                         "                        }"\
                                                         "             },"\
                                                         " \"requested_predicates\":"\
                                                         "             {"\
                                                         "              \"predicate1_uuid\":"\
                                                         "                      {\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}"\
                                                         "             }"\
                                                         "}", [schemaSeqNo integerValue] ];
    NSString *claimsJson = nil;
    
     // TODO: attr1_uuid & predicate1_uuid are empty! why?
    res = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReq:proverWalletHandle
                                                     proofRequestJson:proofReqJson
                                                        outClaimsJson:&claimsJson];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");
    XCTAssertNotNil(claimsJson, @"claimsJson is nil!");

    NSDictionary *claims = [ NSDictionary fromString: claimsJson];
    XCTAssertTrue( claims,  @"serialization failed");
    XCTAssertFalse([claims count] == 0, @"claims array is empty.");
    
    NSDictionary *claims_for_attr_1 = claims[@"attrs" ][@"attr1_uuid"][0];

    XCTAssertTrue( claims_for_attr_1, @"no object for key \"attr1_uuid\"");
    
    NSString *claimUUID = claims_for_attr_1[@"claim_uuid"];
    
    //TODO: add assert here
    
    // 11. Prover create Proof
    NSString* requestedClaimsJson = [ NSString stringWithFormat:@"{"\
                                                                 "  \"self_attested_attributes\":{},"\
                                                                 "  \"requested_attrs\":{\"attr1_uuid\":[\"%@\",true]},"\
                                                                 "  \"requested_predicates\":{\"predicate1_uuid\":\"%@\"}"\
                                                                 "}", claimUUID,claimUUID];

    NSString* schemasJson = [NSString stringWithFormat: @"{\"%@\":%@}", claimUUID, schema];
    
    NSString* claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", claimUUID, claimDefJSON];
    NSString* revocRegsJsons = @"{}";
    
    NSString* proofJson = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverCreateProof: proverWalletHandle
                                                proofReqJson: proofReqJson
                                         requestedClaimsJson: requestedClaimsJson
                                                 schemasJson: schemasJson
                                            masterSecretName: masterSecretName
                                               claimDefsJson: claimDefsJson
                                               revocRegsJson: revocRegsJsons
                                                outProofJson:&proofJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateProof() failed");
    
    BOOL isValid = NO;
    
    res = [[AnoncredsUtils sharedInstance ] verifierVerifyProof:proofReqJson
                                                      proofJson:proofJson
                                                    schemasJson:schemasJson
                                                  claimDefsJson:claimDefsJson
                                                  revocRegsJson:revocRegsJsons
                                                       outValid:&isValid ];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue( isValid, @"isValid == NO");
    NSLog(@"anoncredsWorksForSingleIssuerSingleProverTest() ended...");
    [TestUtils cleanupStorage];
}

-(void) testAnoncredsWorksForMultiplyIssuerSingleProver
{
    [TestUtils cleanupStorage];
    
    NSString* issuer1Did = @"some_issuer1_did";
    NSString* issuer2Did = @"some_issuer2_did";
    NSString* proverDid = @"some_prover_did";
    
    NSString* poolName = @"pool1";
    NSString* issuer1WalletName = @"issuer1_wallet";
    NSString* issuer2WalletName = @"issuer2_wallet";
    NSString* proverWalletName = @"prover_wallet";
    NSString* xtype = @"default";
    NSError*  res = nil;
    
    //1. Issuer1 create wallet, get wallet handles
  
    SovrinHandle issuerGvtWalletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                             walletName:issuer1WalletName
                                                                  xtype:xtype
                                                                 handle:&issuerGvtWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Issuer2 create wallet, get wallet handles
   
    SovrinHandle issuerXyzWalletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                             walletName:issuer2WalletName
                                                                  xtype:xtype
                                                                 handle:&issuerXyzWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createAndOpenWallet() failed");
    
    //3. Prover create wallet, get wallet handles
  
    SovrinHandle proverWalletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                             walletName:proverWalletName
                                                                  xtype:xtype
                                                                 handle:&proverWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    NSMutableDictionary* schemas = [ NSMutableDictionary new];
    NSMutableDictionary* claimDefs = [ NSMutableDictionary new];
    
    //4. Issuer1 create claim definition by gvt schema
  
    NSNumber* gvtSchemaSeqNo = @1;
    NSNumber* gvtClaimDefSeqNo = @1;
    
    NSString* gvtSchema = [[ AnoncredsUtils sharedInstance] getGvtSchemaJson: gvtSchemaSeqNo];
    NSString* gvtClaimDefJson = nil;
    
    res = [[ AnoncredsUtils sharedInstance] createClaimDefinitionAndSetLink:issuerGvtWalletHandle
                                                                     schema:gvtSchema
                                                                      seqNo:gvtClaimDefSeqNo
                                                                    outJson:&gvtClaimDefJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");
    NSLog(@"gvtSchena: %@", gvtSchema);
    NSLog(@"gvtClaimDefJson: %@", gvtClaimDefJson);
    
    [schemas setValue: gvtSchema forKey: [gvtSchemaSeqNo stringValue]];
    [claimDefs setValue: gvtClaimDefJson forKey: [gvtClaimDefSeqNo stringValue]];
    
    //5. Issuer1 create claim definition by xyz schema

    NSNumber* xyzSchemaSeqNo = @2;
    NSNumber* xyzClaimDefSeqNo = @2;
    NSString* xyzClaimDefJson = nil;
    NSString* xyzSchema = [[AnoncredsUtils sharedInstance] getXyzSchemaJson: xyzSchemaSeqNo];
    res = [[AnoncredsUtils sharedInstance] createClaimDefinitionAndSetLink:issuerXyzWalletHandle
                                                                    schema:xyzSchema
                                                                     seqNo:xyzClaimDefSeqNo
                                                                   outJson:&xyzClaimDefJson];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");
    NSLog(@"xyzClaimDefJson: %@", xyzClaimDefJson);
    
    [schemas setValue:xyzSchema forKey:[xyzSchemaSeqNo stringValue]];
    [claimDefs setValue: xyzClaimDefJson forKey:[xyzClaimDefSeqNo stringValue]];
    
    //6. Prover create Master Secret for Issuer1
    
    NSString* masterSecretName1 = @"prover_master_secret_issuer_1";
    res = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName1];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");
    
    //7. Prover create Master Secret for Issuer2
    NSString* masterSecretName2 = @"prover_master_secret_issuer_2";
    
    res = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName2];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");
    
    //8. Prover store Claim Offer received from Issuer1
    NSString* issuer1ClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuer1Did
                                                                                   seqNo:gvtClaimDefSeqNo];

    res = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer1ClaimOfferJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed");
    
    //9. Prover store Claim Offer received from Issuer2
    NSString* issuer2ClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuer2Did
                                                                                   seqNo:xyzClaimDefSeqNo];
    
    
    res = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuer2ClaimOfferJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed");
    
    //10. Prover get Claim Offers
    
    NSString* filterJson = @"{}";
    NSString* claimOffsersJson = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:proverWalletHandle
                                                     filterJson:filterJson
                                             outClaimOffersJSON:&claimOffsersJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils:: proverGetClaimOffers() failed");
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
    
    NSNumber* nd1 = claimOffer1[@"claim_def_seq_no"];
    NSNumber* nd2 = claimOffer2[@"claim_def_seq_no"];
    
    NSString* claimOffer = [nd1 isEqual: gvtClaimDefSeqNo] ? claimOffer1Json : claimOffer2Json;
    
    //11. Prover create Claim Request for gvt claim offer
    
    NSString* gvtClaimReq = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:proverWalletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:gvtClaimDefJson
                                                       masterSecretName:masterSecretName1
                                                        outClaimReqJson:&gvtClaimReq];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //12. Issuer create GVT Claim
    NSString* revocRegUpdateJson = nil;
    NSString* gvtClaimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
                              
    res = [[AnoncredsUtils sharedInstance] issuerCreateClaim:issuerGvtWalletHandle
                                                   claimJson:gvtClaimJson
                                                claimReqJson:gvtClaimReq
                                                outClaimJson:&gvtClaimJson
                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    
    //13. Prover store received GVT Claim

    // TODO: 110 error
    res = [[AnoncredsUtils sharedInstance] proverStoreClaim:proverWalletHandle
                                                 claimsJson:gvtClaimJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    //14. Prover create Claim Request for xyz claim offer
    
    claimOffer = [nd2 isEqual: xyzClaimDefSeqNo] ? claimOffer2Json : claimOffer1Json;
    NSString* xyzClaimReq = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq: proverWalletHandle
                                                              proverDid: proverDid
                                                         claimOfferJson: claimOffer
                                                           claimDefJson: xyzClaimDefJson
                                                       masterSecretName: masterSecretName1
                                                        outClaimReqJson:&xyzClaimReq];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //15. Issuer create XYZ Claim
    
    NSString *xyzClaimJson = [[AnoncredsUtils sharedInstance] getXyzClaimJson];
                              
    res = [[AnoncredsUtils sharedInstance] issuerCreateClaim:issuerXyzWalletHandle
                                                   claimJson:xyzClaimJson
                                                claimReqJson:xyzClaimReq
                                                outClaimJson:&xyzClaimJson
                                       outRevocRegUpdateJSON:&revocRegUpdateJson];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");

    // 16. Prover store received XYZ Claim
    

    res = [[AnoncredsUtils sharedInstance] proverStoreClaim:proverWalletHandle
                                                 claimsJson:xyzClaimJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");

    // 17. Prover gets Claims for Proof Request

    NSString *proofReqJson =[ NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
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
    
    NSString *claimsJson = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReq: proverWalletHandle
                                                     proofRequestJson: proofReqJson
                                                        outClaimsJson:&claimsJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");
    
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
    
    // get schema indexes from claims
    NSInteger claimDefIndexForUniqueClaim1 = [uniqueClaim1[@"claim_def_seq_no"] integerValue];
    NSInteger claimDefIndexForUniqueClaim2 = [uniqueClaim2[@"claim_def_seq_no"] integerValue];
    XCTAssertNotNil(unique_claim_1_UUID, @"claimDefIndexForUniqueClaim1 = nil");
    XCTAssertNotNil(unique_claim_1_UUID, @"claimDefIndexForUniqueClaim2 = nil");
    
    // get claim defines
    NSString *claimDefForUniqueClaim1 = claimDefs[[NSString stringWithFormat:@"%ld", (long)claimDefIndexForUniqueClaim1]];
    NSString *claimDefForUniqueClaim2 = claimDefs[[NSString stringWithFormat:@"%ld", (long)claimDefIndexForUniqueClaim2]];
    XCTAssertNotNil(claimDefForUniqueClaim1, @"claimDefForUniqueClaim1 = nil");
    XCTAssertNotNil(claimDefForUniqueClaim2, @"claimDefForUniqueClaim2 = nil");

    NSString *claimDefsJson = [ NSString stringWithFormat:@"{"\
                             " \"%@\": %@, \"%@\": %@}",
                             unique_claim_1_UUID, claimDefForUniqueClaim1,
                             unique_claim_2_UUID, claimDefForUniqueClaim2];
    
    NSString *revocRegsJson = @"{}";
    NSString *proofJson;
    
    res = [[AnoncredsUtils sharedInstance] proverCreateProof:proverWalletHandle
                                                proofReqJson:proofReqJson
                                         requestedClaimsJson:requestedClaimsJson
                                                 schemasJson:schemasJson
                                            masterSecretName:masterSecretName1
                                               claimDefsJson:claimDefsJson
                                               revocRegsJson:revocRegsJson
                                                outProofJson:&proofJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateProof() failed");
    
    // 19. Verifier verify proof
    
    BOOL isValidJson = NO;
    
    res = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isValidJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    
    [TestUtils cleanupStorage];
}

-(void) testAnoncredsWorksForSingleIssuerMultiplyClaimsSingleProver
{
    [TestUtils cleanupStorage];
    
    NSString* issuerDid = @"some_issuer1_did";
    NSString* proverDid = @"some_prover_did";
    
    NSString* poolName = @"pool1";
    NSString* issuerWalletName = @"issuer_wallet";
    NSString* proverWalletName = @"prover_wallet";
    NSString* xtype = @"default";
    NSError*  res = nil;
    
    //1. Issuer create wallet, get wallet handles
    
    SovrinHandle issuerWalletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                             walletName: issuerWalletName
                                                                  xtype: xtype
                                                                 handle: &issuerWalletHandle];
    
    XCTAssertEqual(res.code, Success, @"WalletUtils::createAndOpenWallet() failed");
    
    //2. Prover create wallet, get wallet handles
    
    SovrinHandle proverWalletHandle = 0;
    res = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName: poolName
                                                             walletName: proverWalletName
                                                                  xtype: xtype
                                                                 handle: &proverWalletHandle];
    
    XCTAssertEqual(res.code, Success, @"WalletUtils::createAndOpenWallet() failed");
    
    NSMutableDictionary* schemas = [ NSMutableDictionary new]; //[Int: String]
    NSMutableDictionary* claimDefs = [ NSMutableDictionary new];
    
    //3. Issuer create claim definition by gvt schema
    
    NSNumber* gvtSchemaSeqNo = @1;
    NSNumber* gvtClaimDefSeqNo = @1;
    
    NSString* gvtSchema = [[ AnoncredsUtils sharedInstance] getGvtSchemaJson: gvtSchemaSeqNo];
    NSString* gvtClaimDefJson = nil;
    
    res = [[ AnoncredsUtils sharedInstance] createClaimDefinitionAndSetLink:issuerWalletHandle
                                                                     schema:gvtSchema
                                                                      seqNo:gvtClaimDefSeqNo
                                                                    outJson:&gvtClaimDefJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");
    
    [schemas setValue: gvtSchema forKey: [gvtSchemaSeqNo stringValue]];
    [claimDefs setValue: gvtClaimDefJson forKey: [gvtClaimDefSeqNo stringValue]];
    
    //4. Issuer create claim definition by xyz schema
    
    NSNumber* xyzSchemaSeqNo = @2;
    NSNumber* xyzClaimDefSeqNo = @2;
    NSString* xyzClaimDefJson = nil;
    NSString* xyzSchema = [[AnoncredsUtils sharedInstance] getXyzSchemaJson: xyzSchemaSeqNo];
    
    res = [[AnoncredsUtils sharedInstance] createClaimDefinitionAndSetLink:issuerWalletHandle
                                                                    schema:xyzSchema
                                                                     seqNo:xyzClaimDefSeqNo
                                                                   outJson:&xyzClaimDefJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");
    
    [schemas setValue:xyzSchema forKey:[xyzSchemaSeqNo stringValue]];
    [claimDefs setValue: xyzClaimDefJson forKey:[xyzClaimDefSeqNo stringValue]];
    
    //5. Prover create Master Secret for Issuer
    
    NSString* masterSecretName = @"prover_master_secret_issuer";
    res = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");
    
    //6. Prover store GVT Claim Offer received from Issuer
    
    NSString* issuerGVTClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                                   seqNo:gvtClaimDefSeqNo];
    
    res = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuerGVTClaimOfferJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaimOffer() failed");
    
    //7. Prover store XYZ Claim Offer received from Issuer
    
    NSString* issuerXYZClaimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                                   seqNo:xyzClaimDefSeqNo];
    
    res = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:proverWalletHandle
                                                  claimOfferJson:issuerXYZClaimOfferJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils:: proverStoreClaimOffer() failed");

    //8. Prover get Claim Offers
    
    NSString* filterJson = [NSString stringWithFormat:@"{"\
                            " \"issuer_did\": \"%@\" "\
                            " }", issuerDid];
    NSString* claimOffsersJson = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:proverWalletHandle
                                                     filterJson:filterJson
                                             outClaimOffersJSON:&claimOffsersJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils:: proverGetClaimOffers() failed");
    
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
    
    NSNumber* claimOffer1_defSeqNo = claimOffer1[@"claim_def_seq_no"];
    NSNumber* claimOffer2_defSeqNo = claimOffer2[@"claim_def_seq_no"];
   // NSNumber* nd2 = claimOffer2[@"claim_def_seq_no"];
    
    NSString* claimOffer = [claimOffer1_defSeqNo isEqual: gvtClaimDefSeqNo] ? claimOffer1Json : claimOffer2Json;
    
    NSString* gvtClaimReq = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:proverWalletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:gvtClaimDefJson
                                                       masterSecretName:masterSecretName
                                                        outClaimReqJson:&gvtClaimReq];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    
    //10. Issuer create GVT Claim
    
    NSString* revocRegUpdateJson = nil;
    NSString* gvtClaimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    
    res = [[AnoncredsUtils sharedInstance] issuerCreateClaim:issuerWalletHandle
                                                   claimJson:gvtClaimJson
                                                claimReqJson:gvtClaimReq
                                                outClaimJson:&gvtClaimJson
                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    
    //11. Prover store received GVT Claim
    
    // TODO: 110 error
    res = [[AnoncredsUtils sharedInstance] proverStoreClaim:proverWalletHandle
                                                 claimsJson:gvtClaimJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    //12. Prover create Claim Request for xyz claim offer
    
    claimOffer = [claimOffer2_defSeqNo isEqual: xyzClaimDefSeqNo] ? claimOffer2Json : claimOffer1Json;
    NSString* xyzClaimReq = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq: proverWalletHandle
                                                              proverDid: proverDid
                                                         claimOfferJson: claimOffer
                                                           claimDefJson: xyzClaimDefJson
                                                       masterSecretName: masterSecretName
                                                        outClaimReqJson: &xyzClaimReq];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");
    
    //13. Issuer create XYZ Claim
    
    NSString *xyzClaimJson = [[AnoncredsUtils sharedInstance] getXyzClaimJson];
    
    res = [[AnoncredsUtils sharedInstance] issuerCreateClaim:issuerWalletHandle
                                                   claimJson:xyzClaimJson
                                                claimReqJson:xyzClaimReq
                                                outClaimJson:&xyzClaimJson
                                       outRevocRegUpdateJSON:&revocRegUpdateJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    
    //14. Prover store received XYZ Claim
    
    res = [[AnoncredsUtils sharedInstance] proverStoreClaim:proverWalletHandle
                                                 claimsJson:xyzClaimJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverStoreClaim() failed");

    //15. Prover gets Claims for Proof Request
    
    
    NSString *proofReqJson =[ NSString stringWithFormat:@"{"\
                             " \"nonce\":\"123432421212\","\
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
                             "}", [gvtSchemaSeqNo integerValue] ];
    
    NSString *claimsJson = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReq: proverWalletHandle
                                                     proofRequestJson: proofReqJson
                                                        outClaimsJson:&claimsJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");

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
    
    // Configure claimDefsJson
    
    // get schema indexes from claims
    NSInteger claimDefIndexForUniqueClaim1 = [uniqueClaim1[@"claim_def_seq_no"] integerValue];
    NSInteger claimDefIndexForUniqueClaim2 = [uniqueClaim2[@"claim_def_seq_no"] integerValue];
    XCTAssertTrue(unique_claim_1_UUID, @"claimDefIndexForUniqueClaim1 = nil");
    XCTAssertTrue(unique_claim_1_UUID, @"claimDefIndexForUniqueClaim2 = nil");
    
    // get claim defines
    NSString *claimDefForUniqueClaim1 = claimDefs[[NSString stringWithFormat:@"%ld", (long)claimDefIndexForUniqueClaim1]];
    NSString *claimDefForUniqueClaim2 = claimDefs[[NSString stringWithFormat:@"%ld", (long)claimDefIndexForUniqueClaim2]];
    XCTAssertTrue(claimDefForUniqueClaim1, @"claimDefForUniqueClaim1 = nil");
    XCTAssertTrue(claimDefForUniqueClaim2, @"claimDefForUniqueClaim2 = nil");
    
    NSString *claimDefsJson = [ NSString stringWithFormat:@"{"\
                               " \"%@\": %@, \"%@\": %@}",
                               unique_claim_1_UUID, claimDefForUniqueClaim1,
                               unique_claim_2_UUID, claimDefForUniqueClaim2];

    NSString *revocRegsJson = @"{}";
    NSString *proofJson;
    
    res = [[AnoncredsUtils sharedInstance] proverCreateProof:proverWalletHandle
                                                proofReqJson:proofReqJson
                                         requestedClaimsJson:requestedClaimsJson
                                                 schemasJson:schemasJson
                                            masterSecretName:masterSecretName
                                               claimDefsJson:claimDefsJson
                                               revocRegsJson:revocRegsJson
                                                outProofJson:&proofJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverCreateProof() failed");
    
    //17. Verifier verify proof
    
    BOOL isValidJson = NO;
    
    res = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJson
                                                      outValid:&isValidJson];
    
    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    
    [TestUtils cleanupStorage];
}

//- (void)testAnoncreds
//{
//    [self anoncredsWorksForSingleIssuerSingleProverTest];
//    //[self anoncredsWorksForMultiplyIssuerSingleProver];
//    //[self anoncredsWorksForSingleIssuerMultiplyClaimsSingleProver];
//}

@end
