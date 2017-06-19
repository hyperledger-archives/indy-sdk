//
//  AnoncredsHighCase.m
//  libsovrin-demo
//
//  Created by Anastasia Tarasova on 16.06.17.
//  Copyright © 2017 Kirill Neznamov. All rights reserved.
//

#import <XCTest/XCTest.h>
#import <libsovrin/libsovrin.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"
#import "NSString+Validation.h"
#import "NSArray+JSON.h"

@interface AnoncredsHignCases : XCTestCase

@end

@implementation AnoncredsHignCases

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

// MARK: - Issuer create and store claim def

- (void)testIssuerCreateAndStoreClaimDefWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. init commmon wallet
    SovrinHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. issuer create claim definition
    NSString *schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)];
    NSString *claimDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:walletHandle
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:NO
                                                                          claimDefJson:&claimDefJson
                                                                          claimDefUUID:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@", claimDefJson);
    
    // 3. Check claim definition
    NSDictionary *claimDef = [NSDictionary fromString:claimDefJson];
    XCTAssertEqual([[claimDef[@"public_key"][@"r"] allKeys] count], 4, @"wrong length:claimDef[publicKey][r]");
    XCTAssertTrue([claimDef[@"public_key"][@"n"] length] > 0, @"wrong length:claimDef[publicKey][n]");
    XCTAssertTrue([claimDef[@"public_key"][@"s"] length] > 0, @"wrong length:claimDef[publicKey][s]");
    XCTAssertTrue([claimDef[@"public_key"][@"rms"] length] > 0, @"wrong length:claimDef[publicKey][rms]");
    XCTAssertTrue([claimDef[@"public_key"][@"z"] length] > 0, @"wrong length:claimDef[publicKey][z]");
    XCTAssertTrue([claimDef[@"public_key"][@"rctxt"] length] > 0, @"wrong length:claimDef[publicKey][rctxt]");
    
    [TestUtils cleanupStorage];
}

- (void)testIssuerCreateAndStoreClaimDefWorksForInvalidWallet
{
    [TestUtils cleanupStorage];
    NSError *ret;
    
    // 1. init commmon wallet
    SovrinHandle walletHandle = 0;
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. Create claim definition
    NSString *schema = [[AnoncredsUtils sharedInstance] getGvtSchemaJson:@(1)];
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimDefinifionWithWalletHandle:invalidWalletHandle
                                                                            schemaJson:schema
                                                                         signatureType:nil
                                                                        createNonRevoc:NO
                                                                          claimDefJson:nil
                                                                          claimDefUUID:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::issuerCreateClaimDefinifionWithWalletHandle failed: returned wrong error code");
    
    [TestUtils cleanupStorage];
}

// MARK: - Prover store claim offer

- (void)testProverStoreClaimOfferWorks
{
   [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offer json
    NSString *claimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:@"NcYxiDXkpYi6ov5FcYDi1e"
                                                                            seqNo:@(1)
                                                                       schemaSeqNo:@(1)];
    
    // 3. Store claim offer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:walletHandle
                                                  claimOfferJson:claimOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimOffer failed");
    
    [TestUtils cleanupStorage];
}

- (void)testProverStoreClaimOfferWorksForInvalidJson
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offer json
    NSString *claimOfferJson = @"{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}";
    
    // 3. Store claim offer
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:walletHandle
                                                  claimOfferJson:claimOfferJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverStoreClaimOffer failed");
    
    [TestUtils cleanupStorage];
}

- (void)testProverStoreClaimOfferWorksForInvalidWallet
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offer json
    NSString *claimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:@"NcYxiDXkpYi6ov5FcYDi1e"
                                                                            seqNo:@(1)
                                                                      schemaSeqNo:@(1)];
    
    // 3. Store claim offer
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimOffer:invalidWalletHandle
                                                  claimOfferJson:claimOfferJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverStoreClaimOffer failed");
    
    [TestUtils cleanupStorage];
}

