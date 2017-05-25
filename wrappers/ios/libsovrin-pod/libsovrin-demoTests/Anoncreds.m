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
    NSAssert(res.code == Success, @"WalletUtils::createWallet() failed");

    //2. Create Prover wallet, get wallet handle
    res = [[WalletUtils sharedInstance] createWallet:poolName walletName:proverWalletName xtype:xtype handle:&proverWalletHandle];
    NSAssert(res.code == Success, @"WalletUtils::createWallet() failed");
    
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
    
    NSAssert(res.code == Success, @"AnoncredsUtils::createClaimDefinitionAndSetLink() failed");

    //4. Prover create Master Secret
    
    NSString *masterSecretName = @"prover_master_secret";
    
    res = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:proverWalletHandle
                                                   masterSecretName:masterSecretName];
    
    NSAssert(res.code == Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");
    
    //5. Prover store Claim Offer received from Issuer
    
    NSString *claimOfferJson = [[ AnoncredsUtils sharedInstance] getClaimOfferJson: issuerDid seqNo: claimDefSeqNo ];
    
    res = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer: proverWalletHandle
                                                  claimOfferJson: claimOfferJson ];
    
    NSAssert(res.code == Success, @"AnoncredsUtils::proverStoreClaimOffer() failed");

    //6. Prover get Claim Offers
    
    NSString *filterJson = [NSString stringWithFormat: @"{ \"issuer_did\":\"%@\"}", issuerDid];
    NSString *claimOffersJson = nil;
    
    res = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:  proverWalletHandle
                                                     filterJson:  filterJson
                                             outClaimOffersJSON: &claimOffersJson];
    
    NSAssert(res.code == Success, @"AnoncredsUtils::proverGetClaimOffers() failed");

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

    NSAssert(res.code == Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq() failed");

    //8. Issuer create Claim
    NSString *revocRegUpdateJson = nil;
    NSString *xclaimJson = nil;
    
    NSString *claimJson = [[ AnoncredsUtils sharedInstance] getGvtClaimJson];

    res = [[AnoncredsUtils sharedInstance] issuerCreateClaim: issuerWalletHandle
                                                claimReqJson: claimReq
                                                   claimJson: claimJson
                                       outRevocRegUpdateJSON:&revocRegUpdateJson
                                                outClaimJson:&xclaimJson ];

    NSAssert(res.code == Success, @"AnoncredsUtils::issuerCreateClaim() failed");
    
    // 9. Prover store received Claim
    
    res = [[AnoncredsUtils sharedInstance] proverStoreClaim: issuerWalletHandle
                                                 claimsJson: claimJson];

    NSAssert(res.code == Success, @"AnoncredsUtils::proverStoreClaim() failed");
    
    
    // 10. Prover gets Claims for Proof Request
    NSString *proofReqJson =[ NSString stringWithFormat:@"{"\
                                                         " \"nonce\":\"123432421212\","\
                                                         " \"requested_attrs\":"\
                                                         "             {\"attr1_uuid\":"\
                                                         "                        {"\
                                                         "                          \"schema_seq_no\":%d,\"name\":\"name\""\
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
                                                        outClaimsJson:&claimJson];

    NSAssert(res.code == Success, @"AnoncredsUtils::proverGetClaimsForProofReq() failed");

    NSDictionary *claims = [NSJSONSerialization JSONObjectWithData:[NSData dataWithBytes:[claimsJson UTF8String]
                                                                                  length:[claimsJson length]]
                                                           options:kNilOptions
                                                             error: &res];
    NSAssert( claims, @"serialization failed");
    
    NSDictionary *claims_for_attr_1 = [claims objectForKey:@"attr1_uuid"];

    NSAssert( claims_for_attr_1, @"no object for key \"attr1_uuid\"");
    
    //TODO: add assert here
    
    [TestUtils cleanupStorage];
}

#[test]
fn anoncreds_works_for_single_issuer_single_prover() {


    
    // 11. Prover create Proof
    let requested_claims_json = format!("{{\
                                        \"self_attested_attributes\":{{}},\
                                        \"requested_attrs\":{{\"attr1_uuid\":[\"{}\",true]}},\
                                        \"requested_predicates\":{{\"predicate1_uuid\":\"{}\"}}\
                                        }}", claim.claim_uuid, claim.claim_uuid);
    
    let schemas_json = format!("{{\"{}\":{}}}", claim.claim_uuid, schema);
    let claim_defs_json = format!("{{\"{}\":{}}}", claim.claim_uuid, claim_def_json);
    let revoc_regs_jsons = "{}";
    
    let res = AnoncredsUtils::prover_create_proof(prover_wallet_handle,
                                                  &proof_req_json,
                                                  &requested_claims_json,
                                                  &schemas_json,
                                                  &master_secret_name,
                                                  &claim_defs_json,
                                                  &revoc_regs_jsons);
    assert!(res.is_ok());
    let proof_json = res.unwrap();
    
    // 12. Verifier verify proof
    let res = AnoncredsUtils::verifier_verify_proof(&proof_req_json,
                                                    &proof_json,
                                                    &schemas_json,
                                                    &claim_defs_json,
                                                    &revoc_regs_jsons);
    assert!(res.is_ok());
    assert!(res.unwrap());
    
    TestUtils::cleanup_storage();
}

- (void)testAnoncreds
{

}

@end
