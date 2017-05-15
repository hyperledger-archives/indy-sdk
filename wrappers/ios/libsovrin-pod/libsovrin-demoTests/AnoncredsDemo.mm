//
//  libsovrin_demoTests.m
//  libsovrin-demoTests
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"

static NSTimeInterval defaultTimeout = 1000;

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

extern "C" {
    extern int32_t fake(int32_t p);
}

- (void)testAnoncredsDemo
{
    [TestUtils cleanupStorage];
    NSString *poolName = @"pool1";
    NSString *walletName = @"issuer_wallet";
    NSString *xType = @"default";
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    // 1. Create wallet
    
    NSError *ret = [[SovrinWallet sharedInstance] createWallet:  poolName
                                                          name:  walletName
                                                         xType:  xType
                                                        config:  nil
                                                   credentials:  nil
                                                    completion: ^(NSError* error)
    {
        XCTAssertEqual(error.code, Success, "createWallet got error in completion");
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"createWallet() failed!");

    // 2. Open Issuer Wallet. Gets Issuer wallet handle
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    __block SovrinHandle walletHandle = 0;

    ret = [[SovrinWallet sharedInstance] openWallet:  walletName
                                      runtimeConfig:  nil
                                        credentials:  nil
                                         completion: ^(NSError* error, SovrinHandle handle)
    {
        XCTAssertEqual(error.code, Success, "openWallet got error in completion");
        walletHandle = handle;
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"openWallet() failed!");
    
    // 3. Issuer create Claim Definition for Schema
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSUInteger seqNo = 1;
    NSString *schema = [ NSString stringWithFormat:@"{\
                        \"name\":\"gvt\",\
                        \"version\":\"1.0\",\
                        \"attribute_names\":[\"age\",\"sex\",\"height\",\"name\"],\
                        \"seq_no\":%d\
                        }", seqNo ];
    
    __block NSString *claimJSON = nil;
    __block NSString *claimDefUUID = nil;
    ret = [[SovrinAnoncreds sharedInstance] issuerCreateAndStoreClaimDef:  walletHandle
                                                              schemaJSON:  schema
                                                           signatureType:  nil
                                                          createNonRevoc:  false
                                                              completion: ^(NSError *error, NSString *claimDefJSON, NSString *claimDefUUID1)
    {
        XCTAssertEqual(error.code, Success, "issuerCreateAndStoreClaimDef got error in completion");
        claimJSON = [ NSString stringWithString: claimDefJSON];
        claimDefUUID = [ NSString stringWithFormat: claimDefUUID1];
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"issuerCreateAndStoreClaimDef() failed!");

    // 4. Create relationship between claim_def_seq_no and claim_def_uuid in wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] walletSetSeqNo:  [NSNumber numberWithInteger: seqNo]
                                              forHandle:  walletHandle
                                                 andKey:  claimDefUUID
                                             completion: ^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "walletSetSeqNo got error in completion");
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"walletSetSeqNo() failed!");
    
    // 5. Prover create Master Secret
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString *masterSecretName = @"master_secret";
    
    ret = [[SovrinAnoncreds sharedInstance] proverCreateMasterSecret:  walletHandle
                                                    masterSecretName:  masterSecretName
                                                          completion: ^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "proverCreateMasterSecret got error in completion");
        [completionExpectation fulfill];

    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"proverCreateMasterSecret() failed!");
    
    // 6. Prover create Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString *proverDiD = @"some_prover_did";
    
    NSString *claimOfferJSON =  [NSString stringWithFormat: @"{\"issuer_did\":\"some_issuer_did\",\"claim_def_seq_no\":%d}", seqNo];
    __block NSString *claimReqJSON = nil;
    
    ret = [[ SovrinAnoncreds sharedInstance] proverCreateAndStoreClaimReq: walletHandle
                                                                proverDid: proverDiD
                                                           claimOfferJSON: claimOfferJSON
                                                         masterSecretName: masterSecretName
                                                             claimDefJSON: claimJSON
                                                               completion:^ (NSError *error, NSString *claimReqJSON1)
    {
        XCTAssertEqual(error.code, Success, "proverCreateAndStoreClaimReq got error in completion");
        claimReqJSON = [ NSString stringWithString: claimReqJSON1 ];
        [completionExpectation fulfill];
        
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"proverCreateAndStoreClaimReq() failed!");

    // 7. Issuer create Claim for Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString*  testClaimJson = @"{\
                                  \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
                                  \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
                                  \"height\":[\"175\",\"175\"],\
                                  \"age\":[\"28\",\"28\"]\
                                 }";
    __block NSString *xClaimJSON = nil;
    
    ret = [[SovrinAnoncreds sharedInstance] issuerCreateClaim:  walletHandle
                                                 claimReqJSON:  claimReqJSON
                                                    claimJSON:  testClaimJson
                                                revocRegSeqNo:  nil
                                               userRevocIndex:  nil
                                                   completion: ^ (NSError* error, NSString* revocRegUpdateJSON, NSString* claimJSON1)
    {
        XCTAssertEqual(error.code, Success, "issuerCreateClaim() got error in completion");
        xClaimJSON = [ NSString stringWithString: claimJSON1];
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"issuerCreateClaim() failed!");

    // 8. Prover process and store Claim
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [[SovrinAnoncreds sharedInstance] proverStoreClaim:  walletHandle
                                                  claimsJSON:  xClaimJSON
                                                  completion: ^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "proverStoreClaim() got error in completion");
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"proverStoreClaim() failed!");

    // 9. Prover gets Claims for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString* proofReqJSON = [ NSString stringWithFormat: @"{\
                                 \"nonce\":\"123432421212\",\
                                 \"requested_attrs\":{{\"attr1_uuid\":{{\"schema_seq_no\":%d,\"name\":\"name\"}}}},\
                                 \"requested_predicates\":{{\"predicate1_uuid\":{{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}}}\
                                 }", seqNo];
    
    ret = [[ SovrinAnoncreds sharedInstance] proverGetClaimsForProofReq:  walletHandle
                                                           proofReqJSON:  proofReqJSON
                                                             completion: ^(NSError* error, NSString* claimsJSON)
    {
        XCTAssertEqual(error.code, Success, "proverGetClaimsForProofReq() got error in completion");
        [completionExpectation fulfill];
        
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"proverGetClaimsForProofReq() failed!");

    // 10. Prover create Proof for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString* requestedClaimsJSON = [ NSString stringWithFormat: @"{\
                                                                       \"self_attested_attributes\":{{}},\
                                                                       \"requested_attrs\":{{\"attr1_uuid\":[\"%d\",true]}},\
                                                                       \"requested_predicates\":{{\"predicate1_uuid\":\"%d\"}}\
                                                                    }", seqNo, seqNo ];
    
    NSString *schemas = [NSString stringWithFormat: @"{\"%d\":%@}", seqNo, schema];
    
    NSString *claimDefsJSON = [NSString stringWithFormat: @"{\"%d\":%@}", seqNo, claimJSON];
    
    NSString *revocRegsJsons = @"{}";
    
    __block NSString *proofJSON = nil;
    
    ret =  [[ SovrinAnoncreds sharedInstance] proverCreateProof:  walletHandle
                                                   proofReqJSON:  proofReqJSON
                                            requestedClaimsJSON:  requestedClaimsJSON
                                                    schemasJSON:  schemas
                                               masterSecretName:  masterSecretName
                                                  claimDefsJSON:  claimDefsJSON
                                                  revocRegsJSON:  revocRegsJsons
                                                     completion: ^(NSError* error, NSString* proofJSON1)
    {
        XCTAssertEqual(error.code, Success, "proverCreateProof() got error in completion");
        proofJSON = [ NSString stringWithString: proofJSON1];
        [completionExpectation fulfill];
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"proverCreateProof() failed!");

    // 11. Verifier verify proof
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [[SovrinAnoncreds sharedInstance] verifierVerifyProof:  walletHandle
                                                   proofReqJSON:  proofReqJSON
                                                      proofJSON:  proofJSON
                                                    schemasJSON:  schemas
                                                  claimDefsJSON:  claimDefsJSON
                                                  revocRegsJSON:  revocRegsJsons
                                                     completion: ^(NSError *error, BOOL valid)
    {
        XCTAssertEqual(error.code, Success, "verifierVerifyProof() got error in completion");
        XCTAssertEqual(valid, true, "verifierVerifyProof() got error in completion");
        [completionExpectation fulfill];
        
    }];

    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"verifierVerifyProof() failed!");
    
    // 12. close wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [[SovrinWallet sharedInstance] closeWallet: walletHandle
                                          completion: ^ (NSError *error)
    {
        XCTAssertEqual(error.code, Success, "closeWallet got error in completion");
        [completionExpectation fulfill];
    }];
    
    [self waitForExpectations: @[completionExpectation] timeout:defaultTimeout];
    NSAssert( ret.code == Success, @"closeWallet() failed!");
    [TestUtils cleanupStorage];
}


@end