// MARK: - Prover get claim offers
- (void)testProverGetClaimOffersWorksForEmptyFilter
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offers
    NSString *claimOffersJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle
                                                     filterJson:@"{}"
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);
    
    // 3. check obtained offers
    NSDictionary *offers = [NSDictionary fromString:claimOffersJson];
    NSArray *array = (NSArray *)offers;
    XCTAssertEqual([array count], 3, @"wrong length of claim offers");
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimOffersWorksForFilterByIssuer
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offers
    // TODO: 109 error
    NSString *filter = @"{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\"}";
    NSString *claimOffersJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle
                                                     filterJson:filter
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);
    
    // 3. Check offers
    NSDictionary *offers = [NSDictionary fromString:claimOffersJson];
    NSArray *array = (NSArray *)offers;
    XCTAssertEqual([array count], 2, @"wrong length of claim offers");
    
    NSMutableDictionary *offer1 = [NSMutableDictionary new];
    offer1[@"issuer_did"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    offer1[@"claim_def_seq_no"] = @(1);
    offer1[@"schema_seq_no"] = @(1);
    
    NSMutableDictionary *offer2 = [NSMutableDictionary new];
    offer2[@"issuer_did"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    offer2[@"claim_def_seq_no"] = @(2);
    offer2[@"schema_seq_no"] = @(2);
    
    XCTAssertTrue([array contains:offer1], @"offers doesn't contain offer1");
    XCTAssertTrue([array contains:offer2], @"offers doesn't contain offer2");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimOffersWorksForFilterByClaimDef
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offers
    NSString *filter = @"{\"claim_def_seq_no\":2}";
    NSString *claimOffersJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle
                                                     filterJson:filter
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);

    // 3. Check offers
    NSDictionary *offers = [NSDictionary fromString:claimOffersJson];
    NSArray *array = (NSArray *)offers;
    XCTAssertEqual([array count], 1, @"wrong length of claim offers");
    
    NSMutableDictionary *offer1 = [NSMutableDictionary new];
    offer1[@"issuer_did"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    offer1[@"claim_def_seq_no"] = @(2);
    offer1[@"schema_seq_no"] = @(2);
    
    XCTAssertTrue([array contains:offer1], @"offers doesn't contain offer1");
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimOffersWorksForFilterBySchema
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offers
    NSString *filter = @"{\"schema_seq_no\":2}";
    NSString *claimOffersJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle
                                                     filterJson:filter
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);
    
    // 3. Check offers
    NSDictionary *offersDict = [NSDictionary fromString:claimOffersJson];
    NSArray *offers = (NSArray *)offersDict;
    XCTAssertEqual([offers count], 2, @"wrong length of claim offers");
    
    NSMutableDictionary *offer1 = [NSMutableDictionary new];
    offer1[@"issuer_did"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    offer1[@"claim_def_seq_no"] = @(2);
    offer1[@"schema_seq_no"] = @(2);
    
    NSMutableDictionary *offer2 = [NSMutableDictionary new];
    offer2[@"issuer_did"] = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    offer2[@"claim_def_seq_no"] = @(3);
    offer2[@"schema_seq_no"] = @(2);
    
    XCTAssertTrue([offers contains:offer1], @"offers doesn't contain offer1");
    XCTAssertTrue([offers contains:offer2], @"offers doesn't contain offer2");
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimOffersWorksForFilterByIssuerAndSchema
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offers
    NSString *filter = @"{\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\",\"schema_seq_no\":1}";
    NSString *claimOffersJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle
                                                     filterJson:filter
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers failed");
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);

    // 3. Check offers
    NSDictionary *offersJson = [NSDictionary fromString:claimOffersJson];
    NSArray *offers = (NSArray *)offersJson;
    XCTAssertEqual([offers count], 1, @"wrong length of claim offers");
    
    NSMutableDictionary *offer1 = [NSMutableDictionary new];
    offer1[@"issuer_did"] = @"NcYxiDXkpYi6ov5FcYDi1e";
    offer1[@"claim_def_seq_no"] = @(1);
    offer1[@"schema_seq_no"] = @(1);
    
    XCTAssertTrue([offers contains:offer1], @"offers doesn't contain offer1");
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimOffersWorksForNoResults
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long)ret.code);
    
    // 2. get claim offers
    NSString *filter = @"{\"claim_def_seq_no\":4}";
    NSString *claimOffersJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:walletHandle
                                                     filterJson:filter
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimOffers failed with code:%ld", (long)ret.code);
    XCTAssertTrue([claimOffersJson isValid], @"invalid claimOffersJson: %@", claimOffersJson);

    // 3. Check offers
    NSDictionary *offersJson = [NSDictionary fromString:claimOffersJson];
    NSArray *offers = (NSArray *)offersJson;
    XCTAssertEqual([offers count], 0, @"wrong length of claim offers");
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimOffersWorksForinvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long)ret.code);
    
    // 2. get claim offers
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    NSString *filter = @"{\"claim_def_seq_no\":1}";
    NSString *claimOffersJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimOffers:invalidWalletHandle
                                                     filterJson:filter
                                             outClaimOffersJSON:&claimOffersJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverGetClaimOffers returned not WalletInvalidHandle code:%ld", (long)ret.code);
    [TestUtils cleanupStorage];
}

