//
//  AnoncredsMediumCases.m
//  Indy-demo
//
//  Created by Anastasia Tarasova on 16.06.17.
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

@interface AnoncredsMediumCases : XCTestCase

@end

@implementation AnoncredsMediumCases

- (void)setUp {
    [super setUp];
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

// MARK: - Issuer create and store claim def

- (void)testIssuerCreateAndStoreClaimDefWorksForInvalidSchema {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *schema = @"{"
            "\"seqNo\":1,"
            "\"name\":\"name\","
            "\"version\":\"1.0\","
            "\"attr_names\":[\"name\"]}";

    // 2. create claim definition
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle returned wrong code");
}

- (void)testIssuerCreateAndStoreClaimDefWorksForEmptySchemaKeys {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *schema = @"{"\
                        "\"seqNo\":1,"\
                        "\"identifier\":\"didissuer\","\
                        "\"data\":{"\
                            "\"name\":\"name\","\
                            "\"version\":\"1.0\","\
                            "\"attr_names\":[]"\
                        "}}";

    // 2. create claim definition
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle returned wrong code");
}

- (void)testIssuerCreateAndStoreClaimDefWorksForInvalidSignatureType {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *schema = [[AnoncredsUtils sharedInstance] getSchemaJson:@"ClaimDefWorksForInvalidSignatureType"];

    // 2. create claim definition
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:@"some_type"
                                                                        createNonRevoc:false
                                                                          claimDefJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle returned wrong code");
}


- (void)testIssuerCreateAndStoreClaimDefWorksForDuplicate {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");


    NSString *schema = [[AnoncredsUtils sharedInstance] getSchemaJson:@"ClaimDefWorksForDuplicate"];

    // 2. create claim definition
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle returned wrong code");

    // 3. create duplicate claim definition
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                             issuerDid:[TestUtils issuerDid]
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:false
                                                                          claimDefJson:nil];
    XCTAssertEqual(ret.code, AnoncredsClaimDefAlreadyExistsError, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle returned wrong code");
}

// MARK: - Prover store claim offer

- (void)testProverStoreClaimOfferWorksForInvalidIssuerDid {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *claimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:[TestUtils invalidBase58Verkey]
                                                                        schemaKey:[[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    // 2. store claim offer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:walletHandle
                                                  claimOfferJson:claimOfferJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverStoreClaimOffer returned wrong code");
}

// MARK: - Prover get claim offers

- (void)testProverGetClaimOffersWorksForInvalidFilterJson {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. prover get claim offers
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle
                                                     filterJson:@"{\"schema_key\":\"gvt\"}"
                                             outClaimOffersJSON:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle returned wrong code");
}

- (void)testProverGetClaimOffersWorksForDifferentWallets {
    NSError *ret;

    // 1. init commmon wallet
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. obtain wallet handles
    IndyHandle walletHandle1 = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle1];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed");

    IndyHandle walletHandle2 = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithPoolName:[TestUtils pool]
                                                                  xtype:nil
                                                                 handle:&walletHandle2];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWalletWithPoolName failed");

    // 3. create claim offers
    NSString *claimOfferJson1 = [[AnoncredsUtils sharedInstance] getClaimOfferJson:[TestUtils issuerDid]
                                                                         schemaKey:[[AnoncredsUtils sharedInstance] getGvtSchemaKey]];
    NSString *claimOfferJson2 = [[AnoncredsUtils sharedInstance] getClaimOfferJson:[TestUtils issuerDid]
                                                                         schemaKey:[[AnoncredsUtils sharedInstance] getXyzSchemaKey]];
    NSString *claimOfferJson3 = [[AnoncredsUtils sharedInstance] getClaimOfferJson:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
                                                                         schemaKey:[[AnoncredsUtils sharedInstance] getXyzSchemaKey]];
    // 4. store claim offers
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:walletHandle1
                                                  claimOfferJson:claimOfferJson1];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer failed for claimOfferJson1");

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:walletHandle2
                                                  claimOfferJson:claimOfferJson2];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer failed for claimOfferJson1");

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:walletHandle2
                                                  claimOfferJson:claimOfferJson3];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer failed for claimOfferJson1");

    // 5. get claim offers
    NSString *claimOffersJson;
    NSString *filter = [NSString stringWithFormat:@"{\"issuer_did\":\"%@\"}", [TestUtils issuerDid]];
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle2
                                                     filterJson:filter
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson");

    NSDictionary *claimOffers = [NSDictionary fromString:claimOffersJson];
    NSArray *claimsArr = (NSArray *) claimOffers;
    XCTAssertEqual([claimsArr count], 1, @"claimsArr contains not 1 element");

    XCTAssertTrue([[NSString stringWithString:claimsArr[0][@"issuer_did"]] isEqualToString:[TestUtils issuerDid]]);

    NSDictionary *xyzSchemaKeyDict = [NSDictionary fromString:[[AnoncredsUtils sharedInstance] getXyzSchemaKey]];
    XCTAssertTrue([[NSDictionary dictionaryWithDictionary:claimsArr[0][@"schema_key"]] isEqualToDictionary:xyzSchemaKeyDict]);

}

// MARK: - Prover create master secter

- (void)testProverCreateMasterSecretWorksForDublicateName {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. create master secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:@"master_secret_name_duplicate"
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret first failed");

    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecretNamed:@"master_secret_name_duplicate"
                                                            walletHandle:walletHandle];
    XCTAssertEqual(ret.code, AnoncredsMasterSecretDuplicateNameError, @"AnoncredsUtils::proverCreateMasterSecret second returned wrong code");
}

// MARK: - Prover create and store claim request

- (void)testProverCreateAndStoreClaimReqWorksForInvalidClaimOffer {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimDefJson;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDefJson
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *claimOfferJson = [NSString stringWithFormat:@"{"\
            "\"issuer_did\":\"%@\"," \
            "\"schema_key\":%@" \
            "}", [TestUtils issuerDid], [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    // 2. create and store claim request
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJson
                                                              masterSecretName:[TestUtils commonMasterSecretName]
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:nil];

    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed with wrong code");
}

- (void)testProverCreateAndStoreClaimReqWorksForInvalidClaimDef {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimOfferJson;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:&claimOfferJson
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. create and store claim request
    NSString *claimDefJson = @"{"\
            "\"schema_eq_no\":1,"\
            "\"signature_type\":\"CL\","\
            "\"primary\":{"\
                "\"n\":\"121212\","\
                "\"s\":\"432192\","\
            "}}";

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJson
                                                                     proverDid:[TestUtils proverDid]
                                                                claimOfferJson:claimOfferJson
                                                              masterSecretName:[TestUtils commonMasterSecretName]
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateAndStoreClaimReq returned wrong code");
}

- (void)testProverCreateAndStoreClaimReqWorksForInvalidMasterSecret {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimDefJson;
    NSString *claimOfferJson;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDefJson
                                                                  claimOfferJson:&claimOfferJson
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. create and store claim request
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJson
                                                                     proverDid:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
                                                                claimOfferJson:claimOfferJson
                                                              masterSecretName:@"invalid_master_secret_name"
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:nil];

    XCTAssertEqual(ret.code, WalletNotFoundError, @"AnoncredsUtils::proverCreateAndStoreClaimReq returned wrong error");
}

// MARK: - Issuer create claim

- (void)testIssuerCreateClaimWorksForInvalidClaimRequestJson {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. issuer create claim
    NSString *claimRequest = [NSString stringWithFormat:@"{"
                                                                "\"blinded_ms\":{"
                                                                "\"ur\":null},"
                                                                "\"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\","
                                                                "\"issuer_did\":\"%@\","
                                                                "\"schema_key\":\"gvt\"}", [TestUtils issuerDid]];

    NSString *claimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:nil
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle returned wrong code");
}

- (void)testIssuerCreateClaimWorksForInvalidClaimJson {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimRequest;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:&claimRequest
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. issuer create claim
    NSString *claimJson = @"{"
            "\"sex\":\"male\","
            "\"name\":\"Alex\","
            "\"height\":\"175\","
            "\"age\":\"28\""
            "}";

    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                              userRevocIndex:nil
                                                                outClaimJson:nil
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle returned wrong code");
}

// MARK: - Prover store claim

- (void)testProverStoreClaimWorksWithoutClaimRequest {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. store claim
    NSString *claimJson = [NSString stringWithFormat:@"{"
                                                             "\"values\":{"
                                                             "\"sex\":[\"male\",\"1\"],"
                                                             "\"age\":[\"28\",\"28\"],"
                                                             "\"name\":[\"Alex\",\"1\"],"
                                                             "\"height\":[\"175\",\"175\"]},"
                                                             "\"issuer_did\":\"%@\","
                                                             "\"revoc_reg_seq_no\":null,"
                                                             "\"schema_key\":%@,"
                                                             "\"signature_correctness_proof\":{"
                                                             "\"se\":\"123456789\","
                                                             "\"c\":\"1\""
                                                             "},"
                                                             "\"signature\":{"
                                                             "\"p_claim\":{"
                                                             "\"m_2\":\"1\","
                                                             "\"a\":\"1\","
                                                             "\"e\":\"2\","
                                                             "\"v\":\"3\"},"
                                                             "\"r_claim\":null}}", @"issueddid", [[AnoncredsUtils sharedInstance] getXyzSchemaKey]];
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                 claimsJson:claimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"AnoncredsUtils::proverStoreClaimWithWalletHandle returned wrong code");
}

