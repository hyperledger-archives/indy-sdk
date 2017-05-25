//
//  libsovrin_demoTests.m
//  libsovrin-demoTests
//

#import <Foundation/Foundation.h>
#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "WalletUtils.h"

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
    NSString *poolName = @"pool1";
    NSString *walletName = @"issuer_wallet";
    NSString *xType = @"default";
    XCTestExpectation* completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    SovrinHandle walletHandle = 0;
    
    // 1. Create and open the wallet
    
    NSError *ret = [[WalletUtils sharedInstance] createWallet:  poolName
                                                   walletName:  walletName
                                                        xtype:  xType
                                                       handle: &walletHandle];

    NSAssert( ret.code == Success, @"WalletUtils::createWallet() failed!");

    // 2. Issuer create Claim Definition for Schema
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSUInteger seqNo = 1;
    NSString *schema = [ NSString stringWithFormat:@"{\
                        \"name\":\"gvt\",\
                        \"version\":\"1.0\",\
                        \"attribute_names\":[\"age\",\"sex\",\"height\",\"name\"],\
                        \"seq_no\":%lu\
                        }", (unsigned long)seqNo ];
    
    __block NSString *claimJSON = nil;
    __block NSString *claimDefUUID = nil;
    
    ret = [SovrinAnoncreds issuerCreateAndStoreClaimDef:  walletHandle
                                             schemaJSON:  schema
                                          signatureType:  nil
                                         createNonRevoc:  false
                                             completion: ^(NSError *error, NSString *claimDefJSON, NSString *claimDefUUID1)
    {
        XCTAssertEqual(error.code, Success, "issuerCreateAndStoreClaimDef got error in completion");
        claimJSON = [ NSString stringWithString: claimDefJSON];
        claimDefUUID = [ NSString stringWithString: claimDefUUID1];
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"issuerCreateAndStoreClaimDef() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    // 3. Create relationship between claim_def_seq_no and claim_def_uuid in wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];
    
    ret = [[SovrinWallet sharedInstance] walletSetSeqNo:  [NSNumber numberWithInteger: seqNo]
                                              forHandle:  walletHandle
                                                 andKey:  claimDefUUID
                                             completion: ^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "walletSetSeqNo got error in completion");
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"walletSetSeqNo() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 4. Prover create Master Secret
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString *masterSecretName = @"master_secret";
    
    ret = [SovrinAnoncreds proverCreateMasterSecret:  walletHandle
                                   masterSecretName:  masterSecretName
                                         completion: ^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "proverCreateMasterSecret got error in completion");
        [completionExpectation fulfill];

    }];

    NSAssert( ret.code == Success, @"proverCreateMasterSecret() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 5. Prover create Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString *proverDiD = @"some_prover_did";
    
    NSString *claimOfferJSON =  [NSString stringWithFormat: @"{\"issuer_did\":\"some_issuer_did\",\"claim_def_seq_no\":%lu}", (unsigned long)seqNo];
    __block NSString *claimReqJSON = nil;
    
    ret = [SovrinAnoncreds proverCreateAndStoreClaimReq: walletHandle
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
    
    NSAssert( ret.code == Success, @"proverCreateAndStoreClaimReq() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    // 6. Issuer create Claim for Claim Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString*  testClaimJson = @"{\
                                  \"sex\":[\"male\",\"5944657099558967239210949258394887428692050081607692519917050011144233115103\"],\
                                  \"name\":[\"Alex\",\"1139481716457488690172217916278103335\"],\
                                  \"height\":[\"175\",\"175\"],\
                                  \"age\":[\"28\",\"28\"]\
                                 }";
    __block NSString *xClaimJSON = nil;
    
    ret = [SovrinAnoncreds issuerCreateClaim:  walletHandle
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

    NSAssert( ret.code == Success, @"issuerCreateClaim() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    // 7. Prover process and store Claim
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [SovrinAnoncreds proverStoreClaim:  walletHandle
                                 claimsJSON:  xClaimJSON
                                 completion: ^(NSError *error)
    {
        XCTAssertEqual(error.code, Success, "proverStoreClaim() got error in completion");
        [completionExpectation fulfill];
    }];

    NSAssert( ret.code == Success, @"proverStoreClaim() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    // 8. Prover gets Claims for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString* proofReqJSON = [NSString stringWithFormat: @"\
                               {\
                               \"nonce\":\"123432421212\",\
                               \"requested_attrs\":{\
                               \"attr1_uuid\":{\
                              \"schema_seq_no\":%lu,\
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
                              }", (unsigned long)seqNo ];
    
    ret = [SovrinAnoncreds proverGetClaimsForProofReq:  walletHandle
                                         proofReqJSON:  proofReqJSON
                                           completion: ^(NSError* error, NSString* claimsJSON)
    {
        XCTAssertEqual(error.code, Success, "proverGetClaimsForProofReq() got error in completion");
        [completionExpectation fulfill];
        
    }];

    NSAssert( ret.code == Success, @"proverGetClaimsForProofReq() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    // 9. Prover create Proof for Proof Request
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    NSString* requestedClaimsJSON = [ NSString stringWithFormat: @"{\
                                                                       \"self_attested_attributes\":{},\
                                                                       \"requested_attrs\":{\"attr1_uuid\":[\"%lu\",true]},\
                                                                       \"requested_predicates\":{\"predicate1_uuid\":\"%lu\"}\
                                                                   }", (unsigned long)seqNo, (unsigned long)seqNo ];
    
    NSString *schemas = [NSString stringWithFormat: @"{\"%lu\":%@}", (unsigned long)seqNo, schema];
    
    NSString *claimDefsJSON = [NSString stringWithFormat: @"{\"%lu\":%@}", (unsigned long)seqNo, claimJSON];
    
    NSString *revocRegsJsons = @"{}";
    
    __block NSString *proofJSON = nil;
    
    ret =  [SovrinAnoncreds proverCreateProof:  walletHandle
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

    NSAssert( ret.code == Success, @"proverCreateProof() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];

    // 10. Verifier verify proof
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [SovrinAnoncreds verifierVerifyProof:  walletHandle
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

    NSAssert( ret.code == Success, @"verifierVerifyProof() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    
    // 11. close wallet
    completionExpectation = [[ XCTestExpectation alloc] initWithDescription: @"completion finished"];

    ret = [[SovrinWallet sharedInstance] closeWallet: walletHandle
                                          completion: ^ (NSError *error)
    {
        XCTAssertEqual(error.code, Success, "closeWallet got error in completion");
        [completionExpectation fulfill];
    }];
    
    NSAssert( ret.code == Success, @"closeWallet() failed!");
    [self waitForExpectations: @[completionExpectation] timeout:[TestUtils defaultTimeout]];
    [TestUtils cleanupStorage];
}


@end