// MARK: - Prover create master secret

- (void)testProverCreateMasterSecretWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long)ret.code);
    
    // 2. create master secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:walletHandle
                                                   masterSecretName:@"master_secret_name1"];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret failed with code:%ld", (long)ret.code);
    [TestUtils cleanupStorage];
}

- (void)testProverCreateMasterSecretWorksInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long)ret.code);

    // 2. create master secret
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:invalidWalletHandle
                                                   masterSecretName:@"master_secret_name2"];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverCreateMasterSecret returned not WalletInvalidHandle code:%ld", (long)ret.code);
    [TestUtils cleanupStorage];
}

// MARK: - Prover create and store claim request
- (void)testProverCreateAndStoreRequestWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    NSString *claimDef;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDef];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long)ret.code);
    
    // 2. get claim offer
    NSString *issuerDid = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSNumber *claimDefSeqNo = @(1);
    NSString *claimOffer = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                        seqNo:claimDefSeqNo
                                                                  schemaSeqNo:@(1)];
    
    // 3. get claim request
    NSString *proverDid = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString *claimRequestJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:walletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:claimDef
                                                       masterSecretName:@"common_master_secret_name"
                                                        outClaimReqJson:&claimRequestJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed with code:%ld", (long)ret.code);
    XCTAssertTrue([claimRequestJson isValid], @"invalid claimRequestJson: %@", claimRequestJson);
    
    // 4. check claim request
    NSDictionary *claimRequest = [NSDictionary fromString:claimRequestJson];
    XCTAssertTrue(claimRequest[@"claim_def_seq_no"] == claimDefSeqNo, @"claimRequest[@\"seqNo\"] != claimDefSeqNo");
    XCTAssertTrue([claimRequest[@"issuer_did"] isEqualToString:issuerDid], @"[claimRequest[@\"issuer_did\"] != issuerDid");
    XCTAssertTrue([claimRequest[@"claim_request"][@"u"] length] > 0, @"claimRequest[@\"claim_request\"][@\"u\"] length <= 0");
}

- (void)testProverCreateAndStoreClaimReqWorksForInvalidWallet
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    NSString *claimDef;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDef];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claim offer
    NSString *issuerDid = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *claimOffer = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                        seqNo:@(1)
                                                                  schemaSeqNo:@(1)];
    
    // 3. create and store claim requets
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    NSString *proverDid = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString *claimRequestJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:invalidWalletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:claimDef
                                                       masterSecretName:@"common_master_secret_name"
                                                        outClaimReqJson:&claimRequestJson];
     XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed");
    [TestUtils cleanupStorage];
}

// TODO: This test is ignored in rust
- (void)testProverCreateAndStoreClaimReqWorksForClaimDefDoesNotCorrespondOfferDiffrentClaimDefSeqNo
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    NSString *claimDef;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDef];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDef isValid], @"invalid claimDef: %@", claimDef);
    
    // 2. get claim offer
    NSString *issuerDid = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *claimOffer = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                        seqNo:@(2)
                                                                  schemaSeqNo:@(1)];

    // 3. create and store claim requets
    NSString *proverDid = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString *claimRequestJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:walletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:claimDef
                                                       masterSecretName:@"common_master_secret_name"
                                                        outClaimReqJson:&claimRequestJson];
    // TODO: Returns 0, not 110
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateAndStoreClaimReq returned wrong error code");
    [TestUtils cleanupStorage];
}

