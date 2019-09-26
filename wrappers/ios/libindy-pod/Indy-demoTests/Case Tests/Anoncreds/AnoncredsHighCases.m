#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"
#import "WalletUtils.h"
#import "AnoncredsUtils.h"
#import "NSDictionary+JSON.h"
#import "NSString+Validation.h"
#import "NSArray+JSON.h"

@interface AnoncredsHignCases : XCTestCase

@end

@implementation AnoncredsHignCases {
    NSError *ret;
}

- (void)setUp {
    [super setUp];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");
    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [super tearDown];
}

// MARK: - Issuer create and store credential def

- (void)testIssuerCreateAndStoreCredentialDefWorks {
    // 1. init commmon wallet
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
}

// MARK: - Prover create master secret

- (void)testProverCreateMasterSecretWorks {
    // 1. init commmon wallet
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long) ret.code);
}

// MARK: - Prover create credential request
- (void)testProverCreateCredentialRequestWorks {
    // 1. init commmon wallet
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed with code:%ld", (long) ret.code);
}

// MARK: - Issuer create credential

- (void)testIssuerCreateCredentialWorks {
    // 1. init commmon wallet
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
}

// MARK: - Prover store credential

- (void)testProverStoreCredentialWorks {
    // 1. init commmon wallet
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:nil
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
}

// MARK: - Prover get credential

- (void)testProverGetCredentialWorks {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credential
    NSString *credentialJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialWithId:[[AnoncredsUtils sharedInstance] credentialId1]
                                                        walletHandle:walletHandle
                                                      credentialJson:&credentialJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForWalletHandle failed");

    NSDictionary *credential = [NSDictionary fromString:credentialJson];
    XCTAssertTrue([[[AnoncredsUtils sharedInstance] getGvtSchemaId] isEqualToString:credential[@"schema_id"]]);
    XCTAssertTrue([[[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId] isEqualToString:credential[@"cred_def_id"]]);
}

// MARK: - Prover get credentials

- (void)testProverGetCredentialsWorksForEmptyFilter {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForFilter:@"{}"
                                                            walletHandle:walletHandle
                                                          credentilsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForWalletHandle failed");

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 3, @"credentials count != 1");
}

- (void)testProverGetCredentialsWorksForFilterByIssuerDid {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *credentialsJson;
    NSString *filter = [NSString stringWithFormat:@"{\"issuer_did\":\"%@\"}", [TestUtils issuer2Did]];
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForFilter:filter
                                                            walletHandle:walletHandle
                                                          credentilsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForWalletHandle failed");

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 1, @"credentials count != 1");
}

- (void)testProverGetCredentialsWorksForEmptyResult {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForFilter:@"{\"issuer_did\":\"didissuer\"}"
                                                            walletHandle:walletHandle
                                                          credentilsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForWalletHandle failed");

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 0, @"credentials count != 0");
}

// MARK: - Prover credentials search

- (void)testProverCredentialSearchWorks {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    IndyHandle searchHandle;
    NSNumber *totalCount;
    ret = [[AnoncredsUtils sharedInstance] proverSearchCredentialsForQuery:@"{}"
                                                               walletHandle:walletHandle
                                                               searchHandle:&searchHandle
                                                                 totalCount:&totalCount];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverSearchCredentialsForQuery failed");

    XCTAssertEqual([totalCount intValue], 3, @"credentials count != 3");

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverFetchCredentialsWithSearchHandle:searchHandle
                                                                            count:totalCount
                                                                   credentilsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverFetchCredentialsWithSearchHandle failed");

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 3, @"credentials count != 3");

    ret = [[AnoncredsUtils sharedInstance] proverCloseCredentialsSearchWithHandle:searchHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCloseCredentialsSearchWithHandle failed");
}

// MARK: - Prover get credentials for proof request

- (void)testProverGetCredentialsForProofReqWorksForRevealedAttr {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    }
            },
            @"requested_predicates": @{}
    }];

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForProofReqWithWalletHandle returned wrong code");

    // 3. check credentials
    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertEqual([[credentials[@"attrs"] allValues] count], 1, @"attrs length != 1");
    XCTAssertEqual([[credentials[@"predicates"] allValues] count], 0, @"predicates length != 0");
    XCTAssertEqual([credentials[@"attrs"][@"attr1_referent"] count], 2, @"attr1_referent length != 2");
}

