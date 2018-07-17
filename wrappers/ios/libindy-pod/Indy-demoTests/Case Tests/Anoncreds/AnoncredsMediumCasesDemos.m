#import <XCTest/XCTest.h>
#import <Indy/Indy.h>
#import "TestUtils.h"

@interface AnoncredsMediumCasesDemos : XCTestCase

@end

@implementation AnoncredsMediumCasesDemos {
    NSError *ret;
}

- (void)setUp {
    [super setUp];
    [TestUtils cleanupStorage];

    ret = [[PoolUtils sharedInstance] setProtocolVersion:[TestUtils protocolVersion]];
    XCTAssertEqual(ret.code, Success, @"PoolUtils::setProtocolVersion() failed!");

    // Put setup code here. This method is called before the invocation of each test method in the class.
}

- (void)tearDown {
    // Put teardown code here. This method is called after the invocation of each test method in the class.
    [TestUtils cleanupStorage];
    [super tearDown];
}

// MARK: - Demos

- (void)testAnoncredsWorksForSingleIssuerSingleProver {
    IndyHandle issuerWalletHandle = 0;
    IndyHandle proverWalletHandle = 0;

    //1. Create Issuer wallet, get wallet handle
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&issuerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Create Prover wallet, get wallet handle
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    // 3. Issuer create Schema
    NSString *schemaId;
    NSString *schemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:[TestUtils issuerDid]
                                                             schemaId:&schemaId
                                                           schemaJson:&schemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    //3. Issuer create credential definition
    NSString *credentialDefId;
    NSString *credentialDefJSON;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:schemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:issuerWalletHandle
                                                                            credDefId:&credentialDefId
                                                                          credDefJson:&credentialDefJSON];
    XCTAssertEqual(ret.code, Success, @"issuerCreateAndStoreCredentialDefForSchema failed");

    //4. Prover create Master Secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:[TestUtils commonMasterSecretName]
                                                       walletHandle:proverWalletHandle
                                                  outMasterSecretId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed");

    // 5. Issuer create Credential Offer
    NSString *credentialOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:credentialDefId
                                                                      walletHandle:issuerWalletHandle
                                                                     credOfferJson:&credentialOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //6. Prover create Credential Request
    NSString *credentialReq = nil;
    NSString *credentialReqMetadata = nil;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:credentialOfferJson
                                                                     credentialDefJSON:credentialDefJSON
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:proverWalletHandle
                                                                           credReqJson:&credentialReq
                                                                   credReqMetadataJson:&credentialReqMetadata];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq() failed");

    //7. Issuer create Credential
    NSString *credentialJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:credentialReq
                                                                        credOfferJSON:credentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:issuerWalletHandle
                                                                             credJson:&credentialJson
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredential() failed");

    // 8. Prover store received Credential
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:credentialJson
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                             credReqMetadataJSON:credentialReqMetadata
                                                     credDefJSON:credentialDefJSON
                                                   revRegDefJSON:nil
                                                    walletHandle:proverWalletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreCredential() failed");

    // 9. Prover gets Credentials for Proof Request
    NSString *proofReqJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    },
                    @"attr2_referent": @{
                            @"name": @"phone"
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

    NSString *credentialsJson = nil;

    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofReqJson
                                                              walletHandle:proverWalletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"proverGetCredentialsForProofReq() failed!");

    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];
    NSDictionary *credentials_for_attr_1 = credentials[@"attrs"][@"attr1_referent"][0];
    NSString *credentialReferent = credentials_for_attr_1[@"cred_info"][@"referent"];

    // 12. Prover create Proof
    NSString *requestedCredentialsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"self_attested_attributes": @{
                    @"attr2_referent": @"value"
            },
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"cred_id": credentialReferent,
                            @"revealed": @(YES)
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"cred_id": credentialReferent
                    }
            }
    }];

    NSString *schemasJson = [[AnoncredsUtils sharedInstance] toJson:@{schemaId: [NSDictionary fromString:schemaJson]}];
    NSString *credentialDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{credentialDefId: [NSDictionary fromString:credentialDefJSON]}];
    NSString *revocStatesJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofReqJson
                                              requestedCredentialsJSON:requestedCredentialsJson
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:proverWalletHandle
                                                             proofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed");

    NSDictionary *proof = [NSDictionary fromString:proofJson];
    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];

    XCTAssertTrue([@"Alex" isEqualToString:revealedAttr1[@"raw"]]);

    NSString *attestedAttr = proof[@"requested_proof"][@"self_attested_attrs"][@"attr2_referent"];
    XCTAssertTrue([attestedAttr isEqualToString:@"value"]);

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";

    // 13. Verifier verify proof
    BOOL isValid = NO;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofReqJson
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJson
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValid];

    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValid, @"isValid == NO");
}