- (void)testProverCreateAndStoreClaimreqWorksForClaimDefDoesNotCorrespondOfferDiffrentSchemaSeqNo
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    NSString *claimDef;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDef];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDef isValid], @"invalid claimDef: %@", claimDef);
    
    
    // 2. get claim offer
    NSString *issuerDid = @"NcYxiDXkpYi6ov5FcYDi1e";
    NSString *claimOffer = [[AnoncredsUtils sharedInstance] getClaimOfferJson:issuerDid
                                                                        seqNo:@(1)
                                                                  schemaSeqNo:@(2)];
    
    // 3. create and store claim requets
    NSString *proverDid = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString *claimRequestJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:walletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOffer
                                                           claimDefJson:claimDef
                                                       masterSecretName:@"common_master_secret_name" outClaimReqJson:&claimRequestJson];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::proverCreateAndStoreClaimReq returned wrong code");
    [TestUtils cleanupStorage];
}

// MARK: - Issuer create claim
- (void)testIssuerCreateClaimWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. create claim
    NSString *claimRequest = @"{"\
                "\"claim_request\":{"\
                "\"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\","\
                "\"u\":\"54172737564529332710724213139048941083013176891644677117322321823630308734620627329227591845094100636256829761959157314784293939045176621327154990908459072821826818718739696323299787928173535529024556540323709578850706993294234966440826690899266872682790228513973999212370574548239877108511283629423807338632435431097339875665075453785141722989098387895970395982432709011505864533727415552566715069675346220752584449560407261446567731711814188836703337365986725429656195275616846543535707364215498980750860746440672050640048215761507774996460985293327604627646056062013419674090094698841792968543317468164175921100038\","\
                "\"ur\":null},"\
                "\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\","\
                "\"claim_def_seq_no\":1}";
    NSString *claimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                                outClaimJson:&claimJson
                                                       outRevocRegUpdateJSON:nil];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([claimJson isValid], @"invalid claimJson: %@", claimJson);
    
    NSDictionary *claim = [NSDictionary fromString:claimJson];
    XCTAssertTrue([claim[@"signature"][@"primary_claim"][@"a"] length] > 0, @"wrong \"a\" length");
    XCTAssertTrue([claim[@"signature"][@"primary_claim"][@"m2"] length] > 0, @"wrong \"m2\" length");
    XCTAssertTrue([claim[@"signature"][@"primary_claim"][@"e"] length] > 0, @"wrong \"e\" length");
    XCTAssertTrue([claim[@"signature"][@"primary_claim"][@"v_prime"] length] > 0, @"wrong \"v_prime\" length");
    [TestUtils cleanupStorage];
}

- (void)testIssuerCreateClaimWorksForClaimDoesNotCorrespondToClaimReq
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. create claim
    NSString *claimRequest = @"{"\
    "\"claim_request\":{"\
    "\"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\","\
    "\"u\":\"78642009183061519681642949186511883517561213024253007693605674585288964920641017651779407190325620073544451273313223865970730324882004218654708785143702626337327148875137393101464687794953218753005927492179012286511197396945795208681795313939767499444933139277315113356530041684437761038663276793040349557294620223093906897574215436647703667891052762523022326049857738264833807472302707972331207200720216038057270470116611478516211732505056236404960175670287081433670657644042478872537481050085523491110773623684416797190117083084618649667528194409150615774512701755156055570554349550169869411668543258825800016015079\","\
    "\"ur\":null},"\
    "\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\","\
    "\"claim_def_seq_no\":1}";
    NSString *claimJson = [[AnoncredsUtils sharedInstance] getXyzClaimJson];
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                                outClaimJson:&claimJson
                                                       outRevocRegUpdateJSON:nil];
    XCTAssertEqual(ret.code, CommonInvalidStructure, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle returned wrong code");

    [TestUtils cleanupStorage];
}

