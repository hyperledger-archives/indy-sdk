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

-(void) anoncredsWorksForSingleIssuerSingleProverTest
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
    
    res = [[WalletUtils sharedInstance] createWallet:poolName walletName:issuerWalletName xtype:xtype handle:&issuerWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createWallet() failed");

    //2. Create Prover wallet, get wallet handle
    res = [[WalletUtils sharedInstance] createWallet:poolName walletName:proverWalletName xtype:xtype handle:&proverWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createWallet() failed");
    
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
    
    NSString *claimOfferJson = [[ AnoncredsUtils sharedInstance] getClaimOfferJson: issuerDid seqNo: claimDefSeqNo ];
    
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

    //8. Issuer create Claim
    NSString *revocRegUpdateJson = nil;
    NSString *xclaimJson = nil;
    
    NSString *claimJson = [[ AnoncredsUtils sharedInstance] getGvtClaimJson];

    res = [[AnoncredsUtils sharedInstance] issuerCreateClaim: issuerWalletHandle
                                                claimReqJson: claimReq
                                                   claimJson: claimJson
                                       outRevocRegUpdateJSON:&revocRegUpdateJson
                                                outClaimJson:&xclaimJson ];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    
    // 9. Prover store received Claim
    
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
    
    res = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReq:proverWalletHandle
                                                     proofRequestJson:proofReqJson
                                                        outClaimsJson:&claimsJson];

    XCTAssertEqual(res.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");

    NSDictionary *claims = [NSJSONSerialization JSONObjectWithData:[NSData dataWithBytes:[claimsJson UTF8String]
                                                                                  length:[claimsJson length]]
                                                           options:kNilOptions
                                                             error: &res];
    XCTAssertTrue( claims,  @"serialization failed");
    
    NSDictionary *claims_for_attr_1 = [[ [claims objectForKey: @"attrs" ] objectForKey: @"attr1_uuid"] objectAtIndex: 0 ];

    XCTAssertTrue( claims_for_attr_1, @"no object for key \"attr1_uuid\"");
    
    NSString *claimUUID = [claims_for_attr_1 objectForKey:@"claim_uuid"];
    
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

-(void) anoncredsWorksForMultiplyIssuerSingleProver
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
    res = [[WalletUtils sharedInstance] createWallet: poolName walletName:issuer1WalletName xtype:xtype handle:&issuerGvtWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createWallet() failed");

    //2. Issuer2 create wallet, get wallet handles
   
    SovrinHandle issuerXyzWalletHandle = 0;
    res = [[WalletUtils sharedInstance] createWallet: poolName walletName:issuer2WalletName xtype:xtype handle:&issuerXyzWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createWallet() failed");
    
    //3. Prover create wallet, get wallet handles
  
    SovrinHandle proverWalletHandle = 0;
    res = [[WalletUtils sharedInstance] createWallet: poolName walletName:proverWalletName xtype:xtype handle:&proverWalletHandle];
    XCTAssertEqual(res.code, Success, @"WalletUtils::createWallet() failed");

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

    NSArray *claimOffers = [NSJSONSerialization JSONObjectWithData:[NSData dataWithBytes:[claimOffsersJson UTF8String]
                                                                                       length:[claimOffsersJson length]]
                                                                options:kNilOptions
                                                                  error: &res];
    XCTAssertTrue(claimOffers, @"claimOffers == nil");
    XCTAssertEqual([claimOffers count], 2, @"[claimOffers count] != 2");
#if 0
    
    let claim_offers: Vec<ClaimOffer> = serde_json::from_str(&claim_offers_json).unwrap();
    assert_eq!(2, claim_offers.len());
    
    let claim_offer_1 = claim_offers[0].clone();
    let claim_offer_2 = claim_offers[1].clone();
    
    let claim_offer_1_json = serde_json::to_string(&claim_offer_1).unwrap();
    let claim_offer_2_json = serde_json::to_string(&claim_offer_2).unwrap();
    
    //11. Prover create Claim Request for gvt claim offer
    let claim_offer = if claim_offer_1.claim_def_seq_no == gvt_claim_def_seq_no { claim_offer_1_json.clone() } else { claim_offer_2_json.clone() };
    
    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer,
                                                                &gvt_claim_def_json,
                                                                master_secret_name_1);
    assert!(res.is_ok());
    let gvt_claim_req = res.unwrap();
    
    //12. Issuer create GVT Claim
    let gvt_claim_json = AnoncredsUtils::get_gvt_claim_json();
    let res = AnoncredsUtils::issuer_create_claim(issuer_gvt_wallet_handle,
                                                  &gvt_claim_req,
                                                  &gvt_claim_json);
    assert!(res.is_ok());
    let (revoc_reg_update_json, gvt_claim_json) = res.unwrap();
    
    //13. Prover store received GVT Claim
    let res = AnoncredsUtils::prover_store_claim(prover_wallet_handle, &gvt_claim_json);
    assert!(res.is_ok());
    
    //14. Prover create Claim Request for xyz claim offer
    let claim_offer = if claim_offer_2.claim_def_seq_no == xyz_claim_def_seq_no { claim_offer_2_json.clone() } else { claim_offer_1_json.clone() };
    let res = AnoncredsUtils::prover_create_and_store_claim_req(prover_wallet_handle,
                                                                prover_did,
                                                                &claim_offer,
                                                                &xyz_claim_def_json,
                                                                master_secret_name_1);
    assert!(res.is_ok());
    let xyz_claim_req = res.unwrap();
    
    //15. Issuer create XYZ Claim
    let xyz_claim_json = AnoncredsUtils::get_xyz_claim_json();
    let res = AnoncredsUtils::issuer_create_claim(issuer_xyz_wallet_handle,
                                                  &xyz_claim_req,
                                                  &xyz_claim_json);
    assert!(res.is_ok());
    let (revoc_reg_update_json, xyz_claim_json) = res.unwrap();
    
    // 16. Prover store received XYZ Claim
    let res = AnoncredsUtils::prover_store_claim(prover_wallet_handle, &xyz_claim_json);
    assert!(res.is_ok());
    
    
    // 17. Prover gets Claims for Proof Request
    let proof_req_json = format!("{{\
                                 \"nonce\":\"123432421212\",\
                                 \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":{},\"name\":\"name\"}},\
                                 \"attr2_uuid\":{{\"schema_seq_no\":{},\"name\":\"status\"}}}},\
                                 \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}},\
                                 \"predicate2_uuid\":{{\"attr_name\":\"period\",\"p_type\":\"GE\",\"value\":5}}}}\
                                 }}", gvt_schema_seq_no, xyz_schema_seq_no);
    
    let res = AnoncredsUtils::prover_get_claims_for_proof_req(prover_wallet_handle, &proof_req_json);
    assert!(res.is_ok());
    let claims_json = res.unwrap();
    
    let claims: ProofClaimsJson = serde_json::from_str(&claims_json).unwrap();
    
    let claims_for_attr_1 = claims.attrs.get("attr1_uuid").unwrap();
    let claims_for_attr_2 = claims.attrs.get("attr2_uuid").unwrap();
    assert_eq!(1, claims_for_attr_1.len());
    assert_eq!(1, claims_for_attr_2.len());
    
    let claim_for_attr_1 = claims_for_attr_1[0].clone();
    let claim_for_attr_2 = claims_for_attr_2[0].clone();
    
    let claims_for_predicate_1 = claims.predicates.get("predicate1_uuid").unwrap();
    let claims_for_predicate_2 = claims.predicates.get("predicate2_uuid").unwrap();
    assert_eq!(1, claims_for_predicate_1.len());
    assert_eq!(1, claims_for_predicate_2.len());
    
    let claim_for_predicate_1 = claims_for_predicate_1[0].clone();
    let claim_for_predicate_2 = claims_for_predicate_2[0].clone();
    
    
    // 18. Prover create Proof
    let requested_claims_json = format!("{{\
                                        \"self_attested_attributes\":{{}},\
                                        \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true],\
                                        \"attr2_uuid\":[\"{}\",true]}},\
                                        \"requested_predicates\":{{\"predicate1_uuid\":\"{}\", \
                                        \"predicate2_uuid\":\"{}\"}}\
                                        }}",
                                        claim_for_attr_1.claim_uuid, claim_for_attr_2.claim_uuid,
                                        claim_for_predicate_1.claim_uuid, claim_for_predicate_2.claim_uuid);
    
    let unique_claims = AnoncredsUtils::get_unique_claims(&claims);
    
    let schemas_json = format!("{{\"{}\":{}, \"{}\":{}}}",
                               unique_claims[0].claim_uuid,
                               schemas.get(&unique_claims[0].schema_seq_no).unwrap(),
                               unique_claims[1].claim_uuid,
                               schemas.get(&unique_claims[1].schema_seq_no).unwrap());
    
    
    let claim_defs_json = format!("{{\"{}\":{}, \"{}\":{}}}",
                                  unique_claims[0].claim_uuid,
                                  claim_defs.get(&unique_claims[0].claim_def_seq_no).unwrap(),
                                  unique_claims[1].claim_uuid,
                                  claim_defs.get(&unique_claims[1].claim_def_seq_no).unwrap());
    let revoc_regs_jsons = "{}";
    
    
    let res = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                  &proof_req_json,
                                                  &requested_claims_json,
                                                  &schemas_json,
                                                  &master_secret_name_1,
                                                  &claim_defs_json,
                                                  &revoc_regs_jsons);
    assert!(res.is_ok());
    let proof_json = res.unwrap();
    
    // 19. Verifier verify proof
    let res = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                    &proof_json,
                                                    &schemas_json,
                                                    &claim_defs_json,
                                                    &revoc_regs_jsons);
    assert!(res.is_ok());
    assert!(res.unwrap());
#endif
    
    [TestUtils cleanupStorage];
}

- (void)testAnoncreds
{
//    [self anoncredsWorksForSingleIssuerSingleProverTest];
    [self anoncredsWorksForMultiplyIssuerSingleProver];
}

@end