- (void)testAnoncredsWorksForMultiplyIssuerSingleProver {
    //1. Issuer1 create wallet, get wallet handles
    IndyHandle issuerGvtWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&issuerGvtWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Issuer2 create wallet, get wallet handles
    IndyHandle issuerXyzWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&issuerXyzWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //3. Prover create wallet, get wallet handles
    IndyHandle proverWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //4. Issuer create GVT Schema
    NSString *gvtSchemaId;
    NSString *gvtSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:[TestUtils issuerDid]
                                                             schemaId:&gvtSchemaId
                                                           schemaJson:&gvtSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    //4. Issuer create XYZ Schema
    NSString *xyzSchemaId;
    NSString *xyzSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils xyzSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils xyzSchemaAttrs]
                                                            issuerDID:[TestUtils issuer2Did]
                                                             schemaId:&xyzSchemaId
                                                           schemaJson:&xyzSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    //4. Issuer1 create credential definition by GVT Schema
    __block NSString *issuer1GvtCredentialDefId;
    __block NSString *issuer1GvtCredentialDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:gvtSchemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:issuerGvtWalletHandle
                                                                            credDefId:&issuer1GvtCredentialDefId
                                                                          credDefJson:&issuer1GvtCredentialDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinifionWithWalletHandle failed");

    //5. Issuer2 create credential definition by XYZ Schema
    NSString *issuer2XyzCredentialDefId;
    NSString *issuer2XyzCredentialDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:xyzSchemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:issuerXyzWalletHandle
                                                                            credDefId:&issuer2XyzCredentialDefId
                                                                          credDefJson:&issuer2XyzCredentialDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinifionWithWalletHandle failed");

    //6. Prover create Master Secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:[TestUtils commonMasterSecretName]
                                                       walletHandle:proverWalletHandle
                                                  outMasterSecretId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed for issuer 1");

    // 7. Issuer1 create Credential Offer
    NSString *issuer1GvtCredentialOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:issuer1GvtCredentialDefId
                                                                      walletHandle:issuerGvtWalletHandle
                                                                     credOfferJson:&issuer1GvtCredentialOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //8. Issuer2 create Credential Offer
    NSString *issuer2XyzCredentialOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:issuer2XyzCredentialDefId
                                                                      walletHandle:issuerXyzWalletHandle
                                                                     credOfferJson:&issuer2XyzCredentialOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //9. Prover create Credential Request for Issuer1 GVT credential offer
    NSString *issuer1GvtCredentialReq;
    NSString *issuer1GvtCredentialReqMetadata;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:issuer1GvtCredentialOfferJson
                                                                     credentialDefJSON:issuer1GvtCredentialDefJson
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:proverWalletHandle
                                                                           credReqJson:&issuer1GvtCredentialReq
                                                                   credReqMetadataJson:&issuer1GvtCredentialReqMetadata];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq() failed");

    //10. Issuer1 create GVT Credential
    NSString *issuer1GvtCredential;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:issuer1GvtCredentialReq
                                                                        credOfferJSON:issuer1GvtCredentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:issuerGvtWalletHandle
                                                                             credJson:&issuer1GvtCredential
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredential() failed for issuerGvtWalletHandle");

    //11. Prover store received GVT Credential
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:issuer1GvtCredential
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                             credReqMetadataJSON:issuer1GvtCredentialReqMetadata
                                                     credDefJSON:issuer1GvtCredentialDefJson
                                                   revRegDefJSON:nil
                                                    walletHandle:proverWalletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreCredential() failed");

    //12. Prover create Credential Request for xyz credential offer
    NSString *issuer2XyzCredentialReq;
    NSString *issuer2XyzCredentialReqMetadata;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:issuer2XyzCredentialOfferJson
                                                                     credentialDefJSON:issuer2XyzCredentialDefJson
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:proverWalletHandle
                                                                           credReqJson:&issuer2XyzCredentialReq
                                                                   credReqMetadataJson:&issuer2XyzCredentialReqMetadata];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq() failed");

    //13. Issuer create XYZ Credential
    NSString *issuer2XyzCredential;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:issuer2XyzCredentialReq
                                                                        credOfferJSON:issuer2XyzCredentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getXyzCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:issuerXyzWalletHandle
                                                                             credJson:&issuer2XyzCredential
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredential() failed for issuerXyzWalletHandle");

    // 14. Prover store received XYZ Credential
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:issuer2XyzCredential
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId2]
                                             credReqMetadataJSON:issuer2XyzCredentialReqMetadata
                                                     credDefJSON:issuer2XyzCredentialDefJson
                                                   revRegDefJSON:nil
                                                    walletHandle:proverWalletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreCredential() failed on step 16");

    // 15. Prover gets Credentials for Proof Request
    NSString *proofReqJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    },
                    @"attr2_referent": @{
                            @"name": @"status"
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"name": @"age",
                            @"p_type": @">=",
                            @"p_value": @(18)
                    },
                    @"predicate2_referent": @{
                            @"name": @"period",
                            @"p_type": @">=",
                            @"p_value": @(5)
                    }
            }
    }];

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofReqJson
                                                              walletHandle:proverWalletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForProofReq() failed");

    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];

    NSDictionary *credentialForAttr1 = credentials[@"attrs"][@"attr1_referent"][0];
    NSDictionary *credentialForAttr2 = credentials[@"attrs"][@"attr2_referent"][0];

    NSDictionary *credentialForPredicate1 = credentials[@"predicates"][@"predicate1_referent"][0];
    NSDictionary *credentialForPredicate2 = credentials[@"predicates"][@"predicate2_referent"][0];

    // 16. Prover create Proof
    NSString *credential_attr_1_UUID = credentialForAttr1[@"cred_info"][@"referent"];
    NSString *credential_attr_2_UUID = credentialForAttr2[@"cred_info"][@"referent"];
    NSString *credential_predicate_1_UUID = credentialForPredicate1[@"cred_info"][@"referent"];
    NSString *credential_predicate_2_UUID = credentialForPredicate2[@"cred_info"][@"referent"];

    NSString *requestedCredentialsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"self_attested_attributes": @{},
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"cred_id": credential_attr_1_UUID,
                            @"revealed": @(YES)
                    },
                    @"attr2_referent": @{
                            @"cred_id": credential_attr_2_UUID,
                            @"revealed": @(YES)
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"cred_id": credential_predicate_1_UUID
                    },
                    @"predicate2_referent": @{
                            @"cred_id": credential_predicate_2_UUID
                    }
            }
    }];

    NSString *schemasJson = [[AnoncredsUtils sharedInstance] toJson:@{
            gvtSchemaId: [NSDictionary fromString:gvtSchemaJson],
            xyzSchemaId: [NSDictionary fromString:xyzSchemaJson]}];
    NSString *credentialDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            issuer1GvtCredentialDefId: [NSDictionary fromString:issuer1GvtCredentialDefJson],
            issuer2XyzCredentialDefId: [NSDictionary fromString:issuer2XyzCredentialDefJson]}];

    NSString *revocStatesJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofReqJson
                                              requestedCredentialsJSON:requestedCredentialsJson
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:proverWalletHandle
                                                             proofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed on step 18");

    // 17. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJson];

    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];
    XCTAssertTrue([@"Alex" isEqualToString:revealedAttr1[@"raw"]]);

    NSDictionary *revealedAttr2 = proof[@"requested_proof"][@"revealed_attrs"][@"attr2_referent"];
    XCTAssertTrue([@"partial" isEqualToString:revealedAttr2[@"raw"]]);

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";

    BOOL isValidJson = NO;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofReqJson
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJson
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValidJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValidJson, @"proof is not verified!");
}