- (void)testIssuerCreateClaimWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. create claim
    NSString *claimRequest = @"{"\
    "\"claim_request\":{"\
    "\"prover_did\":\"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW\","\
    "\"u\":\"54172737564529332710724213139048941083013176891644677117322321823630308734620627329227591845094100636256829761959157314784293939045176621327154990908459072821826818718739696323299787928173535529024556540323709578850706993294234966440826690899266872682790228513973999212370574548239877108511283629423807338632435431097339875665075453785141722989098387895970395982432709011505864533727415552566715069675346220752584449560407261446567731711814188836703337365986725429656195275616846543535707364215498980750860746440672050640048215761507774996460985293327604627646056062013419674090094698841792968543317468164175921100038\","\
    "\"ur\":null},"\
    "\"issuer_did\":\"NcYxiDXkpYi6ov5FcYDi1e\","\
    "\"claim_def_seq_no\":1}";
    NSString *claimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:invalidWalletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                                outClaimJson:&claimJson
                                                       outRevocRegUpdateJSON:nil];

    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle failed");
    [TestUtils cleanupStorage];
}

// MARK: - Prover store claim

- (void)testProverStoreClaimWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    NSString *claimDefJson;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"claimDefJson is wrong:%@", claimDefJson);
    
    // 2. get claim offer json
    NSString *claimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:@"NcYxiDXkpYi6ov5FcYDi1e"
                                                                            seqNo:@(1)
                                                                      schemaSeqNo:@(1)];
    // 3. get claim request
    NSString *proverDid = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString *claimRequest;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:walletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOfferJson
                                                           claimDefJson:claimDefJson
                                                       masterSecretName:@"common_master_secret_name"
                                                        outClaimReqJson:&claimRequest];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed");
    XCTAssertTrue([claimRequest isValid], @"claimRequest is wrong:%@", claimRequest);
    
    // 4. create claim
    NSString *claimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    NSString *xClaimJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                                outClaimJson:&xClaimJson
                                                       outRevocRegUpdateJSON:nil];
    
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([xClaimJson isValid], @"xClaimJson is wrong:%@", xClaimJson);
    
    // 5. store claim
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:walletHandle
                                                                 claimsJson:xClaimJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreClaimWithWalletHandle failed");
    
    [TestUtils cleanupStorage];
}

- (void)testProverStoreClaimWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    NSString *claimDefJson;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"claimDefJson is wrong:%@", claimDefJson);
    
    // 2. get claim offer json
    NSString *claimOfferJson = [[AnoncredsUtils sharedInstance] getClaimOfferJson:@"NcYxiDXkpYi6ov5FcYDi1e"
                                                                            seqNo:@(1)
                                                                      schemaSeqNo:@(1)];
    
    // 3. get claim request
    NSString *proverDid = @"CnEDk9HrMnmiHXEV1WFgbVCRteYnPqsJwrTdcZaNhFVW";
    NSString *claimRequest;
    ret = [[AnoncredsUtils sharedInstance] proverCreateAndStoreClaimReq:walletHandle
                                                              proverDid:proverDid
                                                         claimOfferJson:claimOfferJson
                                                           claimDefJson:claimDefJson
                                                       masterSecretName:@"common_master_secret_name"
                                                        outClaimReqJson:&claimRequest];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreClaimReq failed");
    XCTAssertTrue([claimRequest isValid], @"claimRequest is wrong:%@", claimRequest);
    
    // 4. create claim
    NSString *claimJson = [[AnoncredsUtils sharedInstance] getGvtClaimJson];
    NSString *xClaimJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateClaimWithWalletHandle:walletHandle
                                                                claimReqJson:claimRequest
                                                                   claimJson:claimJson
                                                                outClaimJson:&xClaimJson
                                                       outRevocRegUpdateJSON:nil];
    
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateClaimWithWalletHandle failed");
    XCTAssertTrue([xClaimJson isValid], @"xClaimJson is wrong:%@", xClaimJson);
    
    // 5. store claim
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverStoreClaimWithWalletHandle:invalidWalletHandle
                                                                 claimsJson:xClaimJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverStoreClaimWithWalletHandle failed");

    [TestUtils cleanupStorage];
}

// MARK: - Prover get claims