- (void)testProverStoreClaimWorksForInvalidClaimJson {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimDefJson;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDefJson
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. create and store claim request
    NSString *claimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:[TestUtils issuerDid]
                                                                        schemaKey:[[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReqWithDef:claimDefJson
                                                                     proverDid:@"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW"
                                                                claimOfferJson:claimOfferJson
                                                              masterSecretName:@"common_master_secter_name"
                                                                  walletHandle:walletHandle
                                                               outClaimReqJson:nil];
    //TODO: - Returns 204 error. Is it right? no check in rust
    //XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed");

    NSString *claimJson = [NSString stringWithFormat:@"{"\
            "\"claim\":{"\
                "\"sex\":[\"male\",\"1\"],"\
                "\"age\":[\"28\",\"28\"],"\
                "\"name\":[\"Alex\",\"1\"],"\
                "\"height\":[\"175\",\"175\"]},"\
            "\"issuer_did\":1,"\
            "\"revoc_reg_seq_no\":null,"\
            "\"schema_key\":%@}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                 claimsJson:claimJson
                                                                 revRegJSON:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverStoreClaimWithWalletHandle returned wrong code");
}

// MARK: - Prover get claims

- (void)testProverGetClaimsWorksForInvalidJson {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get claims
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForWalletHandle:walletHandle
                                                               filterJson:@"{\"schema_key\":\"gvt\"}"
                                                            outClaimsJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverGetClaimsForWalletHandle returned wrong code");
}

// MARK: - Prover get claims for proof request

- (void)testProverGetClaimsForProofReqWorksForEmptyRequest {
    NSError *ret;

    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil
                                                                  claimOfferJson:nil
                                                                    claimReqJson:nil
                                                                       claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get claims for proof request
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{},"
            "\"requested_predicates\":{}}";

    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"claimsJson is invalid");

    // 3. check claims
    NSDictionary *claims = [NSDictionary fromString:claimsJson];

    XCTAssertEqual([[claims[@"attrs"] allValues] count], 0, @"attrs is not empty!");
    XCTAssertEqual([[claims[@"predicates"] allValues] count], 0, @"predicates is not empty!");
}

- (void)testProverGetClaimsForProofreqWorksForInvalidProofReq {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:nil
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get claims for proof request
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_predicates\":{}}";

    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong code");
}

- (void)testProverGetClaimsForProofReqWorksForRevealedAttrWithOtherSchemaSeqNo {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:nil
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get claims for proof request
    NSString *proofRequest = [NSString stringWithFormat:@"{"
                                                                "\"nonce\":\"123432421212\","
                                                                "\"name\":\"proof_req_1\","
                                                                "\"version\":\"0.1\","
                                                                "\"requested_attrs\":{\"attr1_referent\":{\"name\":\"name\",\"restrictions\":[{\"schema_key\":%@}]\
}},"
                                                                "\"requested_predicates\":{}}", [[AnoncredsUtils sharedInstance] getXyzSchemaKey]];

    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");

    // 3. check claims
    NSDictionary *claims = [NSDictionary fromString:claimsJson];

    XCTAssertEqual([[claims[@"attrs"] allValues] count], 1, @"attrs contains not 1 element");
    XCTAssertEqual([[claims[@"predicates"] allValues] count], 0, @"predicates should contain 0 elements");

    XCTAssertEqual([claims[@"attrs"][@"attr1_referent"] count], 0, @"attr1_referent should be empty");
}

- (void)testGetClaimsForProofReqWorksForInvalidPredicate {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:nil
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get claims for proof request
    NSString *proofRequest = @"{"\
            "\"nonce\":\"123432421212\","\
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{},"\
            "\"requested_predicates\":{"\
                "\"predicate1_referent\":{"\
                    "\"attr_name\":\"age\"}"\
            "}";

    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong code");
}

- (void)testProverGetClaimsForProofReqWorksForInvaliPredicateType {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:nil
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *proofRequest = @"{"\
            "\"nonce\":\"123432421212\","\
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{},"\
            "\"requested_predicates\":{"\
                "\"predicate1_referent\":{"\
                    "\"attr_name\":\"age\","\
                    "\"p_type\":\"LE\","\
                    "\"value\":58}}"\
            "}";

    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong code");
}

// MARK: - Prover create proof works

- (void)testProverCreateProofWorksForInvalidMasterSecret {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimDefJson;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:&claimDefJson
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. prover get claims for proof request
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{"
            "\"attr1_referent\":{\"name\":\"name\"\
}},"
            "\"requested_predicates\":{}"
            "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");

    // 3. create proof

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSString *attrUUID = claims[@"attrs"][@"attr1_referent"][0][@"referent"];
    XCTAssertTrue([attrUUID isValid], @"innvalid attrUUID:%@", attrUUID);

    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"
                                                                       "\"self_attested_attributes\":{},"
                                                                       "\"requested_attrs\":{"
                                                                       "\"attr1_referent\":[\"%@\",true]},"
                                                                       "\"requested_predicates\":{}"
                                                                       "}", attrUUID];
    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}",
                                                       attrUUID,
                                                       [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)]];
    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}",
                                                         attrUUID, claimDefJson];
    NSString *revocRegsJsons = @"{}";

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofRequest
                                                         requestedClaimsJson:requestedClaimsJson
                                                                 schemasJson:schemasJson
                                                            masterSecretName:@"invalid_master_secret_name" claimDefsJson:claimDefsJson revocRegsJson:revocRegsJsons
                                                                outProofJson:nil];
    XCTAssertEqual(ret.code, WalletNotFoundError, @"AnoncredsUtils::proverCreateProofWithWalletHandle returned wrong error");
}

- (void)testProverCreateProofWorksForInvalidSchemasJson {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimDefJson;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:&claimDefJson
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@", claimDefJson);

    // 2. prover get claims for proof request
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{"
            "\"attr1_referent\":{"
            "\"name\":\"name\"\}},"
            "\"requested_predicates\":{}"
            "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson");

    // 3. create proof

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSString *attrUUID = claims[@"attrs"][@"attr1_referent"][0][@"referent"];
    XCTAssertTrue([attrUUID isValid], @"innvalid attrUUID:%@", attrUUID);

    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"
                                                                       "\"self_attested_attributes\":{},"
                                                                       "\"requested_attrs\":{"
                                                                       "\"attr1_referent\":[\"%@\",true]},"
                                                                       "\"requested_predicates\":{}"
                                                                       "}", attrUUID];
    NSString *schemasJson = @"{}";
    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}",
                                                         attrUUID, claimDefJson];
    NSString *revocRegsJsons = @"{}";

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofRequest
                                                         requestedClaimsJson:requestedClaimsJson

                                                                 schemasJson:schemasJson
                                                            masterSecretName:[TestUtils commonMasterSecretName]
                                                               claimDefsJson:claimDefsJson
                                                               revocRegsJson:revocRegsJsons
                                                                outProofJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateProofWithWalletHandle returned wrong error");
}

- (void)testProverCreateProofWorksforInvalidClaimDefsJson {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimDefJson;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:&claimDefJson
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@", claimDefJson);

    // 2. prover get claims for proof request
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{"
            "\"attr1_referent\":{"
            "\"name\":\"name\"}},"
            "\"requested_predicates\":{}"
            "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson");

    // 3. create proof

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSString *attrUUID = claims[@"attrs"][@"attr1_referent"][0][@"referent"];
    XCTAssertTrue([attrUUID isValid], @"innvalid attrUUID:%@", attrUUID);

    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"
                                                                       "\"self_attested_attributes\":{},"
                                                                       "\"requested_attrs\":{"
                                                                       "\"attr1_referent\":[\"%@\",true]},"
                                                                       "\"requested_predicates\":{}"
                                                                       "}", attrUUID];
    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}",
                                                       attrUUID, [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)]];
    NSString *claimDefsJson = @"{}";
    NSString *revocRegsJsons = @"{}";

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofRequest
                                                         requestedClaimsJson:requestedClaimsJson

                                                                 schemasJson:schemasJson
                                                            masterSecretName:[TestUtils commonMasterSecretName]
                                                               claimDefsJson:claimDefsJson
                                                               revocRegsJson:revocRegsJsons
                                                                outProofJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateProofWithWalletHandle returned wrong error");
}

- (void)testProverCreateProofWorksForInvalidRequestedClaimsJson {
    // 1. init commmon wallet
    IndyHandle walletHandle = 0;
    NSString *claimDefJson;
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                             claimDefJson:&claimDefJson
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@", claimDefJson);

    // 2. prover get claims for proof request
    NSString *proofRequest = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{"
            "\"attr1_referent\":{"
            "\"name\":\"name\"}},"
            "\"requested_predicates\":{}"
            "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson");

    // 3. create proof

    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSString *attrUUID = claims[@"attrs"][@"attr1_referent"][0][@"referent"];
    XCTAssertTrue([attrUUID isValid], @"innvalid attrUUID:%@", attrUUID);

    NSString *requestedClaimsJson = @"{"
            "\"self_attested_attributes\":{},"
            "\"requested_predicates\":{}"
            "}";
    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}",
                                                       attrUUID, [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)]];
    NSString *claimDefsJson = @"{}";
    NSString *revocRegsJsons = @"{}";

    ret = [[AnoncredsUtils sharedInstance] proverCreateProofWithWalletHandle:walletHandle
                                                                proofReqJson:proofRequest
                                                         requestedClaimsJson:requestedClaimsJson

                                                                 schemasJson:schemasJson
                                                            masterSecretName:[TestUtils commonMasterSecretName]
                                                               claimDefsJson:claimDefsJson
                                                               revocRegsJson:revocRegsJsons
                                                                outProofJson:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateProofWithWalletHandle returned wrong error");
}

// MARK: - Verifier verify proof