- (void)testProverGetCredentialsForProofReqWorksForSatisfyPredicate {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{},
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"name": @"age",
                            @"p_type": @">=",
                            @"p_value": @(18)
                    }
            }
    }];

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForProofReqWithWalletHandle returned wrong code");

    // 3. check credentials
    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertEqual([[credentials[@"attrs"] allValues] count], 0, @"attrs length != 1");
    XCTAssertEqual([[credentials[@"predicates"] allValues] count], 1, @"predicates length != 0");
    XCTAssertEqual([credentials[@"predicates"][@"predicate1_referent"] count], 2, @"predicate1_referent length != 2");
}

- (void)testProverGetCredentialsForProofReqWorksForMultiplyAttributeAndPredicates {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials
    NSString *proofRequest = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    },
                    @"attr2_referent": @{
                            @"name": @"sex"
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"name": @"age",
                            @"p_type": @">=",
                            @"p_value": @(18)
                    },
                    @"predicate2_referent": @{
                            @"name": @"height",
                            @"p_type": @">=",
                            @"p_value": @(160)
                    }
            }
    }];

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForProofReqWithWalletHandle returned wrong code");

    // 3. check credentials
    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    XCTAssertEqual([[credentials[@"attrs"] allValues] count], 2, @"attrs length != 2");
    XCTAssertEqual([[credentials[@"predicates"] allValues] count], 2, @"predicates length != 2");
    XCTAssertEqual([credentials[@"attrs"][@"attr1_referent"] count], 2, @"attr1_referent length != 2");
    XCTAssertEqual([credentials[@"attrs"][@"attr2_referent"] count], 2, @"attr2_referent length != 2");
    XCTAssertEqual([credentials[@"predicates"][@"predicate1_referent"] count], 2, @"predicate1_referent length != 2");
    XCTAssertEqual([credentials[@"predicates"][@"predicate2_referent"] count], 2, @"predicate2_referent length != 2");
}

// MARK: - Prover search credentials for proof request

- (void)testProverSearchCredentialsForProofReqWorks {
    IndyHandle walletHandle = 0;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:nil
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. get credentials for proof req
    // 2. get credentials
    NSString *proofRequest = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    }
            },
            @"requested_predicates": @{}
    }];

    IndyHandle searchHandle;
    ret = [[AnoncredsUtils sharedInstance] proverSearchCredentialsForProofRequest:proofRequest
                                                                   extraQueryJson:nil
                                                                     walletHandle:walletHandle
                                                                     searchHandle:&searchHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverSearchCredentialsForProofRequest failed");

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverFetchCredentialsForProofReqItemReferent:@"attr1_referent"
                                                                            searchHandle:searchHandle
                                                                                   count:@(3)
                                                                          credentilsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverFetchCredentialsForProofReqItemReferent failed");

    NSDictionary *credentialsDict = [NSDictionary fromString:credentialsJson];
    NSArray *credentials = (NSArray *) credentialsDict;

    XCTAssertEqual([credentials count], 2, @"credentials count != 2");

    ret = [[AnoncredsUtils sharedInstance] proverCloseCredentialsSearchForProofReqWithHandle:searchHandle];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCloseCredentialsSearchForProofReqWithHandle failed");
}

// MARK: - Prover create proof works

- (void)testProverCreateProofWorks {
    IndyHandle walletHandle = 0;
    NSString *credentialDefJson;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:&credentialDefJson
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");
    XCTAssertTrue([credentialDefJson isValid], @"invalid credentialDefJson: %@", credentialDefJson);

    // 2. get credentials for proof request
    NSString *proofRequest = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"name": @"age",
                            @"p_type": @">=",
                            @"p_value": @(18)
                    }
            }
    }];

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofRequest
                                                              walletHandle:walletHandle
                                                           credentialsJson:&credentialsJson];

    NSString *requestedCredentialsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"self_attested_attributes": @{},
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"cred_id": [[AnoncredsUtils sharedInstance] credentialId1],
                            @"revealed": @(YES)
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"cred_id": [[AnoncredsUtils sharedInstance] credentialId1]
                    }
            }
    }];

    NSString *schemasJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getGvtSchemaId], [[AnoncredsUtils sharedInstance] getGvtSchemaJson]];
    NSString *credentialDefsJson = [NSString stringWithFormat:@"{\"%@\":%@}", [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId], credentialDefJson];
    NSString *revocStatesJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofRequest
                                              requestedCredentialsJSON:requestedCredentialsJson
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:walletHandle
                                                             proofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProofWithWalletHandle failed");
    XCTAssertTrue([proofJson isValid], @"invalid proofJson: %@", proofJson);
}