- (void)testProverGetClaimsWorksForEmptyFilter
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForWalletHandle:walletHandle
                                                               filterJson:@"{}"
                                                            outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"claimsJson is wrong:%@", claimsJson);
    
    NSDictionary *claimsDict = [NSDictionary fromString:claimsJson];
    NSArray *claims = (NSArray *)claimsDict;
    
    XCTAssertEqual([claims count], 1, @"claims count != 1");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsWorksForFilterByClaimDefSeqNo
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get claims
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForWalletHandle:walletHandle
                                                               filterJson:@"{\"claim_def_seq_no\":1}"
                                                            outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"claimsJson is wrong:%@", claimsJson);
    
    NSDictionary *claimsDict = [NSDictionary fromString:claimsJson];
    NSArray *claims = (NSArray *)claimsDict;
    
    XCTAssertEqual([claims count], 1, @"claims count != 1");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsWorksForFilterByClaimDefSeqNoAndSchemaSeqNo
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForWalletHandle:walletHandle
                                                               filterJson:@"{\"claim_def_seq_no\":1, \"schema_seq_no\":1}"
                                                            outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"claimsJson is wrong:%@", claimsJson);
    
    NSDictionary *claimsDict = [NSDictionary fromString:claimsJson];
    NSArray *claims = (NSArray *)claimsDict;
    
    XCTAssertEqual([claims count], 1, @"claims count != 1");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsWorksForEmptyResult
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForWalletHandle:walletHandle
                                                               filterJson:@"{\"claim_def_seq_no\":10}"
                                                            outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"claimsJson is wrong:%@", claimsJson);
    
    NSDictionary *claimsDict = [NSDictionary fromString:claimsJson];
    NSArray *claims = (NSArray *)claimsDict;
    
    XCTAssertEqual([claims count], 0, @"claims count != 0");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *claimsJson;
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForWalletHandle:invalidWalletHandle
                                                               filterJson:@"{}"
                                                            outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverGetClaimsForWalletHandle returned wrong code");
    [TestUtils cleanupStorage];
}

// MARK: - Prover get claims for proof request

- (void)testProverGetClaimsForProofReqWorksForRevealedAttr
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *proofRequest = @"{"\
        "\"nonce\":\"123432421212\","\
        "\"requested_attrs\":{"\
            "\"attr1_uuid\":{"\
                "\"schema_seq_no\":1,"\
                "\"name\":\"name\""\
            "}"\
        "},"\
        "\"requested_predicates\":{}"\
    "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong code");
    
    // 3. check claims
    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertEqual([[claims[@"attrs"] allValues] count], 1, @"attrs length != 1");
    XCTAssertEqual([[claims[@"predicates"] allValues] count], 0, @"predicates length != 0");
    XCTAssertEqual([claims[@"attrs"][@"attr1_uuid"] count], 1, @"attr1_uuid length != 1");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsForProofReqWorksForNotFoundAttribute
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *proofRequest = @"{"\
        "\"nonce\":\"123432421212\","\
        "\"requested_attrs\":{"\
            "\"attr1_uuid\":{"\
            "\"schema_seq_no\":1,"\
            "\"name\":\"some_attr\""\
            "}"\
        "},"\
        "\"requested_predicates\":{}"\
    "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong code");
    
    // 3. check claims
    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertEqual([[claims[@"attrs"] allValues] count], 1, @"attrs length != 1");
    XCTAssertEqual([[claims[@"predicates"] allValues] count], 0, @"predicates length != 0");
    XCTAssertEqual([claims[@"attrs"][@"attr1_uuid"] count], 0, @"attr1_uuid length != 1");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsForProofReqWorksForSatisfyPredicate
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *proofRequest = @"{"\
    "\"nonce\":\"123432421212\","\
    "\"requested_attrs\":{},"\
    "\"requested_predicates\":{"\
        "\"predicate1_uuid\":{"\
            "\"attr_name\":\"age\","\
            "\"p_type\":\"GE\","\
            "\"value\":18}}"\
    "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong code");
    
    // 3. check claims
    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertEqual([[claims[@"attrs"] allValues] count], 0, @"attrs length != 1");
    XCTAssertEqual([[claims[@"predicates"] allValues] count], 1, @"predicates length != 0");
    XCTAssertEqual([claims[@"predicates"][@"predicate1_uuid"] count], 1, @"predicate1_uuid length != 1");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsForProofReqWorksForMultiplyAttributeAndPredicates
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *proofRequest = @"{"\
        "\"nonce\":\"123432421212\","\
        "\"requested_attrs\":{"\
            "\"attr1_uuid\":{"\
                "\"schema_seq_no\":1,"\
                "\"name\":\"name\"},"\
            "\"attr2_uuid\":{"\
                "\"schema_seq_no\":1,"\
                "\"name\":\"sex\"}},"\
        "\"requested_predicates\":{"\
            "\"predicate1_uuid\":{"\
                "\"attr_name\":\"age\","\
                "\"p_type\":\"GE\","\
                "\"value\":18},"\
            "\"predicate2_uuid\":{"\
                "\"attr_name\":\"height\","\
                "\"p_type\":\"GE\","\
                "\"value\":160}"\
            "}"\
    "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong code");
    
    // 3. check claims
    // TODO: Figure out right checks
    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    XCTAssertEqual([[claims[@"attrs"] allValues] count], 2, @"attrs length != 2");
    XCTAssertEqual([[claims[@"predicates"] allValues] count], 2, @"predicates length != 2");
    XCTAssertEqual([claims[@"attrs"][@"attr1_uuid"]count], 1, @"attr1_uuid length != 1");
    XCTAssertEqual([claims[@"attrs"][@"attr2_uuid"] count], 1, @"attr2_uuid length != 1");
    XCTAssertEqual([claims[@"predicates"][@"predicate1_uuid"] count], 1, @"predicate1_uuid length != 1");
    XCTAssertEqual([claims[@"predicates"][@"predicate2_uuid"] count], 1, @"predicate2_uuid length != 1");
    
    [TestUtils cleanupStorage];
}