- (void)testVerifierVerifyProofWorksForInvalidProofJson {
    // 1. init commmon wallet
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                                             claimDefJson:nil
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *proofReqJson = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{"
            "\"attr1_referent\":{"
            "\"name\":\"name\"}},"
            "\"requested_predicates\":{"
            "\"predicate1_referent\":{"
            "\"attr_name\":\"age\","
            "\"p_type\":\">=\","
            "\"value\":18}}"
            "}";

    NSString *claimDef = @"{\"primary\":{\"n\":\"94759924268422840873493186881483285628376767714620627055233230078254863658476446487556117977593248501523199451418346650764648601684276437772084327637083000213497377603495837360299641742248892290843802071224822481683143989223918276185323177379400413928352871249494885563503003839960930062341074783742062464846448855510814252519824733234277681749977392772900212293652238651538092092030867161752390937372967233462027620699196724949212432236376627703446877808405786247217818975482797381180714523093913559060716447170497587855871901716892114835713057965087473682457896508094049813280368069805661739141591558517233009123957\",\"s\":\"3589207374161609293256840431433442367968556468254553005135697551692970564853243905310862234226531556373974144223993822323573625466428920716249949819187529684239371465431718456502388533731367046146704547241076626874082510133130124364613881638153345624380195335138152993132904167470515345775215584510356780117368593105284564368954871044494967246738070895990267205643985529060025311535539534155086912661927003271053443110788963970349858709526217650537936123121324492871282397691771309596632805099306241616501610166028401599243350835158479028294769235556557248339060399322556412171888114265194198405765574333538019124846\",\"rms\":\"57150374376895616256492932008792437185713712934712117819417607831438470701645904776986426606717466732609284990796923331049549544903261623636958698296956103821068569714644825742048584174696465882627177060166162341112552851798863535031243458188976013190131935905789786836375734914391914349188643340535242562896244661798678234667651641013894284156416773868299435641426810968290584996112925365638881750944407842890875840705650290814965768221299488400872767679122749231050406680432079499973527780212310700022178178822528199576164498116369689770884051691678056831493476045361227274839673581033532995523269047577973637307053\",\"r\":{\"age\":\"94304485801056920773231824603827244147437820123357994068540328541540143488826838939836897544389872126768239056314698953816072289663428273075648246498659039419931054256171488371404693243192741923382499918184822032756852725234903892700640856294525441486319095181804549558538523888770076173572615957495813339649470619615099181648313548341951673407624414494737018574238782648822189142664108450534642272145962844003886059737965854042074083374478426875684184904488545593139633653407062308621502392373426120986761417580127895634822264744063122368296502161439648408926687989964483291459079738447940651025900007635890755686910\",\"sex\":\"29253365609829921413347591854991689007250272038394995372767401325848195298844802462252851926995846503104090589196060683329875231216529049681648909174047403783834364995363938741001507091534282239210301727771803410513303526378812888571225762557471133950393342500638551458868147905023198508660460641434022020257614450354085808398293279060446966692082427506909617283562394303716193372887306176319841941848888379308208426966697446699225783646634631703732019477632822374479322570142967559738439193417309205283438893083349863592921249218168590490390313109776446516881569691499831380592661740653935515397472059631417493981532\",\"name\":\"25134437486609445980011967476486104706321061312022352268621323694861467756181853100693555519614894168921947814126694858839278103549577703105305116890325322098078409416441750313062396467567140699008203113519528887729951138845002409659317083029073793314514377377412805387401717457417895322600145580639449003584446356048213839274172751441145076183734269045919984853749007476629365146654240675320041155618450449041510280560040162429566008590065069477149918088087715269037925211599101597422023202484497946662159070023999719865939258557778022770035320019440597702090334486792710436579355608406897769514395306079855023848170\",\"height\":\"59326960517737425423547279838932030505937927873589489863081026714907925093402287263487670945897247474465655528290016645774365383046524346223348261262488616342337864633104758662753452450299389775751012589698563659277683974188553993694220606310980581680471280640591973543996299789038056921309016983827578247477799948667666717056420270448516049047961099547588510086600581628091290215485826514170097211360599793229701811672966818089371089216189744274422526431130783428589346341196561742409198605034972210917502326180305735092988639850309253190875578501020679137562856724998821945605494355779034135306337094344532980411836\"},\"rctxt\":\"9641986614889199796257508700106896585587271615330980339636468819377346498767697681332046156705231986464570206666984343024200482683981302064613556104594051003956610353281701880542337665385482309134369756144345334575765116656633321636736946947493150642615481313285221467998414924865943067790561494301461899025374692884841352282256044388512875752628313052128404892424405230961678931620525106856624692942373538946467902799339061714326383378018581568876147181355325663707572429090278505823900491548970098691127791086305310899642155499128171811034581730190877600697624903963241473287185133286356124371104261592694271730029\",\"z\":\"77594127026421654059198621152153180600664927707984020918609426112642522289621323453889995053400171879296098965678384769043918218957929606187082395048777546641833348694470081024386996548890150355901703252426977094536933434556202865213941384425538749866521536494046548509344678288447175898173634381514948562261015286492185924659638474376885655055568341574638453213864956407243206035973349529545863886325462867413885904072942842465859476940638839087894582648849969332663627779378998245133055807038199937421971988505911494931665143822588532097754480882750243126847177560978100527491344463525107644125030963904001009159559\"},\"revocation\":null,\"schema_seq_no\":1,\"signature_type\":\"CL\"}";


    NSString *schemasJson = [NSString stringWithFormat:@"{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":%@}", [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)]];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":%@}", claimDef];

    NSString *revocRegsJsons = @"{}";

    NSString *proofJson = [NSString stringWithFormat:@"{\"proof\":{\"proofs\":{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":{\"proof\":{\"primary_proof\":{\"eq_proof\":\"a_prime\":\"47629821806628155353444789773246165920681315271529392722265555946090524267165563309836167110610840740533588118152308411732923636370660640410661034994521654033599863817144282118006097899736622728860229305231675970853294584911572355833537271010861501353858292189045263114095480601737776505186511389129055847562085611741257601964074827979121349153316235245772819207422031038042586202074331681302501661153569340935741290924699468188826629478130140797677338573924284871118002193526319478550852287453975107498037063076866410320160118555629090040954555043934303307652160345244864713226315470541231435958298648179413077988340\",\"e\":\"13427639393364185909415877973872458621259927563729922146828001652769380799419438410309469022979920689628523901764614163117469683925816443\",\"v\":\"852136445143816932026946294488424887907102968158908948827421962603492187508454543239422067899916472317305416590471170842186669606584356963437132366711335927890209765986844538775191207999204354235774464468525274918097404114453069375363594310105209141774763909570100638835926337238009617444858777301355087706167735590386774813901740600054753028260344014744801229032610106838480523182317262113911183640784111960909501662169298536941919854667754097841344375972975021196106884215734228415868248724905018661498061287694439466570946597514142085096419985189064172035527690786158872698717583830848410994616274586162550376126607414773916066374234063208380831144157533076866210628625236440222547584539349936639548061601416341705483504386186280800509889531835172071717956251546280392606775903107774727736794828168898273891724336881907672405328368540895104468091907771325910937575557566831844131159128453840354307814975621978196047820\",\"m\":{\"age\":\"1117601261519431120446925325460734824239475567013636538481947258329666056692767097795046086413732472111811628751812987521644198549167671875326968410921589186689138994171774838662\",\"height\":\"7064132689652704067914104576495132313294680087958177180391515757079548676035445873279966783996928425154050462229933823707574545166617858646442019030600136959459527533262821184869\",\"sex\":\"16084497853957041205729191269508720470626311156190485518484640641677445098603656354458362520541393995692536218820724164533958162674375198846036330444513484319280148335515891811530\"},\"m1\":\"154906224259819061652290487122980839849626068919893070220438585977323162319993111920906032317552959103520053742608858935542877608981244065301675821390065831113115709272412144796159984624494428122971995557415296140268002332169405587907128359886810433617416529821500995701094400375272097687818064435577784795275\",\"m2\":\"13805395408072590464827983892588030341708765524663545700917462089376137940485022437657208204460048097312372685954050370540389593952001973312378647790917367330461398089529292217752\"},\"ge_proofs\":[{\"u\":{\"1\":\"7698818972783845439601187851976452936638792889455287252542709653271706844173743185409084669157965935169942655008606334521674712818637940377261656468700786810566551698412412949418\",\"0\":\"11703047052430512223413711314521545616119146623040600935686474696241801697819280425232876917607198335376453682738553665221410353412906194951254558355994401995990233992518110582450\",\"3\":\"13210777821918317858818819091924507295018522783042111457450035423463340571245465760486275059291363621513532153389441883097799049597687545496359999443320001567152794884095192951040\",\"2\":\"15219471780524079156861690098171693383497641272226737821992208834301871102152362116211452788300889697214391366996966539871625433480959011635688106136537800706217506402845296449689\"},\"r\":{\"1\":\"46043242109380749151527145850513330956077996622769158245225343392397735706292106535150958053995712629189143692293204979798837951212291825184346767969751978730000071952944305252032332015837054475531407691352179423131405515518588355918925056889302269768343499864256747177988825578647189563088068257214198650437730618330249172716051559993880468542083352885474175039320848153156858562341041960950299312991459780503345784440261679263045723337629951517601461685539857683027034345542399365706329805317943096391758978877658949910614447086409173234155028671453929715706057153381022697673192590033507204548864311227048268516889390503318015295207078022755834130221198717787608473222789491216667698651180077661375273569115943192\",\"0\":\"135472587547410377947826119498467634347118057359097899596599164976338466445104141784869016998150489852448547539824768048351359572626675997498079394825940306636285481821620973655797996638210760710325933304918452142858879806106214845499670718704532018129553348815327362843246706518826311676917538452317818631484884032929252959289913274829848084561421467966320595980172006456003183536232790787521924655750157145207798486087511869939940023266736153366338179116840490184005332351004990854691988404031259910319601383696749511809898297656135548118786342107367065232798999979296280467063561892962526945512167505847049907450058650930480352253243357594344686769208712964458923557777584158831146374282687397585726706489164423632\",\"DELTA\":\"93540839493959971552865423901789226093328763011922445919928571946113703515842729132879472109395228387208764738970926484618949870591214627692618668077375153559192701474693025462226656116549337248146652482501255820930607033869432220667968682424554711616471973627651716863421554516577716366331699848682958681216261888139409101603059124344125075525791543312721162515584942523419876134808829569829529457617639955678189490257208141837196965948342373022812790844435050648360150869293836349223060722858500537182872294143846213258360218898475766641125493477502149553491502593654061863323857297998048614447925371606038801933864960337435890254277043261512846682042139570000962051463878026338583242360548041329046695667868842400\",\"3\":\"1227675452527605924725300993571504188580051470857656204064614533296779844072852823820754766175236321050062349182891221840452517985644028521499240739391613871973822807731772613052644168369405390658793869751915172749739844553410726807277698347769400977274750672880389943392076308065414059539317340070691852044062594715307024113666759844200606183662256825096857658837519571386467051003466014468855293015652584667669998830524947537781865745830650392641812221679438090257444660715937570193098993118585554478799821072396238689063767016402460690760792908977364175126682041704095200572282644311025594681667826054722587271200221036938804846621444065128275082392082327596239358623150786484106872933657139420542280145197712634108\",\"2\":\"596248147592834822582469335300585333722415132713749620075902332764163096347819006925876158892694742461036531935093982309708492066217459300117157420442081698140277277546563570823996272914068575482008392971932777453900260626542725308060927710122631763045025742980634216666560934260634907599194353151523256914796667535940073668465664206971169038864484235442207811974981191879443614478897291543702607764944403808380921189291059195014621592027660463072969363556421687131446107696579365265893962197300447027501604372738056016734644378437907931412654753728514905671605635291285742886484416973884856055084605172305967034292646171874483670469193852404511746786039743401185954843446037600121496137915619789351744485264614840070\"},\"mj\":\"1117601261519431120446925325460734824239475567013636538481947258329666056692767097795046086413732472111811628751812987521644198549167671875326968410921589186689138994171774838662\",\"alpha\":\"76727612740067576380015106087224381023260815407331375101920043509817863645705120013304683427627332447210083684516403565749916480947649443674885388155460323163682547865307733144184097845709556309570345707127872162476432029772452433292049079349274445907295491125915363620615679995457134810061392296263970553630102299601689685622244925494554558218277670233361938142224820526392365740420502452466959099546877778248089664282581792213376636587293479012783947088070052463503335266180110771978445892744225891676396288437005847308189508347446490710626231658457908472341606549292437553353163031111068977301305043175839949352742711874426231072729977019365761072816602400121302646283352164756787266537474728685656685493249314400351742964904006326192403855909148605656818024621453179832395687665671245528217931951331393482249182516107670379946496778373\",\"t\":{\"1\":\"37203689290881948278188715497642400459048942241931994079434400288578680362970117779048886269388440270597283202033458042171954610700745461571112086648991639439510380585728148682202768590972068041537531136529323260832899360551065706810590032715173070285762675403853992183366951113799098912676809373169763887110420539387555392787590966452796271491986622992160642135480293110112269570862265489120557014181468118619500321000966443141863893743211690388599242584469856365803370202569641902205925191670838354052104480074127555862332399641076324738839120815544432811566503174551735326387678621283249883091766325861497740614317\",\"3\":\"58486787977689017034592833190899828017343431922483563651969628402499947729293364026001243898136737211851089198526360764391403150763769829047179796728616126204105160762333590343947446892105646111520243793053992399512412375936746396187319527051818920531870855183738837254656664620975569939859368862778444291640228229744805843388153451336792379036403300211151424879060241580540910888241769468335914016289938374111481091198264912969768783884602931940994543804730631920434719776196148182987249363641941951160704928605829395517074202388967815738516252602903999010405305463910751219873354588685197134114358234107748126140977\",\"0\":\"60771874648036182010335841594233428920565254732600738082343398028553347795361460295011584446745121430144172025428394361648540904134739046923992231536160801306934272250969829886396340824213814702904457884984387666505055153957942221822193548673145705543973635530652570436109428474727638128773540793530691399549837156239786231362112148914687724325416768262058486101761972044802628459748878200584371058300150212485731451700436345975266860685549673168984700174294811561393162860595319582236734968601457003780816977537443267217411297266600994916897237305128142313335280264655603445636393371224354539882875937093696844430903\",\"DELTA\":\"32816484171372208266594641116109072545171919234551585018140151846920408763078147655907777031259225522515086979967895258126318315788662577171150780535509410112003001556402222994276811926864642497249250763185467678044678144507739529818566125668667424447792097244624010084189629269472698722402896445274092470014229247479740671263651727480322483037149584904549203417226525624083290572692241241259382947122018271686649224741832992966652878170311798126004447080305528487720923103595513611363001766063956060990267107048028416069435287244770875463867263571308182619338433913487209319707428378896314619624990311543563016697299\",\"2\":\"36428320569485697540634597755814766104888687488985202673924762266313135133244610404742081973550848160712054769198012193456278135847215508952327879544434490828380496286187725750283788811367824465072001959950807751252194618152990469069074061195618692339915840384087350671392595652921761835083158086795163935060896053332506433434451836095710383871272788002621913967538399141417857031787255744141437237474972197102809365346359345477248611632307159641948507043668113827177494748159094045928919209335044052792843664865311991178972383241855607627188111601119780878072683890170539599447876998109080150992209773901144245398001\"},\"predicate\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}]},\"non_revoc_proof\":null}}},\"aggregated_proof\":{\"c_hash\":\"33103550379681684069592829341967479618752165928802550870585275205292715916069\",\"c_list\":[[1,121,77,5,144,154,14,192,190,190,145,180,128,71,22,60,168,20,46,163,139,194,71,165,220,188,121,76,25,146,231,114,65,54,69,68,19,200,250,192,47,123,157,132,74,50,28,69,226,195,243,118,45,63,237,197,216,202,206,101,33,56,225,200,128,3,89,12,182,38,113,221,165,119,228,201,156,201,172,136,59,64,51,72,164,198,49,228,223,117,80,64,166,226,37,8,29,146,186,80,210,119,76,252,4,255,62,218,112,163,164,147,247,190,108,76,140,191,76,217,214,184,152,179,193,149,15,70,197,46,90,60,255,247,197,219,252,73,76,0,125,104,114,22,182,161,110,36,162,103,27,42,88,18,161,237,198,43,177,189,181,86,135,207,71,114,0,26,175,12,199,125,25,124,178,87,36,208,251,15,191,127,202,148,152,43,142,92,191,7,89,153,130,195,223,248,176,109,97,164,126,162,181,124,237,130,155,197,66,59,40,197,72,84,32,100,64,55,227,60,214,143,200,200,89,115,236,172,145,56,100,73,20,242,233,95,130,58,112,153,120,115,119,42,199,30,205,88,223,42,196,184,41,19,100,19,244],[1,225,103,238,42,147,91,191,110,69,154,53,57,156,124,43,174,155,76,202,193,98,128,38,207,126,66,70,161,96,109,127,174,44,203,198,177,238,118,117,89,227,170,155,44,251,35,119,219,29,100,173,26,144,95,50,177,4,40,234,117,174,210,192,172,57,160,198,42,199,212,243,240,114,59,91,207,68,57,38,198,2,73,18,16,209,182,145,206,71,17,69,222,49,36,120,72,117,169,107,238,208,235,216,24,183,201,81,15,83,242,45,136,184,166,26,142,136,228,58,229,235,88,169,238,134,205,96,85,9,122,53,147,100,183,114,92,54,125,178,125,75,127,116,50,88,109,152,22,4,121,252,190,18,190,130,143,138,59,231,38,131,176,54,19,194,218,67,144,122,91,43,86,73,233,48,193,30,183,183,191,238,216,167,101,28,185,43,118,64,242,16,62,239,177,27,109,144,67,221,175,202,4,92,130,74,24,20,151,15,227,225,142,71,145,46,192,248,87,57,183,142,253,52,20,56,153,220,234,25,67,116,225,179,211,116,161,37,64,34,48,155,1,1,159,157,37,31,202,19,229,152,23,138,183,126,55],[1,38,181,193,191,72,2,239,34,83,49,36,179,160,82,112,172,98,255,63,60,22,177,249,67,215,220,198,181,7,49,254,133,243,221,214,47,64,229,82,11,94,175,57,86,152,229,192,184,96,136,116,226,123,128,217,23,244,19,204,36,44,123,208,88,24,217,120,145,139,25,233,227,5,119,90,47,147,1,115,92,39,119,194,167,17,229,39,163,167,237,14,116,234,106,252,216,54,33,233,21,54,183,130,144,161,177,142,177,240,51,73,21,202,188,103,244,153,204,219,123,231,139,135,189,155,143,28,4,180,44,148,0,27,103,26,13,203,31,32,166,67,84,87,23,72,234,236,20,1,84,70,86,76,192,164,235,124,86,128,78,230,119,155,95,121,125,20,244,181,121,250,169,9,67,85,213,177,139,111,187,183,114,165,249,177,161,181,175,46,226,66,86,84,124,86,69,143,217,158,161,30,107,133,44,239,89,209,24,150,1,238,122,144,138,179,121,114,90,13,212,209,60,126,37,62,177,180,131,222,168,2,201,156,169,220,224,53,8,203,220,215,163,104,195,184,73,35,241,182,177,80,41,253,230,90,173],[1,32,145,96,219,241,190,19,195,129,219,50,148,152,107,12,189,225,103,171,149,252,193,243,136,132,195,44,19,20,247,140,160,91,230,78,31,242,85,213,65,185,1,91,12,69,118,80,26,135,102,131,4,108,130,230,83,91,176,249,196,56,128,127,82,72,106,49,211,94,133,40,86,72,42,187,199,216,191,223,208,206,121,118,15,167,255,228,57,206,158,217,64,205,212,178,8,248,129,183,221,98,70,54,37,55,47,81,120,59,186,238,165,0,70,173,137,193,232,180,125,211,237,182,249,191,173,107,129,164,148,231,116,225,66,66,71,156,39,248,164,253,234,140,205,177,140,117,47,21,15,242,31,113,118,91,143,89,213,86,143,135,21,46,35,199,214,107,111,65,65,19,26,171,130,16,19,102,145,210,210,61,51,169,148,169,118,182,106,107,253,100,214,232,52,103,180,96,249,254,71,6,11,119,48,129,213,223,205,93,20,117,26,187,32,151,212,137,203,17,237,208,150,72,23,225,235,122,188,34,105,115,0,160,168,251,191,22,242,238,207,74,142,154,66,94,149,191,215,194,134,6,165,244,167,233,241],[1,207,77,250,146,127,242,229,44,172,182,201,183,242,32,242,182,129,233,10,8,180,23,191,163,21,238,158,5,27,216,146,253,173,127,99,95,168,209,132,242,196,242,34,25,25,249,211,51,236,164,153,175,61,65,150,82,251,174,102,186,47,195,82,44,90,252,184,74,89,251,177,254,108,151,136,230,220,93,224,173,247,244,116,132,59,170,215,194,30,87,84,166,147,57,156,201,207,132,203,222,191,253,15,19,228,173,81,156,4,51,121,227,159,50,18,148,129,205,42,42,227,252,138,62,176,115,227,253,52,125,110,178,167,132,244,14,116,195,194,172,44,45,63,38,121,215,136,68,230,21,108,133,159,197,179,94,78,233,107,236,114,92,165,248,22,124,161,23,142,236,224,175,233,134,25,97,150,131,61,220,203,104,154,199,247,146,47,205,56,209,0,133,132,18,103,136,8,202,37,29,100,105,12,232,74,33,6,255,202,96,170,52,229,244,4,235,2,201,125,86,168,179,224,130,81,54,221,185,184,187,141,0,114,98,38,70,225,228,60,157,53,210,238,60,216,215,154,48,73,3,157,192,245,81,170,49],[1,3,244,229,158,71,18,146,198,202,27,2,231,37,13,145,243,84,112,220,61,174,4,175,104,200,64,146,193,20,174,126,42,157,168,76,165,21,50,216,82,211,180,73,244,54,227,200,19,157,25,228,81,37,64,201,19,138,175,50,246,169,11,45,74,194,131,236,127,177,41,242,130,55,112,182,98,22,99,48,153,83,161,250,65,89,3,97,6,5,171,54,223,87,98,103,23,200,212,177,140,155,151,252,125,45,176,55,92,41,56,2,252,32,149,60,3,168,209,193,23,168,230,182,72,193,230,224,5,15,58,63,93,196,33,93,76,188,30,70,31,136,64,204,223,2,230,210,243,255,135,193,52,132,248,160,22,18,164,71,77,80,112,229,120,116,210,225,2,19,139,35,0,214,5,246,9,106,136,204,0,148,97,21,222,153,57,177,162,11,243,252,7,242,34,239,245,50,104,74,221,92,73,13,142,10,184,250,246,167,240,46,230,86,207,181,12,133,81,119,143,164,88,114,223,243,179,208,175,84,161,27,11,225,36,37,177,112,85,81,184,163,223,159,36,9,247,20,13,230,215,108,117,35,99,117,211]]}},\"requested_proof\":{\"revealed_attrs\":{\"attr1_referent\":[\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\",\"Alex\",\"1139481716457488690172217916278103335\"]},\"unrevealed_attrs\":{},\"self_attested_attrs\":{},\"predicates\":{\"predicate1_referent\":\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\"}},\"identifiers\":{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":{\"\issuer_did\":\"did\",schema_key:%@}}}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJsons
                                                      outValid:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::verifierVerifyProof returned wrong code");
}

- (void)testVerifierVerifyProofWorksForInvalidSchemas {
    // 1. init commmon wallet
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                                             claimDefJson:nil
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *proofReqJson = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{"
            "\"attr1_referent\":{"
            "\"name\":\"name\"}},"
            "\"requested_predicates\":{"
            "\"predicate1_referent\":{"
            "\"attr_name\":\"age\","
            "\"p_type\":\">=\","
            "\"value\":18}}"
            "}";

    NSString *claimDef = @"{\"primary\":{\"n\":\"94759924268422840873493186881483285628376767714620627055233230078254863658476446487556117977593248501523199451418346650764648601684276437772084327637083000213497377603495837360299641742248892290843802071224822481683143989223918276185323177379400413928352871249494885563503003839960930062341074783742062464846448855510814252519824733234277681749977392772900212293652238651538092092030867161752390937372967233462027620699196724949212432236376627703446877808405786247217818975482797381180714523093913559060716447170497587855871901716892114835713057965087473682457896508094049813280368069805661739141591558517233009123957\",\"s\":\"3589207374161609293256840431433442367968556468254553005135697551692970564853243905310862234226531556373974144223993822323573625466428920716249949819187529684239371465431718456502388533731367046146704547241076626874082510133130124364613881638153345624380195335138152993132904167470515345775215584510356780117368593105284564368954871044494967246738070895990267205643985529060025311535539534155086912661927003271053443110788963970349858709526217650537936123121324492871282397691771309596632805099306241616501610166028401599243350835158479028294769235556557248339060399322556412171888114265194198405765574333538019124846\",\"rms\":\"57150374376895616256492932008792437185713712934712117819417607831438470701645904776986426606717466732609284990796923331049549544903261623636958698296956103821068569714644825742048584174696465882627177060166162341112552851798863535031243458188976013190131935905789786836375734914391914349188643340535242562896244661798678234667651641013894284156416773868299435641426810968290584996112925365638881750944407842890875840705650290814965768221299488400872767679122749231050406680432079499973527780212310700022178178822528199576164498116369689770884051691678056831493476045361227274839673581033532995523269047577973637307053\",\"r\":{\"age\":\"94304485801056920773231824603827244147437820123357994068540328541540143488826838939836897544389872126768239056314698953816072289663428273075648246498659039419931054256171488371404693243192741923382499918184822032756852725234903892700640856294525441486319095181804549558538523888770076173572615957495813339649470619615099181648313548341951673407624414494737018574238782648822189142664108450534642272145962844003886059737965854042074083374478426875684184904488545593139633653407062308621502392373426120986761417580127895634822264744063122368296502161439648408926687989964483291459079738447940651025900007635890755686910\",\"sex\":\"29253365609829921413347591854991689007250272038394995372767401325848195298844802462252851926995846503104090589196060683329875231216529049681648909174047403783834364995363938741001507091534282239210301727771803410513303526378812888571225762557471133950393342500638551458868147905023198508660460641434022020257614450354085808398293279060446966692082427506909617283562394303716193372887306176319841941848888379308208426966697446699225783646634631703732019477632822374479322570142967559738439193417309205283438893083349863592921249218168590490390313109776446516881569691499831380592661740653935515397472059631417493981532\",\"name\":\"25134437486609445980011967476486104706321061312022352268621323694861467756181853100693555519614894168921947814126694858839278103549577703105305116890325322098078409416441750313062396467567140699008203113519528887729951138845002409659317083029073793314514377377412805387401717457417895322600145580639449003584446356048213839274172751441145076183734269045919984853749007476629365146654240675320041155618450449041510280560040162429566008590065069477149918088087715269037925211599101597422023202484497946662159070023999719865939258557778022770035320019440597702090334486792710436579355608406897769514395306079855023848170\",\"height\":\"59326960517737425423547279838932030505937927873589489863081026714907925093402287263487670945897247474465655528290016645774365383046524346223348261262488616342337864633104758662753452450299389775751012589698563659277683974188553993694220606310980581680471280640591973543996299789038056921309016983827578247477799948667666717056420270448516049047961099547588510086600581628091290215485826514170097211360599793229701811672966818089371089216189744274422526431130783428589346341196561742409198605034972210917502326180305735092988639850309253190875578501020679137562856724998821945605494355779034135306337094344532980411836\"},\"rctxt\":\"9641986614889199796257508700106896585587271615330980339636468819377346498767697681332046156705231986464570206666984343024200482683981302064613556104594051003956610353281701880542337665385482309134369756144345334575765116656633321636736946947493150642615481313285221467998414924865943067790561494301461899025374692884841352282256044388512875752628313052128404892424405230961678931620525106856624692942373538946467902799339061714326383378018581568876147181355325663707572429090278505823900491548970098691127791086305310899642155499128171811034581730190877600697624903963241473287185133286356124371104261592694271730029\",\"z\":\"77594127026421654059198621152153180600664927707984020918609426112642522289621323453889995053400171879296098965678384769043918218957929606187082395048777546641833348694470081024386996548890150355901703252426977094536933434556202865213941384425538749866521536494046548509344678288447175898173634381514948562261015286492185924659638474376885655055568341574638453213864956407243206035973349529545863886325462867413885904072942842465859476940638839087894582648849969332663627779378998245133055807038199937421971988505911494931665143822588532097754480882750243126847177560978100527491344463525107644125030963904001009159559\"},\"revocation\":null,\"schema_seq_no\":1,\"signature_type\":\"CL\"}";

    NSString *schemasJson = [NSString stringWithFormat:@"{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":%@}", [[AnoncredsUtils sharedInstance] getXyzSchemaJson:@(1)]];

    NSString *claimDefsJson = [NSString stringWithFormat:@"{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":%@}", claimDef];

    NSString *revocRegsJsons = @"{}";

    NSString *proofJson = [NSString stringWithFormat:@"{\"proof\":{\"proofs\":{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":{\"primary_proof\":{\"eq_proof\":{\"revealed_attrs\":{\"name\":\"1139481716457488690172217916278103335\"},\"a_prime\":\"47629821806628155353444789773246165920681315271529392722265555946090524267165563309836167110610840740533588118152308411732923636370660640410661034994521654033599863817144282118006097899736622728860229305231675970853294584911572355833537271010861501353858292189045263114095480601737776505186511389129055847562085611741257601964074827979121349153316235245772819207422031038042586202074331681302501661153569340935741290924699468188826629478130140797677338573924284871118002193526319478550852287453975107498037063076866410320160118555629090040954555043934303307652160345244864713226315470541231435958298648179413077988340\",\"e\":\"13427639393364185909415877973872458621259927563729922146828001652769380799419438410309469022979920689628523901764614163117469683925816443\",\"v\":\"852136445143816932026946294488424887907102968158908948827421962603492187508454543239422067899916472317305416590471170842186669606584356963437132366711335927890209765986844538775191207999204354235774464468525274918097404114453069375363594310105209141774763909570100638835926337238009617444858777301355087706167735590386774813901740600054753028260344014744801229032610106838480523182317262113911183640784111960909501662169298536941919854667754097841344375972975021196106884215734228415868248724905018661498061287694439466570946597514142085096419985189064172035527690786158872698717583830848410994616274586162550376126607414773916066374234063208380831144157533076866210628625236440222547584539349936639548061601416341705483504386186280800509889531835172071717956251546280392606775903107774727736794828168898273891724336881907672405328368540895104468091907771325910937575557566831844131159128453840354307814975621978196047820\",\"m\":{\"age\":\"1117601261519431120446925325460734824239475567013636538481947258329666056692767097795046086413732472111811628751812987521644198549167671875326968410921589186689138994171774838662\",\"height\":\"7064132689652704067914104576495132313294680087958177180391515757079548676035445873279966783996928425154050462229933823707574545166617858646442019030600136959459527533262821184869\",\"sex\":\"16084497853957041205729191269508720470626311156190485518484640641677445098603656354458362520541393995692536218820724164533958162674375198846036330444513484319280148335515891811530\"},\"m1\":\"154906224259819061652290487122980839849626068919893070220438585977323162319993111920906032317552959103520053742608858935542877608981244065301675821390065831113115709272412144796159984624494428122971995557415296140268002332169405587907128359886810433617416529821500995701094400375272097687818064435577784795275\",\"m2\":\"13805395408072590464827983892588030341708765524663545700917462089376137940485022437657208204460048097312372685954050370540389593952001973312378647790917367330461398089529292217752\"},\"ge_proofs\":[{\"u\":{\"1\":\"7698818972783845439601187851976452936638792889455287252542709653271706844173743185409084669157965935169942655008606334521674712818637940377261656468700786810566551698412412949418\",\"0\":\"11703047052430512223413711314521545616119146623040600935686474696241801697819280425232876917607198335376453682738553665221410353412906194951254558355994401995990233992518110582450\",\"3\":\"13210777821918317858818819091924507295018522783042111457450035423463340571245465760486275059291363621513532153389441883097799049597687545496359999443320001567152794884095192951040\",\"2\":\"15219471780524079156861690098171693383497641272226737821992208834301871102152362116211452788300889697214391366996966539871625433480959011635688106136537800706217506402845296449689\"},\"r\":{\"1\":\"46043242109380749151527145850513330956077996622769158245225343392397735706292106535150958053995712629189143692293204979798837951212291825184346767969751978730000071952944305252032332015837054475531407691352179423131405515518588355918925056889302269768343499864256747177988825578647189563088068257214198650437730618330249172716051559993880468542083352885474175039320848153156858562341041960950299312991459780503345784440261679263045723337629951517601461685539857683027034345542399365706329805317943096391758978877658949910614447086409173234155028671453929715706057153381022697673192590033507204548864311227048268516889390503318015295207078022755834130221198717787608473222789491216667698651180077661375273569115943192\",\"0\":\"135472587547410377947826119498467634347118057359097899596599164976338466445104141784869016998150489852448547539824768048351359572626675997498079394825940306636285481821620973655797996638210760710325933304918452142858879806106214845499670718704532018129553348815327362843246706518826311676917538452317818631484884032929252959289913274829848084561421467966320595980172006456003183536232790787521924655750157145207798486087511869939940023266736153366338179116840490184005332351004990854691988404031259910319601383696749511809898297656135548118786342107367065232798999979296280467063561892962526945512167505847049907450058650930480352253243357594344686769208712964458923557777584158831146374282687397585726706489164423632\",\"DELTA\":\"93540839493959971552865423901789226093328763011922445919928571946113703515842729132879472109395228387208764738970926484618949870591214627692618668077375153559192701474693025462226656116549337248146652482501255820930607033869432220667968682424554711616471973627651716863421554516577716366331699848682958681216261888139409101603059124344125075525791543312721162515584942523419876134808829569829529457617639955678189490257208141837196965948342373022812790844435050648360150869293836349223060722858500537182872294143846213258360218898475766641125493477502149553491502593654061863323857297998048614447925371606038801933864960337435890254277043261512846682042139570000962051463878026338583242360548041329046695667868842400\",\"3\":\"1227675452527605924725300993571504188580051470857656204064614533296779844072852823820754766175236321050062349182891221840452517985644028521499240739391613871973822807731772613052644168369405390658793869751915172749739844553410726807277698347769400977274750672880389943392076308065414059539317340070691852044062594715307024113666759844200606183662256825096857658837519571386467051003466014468855293015652584667669998830524947537781865745830650392641812221679438090257444660715937570193098993118585554478799821072396238689063767016402460690760792908977364175126682041704095200572282644311025594681667826054722587271200221036938804846621444065128275082392082327596239358623150786484106872933657139420542280145197712634108\",\"2\":\"596248147592834822582469335300585333722415132713749620075902332764163096347819006925876158892694742461036531935093982309708492066217459300117157420442081698140277277546563570823996272914068575482008392971932777453900260626542725308060927710122631763045025742980634216666560934260634907599194353151523256914796667535940073668465664206971169038864484235442207811974981191879443614478897291543702607764944403808380921189291059195014621592027660463072969363556421687131446107696579365265893962197300447027501604372738056016734644378437907931412654753728514905671605635291285742886484416973884856055084605172305967034292646171874483670469193852404511746786039743401185954843446037600121496137915619789351744485264614840070\"},\"mj\":\"1117601261519431120446925325460734824239475567013636538481947258329666056692767097795046086413732472111811628751812987521644198549167671875326968410921589186689138994171774838662\",\"alpha\":\"76727612740067576380015106087224381023260815407331375101920043509817863645705120013304683427627332447210083684516403565749916480947649443674885388155460323163682547865307733144184097845709556309570345707127872162476432029772452433292049079349274445907295491125915363620615679995457134810061392296263970553630102299601689685622244925494554558218277670233361938142224820526392365740420502452466959099546877778248089664282581792213376636587293479012783947088070052463503335266180110771978445892744225891676396288437005847308189508347446490710626231658457908472341606549292437553353163031111068977301305043175839949352742711874426231072729977019365761072816602400121302646283352164756787266537474728685656685493249314400351742964904006326192403855909148605656818024621453179832395687665671245528217931951331393482249182516107670379946496778373\",\"t\":{\"1\":\"37203689290881948278188715497642400459048942241931994079434400288578680362970117779048886269388440270597283202033458042171954610700745461571112086648991639439510380585728148682202768590972068041537531136529323260832899360551065706810590032715173070285762675403853992183366951113799098912676809373169763887110420539387555392787590966452796271491986622992160642135480293110112269570862265489120557014181468118619500321000966443141863893743211690388599242584469856365803370202569641902205925191670838354052104480074127555862332399641076324738839120815544432811566503174551735326387678621283249883091766325861497740614317\",\"3\":\"58486787977689017034592833190899828017343431922483563651969628402499947729293364026001243898136737211851089198526360764391403150763769829047179796728616126204105160762333590343947446892105646111520243793053992399512412375936746396187319527051818920531870855183738837254656664620975569939859368862778444291640228229744805843388153451336792379036403300211151424879060241580540910888241769468335914016289938374111481091198264912969768783884602931940994543804730631920434719776196148182987249363641941951160704928605829395517074202388967815738516252602903999010405305463910751219873354588685197134114358234107748126140977\",\"0\":\"60771874648036182010335841594233428920565254732600738082343398028553347795361460295011584446745121430144172025428394361648540904134739046923992231536160801306934272250969829886396340824213814702904457884984387666505055153957942221822193548673145705543973635530652570436109428474727638128773540793530691399549837156239786231362112148914687724325416768262058486101761972044802628459748878200584371058300150212485731451700436345975266860685549673168984700174294811561393162860595319582236734968601457003780816977537443267217411297266600994916897237305128142313335280264655603445636393371224354539882875937093696844430903\",\"DELTA\":\"32816484171372208266594641116109072545171919234551585018140151846920408763078147655907777031259225522515086979967895258126318315788662577171150780535509410112003001556402222994276811926864642497249250763185467678044678144507739529818566125668667424447792097244624010084189629269472698722402896445274092470014229247479740671263651727480322483037149584904549203417226525624083290572692241241259382947122018271686649224741832992966652878170311798126004447080305528487720923103595513611363001766063956060990267107048028416069435287244770875463867263571308182619338433913487209319707428378896314619624990311543563016697299\",\"2\":\"36428320569485697540634597755814766104888687488985202673924762266313135133244610404742081973550848160712054769198012193456278135847215508952327879544434490828380496286187725750283788811367824465072001959950807751252194618152990469069074061195618692339915840384087350671392595652921761835083158086795163935060896053332506433434451836095710383871272788002621913967538399141417857031787255744141437237474972197102809365346359345477248611632307159641948507043668113827177494748159094045928919209335044052792843664865311991178972383241855607627188111601119780878072683890170539599447876998109080150992209773901144245398001\"},\"predicate\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}]},\"non_revoc_proof\":null}},\"aggregated_proof\":{\"c_hash\":\"33103550379681684069592829341967479618752165928802550870585275205292715916069\",\"c_list\":[[1,121,77,5,144,154,14,192,190,190,145,180,128,71,22,60,168,20,46,163,139,194,71,165,220,188,121,76,25,146,231,114,65,54,69,68,19,200,250,192,47,123,157,132,74,50,28,69,226,195,243,118,45,63,237,197,216,202,206,101,33,56,225,200,128,3,89,12,182,38,113,221,165,119,228,201,156,201,172,136,59,64,51,72,164,198,49,228,223,117,80,64,166,226,37,8,29,146,186,80,210,119,76,252,4,255,62,218,112,163,164,147,247,190,108,76,140,191,76,217,214,184,152,179,193,149,15,70,197,46,90,60,255,247,197,219,252,73,76,0,125,104,114,22,182,161,110,36,162,103,27,42,88,18,161,237,198,43,177,189,181,86,135,207,71,114,0,26,175,12,199,125,25,124,178,87,36,208,251,15,191,127,202,148,152,43,142,92,191,7,89,153,130,195,223,248,176,109,97,164,126,162,181,124,237,130,155,197,66,59,40,197,72,84,32,100,64,55,227,60,214,143,200,200,89,115,236,172,145,56,100,73,20,242,233,95,130,58,112,153,120,115,119,42,199,30,205,88,223,42,196,184,41,19,100,19,244],[1,225,103,238,42,147,91,191,110,69,154,53,57,156,124,43,174,155,76,202,193,98,128,38,207,126,66,70,161,96,109,127,174,44,203,198,177,238,118,117,89,227,170,155,44,251,35,119,219,29,100,173,26,144,95,50,177,4,40,234,117,174,210,192,172,57,160,198,42,199,212,243,240,114,59,91,207,68,57,38,198,2,73,18,16,209,182,145,206,71,17,69,222,49,36,120,72,117,169,107,238,208,235,216,24,183,201,81,15,83,242,45,136,184,166,26,142,136,228,58,229,235,88,169,238,134,205,96,85,9,122,53,147,100,183,114,92,54,125,178,125,75,127,116,50,88,109,152,22,4,121,252,190,18,190,130,143,138,59,231,38,131,176,54,19,194,218,67,144,122,91,43,86,73,233,48,193,30,183,183,191,238,216,167,101,28,185,43,118,64,242,16,62,239,177,27,109,144,67,221,175,202,4,92,130,74,24,20,151,15,227,225,142,71,145,46,192,248,87,57,183,142,253,52,20,56,153,220,234,25,67,116,225,179,211,116,161,37,64,34,48,155,1,1,159,157,37,31,202,19,229,152,23,138,183,126,55],[1,38,181,193,191,72,2,239,34,83,49,36,179,160,82,112,172,98,255,63,60,22,177,249,67,215,220,198,181,7,49,254,133,243,221,214,47,64,229,82,11,94,175,57,86,152,229,192,184,96,136,116,226,123,128,217,23,244,19,204,36,44,123,208,88,24,217,120,145,139,25,233,227,5,119,90,47,147,1,115,92,39,119,194,167,17,229,39,163,167,237,14,116,234,106,252,216,54,33,233,21,54,183,130,144,161,177,142,177,240,51,73,21,202,188,103,244,153,204,219,123,231,139,135,189,155,143,28,4,180,44,148,0,27,103,26,13,203,31,32,166,67,84,87,23,72,234,236,20,1,84,70,86,76,192,164,235,124,86,128,78,230,119,155,95,121,125,20,244,181,121,250,169,9,67,85,213,177,139,111,187,183,114,165,249,177,161,181,175,46,226,66,86,84,124,86,69,143,217,158,161,30,107,133,44,239,89,209,24,150,1,238,122,144,138,179,121,114,90,13,212,209,60,126,37,62,177,180,131,222,168,2,201,156,169,220,224,53,8,203,220,215,163,104,195,184,73,35,241,182,177,80,41,253,230,90,173],[1,32,145,96,219,241,190,19,195,129,219,50,148,152,107,12,189,225,103,171,149,252,193,243,136,132,195,44,19,20,247,140,160,91,230,78,31,242,85,213,65,185,1,91,12,69,118,80,26,135,102,131,4,108,130,230,83,91,176,249,196,56,128,127,82,72,106,49,211,94,133,40,86,72,42,187,199,216,191,223,208,206,121,118,15,167,255,228,57,206,158,217,64,205,212,178,8,248,129,183,221,98,70,54,37,55,47,81,120,59,186,238,165,0,70,173,137,193,232,180,125,211,237,182,249,191,173,107,129,164,148,231,116,225,66,66,71,156,39,248,164,253,234,140,205,177,140,117,47,21,15,242,31,113,118,91,143,89,213,86,143,135,21,46,35,199,214,107,111,65,65,19,26,171,130,16,19,102,145,210,210,61,51,169,148,169,118,182,106,107,253,100,214,232,52,103,180,96,249,254,71,6,11,119,48,129,213,223,205,93,20,117,26,187,32,151,212,137,203,17,237,208,150,72,23,225,235,122,188,34,105,115,0,160,168,251,191,22,242,238,207,74,142,154,66,94,149,191,215,194,134,6,165,244,167,233,241],[1,207,77,250,146,127,242,229,44,172,182,201,183,242,32,242,182,129,233,10,8,180,23,191,163,21,238,158,5,27,216,146,253,173,127,99,95,168,209,132,242,196,242,34,25,25,249,211,51,236,164,153,175,61,65,150,82,251,174,102,186,47,195,82,44,90,252,184,74,89,251,177,254,108,151,136,230,220,93,224,173,247,244,116,132,59,170,215,194,30,87,84,166,147,57,156,201,207,132,203,222,191,253,15,19,228,173,81,156,4,51,121,227,159,50,18,148,129,205,42,42,227,252,138,62,176,115,227,253,52,125,110,178,167,132,244,14,116,195,194,172,44,45,63,38,121,215,136,68,230,21,108,133,159,197,179,94,78,233,107,236,114,92,165,248,22,124,161,23,142,236,224,175,233,134,25,97,150,131,61,220,203,104,154,199,247,146,47,205,56,209,0,133,132,18,103,136,8,202,37,29,100,105,12,232,74,33,6,255,202,96,170,52,229,244,4,235,2,201,125,86,168,179,224,130,81,54,221,185,184,187,141,0,114,98,38,70,225,228,60,157,53,210,238,60,216,215,154,48,73,3,157,192,245,81,170,49],[1,3,244,229,158,71,18,146,198,202,27,2,231,37,13,145,243,84,112,220,61,174,4,175,104,200,64,146,193,20,174,126,42,157,168,76,165,21,50,216,82,211,180,73,244,54,227,200,19,157,25,228,81,37,64,201,19,138,175,50,246,169,11,45,74,194,131,236,127,177,41,242,130,55,112,182,98,22,99,48,153,83,161,250,65,89,3,97,6,5,171,54,223,87,98,103,23,200,212,177,140,155,151,252,125,45,176,55,92,41,56,2,252,32,149,60,3,168,209,193,23,168,230,182,72,193,230,224,5,15,58,63,93,196,33,93,76,188,30,70,31,136,64,204,223,2,230,210,243,255,135,193,52,132,248,160,22,18,164,71,77,80,112,229,120,116,210,225,2,19,139,35,0,214,5,246,9,106,136,204,0,148,97,21,222,153,57,177,162,11,243,252,7,242,34,239,245,50,104,74,221,92,73,13,142,10,184,250,246,167,240,46,230,86,207,181,12,133,81,119,143,164,88,114,223,243,179,208,175,84,161,27,11,225,36,37,177,112,85,81,184,163,223,159,36,9,247,20,13,230,215,108,117,35,99,117,211]]}},\"requested_proof\":{\"revealed_attrs\":{\"attr1_referent\":[\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\",\"Alex\",\"1139481716457488690172217916278103335\"]},\"unrevealed_attrs\":{},\"self_attested_attrs\":{},\"predicates\":{\"predicate1_referent\":\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\"}},\"identifiers\":{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":{\"schema_key\":%@,\"revoc_reg_seq_no\":null,\"issuer_did\":\"did\"}}}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJsons
                                                      outValid:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::verifierVerifyProof returned wrong code");
}

- (void)testVerifierVerifyProofWorksForInvalidClaimDefs {
    // 1. init commmon wallet
    NSError *ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                                             claimDefJson:nil
                                                                           claimOfferJson:nil
                                                                             claimReqJson:nil
                                                                                claimJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    NSString *proofReqJson = @"{"
            "\"nonce\":\"123432421212\","
            "\"name\":\"proof_req_1\","
            "\"version\":\"0.1\","
            "\"requested_attrs\":{"
            "\"attr1_referent\":{"
            "\"name\":\"name\"}},"
            "\"requested_predicates\":{"
            "\"predicate1_referent\":{"
            "\"attr_name\":\"age\","
            "\"p_type\":\">=\","
            "\"value\":18}}"
            "}";

    NSString *schemasJson = [NSString stringWithFormat:@"{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":%@}", [[AnoncredsUtils sharedInstance] getXyzSchemaJson:@(1)]];

    NSString *claimDefsJson = @"{}";

    NSString *revocRegsJsons = @"{}";

    NSString *proofJson = [NSString stringWithFormat:
            @"{\"proof\":{\"proofs\":{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":{\"primary_proof\":{\"eq_proof\":{\"revealed_attrs\":{\"name\":\"1139481716457488690172217916278103335\"},\"a_prime\":\"47629821806628155353444789773246165920681315271529392722265555946090524267165563309836167110610840740533588118152308411732923636370660640410661034994521654033599863817144282118006097899736622728860229305231675970853294584911572355833537271010861501353858292189045263114095480601737776505186511389129055847562085611741257601964074827979121349153316235245772819207422031038042586202074331681302501661153569340935741290924699468188826629478130140797677338573924284871118002193526319478550852287453975107498037063076866410320160118555629090040954555043934303307652160345244864713226315470541231435958298648179413077988340\",\"e\":\"13427639393364185909415877973872458621259927563729922146828001652769380799419438410309469022979920689628523901764614163117469683925816443\",\"v\":\"852136445143816932026946294488424887907102968158908948827421962603492187508454543239422067899916472317305416590471170842186669606584356963437132366711335927890209765986844538775191207999204354235774464468525274918097404114453069375363594310105209141774763909570100638835926337238009617444858777301355087706167735590386774813901740600054753028260344014744801229032610106838480523182317262113911183640784111960909501662169298536941919854667754097841344375972975021196106884215734228415868248724905018661498061287694439466570946597514142085096419985189064172035527690786158872698717583830848410994616274586162550376126607414773916066374234063208380831144157533076866210628625236440222547584539349936639548061601416341705483504386186280800509889531835172071717956251546280392606775903107774727736794828168898273891724336881907672405328368540895104468091907771325910937575557566831844131159128453840354307814975621978196047820\",\"m\":{\"age\":\"1117601261519431120446925325460734824239475567013636538481947258329666056692767097795046086413732472111811628751812987521644198549167671875326968410921589186689138994171774838662\",\"height\":\"7064132689652704067914104576495132313294680087958177180391515757079548676035445873279966783996928425154050462229933823707574545166617858646442019030600136959459527533262821184869\",\"sex\":\"16084497853957041205729191269508720470626311156190485518484640641677445098603656354458362520541393995692536218820724164533958162674375198846036330444513484319280148335515891811530\"},\"m1\":\"154906224259819061652290487122980839849626068919893070220438585977323162319993111920906032317552959103520053742608858935542877608981244065301675821390065831113115709272412144796159984624494428122971995557415296140268002332169405587907128359886810433617416529821500995701094400375272097687818064435577784795275\",\"m2\":\"13805395408072590464827983892588030341708765524663545700917462089376137940485022437657208204460048097312372685954050370540389593952001973312378647790917367330461398089529292217752\"},\"ge_proofs\":[{\"u\":{\"1\":\"7698818972783845439601187851976452936638792889455287252542709653271706844173743185409084669157965935169942655008606334521674712818637940377261656468700786810566551698412412949418\",\"0\":\"11703047052430512223413711314521545616119146623040600935686474696241801697819280425232876917607198335376453682738553665221410353412906194951254558355994401995990233992518110582450\",\"3\":\"13210777821918317858818819091924507295018522783042111457450035423463340571245465760486275059291363621513532153389441883097799049597687545496359999443320001567152794884095192951040\",\"2\":\"15219471780524079156861690098171693383497641272226737821992208834301871102152362116211452788300889697214391366996966539871625433480959011635688106136537800706217506402845296449689\"},\"r\":{\"1\":\"46043242109380749151527145850513330956077996622769158245225343392397735706292106535150958053995712629189143692293204979798837951212291825184346767969751978730000071952944305252032332015837054475531407691352179423131405515518588355918925056889302269768343499864256747177988825578647189563088068257214198650437730618330249172716051559993880468542083352885474175039320848153156858562341041960950299312991459780503345784440261679263045723337629951517601461685539857683027034345542399365706329805317943096391758978877658949910614447086409173234155028671453929715706057153381022697673192590033507204548864311227048268516889390503318015295207078022755834130221198717787608473222789491216667698651180077661375273569115943192\",\"0\":\"135472587547410377947826119498467634347118057359097899596599164976338466445104141784869016998150489852448547539824768048351359572626675997498079394825940306636285481821620973655797996638210760710325933304918452142858879806106214845499670718704532018129553348815327362843246706518826311676917538452317818631484884032929252959289913274829848084561421467966320595980172006456003183536232790787521924655750157145207798486087511869939940023266736153366338179116840490184005332351004990854691988404031259910319601383696749511809898297656135548118786342107367065232798999979296280467063561892962526945512167505847049907450058650930480352253243357594344686769208712964458923557777584158831146374282687397585726706489164423632\",\"DELTA\":\"93540839493959971552865423901789226093328763011922445919928571946113703515842729132879472109395228387208764738970926484618949870591214627692618668077375153559192701474693025462226656116549337248146652482501255820930607033869432220667968682424554711616471973627651716863421554516577716366331699848682958681216261888139409101603059124344125075525791543312721162515584942523419876134808829569829529457617639955678189490257208141837196965948342373022812790844435050648360150869293836349223060722858500537182872294143846213258360218898475766641125493477502149553491502593654061863323857297998048614447925371606038801933864960337435890254277043261512846682042139570000962051463878026338583242360548041329046695667868842400\",\"3\":\"1227675452527605924725300993571504188580051470857656204064614533296779844072852823820754766175236321050062349182891221840452517985644028521499240739391613871973822807731772613052644168369405390658793869751915172749739844553410726807277698347769400977274750672880389943392076308065414059539317340070691852044062594715307024113666759844200606183662256825096857658837519571386467051003466014468855293015652584667669998830524947537781865745830650392641812221679438090257444660715937570193098993118585554478799821072396238689063767016402460690760792908977364175126682041704095200572282644311025594681667826054722587271200221036938804846621444065128275082392082327596239358623150786484106872933657139420542280145197712634108\",\"2\":\"596248147592834822582469335300585333722415132713749620075902332764163096347819006925876158892694742461036531935093982309708492066217459300117157420442081698140277277546563570823996272914068575482008392971932777453900260626542725308060927710122631763045025742980634216666560934260634907599194353151523256914796667535940073668465664206971169038864484235442207811974981191879443614478897291543702607764944403808380921189291059195014621592027660463072969363556421687131446107696579365265893962197300447027501604372738056016734644378437907931412654753728514905671605635291285742886484416973884856055084605172305967034292646171874483670469193852404511746786039743401185954843446037600121496137915619789351744485264614840070\"},\"mj\":\"1117601261519431120446925325460734824239475567013636538481947258329666056692767097795046086413732472111811628751812987521644198549167671875326968410921589186689138994171774838662\",\"alpha\":\"76727612740067576380015106087224381023260815407331375101920043509817863645705120013304683427627332447210083684516403565749916480947649443674885388155460323163682547865307733144184097845709556309570345707127872162476432029772452433292049079349274445907295491125915363620615679995457134810061392296263970553630102299601689685622244925494554558218277670233361938142224820526392365740420502452466959099546877778248089664282581792213376636587293479012783947088070052463503335266180110771978445892744225891676396288437005847308189508347446490710626231658457908472341606549292437553353163031111068977301305043175839949352742711874426231072729977019365761072816602400121302646283352164756787266537474728685656685493249314400351742964904006326192403855909148605656818024621453179832395687665671245528217931951331393482249182516107670379946496778373\",\"t\":{\"1\":\"37203689290881948278188715497642400459048942241931994079434400288578680362970117779048886269388440270597283202033458042171954610700745461571112086648991639439510380585728148682202768590972068041537531136529323260832899360551065706810590032715173070285762675403853992183366951113799098912676809373169763887110420539387555392787590966452796271491986622992160642135480293110112269570862265489120557014181468118619500321000966443141863893743211690388599242584469856365803370202569641902205925191670838354052104480074127555862332399641076324738839120815544432811566503174551735326387678621283249883091766325861497740614317\",\"3\":\"58486787977689017034592833190899828017343431922483563651969628402499947729293364026001243898136737211851089198526360764391403150763769829047179796728616126204105160762333590343947446892105646111520243793053992399512412375936746396187319527051818920531870855183738837254656664620975569939859368862778444291640228229744805843388153451336792379036403300211151424879060241580540910888241769468335914016289938374111481091198264912969768783884602931940994543804730631920434719776196148182987249363641941951160704928605829395517074202388967815738516252602903999010405305463910751219873354588685197134114358234107748126140977\",\"0\":\"60771874648036182010335841594233428920565254732600738082343398028553347795361460295011584446745121430144172025428394361648540904134739046923992231536160801306934272250969829886396340824213814702904457884984387666505055153957942221822193548673145705543973635530652570436109428474727638128773540793530691399549837156239786231362112148914687724325416768262058486101761972044802628459748878200584371058300150212485731451700436345975266860685549673168984700174294811561393162860595319582236734968601457003780816977537443267217411297266600994916897237305128142313335280264655603445636393371224354539882875937093696844430903\",\"DELTA\":\"32816484171372208266594641116109072545171919234551585018140151846920408763078147655907777031259225522515086979967895258126318315788662577171150780535509410112003001556402222994276811926864642497249250763185467678044678144507739529818566125668667424447792097244624010084189629269472698722402896445274092470014229247479740671263651727480322483037149584904549203417226525624083290572692241241259382947122018271686649224741832992966652878170311798126004447080305528487720923103595513611363001766063956060990267107048028416069435287244770875463867263571308182619338433913487209319707428378896314619624990311543563016697299\",\"2\":\"36428320569485697540634597755814766104888687488985202673924762266313135133244610404742081973550848160712054769198012193456278135847215508952327879544434490828380496286187725750283788811367824465072001959950807751252194618152990469069074061195618692339915840384087350671392595652921761835083158086795163935060896053332506433434451836095710383871272788002621913967538399141417857031787255744141437237474972197102809365346359345477248611632307159641948507043668113827177494748159094045928919209335044052792843664865311991178972383241855607627188111601119780878072683890170539599447876998109080150992209773901144245398001\"},\"predicate\":{\"attr_name\":\"age\",\"p_type\":\"GE\",\"value\":18}}]},\"non_revoc_proof\":null}},\"aggregated_proof\":{\"c_hash\":\"33103550379681684069592829341967479618752165928802550870585275205292715916069\",\"c_list\":[[1,121,77,5,144,154,14,192,190,190,145,180,128,71,22,60,168,20,46,163,139,194,71,165,220,188,121,76,25,146,231,114,65,54,69,68,19,200,250,192,47,123,157,132,74,50,28,69,226,195,243,118,45,63,237,197,216,202,206,101,33,56,225,200,128,3,89,12,182,38,113,221,165,119,228,201,156,201,172,136,59,64,51,72,164,198,49,228,223,117,80,64,166,226,37,8,29,146,186,80,210,119,76,252,4,255,62,218,112,163,164,147,247,190,108,76,140,191,76,217,214,184,152,179,193,149,15,70,197,46,90,60,255,247,197,219,252,73,76,0,125,104,114,22,182,161,110,36,162,103,27,42,88,18,161,237,198,43,177,189,181,86,135,207,71,114,0,26,175,12,199,125,25,124,178,87,36,208,251,15,191,127,202,148,152,43,142,92,191,7,89,153,130,195,223,248,176,109,97,164,126,162,181,124,237,130,155,197,66,59,40,197,72,84,32,100,64,55,227,60,214,143,200,200,89,115,236,172,145,56,100,73,20,242,233,95,130,58,112,153,120,115,119,42,199,30,205,88,223,42,196,184,41,19,100,19,244],[1,225,103,238,42,147,91,191,110,69,154,53,57,156,124,43,174,155,76,202,193,98,128,38,207,126,66,70,161,96,109,127,174,44,203,198,177,238,118,117,89,227,170,155,44,251,35,119,219,29,100,173,26,144,95,50,177,4,40,234,117,174,210,192,172,57,160,198,42,199,212,243,240,114,59,91,207,68,57,38,198,2,73,18,16,209,182,145,206,71,17,69,222,49,36,120,72,117,169,107,238,208,235,216,24,183,201,81,15,83,242,45,136,184,166,26,142,136,228,58,229,235,88,169,238,134,205,96,85,9,122,53,147,100,183,114,92,54,125,178,125,75,127,116,50,88,109,152,22,4,121,252,190,18,190,130,143,138,59,231,38,131,176,54,19,194,218,67,144,122,91,43,86,73,233,48,193,30,183,183,191,238,216,167,101,28,185,43,118,64,242,16,62,239,177,27,109,144,67,221,175,202,4,92,130,74,24,20,151,15,227,225,142,71,145,46,192,248,87,57,183,142,253,52,20,56,153,220,234,25,67,116,225,179,211,116,161,37,64,34,48,155,1,1,159,157,37,31,202,19,229,152,23,138,183,126,55],[1,38,181,193,191,72,2,239,34,83,49,36,179,160,82,112,172,98,255,63,60,22,177,249,67,215,220,198,181,7,49,254,133,243,221,214,47,64,229,82,11,94,175,57,86,152,229,192,184,96,136,116,226,123,128,217,23,244,19,204,36,44,123,208,88,24,217,120,145,139,25,233,227,5,119,90,47,147,1,115,92,39,119,194,167,17,229,39,163,167,237,14,116,234,106,252,216,54,33,233,21,54,183,130,144,161,177,142,177,240,51,73,21,202,188,103,244,153,204,219,123,231,139,135,189,155,143,28,4,180,44,148,0,27,103,26,13,203,31,32,166,67,84,87,23,72,234,236,20,1,84,70,86,76,192,164,235,124,86,128,78,230,119,155,95,121,125,20,244,181,121,250,169,9,67,85,213,177,139,111,187,183,114,165,249,177,161,181,175,46,226,66,86,84,124,86,69,143,217,158,161,30,107,133,44,239,89,209,24,150,1,238,122,144,138,179,121,114,90,13,212,209,60,126,37,62,177,180,131,222,168,2,201,156,169,220,224,53,8,203,220,215,163,104,195,184,73,35,241,182,177,80,41,253,230,90,173],[1,32,145,96,219,241,190,19,195,129,219,50,148,152,107,12,189,225,103,171,149,252,193,243,136,132,195,44,19,20,247,140,160,91,230,78,31,242,85,213,65,185,1,91,12,69,118,80,26,135,102,131,4,108,130,230,83,91,176,249,196,56,128,127,82,72,106,49,211,94,133,40,86,72,42,187,199,216,191,223,208,206,121,118,15,167,255,228,57,206,158,217,64,205,212,178,8,248,129,183,221,98,70,54,37,55,47,81,120,59,186,238,165,0,70,173,137,193,232,180,125,211,237,182,249,191,173,107,129,164,148,231,116,225,66,66,71,156,39,248,164,253,234,140,205,177,140,117,47,21,15,242,31,113,118,91,143,89,213,86,143,135,21,46,35,199,214,107,111,65,65,19,26,171,130,16,19,102,145,210,210,61,51,169,148,169,118,182,106,107,253,100,214,232,52,103,180,96,249,254,71,6,11,119,48,129,213,223,205,93,20,117,26,187,32,151,212,137,203,17,237,208,150,72,23,225,235,122,188,34,105,115,0,160,168,251,191,22,242,238,207,74,142,154,66,94,149,191,215,194,134,6,165,244,167,233,241],[1,207,77,250,146,127,242,229,44,172,182,201,183,242,32,242,182,129,233,10,8,180,23,191,163,21,238,158,5,27,216,146,253,173,127,99,95,168,209,132,242,196,242,34,25,25,249,211,51,236,164,153,175,61,65,150,82,251,174,102,186,47,195,82,44,90,252,184,74,89,251,177,254,108,151,136,230,220,93,224,173,247,244,116,132,59,170,215,194,30,87,84,166,147,57,156,201,207,132,203,222,191,253,15,19,228,173,81,156,4,51,121,227,159,50,18,148,129,205,42,42,227,252,138,62,176,115,227,253,52,125,110,178,167,132,244,14,116,195,194,172,44,45,63,38,121,215,136,68,230,21,108,133,159,197,179,94,78,233,107,236,114,92,165,248,22,124,161,23,142,236,224,175,233,134,25,97,150,131,61,220,203,104,154,199,247,146,47,205,56,209,0,133,132,18,103,136,8,202,37,29,100,105,12,232,74,33,6,255,202,96,170,52,229,244,4,235,2,201,125,86,168,179,224,130,81,54,221,185,184,187,141,0,114,98,38,70,225,228,60,157,53,210,238,60,216,215,154,48,73,3,157,192,245,81,170,49],[1,3,244,229,158,71,18,146,198,202,27,2,231,37,13,145,243,84,112,220,61,174,4,175,104,200,64,146,193,20,174,126,42,157,168,76,165,21,50,216,82,211,180,73,244,54,227,200,19,157,25,228,81,37,64,201,19,138,175,50,246,169,11,45,74,194,131,236,127,177,41,242,130,55,112,182,98,22,99,48,153,83,161,250,65,89,3,97,6,5,171,54,223,87,98,103,23,200,212,177,140,155,151,252,125,45,176,55,92,41,56,2,252,32,149,60,3,168,209,193,23,168,230,182,72,193,230,224,5,15,58,63,93,196,33,93,76,188,30,70,31,136,64,204,223,2,230,210,243,255,135,193,52,132,248,160,22,18,164,71,77,80,112,229,120,116,210,225,2,19,139,35,0,214,5,246,9,106,136,204,0,148,97,21,222,153,57,177,162,11,243,252,7,242,34,239,245,50,104,74,221,92,73,13,142,10,184,250,246,167,240,46,230,86,207,181,12,133,81,119,143,164,88,114,223,243,179,208,175,84,161,27,11,225,36,37,177,112,85,81,184,163,223,159,36,9,247,20,13,230,215,108,117,35,99,117,211]]}},\"requested_proof\":{\"revealed_attrs\":{\"attr1_referent\":[\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\",\"Alex\",\"1139481716457488690172217916278103335\"]},\"unrevealed_attrs\":{},\"self_attested_attrs\":{},\"predicates\":{\"predicate1_referent\":\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\"}},\"identifiers\":{\"claim::277478db-bf57-42c3-8530-b1b13cfe0bfd\":{\"schema_key\":%@,\"revoc_reg_seq_no\":null,\"issuer_did\":\"did\"}}}", [[AnoncredsUtils sharedInstance] getGvtSchemaKey]];

    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProof:proofReqJson
                                                     proofJson:proofJson
                                                   schemasJson:schemasJson
                                                 claimDefsJson:claimDefsJson
                                                 revocRegsJson:revocRegsJsons
                                                      outValid:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::verifierVerifyProof returned wrong code");
}


@end