- (void)testAnoncredsWorksForSingleIssuerMultipleCredentialsSingleProver {
    //1. Issuer create wallet, get wallet handles
    IndyHandle issuerWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&issuerWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //2. Prover create wallet, get wallet handles
    IndyHandle proverWalletHandle = 0;
    ret = [[WalletUtils sharedInstance] createAndOpenWalletWithHandle:&proverWalletHandle];
    XCTAssertEqual(ret.code, Success, @"WalletUtils::createAndOpenWallet() failed");

    //4. Issuer create GVT Schema
    NSString *gvtSchemaId;
    NSString *gvtSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils gvtSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils gvtSchemaAttrs]
                                                            issuerDID:[TestUtils issuerDid]
                                                             schemaId:&gvtSchemaId
                                                           schemaJson:&gvtSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    //4. Issuer create XYZ Schema
    NSString *xyzSchemaId;
    NSString *xyzSchemaJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateSchemaWithName:[TestUtils xyzSchemaName]
                                                              version:[TestUtils schemaVersion]
                                                                attrs:[TestUtils xyzSchemaAttrs]
                                                            issuerDID:[TestUtils issuerDid]
                                                             schemaId:&xyzSchemaId
                                                           schemaJson:&xyzSchemaJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateSchemaForIssuerDID failed");

    //4. Issuer create credential definition by GVT Schema
    NSString *gvtCredentialDefId;
    NSString *gvtCredentialDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:gvtSchemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:issuerWalletHandle
                                                                            credDefId:&gvtCredentialDefId
                                                                          credDefJson:&gvtCredentialDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinifionWithWalletHandle failed");

    //5. Issuer create credential definition by XYZ Schema
    NSString *xyzCredentialDefId;
    NSString *xyzCredentialDefJson;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateAndStoreCredentialDefForSchema:xyzSchemaJson
                                                                            issuerDID:[TestUtils issuerDid]
                                                                                  tag:[TestUtils tag]
                                                                                 type:nil
                                                                           configJSON:[[AnoncredsUtils sharedInstance] defaultCredentialDefConfig]
                                                                         walletHandle:issuerWalletHandle
                                                                            credDefId:&xyzCredentialDefId
                                                                          credDefJson:&xyzCredentialDefJson];
    XCTAssertEqual(ret.code, Success, @"issuerCreateCredentialDefinifionWithWalletHandle failed");

    //6. Prover create Master Secret
    ret = [[AnoncredsUtils sharedInstance] proverCreateMasterSecret:[TestUtils commonMasterSecretName]
                                                       walletHandle:proverWalletHandle
                                                  outMasterSecretId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateMasterSecret() failed for issuer 1");

    // 7. Issuer create Credential Offer
    NSString *gvtCredentialOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:gvtCredentialDefId
                                                                      walletHandle:issuerWalletHandle
                                                                     credOfferJson:&gvtCredentialOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //8. Issuer create Credential Offer
    NSString *xyzCredentialOfferJson = nil;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialOfferForCredDefId:xyzCredentialDefId
                                                                      walletHandle:issuerWalletHandle
                                                                     credOfferJson:&xyzCredentialOfferJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateAndStoreCredentialDef() failed!");

    //9. Prover create Credential Request for Issuer GVT credential offer
    NSString *gvtCredentialReq;
    NSString *gvtCredentialReqMetadata;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:gvtCredentialOfferJson
                                                                     credentialDefJSON:gvtCredentialDefJson
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:proverWalletHandle
                                                                           credReqJson:&gvtCredentialReq
                                                                   credReqMetadataJson:&gvtCredentialReqMetadata];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq() failed");

    //10. Issuer create GVT Credential
    NSString *gvtCredential;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:gvtCredentialReq
                                                                        credOfferJSON:gvtCredentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getGvtCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:issuerWalletHandle
                                                                             credJson:&gvtCredential
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredential() failed for issuerGvtWalletHandle");

    //11. Prover store received GVT Credential
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:gvtCredential
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId1]
                                             credReqMetadataJSON:gvtCredentialReqMetadata
                                                     credDefJSON:gvtCredentialDefJson
                                                   revRegDefJSON:nil
                                                    walletHandle:proverWalletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreCredential() failed");

    //12. Prover create Credential Request for xyz credential offer
    NSString *xyzCredentialReq;
    NSString *xyzCredentialReqMetadata;
    ret = [[AnoncredsUtils sharedInstance] proverCreateCredentialReqForCredentialOffer:xyzCredentialOfferJson
                                                                     credentialDefJSON:xyzCredentialDefJson
                                                                             proverDID:[TestUtils proverDid]
                                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                                          walletHandle:proverWalletHandle
                                                                           credReqJson:&xyzCredentialReq
                                                                   credReqMetadataJson:&xyzCredentialReqMetadata];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateAndStoreCredentialReq() failed");

    //13. Issuer create XYZ Credential
    NSString *xyzCredential;
    ret = [[AnoncredsUtils sharedInstance] issuerCreateCredentialForCredentialRequest:xyzCredentialReq
                                                                        credOfferJSON:xyzCredentialOfferJson
                                                                       credValuesJSON:[[AnoncredsUtils sharedInstance] getXyzCredentialValuesJson]
                                                                             revRegId:nil
                                                              blobStorageReaderHandle:nil
                                                                         walletHandle:issuerWalletHandle
                                                                             credJson:&xyzCredential
                                                                          credRevocId:nil
                                                                    revocRegDeltaJSON:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::issuerCreateCredential() failed for issuerXyzWalletHandle");

    // 14. Prover store received XYZ Credential
    ret = [[AnoncredsUtils sharedInstance] proverStoreCredential:xyzCredential
                                                          credID:[[AnoncredsUtils sharedInstance] credentialId2]
                                             credReqMetadataJSON:xyzCredentialReqMetadata
                                                     credDefJSON:xyzCredentialDefJson
                                                   revRegDefJSON:nil
                                                    walletHandle:proverWalletHandle
                                                       outCredId:nil];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverStoreCredential() failed on step 16");

    // 15. Prover gets Credentials for Proof Request
    NSString *proofReqJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"nonce": @"123432421212",
            @"name": @"proof_req_1",
            @"version": @"0.1",
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"name": @"name"
                    },
                    @"attr2_referent": @{
                            @"name": @"status"
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"name": @"age",
                            @"p_type": @">=",
                            @"p_value": @(18)
                    },
                    @"predicate2_referent": @{
                            @"name": @"period",
                            @"p_type": @">=",
                            @"p_value": @(5)
                    }
            }
    }];

    NSString *credentialsJson;
    ret = [[AnoncredsUtils sharedInstance] proverGetCredentialsForProofReq:proofReqJson
                                                              walletHandle:proverWalletHandle
                                                           credentialsJson:&credentialsJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverGetCredentialsForProofReq() failed");

    NSDictionary *credentials = [NSDictionary fromString:credentialsJson];

    NSDictionary *credentialForAttr1 = credentials[@"attrs"][@"attr1_referent"][0];
    NSDictionary *credentialForAttr2 = credentials[@"attrs"][@"attr2_referent"][0];

    NSDictionary *credentialForPredicate1 = credentials[@"predicates"][@"predicate1_referent"][0];
    NSDictionary *credentialForPredicate2 = credentials[@"predicates"][@"predicate2_referent"][0];

    // 16. Prover create Proof
    NSString *credential_attr_1_UUID = credentialForAttr1[@"cred_info"][@"referent"];
    NSString *credential_attr_2_UUID = credentialForAttr2[@"cred_info"][@"referent"];
    NSString *credential_predicate_1_UUID = credentialForPredicate1[@"cred_info"][@"referent"];
    NSString *credential_predicate_2_UUID = credentialForPredicate2[@"cred_info"][@"referent"];

    NSString *requestedCredentialsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            @"self_attested_attributes": @{},
            @"requested_attributes": @{
                    @"attr1_referent": @{
                            @"cred_id": credential_attr_1_UUID,
                            @"revealed": @(YES)
                    },
                    @"attr2_referent": @{
                            @"cred_id": credential_attr_2_UUID,
                            @"revealed": @(YES)
                    }
            },
            @"requested_predicates": @{
                    @"predicate1_referent": @{
                            @"cred_id": credential_predicate_1_UUID
                    },
                    @"predicate2_referent": @{
                            @"cred_id": credential_predicate_2_UUID
                    }
            }
    }];

    NSString *schemasJson = [[AnoncredsUtils sharedInstance] toJson:@{
            gvtSchemaId: [NSDictionary fromString:gvtSchemaJson],
            xyzSchemaId: [NSDictionary fromString:xyzSchemaJson]}];
    NSString *credentialDefsJson = [[AnoncredsUtils sharedInstance] toJson:@{
            gvtCredentialDefId: [NSDictionary fromString:gvtCredentialDefJson],
            xyzCredentialDefId: [NSDictionary fromString:xyzCredentialDefJson]}];
    NSString *revocStatesJson = @"{}";

    NSString *proofJson;
    ret = [[AnoncredsUtils sharedInstance] proverCreateProofForRequest:proofReqJson
                                              requestedCredentialsJSON:requestedCredentialsJson
                                                        masterSecretID:[TestUtils commonMasterSecretName]
                                                           schemasJSON:schemasJson
                                                    credentialDefsJSON:credentialDefsJson
                                                       revocStatesJSON:revocStatesJson
                                                          walletHandle:proverWalletHandle
                                                             proofJson:&proofJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::proverCreateProof() failed on step 18");

    // 17. Verifier verify proof
    NSDictionary *proof = [NSDictionary fromString:proofJson];
    XCTAssertTrue(proof, @"serialization failed");

    NSDictionary *revealedAttr1 = proof[@"requested_proof"][@"revealed_attrs"][@"attr1_referent"];
    XCTAssertTrue([@"Alex" isEqualToString:revealedAttr1[@"raw"]]);

    NSDictionary *revealedAttr2 = proof[@"requested_proof"][@"revealed_attrs"][@"attr2_referent"];
    XCTAssertTrue([@"partial" isEqualToString:revealedAttr2[@"raw"]]);

    NSString *revocRegDefsJson = @"{}";
    NSString *revocRegsJson = @"{}";

    BOOL isValidJson = NO;
    ret = [[AnoncredsUtils sharedInstance] verifierVerifyProofRequest:proofReqJson
                                                            proofJSON:proofJson
                                                          schemasJSON:schemasJson
                                                   credentialDefsJSON:credentialDefsJson
                                                     revocRegDefsJSON:revocRegDefsJson
                                                        revocRegsJSON:revocRegsJson
                                                              isValid:&isValidJson];
    XCTAssertEqual(ret.code, Success, @"AnoncredsUtils::verifierVerifyProof() failed");
    XCTAssertTrue(isValidJson, @"proof is not verified!");
}

@end