// MARK: - Verifier verify proof
- (void)testVerifierVerifyProofWorksForCorrectProof {
    // 2. verify proof
    NSString *proofRequest = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"name": @"age",
                            @"p_type": @">=",
                            @"p_value": @(18)
                    }
            }
    }];

    NSString *schemasJson = [[AnoncredsUtils sharedInstance] toJson:@{
            [[AnoncredsUtils sharedInstance] getGvtSchemaId]: [NSDictionary fromString:[[AnoncredsUtils sharedInstance] getGvtSchemaJson]]
    }];
    NSString *credentialDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId]: [NSDictionary fromString:[[AnoncredsUtils sharedInstance] gvtCredDef]]
    }];

    NSString *revocRegDefsJSON = @"{}";
    NSString *revocRegsJson = @"{}";

    NSString *proofJson = [[AnoncredsUtils sharedInstance] proofJSON];

    BOOL isValid = false;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofRequest
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJSON
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValid];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof failed");
    XCTAssertTrue(isValid, @"isValid is false");
}

- (void)testVerifierVerifyProofWorksForProofDoesNotCorrespondToRequest {
    IndyHandle walletHandle = 0;
    NSString *credentialDefJson;

    // 1. get wallet handle
    ret = [[AnoncredsUtils sharedInstance] initializeCommonWalletAndReturnHandle:&walletHandle
                                                               credentialDefJson:&credentialDefJson
                                                             credentialOfferJson:nil
                                                               credentialReqJson:nil
                                                                  credentialJson:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::initializeCommonWalletAndReturnHandle failed");

    // 2. verify proof
    NSString *proofRequest = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"sex"
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"name": @"height",
                            @"p_type": @">=",
                            @"p_value": @(180)
                    }
            }
    }];
    NSString *schemasJson = [[AnoncredsUtils sharedInstance] toJson:@{
            [[AnoncredsUtils sharedInstance] getGvtSchemaId]: [NSDictionary fromString:[[AnoncredsUtils sharedInstance] getGvtSchemaJson]]
    }];
    NSString *credentialDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId]: [NSDictionary fromString:[[AnoncredsUtils sharedInstance] gvtCredDef]]
    }];

    NSString *revocRegDefsJSON = @"{}";
    NSString *revocRegsJson = @"{}";

    NSString *proofJson = [[AnoncredsUtils sharedInstance] proofJSON];
    BOOL isValid = false;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofRequest
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJSON
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValid];
    XCTAssertEqual(ret.code, AnoncredsProofRejected, @"AnoncredsUtils::verifierVerifyProof returned wrong error");
}

// MARK: - Issuer rotate credential def

- (void)testIssuerRotateCredentialDef {

    IndyHandle localWalletHandle = 0;

    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&localWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    NSString * credDefId = [[AnoncredsUtils sharedInstance] getIssuer1GvtCredDefId];
    
    ret = [[AnoncredsUtils sharedInstance] issuerRotateCredentialDefStartForId:credDefId
                                                                    configJSON:nil
                                                                  walletHandle:localWalletHandle
                                                                   credDefJson:nil];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"AnoncredsUtils::issuerRotateCredentialDefStartForId() returned wrong code!");

    ret = [[AnoncredsUtils sharedInstance] issuerRotateCredentialDefApplyForId:credDefId
                                                                  walletHandle:localWalletHandle];
    XCTAssertEqual(ret.code, WalletItemNotFound, @"AnoncredsUtils::issuerRotateCredentialDefApplyForId() returned wrong code!");

    [[WalletUtils sharedInstance] closeWalletWithHandle:localWalletHandle];
}

// MARK: - To Unqualified

- (void)testToUnqualified {
    NSString *qualified = @"did:sov:NcYxiDXkpYi6ov5FcYDi1e";
    NSString *unqualified = @"NcYxiDXkpYi6ov5FcYDi1e";

    NSString* res;;

    ret = [[AnoncredsUtils sharedInstance] toUnqualified:qualified
                                                     res:&res];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::toUnqualified failed");
    XCTAssertTrue([unqualified isEqualToString:res]);

    ret = [[AnoncredsUtils sharedInstance] toUnqualified:unqualified
                                                     res:&res];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::toUnqualified failed");
    XCTAssertTrue([unqualified isEqualToString:res]);
}


@end