- (void)testProverGetClaimsForProofReqWorksForInvalidWalletHandle
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    
    // 2. get claims
    NSString *proofRequest = @"{"\
            "\"nonce\":\"123432421212\","\
            "\"requested_attrs\":{},"\
            "\"requested_predicates\":{"\
                "\"predicate1_uuid\":{"\
                    "\"attr_name\":\"age\","\
                    "\"p_type\":\"GE\","\
                    "\"value\":58}"\
            "}"\
        "}";
    SovrinHandle invalidWalletHandle = walletHandle + 1;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:invalidWalletHandle
                                                                     proofRequestJson:proofRequest outClaimsJson:nil];
    XCTAssertEqual(ret.code, WalletInvalidHandle, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle returned wrong error code");
    [TestUtils cleanupStorage];
}

// MARK: - Prover create proof works

- (void)testProverCreateProofWorks
{
    [TestUtils cleanupStorage];
    NSError *ret;
    SovrinHandle walletHandle = 0;
    NSString *claimDefJson;
    
    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                                    claimDefJson:&claimDefJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([claimDefJson isValid], @"invalid claimDefJson: %@", claimDefJson);
    
    // 2. get claims for proof request
    
    NSString *proofRequest = @"{"\
            "\"nonce\":\"123432421212\","\
            "\"requested_attrs\":{"\
                "\"attr1_uuid\":{"\
                    "\"schema_seq_no\":1,"\
                    "\"name\":\"name\"}},"\
            "\"requested_predicates\":{"\
                "\"predicate1_uuid\":{"\
                    "\"attr_name\":\"age\","\
                    "\"p_type\":\"GE\","\
                    "\"value\":18},"\
            "}"\
        "}";
    NSString *claimsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetClaimsForProofReqWithWalletHandle:walletHandle
                                                                     proofRequestJson:proofRequest
                                                                        outClaimsJson:&claimsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetClaimsForProofReqWithWalletHandle failed");
    XCTAssertTrue([claimsJson isValid], @"invalid claimsJson: %@", claimsJson);
    
    NSDictionary *claims = [NSDictionary fromString:claimsJson];
    NSDictionary *claimForAttr = claims[@"attrs"][@"attr1_uuid"][@"claim_uuid"];
    NSDictionary *claimForPredicate = claims[@"predicates"][@"predicate1_uuid"][@"claim_uuid"];
    
    NSString *requestedClaimsJson = [NSString stringWithFormat:@"{"\
                                     "\"self_attested_attributes\":{},"\
                                     "\"requested_attrs\":{"\
                                        "\"attr1_uuid\":[\"%@\",true]},"\
                                    "\"requested_predicates\":{"\
                                        "\"predicate1_uuid\":{\"predicate1_uuid\":\"%@\"}"\
                                     "}", claimForAttr, claimForPredicate];
    
}




@end
